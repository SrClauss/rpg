use crate::llm::orchestrator::NarrativeOrchestrator;
use crate::llm::skeleton_agent::SkeletonAgent;
use crate::narrative::chronicle::{ChronicleEntry, ChronicleEntryType};
use crate::narrative::entity_store::TargetSpecificModifier;
use crate::narrative::living_scene::{LivingScene, SceneResolutionReason};
use crate::state::CampaignState;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneClosureResult {
    pub chronicle_event_id: String,
    pub polished_chapter_text: String,
    pub scene_summary: String,
    pub resolution_reason: SceneResolutionReason,
    pub skeleton_was_corrupted: bool,
    pub affected_entities: Vec<String>,
}

pub struct SceneCloser;

impl SceneCloser {
    pub async fn close_and_consolidate(
        scene: &mut LivingScene,
        reason: SceneResolutionReason,
        state: &mut CampaignState,
        orchestrator: &NarrativeOrchestrator,
        skeleton_agent: &SkeletonAgent,
    ) -> Result<SceneClosureResult, Box<dyn Error>> {
        // 1. Gera resumo da cena viva a partir do histórico de turnos
        let mut turns_summary = String::new();
        let mut rules_history = Vec::new();
        let mut involved_actors = Vec::new();

        for turn in &scene.turn_history {
            turns_summary.push_str(&format!(
                "- Rodada/Turno {}: {} executou {}. Reação/Resultado: {}\n",
                turn.turn_number, turn.actor_name, turn.player_input, turn.environment_reaction
            ));

            if let Some(ref res) = turn.rules_result {
                rules_history.push(res.clone());
            }

            if !involved_actors.contains(&turn.actor_id) {
                involved_actors.push(turn.actor_id.clone());
            }
        }

        let summary = format!(
            "Cena '{}' em {} encerrada (Motivo: {:?}).\nTurnos ocorridos:\n{}",
            scene.title, scene.location, reason, turns_summary
        );

        scene.close_scene(reason.clone(), &summary);

        // 2. Registra entrada provisória para gerar o event_id na crônica
        let mut chronicle = state.get_chronicle_book()?;
        let mut entry = ChronicleEntry::new(
            scene.chapter,
            &scene.title,
            ChronicleEntryType::LivingSceneSummary,
            &summary,
            rules_history.clone(),
            involved_actors.clone(),
        );

        let event_id = chronicle.append_entry(&mut entry)?;

        // 3. Aplica impactos nos JSONs das entidades envolvidas com o source_event_id
        let entity_store = state.get_entity_store()?;
        let mut affected_entity_ids = Vec::new();

        let world_entities = entity_store.list_world_entities().unwrap_or_default();
        for mut entity in world_entities {
            let matches_scene = summary.to_lowercase().contains(&entity.name.to_lowercase())
                || scene.location.to_lowercase().contains(&entity.name.to_lowercase());

            if matches_scene {
                let desc = format!("Participou ou esteve presente no evento '{}'", scene.title);
                let psych = format!("Testemunhou as ações do grupo durante o encerramento da cena em {}", scene.location);

                entity.add_impact(
                    &event_id,
                    &desc,
                    &psych,
                    vec![TargetSpecificModifier {
                        target_actor_id: "*".to_string(),
                        dc_adjustment: 0,
                        dice_mode: None,
                        reason: "Reconhecimento das atitudes do grupo".to_string(),
                    }],
                );

                let _ = entity_store.save_world_entity(&entity);
                affected_entity_ids.push(entity.id);
            }
        }

        // 4. Avalia e repara o Esqueleto Oculto caso tenha havido corrupção
        let mut skeleton_corrupted = false;
        if let Some(ref mut manifest) = state.manifest {
            let corrupted = skeleton_agent
                .evaluate_and_repair(
                    &mut manifest.hidden_skeleton,
                    &summary,
                    scene.chapter,
                )
                .await
                .unwrap_or(false);

            if !corrupted {
                // Se não corrompeu, marca o ato atual como completado se os objetivos foram cumpridos
                if reason == SceneResolutionReason::ObjectivesCompleted {
                    manifest.hidden_skeleton.mark_act_completed();
                }
            }
            skeleton_corrupted = corrupted;
            let _ = state.save_manifest();
        }

        // 5. Aciona a Dobradinha Literária (Escritor + Revisor + Polimento)
        let literary_context = format!(
            "Cena: {}. Local: {}. Resumo dos Acontecimentos da Cena Viva:\n{}",
            scene.title, scene.location, summary
        );

        let polished_chapter = match orchestrator
            .generate_refined_narrative(&literary_context, "Consolidação e encerramento do capítulo com prosa poética e dramática.")
            .await
        {
            Ok(p) => p,
            Err(e) => {
                eprintln!("[SCENE_CLOSER] Falha na dobradinha literária: {}. Usando resumo direto.", e);
                format!("Capítulo: {}\n\n{}", scene.title, summary)
            }
        };

        // 6. Atualiza o livro da crônica com o capítulo final literário
        let mut final_chapter_entry = ChronicleEntry::new(
            scene.chapter,
            &format!("Capítulo: {}", scene.title),
            ChronicleEntryType::LiteraryChapter,
            &polished_chapter,
            rules_history,
            involved_actors,
        );
        let final_event_id = chronicle.append_entry(&mut final_chapter_entry)?;

        // 7. Atualiza o estado da campanha salvando a cena concluída e removendo do active_scene
        state.active_scene = None;
        let _ = state.save_active_scene();

        Ok(SceneClosureResult {
            chronicle_event_id: final_event_id,
            polished_chapter_text: polished_chapter,
            scene_summary: summary,
            resolution_reason: reason,
            skeleton_was_corrupted: skeleton_corrupted,
            affected_entities: affected_entity_ids,
        })
    }
}
