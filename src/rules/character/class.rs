use serde::{Deserialize, Serialize};

/// Dado de Vida da classe (Hit Dice)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HitDice {
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
}

/// Interface genérica que toda classe deve implementar
pub trait CharacterClass {
    fn get_hit_dice(&self) -> HitDice;
    fn get_primary_abilities(&self) -> Vec<super::attributes::Ability>;
    fn get_saving_throw_proficiencies(&self) -> Vec<super::attributes::Ability>;
    
    // Futuro: fn get_features_for_level(&self, level: u8) -> Vec<ClassFeature>;
}

/// Estrutura Fixa: Guerreiro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fighter;

impl CharacterClass for Fighter {
    fn get_hit_dice(&self) -> HitDice {
        HitDice::D10
    }

    fn get_primary_abilities(&self) -> Vec<super::attributes::Ability> {
        use super::attributes::Ability;
        vec![Ability::Strength, Ability::Dexterity]
    }

    fn get_saving_throw_proficiencies(&self) -> Vec<super::attributes::Ability> {
        use super::attributes::Ability;
        vec![Ability::Strength, Ability::Constitution]
    }
}

/// Estrutura Fixa: Mago
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wizard;

impl CharacterClass for Wizard {
    fn get_hit_dice(&self) -> HitDice {
        HitDice::D6
    }

    fn get_primary_abilities(&self) -> Vec<super::attributes::Ability> {
        use super::attributes::Ability;
        vec![Ability::Intelligence]
    }

    fn get_saving_throw_proficiencies(&self) -> Vec<super::attributes::Ability> {
        use super::attributes::Ability;
        vec![Ability::Intelligence, Ability::Wisdom]
    }
}

/// A Variante Única de Classe (Garante que a Entidade tenha apenas 1 classe, sem arrays)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassVariant {
    Fighter(Fighter),
    Wizard(Wizard),
    // Adicionar outras classes aqui (Rogue, Cleric, etc)
}

impl ClassVariant {
    /// Helper para acessar o trait CharacterClass subjacente
    pub fn as_trait(&self) -> &dyn CharacterClass {
        match self {
            ClassVariant::Fighter(f) => f,
            ClassVariant::Wizard(w) => w,
        }
    }
}
