use serde::{Deserialize, Serialize};

/// Categoria da Armadura
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmorCategory {
    Light,
    Medium,
    Heavy,
    Shield,
}

/// Representa uma Armadura ou Escudo Equipável
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armor {
    pub name: String,
    pub category: ArmorCategory,
    pub base_ac: i32,
    
    /// Se true, o bônus de Destreza é limitado (normalmente a +2 para armaduras médias)
    pub has_dex_limit: bool,
    pub dex_limit_max: i32,
    
    /// Se true, usar esta armadura impõe desvantagem em testes de Furtividade (Stealth)
    pub stealth_disadvantage: bool,
    
    /// Força mínima necessária para não sofrer penalidade de deslocamento (-10ft)
    pub strength_requirement: i32,
}

impl Armor {
    pub fn new(
        name: &str,
        category: ArmorCategory,
        base_ac: i32,
        has_dex_limit: bool,
        dex_limit_max: i32,
        stealth_disadvantage: bool,
        strength_requirement: i32,
    ) -> Self {
        Self {
            name: name.to_string(),
            category,
            base_ac,
            has_dex_limit,
            dex_limit_max,
            stealth_disadvantage,
            strength_requirement,
        }
    }
}

/// Helpers to create standard SRD armor
pub fn create_leather_armor() -> Armor {
    Armor::new("Leather Armor", ArmorCategory::Light, 11, false, 0, false, 0)
}

pub fn create_chain_shirt() -> Armor {
    Armor::new("Chain Shirt", ArmorCategory::Medium, 13, true, 2, false, 0)
}

pub fn create_plate_armor() -> Armor {
    Armor::new("Plate Armor", ArmorCategory::Heavy, 18, true, 0, true, 15) // Limite DEX = 0
}

pub fn create_shield() -> Armor {
    // Escudos adicionam +2 CA bruta.
    Armor::new("Shield", ArmorCategory::Shield, 2, false, 0, false, 0)
}
