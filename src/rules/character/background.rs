use super::skills::Skill;
use serde::{Deserialize, Serialize};

/// Representa o Antecedente (Background) de um personagem.
/// Em nosso motor tático, focaremos apenas nas vantagens mecânicas (Proficiências).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Background {
    pub name: String,
    pub skill_proficiencies: Vec<Skill>,
    // Futuro: tool_proficiencies: Vec<Tool>
}

impl Background {
    pub fn new(name: &str, skill_proficiencies: Vec<Skill>) -> Self {
        Self {
            name: name.to_string(),
            skill_proficiencies,
        }
    }
}

/// Acolyte Background (SRD padrão)
pub fn create_acolyte() -> Background {
    Background::new("Acolyte", vec![Skill::Insight, Skill::Religion])
}
