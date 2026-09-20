use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::{create_dagger, DamageType};
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, Monster, MonsterAction, MonsterSense,
};

fn create_elemental_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    )
}

pub fn create_air_elemental() -> Monster {
    let mut m = Monster::new("air_elemental", create_elemental_entity("Air Elemental", 14, 20, 14, 6, 10, 6), CreatureType::Elemental, CreatureSize::Large, 5.0, 1800);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.resistances.damage_resistances.push(DamageType::Lightning);
    m.add_trait("Air Form", "Pode passar por espaço de até 1 polegada sem se espremer.");
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m.add_action(MonsterAction::new("Whirlwind", ActionType::Action, "Tormenta de vento arrasta e causa dano em área (Recharge 4-6).", None));
    m
}

pub fn create_earth_elemental() -> Monster {
    let mut m = Monster::new("earth_elemental", create_elemental_entity("Earth Elemental", 20, 8, 20, 5, 10, 5), CreatureType::Elemental, CreatureSize::Large, 5.0, 1800);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_sense(MonsterSense::Tremorsense { range_feet: 60 });
    m.add_trait("Earth Glide", "Pode se mover através de terra/pedra não-mágicas.");
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m
}

pub fn create_fire_elemental() -> Monster {
    let mut m = Monster::new("fire_elemental", create_elemental_entity("Fire Elemental", 10, 17, 16, 6, 10, 7), CreatureType::Elemental, CreatureSize::Large, 5.0, 1800);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.resistances.damage_immunities.push(DamageType::Fire);
    m.add_trait("Fire Form", "Incendeia criaturas ao entrar no mesmo espaço.");
    m.add_action(MonsterAction::attack("Touch", create_dagger()));
    m
}

pub fn create_water_elemental() -> Monster {
    let mut m = Monster::new("water_elemental", create_elemental_entity("Water Elemental", 18, 14, 18, 5, 10, 5), CreatureType::Elemental, CreatureSize::Large, 5.0, 1800);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m.add_action(MonsterAction::new("Whelm", ActionType::Action, "Afoga e prende criaturas em seu corpo líquido (Recharge 4-6).", None));
    m
}

pub fn create_azer() -> Monster {
    let mut m = Monster::new("azer", create_elemental_entity("Azer", 17, 12, 15, 12, 13, 10), CreatureType::Elemental, CreatureSize::Medium, 2.0, 450);
    m.resistances.damage_immunities.push(DamageType::Fire);
    m.add_trait("Heated Body", "Causa dano de fogo a quem o tocar em corpo a corpo.");
    m.add_action(MonsterAction::attack("Warhammer", create_dagger()));
    m
}

pub fn create_djinni() -> Monster {
    let mut m = Monster::new("djinni", create_elemental_entity("Djinni", 21, 15, 22, 15, 16, 20), CreatureType::Elemental, CreatureSize::Large, 11.0, 7200);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.resistances.damage_immunities.push(DamageType::Lightning);
    m.add_action(MonsterAction::attack("Scimitar", create_dagger()));
    m.add_action(MonsterAction::new("Create Whirlwind", ActionType::Action, "Cria um furacão de ar sustentado.", None));
    m
}

pub fn create_efreeti() -> Monster {
    let mut m = Monster::new("efreeti", create_elemental_entity("Efreeti", 22, 12, 24, 16, 15, 16), CreatureType::Elemental, CreatureSize::Large, 11.0, 7200);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.resistances.damage_immunities.push(DamageType::Fire);
    m.add_action(MonsterAction::attack("Scimitar", create_dagger()));
    m.add_action(MonsterAction::new("Hurl Flame", ActionType::Action, "Arremessa bola de fogo a 120ft.", None));
    m
}

pub fn create_gargoyle() -> Monster {
    let mut m = Monster::new("gargoyle", create_elemental_entity("Gargoyle", 15, 11, 16, 6, 11, 7), CreatureType::Elemental, CreatureSize::Medium, 2.0, 450);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.resistances.nonmagical_physical_resistance = true;
    m.add_trait("False Appearance", "Indistingüível de estátua de pedra.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m
}

pub fn create_invisible_stalker() -> Monster {
    let mut m = Monster::new("invisible_stalker", create_elemental_entity("Invisible Stalker", 16, 19, 14, 10, 15, 11), CreatureType::Elemental, CreatureSize::Medium, 6.0, 2300);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Invisibility", "Permanentemente invisível.");
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m
}

pub fn create_magmin() -> Monster {
    let mut m = Monster::new("magmin", create_elemental_entity("Magmin", 7, 15, 12, 8, 10, 10), CreatureType::Elemental, CreatureSize::Small, 0.5, 100);
    m.resistances.damage_immunities.push(DamageType::Fire);
    m.add_action(MonsterAction::attack("Touch", create_dagger()));
    m
}

pub fn create_mephits() -> Monster {
    let mut m = Monster::new("steam_mephit", create_elemental_entity("Steam Mephit", 5, 12, 10, 11, 10, 12), CreatureType::Elemental, CreatureSize::Small, 0.25, 50);
    m.resistances.damage_immunities.push(DamageType::Fire);
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m.add_action(MonsterAction::new("Steam Breath", ActionType::Action, "Sopro de vapor fervente.", None));
    m
}

pub fn create_salamander() -> Monster {
    let mut m = Monster::new("salamander", create_elemental_entity("Salamander", 18, 14, 15, 11, 10, 12), CreatureType::Elemental, CreatureSize::Large, 5.0, 1800);
    m.resistances.damage_immunities.push(DamageType::Fire);
    m.add_action(MonsterAction::attack("Spear", create_dagger()));
    m.add_action(MonsterAction::attack("Tail", create_dagger()));
    m
}

pub fn create_xorn() -> Monster {
    let mut m = Monster::new("xorn", create_elemental_entity("Xorn", 17, 10, 16, 11, 10, 11), CreatureType::Elemental, CreatureSize::Medium, 5.0, 1800);
    m.add_sense(MonsterSense::Tremorsense { range_feet: 60 });
    m.add_trait("Treasure Sense", "Sente a presença e localização de metais preciosos.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Claw", create_dagger()));
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elementals_creation() {
        let air = create_air_elemental();
        assert_eq!(air.id, "air_elemental");
        let efreeti = create_efreeti();
        assert_eq!(efreeti.challenge_rating, 11.0);
    }
}
