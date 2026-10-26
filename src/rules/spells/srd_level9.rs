use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use crate::rules::spells::registry::SpellRegistry;
use crate::rules::spells::spell::{Components, Spell, SpellEffect, SpellSchool};

/// Registra todas as magias de Nível 9 do SRD
pub fn load_level9_spells(reg: &mut SpellRegistry) {
    // 1. Astral Projection
    reg.register_spell(Spell::new(
        "Astral Projection",
        9,
        SpellSchool::Necromancy,
        "1 hour",
        10,
        Components::vsm("For each creature you affect with this spell, you must provide one jacinth worth at least 1,000 gp and one ornately carved bar of silver worth at least 100 gp, all of which the spell consumes"),
        "Special",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Project your astral bodies into the Astral Plane, leaving your physical bodies behind in suspended animation.".to_string() },
    ));

    // 2. Foresight
    reg.register_spell(Spell::new(
        "Foresight",
        9,
        SpellSchool::Divination,
        "1 minute",
        0,
        Components::vsm("A hummingbird feather"),
        "8 hours",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Target gains advantage on all attacks, checks, and saves, and enemies have disadvantage against them. Target can't be surprised.".to_string() },
    ));

    // 3. Gate
    reg.register_spell(Spell::new(
        "Gate",
        9,
        SpellSchool::Conjuration,
        "1 action",
        60,
        Components::vsm("A diamond worth at least 5,000 gp"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Open a portal to a different plane of existence, optionally calling a specific entity through it.".to_string() },
    ));

    // 4. Imprisonment
    reg.register_spell(Spell::new(
        "Imprisonment",
        9,
        SpellSchool::Abjuration,
        "1 minute",
        30,
        Components::vsm("A vellum depiction or a carved statuette in the likeness of the target, and a special component that varies according to the version of the spell you choose, worth at least 500 gp per Hit Die of the target"),
        "Until dispelled",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Trap a creature indefinitely in various forms (Burial, Chaining, Hedged Prison, Minimus Containment, or Slumber).".to_string() },
    ));

    // 5. Mass Heal
    reg.register_spell(Spell::new(
        "Mass Heal",
        9,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::vs(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Healing {
            healing_dice: "700".to_string(),
        },
    ));

    // 6. Meteor Swarm
    reg.register_spell(Spell::new(
        "Meteor Swarm",
        9,
        SpellSchool::Evocation,
        "1 action",
        5280,
        Components::vs(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: true,
            damage_dice: Some("20d6+20d6".to_string()),
            damage_type: Some(DamageType::Fire),
            damage_scaling: None,
            fail_effect: None,
        },
    ));

    // 7. Power Word Heal
    reg.register_spell(Spell::new(
        "Power Word Heal",
        9,
        SpellSchool::Evocation,
        "1 action",
        0,
        Components::v(),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "A creature you touch regains all its hit points, and all conditions (charmed, frightened, paralyzed, stunned) end.".to_string() },
    ));

    // 8. Power Word Kill
    reg.register_spell(Spell::new(
        "Power Word Kill",
        9,
        SpellSchool::Enchantment,
        "1 action",
        60,
        Components::v(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "If the target has 100 hit points or fewer, it dies instantly."
                .to_string(),
        },
    ));

    // 9. Prismatic Wall
    reg.register_spell(Spell::new(
        "Prismatic Wall",
        9,
        SpellSchool::Abjuration,
        "1 action",
        90,
        Components::vs(),
        "10 minutes",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description:
                "Create a wall of multi-colored, multi-layered, destructive magical light."
                    .to_string(),
        },
    ));

    // 10. Shapechange
    reg.register_spell(Spell::new(
        "Shapechange",
        9,
        SpellSchool::Transmutation,
        "1 action",
        0,
        Components::vsm("A jade circlet worth at least 1,500 gp, which you must place on your head before you cast the spell"),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Assume the form of a different creature for the duration, retaining your own alignment and personality.".to_string() },
    ));

    // 11. Time Stop
    reg.register_spell(Spell::new(
        "Time Stop",
        9,
        SpellSchool::Transmutation,
        "1 action",
        0,
        Components::v(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Stop the flow of time for everyone but yourself for 1d4 + 1 turns."
                .to_string(),
        },
    ));

    // 12. True Polymorph
    reg.register_spell(Spell::new(
        "True Polymorph",
        9,
        SpellSchool::Transmutation,
        "1 action",
        30,
        Components::vsm("A drop of mercury, a dollop of gum arabic, and a wisp of smoke"),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Transform a creature into another creature, a creature into an object, or an object into a creature. Can become permanent.".to_string() },
    ));

    // 13. True Resurrection
    reg.register_spell(Spell::new(
        "True Resurrection",
        9,
        SpellSchool::Necromancy,
        "1 hour",
        0,
        Components::vsm("A sprinkle of holy water and diamonds worth at least 25,000 gp, which the spell consumes"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Return a creature that has been dead for no longer than 200 years to life with all hit points and missing body parts restored.".to_string() },
    ));

    // 14. Weird
    reg.register_spell(Spell::new(
        "Weird",
        9,
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
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::Utility {
                description:
                    "Target is frightened and takes 4d10 psychic damage per turn on a failed save."
                        .to_string(),
            })),
        },
    ));

    // 15. Wish
    reg.register_spell(Spell::new(
        "Wish",
        9,
        SpellSchool::Conjuration,
        "1 action",
        0,
        Components::v(),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Alter the very foundations of reality. Can duplicate any spell of 8th level or lower, or create custom massive effects at severe risk to the caster.".to_string() },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astral_projection_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("Astral Projection").expect("Astral Projection not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_foresight_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Foresight").expect("Foresight not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_gate_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Gate").expect("Gate not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_imprisonment_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Imprisonment").expect("Imprisonment not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_mass_heal_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Mass Heal").expect("Mass Heal not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_meteor_swarm_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Meteor Swarm").expect("Meteor Swarm not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_power_word_heal_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Power Word Heal").expect("Power Word Heal not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_power_word_kill_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Power Word Kill").expect("Power Word Kill not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_prismatic_wall_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Prismatic Wall").expect("Prismatic Wall not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_shapechange_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Shapechange").expect("Shapechange not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_time_stop_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Time Stop").expect("Time Stop not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_true_polymorph_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("True Polymorph").expect("True Polymorph not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_true_resurrection_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("True Resurrection").expect("True Resurrection not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_weird_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Weird").expect("Weird not found");
        assert_eq!(spell.level, 9);
    }
    #[test]
    fn test_wish_spell() {
        let mut reg = SpellRegistry::new();
        load_level9_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Wish").expect("Wish not found");
        assert_eq!(spell.level, 9);
    }
}
