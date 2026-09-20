use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::{create_dagger, DamageType};
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, Monster, MonsterAction, MonsterSense, UsageLimit,
};

fn create_construct_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    )
}

pub fn create_animated_armor() -> Monster {
    let mut m = Monster::new("animated_armor", create_construct_entity("Animated Armor", 14, 11, 13, 1, 3, 1), CreatureType::Construct, CreatureSize::Medium, 1.0, 200);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: true });
    m.resistances.damage_immunities.push(DamageType::Poison);
    m.add_trait("Antimagic Susceptibility", "Fica incapacitado em Antimagic Field.");
    m.add_trait("False Appearance", "Indistingüível de uma armadura normal enquanto imóvel.");
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m
}

pub fn create_flying_sword() -> Monster {
    let mut m = Monster::new("flying_sword", create_construct_entity("Flying Sword", 12, 15, 11, 1, 5, 1), CreatureType::Construct, CreatureSize::Small, 0.25, 50);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: true });
    m.resistances.damage_immunities.push(DamageType::Poison);
    m.add_trait("False Appearance", "Indistingüível de uma espada normal enquanto imóvel.");
    m.add_action(MonsterAction::attack("Longsword", create_dagger()));
    m
}

pub fn create_rug_of_smothering() -> Monster {
    let mut m = Monster::new("rug_of_smothering", create_construct_entity("Rug of Smothering", 17, 14, 10, 1, 3, 1), CreatureType::Construct, CreatureSize::Large, 2.0, 450);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: true });
    m.add_trait("Damage Transfer", "Transfere metade do dano recebido para a criatura sufocada por ele.");
    m.add_action(MonsterAction::attack("Smother", create_dagger()));
    m
}

pub fn create_clay_golem() -> Monster {
    let mut m = Monster::new("clay_golem", create_construct_entity("Clay Golem", 20, 9, 18, 3, 11, 1), CreatureType::Construct, CreatureSize::Large, 9.0, 5000);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.resistances.nonmagical_physical_immunity = true;
    m.add_trait("Acid Absorption", "Dano de Ácido cura o Golem.");
    m.add_trait("Berserk", "Pode entrar em fúria incontrolável ao sofrer dano se falhar no teste.");
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m.add_action(MonsterAction::new("Haste", ActionType::Action, "Ganha bônus de velocidade e ataque extra (Recharge 5-6).", None));
    m
}

pub fn create_flesh_golem() -> Monster {
    let mut m = Monster::new("flesh_golem", create_construct_entity("Flesh Golem", 19, 9, 18, 6, 10, 5), CreatureType::Construct, CreatureSize::Medium, 5.0, 1800);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.resistances.nonmagical_physical_immunity = true;
    m.add_trait("Lightning Absorption", "Dano elétrico cura o Golem.");
    m.add_trait("Aversion to Fire", "Se sofrer dano de fogo, tem desvantagem em ataques por 1 rodada.");
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m
}

pub fn create_iron_golem() -> Monster {
    let mut m = Monster::new("iron_golem", create_construct_entity("Iron Golem", 24, 9, 20, 3, 11, 1), CreatureType::Construct, CreatureSize::Large, 16.0, 15000);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.resistances.nonmagical_physical_immunity = true;
    m.add_trait("Fire Absorption", "Dano de Fogo cura o Golem.");
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m.add_action(MonsterAction::attack("Sword", create_dagger()));
    m.add_action(MonsterAction::new("Poison Breath", ActionType::Action, "Sopro de veneno em cone de 15ft (Recharge 6).", None));
    m
}

pub fn create_stone_golem() -> Monster {
    let mut m = Monster::new("stone_golem", create_construct_entity("Stone Golem", 22, 9, 20, 3, 11, 1), CreatureType::Construct, CreatureSize::Large, 10.0, 5900);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.resistances.nonmagical_physical_immunity = true;
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m.add_action(MonsterAction::new("Slow", ActionType::Action, "Aplica efeito de lentidão num raio de 10ft (Recharge 5-6).", None));
    m
}

pub fn create_homunculus() -> Monster {
    let mut m = Monster::new("homunculus", create_construct_entity("Homunculus", 4, 15, 11, 10, 10, 7), CreatureType::Construct, CreatureSize::Tiny, 0.0, 10);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Telepathic Bond", "Comunica-se telepaticamente com seu mestre a qualquer distância no mesmo plano.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_shield_guardian() -> Monster {
    let mut m = Monster::new("shield_guardian", create_construct_entity("Shield Guardian", 18, 8, 18, 7, 10, 3), CreatureType::Construct, CreatureSize::Large, 7.0, 2900);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Bound", "Vinculado a um amuleto mágica.");
    m.add_trait("Spell Storing", "Pode armazenar 1 magia de até 4º nível.");
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m.add_action(MonsterAction::new("Shield", ActionType::Reaction, "Adiciona +4 na CA do mestre quando atacado a 5ft.", None));
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructs_creation() {
        let iron_golem = create_iron_golem();
        assert_eq!(iron_golem.id, "iron_golem");
        assert_eq!(iron_golem.challenge_rating, 16.0);
    }
}
