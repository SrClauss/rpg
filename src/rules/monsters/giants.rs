use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::create_dagger;
use crate::rules::monsters::monster::{CreatureSize, CreatureType, Monster, MonsterAction, MonsterSense};

fn create_giant_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    )
}

fn create_giant_base(id: &str, name: &str, size: CreatureSize, cr: f32, xp: u32, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Monster {
    let mut m = Monster::new(id, create_giant_entity(name, str_, dex, con, int_, wis, cha), CreatureType::Giant, size, cr, xp);
    m.add_action(MonsterAction::attack("Greatclub", create_dagger()));
    m.add_action(MonsterAction::attack("Rock", create_dagger()));
    m
}

pub fn create_cloud_giant() -> Monster { create_giant_base("cloud_giant", "Cloud Giant", CreatureSize::Huge, 9.0, 5000, 27, 10, 22, 12, 16, 16) }
pub fn create_fire_giant() -> Monster { create_giant_base("fire_giant", "Fire Giant", CreatureSize::Huge, 9.0, 5000, 25, 9, 23, 10, 14, 13) }
pub fn create_frost_giant() -> Monster { create_giant_base("frost_giant", "Frost Giant", CreatureSize::Huge, 8.0, 3900, 23, 9, 21, 9, 10, 12) }
pub fn create_hill_giant() -> Monster { create_giant_base("hill_giant", "Hill Giant", CreatureSize::Huge, 5.0, 1800, 21, 8, 19, 5, 9, 6) }
pub fn create_stone_giant() -> Monster { create_giant_base("stone_giant", "Stone Giant", CreatureSize::Huge, 7.0, 2900, 23, 15, 20, 10, 12, 9) }
pub fn create_storm_giant() -> Monster { create_giant_base("storm_giant", "Storm Giant", CreatureSize::Huge, 13.0, 10000, 29, 14, 20, 16, 18, 18) }
pub fn create_ettin() -> Monster {
    let mut m = create_giant_base("ettin", "Ettin", CreatureSize::Large, 4.0, 1100, 21, 8, 17, 6, 10, 8);
    m.add_trait("Two Heads", "Vantagem em testes de Percepção e salvaguardas contra ficar cego/surdo/atordoado.");
    m
}
pub fn create_ogre() -> Monster { create_giant_base("ogre", "Ogre", CreatureSize::Large, 2.0, 450, 19, 8, 16, 5, 7, 7) }
pub fn create_oni() -> Monster {
    let mut m = create_giant_base("oni", "Oni", CreatureSize::Large, 7.0, 2900, 19, 11, 16, 14, 12, 15);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Regeneration", "Recupera 10 HP no início de seu turno se tiver pelo menos 1 HP.");
    m
}
pub fn create_troll() -> Monster {
    let mut m = create_giant_base("troll", "Troll", CreatureSize::Large, 5.0, 1800, 18, 13, 20, 7, 9, 7);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Regeneration", "Recupera 10 HP no início de seu turno a menos que receba fogo/ácido.");
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_giants_creation() {
        let storm = create_storm_giant();
        assert_eq!(storm.id, "storm_giant");
        assert_eq!(storm.challenge_rating, 13.0);
    }
}
