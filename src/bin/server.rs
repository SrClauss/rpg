use dotenvy::dotenv;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

use rpg_engine::api::{app_router, ServerState};
use rpg_engine::auth::UserStore;
use rpg_engine::llm::{EnvironmentAgent, NarrativeOrchestrator, SkeletonAgent};
use rpg_engine::state::CampaignState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("============================================================");
    println!("   RPG D&D 5E ENGINE — SERVIDOR MULTIPLAYER (AXUM + WS)    ");
    println!("============================================================");

    let gemini_key = env::var("GEMINI_API_KEY").unwrap_or_else(|_| "dummy_key".to_string());
    let deepseek_key = env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| "dummy_key".to_string());

    let skeleton_agent = Arc::new(SkeletonAgent::new(gemini_key.clone()));
    let environment_agent = Arc::new(EnvironmentAgent::new(gemini_key.clone()));
    let orchestrator = Arc::new(NarrativeOrchestrator::new(gemini_key, deepseek_key));
    let user_store = Arc::new(UserStore::new("data/users"));

    let server_state = ServerState {
        rooms: Arc::new(RwLock::new(HashMap::new())),
        campaigns: Arc::new(RwLock::new(HashMap::new())),
        channels: Arc::new(RwLock::new(HashMap::new())),
        turn_locks: Arc::new(RwLock::new(HashMap::new())),
        skeleton_agent,
        environment_agent,
        orchestrator,
        user_store,
    };

    // Cria sala padrão de demonstração para testes diretos
    {
        let default_camp = CampaignState::with_id_and_room("default_campaign", "0000");
        let arc_camp = Arc::new(RwLock::new(default_camp));
        server_state.rooms.write().await.insert("0000".to_string(), arc_camp.clone());
        server_state.campaigns.write().await.insert("default_campaign".to_string(), arc_camp);
        let (tx, _) = broadcast::channel(100);
        server_state.channels.write().await.insert("0000".to_string(), tx);
    }

    println!(">>> [STARTUP] Servidor pronto em estado inicial limpo (sem restauração automática de campanhas passadas).");

    // Constrói a aplicação modularizada via rpg_engine::api
    let app = app_router(server_state);

    let addr = "0.0.0.0:3001";
    println!("Servidor HTTP e WebSocket ouvindo em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod server_concurrency_and_permission_tests {
    use super::*;
    use axum::http::StatusCode;
    use rpg_engine::api::helpers::validate_actor_turn_permission;
    use rpg_engine::api::state::get_turn_lock;
    use rpg_engine::state::{CampaignManifest, PlayerSlot};

    #[tokio::test]
    async fn test_turn_lock_concurrency_mutex() {
        let server_state = ServerState {
            rooms: Arc::new(RwLock::new(HashMap::new())),
            campaigns: Arc::new(RwLock::new(HashMap::new())),
            channels: Arc::new(RwLock::new(HashMap::new())),
            turn_locks: Arc::new(RwLock::new(HashMap::new())),
            skeleton_agent: Arc::new(SkeletonAgent::new("dummy".to_string())),
            environment_agent: Arc::new(EnvironmentAgent::new("dummy".to_string())),
            orchestrator: Arc::new(NarrativeOrchestrator::new("dummy".to_string(), "dummy".to_string())),
            user_store: Arc::new(UserStore::new("data/users")),
        };

        let lock1 = get_turn_lock(&server_state, "camp_1").await;
        let lock2 = get_turn_lock(&server_state, "camp_1").await;

        // Adquire a trava simulando processamento da IA
        let guard = lock1.try_lock();
        assert!(guard.is_ok(), "Primeira requisição deve adquirir o turn lock com sucesso");

        // Segunda requisição concorrente na mesma campanha tenta adquirir enquanto a primeira está ocupada
        let guard2 = lock2.try_lock();
        assert!(guard2.is_err(), "Segunda requisição concorrente deve ser bloqueada (CONFLICT)");

        // Libera a trava da primeira requisição
        drop(guard);

        // Agora a trava fica disponível novamente
        let guard3 = lock2.try_lock();
        assert!(guard3.is_ok(), "Após liberação, a trava deve poder ser adquirida");
    }

    #[test]
    fn test_solo_mode_allows_any_actor_command() {
        let manifest = CampaignManifest {
            campaign_id: "camp_solo".to_string(),
            room_code: "ROOM1".to_string(),
            host_player_id: "player_host".to_string(),
            player_ids: vec!["player_host".to_string()],
            player_slots: vec![PlayerSlot {
                player_number: 1,
                player_id: "player_host".to_string(),
                player_name: "Guerreiro Solo".to_string(),
                class_name: "Fighter".to_string(),
                player_code: "SOLO".to_string(),
                is_host: true,
            }],
            premise: "Premissa Solo".to_string(),
            literary_style: "Fantasia".to_string(),
            hidden_skeleton: rpg_engine::narrative::skeleton::NarrativeSkeleton::new("Premissa", "Fantasia"),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            status: "active".to_string(),
            custom_monsters: vec![],
            world_prologue: None,
        };

        let res = validate_actor_turn_permission(
            Some(&manifest),
            "ally_companion",
            Some("player_host"),
            Some("SOLO"),
        );
        assert!(res.is_ok(), "Modo solo deve permitir comandar qualquer personagem da party");
    }

    #[test]
    fn test_multiplayer_mode_validates_turn_ownership() {
        let manifest = CampaignManifest {
            campaign_id: "camp_multi".to_string(),
            room_code: "MULT".to_string(),
            host_player_id: "host_id".to_string(),
            player_ids: vec!["host_id".to_string(), "guest_id".to_string()],
            player_slots: vec![
                PlayerSlot {
                    player_number: 1,
                    player_id: "host_id".to_string(),
                    player_name: "Host Hero".to_string(),
                    class_name: "Paladin".to_string(),
                    player_code: "HOST".to_string(),
                    is_host: true,
                },
                PlayerSlot {
                    player_number: 2,
                    player_id: "guest_id".to_string(),
                    player_name: "Guest Rogue".to_string(),
                    class_name: "Rogue".to_string(),
                    player_code: "GUES".to_string(),
                    is_host: false,
                },
            ],
            premise: "Premissa Multi".to_string(),
            literary_style: "Fantasia".to_string(),
            hidden_skeleton: rpg_engine::narrative::skeleton::NarrativeSkeleton::new("Premissa", "Fantasia"),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            status: "active".to_string(),
            custom_monsters: vec![],
            world_prologue: None,
        };

        // 1. Guest comandando seu próprio personagem -> Permitido
        let ok_res = validate_actor_turn_permission(
            Some(&manifest),
            "guest_id",
            Some("guest_id"),
            Some("GUES"),
        );
        assert!(ok_res.is_ok(), "Guest deve conseguir comandar seu próprio personagem");

        // 2. Guest tentando comandar o personagem do Host -> Rejeitado (403 Forbidden)
        let forbidden_res = validate_actor_turn_permission(
            Some(&manifest),
            "host_id",
            Some("guest_id"),
            Some("GUES"),
        );
        assert!(forbidden_res.is_err(), "Guest não pode comandar personagem de outro jogador");
        let (status, val) = forbidden_res.unwrap_err();
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(val["code"], "NOT_YOUR_TURN_OR_ACTOR");

        // 3. Host comandando personagem de outro jogador -> Permitido
        let host_override = validate_actor_turn_permission(
            Some(&manifest),
            "guest_id",
            Some("host_id"),
            Some("HOST"),
        );
        assert!(host_override.is_ok(), "Host mestre deve ter permissão administrativa");
    }
}
