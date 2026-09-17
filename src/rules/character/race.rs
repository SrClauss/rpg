use serde::{Deserialize, Serialize};
use super::attributes::Ability;

/// Tamanho da criatura (Size)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

/// Interface para Raças do D&D
pub trait Race {
    fn get_speed(&self) -> u8;
    fn get_size(&self) -> Size;
    /// Retorna uma lista de Tuplas (Atributo, Bônus)
    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)>;
}

/// Estrutura Fixa: Humano
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Human;

impl Race for Human {
    fn get_speed(&self) -> u8 {
        30
    }

    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        // Human base: +1 para todos os atributos
        vec![
            (Ability::Strength, 1),
            (Ability::Dexterity, 1),
            (Ability::Constitution, 1),
            (Ability::Intelligence, 1),
            (Ability::Wisdom, 1),
            (Ability::Charisma, 1),
        ]
    }
}

/// Estrutura Fixa: Elfo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Elf;

impl Race for Elf {
    fn get_speed(&self) -> u8 {
        30
    }

    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        // Elfo base: +2 em Destreza
        vec![(Ability::Dexterity, 2)]
    }
}

/// A Variante Única de Raça
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RaceVariant {
    Human(Human),
    Elf(Elf),
    // Outras raças serão adicionadas aqui
}

impl RaceVariant {
    pub fn as_trait(&self) -> &dyn Race {
        match self {
            RaceVariant::Human(h) => h,
            RaceVariant::Elf(e) => e,
        }
    }
}
