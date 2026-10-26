use crate::rules::character::entity::Entity;
use crate::rules::conditions::Condition;
use crate::rules::equipment::weapons::{DamageType, Weapon};
use serde::{Deserialize, Serialize};

/// Categorias de Tamanho de Criatura no D&D 5e (SRD 5.1, p. 254)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreatureSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl CreatureSize {
    /// Retorna o dado de vida (Hit Die) correspondente ao tamanho da criatura (SRD p. 256)
    pub fn hit_die(&self) -> u8 {
        match self {
            CreatureSize::Tiny => 4,
            CreatureSize::Small => 6,
            CreatureSize::Medium => 8,
            CreatureSize::Large => 10,
            CreatureSize::Huge => 12,
            CreatureSize::Gargantuan => 20,
        }
    }

    /// Retorna o espaço controlado em combate (em pés / feet)
    pub fn space_feet(&self) -> f32 {
        match self {
            CreatureSize::Tiny => 2.5,
            CreatureSize::Small => 5.0,
            CreatureSize::Medium => 5.0,
            CreatureSize::Large => 10.0,
            CreatureSize::Huge => 15.0,
            CreatureSize::Gargantuan => 20.0,
        }
    }
}

/// Tipos Principais de Criatura no SRD 5.1 (p. 254-255)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreatureType {
    Aberration,
    Beast,
    Celestial,
    Construct,
    Dragon,
    Elemental,
    Fey,
    Fiend,
    Giant,
    Humanoid(Option<String>),
    Monstrosity,
    Ooze,
    Plant,
    Undead,
    Custom(String),
}

/// Sentidos Especiais do Monstro (SRD p. 256-257)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonsterSense {
    Blindsight { range_feet: u32, blind_beyond: bool },
    Darkvision { range_feet: u32 },
    Tremorsense { range_feet: u32 },
    Truesight { range_feet: u32 },
    PassivePerception(u8),
}

/// Modificadores de Danos e Condições para Monstros
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MonsterResistances {
    pub damage_vulnerabilities: Vec<DamageType>,
    pub damage_resistances: Vec<DamageType>,
    pub damage_immunities: Vec<DamageType>,
    pub condition_immunities: Vec<Condition>,
    pub nonmagical_physical_resistance: bool,
    pub nonmagical_physical_immunity: bool,
}

/// Suporte para Mecânica de Recarga (Recharge X-Y) (SRD p. 259)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RechargeMechanism {
    pub min_roll: u8, // Ex: 5 para Recharge 5-6, 6 para Recharge 6
    pub is_charged: bool,
}

impl RechargeMechanism {
    pub fn new(min_roll: u8) -> Self {
        Self {
            min_roll,
            is_charged: true, // Começa carregado no início da luta
        }
    }

    /// Rola 1d6 no início do turno para verificar se recarrega
    pub fn roll_recharge(&mut self, roll: u8) -> bool {
        if roll >= self.min_roll {
            self.is_charged = true;
        }
        self.is_charged
    }
}

/// Restrição de uso de habilidades
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UsageLimit {
    AtWill,
    PerDay(u8),
    Recharge(RechargeMechanism),
}

/// Conjuração Inata / Psionismo de Monstros (SRD p. 258-259)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InnateSpellcasting {
    pub spellcasting_ability: String, // Ex: "Charisma", "Intelligence"
    pub dc: u8,
    pub spell_attack_bonus: i8,
    pub is_psionics: bool,
    pub spells: Vec<(String, UsageLimit)>, // (Nome da Magia, Limite de Uso)
}

/// Categoria da Ação de Monstro (SRD p. 258-260)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Action,
    BonusAction,
    Reaction,
    LegendaryAction { cost: u8 },
    LairAction,
}

/// Traço Especial / Habilidade Passiva do Monstro
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonsterTrait {
    pub name: String,
    pub description: String,
}

impl MonsterTrait {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

/// Representa uma Ação de Combate ou Habilidade Ativa do Monstro
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonsterAction {
    pub name: String,
    pub action_type: ActionType,
    pub description: String,
    pub usage: UsageLimit,
    pub weapon: Option<Weapon>,
}

impl MonsterAction {
    pub fn new(
        name: &str,
        action_type: ActionType,
        description: &str,
        weapon: Option<Weapon>,
    ) -> Self {
        Self {
            name: name.to_string(),
            action_type,
            description: description.to_string(),
            usage: UsageLimit::AtWill,
            weapon,
        }
    }

    pub fn with_usage(mut self, usage: UsageLimit) -> Self {
        self.usage = usage;
        self
    }

    pub fn attack(name: &str, weapon: Weapon) -> Self {
        Self::new(name, ActionType::Action, "Ataque com Arma", Some(weapon))
    }
}

/// Ações Lendárias & Guarida (SRD p. 260)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LegendaryDetails {
    pub max_legendary_actions: u8,
    pub current_legendary_actions: u8,
    pub lair_description: Option<String>,
    pub regional_effects: Vec<String>,
}

/// Estrutura Completa de um Monstro / Criatura (SRD 5.1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monster {
    pub id: String,
    pub entity: Entity,
    pub creature_type: CreatureType,
    pub size: CreatureSize,
    pub challenge_rating: f32, // Ex: 0.125 (1/8), 0.25 (1/4), 0.5 (1/2), 1.0, 10.0, 30.0
    pub xp_reward: u32,
    pub senses: Vec<MonsterSense>,
    pub resistances: MonsterResistances,
    pub innate_spellcasting: Option<InnateSpellcasting>,
    pub traits: Vec<MonsterTrait>,
    pub actions: Vec<MonsterAction>,
    pub legendary: Option<LegendaryDetails>,
}

impl Monster {
    pub fn new(
        id: &str,
        entity: Entity,
        creature_type: CreatureType,
        size: CreatureSize,
        challenge_rating: f32,
        xp_reward: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            entity,
            creature_type,
            size,
            challenge_rating,
            xp_reward,
            senses: Vec::new(),
            resistances: MonsterResistances::default(),
            innate_spellcasting: None,
            traits: Vec::new(),
            actions: Vec::new(),
            legendary: None,
        }
    }

    pub fn add_sense(&mut self, sense: MonsterSense) {
        self.senses.push(sense);
    }

    pub fn add_trait(&mut self, name: &str, description: &str) {
        self.traits.push(MonsterTrait::new(name, description));
    }

    pub fn add_action(&mut self, action: MonsterAction) {
        self.actions.push(action);
    }

    pub fn set_legendary(&mut self, count: u8) {
        self.legendary = Some(LegendaryDetails {
            max_legendary_actions: count,
            current_legendary_actions: count,
            lair_description: None,
            regional_effects: Vec::new(),
        });
    }

    pub fn reset_legendary_actions(&mut self) {
        if let Some(ref mut leg) = self.legendary {
            leg.current_legendary_actions = leg.max_legendary_actions;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::background::create_acolyte;
    use crate::rules::character::class::{ClassVariant, Fighter};
    use crate::rules::character::race::{Human, RaceVariant};
    use crate::rules::equipment::weapons::create_dagger;

    #[test]
    fn test_creature_size_hit_die() {
        assert_eq!(CreatureSize::Tiny.hit_die(), 4);
        assert_eq!(CreatureSize::Small.hit_die(), 6);
        assert_eq!(CreatureSize::Medium.hit_die(), 8);
        assert_eq!(CreatureSize::Large.hit_die(), 10);
        assert_eq!(CreatureSize::Huge.hit_die(), 12);
        assert_eq!(CreatureSize::Gargantuan.hit_die(), 20);
    }

    #[test]
    fn test_recharge_mechanic() {
        let mut recharge = RechargeMechanism::new(5);
        assert!(recharge.is_charged);

        recharge.is_charged = false;
        assert!(!recharge.roll_recharge(3));
        assert!(recharge.roll_recharge(5));
        assert!(recharge.is_charged);
    }

    #[test]
    fn test_monster_creation_with_srd_features() {
        let attrs = AbilityScores::new(8, 14, 10, 10, 8, 8);
        let base_entity = Entity::new(
            "Goblin",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let mut goblin = Monster::new(
            "goblin",
            base_entity,
            CreatureType::Humanoid(Some("goblinoid".to_string())),
            CreatureSize::Small,
            0.25,
            50,
        );

        goblin.add_sense(MonsterSense::Darkvision { range_feet: 60 });
        goblin.add_trait(
            "Nimble Escape",
            "Pode usar Disengage ou Hide como Ação Bônus",
        );
        goblin.add_action(MonsterAction::attack("Scimitar", create_dagger()));

        assert_eq!(goblin.id, "goblin");
        assert_eq!(goblin.senses.len(), 1);
        assert_eq!(goblin.traits.len(), 1);
        assert_eq!(goblin.actions.len(), 1);
        assert_eq!(goblin.challenge_rating, 0.25);
    }
}

