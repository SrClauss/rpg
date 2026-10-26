use crate::llm::gemini::GeminiClient;
use crate::narrative::entity_store::EntityStore;
use crate::narrative::living_scene::{
    LivingScene, SceneInteractionTurn, SceneLootItem, SceneObjective, SceneResolutionReason, TurnParticipant,
};
use crate::rules::resolver::RulesResolver;
use crate::state::CampaignState;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvironmentTurnOutput {
    pub narrative_reaction: String,
    pub mechanical_intent: Option<Value>,
    pub rules_result: Option<Value>,
    pub completed_objective_id: Option<String>,
    pub scene_resolution: Option<SceneResolutionReason>,
    pub memory_impacts_to_apply: Vec<Value>,
}

pub struct EnvironmentAgent {
    pub gemini: GeminiClient,
}

impl EnvironmentAgent {
    pub fn new(api_key: String) -> Self {
        Self {
            gemini: GeminiClient::new(api_key),
        }
    }

    pub fn with_client(gemini: GeminiClient) -> Self {
        Self { gemini }
    }

    /// Gera dinamicamente a configuração inicial de uma Cena Viva (Título, Local, Narração de Abertura, Objetivos, NPCs e Opções de Cenário Via 2) a partir do Esqueleto Narrativo e da Crônica da Campanha
    pub async fn generate_dynamic_scene(
        &self,
        campaign_id: &str,
        premise: &str,
        literary_style: &str,
        act_number: u32,
        act_title: &str,
        act_synopsis: &str,
        key_milestones: &[String],
        latest_chronicle_prose: &str,
        participants: Vec<TurnParticipant>,
    ) -> LivingScene {
        let milestones_text = if key_milestones.is_empty() {
            "Investigar a ameaça central do ato; Interagir com os personagens da trama; Superar o obstáculo imediato".to_string()
        } else {
            key_milestones.join(" | ")
        };

        let system_instruction = "Você é o Agente de Ambiente e Diretor Tático D&D 5e (Gemini Flash-Lite). \
Sua missão é criar o payload dinâmico de abertura de uma CENA VIVA interativa estritamente fiel à PREMISSA, ao ATO ATUAL do Esqueleto Narrativo e ao PRÓLOGO ÉPICO / WORLDBUILDING da campanha. \
DIRETRIZ DE ABERTURA E AMBIENTAÇÃO: \
A narração de abertura (`opening_narration`) DEVE ter de 3 a 4 parágrafos literários imersivos. Ela deve conectar o ambiente imediato à geopolítica e história mais ampla do reino descrita no prólogo, para que a transição seja cadenciada e épica, antes de focar na postura tática dos heróis e na reação ativa dos NPCs ou inimigos. \
DIRETRIZ DE ITENS / TESOUROS OCULTOS E SURPRESAS (D&D 5E): \
Em todas as cenas, você DEVE colocar de 1 a 3 itens em `scene_loot`, mas eles começam OCULTOS (`is_hidden: true`). Na narração de abertura (`opening_narration`), insira PISTAS SENSORIAIS SUTIS (ex: reflexo de metal sob escombros, aroma de ervas alquímicas, fendas no piso, armaduras rúnicas em inimigos) para que o jogador INTUA que há segredos ou tesouros no local e decida investigar. NUNCA revele uma lista de compras explícita. Itens também podem ser saqueados de inimigos derrotados. \
- Itens recomendados: Armaduras de Mithral, Poções, Pergaminhos, Relíquias do cenário. \
Você DEVE responder APENAS com um JSON puro válido no seguinte formato:
{
  \"title\": \"Título específico e evocativo da cena atual\",
  \"location\": \"Nome exato do local onde a cena acontece na história\",
  \"opening_narration\": \"3 a 4 parágrafos substanciais e imersivos que conectam o local imediato ao contexto maior do reino e à atmosfera da campanha, detalhando sensorialmente o terreno, as pistas de segredos ocultos, o posicionamento dos heróis e a primeira ação ou postura hostil ativa dos NPCs/inimigos\",
  \"objectives\": [
    \"Objetivo claro 1 derivado dos marcos do ato\",
    \"Objetivo claro 2 derivado dos marcos do ato\",
    \"Objetivo claro 3 derivado dos marcos do ato\"
  ],
  \"active_npcs\": [
    \"Nome do NPC ou Inimigo 1 presente na cena\",
    \"Nome do NPC ou Inimigo 2 presente na cena\"
  ],
  \"scene_loot\": [
    {
      \"id\": \"loot_1\",
      \"name\": \"Nome do Item ou Relíquia\",
      \"item_type\": \"Arma Mágica\" ou \"Armadura\" ou \"Poção\" ou \"Relíquia\",
      \"description\": \"Origem e detalhes do item\",
      \"rarity\": \"Incomum\" ou \"Raro\" ou \"Muito Raro\",
      \"mechanical_bonus\": \"Ex: +1 no acerto / Resistência a Frio / Cura 4d4+4\",
      \"location_hint\": \"Ex: No baú de ferro dos mineiros / Guardado na alcova secreta / Empunhado pelo Boss\"
    }
  ],
  \"scenario_options\": [
    {
      \"id\": \"scen_opt_1\",
      \"title\": \"Título curto da ação contextual no cenário\",
      \"description\": \"O que o personagem faz interagindo com elementos desta cena específica\",
      \"actionCost\": \"action\",
      \"skillCheckSuggested\": {
        \"skill\": \"Perception\",
        \"dcEstimate\": 13
      },
      \"tacticalBenefit\": \"Vantagem tática ou descoberta narrativa obtida em caso de sucesso\"
    },
    {
      \"id\": \"scen_opt_2\",
      \"title\": \"Segunda opção tática/social específica da cena\",
      \"description\": \"Descrição ligada aos NPCs ou terreno da história\",
      \"actionCost\": \"action\",
      \"skillCheckSuggested\": {
        \"skill\": \"Insight\",
        \"dcEstimate\": 14
      },
      \"tacticalBenefit\": \"Benefício tático ou social\"
    },
    {
      \"id\": \"scen_opt_3\",
      \"title\": \"Manobra de terreno/posicionamento na cena\",
      \"description\": \"Uso criativo do ambiente descrito\",
      \"actionCost\": \"bonus_action\",
      \"skillCheckSuggested\": {
        \"skill\": \"Stealth\",
        \"dcEstimate\": 12
      },
      \"tacticalBenefit\": \"Cobertura ou vantagem posicional\"
    }
  ]
}";

        let prompt = format!(
            "PREMISSA DA CAMPANHA:\n{}\n\nESTILO LITERÁRIO:\n{}\n\nATO ATUAL (ATO {} — {}):\nSinopse do Ato: {}\nMarcos Obrigatórios do Ato: {}\n\nTEXTO RECENTE DA CRÔNICA / PRÓLOGO:\n{}\n\nGere agora o JSON completo da Cena Viva para este momento da história.",
            premise,
            literary_style,
            act_number,
            act_title,
            act_synopsis,
            milestones_text,
            latest_chronicle_prose
        );

        let fallback_objectives: Vec<SceneObjective> = if !key_milestones.is_empty() {
            key_milestones
                .iter()
                .enumerate()
                .map(|(i, m)| SceneObjective {
                    id: format!("obj_{}", i + 1),
                    description: m.clone(),
                    completed: false,
                    completion_trigger: None,
                })
                .collect()
        } else {
            vec![
                SceneObjective {
                    id: "obj_1".to_string(),
                    description: format!("Desvendar a situação central de '{}'", act_title),
                    completed: false,
                    completion_trigger: None,
                },
                SceneObjective {
                    id: "obj_2".to_string(),
                    description: "Confrontar ou interagir com os envolvidos na trama".to_string(),
                    completed: false,
                    completion_trigger: None,
                },
            ]
        };

        let fallback_options: Vec<Value> = fallback_objectives
            .iter()
            .enumerate()
            .map(|(i, obj)| {
                json!({
                    "id": format!("scen_story_{}", i + 1),
                    "title": format!("Focar em: {}", obj.description),
                    "description": format!("Direciona sua atenção tática e investigativa no cenário para cumprir: {}.", obj.description),
                    "actionCost": "action",
                    "skillCheckSuggested": {
                        "skill": if i == 0 { "Investigation" } else if i == 1 { "Persuasion" } else { "Perception" },
                        "dcEstimate": 12 + (i as i32)
                    },
                    "tacticalBenefit": "Avança diretamente na resolução dos marcos do Ato atual."
                })
            })
            .collect();

        if let Ok(raw) = self.gemini.generate_narrative(&prompt, system_instruction).await {
            let clean = Self::extract_json(&raw);
            if let Ok(parsed) = serde_json::from_str::<Value>(&clean) {
                let title = parsed["title"]
                    .as_str()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or(act_title)
                    .to_string();

                let location = parsed["location"]
                    .as_str()
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("Cenário do Ato {}: {}", act_number, act_title));

                let opening_narration = parsed["opening_narration"]
                    .as_str()
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("{}\n\n⚔️ O ambiente ganha vida ao seu redor! Cada ação consumirá seu orçamento de turno e provocará a reação imediata das forças presentes na cena.", act_synopsis));

                let objectives: Vec<SceneObjective> = parsed["objectives"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .enumerate()
                            .map(|(i, desc)| SceneObjective {
                                id: format!("obj_{}", i + 1),
                                description: desc.to_string(),
                                completed: false,
                                completion_trigger: None,
                            })
                            .collect::<Vec<_>>()
                    })
                    .filter(|v| !v.is_empty())
                    .unwrap_or_else(|| fallback_objectives.clone());

                let active_npcs: Vec<String> = parsed["active_npcs"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect::<Vec<_>>()
                    })
                    .filter(|v| !v.is_empty())
                    .unwrap_or_else(|| vec![format!("Personagens de {}", act_title)]);

                let scenario_options: Vec<Value> = parsed["scenario_options"]
                    .as_array()
                    .cloned()
                    .filter(|v| !v.is_empty())
                    .unwrap_or_else(|| fallback_options.clone());

                let scene_loot: Vec<SceneLootItem> = parsed["scene_loot"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .enumerate()
                            .map(|(i, item)| SceneLootItem {
                                id: item["id"].as_str().map(|s| s.to_string()).unwrap_or_else(|| format!("loot_{}", i + 1)),
                                name: item["name"].as_str().unwrap_or("Relíquia Desconhecida").to_string(),
                                item_type: item["item_type"].as_str().unwrap_or("Relíquia").to_string(),
                                description: item["description"].as_str().unwrap_or("Encontrado nas profundezas do cenário").to_string(),
                                rarity: item["rarity"].as_str().unwrap_or("Incomum").to_string(),
                                mechanical_bonus: item["mechanical_bonus"].as_str().map(|s| s.to_string()),
                                location_hint: item["location_hint"].as_str().unwrap_or("No cenário").to_string(),
                                is_hidden: item["is_hidden"].as_bool().unwrap_or(true),
                                is_claimed: false,
                                claimed_by: None,
                            })
                            .collect()
                    })
                    .unwrap_or_else(|| vec![
                        SceneLootItem {
                            id: "loot_fallback_1".to_string(),
                            name: "Cota de Malha de Mithral dos Anões".to_string(),
                            item_type: "Armadura".to_string(),
                            description: "Armadura forjada nas veias profundas da montanha; leve, não impõe desvantagem em Furtividade.".to_string(),
                            rarity: "Incomum".to_string(),
                            mechanical_bonus: Some("CA 16 sem desvantagem em Furtividade".to_string()),
                            location_hint: "Em um baú de pedra esquecido nas galerias da mina".to_string(),
                            is_hidden: true,
                            is_claimed: false,
                            claimed_by: None,
                        },
                        SceneLootItem {
                            id: "loot_fallback_2".to_string(),
                            name: "Poção de Cura Superior".to_string(),
                            item_type: "Poção".to_string(),
                            description: "Frasco de cristal espesso contendo um líquido escarlate brilhante.".to_string(),
                            rarity: "Rara".to_string(),
                            mechanical_bonus: Some("Restaura 8d4 + 8 Pontos de Vida".to_string()),
                            location_hint: "No estojo de couro de um batedor caído".to_string(),
                            is_hidden: true,
                            is_claimed: false,
                            claimed_by: None,
                        }
                    ]);

                let mut scene = LivingScene::new(
                    campaign_id,
                    &title,
                    &location,
                    act_number,
                    objectives,
                    participants,
                );
                scene.opening_narration = Some(opening_narration);
                scene.active_npcs = active_npcs;
                scene.scenario_options = scenario_options;
                scene.scene_loot = scene_loot;
                scene.world_prologue = Some(latest_chronicle_prose.to_string());
                return scene;
            }
        }

        let mut scene = LivingScene::new(
            campaign_id,
            act_title,
            &format!("Cenário do Ato {}: {}", act_number, act_title),
            act_number,
            fallback_objectives,
            participants,
        );
        scene.opening_narration = Some(format!(
            "{}\n\n⚔️ A Cena Viva de '{}' começou! Suas ações agora desencadeiam rolagens determinísticas D&D 5e e respostas imediatas dos personagens e ameaças deste ato.",
            act_synopsis, act_title
        ));
        scene.active_npcs = vec![format!("Presenças em {}", act_title)];
        scene.scenario_options = fallback_options;
        scene.world_prologue = Some(latest_chronicle_prose.to_string());
        scene
    }

    /// Sincroniza o HP atual de um jogador no EntityStore (dnd_stats e full_sheet.vitals)
    pub fn sync_player_hp_to_store(
        entity_store: &EntityStore,
        player_id: &str,
        player_name: &str,
        new_hp: i32,
    ) {
        if let Ok(players) = entity_store.list_player_entities() {
            if let Some(mut sheet) = players
                .into_iter()
                .find(|p| p.id == player_id || p.name.eq_ignore_ascii_case(player_name))
            {
                if let Some(ref mut dnd) = sheet.dnd_stats {
                    dnd.current_hp = new_hp;
                }
                if let Some(full_sheet) = sheet.custom_flags.get_mut("full_sheet") {
                    if let Some(vitals) = full_sheet.get_mut("vitals") {
                        if let Some(obj) = vitals.as_object_mut() {
                            obj.insert("hpCurrent".to_string(), json!(new_hp));
                        }
                    }
                }
                let _ = entity_store.save_player_entity(&sheet);
            }
        }
    }

    /// Verifica se todos os inimigos hostis foram derrotados e aplica o estado de Vitória Tática
    pub fn check_and_apply_combat_victory(scene: &mut LivingScene) -> bool {
        let had_hostiles = scene.participants.iter().any(|p| !p.is_player && p.is_hostile);
        if scene.in_combat && had_hostiles && !scene.has_live_hostiles() {
            scene.in_combat = false;
            let mut marked_any = false;
            for obj in &mut scene.objectives {
                let desc_l = obj.description.to_lowercase();
                if desc_l.contains("sobreviv")
                    || desc_l.contains("elimin")
                    || desc_l.contains("derrot")
                    || desc_l.contains("emboscad")
                    || desc_l.contains("batedor")
                    || desc_l.contains("goblin")
                    || desc_l.contains("inimig")
                    || desc_l.contains("combat")
                    || desc_l.contains("proteg")
                {
                    obj.completed = true;
                    marked_any = true;
                }
            }
            if !marked_any {
                if let Some(first_incomplete) = scene.objectives.iter_mut().find(|o| !o.completed) {
                    first_incomplete.completed = true;
                }
            }
            return true;
        }
        false
    }

    /// Avança para o próximo jogador vivo na ordem de iniciativa, executando ataques reais e movimentação de todos os NPCs/inimigos entre eles
    pub fn advance_to_next_player_and_run_npcs(
        scene: &mut LivingScene,
        entity_store: &EntityStore,
    ) -> (Vec<String>, Vec<Value>, bool) {
        use rand::Rng;
        let mut combat_logs = Vec::new();
        let mut enemy_turns_json = Vec::new();
        let mut victory_achieved = false;

        if scene.participants.is_empty() {
            return (combat_logs, enemy_turns_json, victory_achieved);
        }

        let any_live_player = scene
            .participants
            .iter()
            .any(|p| p.is_player && !p.is_dead && p.current_hp > 0);
        if !any_live_player {
            scene.in_combat = false;
            return (combat_logs, enemy_turns_json, victory_achieved);
        }

        let len = scene.participants.len();
        let max_steps = len * 2;
        let mut rng = rand::thread_rng();
        let mut targeted_this_round: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();

        for _ in 0..max_steps {
            scene.current_participant_index += 1;
            if scene.current_participant_index >= len {
                scene.current_participant_index = 0;
                scene.round_number += 1;
                targeted_this_round.clear();
                for p in &mut scene.participants {
                    p.actions_budget.reset_turn();
                }
            }

            let idx = scene.current_participant_index;
            if scene.participants[idx].is_dead || scene.participants[idx].current_hp <= 0 {
                continue;
            }

            if scene.participants[idx].is_player {
                scene.participants[idx].actions_budget.reset_turn();
                break;
            }

            // Participante atual é um NPC vivo (!is_player && !is_dead)
            if !scene.in_combat || !scene.has_live_hostiles() {
                continue;
            }

            let is_hostile = scene.participants[idx].is_hostile;
            let npc_id = scene.participants[idx].id.clone();
            let npc_name = scene.participants[idx].name.clone();
            let (npc_x, npc_y) = scene.participants[idx].position.unwrap_or((45, 20));

            if is_hostile {
                // Alvo Inteligente: ataca qualquer combatente da comitiva (jogadores ou aliados)
                // Evita "Focus Fire" irrealista distribuindo alvos entre a linha de frente
                let mut candidates: Vec<(usize, i32)> = scene
                    .participants
                    .iter()
                    .enumerate()
                    .filter(|(i, p)| {
                        *i != idx
                            && (p.is_player || !p.is_hostile)
                            && !p.is_dead
                            && p.current_hp > 0
                    })
                    .map(|(i, p)| {
                        let (px, py) = p.position.unwrap_or((10, 20));
                        let dx = px - npc_x;
                        let dy = py - npc_y;
                        let dist_sq = dx * dx + dy * dy;
                        let times_targeted = targeted_this_round.get(&i).copied().unwrap_or(0);
                        let is_melee = dist_sq <= 36; // <= 6ft (engajado corpo-a-corpo)
                        let focus_penalty = if is_melee {
                            times_targeted as i32 * 60
                        } else {
                            times_targeted as i32 * 200
                        };
                        (i, dist_sq + focus_penalty)
                    })
                    .collect();

                candidates.sort_by_key(|(_, score)| *score);
                let target_idx_opt = candidates.first().map(|(i, _)| *i);

                if let Some(target_idx) = target_idx_opt {
                    *targeted_this_round.entry(target_idx).or_insert(0) += 1;
                    let (px, py) = scene.participants[target_idx].position.unwrap_or((10, 20));
                    let dx = (px - npc_x) as f64;
                    let dy = (py - npc_y) as f64;
                    let dist = (dx * dx + dy * dy).sqrt();
                    let (new_x, new_y) = if dist > 5.0 {
                        let step = 12.0_f64.min(dist - 5.0);
                        (
                            ((npc_x as f64 + (dx / dist) * step).round() as i32).clamp(2, 58),
                            ((npc_y as f64 + (dy / dist) * step).round() as i32).clamp(2, 38),
                        )
                    } else {
                        (npc_x, npc_y)
                    };
                    scene.participants[idx].position = Some((new_x, new_y));
                    scene.participants[idx].actions_budget.action_used = true;

                    let is_batedor_or_minion = npc_name.to_lowercase().contains("batedor")
                        || npc_name.to_lowercase().contains("scout")
                        || npc_name.to_lowercase().contains("cão")
                        || npc_name.to_lowercase().contains("cao")
                        || npc_name.to_lowercase().contains("kobold");

                    let d20: i32 = rng.gen_range(1..=20);
                    let atk_bonus: i32 = if is_batedor_or_minion { 3 } else { 4 };
                    let total = d20 + atk_bonus;
                    let is_crit = d20 == 20;
                    let target_ac = scene.participants[target_idx].armor_class.max(10);
                    let is_hit = d20 != 1 && (is_crit || total >= target_ac);

                    // Balanceamento D&D 5e: batedores/lacaios causam 1d4+1 (média 3.5); normais 1d6+1 (média 4.5)
                    let dmg = if is_hit {
                        if is_batedor_or_minion {
                            let dice = if is_crit {
                                rng.gen_range(1..=4) + rng.gen_range(1..=4)
                            } else {
                                rng.gen_range(1..=4)
                            };
                            (dice + 1).max(1)
                        } else {
                            let dice = if is_crit {
                                rng.gen_range(1..=6) + rng.gen_range(1..=6)
                            } else {
                                rng.gen_range(1..=6)
                            };
                            (dice + 1).max(1)
                        }
                    } else {
                        0
                    };

                    let target_player = &mut scene.participants[target_idx];
                    if is_hit {
                        target_player.current_hp = (target_player.current_hp - dmg).max(0);
                        if target_player.current_hp <= 0 {
                            target_player.current_hp = 0;
                            target_player.is_dead = true;
                        }
                    }

                    let t_id = target_player.id.clone();
                    let t_name = target_player.name.clone();
                    let t_cur_hp = target_player.current_hp;
                    let t_max_hp = target_player.max_hp.max(12);

                    if is_hit && target_player.is_player {
                        Self::sync_player_hp_to_store(entity_store, &t_id, &t_name, t_cur_hp);
                    }

                    let log_line = if is_hit {
                        format!(
                            "⚔️ [ATAQUE INIMIGO] {} avançou para ({}ft, {}ft) e atacou {}: d20={}+{} ({}) vs CA {} => ACERTO (-{} PV, restam {}/{} PV)",
                            npc_name, new_x, new_y, t_name, d20, atk_bonus, total, target_ac, dmg, t_cur_hp, t_max_hp
                        )
                    } else {
                        format!(
                            "⚔️ [ATAQUE INIMIGO] {} avançou para ({}ft, {}ft) e atacou {}: d20={}+{} ({}) vs CA {} => ERROU! (restam {}/{} PV)",
                            npc_name, new_x, new_y, t_name, d20, atk_bonus, total, target_ac, t_cur_hp, t_max_hp
                        )
                    };

                    combat_logs.push(log_line);
                    enemy_turns_json.push(json!({
                        "actor_id": npc_id,
                        "actor": npc_name,
                        "is_hostile": true,
                        "target_id": t_id,
                        "target": t_name,
                        "moved_to": [new_x, new_y],
                        "d20_roll": d20,
                        "modifier": atk_bonus,
                        "total_to_hit": total,
                        "target_ac": target_ac,
                        "is_hit": is_hit,
                        "is_critical": is_crit,
                        "damage": dmg,
                        "target_hp_remaining": t_cur_hp,
                        "target_max_hp": t_max_hp
                    }));
                }
            } else {
                // NPC Aliado (!is_hostile): ataca um inimigo hostil vivo se houver
                let enemy_idx_opt = scene
                    .participants
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| !p.is_player && p.is_hostile && !p.is_dead && p.current_hp > 0)
                    .min_by_key(|(_, p)| {
                        let (ex, ey) = p.position.unwrap_or((45, 20));
                        let dx = ex - npc_x;
                        let dy = ey - npc_y;
                        dx * dx + dy * dy
                    })
                    .map(|(i, _)| i);

                if let Some(enemy_idx) = enemy_idx_opt {
                    let (ex, ey) = scene.participants[enemy_idx].position.unwrap_or((45, 20));
                    let dx = (ex - npc_x) as f64;
                    let dy = (ey - npc_y) as f64;
                    let dist = (dx * dx + dy * dy).sqrt();
                    let (new_x, new_y) = if dist > 10.0 {
                        let step = 10.0_f64.min(dist - 5.0);
                        (
                            ((npc_x as f64 + (dx / dist) * step).round() as i32).clamp(2, 58),
                            ((npc_y as f64 + (dy / dist) * step).round() as i32).clamp(2, 38),
                        )
                    } else {
                        (npc_x, npc_y)
                    };
                    scene.participants[idx].position = Some((new_x, new_y));
                    scene.participants[idx].actions_budget.action_used = true;

                    let d20: i32 = rng.gen_range(1..=20);
                    let atk_bonus: i32 = 5;
                    let total = d20 + atk_bonus;
                    let is_crit = d20 == 20;
                    let enemy_ac = scene.participants[enemy_idx].armor_class.max(12);
                    let is_hit = d20 != 1 && (is_crit || total >= enemy_ac);

                    let dmg = if is_hit {
                        let dice = if is_crit {
                            rng.gen_range(1..=8) + rng.gen_range(1..=8)
                        } else {
                            rng.gen_range(1..=8)
                        };
                        (dice + 3).max(1)
                    } else {
                        0
                    };

                    let enemy_p = &mut scene.participants[enemy_idx];
                    if is_hit {
                        enemy_p.current_hp = (enemy_p.current_hp - dmg).max(0);
                        if enemy_p.current_hp <= 0 {
                            enemy_p.current_hp = 0;
                            enemy_p.is_dead = true;
                        }
                    }

                    let e_id = enemy_p.id.clone();
                    let e_name = enemy_p.name.clone();
                    let e_cur_hp = enemy_p.current_hp;
                    let e_max_hp = enemy_p.max_hp.max(12);
                    let e_dead = enemy_p.is_dead;

                    if is_hit {
                        if let Ok(Some(mut w_sheet)) = entity_store.get_world_entity(&e_id) {
                            if let Some(ref mut dnd) = w_sheet.dnd_stats {
                                dnd.current_hp = e_cur_hp;
                            }
                            let _ = entity_store.save_world_entity(&w_sheet);
                        }
                    }

                    let killed_tag = if e_dead { " — 💀 ABATIDO!" } else { "" };
                    let log_line = if is_hit {
                        format!(
                            "🏹 [ATAQUE ALIADO] {} atacou {}: d20={}+5 ({}) vs CA {} => ACERTO (-{} PV, restam {}/{} PV{})",
                            npc_name, e_name, d20, total, enemy_ac, dmg, e_cur_hp, e_max_hp, killed_tag
                        )
                    } else {
                        format!(
                            "🏹 [ATAQUE ALIADO] {} atacou {}: d20={}+5 ({}) vs CA {} => ERROU! (restam {}/{} PV)",
                            npc_name, e_name, d20, total, enemy_ac, e_cur_hp, e_max_hp
                        )
                    };

                    combat_logs.push(log_line);
                    enemy_turns_json.push(json!({
                        "actor_id": npc_id,
                        "actor": npc_name,
                        "is_hostile": false,
                        "target_id": e_id,
                        "target": e_name,
                        "moved_to": [new_x, new_y],
                        "d20_roll": d20,
                        "modifier": atk_bonus,
                        "total_to_hit": total,
                        "target_ac": enemy_ac,
                        "is_hit": is_hit,
                        "is_critical": is_crit,
                        "damage": dmg,
                        "target_hp_remaining": e_cur_hp,
                        "target_max_hp": e_max_hp,
                        "target_killed": e_dead
                    }));

                    if Self::check_and_apply_combat_victory(scene) {
                        victory_achieved = true;
                    }
                }
            }
        }

        (combat_logs, enemy_turns_json, victory_achieved)
    }

    /// Executa o turno do jogador dentro da Cena Viva com soberania das regras do D&D 5e e resposta ativa do ambiente
    pub async fn process_turn(
        &self,
        scene: &mut LivingScene,
        actor_id: &str,
        actor_name: &str,
        via_type: &str,
        player_input: &str,
        action_id: Option<&str>,
        state: &mut CampaignState,
        entity_store: &EntityStore,
    ) -> Result<EnvironmentTurnOutput, Box<dyn Error>> {
        use rand::Rng;

        // 1. Coleta contexto das entidades do mundo presentes na cena e seus modificadores contra o ator
        let world_entities = entity_store.list_world_entities().unwrap_or_default();
        let mut entity_contexts = Vec::new();

        for ent in &world_entities {
            let (dc_adj, dice_mode) = ent.get_modifiers_against(actor_id);
            let recent_traumas: Vec<String> = ent
                .memory_impacts
                .iter()
                .rev()
                .take(2)
                .map(|m| format!("[Evento {}]: {}", m.source_event_id, m.psychological_impact))
                .collect();

            entity_contexts.push(json!({
                "id": ent.id,
                "name": ent.name,
                "type": format!("{:?}", ent.entity_type),
                "dc_adjustment_vs_player": dc_adj,
                "dice_mode_vs_player": dice_mode,
                "recent_memories": recent_traumas,
            }));
        }

        let objectives_json: Vec<Value> = scene
            .objectives
            .iter()
            .map(|o| {
                json!({
                    "id": o.id,
                    "description": o.description,
                    "completed": o.completed
                })
            })
            .collect();

        let campaign_premise = state
            .manifest
            .as_ref()
            .map(|m| m.premise.as_str())
            .unwrap_or("");
        let current_act_synopsis = state
            .manifest
            .as_ref()
            .and_then(|m| m.hidden_skeleton.current_act())
            .map(|a| a.synopsis.as_str())
            .unwrap_or("");

        // 2. Identifica custo da ação (Ação Principal vs Ação Bônus vs Disparada) e atualiza o orçamento do jogador
        let act_id_lower = action_id.unwrap_or("").to_lowercase();
        let input_lower = player_input.to_lowercase();

        let is_scen_bonus = action_id
            .and_then(|aid| {
                scene.scenario_options.iter().find(|opt| {
                    opt["id"].as_str().map(|id| id == aid).unwrap_or(false)
                })
            })
            .and_then(|opt| opt["actionCost"].as_str())
            .map(|cost| cost == "bonus_action")
            .unwrap_or(false);

        let is_bonus_action = is_scen_bonus
            || act_id_lower.contains("bonus")
            || act_id_lower == "scen_opt_3"
            || act_id_lower.contains("fortify")
            || act_id_lower.contains("cunning")
            || act_id_lower.contains("rage")
            || act_id_lower.contains("second_wind")
            || act_id_lower.contains("healing_word")
            || input_lower.contains("ação bônus")
            || input_lower.contains("acao bonus")
            || input_lower.contains("astúcia")
            || input_lower.contains("astucia")
            || input_lower.contains("fúria")
            || input_lower.contains("furia")
            || input_lower.contains("palavra curativa")
            || input_lower.contains("retomar o fôlego")
            || input_lower.contains("retomar o folego");

        let is_dash = act_id_lower.contains("dash")
            || act_id_lower.contains("disparada")
            || input_lower.contains("disparada")
            || input_lower.contains("dash");

        if let Some(actor_idx) = scene.participants.iter().position(|p| {
            p.id == actor_id || p.name.eq_ignore_ascii_case(actor_name)
        }) {
            scene.current_participant_index = actor_idx;
            let p = &mut scene.participants[actor_idx];
            if is_bonus_action {
                p.actions_budget.bonus_action_used = true;
            } else {
                p.actions_budget.action_used = true;
            }
            if is_dash {
                p.actions_budget.movement_remaining_ft += 30;
            }
        }

        // 3. Prompt para o Agente de Ambiente (com fallback determinístico caso offline/erro)
        let system_instruction = "Você é o Agente de Ambiente para a Cena Viva de RPG D&D 5e (Gemini Flash-Lite). \
Você joga PELO MUNDO e PELOS NPCs de forma empática, reativa e tática, estritamente fiel à PREMISSA e ao ATO ATUAL da história. Não há etapa de revisão editorial aqui; seja direto, imersivo e ágil. \
DINÂMICA DE RODADA D&D 5E (AÇÃO DO JOGADOR + JOGADA DO AMBIENTE): \
A cada turno do jogador, você DEVE obrigatoriamente: \
1. Avaliar a ação do jogador e emitir intenção mecânica D&D 5e (se for teste, ataque ou magia). \
2. FAZER A JOGADA ATIVA DO AMBIENTE OU DOS NPCs DA HISTÓRIA! O mundo NUNCA fica inerte: os monstros/antagonistas da trama atacam, se reposicionam, emboscam; os NPCs respondem verbalmente, fogem ou sacam armas; o ambiente reage. \
Você DEVE responder APENAS com um JSON puro no seguinte formato:
{
  \"player_resolution_text\": \"Descreva como a ação do jogador se desenrola no ambiente\",
  \"environment_active_turn\": \"A JOGADA ATIVA DO AMBIENTE/NPCs: Descreva a ação ou fala dos inimigos/NPCs da história em resposta direta\",
  \"mechanical_intent\": {
    \"action\": \"skill_check\" | \"ability_check\" | \"attack\" | null,
    \"actor\": \"Nome do Jogador\",
    \"skill\": \"Persuasion\" | \"Intimidation\" | \"Athletics\" | \"Investigation\" | \"Survival\" | \"Perception\" | etc,
    \"ability\": \"Strength\" | \"Dexterity\" | etc,
    \"dc\": 14,
    \"target\": \"id_do_alvo\"
  },
  \"completed_objective_id\": \"id_do_objetivo_se_cumprido_neste_turno\" | null,
  \"party_departed\": true | false,
  \"memory_impacts\": [
    {
      \"entity_id\": \"id_da_entidade_afetada\",
      \"description\": \"o que aconteceu\",
      \"psychological_impact\": \"como o npc se sente\",
      \"dc_adj\": 0
    }
  ]
}";

        let live_hostiles_names: Vec<String> = scene
            .participants
            .iter()
            .filter(|p| !p.is_player && p.is_hostile && !p.is_dead)
            .map(|p| format!("{} (HP: {}/{}, CA: {})", p.name, p.current_hp, p.max_hp, p.armor_class))
            .collect();

        let prompt = format!(
            "PREMISSA DA CAMPANHA:\n{}\nSINOPSE DO ATO ATUAL:\n{}\n\nCENA VIVA ATUAL:\nTítulo: {}\nLocal: {}\nRodada: {}\nEm Combate: {}\nInimigos Hostis Vivos: {:?}\nObjetivos:\n{}\n\nENTIDADES PRESENTES E MEMÓRIAS:\n{}\n\nAÇÃO DO JOGADOR:\nAtor: {} ({})\nVia Utilizada: {}\nID da Ação: {:?}\nDescrição/Fala: \"{}\"",
            campaign_premise,
            current_act_synopsis,
            scene.title,
            scene.location,
            scene.round_number,
            scene.in_combat,
            live_hostiles_names,
            serde_json::to_string_pretty(&objectives_json)?,
            serde_json::to_string_pretty(&entity_contexts)?,
            actor_name,
            actor_id,
            via_type,
            action_id,
            player_input
        );

        let parsed: Value = match self.gemini.generate_narrative(&prompt, system_instruction).await {
            Ok(raw) => {
                let clean_json = Self::extract_json(&raw);
                serde_json::from_str(&clean_json).unwrap_or_else(|_| {
                    Self::build_offline_parsed_json(actor_name, player_input, via_type)
                })
            }
            Err(e) => {
                eprintln!(
                    "[ENVIRONMENT_AGENT] Aviso ao chamar Gemini API: {}. Executando motor tático determinístico offline.",
                    e
                );
                Self::build_offline_parsed_json(actor_name, player_input, via_type)
            }
        };

        let mut player_text = parsed["player_resolution_text"]
            .as_str()
            .or_else(|| parsed["narrative_reaction"].as_str())
            .unwrap_or("Você executa sua ação tática no cenário.")
            .to_string();

        let environment_active_turn = parsed["environment_active_turn"]
            .as_str()
            .unwrap_or("As forças hostis no campo de batalha reagem imediatamente à sua movimentação!")
            .to_string();

        let mut mechanical_intent = if parsed["mechanical_intent"].is_object() {
            Some(parsed["mechanical_intent"].clone())
        } else {
            None
        };

        // 4. Classificação Mecânica Determinística (Fuga, Cura, Ataque/Magia Ofensiva ou Teste de Perícia)
        let is_flee = act_id_lower.contains("flee")
            || act_id_lower.contains("retreat")
            || act_id_lower.contains("fugir")
            || act_id_lower.contains("retirada")
            || input_lower.contains("fugir")
            || input_lower.contains("fuga")
            || input_lower.contains("retirada tática")
            || input_lower.contains("retirada tatica")
            || input_lower.contains("recuar da batalha");

        let is_heal = !is_flee
            && (act_id_lower.contains("heal")
                || act_id_lower.contains("cure")
                || act_id_lower.contains("second_wind")
                || act_id_lower.contains("potion")
                || input_lower.contains("curar ferimentos")
                || input_lower.contains("palavra curativa")
                || input_lower.contains("retomar o fôlego")
                || input_lower.contains("retomar o folego")
                || input_lower.contains("poção de cura")
                || input_lower.contains("pocao de cura")
                || input_lower.contains("imposição das mãos")
                || input_lower.contains("imposicao das maos"));

        let is_buff_only = input_lower.contains("bênção")
            || input_lower.contains("bencao")
            || input_lower.contains("escudo da fé")
            || input_lower.contains("escudo da fe")
            || input_lower.contains("esquivar")
            || act_id_lower.contains("dodge");

        let is_spell_attack = !is_heal
            && !is_buff_only
            && (act_id_lower.contains("spell")
                || act_id_lower.contains("cantrip")
                || input_lower.contains("conjurar")
                || input_lower.contains("magia")
                || input_lower.contains("bola de fogo")
                || input_lower.contains("mísseis")
                || input_lower.contains("misseis")
                || input_lower.contains("raio")
                || input_lower.contains("rajada")
                || input_lower.contains("chama")
                || input_lower.contains("trovão")
                || input_lower.contains("trovao"));

        let is_weapon_or_offensive_attack = !is_flee
            && !is_heal
            && !is_buff_only
            && !is_dash
            && (is_spell_attack
                || act_id_lower.contains("attack")
                || act_id_lower.contains("strike")
                || act_id_lower.contains("smite")
                || (via_type == "via1"
                    && (input_lower.contains("ataque")
                        || input_lower.contains("atacar")
                        || input_lower.contains("golpe")
                        || input_lower.contains("disparo")
                        || input_lower.contains("espada")
                        || input_lower.contains("arco")
                        || input_lower.contains("adaga")
                        || input_lower.contains("machado")
                        || input_lower.contains("martelo")
                        || input_lower.contains("lança")
                        || input_lower.contains("punho")))
                || input_lower.contains("ataco")
                || input_lower.contains("atacar")
                || input_lower.contains("golpeio")
                || input_lower.contains("disparo contra")
                || mechanical_intent
                    .as_ref()
                    .and_then(|m| m["action"].as_str())
                    .map(|a| a == "attack")
                    .unwrap_or(false));

        let mut rules_result: Option<Value> = None;
        let mut fled_successfully = false;

        if is_flee {
            let mut rng = rand::thread_rng();
            let d20: i32 = rng.gen_range(1..=20);
            let mod_val: i32 = 3;
            let total = d20 + mod_val;
            let dc = 12;
            let is_success = d20 == 20 || (d20 != 1 && total >= dc);

            if is_success {
                scene.in_combat = false;
                fled_successfully = true;
                player_text = format!(
                    "{}\n\n🏃 **[RETIRADA TÁTICA BEM-SUCEDIDA! Teste de Destreza/Atletismo: d20={}+{} ({}) vs CD {} => SUCESSO! O grupo rompe o engajamento e recua em segurança!]**",
                    player_text, d20, mod_val, total, dc
                );
            } else {
                player_text = format!(
                    "{}\n\n⚠️ **[FALHA NA RETIRADA TÁTICA! Teste de Destreza/Atletismo: d20={}+{} ({}) vs CD {} => FALHA! Os inimigos cortam a rota de fuga e o combate continua!]**",
                    player_text, d20, mod_val, total, dc
                );
            }

            let flee_intent = json!({
                "action": "flee",
                "actor": actor_name,
                "actor_id": actor_id,
                "skill": "Athletics",
                "dc": dc
            });
            mechanical_intent = Some(flee_intent);
            rules_result = Some(json!({
                "status": "success",
                "action": "flee",
                "actor": actor_name,
                "skill": "Athletics/Stealth",
                "d20_roll": d20,
                "modifier": mod_val,
                "total": total,
                "dc": dc,
                "is_success": is_success,
                "combat_ended": is_success
            }));
        } else if is_heal {
            let mut rng = rand::thread_rng();
            let roll_d8: i32 = rng.gen_range(1..=8);
            let heal_amount = roll_d8 + 3;
            let mut new_hp = 12;
            let mut max_hp = 12;

            if let Some(p) = scene.participants.iter_mut().find(|p| {
                p.id == actor_id || p.name.eq_ignore_ascii_case(actor_name)
            }) {
                max_hp = p.max_hp.max(12);
                p.current_hp = (p.current_hp + heal_amount).min(max_hp);
                if p.current_hp > 0 {
                    p.is_dead = false;
                }
                new_hp = p.current_hp;
            }

            Self::sync_player_hp_to_store(entity_store, actor_id, actor_name, new_hp);

            player_text = format!(
                "{} [💚 D&D 5e Cura (1d8+3): +{} PV => {}/{} PV]",
                player_text, heal_amount, new_hp, max_hp
            );

            mechanical_intent = Some(json!({
                "action": "heal",
                "actor": actor_name,
                "actor_id": actor_id
            }));
            rules_result = Some(json!({
                "status": "success",
                "action": "heal",
                "actor": actor_name,
                "target": actor_name,
                "d20_roll": roll_d8,
                "modifier": 3,
                "total": heal_amount,
                "heal_amount": heal_amount,
                "target_hp_remaining": new_hp,
                "target_max_hp": max_hp,
                "is_success": true
            }));
        } else if is_weapon_or_offensive_attack && scene.participants.iter().any(|p| !p.is_player && p.is_hostile) {
            let target_hint = mechanical_intent
                .as_ref()
                .and_then(|m| m["target"].as_str())
                .filter(|t| !t.is_empty() && *t != "id_do_alvo")
                .unwrap_or(player_input);

            let atk_payload = json!({
                "action": "attack",
                "actor": actor_name,
                "actor_id": actor_id,
                "target": target_hint,
                "is_spell": is_spell_attack
            });
            mechanical_intent = Some(atk_payload.clone());

            let resolved = RulesResolver::resolve_with_scene(
                &atk_payload,
                state,
                Some(scene),
                Some(entity_store),
            );

            let d20_roll = resolved["d20_roll"].as_i64().unwrap_or(10);
            let modifier = resolved["modifier"].as_i64().unwrap_or(4);
            let total_hit = resolved["total_to_hit"].as_i64().unwrap_or(14);
            let target_ac = resolved["target_ac"].as_i64().unwrap_or(13);
            let is_hit = resolved["is_hit"].as_bool().unwrap_or(false);
            let is_crit = resolved["is_critical"].as_bool().unwrap_or(false);
            let dmg = resolved["damage"].as_i64().unwrap_or(0);
            let target_name = resolved["target"].as_str().unwrap_or("Inimigo");
            let hp_rem = resolved["target_hp_remaining"].as_i64().unwrap_or(0);
            let hp_max = resolved["target_max_hp"].as_i64().unwrap_or(12);
            let killed = resolved["target_killed"].as_bool().unwrap_or(false);

            if is_hit {
                let hit_label = if is_crit { "ACERTO CRÍTICO" } else { "ACERTO" };
                let kill_label = if killed { " — 💀 INIMIGO ABATIDO!" } else { "" };
                player_text = format!(
                    "{} [⚔️ D&D 5e Ataque contra {}: d20={}+{} ({}) vs CA {} => {}! Dano: -{} PV (HP: {}/{}{})]",
                    player_text, target_name, d20_roll, modifier, total_hit, target_ac, hit_label, dmg, hp_rem, hp_max, kill_label
                );
            } else {
                player_text = format!(
                    "{} [🛡️ D&D 5e Ataque contra {}: d20={}+{} ({}) vs CA {} => ERROU! (HP: {}/{})]",
                    player_text, target_name, d20_roll, modifier, total_hit, target_ac, hp_rem, hp_max
                );
            }

            rules_result = Some(resolved);
        } else {
            // Se for Via 2 sem mechanical_intent da IA, extrai da opção de cenário sugerida
            if mechanical_intent.is_none() && via_type == "via2" {
                if let Some(aid) = action_id {
                    if let Some(opt) = scene.scenario_options.iter().find(|o| o["id"].as_str() == Some(aid)) {
                        let skill = opt["skillCheckSuggested"]["skill"].as_str().unwrap_or("Perception");
                        let dc = opt["skillCheckSuggested"]["dcEstimate"].as_i64().unwrap_or(13);
                        mechanical_intent = Some(json!({
                            "action": "skill_check",
                            "actor": actor_name,
                            "actor_id": actor_id,
                            "skill": skill,
                            "dc": dc
                        }));
                    }
                }
            }

            if let Some(ref mut intent) = mechanical_intent {
                if let Some(obj) = intent.as_object_mut() {
                    obj.insert("actor".to_string(), json!(actor_name));
                    obj.insert("actor_id".to_string(), json!(actor_id));
                }
                let mut resolved = RulesResolver::resolve_with_scene(
                    intent,
                    state,
                    Some(scene),
                    Some(entity_store),
                );

                if let Some(target_id) = intent["target"].as_str() {
                    if let Ok(Some(target_sheet)) = entity_store.get_world_entity(target_id) {
                        let (dc_adj, dice_mode) = target_sheet.get_modifiers_against(actor_id);
                        if dc_adj != 0 || dice_mode.is_some() {
                            resolved["memory_modifier_applied"] = json!({
                                "dc_adjustment": dc_adj,
                                "dice_mode": dice_mode
                            });
                        }
                    }
                }

                let d20_roll = resolved["d20_roll"].as_i64().unwrap_or(10);
                let total = resolved["total"]
                    .as_i64()
                    .or_else(|| resolved["total_to_hit"].as_i64())
                    .unwrap_or(12);
                let is_success = resolved["is_success"]
                    .as_bool()
                    .or_else(|| resolved["is_hit"].as_bool())
                    .unwrap_or(true);

                player_text = format!(
                    "{} [D&D 5e: Rolagem d20={} (Total={}) => {}]",
                    player_text,
                    d20_roll,
                    total,
                    if is_success { "SUCESSO" } else { "FALHA" }
                );

                rules_result = Some(resolved);
            }
        }

        // 5. Checagem de Vitória Tática em Combate após ação do jogador
        let mut victory_achieved = Self::check_and_apply_combat_victory(scene);

        // 6. Auto-Avanço de Turno e Execução Mecânica Real dos Inimigos / Aliados
        let mut enemy_combat_logs = Vec::new();
        let mut enemy_turns_json = Vec::new();

        if !is_bonus_action {
            let (logs, turns, ally_victory) =
                Self::advance_to_next_player_and_run_npcs(scene, entity_store);
            enemy_combat_logs = logs;
            enemy_turns_json = turns;
            if ally_victory {
                victory_achieved = true;
            }
        }

        if !enemy_turns_json.is_empty() {
            if let Some(ref mut res_obj) = rules_result {
                if let Some(map) = res_obj.as_object_mut() {
                    map.insert("enemy_turns".to_string(), json!(enemy_turns_json));
                }
            } else {
                rules_result = Some(json!({
                    "status": "success",
                    "action": "environment_turn",
                    "enemy_turns": enemy_turns_json
                }));
            }
        }

        // 7. Monta a reação narrativa combinando texto da IA + logs mecânicos reais dos inimigos + banner de vitória
        let mut narrative_reaction = format!(
            "{}\n\n⚔️ [TURNO DO AMBIENTE / NPCs]:\n{}",
            player_text, environment_active_turn
        );

        if !enemy_combat_logs.is_empty() {
            narrative_reaction.push_str("\n\n🎲 **[ROLAGENS TÁTICAS DOS INIMIGOS E ALIADOS]:**\n");
            for line in &enemy_combat_logs {
                narrative_reaction.push_str(&format!("- {}\n", line));
            }
        }

        if victory_achieved {
            narrative_reaction.push_str(
                "\n\n🏆 **[VITÓRIA TÁTICA! Todos os inimigos hostis foram abatidos! O campo de batalha está seguro para exploração e saque.]**",
            );
        }

        // 8. Checagem de objetivos cumpridos
        let mut completed_obj_id = None;
        if let Some(obj_id) = parsed["completed_objective_id"].as_str() {
            if !scene.in_combat || !scene.has_live_hostiles() {
                scene.mark_objective_completed(obj_id);
                completed_obj_id = Some(obj_id.to_string());
            }
        }

        // 9. Checagem de resolução da cena respeitando a Trava de Batalha (só sai se derrotou todos os hostis ou se passou no teste de Fuga CD 12)
        let mut scene_res = if fled_successfully {
            Some(SceneResolutionReason::PartyDeparted)
        } else if !scene.participants.iter().any(|p| p.is_player && !p.is_dead && p.current_hp > 0) {
            scene.in_combat = false;
            Some(SceneResolutionReason::PartyDefeated)
        } else {
            scene.check_completion()
        };

        if scene_res.is_none()
            && (!scene.in_combat || !scene.has_live_hostiles())
            && parsed["party_departed"].as_bool().unwrap_or(false)
        {
            scene_res = Some(SceneResolutionReason::PartyDeparted);
        }

        // 10. Registra turno no histórico da Cena Viva
        let turn_record = SceneInteractionTurn {
            turn_number: scene.turn_history.len() as u32 + 1,
            actor_id: actor_id.to_string(),
            actor_name: actor_name.to_string(),
            via_type: via_type.to_string(),
            player_input: player_input.to_string(),
            mechanical_intent: mechanical_intent.clone(),
            rules_result: rules_result.clone(),
            environment_reaction: narrative_reaction.clone(),
        };
        scene.record_turn(turn_record);

        let memory_impacts_to_apply = parsed["memory_impacts"]
            .as_array()
            .cloned()
            .unwrap_or_default();

        Ok(EnvironmentTurnOutput {
            narrative_reaction,
            mechanical_intent,
            rules_result,
            completed_objective_id: completed_obj_id,
            scene_resolution: scene_res,
            memory_impacts_to_apply,
        })
    }

    fn extract_json(raw: &str) -> String {
        let trimmed = raw.trim();
        if let Some(start) = trimmed.find('{') {
            if let Some(end) = trimmed.rfind('}') {
                return trimmed[start..=end].to_string();
            }
        }
        trimmed.to_string()
    }

    fn build_offline_parsed_json(actor_name: &str, player_input: &str, via_type: &str) -> Value {
        let lower = player_input.to_lowercase();
        let is_investigation =
            lower.contains("rastr") || lower.contains("investig") || lower.contains("examin");
        let is_persuasion = lower.contains("convenc")
            || lower.contains("fal")
            || lower.contains("pergunt")
            || lower.contains("interrog");

        let intent = if is_investigation {
            json!({
                "action": "skill_check",
                "actor": actor_name,
                "skill": "Investigation",
                "dc": 12
            })
        } else if is_persuasion {
            json!({
                "action": "skill_check",
                "actor": actor_name,
                "skill": "Persuasion",
                "dc": 13
            })
        } else {
            Value::Null
        };

        let env_active_turn = if is_investigation {
            "Das sombras do terreno, olhos hostis brilham na penumbra enquanto os inimigos ajustam sua formação tática!"
        } else if is_persuasion {
            "As figuras presentes reagem com tensão às suas palavras enquanto o ambiente permanece carregado de perigo."
        } else {
            "Os combatentes no campo de batalha cerram fileiras e respondem imediatamente com ferro e movimento tático!"
        };

        json!({
            "player_resolution_text": format!("{} executa sua manobra ({}) com determinação: \"{}\".", actor_name, via_type, player_input),
            "environment_active_turn": env_active_turn,
            "mechanical_intent": intent,
            "completed_objective_id": null,
            "party_departed": false,
            "memory_impacts": []
        })
    }
}
