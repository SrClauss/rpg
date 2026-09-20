use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use crate::rules::spells::registry::SpellRegistry;
use crate::rules::spells::spell::{Components, Spell, SpellEffect, SpellSchool};

/// Registra todas as magias de Nível 5 do SRD
pub fn load_level5_spells(reg: &mut SpellRegistry) {
    // 1. Animate Objects
    reg.register_spell(Spell::new(
        "Animate Objects",
        5,
        SpellSchool::Transmutation,
        "1 action",
        120,
        Components::vs(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Animate up to 10 nonmagical objects.".to_string(),
        },
    ));

    // 2. Antilife Shell
    reg.register_spell(Spell::new(
        "Antilife Shell",
        5,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vs(),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "A 10-foot radius barrier prevents living creatures from passing or reaching through.".to_string() },
    ));

    // 3. Awaken
    reg.register_spell(Spell::new(
        "Awaken",
        5,
        SpellSchool::Transmutation,
        "8 hours",
        0,
        Components::vsm("An agate worth at least 1,000 gp, which the spell consumes"),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Awaken a huge or smaller beast or plant to sentience.".to_string(),
        },
    ));

    // 4. Cloudkill
    reg.register_spell(Spell::new(
        "Cloudkill",
        5,
        SpellSchool::Conjuration,
        "1 action",
        120,
        Components::vs(),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Constitution,
            half_damage_on_save: true,
            damage_dice: Some("5d8".to_string()),
            damage_type: Some(DamageType::Poison),
            damage_scaling: Some("1d8".to_string()),
            fail_effect: None,
        },
    ));

    // 5. Commune
    reg.register_spell(Spell::new(
        "Commune",
        5,
        SpellSchool::Divination,
        "1 minute",
        0,
        Components::vsm("Incense and a vial of holy or unholy water"),
        "1 minute",
        false, // Concentration
        true,  // Ritual
        SpellEffect::Utility {
            description: "Contact your deity and ask up to three questions.".to_string(),
        },
    ));

    // 6. Commune with Nature
    reg.register_spell(Spell::new(
        "Commune with Nature",
        5,
        SpellSchool::Divination,
        "1 minute",
        0,
        Components::vs(),
        "Instantaneous",
        false, // Concentration
        true,  // Ritual
        SpellEffect::Utility {
            description: "Gain knowledge of the surrounding territory.".to_string(),
        },
    ));

    // 7. Cone of Cold
    reg.register_spell(Spell::new(
        "Cone of Cold",
        5,
        SpellSchool::Evocation,
        "1 action",
        0,
        Components::vsm("A small crystal or glass cone"),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Constitution,
            half_damage_on_save: true,
            damage_dice: Some("8d8".to_string()),
            damage_type: Some(DamageType::Cold),
            damage_scaling: Some("1d8".to_string()),
            fail_effect: None,
        },
    ));

    // 8. Conjure Elemental
    reg.register_spell(Spell::new(
        "Conjure Elemental",
        5,
        SpellSchool::Conjuration,
        "1 minute",
        90,
        Components::vsm("Burning incense for air, soft clay for earth, sulfur and phosphorus for fire, or water and sand for water"),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Summon { creature_name: "Elemental".to_string(), count: 1 },
    ));

    // 9. Contact Other Plane
    reg.register_spell(Spell::new(
        "Contact Other Plane",
        5,
        SpellSchool::Divination,
        "1 minute",
        0,
        Components::v(),
        "1 minute",
        false,  // Concentration
        true, // Ritual
        SpellEffect::Utility { description: "Mentally contact a demigod, the spirit of a long-dead sage, or some other mysterious entity from another plane.".to_string() },
    ));

    // 10. Contagion
    reg.register_spell(Spell::new(
        "Contagion",
        5,
        SpellSchool::Necromancy,
        "1 action",
        0,
        Components::vs(),
        "7 days",
        false, // Concentration
        false, // Ritual
        SpellEffect::SpellAttack {
            damage_dice: "".to_string(),
            damage_type: DamageType::Poison,
            damage_scaling: None,
            on_hit_effect: Some(Box::new(SpellEffect::Utility {
                description:
                    "Target is poisoned and must make Constitution saves to avoid a disease."
                        .to_string(),
            })),
        },
    ));

    // 11. Creation
    reg.register_spell(Spell::new(
        "Creation",
        5,
        SpellSchool::Illusion,
        "1 minute",
        30,
        Components::vsm("A tiny piece of matter of the same type of the item you plan to create"),
        "Special",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description:
                "Pull wisps of shadow material from the Shadowfell to create a nonliving object."
                    .to_string(),
        },
    ));

    // 12. Dispel Evil and Good
    reg.register_spell(Spell::new(
        "Dispel Evil and Good",
        5,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vsm("Holy water or powdered silver and iron"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Celestials, elementals, fey, fiends, and undead have disadvantage on attack rolls against you. You can also end the spell early to break enchantments or banish creatures.".to_string() },
    ));

    // 13. Dominate Person
    reg.register_spell(Spell::new(
        "Dominate Person",
        5,
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

    // 14. Dream
    reg.register_spell(Spell::new(
        "Dream",
        5,
        SpellSchool::Illusion,
        "1 minute",
        0,
        Components::vsm("A handful of sand, a dab of ink, and a writing quill plucked from a sleeping bird"),
        "8 hours",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Shape a creature's dreams, potentially dealing 3d6 psychic damage and preventing rest.".to_string() },
    ));

    // 15. Flame Strike
    reg.register_spell(Spell::new(
        "Flame Strike",
        5,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::vsm("Pinch of sulfur"),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: true,
            damage_dice: Some("4d6+4d6".to_string()),
            damage_type: Some(DamageType::Fire),
            damage_scaling: Some("1d6".to_string()),
            fail_effect: None,
        },
    ));

    // 16. Geas
    reg.register_spell(Spell::new(
        "Geas",
        5,
        SpellSchool::Enchantment,
        "1 minute",
        60,
        Components::v(),
        "30 days",
        false,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Wisdom, half_damage_on_save: false, damage_dice: None, damage_type: None, damage_scaling: None, fail_effect: Some(Box::new(SpellEffect::Utility { description: "Charmed and forced to carry out a service, taking 5d10 psychic damage if acting contrary.".to_string() })) },
    ));

    // 17. Greater Restoration
    reg.register_spell(Spell::new(
        "Greater Restoration",
        5,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vsm("Diamond dust worth at least 100 gp, which the spell consumes"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Undo a debilitating effect: exhaustion, charm, petrification, curse, stat reduction, or max HP reduction.".to_string() },
    ));

    // 18. Hallow
    reg.register_spell(Spell::new(
        "Hallow",
        5,
        SpellSchool::Evocation,
        "24 hours",
        0,
        Components::vsm("Herbs, oils, and incense worth at least 1,000 gp, which the spell consumes"),
        "Until dispelled",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Infuse an area with holy (or unholy) power, providing various protections and effects.".to_string() },
    ));

    // 19. Hold Monster
    reg.register_spell(Spell::new(
        "Hold Monster",
        5,
        SpellSchool::Enchantment,
        "1 action",
        90,
        Components::vsm("A small, straight piece of iron"),
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
                effect: ActiveEffect::Utility("Paralyzed.".to_string()),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    // 20. Insect Plague
    reg.register_spell(Spell::new(
        "Insect Plague",
        5,
        SpellSchool::Conjuration,
        "1 action",
        300,
        Components::vsm("A few grains of sugar, some kernels of grain, and a smear of fat"),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Constitution,
            half_damage_on_save: true,
            damage_dice: Some("4d10".to_string()),
            damage_type: Some(DamageType::Piercing),
            damage_scaling: Some("1d10".to_string()),
            fail_effect: None,
        },
    ));

    // 21. Legend Lore
    reg.register_spell(Spell::new(
        "Legend Lore",
        5,
        SpellSchool::Divination,
        "10 minutes",
        0,
        Components::vsm("Incense worth at least 250 gp, which the spell consumes, and four ivory strips worth at least 50 gp each"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Bring to mind a brief summary of the significant lore about a person, place, or object.".to_string() },
    ));

    // 22. Mass Cure Wounds
    reg.register_spell(Spell::new(
        "Mass Cure Wounds",
        5,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::vs(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::MultiTargetHealing {
            max_targets: 6,
            healing_dice: "3d8".to_string(),
        },
    ));

    // 23. Mislead
    reg.register_spell(Spell::new(
        "Mislead",
        5,
        SpellSchool::Illusion,
        "1 action",
        0,
        Components::s(),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Become invisible and create an illusory double that you can control and perceive through.".to_string() },
    ));

    // 24. Modify Memory
    reg.register_spell(Spell::new(
        "Modify Memory",
        5,
        SpellSchool::Enchantment,
        "1 action",
        30,
        Components::vs(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Wisdom, half_damage_on_save: false, damage_dice: None, damage_type: None, damage_scaling: None, fail_effect: Some(Box::new(SpellEffect::Utility { description: "Charmed, incapacitated, and unaware of surroundings; memory of the past 24 hours can be altered.".to_string() })) },
    ));

    // 25. Passwall
    reg.register_spell(Spell::new(
        "Passwall",
        5,
        SpellSchool::Transmutation,
        "1 action",
        30,
        Components::vsm("A pinch of sesame seeds"),
        "1 hour",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "A passage appears in a wooden, plaster, or stone surface.".to_string(),
        },
    ));

    // 26. Planar Binding
    reg.register_spell(Spell::new(
        "Planar Binding",
        5,
        SpellSchool::Abjuration,
        "1 hour",
        60,
        Components::vsm("A jewel worth at least 1,000 gp, which the spell consumes"),
        "24 hours",
        false, // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Charisma,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::Utility {
                description: "Bind a celestial, elemental, fey, or fiend to your service."
                    .to_string(),
            })),
        },
    ));

    // 27. Raise Dead
    reg.register_spell(Spell::new(
        "Raise Dead",
        5,
        SpellSchool::Necromancy,
        "1 hour",
        0,
        Components::vsm("A diamond worth at least 500 gp, which the spell consumes"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Return a dead creature to life with 1 hit point, provided it has not been dead longer than 10 days.".to_string() },
    ));

    // 28. Reincarnate
    reg.register_spell(Spell::new(
        "Reincarnate",
        5,
        SpellSchool::Transmutation,
        "1 hour",
        0,
        Components::vsm("Rare oils and unguents worth at least 1,000 gp, which the spell consumes"),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description:
                "Create a new adult body for a dead humanoid and cause its soul to enter that body."
                    .to_string(),
        },
    ));

    // 29. Scrying
    reg.register_spell(Spell::new(
        "Scrying",
        5,
        SpellSchool::Divination,
        "10 minutes",
        0,
        Components::vsm("A focus worth at least 1,000 gp, such as a crystal ball, a silver mirror, or a font filled with holy water"),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Wisdom, half_damage_on_save: false, damage_dice: None, damage_type: None, damage_scaling: None, fail_effect: Some(Box::new(SpellEffect::Utility { description: "See and hear a particular creature you choose that is on the same plane of existence.".to_string() })) },
    ));

    // 30. Seeming
    reg.register_spell(Spell::new(
        "Seeming",
        5,
        SpellSchool::Illusion,
        "1 action",
        30,
        Components::vs(),
        "8 hours",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Change the appearance of any number of creatures you can see."
                .to_string(),
        },
    ));

    // 31. Telekinesis
    reg.register_spell(Spell::new(
        "Telekinesis",
        5,
        SpellSchool::Transmutation,
        "1 action",
        60,
        Components::vs(),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Move objects or creatures with your mind.".to_string(),
        },
    ));

    // 32. Teleportation Circle
    reg.register_spell(Spell::new(
        "Teleportation Circle",
        5,
        SpellSchool::Conjuration,
        "1 minute",
        10,
        Components::vsm(
            "Rare chalks and inks infused with precious gems with 50 gp, which the spell consumes",
        ),
        "1 round",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Create a circular portal linked to a permanent teleportation circle."
                .to_string(),
        },
    ));

    // 33. Tree Stride
    reg.register_spell(Spell::new(
        "Tree Stride",
        5,
        SpellSchool::Conjuration,
        "1 action",
        0,
        Components::vs(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Step into one living tree and emerge from another.".to_string(),
        },
    ));

    // 34. Wall of Force
    reg.register_spell(Spell::new(
        "Wall of Force",
        5,
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vsm("A pinch of powder made by crushing a clear gemstone"),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "An invisible wall of force springs into existence.".to_string(),
        },
    ));

    // 35. Wall of Stone
    reg.register_spell(Spell::new(
        "Wall of Stone",
        5,
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vsm("A small block of granite"),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "A nonmagical wall of solid stone springs into existence.".to_string(),
        },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animate_objects_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Animate Objects").expect("Animate Objects not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_antilife_shell_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Antilife Shell").expect("Antilife Shell not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_awaken_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Awaken").expect("Awaken not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_cloudkill_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Cloudkill").expect("Cloudkill not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_commune_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Commune").expect("Commune not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_commune_with_nature_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("Commune with Nature").expect("Commune with Nature not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_cone_of_cold_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Cone of Cold").expect("Cone of Cold not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_conjure_elemental_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("Conjure Elemental").expect("Conjure Elemental not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_contact_other_plane_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("Contact Other Plane").expect("Contact Other Plane not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_contagion_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Contagion").expect("Contagion not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_creation_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Creation").expect("Creation not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_dispel_evil_and_good_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Dispel Evil and Good")
            .expect("Dispel Evil and Good not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_dominate_person_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Dominate Person").expect("Dominate Person not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_dream_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Dream").expect("Dream not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_flame_strike_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Flame Strike").expect("Flame Strike not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_geas_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Geas").expect("Geas not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_greater_restoration_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("Greater Restoration").expect("Greater Restoration not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_hallow_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Hallow").expect("Hallow not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_hold_monster_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Hold Monster").expect("Hold Monster not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_insect_plague_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Insect Plague").expect("Insect Plague not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_legend_lore_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Legend Lore").expect("Legend Lore not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_mass_cure_wounds_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("Mass Cure Wounds").expect("Mass Cure Wounds not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_mislead_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Mislead").expect("Mislead not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_modify_memory_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Modify Memory").expect("Modify Memory not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_passwall_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Passwall").expect("Passwall not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_planar_binding_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Planar Binding").expect("Planar Binding not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_raise_dead_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Raise Dead").expect("Raise Dead not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_reincarnate_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Reincarnate").expect("Reincarnate not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_scrying_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Scrying").expect("Scrying not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_seeming_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Seeming").expect("Seeming not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_telekinesis_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Telekinesis").expect("Telekinesis not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_teleportation_circle_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Teleportation Circle")
            .expect("Teleportation Circle not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_tree_stride_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Tree Stride").expect("Tree Stride not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_wall_of_force_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Wall of Force").expect("Wall of Force not found");
        assert_eq!(spell.level, 5);
    }
    #[test]
    fn test_wall_of_stone_spell() {
        let mut reg = SpellRegistry::new();
        load_level5_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Wall of Stone").expect("Wall of Stone not found");
        assert_eq!(spell.level, 5);
    }
}
