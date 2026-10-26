use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualMemory {
    pub id: Option<i32>,
    pub node_id: Uuid,
    pub timestamp: String,
    pub entity_type: String,
    pub entity_name: String,
    pub event_type: String,
    pub context_summary: String,
    pub emotional_state: Option<String>,
}

impl ContextualMemory {
    pub fn new(
        node_id: Uuid,
        entity_type: &str,
        entity_name: &str,
        event_type: &str,
        context_summary: &str,
        emotional_state: Option<&str>,
    ) -> Self {
        ContextualMemory {
            id: None,
            node_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
            entity_type: entity_type.to_string(),
            entity_name: entity_name.to_string(),
            event_type: event_type.to_string(),
            context_summary: context_summary.to_string(),
            emotional_state: emotional_state.map(|s| s.to_string()),
        }
    }
}
