use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use rpg_engine::llm::NarrativeOrchestrator;
use rpg_engine::narrative::telemetry::EntityStats;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;
use std::thread;

enum OrchestratorCommand {
    ProcessAction {
        prompt: String,
        responder: oneshot::Sender<Result<String, String>>,
    },
    GetParty {
        responder: oneshot::Sender<Vec<String>>,
    },
}

#[derive(Clone)]
struct AppState {
    tx: mpsc::Sender<OrchestratorCommand>,
}

#[derive(Deserialize)]
struct ActionRequest {
    prompt: String,
}

#[derive(Serialize)]
struct ActionResponse {
    narration: String,
}

#[derive(Serialize)]
struct PartyResponse {
    characters: Vec<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let gemini_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY não encontrada");
    let deepseek_key = std::env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY não encontrada");
    
    // Canal para comunicação com a Thread do Orquestrador
    let (tx, mut rx) = mpsc::channel::<OrchestratorCommand>(32);

    // Inicia uma thread do SO para rodar o runtime LocalSet (para lidar com !Send Futures do SQLite)
    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        
        rt.block_on(async {
            let local = tokio::task::LocalSet::new();
            local.run_until(async move {
                let orchestrator = NarrativeOrchestrator::new(gemini_key, deepseek_key);
                
                EntityStats::get_or_create(&orchestrator.db, "Kaelen (Guerreiro)").unwrap();
                EntityStats::get_or_create(&orchestrator.db, "Lira (Maga)").unwrap();
                EntityStats::get_or_create(&orchestrator.db, "Torin (Ladino)").unwrap();
                EntityStats::get_or_create(&orchestrator.db, "Nyx (Clériga)").unwrap();

                while let Some(cmd) = rx.recv().await {
                    match cmd {
                        OrchestratorCommand::ProcessAction { prompt, responder } => {
                            let result = match orchestrator.process_player_action(&prompt).await {
                                Ok(res) => Ok(res),
                                Err(e) => Err(e.to_string()),
                            };
                            let _ = responder.send(result);
                        }
                        OrchestratorCommand::GetParty { responder } => {
                            let mut stmt = orchestrator.db.conn.prepare("SELECT entity_id FROM EntityStats").unwrap();
                            let names: Vec<String> = stmt.query_map([], |row| row.get(0)).unwrap().map(|r| r.unwrap()).collect();
                            let _ = responder.send(names);
                        }
                    }
                }
            }).await;
        });
    });

    let state = AppState { tx };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/api/action", post(handle_action))
        .route("/api/party", get(handle_party))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor API rodando em http://localhost:3000...");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_action(
    State(state): State<AppState>,
    Json(payload): Json<ActionRequest>,
) -> Json<ActionResponse> {
    let (tx, rx) = oneshot::channel();
    let _ = state.tx.send(OrchestratorCommand::ProcessAction { 
        prompt: payload.prompt, 
        responder: tx 
    }).await;

    match rx.await {
        Ok(Ok(narration)) => Json(ActionResponse { narration }),
        Ok(Err(err)) => Json(ActionResponse { narration: format!("Erro interno: {}", err) }),
        Err(_) => Json(ActionResponse { narration: "Erro ao comunicar com o Orquestrador.".into() }),
    }
}

async fn handle_party(
    State(state): State<AppState>,
) -> Json<PartyResponse> {
    let (tx, rx) = oneshot::channel();
    let _ = state.tx.send(OrchestratorCommand::GetParty { responder: tx }).await;

    match rx.await {
        Ok(characters) => Json(PartyResponse { characters }),
        Err(_) => Json(PartyResponse { characters: vec![] }),
    }
}
