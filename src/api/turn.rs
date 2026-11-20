use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde_json::{json, Value};

use crate::api::campaign::get_or_load_campaign;
use crate::api::helpers::{
    build_player_participant, hydrate_campaign_active_scene, validate_actor_turn_permission,
};
use crate::api::models::TurnActionRequest;
use crate::api::state::{get_turn_lock, ServerState};
use crate::llm::SceneCloser;
use crate::narrative::living_scene::{LivingScene, SceneObjective, SceneResolutionReason};

pub fn turn_router() -> Router<ServerState> {
    Router::new()
        // Rotas canônicas unificadas de ação de turno
        .route("/api/campaigns/{id}/turn", post(process_campaign_turn))
        .route("/api/campaigns/{id}/action/turn", post(process_campaign_turn))
}

pub async fn process_campaign_turn(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(req): Json<TurnActionRequest>,
) -> (StatusCode, Json<Value>) {
    let camp_arc = match get_or_load_campaign(&state, &campaign_id).await {
        Some(c) => c,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "status": "error", "message": "Campanha não encontrada" })),
            )
        }
    };

    // 1. Validação de Concorrência: Trava de Turno por Campanha/Sala
    let turn_lock = get_turn_lock(&state, &campaign_id).await;
    let _turn_guard = match turn_lock.try_lock() {
        Ok(guard) => guard,
        Err(_) => {
            return (
                StatusCode::CONFLICT,
                Json(json!({
                    "status": "error",
                    "code": "TURN_IN_PROGRESS",
                    "message": "O turno atual já está sendo processado pela IA. Por favor, aguarde a conclusão."
                })),
            );
        }
    };

    let mut camp = camp_arc.write().await;
    hydrate_campaign_active_scene(&mut camp);
    let room_code = camp.room_code.clone();

    let actor_id = req.actor_id.clone().unwrap_or_else(|| "player_1".to_string());

    // 2. Validação de Permissão (Solo: irrestrito | Multiplayer: valida dono/slot ou host)
    if let Err((status, err_json)) = validate_actor_turn_permission(
        camp.manifest.as_ref(),
        &actor_id,
        req.player_id.as_deref(),
        req.player_code.as_deref(),
    ) {
        return (status, Json(err_json));
    }

    if camp.active_scene.is_none() {
        let existing_players = camp
            .get_entity_store()
            .and_then(|s| s.list_player_entities())
            .unwrap_or_default();
        let obj = vec![SceneObjective {
            id: "obj_explore".to_string(),
            description: "Investigar e avançar pelo local atual".to_string(),
            completed: false,
            completion_trigger: None,
        }];
        let p_id = actor_id.clone();
        let part = vec![build_player_participant(
            &p_id,
            &req.character_name,
            &req.class_name,
            0,
            &existing_players,
        )];
        let mut fallback_scene = LivingScene::new(
            &campaign_id,
            "Exploração",
            "Local Atual",
            1,
            obj,
            part,
        );
        fallback_scene.in_combat = false;
        camp.active_scene = Some(fallback_scene);
    }

    let mut active_scene = match camp.active_scene.take() {
        Some(s) => s,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": "error",
                    "code": "NO_ACTIVE_SCENE",
                    "message": "Nenhuma cena ativa encontrada para a campanha"
                })),
            );
        }
    };
    let entity_store = match camp.get_entity_store() {
        Ok(store) => store,
        Err(e) => {
            camp.active_scene = Some(active_scene);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "code": "ENTITY_STORE_ERROR",
                    "message": format!("Erro ao obter entity store da campanha: {}", e)
                })),
            );
        }
    };
    let player_input = req.player_text.unwrap_or_else(|| "Ação de turno".to_string());

    // Marca ação consumida se ID fornecido
    if let Some(ref act_id) = req.action_id {
        if !active_scene.consumed_scenario_options.contains(act_id) {
            active_scene.consumed_scenario_options.push(act_id.clone());
        }
    }

    let turn_output = match state
        .environment_agent
        .process_turn(
            &mut active_scene,
            &actor_id,
            &req.character_name,
            &req.via_type,
            &player_input,
            req.action_id.as_deref(),
            &mut camp,
            &entity_store,
        )
        .await
    {
        Ok(out) => out,
        Err(e) => {
            eprintln!("[TURN] Erro no processamento de turno: {}", e);
            camp.active_scene = Some(active_scene);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": e.to_string() })),
            );
        }
    };

    // Revelação orgânica de loot oculto na exploração
    let player_text_lower = player_input.to_lowercase();
    let is_searching = player_text_lower.contains("investig")
        || player_text_lower.contains("procur")
        || player_text_lower.contains("examin")
        || player_text_lower.contains("vasculh")
        || player_text_lower.contains("saque")
        || player_text_lower.contains("baú")
        || player_text_lower.contains("bau")
        || player_text_lower.contains("corpo")
        || player_text_lower.contains("fenda")
        || player_text_lower.contains("abrir")
        || req.action_id.as_deref().map(|s| s.contains("investig") || s.contains("procur")).unwrap_or(false);

    let is_check_success = turn_output.rules_result.as_ref()
        .and_then(|r| r.get("is_success"))
        .and_then(|b| b.as_bool())
        .unwrap_or(true);

    let mut final_reaction = turn_output.narrative_reaction.clone();
    if is_searching && is_check_success {
        if let Some(hidden_item) = active_scene.scene_loot.iter_mut().find(|item| item.is_hidden && !item.is_claimed) {
            hidden_item.is_hidden = false;
            let note = format!(
                "\n\n✨ [DESCOBERTA / ITEM ENCONTRADO]: Durante a investigação atenta do local, você descobriu: **{}**! ({})\n*(O item agora está visível nas descobertas da cena e pode ser coletado pelo grupo).* ",
                hidden_item.name, hidden_item.description
            );
            final_reaction.push_str(&note);
            println!(">>> [CENA VIVA] Item oculto revelado por investigação: {}", hidden_item.name);
        }
    }

    // Atualiza opções situacionais fora de combate para a nova rodada
    let active_options_count = active_scene.scenario_options.iter().filter(|o| {
        let id = o["id"].as_str().unwrap_or("");
        !active_scene.consumed_scenario_options.contains(&id.to_string())
    }).count();

    let act_boss = camp.manifest.as_ref()
        .and_then(|m| m.hidden_skeleton.current_act())
        .and_then(|a| a.boss_name.as_deref())
        .unwrap_or("o Chefe da Região");

    let wants_advance = (!active_scene.in_combat || !active_scene.has_live_hostiles())
        && (req.action_id.as_deref().map(|id| id.contains("advance") || id.contains("boss_confront") || id.contains("unlock_sector")).unwrap_or(false)
            || player_text_lower.contains("avançar para o próximo ato")
            || player_text_lower.contains("concluir cena"));

    if active_options_count < 2 {
        let round_num = active_scene.turn_history.len() + 1;
        active_scene.scenario_options.push(json!({
            "id": format!("scen_opt_boss_confront_{}", round_num),
            "title": format!("Confrontar as Patrulhas e Buscar {}", act_boss),
            "description": format!("Seguir os rastros de destruição e energia deixados para trás, marchando diretamente em direção a {}.", act_boss),
            "actionCost": "action",
            "skillCheckSuggested": { "skill": "Athletics", "dcEstimate": 13 },
            "tacticalBenefit": "Inicia o confronto decisivo com iniciativa vantajosa."
        }));
        active_scene.scenario_options.push(json!({
            "id": format!("scen_opt_unlock_sector_{}", round_num),
            "title": "Forçar passagem mecânica para o próximo setor",
            "description": "Alavancar as engrenagens emperradas e romper o bloqueio para transpor o setor.",
            "actionCost": "action",
            "skillCheckSuggested": { "skill": "Athletics", "dcEstimate": 12 },
            "tacticalBenefit": "Destrava o avanço definitivo e escapa das patrulhas menores."
        }));
        active_scene.scenario_options.push(json!({
            "id": format!("scen_opt_fortify_{}", round_num),
            "title": "Organizar emboscada e atrair os guardas",
            "description": "Criar uma distração com reflexos e som para encurralar as patrulhas.",
            "actionCost": "bonus_action",
            "skillCheckSuggested": { "skill": "Stealth", "dcEstimate": 12 },
            "tacticalBenefit": "Surpreende o inimigo na primeira rodada de combate."
        }));
    }

    // Checa se a cena foi concluída ou solicitou avanço decisivo (respeitando a Trava de Batalha)
    let mut scene_closure_result = None;
    let mut scene_resolution_final = turn_output.scene_resolution.clone();
    if wants_advance && scene_resolution_final.is_none() {
        println!(">>> [CENA VIVA] Jogador acionou avanço após combate concluído. Concluindo cena...");
        scene_resolution_final = Some(SceneResolutionReason::ObjectivesCompleted);
    }

    if let Some(reason) = scene_resolution_final {
        println!(">>> [CENA VIVA] Condição de término detectada: {:?}. Acionando SceneCloser...", reason);
        match SceneCloser::close_and_consolidate(
            &mut active_scene,
            reason,
            &mut camp,
            &state.orchestrator,
            &state.skeleton_agent,
        )
        .await
        {
            Ok(res) => {
                scene_closure_result = Some(res);
            }
            Err(e) => {
                eprintln!("[SCENE_CLOSER] Erro no fechamento da cena: {}", e);
            }
        }
    } else {
        camp.active_scene = Some(active_scene.clone());
        let _ = camp.save_active_scene();
    }

    let updated_players = entity_store.list_player_entities().unwrap_or_default();

    // Broadcast da atualização para todos os jogadores na sala
    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&room_code) {
        let _ = tx.send(
            json!({
                "type": "TURN_PROCESSED",
                "actor_id": actor_id,
                "actor_name": req.character_name,
                "via_type": req.via_type,
                "narrative_reaction": final_reaction,
                "rules_result": turn_output.rules_result,
                "active_scene": camp.active_scene,
                "player_entities": updated_players,
                "scene_closed": scene_closure_result,
            })
            .to_string(),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "narrative_reaction": final_reaction,
            "rules_result": turn_output.rules_result,
            "active_scene": camp.active_scene,
            "player_entities": updated_players,
            "scene_closed": scene_closure_result,
        })),
    )
}
