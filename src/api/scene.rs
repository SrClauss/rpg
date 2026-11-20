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
    build_player_participant, ensure_tactical_scene_hydrated, expand_and_build_npc_participants,
    hydrate_campaign_active_scene, validate_actor_turn_permission,
};
use crate::api::models::{
    ClaimItemRequest, EndTurnRequest, SceneMoveRequest, StartSceneRequest,
};
use crate::api::state::ServerState;
use crate::llm::{EnvironmentAgent, SceneCloser};
use crate::narrative::chronicle::{ChronicleEntry, ChronicleEntryType};
use crate::narrative::entity_store::{EntityDndStats, EntityType, WorldEntitySheet};
use crate::narrative::living_scene::{
    SceneObjective, SceneResolutionReason,
};

pub fn scene_router() -> Router<ServerState> {
    Router::new()
        .route("/api/campaigns/{id}/scene/start", post(start_living_scene))
        .route("/api/campaigns/{id}/scene/move", post(move_scene_participant))
        .route("/api/campaigns/{id}/scene/end-turn", post(end_scene_turn))
        .route("/api/campaigns/{id}/scene/claim_item", post(claim_scene_item))
        .route("/api/campaigns/{id}/scene/resolve", post(resolve_scene))
}

pub async fn start_living_scene(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(req): Json<StartSceneRequest>,
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

    let mut camp = camp_arc.write().await;

    let mut participants = Vec::new();
    let existing_players = camp
        .get_entity_store()
        .and_then(|s| s.list_player_entities())
        .unwrap_or_default();

    if let Some(ref m) = camp.manifest {
        for (i, p_id) in m.player_ids.iter().enumerate() {
            let slot_opt = m.player_slots.iter().find(|s| &s.player_id == p_id);
            let p_name = existing_players
                .iter()
                .find(|pe| &pe.id == p_id)
                .map(|pe| pe.name.clone())
                .or_else(|| slot_opt.map(|s| s.player_name.clone()))
                .unwrap_or_else(|| format!("Jogador {}", i + 1));
            let p_class = slot_opt
                .map(|s| s.class_name.as_str())
                .unwrap_or("Fighter");

            participants.push(build_player_participant(
                p_id,
                &p_name,
                p_class,
                i,
                &existing_players,
            ));
        }
    }

    let premise = camp
        .manifest
        .as_ref()
        .map(|m| m.premise.clone())
        .unwrap_or_default();
    let literary_style = camp
        .manifest
        .as_ref()
        .map(|m| m.literary_style.clone())
        .unwrap_or_default();
    let (act_number, act_title, act_synopsis, key_milestones) = camp
        .manifest
        .as_ref()
        .and_then(|m| m.hidden_skeleton.current_act())
        .map(|a| {
            (
                a.act_number,
                a.title.clone(),
                a.synopsis.clone(),
                a.expected_milestones.clone(),
            )
        })
        .unwrap_or_else(|| (1, "Ato I".to_string(), premise.clone(), Vec::new()));

    let latest_chronicle_prose = camp
        .get_chronicle_book()
        .and_then(|b| b.get_all_entries())
        .ok()
        .and_then(|entries| entries.last().map(|e| e.narrative_prose.clone()))
        .unwrap_or_else(|| act_synopsis.clone());

    let mut new_scene = state
        .environment_agent
        .generate_dynamic_scene(
            &campaign_id,
            &premise,
            &literary_style,
            act_number,
            &act_title,
            &act_synopsis,
            &key_milestones,
            &latest_chronicle_prose,
            participants,
        )
        .await;

    if let Some(custom_title) = req.title.filter(|t| !t.trim().is_empty()) {
        new_scene.title = custom_title;
    }
    if let Some(custom_loc) = req.location.filter(|l| !l.trim().is_empty()) {
        new_scene.location = custom_loc;
    }
    if !req.objectives.is_empty() {
        new_scene.objectives = req
            .objectives
            .into_iter()
            .enumerate()
            .map(|(i, desc)| SceneObjective {
                id: format!("obj_{}", i + 1),
                description: desc,
                completed: false,
                completion_trigger: None,
            })
            .collect();
    }

    let custom_monsters = camp
        .manifest
        .as_ref()
        .map(|m| m.custom_monsters.clone())
        .unwrap_or_default();
    let homebrew_bosses = camp
        .manifest
        .as_ref()
        .map(|m| m.hidden_skeleton.homebrew_bosses.clone())
        .unwrap_or_default();
    let world_entities = camp
        .get_entity_store()
        .and_then(|s| s.list_world_entities())
        .unwrap_or_default();

    let npc_participants = expand_and_build_npc_participants(
        &new_scene.active_npcs,
        act_number,
        &custom_monsters,
        &homebrew_bosses,
        &world_entities,
    );

    if let Ok(store) = camp.get_entity_store() {
        for np in &npc_participants {
            if store.get_world_entity(&np.id).ok().flatten().is_none() {
                let mut sheet = WorldEntitySheet::new(
                    &np.id,
                    &np.name,
                    EntityType::Npc,
                    &format!("Entidade ativa na cena '{}' ({})", new_scene.title, new_scene.location),
                );
                let mut dnd = EntityDndStats::default();
                dnd.max_hp = np.max_hp;
                dnd.current_hp = np.current_hp;
                dnd.armor_class = np.armor_class;
                sheet.dnd_stats = Some(dnd);
                let _ = store.save_world_entity(&sheet);
            }
        }
    }

    new_scene.participants.extend(npc_participants);
    ensure_tactical_scene_hydrated(&mut new_scene, &existing_players);

    camp.active_scene = Some(new_scene.clone());
    let _ = camp.save_active_scene();

    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&camp.room_code) {
        let _ = tx.send(
            json!({
                "type": "SCENE_STARTED",
                "scene": new_scene
            })
            .to_string(),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Cena Viva dinâmica iniciada com sucesso a partir do Esqueleto Narrativo",
            "scene": new_scene
        })),
    )
}

pub async fn move_scene_participant(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(req): Json<SceneMoveRequest>,
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

    let mut camp = camp_arc.write().await;
    hydrate_campaign_active_scene(&mut camp);
    let room_code = camp.room_code.clone();

    if let Err((status, err_json)) = validate_actor_turn_permission(
        camp.manifest.as_ref(),
        &req.actor_id,
        req.player_id.as_deref(),
        req.player_code.as_deref(),
    ) {
        return (status, Json(err_json));
    }

    let scene = match camp.active_scene.as_mut() {
        Some(s) => s,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "status": "error", "message": "Nenhuma cena ativa para movimentação" })),
            )
        }
    };

    let in_combat = scene.in_combat && scene.has_live_hostiles();
    let p_idx_opt = scene
        .participants
        .iter()
        .position(|p| p.id == req.actor_id || p.name.eq_ignore_ascii_case(&req.actor_id))
        .or_else(|| {
            let cur = scene.current_participant_index;
            if scene.participants.get(cur).map(|p| p.is_player).unwrap_or(false) {
                Some(cur)
            } else {
                scene.participants.iter().position(|p| p.is_player && !p.is_dead)
            }
        });

    let p_idx = match p_idx_opt {
        Some(idx) => idx,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "status": "error", "message": "Participante não encontrado na cena" })),
            )
        }
    };

    let target_x = req.x.clamp(0, 60);
    let target_y = req.y.clamp(0, 40);
    let (cur_x, cur_y) = scene.participants[p_idx].position.unwrap_or((10, 20));
    let dx = (target_x - cur_x) as f64;
    let dy = (target_y - cur_y) as f64;
    let dist_ft = (dx * dx + dy * dy).sqrt().round() as u32;

    if !in_combat {
        scene.participants[p_idx].position = Some((target_x, target_y));
        scene.participants[p_idx].actions_budget.movement_remaining_ft = 30;
    } else {
        let rem = scene.participants[p_idx].actions_budget.movement_remaining_ft;
        if rem == 0 && dist_ft > 0 {
            let scene_clone = scene.clone();
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Sem deslocamento restante neste turno (0 ft). Use Disparada (Dash) ou encerre seu turno.",
                    "active_scene": scene_clone
                })),
            );
        }

        if dist_ft <= rem {
            scene.participants[p_idx].position = Some((target_x, target_y));
            scene.participants[p_idx].actions_budget.movement_remaining_ft = rem.saturating_sub(dist_ft);
        } else if dist_ft > 0 {
            let ratio = (rem as f64) / (dist_ft as f64);
            let clamped_x = (cur_x as f64 + dx * ratio).round() as i32;
            let clamped_y = (cur_y as f64 + dy * ratio).round() as i32;
            scene.participants[p_idx].position = Some((clamped_x.clamp(0, 60), clamped_y.clamp(0, 40)));
            scene.participants[p_idx].actions_budget.movement_remaining_ft = 0;
        }
    }

    let scene_clone = scene.clone();
    let _ = camp.save_active_scene();

    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&room_code) {
        let _ = tx.send(
            json!({
                "type": "SCENE_UPDATED",
                "active_scene": scene_clone
            })
            .to_string(),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "active_scene": scene_clone
        })),
    )
}

pub async fn end_scene_turn(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(req): Json<EndTurnRequest>,
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

    let mut camp = camp_arc.write().await;
    hydrate_campaign_active_scene(&mut camp);
    let room_code = camp.room_code.clone();

    let entity_store = match camp.get_entity_store() {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": e.to_string() })),
            )
        }
    };

    let mut enemy_logs = Vec::new();
    let mut enemy_turns = Vec::new();

    let manifest_clone = camp.manifest.clone();

    if let Some(ref mut scene) = camp.active_scene {
        if let Some(ref aid) = req.actor_id {
            if let Some(idx) = scene
                .participants
                .iter()
                .position(|p| &p.id == aid || p.name.eq_ignore_ascii_case(aid))
            {
                scene.current_participant_index = idx;
            }
        }

        let prev_actor_name = scene
            .current_participant()
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "Herói".to_string());
        let prev_actor_id = scene
            .current_participant()
            .map(|p| p.id.clone())
            .unwrap_or_else(|| "player_1".to_string());

        if let Err((status, err_json)) = validate_actor_turn_permission(
            manifest_clone.as_ref(),
            &prev_actor_id,
            req.player_id.as_deref(),
            req.player_code.as_deref(),
        ) {
            return (status, Json(err_json));
        }

        let (logs, turns, victory_achieved) =
            EnvironmentAgent::advance_to_next_player_and_run_npcs(scene, &entity_store);
        enemy_logs = logs;
        enemy_turns = turns;

        if !enemy_logs.is_empty() || victory_achieved {
            let mut reaction = String::from("⏭️ **Turno encerrado pelo jogador.**\n\n🎲 **[ROLAGENS TÁTICAS DOS INIMIGOS E ALIADOS]:**\n");
            for l in &enemy_logs {
                reaction.push_str(&format!("- {}\n", l));
            }
            if victory_achieved {
                let (tot_xp, hero_xp) = EnvironmentAgent::award_combat_xp(scene, &entity_store);
                reaction.push_str(&format!(
                    "\n🏆 **[VITÓRIA TÁTICA! Todos os inimigos hostis foram abatidos!]**\n⭐ **[RECOMPENSA D&D 5e]: +{} XP concedidos para cada herói (Total do combate: {} XP)!**\n🎒 *O campo de batalha está seguro para exploração e saque.*",
                    hero_xp, tot_xp
                ));
            }
            let turn_rec = crate::narrative::living_scene::SceneInteractionTurn {
                turn_number: scene.turn_history.len() as u32 + 1,
                actor_id: prev_actor_id,
                actor_name: prev_actor_name,
                via_type: crate::narrative::living_scene::ViaType::EndTurn,
                player_input: "Encerrar Turno".to_string(),
                mechanical_intent: None,
                rules_result: Some(json!({
                    "status": "success",
                    "action": "end_turn",
                    "enemy_turns": enemy_turns
                })),
                environment_reaction: reaction,
            };
            scene.record_turn(turn_rec);
        }
    }

    let _ = camp.save_active_scene();
    let updated_players = entity_store.list_player_entities().unwrap_or_default();
    let scene_clone = camp.active_scene.clone();

    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&room_code) {
        let _ = tx.send(
            json!({
                "type": "SCENE_UPDATED",
                "active_scene": scene_clone,
                "player_entities": updated_players,
                "enemy_turns": enemy_turns,
            })
            .to_string(),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "active_scene": scene_clone,
            "player_entities": updated_players,
            "enemy_turns": enemy_turns,
            "enemy_combat_log": enemy_logs,
        })),
    )
}

pub async fn claim_scene_item(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(req): Json<ClaimItemRequest>,
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

    let mut camp = camp_arc.write().await;
    let room_code = camp.room_code.clone();

    let mut claimed_item_clone = None;
    let mut scene_chapter = 1;
    let mut scene_title = String::new();

    if let Some(ref mut scene) = camp.active_scene {
        if let Some(item) = scene.scene_loot.iter_mut().find(|i| i.id == req.item_id) {
            if item.is_claimed {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "status": "error", "message": "Item já foi reivindicado anteriormente." })),
                );
            }

            item.is_claimed = true;
            item.claimed_by = Some(req.player_name.clone());

            claimed_item_clone = Some(item.clone());
            scene_chapter = scene.chapter;
            scene_title = scene.title.clone();
        }
    }

    if let Some(claimed_item) = claimed_item_clone {
        let _ = camp.save_active_scene();

        if let Ok(store) = camp.get_entity_store() {
            if let Ok(mut players) = store.list_player_entities() {
                if let Some(p_sheet) = players.iter_mut().find(|p| p.name == req.player_name) {
                    let mut inv = p_sheet
                        .custom_flags
                        .get("inventory")
                        .and_then(|v| v.as_array().cloned())
                        .unwrap_or_default();
                    inv.push(json!({
                        "name": claimed_item.name,
                        "quantity": 1,
                        "weightLbs": 1,
                        "isEquipped": false,
                        "rarity": claimed_item.rarity,
                        "description": claimed_item.description
                    }));
                    p_sheet.custom_flags.insert("inventory".to_string(), json!(inv));
                    let _ = store.save_player_entity(p_sheet);
                }
            }
        }

        if let Ok(mut book) = camp.get_chronicle_book() {
            let note = format!(
                "🎒 O herói {} encontrou e coletou o item: '{}' ({}) - {}!",
                req.player_name,
                claimed_item.name,
                claimed_item.rarity,
                claimed_item.description
            );
            let mut entry = ChronicleEntry::new(
                scene_chapter,
                &scene_title,
                ChronicleEntryType::MechanicalAction,
                &note,
                vec![],
                vec![req.player_name.clone()],
            );
            let _ = book.append_entry(&mut entry);
        }

        let active_scene_clone = camp.active_scene.clone();

        let channels = state.channels.read().await;
        if let Some(tx) = channels.get(&room_code) {
            let _ = tx.send(
                json!({
                    "type": "ITEM_CLAIMED",
                    "item": claimed_item,
                    "player_name": req.player_name,
                    "active_scene": active_scene_clone
                })
                .to_string(),
            );
        }

        return (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": format!("Item '{}' coletado com sucesso por {}!", claimed_item.name, req.player_name),
                "item": claimed_item,
                "active_scene": active_scene_clone
            })),
        );
    }

    (
        StatusCode::NOT_FOUND,
        Json(json!({ "status": "error", "message": "Item ou cena ativa não encontrados" })),
    )
}

pub async fn resolve_scene(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(payload): Json<Value>,
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

    let mut camp = camp_arc.write().await;
    hydrate_campaign_active_scene(&mut camp);

    if camp.active_scene.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "status": "error", "message": "Nenhuma cena ativa para concluir" })),
        );
    }

    let reason_str = payload["reason"].as_str().unwrap_or("objectives_completed");
    let force_close = payload["force"].as_bool().unwrap_or(false);
    let reason = match reason_str {
        "party_departed" | "flee" => SceneResolutionReason::PartyDeparted,
        "party_defeated" => SceneResolutionReason::PartyDefeated,
        _ => SceneResolutionReason::ObjectivesCompleted,
    };

    if let Some(ref sc) = camp.active_scene {
        if sc.in_combat
            && sc.has_live_hostiles()
            && reason != SceneResolutionReason::PartyDefeated
            && !force_close
        {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Combate ativo em andamento! Derrote todos os inimigos hostis ou realize uma ação mecânica de Fuga / Retirada Tática (CD 12) antes de sair da cena.",
                    "active_scene": sc
                })),
            );
        }
    }

    let mut active_scene = match camp.active_scene.take() {
        Some(s) => s,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "code": "NO_ACTIVE_SCENE",
                    "message": "Nenhuma cena ativa para concluir"
                })),
            );
        }
    };

    let resolution = match SceneCloser::close_and_consolidate(
        &mut active_scene,
        reason,
        &mut camp,
        &state.orchestrator,
        &state.skeleton_agent,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("[SCENE_RESOLVE] Erro ao consolidar cena: {}", e);
            camp.active_scene = Some(active_scene);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": e.to_string() })),
            );
        }
    };

    camp.active_scene = None;
    let _ = camp.save_active_scene();

    let room_code = camp.room_code.clone();
    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&room_code) {
        let _ = tx.send(
            json!({
                "type": "SCENE_RESOLVED",
                "resolution": resolution,
                "active_scene": null,
            })
            .to_string(),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Cena concluída e consolidada na Crônica!",
            "resolution": resolution,
        })),
    )
}
