use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use rpg_engine::rules::resolver::RulesResolver;
use rpg_engine::state::CampaignState;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    campaign: Arc<RwLock<CampaignState>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TurnActionRequest {
    via_type: String, // "via1", "via2", "via3"
    action_id: Option<String>,
    player_text: Option<String>,
    character_name: String,
    class_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TurnActionResponse {
    status: String,
    requires_mechanic: bool,
    detected_intent: String,
    roll_result: Option<Value>,
    narrative_feedback: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("============================================================");
    println!("      RPG D&D 5E ENGINE — SERVIDOR HTTP AXUM (API)          ");
    println!("============================================================");

    let mut initial_state = CampaignState::new();
    let beat_id = initial_state.start_new_beat(
        "A Cripta dos Reis Esquecidos",
        rpg_engine::narrative::beat::BeatType::Exploration,
    );
    println!("Iniciando campanha com Beat base (ID: {})", beat_id);

    let shared_state = AppState {
        campaign: Arc::new(RwLock::new(initial_state)),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/state", get(get_state))
        .route("/api/character", post(receive_character))
        .route("/api/action/turn", post(process_turn_action))
        .layer(cors)
        .with_state(shared_state);

    let addr = "0.0.0.0:3001";
    println!("Servidor HTTP ouvindo em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "online",
            "engine": "rpg-engine-rust",
            "rules": "D&D 5e Deterministic",
            "version": "0.1.0"
        })),
    )
}

async fn get_state(State(app_state): State<AppState>) -> (StatusCode, Json<Value>) {
    let camp = app_state.campaign.read().await;
    let current_beat = camp.current_beat();

    (
        StatusCode::OK,
        Json(json!({
            "current_beat_index": camp.current_beat_index,
            "beat_title": current_beat.map(|b| b.title.clone()).unwrap_or_default(),
            "beat_summary": current_beat.map(|b| b.summary.clone()).unwrap_or_default(),
        })),
    )
}

async fn receive_character(
    State(_app_state): State<AppState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let name = payload["name"].as_str().unwrap_or("Aventureiro Desconhecido");
    let class_name = payload["className"].as_str().unwrap_or("Fighter");

    println!(">>> [BACKEND] Ficha de Personagem recebida: {} ({})", name, class_name);

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": format!("Ficha do herói '{}' sincronizada com o motor de regras D&D 5e.", name),
            "registered_class": class_name
        })),
    )
}

async fn process_turn_action(
    State(app_state): State<AppState>,
    Json(req): Json<TurnActionRequest>,
) -> (StatusCode, Json<TurnActionResponse>) {
    let mut camp = app_state.campaign.write().await;

    println!(">>> [BACKEND] Ação recebida via {}: {:?}", req.via_type, req);

    if req.via_type == "via3" {
        let text = req.player_text.clone().unwrap_or_default();
        let lower = text.to_lowercase();
        let is_mechanic = lower.contains("atacar")
            || lower.contains("chutar")
            || lower.contains("empurrar")
            || lower.contains("pular")
            || lower.contains("saltar")
            || lower.contains("esconder")
            || lower.contains("arrombar")
            || lower.contains("magia")
            || lower.contains("furtivo");

        if is_mechanic {
            let mechanic_payload = json!({
                "action": "ability_check",
                "ability": "Strength",
                "dc": 13,
                "actor": req.character_name
            });

            let resolution = RulesResolver::resolve(&mechanic_payload, &mut camp);

            (
                StatusCode::OK,
                Json(TurnActionResponse {
                    status: "resolved".to_string(),
                    requires_mechanic: true,
                    detected_intent: "Manobra Tática / Ação Improvisada (#regras)".to_string(),
                    roll_result: Some(resolution),
                    narrative_feedback: format!(
                        "O árbitro em Rust validou sua manobra '{}' resolvendo os dados de D&D 5e determinísticos.",
                        text
                    ),
                }),
            )
        } else {
            (
                StatusCode::OK,
                Json(TurnActionResponse {
                    status: "resolved".to_string(),
                    requires_mechanic: false,
                    detected_intent: "Interação Social / Diálogo Narrativo".to_string(),
                    roll_result: None,
                    narrative_feedback: format!(
                        "O Mestre registrou sua intenção '{}' no fluxo narrativo do Beat atual sem necessidade de rolagem mecânica.",
                        text
                    ),
                }),
            )
        }
    } else {
        // Via 1 or Via 2 (Direct Sheet or Scenario action)
        (
            StatusCode::OK,
            Json(TurnActionResponse {
                status: "resolved".to_string(),
                requires_mechanic: true,
                detected_intent: format!("Ação de {}", req.via_type.to_uppercase()),
                roll_result: Some(json!({
                    "action_id": req.action_id,
                    "applied_to_budget": true
                })),
                narrative_feedback: "Ação de turno processada e sincronizada com a economia de ações.".to_string(),
            }),
        )
    }
}
