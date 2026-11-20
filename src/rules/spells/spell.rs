use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use serde::{Deserialize, Serialize};

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

    pub fn s() -> Self {
        Self::new(false, true, None)
    }

    pub fn vm(material: &str) -> Self {
        Self::new(true, false, Some(material))
    }

    pub fn sm(material: &str) -> Self {
        Self::new(false, true, Some(material))
    }
}

/// Tipos de Efeito que uma Magia pode causar
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellEffect {
    /// Ataque Mágico (rola ataque vs CA do alvo)
    SpellAttack {
        damage_dice: String,
        damage_type: DamageType,
        damage_scaling: Option<String>, // Regra de escalonamento (ex: "cantrip")
        on_hit_effect: Option<Box<SpellEffect>>,
    },
    /// Exige Teste de Salvaguarda do alvo
    SaveThrow {
        save_ability: Ability,
        half_damage_on_save: bool,
        damage_dice: Option<String>,
        damage_type: Option<DamageType>,
        damage_scaling: Option<String>,
        fail_effect: Option<Box<SpellEffect>>,
    },
    /// Restauração de Pontos de Vida
    Healing { healing_dice: String },
    /// Curar em área ou em alvos múltiplos
    MultiTargetHealing {
        max_targets: u32,
        healing_dice: String,
    },
    /// Aplica um Efeito Ativo (Buff/Debuff) na Entidade alvo
    ApplyEffect {
        effect: ActiveEffect,
        duration: DurationType,
    },
    /// Efeito Utilitário / Condição (Narrativo)
    Utility { description: String },
    /// Modifica o terreno do campo de batalha (Grid Hexagonal) criando um Perigo de Terreno
    AreaHazard {
        radius_feet: u32,
        save_ability: Option<Ability>,
        condition: Option<crate::rules::conditions::Condition>,
        damage_dice: Option<String>,
        damage_type: Option<DamageType>,
        description: String,
    },
    /// Ataque Mágico com Múltiplos Alvos / Projéteis automáticos (ex: Magic Missile)
    MultiTargetSpellAttack {
        projectiles_count: u32,
        damage_dice: String,
        damage_type: DamageType,
        auto_hit: bool,
    },
    /// Combinação de múltiplos efeitos simultâneos (ex: Dano + Aplicação de Debuff)
    Compound(Vec<SpellEffect>),
    /// Permite que a magia tenha múltiplas opções de efeito, das quais o conjurador deve escolher uma.
    Choice(Vec<(String, Box<SpellEffect>)>),
    /// Efeito especial da magia Sleep (Adormece criaturas em ordem de PV)
    Sleep { pool_dice: String, radius_feet: u32 },
    /// Invocação (Cria entidades no campo de batalha)
    Summon { creature_name: String, count: u32 },
    /// Invoca uma criatura
    SummonCreature { monster_id: String },
    /// Aplica um Efeito Ativo (Buff/Debuff) em múltiplos alvos (ex: Aid, Bless)
    MultiTargetApplyEffect {
        max_targets: u32,
        effect: ActiveEffect,
        duration: DurationType,
    },
    /// Remove uma das condições especificadas do alvo
    CureCondition(Vec<crate::rules::conditions::Condition>),
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
    pub is_ritual: bool,
    pub effect: SpellEffect,
}

impl Spell {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &str,
        level: u8,
        school: SpellSchool,
        casting_time: &str,
        range_feet: u32,
        components: Components,
        duration: &str,
        requires_concentration: bool,
        is_ritual: bool,
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
            is_ritual,
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
        false,
        SpellEffect::SpellAttack {
            damage_dice: "1d10".to_string(),
            damage_type: DamageType::Fire,
            damage_scaling: Some("cantrip".to_string()),
            on_hit_effect: None,
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
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: true,
            damage_dice: Some("8d6".to_string()),
            damage_type: Some(DamageType::Fire),
            damage_scaling: Some("spell_slot".to_string()),
            fail_effect: None,
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
        assert_eq!(
            fireball.components.material,
            Some("A tiny ball of bat guano and sulfur".to_string())
        );
    }
}
