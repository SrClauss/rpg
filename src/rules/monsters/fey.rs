use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::create_dagger;
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, InnateSpellcasting, Monster, MonsterAction, MonsterSense, UsageLimit,
};

fn create_fey_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    )
}

pub fn create_dryad() -> Monster {
    let mut m = Monster::new("dryad", create_fey_entity("Dryad", 10, 12, 11, 14, 15, 18), CreatureType::Fey, CreatureSize::Medium, 1.0, 200);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Tree Stride", "Teleporta entre árvores vivas gastando 10ft de movimento.");
    m.add_action(MonsterAction::attack("Club", create_dagger()));
    m.add_action(MonsterAction::new("Fey Charm", ActionType::Action, "Encanta humanoide a 30ft (CD 14 WIS save).", None));
    m
}

pub fn create_green_hag() -> Monster {
    let mut m = Monster::new("green_hag", create_fey_entity("Green Hag", 18, 12, 16, 13, 14, 14), CreatureType::Fey, CreatureSize::Medium, 3.0, 700);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Amphibious", "Respira ar e água.");
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m.add_action(MonsterAction::new("Illusory Appearance", ActionType::Action, "Altera a aparência fisicamente para parecr humanoide.", None));
    m.add_action(MonsterAction::new("Invisible Passage", ActionType::BonusAction, "Torna-se invisível deixando uma ilusão no lugar.", None));
    m
}

pub fn create_pixie() -> Monster {
    let mut m = Monster::new("pixie", create_fey_entity("Pixie", 2, 20, 8, 10, 14, 15), CreatureType::Fey, CreatureSize::Tiny, 0.25, 50);
    m.add_trait("Magic Resistance", "Vantagem em salvaguardas contra magias.");
    m.add_trait("Superior Invisibility", "Fica invisível como ação bônus sem exigir concentração.");
    m.innate_spellcasting = Some(InnateSpellcasting {
        spellcasting_ability: "Charisma".to_string(),
        dc: 12,
        spell_attack_bonus: 4,
        is_psionics: false,
        spells: vec![
            ("At Will: Druidcraft".to_string(), UsageLimit::AtWill),
            ("1/day each: Confusion, Dancing Lights, Detect Evil/Good, Entangle, Fly, Phantasmal Force, Polymorph, Sleep".to_string(), UsageLimit::PerDay(1)),
        ],
    });
    m
}

pub fn create_satyr() -> Monster {
    let mut m = Monster::new("satyr", create_fey_entity("Satyr", 12, 16, 11, 12, 10, 14), CreatureType::Fey, CreatureSize::Medium, 0.5, 100);
    m.add_trait("Magic Resistance", "Vantagem em salvaguardas contra magias.");
    m.add_action(MonsterAction::attack("Ram", create_dagger()));
    m.add_action(MonsterAction::attack("Shortsword", create_dagger()));
    m.add_action(MonsterAction::new("Panpipes", ActionType::Action, "Toca flauta mística invocando fascínio, pavor ou sono (CD 13 WIS).", None));
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fey_creation() {
        let pixie = create_pixie();
        assert_eq!(pixie.id, "pixie");
        assert!(pixie.innate_spellcasting.is_some());
    }
}
