use serde::{Deserialize, Serialize};
use super::attributes::Ability;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl Skill {
    /// Retorna o Atributo (Ability Score) associado à perícia segundo o SRD.
    pub fn base_ability(&self) -> Ability {
        match self {
            Skill::Athletics => Ability::Strength,
            
            Skill::Acrobatics 
            | Skill::SleightOfHand 
            | Skill::Stealth => Ability::Dexterity,
            
            Skill::Arcana 
            | Skill::History 
            | Skill::Investigation 
            | Skill::Nature 
            | Skill::Religion => Ability::Intelligence,
            
            Skill::AnimalHandling 
            | Skill::Insight 
            | Skill::Medicine 
            | Skill::Perception 
            | Skill::Survival => Ability::Wisdom,
            
            Skill::Deception 
            | Skill::Intimidation 
            | Skill::Performance 
            | Skill::Persuasion => Ability::Charisma,
        }
    }
}
