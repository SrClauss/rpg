use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{Fighter, ClassVariant};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::{create_dagger, create_longsword, create_greatsword, DamageType, Weapon};
use crate::rules::monsters::monster::{ActionType, CreatureSize, CreatureType, Monster, MonsterAction};

/// Registro Central de Monstros (Suporta SRD Padrão e Conteúdo Homebrew/JSON)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BestiaryRegistry {
    monsters: HashMap<String, Monster>,
}

impl BestiaryRegistry {
    /// Inicializa o repositório registrando automaticamente os monstros base do SRD
    pub fn new() -> Self {
        let mut registry = Self {
            monsters: HashMap::new(),
        };
        registry.load_srd_defaults();
        registry
    }

    /// Registra um novo monstro no catálogo
    pub fn register_monster(&mut self, monster: Monster) {
        self.monsters.insert(monster.id.clone(), monster);
    }

    /// Retorna a referência de um monstro cadastrado pelo ID
    pub fn get_monster(&self, id: &str) -> Option<&Monster> {
        self.monsters.get(id)
    }

    /// Clona e instancia um novo monstro pelo ID para ser usado em combate
    pub fn spawn_monster(&self, id: &str) -> Option<Monster> {
        self.monsters.get(id).cloned()
    }

    /// Retorna todos os IDs cadastrados no repositório
    pub fn list_monster_ids(&self) -> Vec<String> {
        self.monsters.keys().cloned().collect()
    }

    /// Carrega e registra uma lista de monstros a partir de uma string em formato JSON (Homebrew)
    pub fn load_json(&mut self, json_str: &str) -> Result<usize, String> {
        let list: Vec<Monster> = serde_json::from_str(json_str)
            .map_err(|e| format!("Erro na desserialização de monstros JSON: {}", e))?;
        
        let count = list.len();
        for monster in list {
            self.register_monster(monster);
        }
        Ok(count)
    }

    /// Carrega os monstros iniciais padrão do SRD D&D 5e
    fn load_srd_defaults(&mut self) {
        // 1. Goblin (CR 1/4)
        let goblin_attrs = AbilityScores::new(8, 14, 10, 10, 8, 8);
        let goblin_entity = Entity::new("Goblin", goblin_attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());
        let mut goblin = Monster::new("goblin", goblin_entity, CreatureType::Humanoid("goblinoid".to_string()), CreatureSize::Small, 0.25, 50);
        goblin.add_trait("Nimble Escape", "Pode usar as ações Disengage ou Hide como Ação Bônus");
        goblin.add_action(MonsterAction::attack("Scimitar", create_dagger()));
        self.register_monster(goblin);

        // 2. Orc (CR 1/2)
        let orc_attrs = AbilityScores::new(16, 12, 16, 7, 11, 10);
        let orc_entity = Entity::new("Orc", orc_attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());
        let mut orc = Monster::new("orc", orc_entity, CreatureType::Humanoid("orc".to_string()), CreatureSize::Medium, 0.5, 100);
        orc.add_trait("Aggressive", "Como ação bônus, pode se mover até seu deslocamento em direção a um inimigo");
        orc.add_action(MonsterAction::attack("Greataxe", create_greatsword()));
        self.register_monster(orc);

        // 3. Zombie (CR 1/4)
        let zombie_attrs = AbilityScores::new(13, 6, 16, 3, 6, 5);
        let zombie_entity = Entity::new("Zombie", zombie_attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());
        let mut zombie = Monster::new("zombie", zombie_entity, CreatureType::Undead, CreatureSize::Medium, 0.25, 50);
        zombie.add_trait("Undead Fortitude", "Se dano não-radiante/crítico reduzir a 0 HP, faz Salvaguarda de CON (CD 5+dano) para ficar com 1 HP");
        zombie.add_action(MonsterAction::attack("Slam", create_longsword()));
        self.register_monster(zombie);

        // 4. Young Red Dragon (CR 10)
        let dragon_attrs = AbilityScores::new(23, 10, 21, 14, 11, 19);
        let dragon_entity = Entity::new("Young Red Dragon", dragon_attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());
        let mut dragon = Monster::new("young_red_dragon", dragon_entity, CreatureType::Dragon, CreatureSize::Large, 10.0, 5900);
        dragon.add_trait("Fire Immunity", "Imune a dano de Fogo");
        dragon.add_action(MonsterAction::new(
            "Fire Breath",
            ActionType::Action,
            "Sopro de Fogo em cone de 30ft (16d6 de dano de Fogo, CD 17 DEX para metade)",
            None,
        ));
        self.register_monster(dragon);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_srd_defaults() {
        let registry = BestiaryRegistry::new();
        assert!(registry.get_monster("goblin").is_some());
        assert!(registry.get_monster("orc").is_some());
        assert!(registry.get_monster("zombie").is_some());
        assert!(registry.get_monster("young_red_dragon").is_some());

        let spawned = registry.spawn_monster("goblin").unwrap();
        assert_eq!(spawned.entity.name, "Goblin");
    }

    #[test]
    fn test_load_homebrew_json() {
        let mut registry = BestiaryRegistry::new();

        let attrs = AbilityScores::new(18, 18, 16, 12, 14, 16);
        let base_entity = Entity::new("Lion-O, Rei dos Felídeos", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());
        let mut lion_o_original = Monster::new("lion_o", base_entity, CreatureType::Custom("Felídeo".to_string()), CreatureSize::Medium, 8.0, 3900);
        lion_o_original.add_trait("Visão Além do Alcance", "Revela invisibilidade e ilusões");

        let json_str = serde_json::to_string(&vec![lion_o_original]).unwrap();

        let loaded = registry.load_json(&json_str);
        assert!(loaded.is_ok());
        assert_eq!(loaded.unwrap(), 1);

        let lion_o = registry.spawn_monster("lion_o").unwrap();
        assert_eq!(lion_o.entity.name, "Lion-O, Rei dos Felídeos");
        assert_eq!(lion_o.traits[0].name, "Visão Além do Alcance");
    }
}
