use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock, Mutex};
use axum::http::HeaderMap;

use crate::auth::UserStore;
use crate::llm::{EnvironmentAgent, NarrativeOrchestrator, SkeletonAgent};
use crate::state::CampaignState;

#[derive(Clone)]
pub struct ServerState {
    pub rooms: Arc<RwLock<HashMap<String, Arc<RwLock<CampaignState>>>>>, // room_code -> CampaignState
    pub campaigns: Arc<RwLock<HashMap<String, Arc<RwLock<CampaignState>>>>>, // campaign_id -> CampaignState
    pub channels: Arc<RwLock<HashMap<String, broadcast::Sender<String>>>>, // room_code -> broadcast::Sender
    pub turn_locks: Arc<RwLock<HashMap<String, Arc<Mutex<()>>>>>, // campaign_id/room_code -> Mutex<()>
    pub skeleton_agent: Arc<SkeletonAgent>,
    pub environment_agent: Arc<EnvironmentAgent>,
    pub orchestrator: Arc<NarrativeOrchestrator>,
    pub user_store: Arc<UserStore>,
}

pub fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|val| val.to_str().ok())
        .and_then(|auth| {
            if auth.starts_with("Bearer ") {
                Some(auth[7..].trim().to_string())
            } else {
                Some(auth.trim().to_string())
            }
        })
}

/// Helper para obter ou criar Mutex assíncrono de turno por campanha
pub async fn get_turn_lock(state: &ServerState, campaign_id: &str) -> Arc<Mutex<()>> {
    let mut locks = state.turn_locks.write().await;
    locks
        .entry(campaign_id.to_string())
        .or_insert_with(|| Arc::new(Mutex::new(())))
        .clone()
}
