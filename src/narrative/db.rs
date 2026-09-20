use rusqlite::{Connection, Result};

pub struct NarrativeDb {
    pub conn: Connection,
}

impl NarrativeDb {
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = NarrativeDb { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = NarrativeDb { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Nodes (
                id TEXT PRIMARY KEY,
                node_type TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Edges (
                id TEXT PRIMARY KEY,
                source_node_id TEXT NOT NULL,
                target_node_id TEXT NOT NULL,
                action_text TEXT NOT NULL,
                condition_logic TEXT
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS ContextualMemory (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                node_id TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                entity_type TEXT NOT NULL,
                entity_name TEXT NOT NULL,
                event_type TEXT NOT NULL,
                context_summary TEXT NOT NULL,
                emotional_state TEXT
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_memory_entity ON ContextualMemory(entity_type, entity_name)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_memory_event ON ContextualMemory(event_type)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_memory_node ON ContextualMemory(node_id)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS EventLogs (
                id TEXT PRIMARY KEY,
                node_id TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                event_category TEXT NOT NULL,
                actor_name TEXT NOT NULL,
                target_name TEXT,
                description TEXT NOT NULL,
                raw_json_data TEXT
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_actor ON EventLogs(actor_name)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_category ON EventLogs(event_category)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_time ON EventLogs(timestamp)",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS EntityStatistics (
                entity_name TEXT PRIMARY KEY,
                total_damage_dealt INTEGER DEFAULT 0,
                total_damage_taken INTEGER DEFAULT 0,
                enemies_defeated INTEGER DEFAULT 0,
                spells_cast INTEGER DEFAULT 0,
                critical_hits_scored INTEGER DEFAULT 0,
                times_unconscious INTEGER DEFAULT 0,
                distance_traveled INTEGER DEFAULT 0
            )",
            [],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_initialization() {
        let db = NarrativeDb::new_in_memory().unwrap();
        
        let mut stmt = db.conn.prepare("SELECT name FROM sqlite_master WHERE type='table'").unwrap();
        let tables: Vec<String> = stmt.query_map([], |row| row.get(0)).unwrap().map(|r| r.unwrap()).collect();
        
        assert!(tables.contains(&"Nodes".to_string()));
        assert!(tables.contains(&"Edges".to_string()));
        assert!(tables.contains(&"ContextualMemory".to_string()));
        assert!(tables.contains(&"EventLogs".to_string()));
        assert!(tables.contains(&"EntityStatistics".to_string()));
    }
}
