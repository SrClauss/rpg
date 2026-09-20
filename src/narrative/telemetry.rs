use rusqlite::{params, Result, OptionalExtension};
use uuid::Uuid;

use crate::narrative::db::NarrativeDb;

#[derive(Debug, Clone)]
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
            timestamp: "".to_string(), // Handled by DB DEFAULT CURRENT_TIMESTAMP
            event_category: event_category.to_string(),
            actor_name: actor_name.to_string(),
            target_name: target_name.map(|s| s.to_string()),
            description: description.to_string(),
            raw_json_data: raw_json_data.map(|s| s.to_string()),
        }
    }

    pub fn insert(&self, db: &NarrativeDb) -> Result<()> {
        db.conn.execute(
            "INSERT INTO EventLogs (id, node_id, event_category, actor_name, target_name, description, raw_json_data)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                self.id.to_string(),
                self.node_id,
                self.event_category,
                self.actor_name,
                self.target_name,
                self.description,
                self.raw_json_data
            ],
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
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
    pub fn get_or_create(db: &NarrativeDb, entity_name: &str) -> Result<Self> {
        let mut stmt = db.conn.prepare(
            "SELECT entity_name, total_damage_dealt, total_damage_taken, enemies_defeated, 
                    spells_cast, critical_hits_scored, times_unconscious, distance_traveled
             FROM EntityStatistics
             WHERE entity_name = ?1"
        )?;

        let stats = stmt.query_row(params![entity_name], |row| {
            Ok(EntityStats {
                entity_name: row.get(0)?,
                total_damage_dealt: row.get(1)?,
                total_damage_taken: row.get(2)?,
                enemies_defeated: row.get(3)?,
                spells_cast: row.get(4)?,
                critical_hits_scored: row.get(5)?,
                times_unconscious: row.get(6)?,
                distance_traveled: row.get(7)?,
            })
        }).optional()?;

        match stats {
            Some(s) => Ok(s),
            None => {
                let s = EntityStats { entity_name: entity_name.to_string(), ..Default::default() };
                s.save(db)?;
                Ok(s)
            }
        }
    }

    pub fn save(&self, db: &NarrativeDb) -> Result<()> {
        db.conn.execute(
            "INSERT INTO EntityStatistics (
                entity_name, total_damage_dealt, total_damage_taken, enemies_defeated, 
                spells_cast, critical_hits_scored, times_unconscious, distance_traveled
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(entity_name) DO UPDATE SET
                total_damage_dealt = excluded.total_damage_dealt,
                total_damage_taken = excluded.total_damage_taken,
                enemies_defeated = excluded.enemies_defeated,
                spells_cast = excluded.spells_cast,
                critical_hits_scored = excluded.critical_hits_scored,
                times_unconscious = excluded.times_unconscious,
                distance_traveled = excluded.distance_traveled",
            params![
                self.entity_name,
                self.total_damage_dealt,
                self.total_damage_taken,
                self.enemies_defeated,
                self.spells_cast,
                self.critical_hits_scored,
                self.times_unconscious,
                self.distance_traveled
            ],
        )?;
        Ok(())
    }

    // Helper method to easily add damage
    pub fn add_damage_dealt(&mut self, db: &NarrativeDb, amount: i32) -> Result<()> {
        self.total_damage_dealt += amount;
        self.save(db)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_log_insertion() {
        let db = NarrativeDb::new_in_memory().unwrap();
        let log = EventLog::new(
            "node-123",
            "combat_roll",
            "Claus",
            Some("Goblin"),
            "Claus rolled 18 to hit.",
            Some(r#"{"roll": 18, "is_crit": false}"#)
        );
        log.insert(&db).unwrap();

        let count: i32 = db.conn.query_row("SELECT COUNT(*) FROM EventLogs", [], |row| row.get(0)).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_entity_stats_upsert() {
        let db = NarrativeDb::new_in_memory().unwrap();
        
        let mut stats = EntityStats::get_or_create(&db, "Hero").unwrap();
        assert_eq!(stats.total_damage_dealt, 0);

        stats.add_damage_dealt(&db, 15).unwrap();

        let updated_stats = EntityStats::get_or_create(&db, "Hero").unwrap();
        assert_eq!(updated_stats.total_damage_dealt, 15);
    }
}
