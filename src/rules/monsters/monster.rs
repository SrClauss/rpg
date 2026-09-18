use serde::{Deserialize, Serialize};
use crate::rules::character::entity::Entity;
use crate::rules::equipment::weapons::Weapon;

/// Categorias de Tamanho de Criatura no D&D 5e
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreatureSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

/// Tipos Principais de Criatura (Suporta Custom para Homebrew)
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
    Humanoid(String), // Ex: "goblinoid", "felid", "human"
    Monstrosity,
    Ooze,
    Plant,
    Undead,
    Custom(String),   // Ex: "Alien Felídeo"
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

/// Categoria da Ação de Monstro
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Action,
    BonusAction,
    Reaction,
    LegendaryAction,
}

/// Representa uma Ação de Combate ou Habilidade Ativa do Monstro
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonsterAction {
    pub name: String,
    pub action_type: ActionType,
    pub description: String,
    pub weapon: Option<Weapon>,
}

impl MonsterAction {
    pub fn new(name: &str, action_type: ActionType, description: &str, weapon: Option<Weapon>) -> Self {
        Self {
            name: name.to_string(),
            action_type,
            description: description.to_string(),
            weapon,
        }
    }

    pub fn attack(name: &str, weapon: Weapon) -> Self {
        Self::new(name, ActionType::Action, "Ataque com Arma", Some(weapon))
    }
}

/// Estrutura Base de um Monstro / Criatura (Extensível para SRD e Homebrew)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monster {
    pub id: String,
    pub entity: Entity,
    pub creature_type: CreatureType,
    pub size: CreatureSize,
    pub challenge_rating: f32, // Ex: 0.25 (CR 1/4), 0.5 (CR 1/2), 1.0, 10.0
    pub xp_reward: u32,
    pub traits: Vec<MonsterTrait>,
    pub actions: Vec<MonsterAction>,
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
            traits: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn add_trait(&mut self, name: &str, description: &str) {
        self.traits.push(MonsterTrait::new(name, description));
    }

    pub fn add_action(&mut self, action: MonsterAction) {
        self.actions.push(action);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::class::{Fighter, ClassVariant};
    use crate::rules::character::race::{Human, RaceVariant};
    use crate::rules::character::background::create_acolyte;
    use crate::rules::equipment::weapons::create_dagger;

    #[test]
    fn test_monster_creation() {
        let attrs = AbilityScores::new(8, 14, 10, 10, 8, 8);
        let base_entity = Entity::new("Goblin", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());

        let mut goblin = Monster::new(
            "goblin",
            base_entity,
            CreatureType::Humanoid("goblinoid".to_string()),
            CreatureSize::Small,
            0.25,
            50,
        );

        goblin.add_trait("Nimble Escape", "Pode usar Disengage ou Hide como Ação Bônus");
        goblin.add_action(MonsterAction::attack("Scimitar", create_dagger()));

        assert_eq!(goblin.id, "goblin");
        assert_eq!(goblin.traits.len(), 1);
        assert_eq!(goblin.actions.len(), 1);
        assert_eq!(goblin.challenge_rating, 0.25);
    }
}
