use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use crate::rules::spells::registry::SpellRegistry;
use crate::rules::spells::spell::{Components, Spell, SpellEffect, SpellSchool};

/// Registra todas as magias de Nível 6 do SRD
pub fn load_level6_spells(reg: &mut SpellRegistry) {
    // 1. Blade Barrier
    reg.register_spell(Spell::new(
        "Blade Barrier",
        6,
        SpellSchool::Evocation,
        "1 action",
        90,
        Components::vs(),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Creates a wall of whirling, razor-sharp blades.".to_string() },
    ));

    // 2. Chain Lightning
    reg.register_spell(Spell::new(
        "Chain Lightning",
        6,
        SpellSchool::Evocation,
        "1 action",
        150,
        Components::vsm("A bit of fur; a piece of amber, glass, or a crystal rod; and three silver pins"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Dexterity, half_damage_on_save: true, damage_dice: Some("10d8".to_string()), damage_type: Some(DamageType::Lightning), damage_scaling: Some("1d8".to_string()), fail_effect: None },
    ));

    // 3. Circle of Death
    reg.register_spell(Spell::new(
        "Circle of Death",
        6,
        SpellSchool::Necromancy,
        "1 action",
        150,
        Components::vsm("The powder of a crushed black pearl worth at least 500 gp"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Constitution, half_damage_on_save: true, damage_dice: Some("8d6".to_string()), damage_type: Some(DamageType::Necrotic), damage_scaling: Some("2d6".to_string()), fail_effect: None },
    ));

    // 4. Conjure Fey
    reg.register_spell(Spell::new(
        "Conjure Fey",
        6,
        SpellSchool::Conjuration,
        "1 minute",
        90,
        Components::vs(),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Summon { creature_name: "Fey".to_string(), count: 1 },
    ));

    // 5. Contingency
    reg.register_spell(Spell::new(
        "Contingency",
        6,
        SpellSchool::Evocation,
        "10 minutes",
        0,
        Components::vsm("A statuette of yourself carved from ivory and decorated with gems worth at least 1,500 gp"),
        "10 days",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Set a spell of 5th level or lower to take effect under a specific trigger condition.".to_string() },
    ));

    // 6. Create Undead
    reg.register_spell(Spell::new(
        "Create Undead",
        6,
        SpellSchool::Necromancy,
        "1 minute",
        10,
        Components::vsm("One clay pot filled with grave dirt, one clay pot filled with brackish water, and one 150 gp black onyx stone for each corpse"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Summon { creature_name: "Ghoul".to_string(), count: 3 },
    ));

    // 7. Disintegrate
    reg.register_spell(Spell::new(
        "Disintegrate",
        6,
        SpellSchool::Transmutation,
        "1 action",
        60,
        Components::vsm("A lodestone and a pinch of dust"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Dexterity, half_damage_on_save: false, damage_dice: Some("10d6+40".to_string()), damage_type: Some(DamageType::Force), damage_scaling: Some("3d6".to_string()), fail_effect: None },
    ));

    // 8. Eyebite
    reg.register_spell(Spell::new(
        "Eyebite",
        6,
        SpellSchool::Necromancy,
        "1 action",
        0,
        Components::vs(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Inflict dread, sickness, or sleep on a creature you can see.".to_string() },
    ));

    // 9. Find the Path
    reg.register_spell(Spell::new(
        "Find the Path",
        6,
        SpellSchool::Divination,
        "1 minute",
        0,
        Components::vsm("A set of divinatory tools—such as bones, ivory sticks, cards, teeth, or carved runes—worth 100 gp and an object from the location you wish to find"),
        "1 day",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Find the most direct physical route to a specific fixed location.".to_string() },
    ));

    // 10. Forbiddance
    reg.register_spell(Spell::new(
        "Forbiddance",
        6,
        SpellSchool::Abjuration,
        "10 minutes",
        0,
        Components::vsm("A sprinkling of holy water, rare incense, and ruby dust worth at least 1,000 gp"),
        "1 day",
        false,  // Concentration
        true, // Ritual
        SpellEffect::Utility { description: "Protect up to 40,000 square feet against magical travel and planar creatures.".to_string() },
    ));

    // 11. Globe of Invulnerability
    reg.register_spell(Spell::new(
        "Globe of Invulnerability",
        6,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vsm("A glass or crystal bead that shatters when the spell ends"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "An immobile, faintly shimmering barrier springs into existence in a 10-foot radius around you and remains for the duration.".to_string() },
    ));

    // 12. Guards and Wards
    reg.register_spell(Spell::new(
        "Guards and Wards",
        6,
        SpellSchool::Abjuration,
        "10 minutes",
        0,
        Components::vsm("Burning incense, a small measure of brimstone and oil, a knotted string, a large amount of umber hulk blood, and a small silver rod worth at least 10 gp"),
        "24 hours",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Create a ward that protects up to 2,500 square feet of floor space.".to_string() },
    ));

    // 13. Harm
    reg.register_spell(Spell::new(
        "Harm",
        6,
        SpellSchool::Necromancy,
        "1 action",
        60,
        Components::vs(),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Constitution, half_damage_on_save: true, damage_dice: Some("14d6".to_string()), damage_type: Some(DamageType::Necrotic), damage_scaling: None, fail_effect: None },
    ));

    // 14. Heal
    reg.register_spell(Spell::new(
        "Heal",
        6,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::vs(),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Healing { healing_dice: "70".to_string() },
    ));

    // 15. Heroes' Feast
    reg.register_spell(Spell::new(
        "Heroes' Feast",
        6,
        SpellSchool::Conjuration,
        "10 minutes",
        30,
        Components::vsm("A gem-encrusted bowl worth at least 1,000 gp, which the spell consumes"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Bring forth a great feast, including magnificent food and drink.".to_string() },
    ));

    // 16. Instant Summons
    reg.register_spell(Spell::new(
        "Instant Summons",
        6,
        SpellSchool::Conjuration,
        "1 minute",
        0,
        Components::vsm("A sapphire worth 1,000 gp"),
        "Until dispelled",
        false,  // Concentration
        true, // Ritual
        SpellEffect::Utility { description: "Touch an object weighing 10 pounds or less whose longest dimension is 6 feet or less, making an invisible mark on it.".to_string() },
    ));

    // 17. Irresistible Dance
    reg.register_spell(Spell::new(
        "Irresistible Dance",
        6,
        SpellSchool::Enchantment,
        "1 action",
        30,
        Components::v(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Force a creature to dance, giving it disadvantage on attacks and Dex saves, and granting advantage to attackers.".to_string() },
    ));

    // 18. Move Earth
    reg.register_spell(Spell::new(
        "Move Earth",
        6,
        SpellSchool::Transmutation,
        "1 action",
        120,
        Components::vsm("An iron blade and a small bag containing a mixture of soils—clay, loam, and sand"),
        "2 hours",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Reshape dirt, sand, or clay in the area.".to_string() },
    ));

    // 19. Planar Ally
    reg.register_spell(Spell::new(
        "Planar Ally",
        6,
        SpellSchool::Conjuration,
        "10 minutes",
        60,
        Components::vs(),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Beseech an otherworldly entity for aid.".to_string() },
    ));

    // 20. Programmed Illusion
    reg.register_spell(Spell::new(
        "Programmed Illusion",
        6,
        SpellSchool::Illusion,
        "1 action",
        120,
        Components::vsm("A bit of fleece and jade dust worth at least 25 gp"),
        "Until dispelled",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Create an illusion of an object, a creature, or some other visible phenomenon within range that activates when a specific condition occurs.".to_string() },
    ));

    // 21. Sunbeam
    reg.register_spell(Spell::new(
        "Sunbeam",
        6,
        SpellSchool::Evocation,
        "1 action",
        0,
        Components::vsm("A magnifying glass"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Constitution, half_damage_on_save: true, damage_dice: Some("6d8".to_string()), damage_type: Some(DamageType::Radiant), damage_scaling: None, fail_effect: Some(Box::new(SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Blinded".to_string()), duration: DurationType::UntilEndOfNextTurn })) },
    ));

    // 22. Transport via Plants
    reg.register_spell(Spell::new(
        "Transport via Plants",
        6,
        SpellSchool::Conjuration,
        "1 action",
        10,
        Components::vs(),
        "1 round",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Create a magical link between a Large or larger inanimate plant within range and another plant, at any distance, on the same plane of existence.".to_string() },
    ));

    // 23. True Seeing
    reg.register_spell(Spell::new(
        "True Seeing",
        6,
        SpellSchool::Divination,
        "1 action",
        0,
        Components::vsm("An ointment for the eyes that costs 25 gp; is made from mushroom powder, saffron, and fat; and is consumed by the spell"),
        "1 hour",
        false,  // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("True Sight".to_string()), duration: DurationType::DurationHours(1) },
    ));

    // 24. Wall of Ice
    reg.register_spell(Spell::new(
        "Wall of Ice",
        6,
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vsm("A small piece of quartz"),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Create a wall of ice on a solid surface within range.".to_string() },
    ));

    // 25. Wall of Thorns
    reg.register_spell(Spell::new(
        "Wall of Thorns",
        6,
        SpellSchool::Conjuration,
        "1 action",
        120,
        Components::vs(),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Create a wall of tough, pliable, tangled brush bristling with needle-sharp thorns.".to_string() },
    ));

    // 26. Wind Walk
    reg.register_spell(Spell::new(
        "Wind Walk",
        6,
        SpellSchool::Transmutation,
        "1 minute",
        30,
        Components::vsm("Fire and holy water"),
        "8 hours",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "You and up to ten willing creatures you can see assume a gaseous form for the duration, appearing as wisps of cloud.".to_string() },
    ));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blade_barrier_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Blade Barrier").expect("Blade Barrier not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_chain_lightning_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Chain Lightning").expect("Chain Lightning not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_circle_of_death_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Circle of Death").expect("Circle of Death not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_conjure_fey_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Conjure Fey").expect("Conjure Fey not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_contingency_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Contingency").expect("Contingency not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_create_undead_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Create Undead").expect("Create Undead not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_disintegrate_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Disintegrate").expect("Disintegrate not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_eyebite_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Eyebite").expect("Eyebite not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_find_the_path_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Find the Path").expect("Find the Path not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_forbiddance_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Forbiddance").expect("Forbiddance not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_globe_of_invulnerability_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Globe of Invulnerability").expect("Globe of Invulnerability not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_guards_and_wards_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Guards and Wards").expect("Guards and Wards not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_harm_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Harm").expect("Harm not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_heal_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Heal").expect("Heal not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_heroes_feast_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Heroes' Feast").expect("Heroes' Feast not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_instant_summons_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Instant Summons").expect("Instant Summons not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_irresistible_dance_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Irresistible Dance").expect("Irresistible Dance not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_move_earth_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Move Earth").expect("Move Earth not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_planar_ally_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Planar Ally").expect("Planar Ally not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_programmed_illusion_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Programmed Illusion").expect("Programmed Illusion not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_sunbeam_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Sunbeam").expect("Sunbeam not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_transport_via_plants_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Transport via Plants").expect("Transport via Plants not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_true_seeing_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("True Seeing").expect("True Seeing not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_wall_of_ice_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Wall of Ice").expect("Wall of Ice not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_wall_of_thorns_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Wall of Thorns").expect("Wall of Thorns not found");
        assert_eq!(spell.level, 6);
    }
    #[test]
    fn test_wind_walk_spell() {
        let mut reg = SpellRegistry::new();
        load_level6_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Wind Walk").expect("Wind Walk not found");
        assert_eq!(spell.level, 6);
    }
}
