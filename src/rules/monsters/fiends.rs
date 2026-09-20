use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::{create_dagger, DamageType};
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, Monster, MonsterAction, MonsterSense, UsageLimit,
};

fn create_fiend_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    )
}

fn create_fiend_base(id: &str, name: &str, size: CreatureSize, cr: f32, xp: u32, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Monster {
    let mut m = Monster::new(id, create_fiend_entity(name, str_, dex, con, int_, wis, cha), CreatureType::Fiend, size, cr, xp);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.resistances.damage_resistances.push(DamageType::Fire);
    m.resistances.damage_immunities.push(DamageType::Poison);
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m
}

pub fn create_balor() -> Monster {
    let mut m = create_fiend_base("balor", "Balor", CreatureSize::Huge, 19.0, 22000, 26, 15, 22, 20, 16, 22);
    m.add_trait("Fire Aura", "Causa 3d6 de fogo a quem o tocar em corpo a corpo.");
    m.add_trait("Death Throes", "Explode ao morrer causando 20d6 de dano de fogo num raio de 30ft.");
    m.add_action(MonsterAction::attack("Longsword", create_dagger()));
    m.add_action(MonsterAction::attack("Whip", create_dagger()));
    m
}

pub fn create_dretch() -> Monster { create_fiend_base("dretch", "Dretch", CreatureSize::Small, 0.25, 50, 11, 11, 12, 5, 8, 3) }
pub fn create_glabrezu() -> Monster { create_fiend_base("glabrezu", "Glabrezu", CreatureSize::Large, 9.0, 5000, 20, 15, 21, 19, 17, 16) }
pub fn create_hezrou() -> Monster { create_fiend_base("hezrou", "Hezrou", CreatureSize::Large, 8.0, 3900, 19, 17, 20, 5, 12, 13) }
pub fn create_marilith() -> Monster { create_fiend_base("marilith", "Marilith", CreatureSize::Large, 16.0, 15000, 18, 20, 20, 18, 16, 18) }
pub fn create_nalfeshnee() -> Monster { create_fiend_base("nalfeshnee", "Nalfeshnee", CreatureSize::Large, 13.0, 10000, 21, 10, 22, 19, 12, 15) }
pub fn create_quasit() -> Monster { create_fiend_base("quasit", "Quasit", CreatureSize::Tiny, 1.0, 200, 5, 17, 10, 7, 10, 10) }
pub fn create_shadow_demon() -> Monster { create_fiend_base("shadow_demon", "Shadow Demon", CreatureSize::Medium, 4.0, 1100, 1, 17, 12, 14, 13, 14) }
pub fn create_vrock() -> Monster { create_fiend_base("vrock", "Vrock", CreatureSize::Large, 6.0, 2300, 19, 15, 18, 8, 13, 8) }

pub fn create_barbed_devil() -> Monster { create_fiend_base("barbed_devil", "Barbed Devil", CreatureSize::Medium, 5.0, 1800, 16, 17, 18, 12, 14, 14) }
pub fn create_bearded_devil() -> Monster { create_fiend_base("bearded_devil", "Bearded Devil", CreatureSize::Medium, 3.0, 700, 16, 15, 15, 9, 11, 11) }
pub fn create_bone_devil() -> Monster { create_fiend_base("bone_devil", "Bone Devil", CreatureSize::Large, 9.0, 5000, 18, 16, 18, 13, 14, 16) }
pub fn create_chain_devil() -> Monster { create_fiend_base("chain_devil", "Chain Devil", CreatureSize::Medium, 8.0, 3900, 18, 15, 18, 11, 12, 14) }
pub fn create_erinyes() -> Monster { create_fiend_base("erinyes", "Erinyes", CreatureSize::Medium, 12.0, 8400, 18, 16, 18, 14, 14, 18) }
pub fn create_horned_devil() -> Monster { create_fiend_base("horned_devil", "Horned Devil", CreatureSize::Large, 11.0, 7200, 22, 17, 21, 12, 16, 17) }
pub fn create_ice_devil() -> Monster { create_fiend_base("ice_devil", "Ice Devil", CreatureSize::Large, 14.0, 11500, 21, 14, 22, 18, 15, 18) }
pub fn create_imp() -> Monster { create_fiend_base("imp", "Imp", CreatureSize::Tiny, 1.0, 200, 6, 17, 13, 11, 12, 14) }
pub fn create_lemure() -> Monster { create_fiend_base("lemure", "Lemure", CreatureSize::Medium, 0.0, 10, 10, 5, 11, 1, 11, 3) }

pub fn create_pit_fiend() -> Monster {
    let mut m = create_fiend_base("pit_fiend", "Pit Fiend", CreatureSize::Large, 20.0, 25000, 26, 14, 24, 22, 18, 24);
    m.add_trait("Fear Aura", "Aura de pavor obriga salvaguarda de WIS a 20ft.");
    m.add_action(MonsterAction::attack("Mace", create_dagger()));
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_hell_hound() -> Monster { create_fiend_base("hell_hound", "Hell Hound", CreatureSize::Medium, 3.0, 700, 17, 12, 14, 6, 13, 6) }
pub fn create_nightmare() -> Monster { create_fiend_base("nightmare", "Nightmare", CreatureSize::Large, 3.0, 700, 18, 15, 16, 10, 13, 15) }
pub fn create_rakshasa() -> Monster { create_fiend_base("rakshasa", "Rakshasa", CreatureSize::Medium, 13.0, 10000, 14, 17, 18, 13, 16, 20) }
pub fn create_succubus_incubus() -> Monster { create_fiend_base("succubus_incubus", "Succubus/Incubus", CreatureSize::Medium, 4.0, 1100, 8, 17, 13, 15, 12, 20) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fiends_creation() {
        let balor = create_balor();
        assert_eq!(balor.id, "balor");
        let pit_fiend = create_pit_fiend();
        assert_eq!(pit_fiend.challenge_rating, 20.0);
    }
}
