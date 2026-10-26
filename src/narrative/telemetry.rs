use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLog {
    pub id: Uuid,
    pub node_id: String,
    pub timestamp: String,
    pub event_category: String,
    pub actor_name: String,
    pub target_name: Option<String>,
    pub description: String,
    pub raw_json_data: Option<String>,
}

impl EventLog {
    pub fn new(
        node_id: &str,
        event_category: &str,
        actor_name: &str,
        target_name: Option<&str>,
        description: &str,
        raw_json_data: Option<&str>,
    ) -> Self {
        EventLog {
            id: Uuid::new_v4(),
            node_id: node_id.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            event_category: event_category.to_string(),
            actor_name: actor_name.to_string(),
            target_name: target_name.map(|s| s.to_string()),
            description: description.to_string(),
            raw_json_data: raw_json_data.map(|s| s.to_string()),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntityStats {
    pub entity_name: String,
    pub total_damage_dealt: i32,
    pub total_damage_taken: i32,
    pub enemies_defeated: i32,
    pub spells_cast: i32,
    pub critical_hits_scored: i32,
    pub times_unconscious: i32,
    pub distance_traveled: i32,
}

impl EntityStats {
    pub fn new(entity_name: &str) -> Self {
        Self {
            entity_name: entity_name.to_string(),
            ..Default::default()
        }
    }

    pub fn add_damage_dealt(&mut self, amount: i32) {
        self.total_damage_dealt += amount;
    }
}
