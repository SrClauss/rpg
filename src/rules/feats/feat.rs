use crate::rules::character::attributes::Ability;
use crate::rules::equipment::validator::ValidatedEffect;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pré-requisitos para um personagem poder adquirir um Talento
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct FeatPrerequisites {
    pub min_strength: Option<u8>,
    pub min_dexterity: Option<u8>,
    pub min_constitution: Option<u8>,
    pub min_intelligence: Option<u8>,
    pub min_wisdom: Option<u8>,
    pub min_charisma: Option<u8>,
    pub requires_spellcasting: bool,
    pub requires_armor_proficiency: Option<String>,
}

/// Propriedades mecânicas estruturadas calculadas nativamente pelo Rust
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct FeatMechanicalEffect {
    pub initiative_bonus: i32,
    pub cannot_be_surprised: bool,
    pub denies_advantage_from_hidden_attackers: bool,
    pub advantage_against_grappled: bool,
    pub can_pin_grappled_target: bool,
    pub ignore_half_cover: bool,
    pub ignore_three_quarters_cover: bool,
    pub hp_per_level_bonus: u8,
    #[serde(default)]
    pub stat_increases: HashMap<Ability, u8>,
}

/// Definição completa de um Talento (SRD 5.1 ou Homebrew)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feat {
    pub id: String,
    pub name: String,
    pub description: String,
    pub prerequisites: FeatPrerequisites,
    pub effect: ValidatedEffect,
    #[serde(default)]
    pub mechanical_details: Option<FeatMechanicalEffect>,
}

impl Feat {
    pub fn new_mechanical(
        id: &str,
        name: &str,
        description: &str,
        prerequisites: FeatPrerequisites,
        details: FeatMechanicalEffect,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            prerequisites,
            effect: ValidatedEffect::Mechanical {
                effect_name: name.to_string(),
                description: description.to_string(),
                save_dc: None,
                save_ability: None,
                condition: None,
            },
            mechanical_details: Some(details),
        }
    }

    pub fn new_narrative_custom(
        id: &str,
        name: &str,
        description: &str,
        prerequisites: FeatPrerequisites,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            prerequisites,
            effect: ValidatedEffect::NarrativeCustom {
                description: description.to_string(),
            },
            mechanical_details: None,
        }
    }
}
