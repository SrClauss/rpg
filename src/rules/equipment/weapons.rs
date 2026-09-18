use serde::{Deserialize, Serialize};

/// Tipos de dano no D&D 5e
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    Slashing,
    Piercing,
    Bludgeoning,
    Acid,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Poison,
    Psychic,
    Radiant,
    Thunder,
}

/// Propriedades Especiais de Armas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponProperty {
    Finesse,   // Acuidade (pode usar FOR ou DES)
    Heavy,     // Pesada (Desvantagem para criaturas Pequenas)
    Light,     // Leve (Usada para combate com duas armas)
    TwoHanded, // Duas mãos
    Versatile, // Versátil (pode usar com duas mãos para mais dano)
    Thrown,    // Arremesso
    Reach,     // Alcance (10ft)
}

/// Representa uma Arma Equipável
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub damage_dice: String, // Ex: "1d8", será parseado pelo módulo Dice
    pub damage_type: DamageType,
    pub properties: Vec<WeaponProperty>,
}

impl Weapon {
    pub fn new(name: &str, damage_dice: &str, damage_type: DamageType, properties: Vec<WeaponProperty>) -> Self {
        Self {
            name: name.to_string(),
            damage_dice: damage_dice.to_string(),
            damage_type,
            properties,
        }
    }

    /// Helper para verificar se a arma tem certa propriedade
    pub fn has_property(&self, prop: WeaponProperty) -> bool {
        self.properties.contains(&prop)
    }
}

/// Helper functions to quickly create standard SRD weapons
pub fn create_longsword() -> Weapon {
    Weapon::new(
        "Longsword",
        "1d8",
        DamageType::Slashing,
        vec![WeaponProperty::Versatile], // Versátil (1d10)
    )
}

pub fn create_dagger() -> Weapon {
    Weapon::new(
        "Dagger",
        "1d4",
        DamageType::Piercing,
        vec![WeaponProperty::Finesse, WeaponProperty::Light, WeaponProperty::Thrown],
    )
}

pub fn create_greatsword() -> Weapon {
    Weapon::new(
        "Greatsword",
        "2d6",
        DamageType::Slashing,
        vec![WeaponProperty::Heavy, WeaponProperty::TwoHanded],
    )
}
