use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use crate::rules::spells::registry::SpellRegistry;
use crate::rules::spells::spell::{Components, Spell, SpellEffect, SpellSchool};

/// Registra todas as magias de Nível 4 do SRD
pub fn load_level4_spells(reg: &mut SpellRegistry) {
    // 1. Banishment
    reg.register_spell(Spell::new(
        "Banishment",
        4,
        SpellSchool::Abjuration,
        "1 action",
        60,
        Components::vsm("An item distasteful to the target"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Charisma,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility(
                    "Incapacitated and banished to another plane.".to_string(),
                ),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    // 2. Blight
    reg.register_spell(Spell::new(
        "Blight",
        4,
        SpellSchool::Necromancy,
        "1 action",
        30,
        Components::vs(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Constitution,
            half_damage_on_save: true,
            damage_dice: Some("8d8".to_string()),
            damage_type: Some(DamageType::Necrotic),
            damage_scaling: Some("1d8".to_string()),
            fail_effect: None,
        },
    ));

    // 3. Compulsion
    reg.register_spell(Spell::new(
        "Compulsion",
        4,
        SpellSchool::Enchantment,
        "1 action",
        30,
        Components::vs(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Wisdom,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility(
                    "Compelled to move in a designated direction on turn.".to_string(),
                ),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    // 4. Confusion
    reg.register_spell(Spell::new(
        "Confusion",
        4,
        SpellSchool::Enchantment,
        "1 action",
        90,
        Components::vsm("Three nut shells"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Wisdom,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility(
                    "Confused: Roll d10 each turn to determine random behavior.".to_string(),
                ),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    // 5. Conjure Minor Elementals
    reg.register_spell(Spell::new(
        "Conjure Minor Elementals",
        4,
        SpellSchool::Conjuration,
        "1 minute",
        90,
        Components::vs(),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Summon {
            creature_name: "Minor Elementals".to_string(),
            count: 4,
        },
    ));

    // 6. Conjure Woodland Beings
    reg.register_spell(Spell::new(
        "Conjure Woodland Beings",
        4,
        SpellSchool::Conjuration,
        "1 action",
        60,
        Components::vsm("One holly berry per creature summoned"),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Summon {
            creature_name: "Woodland Beings".to_string(),
            count: 4,
        },
    ));

    // 7. Control Water
    reg.register_spell(Spell::new(
        "Control Water",
        4,
        SpellSchool::Transmutation,
        "1 action",
        300,
        Components::vsm("A drop of water and a pinch of dust"),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description:
                "Manipulate water in a 100-ft cube: Flood, Redirect, Part Water, or Whirlpool."
                    .to_string(),
        },
    ));

    // 8. Death Ward
    reg.register_spell(Spell::new(
        "Death Ward",
        4,
        SpellSchool::Abjuration,
        "1 action",
        0, // Touch
        Components::vs(),
        "8 hours",
        false, // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility(
                "Death Ward: The first time target drops to 0 HP, it drops to 1 HP instead."
                    .to_string(),
            ),
            duration: DurationType::DurationHours(8),
        },
    ));

    // 9. Dimension Door
    reg.register_spell(Spell::new(
        "Dimension Door",
        4,
        SpellSchool::Conjuration,
        "1 action",
        500,
        Components::v(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Teleport yourself and optionally one willing creature up to 500 feet to a target location.".to_string(),
        },
    ));

    // 10. Divination
    reg.register_spell(Spell::new(
        "Divination",
        4,
        SpellSchool::Divination,
        "1 action",
        0, // Self
        Components::vsm("Incense and a sacrificial offering worth at least 25 gp"),
        "Instantaneous",
        false, // Concentration
        true,  // Ritual
        SpellEffect::Utility {
            description: "Ask a question concerning a specific goal, event, or activity to occur within 7 days.".to_string(),
        },
    ));

    // 11. Dominate Beast
    reg.register_spell(Spell::new(
        "Dominate Beast",
        4,
        SpellSchool::Enchantment,
        "1 action",
        60,
        Components::vs(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Wisdom,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility("Charmed and telepathically controlled.".to_string()),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    // 12. Evard's Black Tentacles
    reg.register_spell(Spell::new(
        "Evard's Black Tentacles",
        4,
        SpellSchool::Conjuration,
        "1 action",
        90,
        Components::vsm("A piece of tentacle from a giant octopus or giant squid"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: false,
            damage_dice: Some("3d6".to_string()),
            damage_type: Some(DamageType::Bludgeoning),
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility("Restrained by tentacles.".to_string()),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    // 13. Fabricate
    reg.register_spell(Spell::new(
        "Fabricate",
        4,
        SpellSchool::Transmutation,
        "10 minutes",
        120,
        Components::vs(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Convert raw materials into products of the same material (e.g. wood into a bridge).".to_string(),
        },
    ));

    // 14. Faithful Hound
    reg.register_spell(Spell::new(
        "Faithful Hound",
        4,
        SpellSchool::Conjuration,
        "1 action",
        30,
        Components::vsm("A tiny silver whistle, a piece of bone, and a thread"),
        "8 hours",
        false, // Concentration
        false, // Ritual
        SpellEffect::Summon {
            creature_name: "Phantom Faithful Hound".to_string(),
            count: 1,
        },
    ));

    // 15. Giant Insect
    reg.register_spell(Spell::new(
        "Giant Insect",
        4,
        SpellSchool::Transmutation,
        "1 action",
        30,
        Components::vs(),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Summon {
            creature_name: "Giant Insects".to_string(),
            count: 5,
        },
    ));

    // 16. Greater Invisibility
    reg.register_spell(Spell::new(
        "Greater Invisibility",
        4,
        SpellSchool::Illusion,
        "1 action",
        0, // Touch
        Components::vs(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility(
                "Invisible (does not break on attack or casting).".to_string(),
            ),
            duration: DurationType::DurationMinutes(1),
        },
    ));

    // 17. Hallucinatory Terrain
    reg.register_spell(Spell::new(
        "Hallucinatory Terrain",
        4,
        SpellSchool::Illusion,
        "10 minutes",
        300,
        Components::vsm("A stone, a twig, and a bit of green plant"),
        "24 hours",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description:
                "Make natural terrain look, sound, and smell like another type of natural terrain."
                    .to_string(),
        },
    ));

    // 18. Ice Storm
    reg.register_spell(Spell::new(
        "Ice Storm",
        4,
        SpellSchool::Evocation,
        "1 action",
        300,
        Components::vsm("A pinch of dust and a few drops of water"),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: true,
            damage_dice: Some("2d8+4d6".to_string()),
            damage_type: Some(DamageType::Cold),
            damage_scaling: Some("1d8".to_string()),
            fail_effect: None,
        },
    ));

    // 19. Locate Creature
    reg.register_spell(Spell::new(
        "Locate Creature",
        4,
        SpellSchool::Divination,
        "1 action",
        0, // Self
        Components::vsm("A bit of fur from a bloodhound"),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Senses the direction to a known or described creature within 1,000 feet."
                .to_string(),
        },
    ));

    // 20. Phantasmal Killer
    reg.register_spell(Spell::new(
        "Phantasmal Killer",
        4,
        SpellSchool::Illusion,
        "1 action",
        120,
        Components::vs(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Wisdom,
            half_damage_on_save: false,
            damage_dice: Some("4d10".to_string()),
            damage_type: Some(DamageType::Psychic),
            damage_scaling: Some("1d10".to_string()),
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility("Frightened by nightmare vision.".to_string()),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    // 21. Polymorph
    reg.register_spell(Spell::new(
        "Polymorph",
        4,
        SpellSchool::Transmutation,
        "1 action",
        60,
        Components::vsm("A caterpillar cocoon"),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Wisdom,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility(
                    "Transformed into a beast form with new stat block.".to_string(),
                ),
                duration: DurationType::DurationHours(1),
            })),
        },
    ));

    // 22. Resilient Sphere
    reg.register_spell(Spell::new(
        "Resilient Sphere",
        4,
        SpellSchool::Evocation,
        "1 action",
        30,
        Components::vsm("A hemispherical piece of clear crystal and a matching piece of amber"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility(
                    "Enclosed in an immune sphere blocking all physical/spell attacks.".to_string(),
                ),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    // 23. Secret Chest
    reg.register_spell(Spell::new(
        "Secret Chest",
        4,
        SpellSchool::Conjuration,
        "1 action",
        0, // Touch
        Components::vsm("An exquisite chest worth 5,000 gp and a tiny replica worth 50 gp"),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Hide a chest and its contents in the Ethereal Plane, recallable at will."
                .to_string(),
        },
    ));

    // 24. Stoneskin
    reg.register_spell(Spell::new(
        "Stoneskin",
        4,
        SpellSchool::Abjuration,
        "1 action",
        0, // Touch
        Components::vsm("Diamond dust worth at least 100 gp, consumed by the spell"),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility(
                "Target gains resistance to nonmagical bludgeoning, piercing, and slashing damage."
                    .to_string(),
            ),
            duration: DurationType::DurationHours(1),
        },
    ));

    // 25. Wall of Fire
    reg.register_spell(Spell::new(
        "Wall of Fire",
        4,
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vsm("A small piece of phosphorus"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: true,
            damage_dice: Some("5d8".to_string()),
            damage_type: Some(DamageType::Fire),
            damage_scaling: Some("1d8".to_string()),
            fail_effect: None,
        },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banishment_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Banishment").expect("Banishment not found");
        assert_eq!(spell.level, 4);
        assert_eq!(spell.school, SpellSchool::Abjuration);
        assert!(spell.requires_concentration);
    }

    #[test]
    fn test_blight_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Blight").expect("Blight not found");
        assert_eq!(spell.level, 4);
        assert_eq!(spell.school, SpellSchool::Necromancy);
    }

    #[test]
    fn test_compulsion_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Compulsion").expect("Compulsion not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_confusion_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Confusion").expect("Confusion not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_conjure_minor_elementals_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Conjure Minor Elementals")
            .expect("Conjure Minor Elementals not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_conjure_woodland_beings_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Conjure Woodland Beings")
            .expect("Conjure Woodland Beings not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_control_water_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Control Water").expect("Control Water not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_death_ward_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Death Ward").expect("Death Ward not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_dimension_door_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Dimension Door").expect("Dimension Door not found");
        assert_eq!(spell.level, 4);
        assert_eq!(spell.range_feet, 500);
    }

    #[test]
    fn test_divination_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Divination").expect("Divination not found");
        assert_eq!(spell.level, 4);
        assert!(spell.is_ritual);
    }

    #[test]
    fn test_dominate_beast_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Dominate Beast").expect("Dominate Beast not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_evards_black_tentacles_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Evard's Black Tentacles")
            .expect("Evard's Black Tentacles not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_fabricate_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Fabricate").expect("Fabricate not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_faithful_hound_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Faithful Hound").expect("Faithful Hound not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_giant_insect_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Giant Insect").expect("Giant Insect not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_greater_invisibility_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Greater Invisibility")
            .expect("Greater Invisibility not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_hallucinatory_terrain_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Hallucinatory Terrain")
            .expect("Hallucinatory Terrain not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_ice_storm_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Ice Storm").expect("Ice Storm not found");
        assert_eq!(spell.level, 4);
        assert_eq!(spell.school, SpellSchool::Evocation);
    }

    #[test]
    fn test_locate_creature_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Locate Creature").expect("Locate Creature not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_phantasmal_killer_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("Phantasmal Killer").expect("Phantasmal Killer not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_polymorph_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Polymorph").expect("Polymorph not found");
        assert_eq!(spell.level, 4);
        assert_eq!(spell.school, SpellSchool::Transmutation);
    }

    #[test]
    fn test_resilient_sphere_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("Resilient Sphere").expect("Resilient Sphere not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_secret_chest_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Secret Chest").expect("Secret Chest not found");
        assert_eq!(spell.level, 4);
    }

    #[test]
    fn test_stoneskin_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Stoneskin").expect("Stoneskin not found");
        assert_eq!(spell.level, 4);
        assert_eq!(spell.school, SpellSchool::Abjuration);
    }

    #[test]
    fn test_wall_of_fire_spell() {
        let mut reg = SpellRegistry::new();
        load_level4_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Wall of Fire").expect("Wall of Fire not found");
        assert_eq!(spell.level, 4);
        assert_eq!(spell.school, SpellSchool::Evocation);
    }
}
