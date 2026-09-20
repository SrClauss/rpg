use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::{create_dagger, DamageType};
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, Monster, MonsterAction, MonsterSense,
};

fn create_ooze_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    )
}

pub fn create_black_pudding() -> Monster {
    let mut m = Monster::new("black_pudding", create_ooze_entity("Black Pudding", 16, 5, 16, 1, 6, 1), CreatureType::Ooze, CreatureSize::Large, 4.0, 1100);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: true });
    m.resistances.damage_immunities.push(DamageType::Acid);
    m.resistances.damage_immunities.push(DamageType::Lightning);
    m.resistances.damage_immunities.push(DamageType::Slashing);
    m.add_trait("Amorphous", "Pode passar por espaço de 1 polegada sem se espremer.");
    m.add_trait("Corrosive Form", "Corrói armas de metal ou madeira que o atingirem.");
    m.add_trait("Split", "Reação ao receber dano cortante ou elétrico: divide-se em duas novas gosmas se tiver >= 10 HP.");
    m.add_action(MonsterAction::attack("Pseudopod", create_dagger()));
    m
}

pub fn create_gelatinous_cube() -> Monster {
    let mut m = Monster::new("gelatinous_cube", create_ooze_entity("Gelatinous Cube", 14, 6, 20, 1, 6, 1), CreatureType::Ooze, CreatureSize::Large, 2.0, 450);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: true });
    m.add_trait("Transparent", "Teste de Percepção CD 15 para notá-lo se não tiver se movido.");
    m.add_action(MonsterAction::attack("Pseudopod", create_dagger()));
    m.add_action(MonsterAction::new("Engulf", ActionType::Action, "Engole criaturas que falharem no teste de salvaguarda de DEX.", None));
    m
}

pub fn create_gray_ooze() -> Monster {
    let mut m = Monster::new("gray_ooze", create_ooze_entity("Gray Ooze", 12, 6, 16, 1, 6, 2), CreatureType::Ooze, CreatureSize::Medium, 0.5, 100);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: true });
    m.resistances.damage_resistances.push(DamageType::Acid);
    m.resistances.damage_resistances.push(DamageType::Cold);
    m.resistances.damage_resistances.push(DamageType::Fire);
    m.add_trait("Corrosive Form", "Corrói armaduras de metal.");
    m.add_action(MonsterAction::attack("Pseudopod", create_dagger()));
    m
}

pub fn create_ochre_jelly() -> Monster {
    let mut m = Monster::new("ochre_jelly", create_ooze_entity("Ochre Jelly", 15, 6, 14, 2, 6, 1), CreatureType::Ooze, CreatureSize::Large, 2.0, 450);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: true });
    m.resistances.damage_immunities.push(DamageType::Lightning);
    m.resistances.damage_immunities.push(DamageType::Slashing);
    m.add_trait("Amorphous", "Passa por 1 polegada.");
    m.add_trait("Split", "Divide-se ao receber dano de slashing ou lightning.");
    m.add_action(MonsterAction::attack("Pseudopod", create_dagger()));
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oozes_creation() {
        let cube = create_gelatinous_cube();
        assert_eq!(cube.id, "gelatinous_cube");
        let pudding = create_black_pudding();
        assert_eq!(pudding.challenge_rating, 4.0);
    }
}
