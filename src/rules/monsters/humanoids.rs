use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte as create_acolyte_bg;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::create_dagger;
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, Monster, MonsterAction, MonsterSense, UsageLimit,
};

fn create_humanoid_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte_bg(),
    )
}

fn create_humanoid_base(id: &str, name: &str, sub: &str, cr: f32, xp: u32, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Monster {
    let mut m = Monster::new(
        id,
        create_humanoid_entity(name, str_, dex, con, int_, wis, cha),
        CreatureType::Humanoid(Some(sub.to_string())),
        CreatureSize::Medium,
        cr,
        xp,
    );
    m.add_action(MonsterAction::attack("Weapon Attack", create_dagger()));
    m
}

// 1. NPC Statblocks SRD
pub fn create_acolyte() -> Monster { create_humanoid_base("acolyte", "Acolyte", "any race", 0.25, 50, 10, 10, 10, 10, 14, 11) }
pub fn create_archmage() -> Monster {
    let mut m = create_humanoid_base("archmage", "Archmage", "any race", 12.0, 8400, 10, 14, 12, 20, 15, 16);
    m.add_trait("Magic Resistance", "Vantagem em testes de salvaguarda contra magias.");
    m
}
pub fn create_assassin() -> Monster {
    let mut m = create_humanoid_base("assassin", "Assassin", "any race", 8.0, 3900, 11, 16, 14, 13, 11, 10);
    m.add_trait("Assassinate", "Vantagem em ataques contra criaturas que ainda não agiram no combate.");
    m.add_trait("Sneak Attack", "Causa 4d6 de dano extra em ataques com vantagem.");
    m
}
pub fn create_bandit() -> Monster { create_humanoid_base("bandit", "Bandit", "any race", 0.125, 25, 11, 12, 12, 10, 10, 10) }
pub fn create_bandit_captain() -> Monster { create_humanoid_base("bandit_captain", "Bandit Captain", "any race", 2.0, 450, 15, 16, 14, 14, 11, 14) }
pub fn create_berserker() -> Monster {
    let mut m = create_humanoid_base("berserker", "Berserker", "any race", 2.0, 450, 16, 12, 17, 9, 11, 9);
    m.add_trait("Reckless", "Ganha vantagem em ataques corpo a corpo, mas ataques sofridos ganham vantagem.");
    m
}
pub fn create_commoner() -> Monster { create_humanoid_base("commoner", "Commoner", "any race", 0.0, 10, 10, 10, 10, 10, 10, 10) }
pub fn create_cultist() -> Monster { create_humanoid_base("cultist", "Cultist", "any race", 0.125, 25, 11, 12, 10, 10, 11, 10) }
pub fn create_cult_fanatic() -> Monster { create_humanoid_base("cult_fanatic", "Cult Fanatic", "any race", 2.0, 450, 11, 14, 12, 10, 13, 14) }
pub fn create_druid() -> Monster { create_humanoid_base("druid", "Druid", "any race", 2.0, 450, 10, 12, 13, 12, 15, 11) }
pub fn create_gladiator() -> Monster { create_humanoid_base("gladiator", "Gladiator", "any race", 5.0, 1800, 18, 15, 16, 10, 12, 15) }
pub fn create_guard() -> Monster { create_humanoid_base("guard", "Guard", "any race", 0.125, 25, 13, 12, 12, 10, 11, 10) }
pub fn create_knight() -> Monster { create_humanoid_base("knight", "Knight", "any race", 3.0, 700, 16, 11, 14, 11, 11, 15) }
pub fn create_mage() -> Monster { create_humanoid_base("mage", "Mage", "any race", 6.0, 2300, 9, 14, 11, 17, 12, 11) }
pub fn create_noble() -> Monster { create_humanoid_base("noble", "Noble", "any race", 0.125, 25, 11, 12, 11, 12, 14, 16) }
pub fn create_priest() -> Monster { create_humanoid_base("priest", "Priest", "any race", 2.0, 450, 10, 10, 12, 13, 16, 13) }
pub fn create_scout() -> Monster { create_humanoid_base("scout", "Scout", "any race", 0.5, 100, 11, 14, 12, 11, 13, 11) }
pub fn create_spy() -> Monster { create_humanoid_base("spy", "Spy", "any race", 1.0, 200, 10, 15, 10, 12, 14, 16) }
pub fn create_thug() -> Monster { create_humanoid_base("thug", "Thug", "any race", 0.5, 100, 15, 11, 14, 10, 10, 11) }
pub fn create_tribal_warrior() -> Monster { create_humanoid_base("tribal_warrior", "Tribal Warrior", "any race", 0.125, 25, 13, 11, 12, 8, 11, 8) }
pub fn create_veteran() -> Monster { create_humanoid_base("veteran", "Veteran", "any race", 3.0, 700, 16, 13, 14, 10, 11, 10) }

// 2. Raças Humanóides Específicas
pub fn create_bugbear() -> Monster {
    let mut m = create_humanoid_base("bugbear", "Bugbear", "goblinoid", 1.0, 200, 15, 14, 13, 8, 11, 9);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Surprise Attack", "Causa 2d6 de dano extra se surpreender o alvo no 1º turno.");
    m
}

pub fn create_duergar() -> Monster {
    let mut m = create_humanoid_base("duergar", "Duergar", "dwarf", 1.0, 200, 14, 11, 14, 11, 10, 9);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.add_trait("Duergar Resilience", "Vantagem contra ilusões, paralisação e encantamento.");
    m.add_action(MonsterAction::new("Enlarge", ActionType::Action, "Cresce de tamanho e ganha bônus de dano.", None).with_usage(UsageLimit::PerDay(1)));
    m.add_action(MonsterAction::new("Invisibility", ActionType::Action, "Fica invisível.", None).with_usage(UsageLimit::PerDay(1)));
    m
}

pub fn create_drow() -> Monster {
    let mut m = create_humanoid_base("drow", "Elf (Drow)", "elf", 0.25, 50, 10, 14, 10, 11, 11, 12);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.add_trait("Fey Ancestry", "Vantagem contra ser encantado e imune a sono mágico.");
    m.add_trait("Sunlight Sensitivity", "Desvantagem em ataques sob luz do sol.");
    m
}

pub fn create_gnoll() -> Monster {
    let mut m = create_humanoid_base("gnoll", "Gnoll", "gnoll", 0.5, 100, 14, 12, 11, 6, 10, 7);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Rampage", "Ação bônus para mover e morder ao reduzir alvo a 0 HP.");
    m
}

pub fn create_goblin_humanoid() -> Monster {
    let mut m = create_humanoid_base("goblin_srd", "Goblin", "goblinoid", 0.25, 50, 8, 14, 10, 10, 8, 8);
    m.size = CreatureSize::Small;
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Nimble Escape", "Disengage ou Hide como Ação Bônus.");
    m
}

pub fn create_grimlock() -> Monster {
    let mut m = create_humanoid_base("grimlock", "Grimlock", "grimlock", 0.25, 50, 16, 12, 12, 9, 8, 6);
    m.add_sense(MonsterSense::Blindsight { range_feet: 30, blind_beyond: true });
    m.add_trait("Blind Senses", "Cego além do alcance da percepção às cegas.");
    m
}

pub fn create_hobgoblin() -> Monster {
    let mut m = create_humanoid_base("hobgoblin", "Hobgoblin", "goblinoid", 0.5, 100, 13, 12, 12, 10, 10, 9);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Martial Advantage", "Causa 2d6 extra 1/turno se aliado estiver a 5ft do alvo.");
    m
}

pub fn create_kobold() -> Monster {
    let mut m = create_humanoid_base("kobold", "Kobold", "kobold", 0.125, 25, 7, 15, 9, 8, 7, 8);
    m.size = CreatureSize::Small;
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Pack Tactics", "Vantagem em ataques com aliado a 5ft.");
    m.add_trait("Sunlight Sensitivity", "Desvantagem sob luz solar.");
    m
}

pub fn create_lizardfolk() -> Monster {
    let mut m = create_humanoid_base("lizardfolk", "Lizardfolk", "lizardfolk", 0.5, 100, 15, 10, 13, 7, 12, 7);
    m.add_trait("Hold Breath", "Pode prender a respiração por 15 minutos.");
    m
}

pub fn create_merfolk() -> Monster {
    let mut m = create_humanoid_base("merfolk", "Merfolk", "merfolk", 0.125, 25, 10, 13, 12, 11, 11, 12);
    m.add_trait("Amphibious", "Respira ar e água.");
    m
}

pub fn create_orc_humanoid() -> Monster {
    let mut m = create_humanoid_base("orc_srd", "Orc", "orc", 0.5, 100, 16, 12, 16, 7, 11, 10);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Aggressive", "Move-se em direção ao inimigo como ação bônus.");
    m
}

pub fn create_sahuagin() -> Monster {
    let mut m = create_humanoid_base("sahuagin", "Sahuagin", "sahuagin", 0.5, 100, 13, 11, 12, 12, 13, 9);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.add_trait("Blood Frenzy", "Vantagem em ataques contra alvos feridos.");
    m
}

// 3. Licantropos (Were-creatures)
pub fn create_werebear() -> Monster {
    let mut m = create_humanoid_base("werebear", "Werebear", "human, shapechanger", 5.0, 1800, 19, 10, 17, 11, 12, 12);
    m.resistances.nonmagical_physical_immunity = true;
    m.add_action(MonsterAction::new("Shapechange", ActionType::Action, "Transforma-se em Urso ou forma Híbrida.", None));
    m
}

pub fn create_wereboar() -> Monster {
    let mut m = create_humanoid_base("wereboar", "Wereboar", "human, shapechanger", 4.0, 1100, 17, 10, 15, 10, 11, 8);
    m.resistances.nonmagical_physical_immunity = true;
    m.add_action(MonsterAction::new("Shapechange", ActionType::Action, "Transforma-se em Javali ou forma Híbrida.", None));
    m
}

pub fn create_wererat() -> Monster {
    let mut m = create_humanoid_base("wererat", "Wererat", "human, shapechanger", 2.0, 450, 10, 15, 12, 11, 10, 8);
    m.resistances.nonmagical_physical_immunity = true;
    m.add_action(MonsterAction::new("Shapechange", ActionType::Action, "Transforma-se em Rato Gigante ou forma Híbrida.", None));
    m
}

pub fn create_weretiger() -> Monster {
    let mut m = create_humanoid_base("weretiger", "Weretiger", "human, shapechanger", 4.0, 1100, 17, 15, 16, 10, 13, 11);
    m.resistances.nonmagical_physical_immunity = true;
    m.add_action(MonsterAction::new("Shapechange", ActionType::Action, "Transforma-se em Tigre ou forma Híbrida.", None));
    m
}

pub fn create_werewolf() -> Monster {
    let mut m = create_humanoid_base("werewolf", "Werewolf", "human, shapechanger", 3.0, 700, 15, 13, 14, 10, 11, 10);
    m.resistances.nonmagical_physical_immunity = true;
    m.add_action(MonsterAction::new("Shapechange", ActionType::Action, "Transforma-se em Lobo ou forma Híbrida.", None));
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_humanoids_creation() {
        let archmage = create_archmage();
        assert_eq!(archmage.id, "archmage");
        let werewolf = create_werewolf();
        assert_eq!(werewolf.challenge_rating, 3.0);
        assert!(werewolf.resistances.nonmagical_physical_immunity);
    }
}
