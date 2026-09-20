use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::{create_dagger, DamageType};
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, Monster, MonsterAction, MonsterSense,
};

fn create_plant_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    )
}

pub fn create_awakened_shrub() -> Monster {
    let mut m = Monster::new("awakened_shrub", create_plant_entity("Awakened Shrub", 3, 8, 11, 10, 10, 6), CreatureType::Plant, CreatureSize::Small, 0.0, 10);
    m.resistances.damage_vulnerabilities.push(DamageType::Fire);
    m.add_trait("False Appearance", "Indistingüível de um arbusto normal enquanto imóvel.");
    m.add_action(MonsterAction::attack("Rake", create_dagger()));
    m
}

pub fn create_awakened_tree() -> Monster {
    let mut m = Monster::new("awakened_tree", create_plant_entity("Awakened Tree", 19, 6, 15, 10, 10, 7), CreatureType::Plant, CreatureSize::Huge, 2.0, 450);
    m.resistances.damage_vulnerabilities.push(DamageType::Fire);
    m.add_trait("False Appearance", "Indistingüível de uma árvore normal enquanto imóvel.");
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m
}

pub fn create_shambling_mound() -> Monster {
    let mut m = Monster::new("shambling_mound", create_plant_entity("Shambling Mound", 18, 8, 16, 5, 10, 5), CreatureType::Plant, CreatureSize::Large, 5.0, 1800);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: true });
    m.resistances.damage_immunities.push(DamageType::Lightning);
    m.resistances.damage_resistances.push(DamageType::Cold);
    m.resistances.damage_resistances.push(DamageType::Fire);
    m.add_trait("Lightning Absorption", "Dano elétrico cura a criatura em vez de machucar.");
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m.add_action(MonsterAction::attack("Engulf", create_dagger()));
    m
}

pub fn create_treant() -> Monster {
    let mut m = Monster::new("treant", create_plant_entity("Treant", 23, 8, 21, 12, 16, 12), CreatureType::Plant, CreatureSize::Huge, 9.0, 5000);
    m.resistances.damage_vulnerabilities.push(DamageType::Fire);
    m.resistances.nonmagical_physical_resistance = true;
    m.add_trait("False Appearance", "Indistingüível de uma árvore normal.");
    m.add_trait("Siege Monster", "Causa o dobro de dano a estruturas.");
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m.add_action(MonsterAction::attack("Rock", create_dagger()));
    m.add_action(MonsterAction::new("Animate Trees", ActionType::Action, "Anima até duas árvores num raio de 60ft (1/Day).", None));
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plants_creation() {
        let treant = create_treant();
        assert_eq!(treant.id, "treant");
        assert_eq!(treant.challenge_rating, 9.0);
    }
}
