use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::{create_dagger, DamageType};
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, InnateSpellcasting, Monster, MonsterAction, MonsterSense, UsageLimit,
};

fn create_celestial_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    )
}

/// 1. Deva (CR 10) - SRD 5.1 p. 261
pub fn create_deva() -> Monster {
    let mut m = Monster::new("deva", create_celestial_entity("Deva", 18, 18, 18, 17, 20, 20), CreatureType::Celestial, CreatureSize::Medium, 10.0, 5900);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.add_sense(MonsterSense::Truesight { range_feet: 120 });
    m.add_sense(MonsterSense::PassivePerception(19));
    m.resistances.damage_resistances.push(DamageType::Radiant);

    m.add_trait("Angel Weapons", "Ataques com armas causam 4d8 de dano radiante adicional.");
    m.add_trait("Magic Resistance", "Vantagem em testes de salvaguarda contra magias e efeitos mágicos.");

    m.innate_spellcasting = Some(InnateSpellcasting {
        spellcasting_ability: "Charisma".to_string(),
        dc: 17,
        spell_attack_bonus: 9,
        is_psionics: false,
        spells: vec![
            ("At Will: Detect Evil and Good".to_string(), UsageLimit::AtWill),
            ("1/day each: Commune, Raise Dead".to_string(), UsageLimit::PerDay(1)),
        ],
    });

    m.add_action(MonsterAction::attack("Mace", create_dagger()));
    m.add_action(MonsterAction::new("Healing Touch", ActionType::Action, "Toca uma criatura restaurando 4d8+2 de HP.", None).with_usage(UsageLimit::PerDay(3)));
    m.add_action(MonsterAction::new("Change Shape", ActionType::Action, "Metamorfoseia-se em um humanoide ou besta.", None));
    m
}

/// 2. Planetar (CR 16) - SRD 5.1 p. 262
pub fn create_planetar() -> Monster {
    let mut m = Monster::new("planetar", create_celestial_entity("Planetar", 24, 20, 24, 19, 22, 25), CreatureType::Celestial, CreatureSize::Large, 16.0, 15000);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.add_sense(MonsterSense::Truesight { range_feet: 120 });
    m.add_sense(MonsterSense::PassivePerception(21));
    m.resistances.damage_resistances.push(DamageType::Radiant);

    m.add_trait("Angel Weapons", "Ataques causam 5d8 de dano radiante extra.");
    m.add_trait("Magic Resistance", "Vantagem em testes de salvaguarda contra magias.");
    m.add_trait("Divine Awareness", "Sabe imediatamente se está ouvindo uma mentira.");

    m.add_action(MonsterAction::attack("Greatsword", create_dagger()));
    m.add_action(MonsterAction::new("Healing Touch", ActionType::Action, "Cura 6d8+3 HP e remove doenças/condições.", None).with_usage(UsageLimit::PerDay(4)));
    m
}

/// 3. Solar (CR 21) - SRD 5.1 p. 262
pub fn create_solar() -> Monster {
    let mut m = Monster::new("solar", create_celestial_entity("Solar", 26, 22, 26, 25, 25, 30), CreatureType::Celestial, CreatureSize::Large, 21.0, 33000);
    m.add_sense(MonsterSense::Truesight { range_feet: 120 });
    m.add_sense(MonsterSense::PassivePerception(24));
    m.resistances.damage_immunities.push(DamageType::Radiant);
    m.resistances.damage_immunities.push(DamageType::Necrotic);

    m.add_trait("Angel Weapons", "Ataques causam 6d8 de dano radiante extra.");
    m.add_trait("Magic Resistance", "Vantagem em testes de salvaguarda contra magias.");
    m.add_trait("Slaying Longbow", "Flexa mística: obriga o alvo a passar em CD 21 CON ou morrer se tiver <= 100 HP.");

    m.add_action(MonsterAction::attack("Greatsword", create_dagger()));
    m.add_action(MonsterAction::attack("Slaying Longbow", create_dagger()));
    m.set_legendary(3);
    m
}

/// 4. Couatl (CR 4) - SRD 5.1 p. 269
pub fn create_couatl() -> Monster {
    let mut m = Monster::new("couatl", create_celestial_entity("Couatl", 16, 20, 17, 18, 20, 18), CreatureType::Celestial, CreatureSize::Medium, 4.0, 1100);
    m.add_sense(MonsterSense::Truesight { range_feet: 120 });
    m.resistances.damage_immunities.push(DamageType::Psychic);

    m.add_trait("Shielded Mind", "Imune a telepatia e qualquer tentativa de ler pensamentos.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Constrict", create_dagger()));
    m.add_action(MonsterAction::new("Change Shape", ActionType::Action, "Metamorfoseia-se em humanoide ou besta.", None));
    m
}

/// 5. Pegasus (CR 2) - SRD 5.1 p. 339
pub fn create_pegasus() -> Monster {
    let mut m = Monster::new("pegasus", create_celestial_entity("Pegasus", 18, 15, 16, 10, 15, 13), CreatureType::Celestial, CreatureSize::Large, 2.0, 450);
    m.add_sense(MonsterSense::PassivePerception(16));
    m.add_action(MonsterAction::attack("Hooves", create_dagger()));
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celestials_creation() {
        let deva = create_deva();
        assert_eq!(deva.id, "deva");
        let solar = create_solar();
        assert_eq!(solar.challenge_rating, 21.0);
        assert!(solar.legendary.is_some());
    }
}
