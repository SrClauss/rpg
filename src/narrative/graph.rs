use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
}

impl FromStr for NodeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "scene" => NodeType::Scene,
            "combat" => NodeType::Combat,
            "dialogue" => NodeType::Dialogue,
            "location" => NodeType::Location,
            _ => NodeType::Custom(s.to_string()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}
