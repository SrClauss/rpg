use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::{create_dagger, DamageType};
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, Monster, MonsterAction, MonsterSense,
};

fn create_dragon_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    )
}

fn create_dragon_base(id: &str, name: &str, size: CreatureSize, cr: f32, xp: u32, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8, damage_immunity: DamageType) -> Monster {
    let mut m = Monster::new(id, create_dragon_entity(name, str_, dex, con, int_, wis, cha), CreatureType::Dragon, size, cr, xp);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: false });
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.resistances.damage_immunities.push(damage_immunity);
    if cr >= 13.0 {
        m.add_trait("Legendary Resistance", "Se falhar numa salvaguarda, pode escolher passar em vez disso (3/Day).");
        m.set_legendary(3);
    }
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Claw", create_dagger()));
    m.add_action(MonsterAction::new("Breath Weapon", ActionType::Action, "Sopro elemental de dragão (Recharge 5-6).", None));
    m
}

// 1. Black Dragon
pub fn create_black_dragon_wyrmling() -> Monster { create_dragon_base("black_dragon_wyrmling", "Black Dragon Wyrmling", CreatureSize::Medium, 2.0, 450, 15, 14, 13, 10, 11, 13, DamageType::Acid) }
pub fn create_young_black_dragon() -> Monster { create_dragon_base("young_black_dragon", "Young Black Dragon", CreatureSize::Large, 7.0, 2900, 19, 14, 17, 12, 11, 15, DamageType::Acid) }
pub fn create_adult_black_dragon() -> Monster { create_dragon_base("adult_black_dragon", "Adult Black Dragon", CreatureSize::Huge, 14.0, 11500, 23, 14, 21, 14, 13, 17, DamageType::Acid) }
pub fn create_ancient_black_dragon() -> Monster { create_dragon_base("ancient_black_dragon", "Ancient Black Dragon", CreatureSize::Gargantuan, 21.0, 33000, 27, 14, 25, 16, 15, 19, DamageType::Acid) }

// 2. Blue Dragon
pub fn create_blue_dragon_wyrmling() -> Monster { create_dragon_base("blue_dragon_wyrmling", "Blue Dragon Wyrmling", CreatureSize::Medium, 3.0, 700, 17, 10, 15, 12, 11, 15, DamageType::Lightning) }
pub fn create_young_blue_dragon() -> Monster { create_dragon_base("young_blue_dragon", "Young Blue Dragon", CreatureSize::Large, 9.0, 5000, 21, 10, 19, 14, 13, 17, DamageType::Lightning) }
pub fn create_adult_blue_dragon() -> Monster { create_dragon_base("adult_blue_dragon", "Adult Blue Dragon", CreatureSize::Huge, 16.0, 15000, 25, 10, 23, 16, 15, 19, DamageType::Lightning) }
pub fn create_ancient_blue_dragon() -> Monster { create_dragon_base("ancient_blue_dragon", "Ancient Blue Dragon", CreatureSize::Gargantuan, 23.0, 50000, 29, 10, 27, 18, 17, 21, DamageType::Lightning) }

// 3. Red Dragon
pub fn create_red_dragon_wyrmling() -> Monster { create_dragon_base("red_dragon_wyrmling", "Red Dragon Wyrmling", CreatureSize::Medium, 4.0, 1100, 19, 10, 17, 12, 11, 15, DamageType::Fire) }
pub fn create_young_red_dragon() -> Monster { create_dragon_base("young_red_dragon", "Young Red Dragon", CreatureSize::Large, 10.0, 5900, 23, 10, 21, 14, 11, 19, DamageType::Fire) }
pub fn create_adult_red_dragon() -> Monster { create_dragon_base("adult_red_dragon", "Adult Red Dragon", CreatureSize::Huge, 17.0, 18000, 27, 10, 25, 16, 13, 21, DamageType::Fire) }
pub fn create_ancient_red_dragon() -> Monster { create_dragon_base("ancient_red_dragon", "Ancient Red Dragon", CreatureSize::Gargantuan, 24.0, 62000, 30, 10, 29, 18, 15, 23, DamageType::Fire) }

// 4. White Dragon
pub fn create_white_dragon_wyrmling() -> Monster { create_dragon_base("white_dragon_wyrmling", "White Dragon Wyrmling", CreatureSize::Medium, 2.0, 450, 14, 10, 14, 6, 10, 11, DamageType::Cold) }
pub fn create_young_white_dragon() -> Monster { create_dragon_base("young_white_dragon", "Young White Dragon", CreatureSize::Large, 6.0, 2300, 18, 10, 18, 6, 11, 12, DamageType::Cold) }
pub fn create_adult_white_dragon() -> Monster { create_dragon_base("adult_white_dragon", "Adult White Dragon", CreatureSize::Huge, 13.0, 10000, 22, 10, 22, 8, 12, 14, DamageType::Cold) }
pub fn create_ancient_white_dragon() -> Monster { create_dragon_base("ancient_white_dragon", "Ancient White Dragon", CreatureSize::Gargantuan, 20.0, 25000, 26, 10, 26, 10, 13, 16, DamageType::Cold) }

// 5. Green Dragon
pub fn create_green_dragon_wyrmling() -> Monster { create_dragon_base("green_dragon_wyrmling", "Green Dragon Wyrmling", CreatureSize::Medium, 2.0, 450, 15, 12, 13, 14, 11, 13, DamageType::Poison) }
pub fn create_young_green_dragon() -> Monster { create_dragon_base("young_green_dragon", "Young Green Dragon", CreatureSize::Large, 8.0, 3900, 19, 12, 17, 16, 13, 15, DamageType::Poison) }
pub fn create_adult_green_dragon() -> Monster { create_dragon_base("adult_green_dragon", "Adult Green Dragon", CreatureSize::Huge, 15.0, 13000, 23, 12, 21, 18, 15, 17, DamageType::Poison) }
pub fn create_ancient_green_dragon() -> Monster { create_dragon_base("ancient_green_dragon", "Ancient Green Dragon", CreatureSize::Gargantuan, 22.0, 41000, 27, 12, 25, 20, 17, 19, DamageType::Poison) }

// 6. Dragon Turtle
pub fn create_dragon_turtle() -> Monster {
    let mut m = Monster::new("dragon_turtle", create_dragon_entity("Dragon Turtle", 25, 10, 20, 10, 12, 12), CreatureType::Dragon, CreatureSize::Gargantuan, 17.0, 18000);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.resistances.damage_resistances.push(DamageType::Fire);
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Claw", create_dagger()));
    m.add_action(MonsterAction::new("Steam Breath", ActionType::Action, "Sopro de vapor fervoroso em cone de 60ft (Recharge 5-6).", None));
    m
}

// 7. Pseudodragon
pub fn create_pseudodragon() -> Monster {
    let mut m = Monster::new("pseudodragon", create_dragon_entity("Pseudodragon", 6, 15, 13, 10, 12, 10), CreatureType::Dragon, CreatureSize::Tiny, 0.25, 50);
    m.add_sense(MonsterSense::Blindsight { range_feet: 10, blind_beyond: false });
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Limited Telepathy", "Transmite emoções e imagens telepaticamente.");
    m.add_action(MonsterAction::attack("Stinger", create_dagger()));
    m
}

// 8. Wyvern
pub fn create_wyvern() -> Monster {
    let mut m = Monster::new("wyvern", create_dragon_entity("Wyvern", 19, 12, 16, 5, 12, 6), CreatureType::Dragon, CreatureSize::Large, 6.0, 2300);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Stinger", create_dagger()));
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dragons_creation() {
        let ancient_red = create_ancient_red_dragon();
        assert_eq!(ancient_red.id, "ancient_red_dragon");
        assert_eq!(ancient_red.challenge_rating, 24.0);
        assert!(ancient_red.legendary.is_some());
    }
}
