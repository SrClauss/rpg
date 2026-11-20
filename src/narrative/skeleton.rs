use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkeletonNode {
    pub id: String,
    pub act_number: u32,
    pub title: String,
    pub synopsis: String,
    pub expected_milestones: Vec<String>,
    pub completed: bool,
    #[serde(default)]
    pub arc_name: Option<String>,
    #[serde(default)]
    pub arc_number: Option<u32>,
    #[serde(default)]
    pub boss_name: Option<String>,
    #[serde(default)]
    pub battle_intensity: Option<String>, // "Roleplay Primário", "Masmorra & Combate", "Equilibrado", "D20 Sorte"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutonomousWorldEvent {
    pub id: String,
    pub trigger_chapter: u32,
    pub description: String,
    pub executed: bool,
    pub impact_summary: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SkeletonCorruptionRecord {
    pub timestamp: String,
    pub divergence_reason: String,
    pub previous_plan_summary: String,
    pub new_plan_summary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StoryFocus {
    Combat,
    Roleplay,
    D20,
}

impl StoryFocus {
    pub fn as_str(&self) -> &'static str {
        match self {
            StoryFocus::Combat => "combat",
            StoryFocus::Roleplay => "roleplay",
            StoryFocus::D20 => "d20",
        }
    }
}

impl std::fmt::Display for StoryFocus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for StoryFocus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "combat" => Ok(StoryFocus::Combat),
            "roleplay" => Ok(StoryFocus::Roleplay),
            "d20" => Ok(StoryFocus::D20),
            other => Err(format!("StoryFocus inválido: {}", other)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NarrativeSkeleton {
    pub id: String,
    pub premise: String,
    pub literary_style: String,
    pub expected_acts: Vec<SkeletonNode>,
    pub autonomous_world_events: Vec<AutonomousWorldEvent>,
    pub is_corrupted: bool,
    pub corruption_history: Vec<SkeletonCorruptionRecord>,
    pub current_act_index: usize,
    #[serde(default)]
    pub homebrew_bosses: Vec<serde_json::Value>,
    #[serde(default)]
    pub story_focus: Option<StoryFocus>,
}

impl NarrativeSkeleton {
    pub fn new(premise: &str, literary_style: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            premise: premise.to_string(),
            literary_style: literary_style.to_string(),
            expected_acts: Vec::new(),
            autonomous_world_events: Vec::new(),
            is_corrupted: false,
            corruption_history: Vec::new(),
            current_act_index: 0,
            homebrew_bosses: Vec::new(),
            story_focus: None,
        }
    }

    pub fn current_act(&self) -> Option<&SkeletonNode> {
        self.expected_acts.get(self.current_act_index)
    }

    pub fn current_act_mut(&mut self) -> Option<&mut SkeletonNode> {
        self.expected_acts.get_mut(self.current_act_index)
    }

    pub fn mark_act_completed(&mut self) {
        if let Some(act) = self.current_act_mut() {
            act.completed = true;
        }
        if self.current_act_index + 1 < self.expected_acts.len() {
            self.current_act_index += 1;
        }
    }

    pub fn record_corruption(
        &mut self,
        divergence_reason: &str,
        previous_plan: &str,
        new_plan: &str,
        new_remaining_acts: Vec<SkeletonNode>,
    ) {
        self.is_corrupted = true;
        self.corruption_history.push(SkeletonCorruptionRecord {
            timestamp: Utc::now().to_rfc3339(),
            divergence_reason: divergence_reason.to_string(),
            previous_plan_summary: previous_plan.to_string(),
            new_plan_summary: new_plan.to_string(),
        });

        // Retém atos já completados e substitui os futuros
        self.expected_acts.truncate(self.current_act_index);
        self.expected_acts.extend(new_remaining_acts);
    }
}
