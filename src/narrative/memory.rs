use rusqlite::{params, Result};
use uuid::Uuid;

use crate::narrative::db::NarrativeDb;

#[derive(Debug, Clone)]
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
            timestamp: "".to_string(), // handled by sqlite
            entity_type: entity_type.to_string(),
            entity_name: entity_name.to_string(),
            event_type: event_type.to_string(),
            context_summary: context_summary.to_string(),
            emotional_state: emotional_state.map(|s| s.to_string()),
        }
    }

    pub fn record(&self, db: &NarrativeDb) -> Result<()> {
        db.conn.execute(
            "INSERT INTO ContextualMemory (node_id, entity_type, entity_name, event_type, context_summary, emotional_state)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                self.node_id.to_string(),
                self.entity_type,
                self.entity_name,
                self.event_type,
                self.context_summary,
                self.emotional_state
            ],
        )?;
        Ok(())
    }

    pub fn fetch_for_entity(db: &NarrativeDb, entity_name: &str) -> Result<Vec<ContextualMemory>> {
        let mut stmt = db.conn.prepare(
            "SELECT id, node_id, timestamp, entity_type, entity_name, event_type, context_summary, emotional_state
             FROM ContextualMemory
             WHERE entity_name = ?1
             ORDER BY timestamp DESC"
        )?;

        let memory_iter = stmt.query_map(params![entity_name], |row| {
            let node_id_str: String = row.get(1)?;
            Ok(ContextualMemory {
                id: Some(row.get(0)?),
                node_id: Uuid::parse_str(&node_id_str).unwrap_or_default(),
                timestamp: row.get(2)?,
                entity_type: row.get(3)?,
                entity_name: row.get(4)?,
                event_type: row.get(5)?,
                context_summary: row.get(6)?,
                emotional_state: row.get(7)?,
            })
        })?;

        let mut memories = Vec::new();
        for mem in memory_iter {
            memories.push(mem?);
        }

        Ok(memories)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::narrative::graph::{NarrativeNode, NodeType};

    #[test]
    fn test_memory_recording_and_fetching() {
        let db = NarrativeDb::new_in_memory().unwrap();
        let node = NarrativeNode::new(NodeType::Combat, "Battle in the woods", "...");
        node.insert(&db).unwrap();

        let memory = ContextualMemory::new(
            node.id,
            "npc",
            "Goblin King",
            "combat",
            "The Goblin King was ambushed.",
            Some("angry"),
        );
        memory.record(&db).unwrap();

        let results = ContextualMemory::fetch_for_entity(&db, "Goblin King").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].context_summary, "The Goblin King was ambushed.");
        assert_eq!(results[0].emotional_state.as_deref(), Some("angry"));
    }
}
