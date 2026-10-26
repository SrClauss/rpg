use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte as create_acolyte_bg;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::{
    create_dagger, create_greatsword, create_longsword,
};
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, Monster, MonsterAction,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        use crate::rules::monsters::aberrations::*;
        use crate::rules::monsters::beasts::*;
        use crate::rules::monsters::celestials::*;
        use crate::rules::monsters::constructs::*;
        use crate::rules::monsters::dragons::*;
        use crate::rules::monsters::elementals::*;
        use crate::rules::monsters::fey::*;
        use crate::rules::monsters::fiends::*;
        use crate::rules::monsters::giants::*;
        use crate::rules::monsters::humanoids::*;
        use crate::rules::monsters::monstrosities::*;
        use crate::rules::monsters::oozes::*;
        use crate::rules::monsters::plants::*;
        use crate::rules::monsters::undead::*;

        // Oozes
        self.register_monster(create_black_pudding());
        self.register_monster(create_gelatinous_cube());
        self.register_monster(create_gray_ooze());
        self.register_monster(create_ochre_jelly());

        // Plants
        self.register_monster(create_awakened_shrub());
        self.register_monster(create_awakened_tree());
        self.register_monster(create_shambling_mound());
        self.register_monster(create_treant());

        // Undead
        self.register_monster(create_banshee());
        self.register_monster(create_ghoul());
        self.register_monster(create_ghast());
        self.register_monster(create_ghost());
        self.register_monster(create_lich());
        self.register_monster(create_minotaur_skeleton());
        self.register_monster(create_mummy());
        self.register_monster(create_mummy_lord());
        self.register_monster(create_ogre_zombie());
        self.register_monster(create_shadow());
        self.register_monster(create_skeleton());
        self.register_monster(create_specter());
        self.register_monster(create_vampire());
        self.register_monster(create_vampire_spawn());
        self.register_monster(create_wight());
        self.register_monster(create_wraith());
        self.register_monster(create_zombie_srd());

        // Monstrosities
        self.register_monster(create_ankheg());
        self.register_monster(create_basilisk());
        self.register_monster(create_behir());
        self.register_monster(create_bulette());
        self.register_monster(create_centaur());
        self.register_monster(create_chimera());
        self.register_monster(create_cockatrice());
        self.register_monster(create_darkmantle());
        self.register_monster(create_displacer_beast());
        self.register_monster(create_doppelganger());
        self.register_monster(create_drider());
        self.register_monster(create_gorgon());
        self.register_monster(create_grick());
        self.register_monster(create_griffon());
        self.register_monster(create_harpy());
        self.register_monster(create_hippogriff());
        self.register_monster(create_hydra());
        self.register_monster(create_kraken());
        self.register_monster(create_lamia());
        self.register_monster(create_manticore());
        self.register_monster(create_medusa());
        self.register_monster(create_minotaur());
        self.register_monster(create_naga());
        self.register_monster(create_owlbear());
        self.register_monster(create_purple_worm());
        self.register_monster(create_remorhaz());
        self.register_monster(create_roc());
        self.register_monster(create_rust_monster());
        self.register_monster(create_sphinx());
        self.register_monster(create_tarrasque());
        self.register_monster(create_unicorn());
        self.register_monster(create_yuan_ti_pureblood());

        // Humanoids & NPCs
        self.register_monster(create_acolyte());
        self.register_monster(create_archmage());
        self.register_monster(create_assassin());
        self.register_monster(create_bandit());
        self.register_monster(create_bandit_captain());
        self.register_monster(create_berserker());
        self.register_monster(create_commoner());
        self.register_monster(create_cultist());
        self.register_monster(create_cult_fanatic());
        self.register_monster(create_druid());
        self.register_monster(create_gladiator());
        self.register_monster(create_guard());
        self.register_monster(create_knight());
        self.register_monster(create_mage());
        self.register_monster(create_noble());
        self.register_monster(create_priest());
        self.register_monster(create_scout());
        self.register_monster(create_spy());
        self.register_monster(create_thug());
        self.register_monster(create_tribal_warrior());
        self.register_monster(create_veteran());
        self.register_monster(create_bugbear());
        self.register_monster(create_duergar());
        self.register_monster(create_drow());
        self.register_monster(create_gnoll());
        self.register_monster(create_goblin_humanoid());
        self.register_monster(create_grimlock());
        self.register_monster(create_hobgoblin());
        self.register_monster(create_kobold());
        self.register_monster(create_lizardfolk());
        self.register_monster(create_merfolk());
        self.register_monster(create_orc_humanoid());
        self.register_monster(create_sahuagin());
        self.register_monster(create_werebear());
        self.register_monster(create_wereboar());
        self.register_monster(create_wererat());
        self.register_monster(create_weretiger());
        self.register_monster(create_werewolf());

        // Elementals
        self.register_monster(create_air_elemental());
        self.register_monster(create_earth_elemental());
        self.register_monster(create_fire_elemental());
        self.register_monster(create_water_elemental());
        self.register_monster(create_azer());
        self.register_monster(create_djinni());
        self.register_monster(create_efreeti());
        self.register_monster(create_gargoyle());
        self.register_monster(create_invisible_stalker());
        self.register_monster(create_magmin());
        self.register_monster(create_mephits());
        self.register_monster(create_salamander());
        self.register_monster(create_xorn());

        // Fey
        self.register_monster(create_dryad());
        self.register_monster(create_green_hag());
        self.register_monster(create_pixie());
        self.register_monster(create_satyr());

        // Fiends
        self.register_monster(create_balor());
        self.register_monster(create_dretch());
        self.register_monster(create_glabrezu());
        self.register_monster(create_hezrou());
        self.register_monster(create_marilith());
        self.register_monster(create_nalfeshnee());
        self.register_monster(create_quasit());
        self.register_monster(create_shadow_demon());
        self.register_monster(create_vrock());
        self.register_monster(create_barbed_devil());
        self.register_monster(create_bearded_devil());
        self.register_monster(create_bone_devil());
        self.register_monster(create_chain_devil());
        self.register_monster(create_erinyes());
        self.register_monster(create_horned_devil());
        self.register_monster(create_ice_devil());
        self.register_monster(create_imp());
        self.register_monster(create_lemure());
        self.register_monster(create_pit_fiend());
        self.register_monster(create_hell_hound());
        self.register_monster(create_nightmare());
        self.register_monster(create_rakshasa());
        self.register_monster(create_succubus_incubus());

        // Giants
        self.register_monster(create_cloud_giant());
        self.register_monster(create_fire_giant());
        self.register_monster(create_frost_giant());
        self.register_monster(create_hill_giant());
        self.register_monster(create_stone_giant());
        self.register_monster(create_storm_giant());
        self.register_monster(create_ettin());
        self.register_monster(create_ogre());
        self.register_monster(create_oni());
        self.register_monster(create_troll());

        // Celestials
        self.register_monster(create_deva());
        self.register_monster(create_planetar());
        self.register_monster(create_solar());
        self.register_monster(create_couatl());
        self.register_monster(create_pegasus());

        // Constructs
        self.register_monster(create_animated_armor());
        self.register_monster(create_flying_sword());
        self.register_monster(create_rug_of_smothering());
        self.register_monster(create_clay_golem());
        self.register_monster(create_flesh_golem());
        self.register_monster(create_iron_golem());
        self.register_monster(create_stone_golem());
        self.register_monster(create_homunculus());
        self.register_monster(create_shield_guardian());

        // Dragons
        self.register_monster(create_black_dragon_wyrmling());
        self.register_monster(create_young_black_dragon());
        self.register_monster(create_adult_black_dragon());
        self.register_monster(create_ancient_black_dragon());
        self.register_monster(create_blue_dragon_wyrmling());
        self.register_monster(create_young_blue_dragon());
        self.register_monster(create_adult_blue_dragon());
        self.register_monster(create_ancient_blue_dragon());
        self.register_monster(create_red_dragon_wyrmling());
        self.register_monster(create_young_red_dragon());
        self.register_monster(create_adult_red_dragon());
        self.register_monster(create_ancient_red_dragon());
        self.register_monster(create_white_dragon_wyrmling());
        self.register_monster(create_young_white_dragon());
        self.register_monster(create_adult_white_dragon());
        self.register_monster(create_ancient_white_dragon());
        self.register_monster(create_green_dragon_wyrmling());
        self.register_monster(create_young_green_dragon());
        self.register_monster(create_adult_green_dragon());
        self.register_monster(create_ancient_green_dragon());
        self.register_monster(create_dragon_turtle());
        self.register_monster(create_pseudodragon());
        self.register_monster(create_wyvern());

        // Aberrations & Beasts
        self.register_monster(create_aboleth());
        self.register_monster(create_cloaker());
        self.register_monster(create_gibbering_mouther());
        self.register_monster(create_otyugh());
        self.register_monster(create_roper());
        self.register_monster(create_will_o_wisp());

        self.register_monster(create_ape());
        self.register_monster(create_baboon());
        self.register_monster(create_badger());
        self.register_monster(create_black_bear());
        self.register_monster(create_boar());
        self.register_monster(create_brown_bear());
        self.register_monster(create_constrictor_snake());
        self.register_monster(create_crocodile());
        self.register_monster(create_dire_wolf());
        self.register_monster(create_draft_horse());
        self.register_monster(create_eagle());
        self.register_monster(create_elephant());
        self.register_monster(create_giant_ape());
        self.register_monster(create_giant_badger());
        self.register_monster(create_giant_bat());
        self.register_monster(create_giant_boar());
        self.register_monster(create_giant_centipede());
        self.register_monster(create_giant_constrictor_snake());
        self.register_monster(create_giant_crab());
        self.register_monster(create_giant_crocodile());
        self.register_monster(create_giant_eagle());
        self.register_monster(create_giant_elk());
        self.register_monster(create_giant_fire_beetle());
        self.register_monster(create_giant_frog());
        self.register_monster(create_giant_goat());
        self.register_monster(create_giant_hyena());
        self.register_monster(create_giant_lizard());
        self.register_monster(create_giant_octopus());
        self.register_monster(create_giant_owl());
        self.register_monster(create_giant_poisonous_snake());
        self.register_monster(create_giant_rat());
        self.register_monster(create_giant_raven());
        self.register_monster(create_giant_scorpion());
        self.register_monster(create_giant_shark());
        self.register_monster(create_giant_spider());
        self.register_monster(create_giant_toad());
        self.register_monster(create_giant_vulture());
        self.register_monster(create_giant_wasp());
        self.register_monster(create_giant_weasel());
        self.register_monster(create_giant_wolf_spider());
        self.register_monster(create_goat());
        self.register_monster(create_hawk());
        self.register_monster(create_hunter_shark());
        self.register_monster(create_hyena());
        self.register_monster(create_jackal());
        self.register_monster(create_killer_whale());
        self.register_monster(create_lion());
        self.register_monster(create_lizard());
        self.register_monster(create_mule());
        self.register_monster(create_octopus());
        self.register_monster(create_owl());
        self.register_monster(create_panther());
        self.register_monster(create_phase_spider());
        self.register_monster(create_plesiosaurus());
        self.register_monster(create_poisonous_snake());
        self.register_monster(create_pony());
        self.register_monster(create_reef_shark());
        self.register_monster(create_rhinoceros());
        self.register_monster(create_riding_horse());
        self.register_monster(create_saber_toothed_tiger());
        self.register_monster(create_scorpion());
        self.register_monster(create_sea_horse());
        self.register_monster(create_spider());
        self.register_monster(create_tiger());
        self.register_monster(create_tyrannosaurus_rex());
        self.register_monster(create_vulture());
        self.register_monster(create_warhorse());
        self.register_monster(create_weasel());
        self.register_monster(create_wolf());
        // 1. Goblin (CR 1/4)
        let goblin_attrs = AbilityScores::new(8, 14, 10, 10, 8, 8);
        let goblin_entity = Entity::new(
            "Goblin",
            goblin_attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte_bg(),
        );
        let mut goblin = Monster::new(
            "goblin",
            goblin_entity,
            CreatureType::Humanoid(Some("goblinoid".to_string())),
            CreatureSize::Small,
            0.25,
            50,
        );
        goblin.add_trait(
            "Nimble Escape",
            "Pode usar as ações Disengage ou Hide como Ação Bônus",
        );
        goblin.add_action(MonsterAction::attack("Scimitar", create_dagger()));
        self.register_monster(goblin);

        // 2. Orc (CR 1/2)
        let orc_attrs = AbilityScores::new(16, 12, 16, 7, 11, 10);
        let orc_entity = Entity::new(
            "Orc",
            orc_attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte_bg(),
        );
        let mut orc = Monster::new(
            "orc",
            orc_entity,
            CreatureType::Humanoid(Some("orc".to_string())),
            CreatureSize::Medium,
            0.5,
            100,
        );
        orc.add_trait(
            "Aggressive",
            "Como ação bônus, pode se mover até seu deslocamento em direção a um inimigo",
        );
        orc.add_action(MonsterAction::attack("Greataxe", create_greatsword()));
        self.register_monster(orc);

        // 3. Zombie (CR 1/4)
        let zombie_attrs = AbilityScores::new(13, 6, 16, 3, 6, 5);
        let zombie_entity = Entity::new(
            "Zombie",
            zombie_attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte_bg(),
        );
        let mut zombie = Monster::new(
            "zombie",
            zombie_entity,
            CreatureType::Undead,
            CreatureSize::Medium,
            0.25,
            50,
        );
        zombie.add_trait("Undead Fortitude", "Se dano não-radiante/crítico reduzir a 0 HP, faz Salvaguarda de CON (CD 5+dano) para ficar com 1 HP");
        zombie.add_action(MonsterAction::attack("Slam", create_longsword()));
        self.register_monster(zombie);

        // 4. Young Red Dragon (CR 10)
        let dragon_attrs = AbilityScores::new(23, 10, 21, 14, 11, 19);
        let dragon_entity = Entity::new(
            "Young Red Dragon",
            dragon_attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte_bg(),
        );
        let mut dragon = Monster::new(
            "young_red_dragon",
            dragon_entity,
            CreatureType::Dragon,
            CreatureSize::Large,
            10.0,
            5900,
        );
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
        let base_entity = Entity::new(
            "Lion-O, Rei dos Felídeos",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte_bg(),
        );
        let mut lion_o_original = Monster::new(
            "lion_o",
            base_entity,
            CreatureType::Custom("Felídeo".to_string()),
            CreatureSize::Medium,
            8.0,
            3900,
        );
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
