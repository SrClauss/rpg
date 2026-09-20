use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::{create_dagger, DamageType};
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, Monster, MonsterAction, MonsterSense, UsageLimit,
};

fn create_undead_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    )
}

fn create_undead_base(id: &str, name: &str, size: CreatureSize, cr: f32, xp: u32, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Monster {
    let mut m = Monster::new(id, create_undead_entity(name, str_, dex, con, int_, wis, cha), CreatureType::Undead, size, cr, xp);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.resistances.damage_immunities.push(DamageType::Poison);
    m.add_action(MonsterAction::attack("Attack", create_dagger()));
    m
}

pub fn create_banshee() -> Monster {
    let mut m = create_undead_base("banshee", "Banshee", CreatureSize::Medium, 4.0, 1100, 1, 14, 10, 12, 11, 17);
    m.resistances.damage_immunities.push(DamageType::Necrotic);
    m.add_action(MonsterAction::new("Wail", ActionType::Action, "Grito mortal obriga salvaguarda de CON a 30ft ou cai para 0 HP (1/Day).", None).with_usage(UsageLimit::PerDay(1)));
    m
}

pub fn create_ghoul() -> Monster {
    let mut m = create_undead_base("ghoul", "Ghoul", CreatureSize::Medium, 1.0, 200, 13, 15, 10, 7, 10, 6);
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m
}

pub fn create_ghast() -> Monster {
    let mut m = create_undead_base("ghast", "Ghast", CreatureSize::Medium, 2.0, 450, 16, 17, 10, 11, 10, 8);
    m.add_trait("Stench", "Aura de fedor a 5ft envenena alvos que falharem no teste de CON.");
    m
}

pub fn create_ghost() -> Monster {
    let mut m = create_undead_base("ghost", "Ghost", CreatureSize::Medium, 4.0, 1100, 7, 13, 10, 10, 12, 17);
    m.resistances.nonmagical_physical_resistance = true;
    m.add_trait("Ethereal Sight", "Enxerga o plano Etéreo a 60ft.");
    m.add_trait("Incorporeal Movement", "Passa por criaturas e objetos como terreno difícil.");
    m.add_action(MonsterAction::new("Possession", ActionType::Action, "Possui corpo de um humanoide (Recharge 6).", None));
    m
}

pub fn create_lich() -> Monster {
    let mut m = create_undead_base("lich", "Lich", CreatureSize::Medium, 21.0, 33000, 11, 16, 16, 20, 14, 16);
    m.add_sense(MonsterSense::Truesight { range_feet: 120 });
    m.resistances.damage_immunities.push(DamageType::Lightning);
    m.resistances.damage_immunities.push(DamageType::Cold);
    m.resistances.nonmagical_physical_immunity = true;
    m.add_trait("Legendary Resistance", "Escolha passar em salvaguarda falha (3/Day).");
    m.add_trait("Rejuvenating", "Se for destruído, seu filactério constrói um novo corpo em 1d10 dias.");
    m.set_legendary(3);
    m
}

pub fn create_minotaur_skeleton() -> Monster { create_undead_base("minotaur_skeleton", "Minotaur Skeleton", CreatureSize::Large, 2.0, 450, 18, 11, 15, 6, 8, 5) }
pub fn create_mummy() -> Monster {
    let mut m = create_undead_base("mummy", "Mummy", CreatureSize::Medium, 3.0, 700, 16, 8, 15, 6, 10, 12);
    m.resistances.damage_vulnerabilities.push(DamageType::Fire);
    m.add_trait("Dreadful Glare", "Aterroriza e paralisa alvos com o olhar.");
    m
}

pub fn create_mummy_lord() -> Monster {
    let mut m = create_undead_base("mummy_lord", "Mummy Lord", CreatureSize::Medium, 15.0, 13000, 18, 10, 17, 16, 18, 16);
    m.resistances.damage_vulnerabilities.push(DamageType::Fire);
    m.set_legendary(3);
    m
}

pub fn create_ogre_zombie() -> Monster { create_undead_base("ogre_zombie", "Ogre Zombie", CreatureSize::Large, 2.0, 450, 19, 6, 18, 3, 6, 5) }
pub fn create_shadow() -> Monster {
    let mut m = create_undead_base("shadow", "Shadow", CreatureSize::Medium, 0.5, 100, 6, 14, 13, 6, 10, 8);
    m.add_trait("Strength Drain", "Ataques reduzem o atributo de Força da vítima em 1d4.");
    m
}

pub fn create_skeleton() -> Monster {
    let mut m = create_undead_base("skeleton", "Skeleton", CreatureSize::Medium, 0.25, 50, 10, 14, 15, 6, 8, 5);
    m.resistances.damage_vulnerabilities.push(DamageType::Bludgeoning);
    m
}

pub fn create_specter() -> Monster { create_undead_base("specter", "Specter", CreatureSize::Medium, 1.0, 200, 1, 14, 11, 10, 10, 11) }

pub fn create_vampire() -> Monster {
    let mut m = create_undead_base("vampire", "Vampire", CreatureSize::Medium, 13.0, 10000, 18, 18, 18, 17, 15, 18);
    m.add_trait("Misty Escape", "Transforma-se em névoa ao ser reduzido a 0 HP.");
    m.add_trait("Regeneration", "Recupera 20 HP por turno se não estiver sob luz solar ou água corrente.");
    m.add_trait("Vampire Weaknesses", "Toma dano em água corrente e luz solar.");
    m.set_legendary(3);
    m
}

pub fn create_vampire_spawn() -> Monster { create_undead_base("vampire_spawn", "Vampire Spawn", CreatureSize::Medium, 5.0, 1800, 16, 16, 16, 11, 10, 12) }
pub fn create_wight() -> Monster { create_undead_base("wight", "Wight", CreatureSize::Medium, 3.0, 700, 15, 14, 16, 10, 13, 15) }
pub fn create_wraith() -> Monster { create_undead_base("wraith", "Wraith", CreatureSize::Medium, 5.0, 1800, 6, 16, 16, 12, 14, 15) }
pub fn create_zombie_srd() -> Monster { create_undead_base("zombie_srd", "Zombie", CreatureSize::Medium, 0.25, 50, 13, 6, 16, 3, 6, 5) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undead_creation() {
        let lich = create_lich();
        assert_eq!(lich.id, "lich");
        assert_eq!(lich.challenge_rating, 21.0);
        let vampire = create_vampire();
        assert_eq!(vampire.challenge_rating, 13.0);
    }
}
