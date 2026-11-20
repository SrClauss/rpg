pub mod admin;
pub mod auth;
pub mod campaign;
pub mod error;
pub mod helpers;
pub mod models;
pub mod openapi;
pub mod scene;
pub mod state;
pub mod turn;
pub mod ws;

pub use admin::admin_router;
pub use auth::auth_router;
pub use campaign::campaign_router;
pub use error::{ApiError, ApiResult};
pub use openapi::{swagger_router, ApiDoc};
pub use scene::scene_router;
pub use state::ServerState;
pub use turn::turn_router;
pub use ws::ws_router;

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use tower_http::cors::{Any, CorsLayer};

pub async fn health_check() -> (axum::http::StatusCode, Json<Value>) {
    (
        axum::http::StatusCode::OK,
        Json(json!({
            "status": "online",
            "engine": "rpg-engine-rust",
            "rules": "D&D 5e Deterministic",
            "multiplayer": "WebSockets Active",
            "version": "0.2.0"
        })),
    )
}

/// Constrói o roteador Axum completo da aplicação modularizada
pub fn app_router(server_state: ServerState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(health_check))
        .merge(auth_router())
        .merge(admin_router())
        .merge(campaign_router())
        .merge(scene_router())
        .merge(turn_router())
        .merge(ws_router())
        .merge(swagger_router())
        .layer(cors)
        .with_state(server_state)
}
