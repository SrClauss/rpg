use serde::{Deserialize, Serialize};
use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;

/// Escolas de Magia do D&D 5e
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

/// Componentes de Conjuração (V, S, M)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Components {
    pub verbal: bool,
    pub somatic: bool,
    pub material: Option<String>,
}

impl Components {
    pub fn new(verbal: bool, somatic: bool, material: Option<&str>) -> Self {
        Self {
            verbal,
            somatic,
            material: material.map(|s| s.to_string()),
        }
    }

    pub fn vsm(material: &str) -> Self {
        Self::new(true, true, Some(material))
    }

    pub fn vs() -> Self {
        Self::new(true, true, None)
    }

    pub fn v() -> Self {
        Self::new(true, false, None)
    }
}

/// Tipos de Efeito que uma Magia pode causar
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellEffect {
    /// Ataque Mágico (rola ataque vs CA do alvo)
    SpellAttack {
        damage_dice: String,
        damage_type: DamageType,
    },
    /// Exige Teste de Salvaguarda do alvo
    SaveThrow {
        save_ability: Ability,
        half_damage_on_save: bool,
        damage_dice: Option<String>,
        damage_type: Option<DamageType>,
    },
    /// Restauração de Pontos de Vida
    Healing {
        healing_dice: String,
    },
    /// Efeito Utilitário / Condição
    Utility {
        description: String,
    },
}

/// Representa uma Magia do SRD D&D 5e
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub level: u8, // 0 = Truque (Cantrip), 1 a 9 = Nível da magia
    pub school: SpellSchool,
    pub casting_time: String,
    pub range_feet: u32,
    pub components: Components,
    pub duration: String,
    pub requires_concentration: bool,
    pub effect: SpellEffect,
}

impl Spell {
    pub fn new(
        name: &str,
        level: u8,
        school: SpellSchool,
        casting_time: &str,
        range_feet: u32,
        components: Components,
        duration: &str,
        requires_concentration: bool,
        effect: SpellEffect,
    ) -> Self {
        Self {
            name: name.to_string(),
            level,
            school,
            casting_time: casting_time.to_string(),
            range_feet,
            components,
            duration: duration.to_string(),
            requires_concentration,
            effect,
        }
    }

    pub fn is_cantrip(&self) -> bool {
        self.level == 0
    }
}

/// Helpers para criar magias de exemplo do SRD
pub fn create_fire_bolt() -> Spell {
    Spell::new(
        "Fire Bolt",
        0, // Truque
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vs(),
        "Instantaneous",
        false,
        SpellEffect::SpellAttack {
            damage_dice: "1d10".to_string(),
            damage_type: DamageType::Slashing, // Usando Slashing enquanto não expandimos para Fire
        },
    )
}

pub fn create_cure_wounds() -> Spell {
    Spell::new(
        "Cure Wounds",
        1,
        SpellSchool::Evocation,
        "1 action",
        0, // Toque
        Components::vs(),
        "Instantaneous",
        false,
        SpellEffect::Healing {
            healing_dice: "1d8".to_string(),
        },
    )
}

pub fn create_fireball() -> Spell {
    Spell::new(
        "Fireball",
        3,
        SpellSchool::Evocation,
        "1 action",
        150,
        Components::vsm("A tiny ball of bat guano and sulfur"),
        "Instantaneous",
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: true,
            damage_dice: Some("8d6".to_string()),
            damage_type: Some(DamageType::Bludgeoning),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spell_creation() {
        let cantrip = create_fire_bolt();
        assert!(cantrip.is_cantrip());
        assert_eq!(cantrip.level, 0);
        assert!(!cantrip.requires_concentration);

        let fireball = create_fireball();
        assert!(!fireball.is_cantrip());
        assert_eq!(fireball.level, 3);
        assert_eq!(fireball.components.material, Some("A tiny ball of bat guano and sulfur".to_string()));
    }
}
