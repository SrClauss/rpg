use rusqlite::{params, Result};
use uuid::Uuid;

use crate::narrative::db::NarrativeDb;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Scene,
    Combat,
    Dialogue,
    Location,
    Custom(String),
}

impl NodeType {
    pub fn as_str(&self) -> &str {
        match self {
            NodeType::Scene => "scene",
            NodeType::Combat => "combat",
            NodeType::Dialogue => "dialogue",
            NodeType::Location => "location",
            NodeType::Custom(s) => s.as_str(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "scene" => NodeType::Scene,
            "combat" => NodeType::Combat,
            "dialogue" => NodeType::Dialogue,
            "location" => NodeType::Location,
            _ => NodeType::Custom(s.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NarrativeNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub title: String,
    pub description: String,
}

impl NarrativeNode {
    pub fn new(node_type: NodeType, title: &str, description: &str) -> Self {
        NarrativeNode {
            id: Uuid::new_v4(),
            node_type,
            title: title.to_string(),
            description: description.to_string(),
        }
    }

    pub fn insert(&self, db: &NarrativeDb) -> Result<()> {
        db.conn.execute(
            "INSERT INTO Nodes (id, node_type, title, description) VALUES (?1, ?2, ?3, ?4)",
            params![
                self.id.to_string(),
                self.node_type.as_str(),
                self.title,
                self.description
            ],
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct NarrativeEdge {
    pub id: Uuid,
    pub source_node_id: Uuid,
    pub target_node_id: Uuid,
    pub action_text: String,
    pub condition_logic: Option<String>,
}

impl NarrativeEdge {
    pub fn new(source: Uuid, target: Uuid, action_text: &str, condition_logic: Option<&str>) -> Self {
        NarrativeEdge {
            id: Uuid::new_v4(),
            source_node_id: source,
            target_node_id: target,
            action_text: action_text.to_string(),
            condition_logic: condition_logic.map(|s| s.to_string()),
        }
    }

    pub fn insert(&self, db: &NarrativeDb) -> Result<()> {
        db.conn.execute(
            "INSERT INTO Edges (id, source_node_id, target_node_id, action_text, condition_logic) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                self.id.to_string(),
                self.source_node_id.to_string(),
                self.target_node_id.to_string(),
                self.action_text,
                self.condition_logic
            ],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_insertion() {
        let db = NarrativeDb::new_in_memory().unwrap();
        let node = NarrativeNode::new(NodeType::Scene, "Dark Cave", "It's pitch black.");
        node.insert(&db).unwrap();
        
        let count: i32 = db.conn.query_row("SELECT COUNT(*) FROM Nodes", [], |row| row.get(0)).unwrap();
        assert_eq!(count, 1);
    }
    
    #[test]
    fn test_edge_insertion() {
        let db = NarrativeDb::new_in_memory().unwrap();
        let node1 = NarrativeNode::new(NodeType::Location, "Cave Entrance", "...");
        let node2 = NarrativeNode::new(NodeType::Location, "Deep Cave", "...");
        node1.insert(&db).unwrap();
        node2.insert(&db).unwrap();
        
        let edge = NarrativeEdge::new(node1.id, node2.id, "Enter the cave", None);
        edge.insert(&db).unwrap();
        
        let count: i32 = db.conn.query_row("SELECT COUNT(*) FROM Edges", [], |row| row.get(0)).unwrap();
        assert_eq!(count, 1);
    }
}
