use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use crate::rules::spells::registry::SpellRegistry;
use crate::rules::spells::spell::{Components, Spell, SpellEffect, SpellSchool};

/// Registra todas as magias de Nível 3 do SRD
pub fn load_level3_spells(reg: &mut SpellRegistry) {
    // 1. Animate Dead
    reg.register_spell(Spell::new(
        "Animate Dead",
        3,
        SpellSchool::Necromancy,
        "1 minute",
        10,
        Components::vsm("A drop of blood, a piece of flesh, and a pinch of bone dust"),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Summon {
            creature_name: "Skeleton/Zombie".to_string(),
            count: 1,
        },
    ));

    // 2. Beacon of Hope
    reg.register_spell(Spell::new(
        "Beacon of Hope",
        3,
        SpellSchool::Abjuration,
        "1 action",
        30,
        Components::vs(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::MultiTargetApplyEffect {
            max_targets: 99, // Qualquer número no alcance
            effect: ActiveEffect::Utility(
                "Advantage on Wisdom saves and Death saves. Max healing from any healing spell."
                    .to_string(),
            ),
            duration: DurationType::DurationMinutes(1),
        },
    ));

    // 3. Bestow Curse
    reg.register_spell(Spell::new(
        "Bestow Curse",
        3,
        SpellSchool::Necromancy,
        "1 action",
        0, // Touch
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
                    "Cursed: Penalty to checks/saves, wasted actions, or extra damage taken."
                        .to_string(),
                ),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    // 4. Blink
    reg.register_spell(Spell::new(
        "Blink",
        3,
        SpellSchool::Transmutation,
        "1 action",
        0, // Self
        Components::vs(),
        "1 minute",
        false, // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility("Roll a d20 at the end of each turn. On an 11 or higher, vanish to the Ethereal Plane until start of next turn.".to_string()),
            duration: DurationType::DurationMinutes(1),
        }
    ));

    // 5. Call Lightning
    reg.register_spell(Spell::new(
        "Call Lightning",
        3,
        SpellSchool::Conjuration,
        "1 action",
        120,
        Components::vs(),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 5,
            save_ability: Some(Ability::Dexterity),
            condition: None,
            damage_dice: Some("3d10".to_string()),
            damage_type: Some(DamageType::Lightning),
            description:
                "A storm cloud creates a lightning bolt. Can repeat action on subsequent turns."
                    .to_string(),
        },
    ));

    // 6. Clairvoyance
    reg.register_spell(Spell::new(
        "Clairvoyance",
        3,
        SpellSchool::Divination,
        "10 minutes",
        5280, // 1 mile
        Components::vsm("A focus worth at least 100 gp, either a jeweled horn for hearing or a glass eye for seeing"),
        "10 minutes",
        true, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Creates an invisible sensor that lets you hear or see in a familiar location up to 1 mile away.".to_string(),
        }
    ));

    // 7. Conjure Animals
    reg.register_spell(Spell::new(
        "Conjure Animals",
        3,
        SpellSchool::Conjuration,
        "1 action",
        60,
        Components::vs(),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Summon {
            creature_name: "Fey Beasts".to_string(),
            count: 1, // Varies by CR, abstracted to 1 effect representing the summon
        },
    ));

    // 8. Counterspell
    reg.register_spell(Spell::new(
        "Counterspell",
        3,
        SpellSchool::Abjuration,
        "1 reaction",
        60,
        Components::new(false, true, None),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Interrupts a creature you can see in the process of casting a spell. Fails if level 3 or lower. DC 10+level if higher.".to_string(),
        }
    ));

    // 9. Create Food and Water
    reg.register_spell(Spell::new(
        "Create Food and Water",
        3,
        SpellSchool::Conjuration,
        "1 action",
        30,
        Components::vs(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Creates 45 pounds of food and 30 gallons of water.".to_string(),
        },
    ));

    // 10. Daylight
    reg.register_spell(Spell::new(
        "Daylight",
        3,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::vs(),
        "1 hour",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "A 60-foot sphere of bright light spreading from a point. Dispels magical darkness created by spells of 3rd level or lower.".to_string(),
        },
    ));

    // 11. Dispel Magic
    reg.register_spell(Spell::new(
        "Dispel Magic",
        3,
        SpellSchool::Abjuration,
        "1 action",
        120,
        Components::vs(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Choose one creature, object, or magical effect. Any spell of 3rd level or lower on the target ends. For higher levels, make an ability check.".to_string() }
    ));

    // 12. Fear
    reg.register_spell(Spell::new(
        "Fear",
        3,
        SpellSchool::Illusion,
        "1 action",
        30,
        Components::vsm("A white feather or the heart of a hen"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 30,
            save_ability: Some(Ability::Wisdom),
            condition: Some(crate::rules::conditions::Condition::Frightened),
            damage_dice: None,
            damage_type: None,
            description: "".to_string(),
        },
    ));

    // 13. Feign Death
    reg.register_spell(Spell::new(
        "Feign Death",
        3,
        SpellSchool::Necromancy,
        "1 action",
        0,
        Components::vsm("A pinch of graveyard dirt"),
        "1 hour",
        false, // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target appears dead and is blinded/incapacitated, but has resistance to all damage except psychic.".to_string()), duration: DurationType::DurationMinutes(60) }
    ));

    // 14. Fireball
    reg.register_spell(Spell::new(
        "Fireball",
        3,
        SpellSchool::Evocation,
        "1 action",
        150,
        Components::vsm("A tiny ball of bat guano and sulfur"),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 20,
            save_ability: Some(Ability::Dexterity),
            condition: None,
            damage_dice: Some("8d6".to_string()),
            damage_type: Some(DamageType::Fire),
            description: "".to_string(),
        },
    ));

    // 15. Fly
    reg.register_spell(Spell::new(
        "Fly",
        3,
        SpellSchool::Transmutation,
        "1 action",
        0,
        Components::vsm("A wing feather from any bird"),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility("Target gains a flying speed of 60 feet.".to_string()),
            duration: DurationType::DurationMinutes(10),
        },
    ));

    // 16. Gaseous Form
    reg.register_spell(Spell::new(
        "Gaseous Form",
        3,
        SpellSchool::Transmutation,
        "1 action",
        0,
        Components::vsm("A bit of gauze and a wisp of smoke"),
        "1 hour",
        true, // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target transforms into a misty cloud. Has fly speed 10 ft, resistance to nonmagical damage, and advantage on STR/DEX/CON saves.".to_string()), duration: DurationType::DurationMinutes(60) }
    ));

    // 17. Glyph of Warding
    reg.register_spell(Spell::new(
        "Glyph of Warding",
        3,
        SpellSchool::Abjuration,
        "1 hour",
        0,
        Components::vsm("Incense and powdered diamond worth at least 200 gp, which the spell consumes"),
        "Until dispelled",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Inscribes a glyph that harms other creatures either with explosive runes (5d8 damage) or a stored spell up to 3rd level.".to_string() }
    ));

    // 18. Haste
    reg.register_spell(Spell::new(
        "Haste",
        3,
        SpellSchool::Transmutation,
        "1 action",
        30,
        Components::vsm("A shaving of licorice root"),
        "1 minute",
        true, // Concentration
        false, // Ritual
        SpellEffect::Compound(vec![SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target speed is doubled, gains +2 AC, advantage on DEX saves, and an additional action each turn.".to_string()), duration: DurationType::DurationMinutes(1) }])
    ));

    // 19. Hypnotic Pattern
    reg.register_spell(Spell::new(
        "Hypnotic Pattern",
        3,
        SpellSchool::Illusion,
        "1 action",
        120,
        Components::sm(
            "A glowing stick of incense or a crystal vial filled with phosphorescent material",
        ),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 30,
            save_ability: Some(Ability::Wisdom),
            condition: Some(crate::rules::conditions::Condition::Charmed),
            damage_dice: None,
            damage_type: None,
            description: "".to_string(),
        },
    ));

    // 20. Lightning Bolt
    reg.register_spell(Spell::new(
        "Lightning Bolt",
        3,
        SpellSchool::Evocation,
        "1 action",
        100,
        Components::vsm("A bit of fur and a rod of amber, crystal, or glass"),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 5,
            save_ability: Some(Ability::Dexterity),
            condition: None,
            damage_dice: Some("8d6".to_string()),
            damage_type: Some(DamageType::Lightning),
            description: "".to_string(),
        },
    ));

    // 21. Magic Circle
    reg.register_spell(Spell::new(
        "Magic Circle",
        3,
        SpellSchool::Abjuration,
        "1 minute",
        10,
        Components::vsm("Holy water or powdered silver and iron worth at least 100 gp, which the spell consumes"),
        "1 hour",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Creates a 10-foot radius cylinder of magical energy. Creatures of chosen types can't willingly enter by nonmagical means.".to_string() }
    ));

    // 22. Major Image
    reg.register_spell(Spell::new(
        "Major Image",
        3,
        SpellSchool::Illusion,
        "1 action",
        120,
        Components::vsm("A bit of fleece"),
        "10 minutes",
        true, // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Creates an illusion of an object, creature, or phenomenon no larger than a 20-foot cube, including sounds, smells, and temperature.".to_string() }
    ));

    // 23. Mass Healing Word
    reg.register_spell(Spell::new(
        "Mass Healing Word",
        3,
        SpellSchool::Evocation,
        "1 bonus action",
        60,
        Components::v(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::MultiTargetSpellAttack {
            projectiles_count: 6,
            damage_dice: "1d4".to_string(),
            damage_type: DamageType::Radiant,
            auto_hit: true,
        },
    ));

    // 24. Meld into Stone
    reg.register_spell(Spell::new(
        "Meld into Stone",
        3,
        SpellSchool::Transmutation,
        "1 action",
        0,
        Components::vs(),
        "8 hours",
        false, // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target steps into a stone object or surface large enough to fully contain it. Cannot see out, has advantage on hearing checks.".to_string()), duration: DurationType::DurationMinutes(480) }
    ));

    // 25. Nondetection
    reg.register_spell(Spell::new(
        "Nondetection",
        3,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vsm("A pinch of diamond dust worth 25 gp sprinkled over the target, which the spell consumes"),
        "8 hours",
        false, // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target is hidden from divination magic and cannot be targeted by such spells or perceived through magical scrying sensors.".to_string()), duration: DurationType::DurationMinutes(480) }
    ));

    // 26. Phantom Steed
    reg.register_spell(Spell::new(
        "Phantom Steed",
        3,
        SpellSchool::Illusion,
        "1 minute",
        30,
        Components::vs(),
        "1 hour",
        false, // Concentration
        false, // Ritual
        SpellEffect::SummonCreature {
            monster_id: "Phantom Steed".to_string(),
        },
    ));

    // 27. Protection from Energy
    reg.register_spell(Spell::new(
        "Protection from Energy",
        3,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vs(),
        "1 hour",
        true, // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target has resistance to one damage type: acid, cold, fire, lightning, or thunder.".to_string()), duration: DurationType::DurationMinutes(60) }
    ));

    // 28. Remove Curse
    reg.register_spell(Spell::new(
        "Remove Curse",
        3,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vs(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "At your touch, all curses on one creature or object end.".to_string(),
        },
    ));

    // 29. Revivify
    reg.register_spell(Spell::new(
        "Revivify",
        3,
        SpellSchool::Necromancy,
        "1 action",
        0,
        Components::vsm("Diamonds worth 300 gp, which the spell consumes"),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "You touch a creature that has died within the last minute. That creature returns to life with 1 hit point.".to_string() }
    ));

    // 30. Sending
    reg.register_spell(Spell::new(
        "Sending",
        3,
        SpellSchool::Evocation,
        "1 action",
        9999,
        Components::vsm("A short piece of fine copper wire"),
        "1 round",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Send a short message of 25 words or less to a creature with which you are familiar. They can reply.".to_string() }
    ));

    // 31. Sleet Storm
    reg.register_spell(Spell::new(
        "Sleet Storm",
        3,
        SpellSchool::Conjuration,
        "1 action",
        150,
        Components::vsm("A pinch of dust and a few drops of water"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 40,
            save_ability: Some(Ability::Dexterity),
            condition: Some(crate::rules::conditions::Condition::Prone),
            damage_dice: None,
            damage_type: None,
            description: "".to_string(),
        },
    ));

    // 32. Slow
    reg.register_spell(Spell::new(
        "Slow",
        3,
        SpellSchool::Transmutation,
        "1 action",
        120,
        Components::vsm("A drop of molasses"),
        "1 minute",
        true, // Concentration
        false, // Ritual
        SpellEffect::MultiTargetApplyEffect { max_targets: 6, effect: ActiveEffect::Utility("Target speed halved, -2 to AC and DEX saves, and cannot use reactions. Can only use action or bonus action, not both. Spells might take 2 turns to cast.".to_string()), duration: DurationType::DurationMinutes(1) }
    ));

    // 33. Speak with Dead
    reg.register_spell(Spell::new(
        "Speak with Dead",
        3,
        SpellSchool::Necromancy,
        "1 action",
        10,
        Components::vsm("Burning incense"),
        "10 minutes",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Grant the semblance of life and intelligence to a corpse of your choice, allowing it to answer up to 5 questions.".to_string() }
    ));

    // 34. Speak with Plants
    reg.register_spell(Spell::new(
        "Speak with Plants",
        3,
        SpellSchool::Transmutation,
        "1 action",
        0,
        Components::vs(),
        "10 minutes",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Imbue plants within 30 feet of you with limited sentience and animation, giving them the ability to communicate with you and follow simple commands.".to_string() }
    ));

    // 35. Spirit Guardians
    reg.register_spell(Spell::new(
        "Spirit Guardians",
        3,
        SpellSchool::Conjuration,
        "1 action",
        0,
        Components::vsm("A holy symbol"),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 15,
            save_ability: Some(Ability::Wisdom),
            condition: None,
            damage_dice: Some("3d8".to_string()),
            damage_type: Some(DamageType::Radiant),
            description: "".to_string(),
        },
    ));

    // 36. Stinking Cloud
    reg.register_spell(Spell::new(
        "Stinking Cloud",
        3,
        SpellSchool::Conjuration,
        "1 action",
        90,
        Components::vsm("A rotten egg or several skunk cabbage leaves"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 20,
            save_ability: Some(Ability::Constitution),
            condition: Some(crate::rules::conditions::Condition::Incapacitated),
            damage_dice: None,
            damage_type: None,
            description: "".to_string(),
        },
    ));

    // 37. Tongues
    reg.register_spell(Spell::new(
        "Tongues",
        3,
        SpellSchool::Divination,
        "1 action",
        0,
        Components::vsm("A small clay model of a ziggurat"),
        "1 hour",
        false, // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target understands any spoken language it hears, and any creature that knows at least one language and can hear the target understands what it says.".to_string()), duration: DurationType::DurationMinutes(60) }
    ));

    // 38. Vampiric Touch
    reg.register_spell(Spell::new(
        "Vampiric Touch",
        3,
        SpellSchool::Necromancy,
        "1 action",
        0,
        Components::vs(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SpellAttack {
            damage_dice: "3d6".to_string(),
            damage_type: DamageType::Necrotic,
            damage_scaling: None,
            on_hit_effect: None,
        },
    ));

    // 39. Water Breathing
    reg.register_spell(Spell::new(
        "Water Breathing",
        3,
        SpellSchool::Transmutation,
        "1 action",
        30,
        Components::vsm("A short reed or piece of straw"),
        "24 hours",
        false, // Concentration
        false, // Ritual
        SpellEffect::MultiTargetApplyEffect {
            max_targets: 10,
            effect: ActiveEffect::Utility(
                "Targets can breathe underwater for the duration.".to_string(),
            ),
            duration: DurationType::DurationMinutes(1440),
        },
    ));

    // 40. Water Walk
    reg.register_spell(Spell::new(
        "Water Walk",
        3,
        SpellSchool::Transmutation,
        "1 action",
        30,
        Components::vsm("A piece of cork"),
        "1 hour",
        false, // Concentration
        false, // Ritual
        SpellEffect::MultiTargetApplyEffect {
            max_targets: 10,
            effect: ActiveEffect::Utility(
                "Targets can move across any liquid surface as if it were harmless solid ground."
                    .to_string(),
            ),
            duration: DurationType::DurationMinutes(60),
        },
    ));

    // 41. Wind Wall
    reg.register_spell(Spell::new(
        "Wind Wall",
        3,
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vsm("A tiny fan and a feather of exotic origin"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 50,
            save_ability: Some(Ability::Strength),
            condition: None,
            damage_dice: Some("3d8".to_string()),
            damage_type: Some(DamageType::Bludgeoning),
            description: "".to_string(),
        },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::dice::DiceMode;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::background::create_acolyte;
    use crate::rules::character::class::{ClassVariant, Fighter};
    use crate::rules::character::entity::Entity;
    use crate::rules::character::race::{Human, RaceVariant};
    use crate::rules::combat::attack::DamageAffinity;
    use crate::rules::spells::casting::{cast_spell, CastOutcome};

    #[test]
    fn test_animate_dead_spell() {
        let mut reg = SpellRegistry::new();
        load_level3_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Animate Dead").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2); // needs level 3 slots
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            None,
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::Summoned { .. }));
    }

    #[test]
    fn test_beacon_of_hope_spell() {
        let mut reg = SpellRegistry::new();
        load_level3_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Beacon of Hope").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let mut target = caster.clone();
        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_bestow_curse_spell() {
        let mut reg = SpellRegistry::new();
        load_level3_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Bestow Curse").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let mut target = Entity::new(
            "Goblin",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::SaveRoll { .. }));
    }

    #[test]
    fn test_blink_spell() {
        let mut reg = SpellRegistry::new();
        load_level3_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Blink").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let mut target = caster.clone();
        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_call_lightning_spell() {
        let mut reg = SpellRegistry::new();
        load_level3_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Call Lightning").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Druid",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            None,
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::AreaHazardCreated { .. }
        ));
    }

    #[test]
    fn test_clairvoyance_spell() {
        let mut reg = SpellRegistry::new();
        load_level3_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Clairvoyance").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            None,
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_conjure_animals_spell() {
        let mut reg = SpellRegistry::new();
        load_level3_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Conjure Animals").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Druid",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            None,
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::Summoned { .. }));
    }

    #[test]
    fn test_counterspell_spell() {
        let mut reg = SpellRegistry::new();
        load_level3_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Counterspell").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            None,
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_create_food_and_water_spell() {
        let mut reg = SpellRegistry::new();
        load_level3_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Create Food and Water").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            None,
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_daylight_spell() {
        let mut reg = SpellRegistry::new();
        load_level3_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Daylight").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            None,
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_dispel_magic_spell() {
        let spell = Spell::new(
            "Dispel Magic",
            3,
            SpellSchool::Abjuration,
            "1 action",
            120,
            Components::vs(),
            "Instantaneous",
            false,
            false,
            SpellEffect::Utility { description: "Choose one creature, object, or magical effect. Any spell of 3rd level or lower on the target ends. For higher levels, make an ability check.".to_string() }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_fear_spell() {
        let spell = Spell::new(
            "Fear",
            3,
            SpellSchool::Illusion,
            "1 action",
            30,
            Components::vsm("A white feather or the heart of a hen"),
            "1 minute",
            true,
            false,
            SpellEffect::AreaHazard {
                radius_feet: 30,
                save_ability: Some(Ability::Wisdom),
                condition: Some(crate::rules::conditions::Condition::Frightened),
                damage_dice: None,
                damage_type: None,
                description: "".to_string(),
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::AreaHazardCreated { .. }
        ));
    }

    #[test]
    fn test_feign_death_spell() {
        let spell = Spell::new(
            "Feign Death",
            3,
            SpellSchool::Necromancy,
            "1 action",
            0,
            Components::vsm("A pinch of graveyard dirt"),
            "1 hour",
            false,
            false,
            SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target appears dead and is blinded/incapacitated, but has resistance to all damage except psychic.".to_string()), duration: DurationType::DurationMinutes(60) }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_fireball_spell() {
        let spell = Spell::new(
            "Fireball",
            3,
            SpellSchool::Evocation,
            "1 action",
            150,
            Components::vsm("A tiny ball of bat guano and sulfur"),
            "Instantaneous",
            false,
            false,
            SpellEffect::AreaHazard {
                radius_feet: 20,
                save_ability: Some(Ability::Dexterity),
                condition: None,
                damage_dice: Some("8d6".to_string()),
                damage_type: Some(DamageType::Fire),
                description: "".to_string(),
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::AreaHazardCreated { .. }
        ));
    }

    #[test]
    fn test_fly_spell() {
        let spell = Spell::new(
            "Fly",
            3,
            SpellSchool::Transmutation,
            "1 action",
            0,
            Components::vsm("A wing feather from any bird"),
            "10 minutes",
            true,
            false,
            SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility(
                    "Target gains a flying speed of 60 feet.".to_string(),
                ),
                duration: DurationType::DurationMinutes(10),
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_gaseous_form_spell() {
        let spell = Spell::new(
            "Gaseous Form",
            3,
            SpellSchool::Transmutation,
            "1 action",
            0,
            Components::vsm("A bit of gauze and a wisp of smoke"),
            "1 hour",
            true,
            false,
            SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target transforms into a misty cloud. Has fly speed 10 ft, resistance to nonmagical damage, and advantage on STR/DEX/CON saves.".to_string()), duration: DurationType::DurationMinutes(60) }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_glyph_of_warding_spell() {
        let spell = Spell::new(
            "Glyph of Warding",
            3,
            SpellSchool::Abjuration,
            "1 hour",
            0,
            Components::vsm("Incense and powdered diamond worth at least 200 gp, which the spell consumes"),
            "Until dispelled",
            false,
            false,
            SpellEffect::Utility { description: "Inscribes a glyph that harms other creatures either with explosive runes (5d8 damage) or a stored spell up to 3rd level.".to_string() }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_haste_spell() {
        let spell = Spell::new(
            "Haste",
            3,
            SpellSchool::Transmutation,
            "1 action",
            30,
            Components::vsm("A shaving of licorice root"),
            "1 minute",
            true,
            false,
            SpellEffect::Compound(vec![SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target speed is doubled, gains +2 AC, advantage on DEX saves, and an additional action each turn.".to_string()), duration: DurationType::DurationMinutes(1) }])
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::CompoundOutcome(..)));
    }

    #[test]
    fn test_hypnotic_pattern_spell() {
        let spell = Spell::new(
            "Hypnotic Pattern",
            3,
            SpellSchool::Illusion,
            "1 action",
            120,
            Components::sm(
                "A glowing stick of incense or a crystal vial filled with phosphorescent material",
            ),
            "1 minute",
            true,
            false,
            SpellEffect::AreaHazard {
                radius_feet: 30,
                save_ability: Some(Ability::Wisdom),
                condition: Some(crate::rules::conditions::Condition::Charmed),
                damage_dice: None,
                damage_type: None,
                description: "".to_string(),
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::AreaHazardCreated { .. }
        ));
    }

    #[test]
    fn test_lightning_bolt_spell() {
        let spell = Spell::new(
            "Lightning Bolt",
            3,
            SpellSchool::Evocation,
            "1 action",
            100,
            Components::vsm("A bit of fur and a rod of amber, crystal, or glass"),
            "Instantaneous",
            false,
            false,
            SpellEffect::AreaHazard {
                radius_feet: 5,
                save_ability: Some(Ability::Dexterity),
                condition: None,
                damage_dice: Some("8d6".to_string()),
                damage_type: Some(DamageType::Lightning),
                description: "".to_string(),
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::AreaHazardCreated { .. }
        ));
    }

    #[test]
    fn test_magic_circle_spell() {
        let spell = Spell::new(
            "Magic Circle",
            3,
            SpellSchool::Abjuration,
            "1 minute",
            10,
            Components::vsm("Holy water or powdered silver and iron worth at least 100 gp, which the spell consumes"),
            "1 hour",
            false,
            false,
            SpellEffect::Utility { description: "Creates a 10-foot radius cylinder of magical energy. Creatures of chosen types can't willingly enter by nonmagical means.".to_string() }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_major_image_spell() {
        let spell = Spell::new(
            "Major Image",
            3,
            SpellSchool::Illusion,
            "1 action",
            120,
            Components::vsm("A bit of fleece"),
            "10 minutes",
            true,
            false,
            SpellEffect::Utility { description: "Creates an illusion of an object, creature, or phenomenon no larger than a 20-foot cube, including sounds, smells, and temperature.".to_string() }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_mass_healing_word_spell() {
        let spell = Spell::new(
            "Mass Healing Word",
            3,
            SpellSchool::Evocation,
            "1 bonus action",
            60,
            Components::v(),
            "Instantaneous",
            false,
            false,
            SpellEffect::MultiTargetSpellAttack {
                projectiles_count: 6,
                damage_dice: "1d4".to_string(),
                damage_type: DamageType::Radiant,
                auto_hit: true,
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::Healed { .. }));
    }

    #[test]
    fn test_meld_into_stone_spell() {
        let spell = Spell::new(
            "Meld into Stone",
            3,
            SpellSchool::Transmutation,
            "1 action",
            0,
            Components::vs(),
            "8 hours",
            false,
            false,
            SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target steps into a stone object or surface large enough to fully contain it. Cannot see out, has advantage on hearing checks.".to_string()), duration: DurationType::DurationMinutes(480) }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_nondetection_spell() {
        let spell = Spell::new(
            "Nondetection",
            3,
            SpellSchool::Abjuration,
            "1 action",
            0,
            Components::vsm("A pinch of diamond dust worth 25 gp sprinkled over the target, which the spell consumes"),
            "8 hours",
            false,
            false,
            SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target is hidden from divination magic and cannot be targeted by such spells or perceived through magical scrying sensors.".to_string()), duration: DurationType::DurationMinutes(480) }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_phantom_steed_spell() {
        let spell = Spell::new(
            "Phantom Steed",
            3,
            SpellSchool::Illusion,
            "1 minute",
            30,
            Components::vs(),
            "1 hour",
            false,
            false,
            SpellEffect::SummonCreature {
                monster_id: "Phantom Steed".to_string(),
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::Summoned { .. }));
    }

    #[test]
    fn test_protection_from_energy_spell() {
        let spell = Spell::new(
            "Protection from Energy",
            3,
            SpellSchool::Abjuration,
            "1 action",
            0,
            Components::vs(),
            "1 hour",
            true,
            false,
            SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target has resistance to one damage type: acid, cold, fire, lightning, or thunder.".to_string()), duration: DurationType::DurationMinutes(60) }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_remove_curse_spell() {
        let spell = Spell::new(
            "Remove Curse",
            3,
            SpellSchool::Abjuration,
            "1 action",
            0,
            Components::vs(),
            "Instantaneous",
            false,
            false,
            SpellEffect::Utility {
                description: "At your touch, all curses on one creature or object end.".to_string(),
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_revivify_spell() {
        let spell = Spell::new(
            "Revivify",
            3,
            SpellSchool::Necromancy,
            "1 action",
            0,
            Components::vsm("Diamonds worth 300 gp, which the spell consumes"),
            "Instantaneous",
            false,
            false,
            SpellEffect::Utility { description: "You touch a creature that has died within the last minute. That creature returns to life with 1 hit point.".to_string() }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_sending_spell() {
        let spell = Spell::new(
            "Sending",
            3,
            SpellSchool::Evocation,
            "1 action",
            9999,
            Components::vsm("A short piece of fine copper wire"),
            "1 round",
            false,
            false,
            SpellEffect::Utility { description: "Send a short message of 25 words or less to a creature with which you are familiar. They can reply.".to_string() }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_sleet_storm_spell() {
        let spell = Spell::new(
            "Sleet Storm",
            3,
            SpellSchool::Conjuration,
            "1 action",
            150,
            Components::vsm("A pinch of dust and a few drops of water"),
            "1 minute",
            true,
            false,
            SpellEffect::AreaHazard {
                radius_feet: 40,
                save_ability: Some(Ability::Dexterity),
                condition: Some(crate::rules::conditions::Condition::Prone),
                damage_dice: None,
                damage_type: None,
                description: "".to_string(),
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::AreaHazardCreated { .. }
        ));
    }

    #[test]
    fn test_slow_spell() {
        let spell = Spell::new(
            "Slow",
            3,
            SpellSchool::Transmutation,
            "1 action",
            120,
            Components::vsm("A drop of molasses"),
            "1 minute",
            true,
            false,
            SpellEffect::MultiTargetApplyEffect { max_targets: 6, effect: ActiveEffect::Utility("Target speed halved, -2 to AC and DEX saves, and cannot use reactions. Can only use action or bonus action, not both. Spells might take 2 turns to cast.".to_string()), duration: DurationType::DurationMinutes(1) }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_speak_with_dead_spell() {
        let spell = Spell::new(
            "Speak with Dead",
            3,
            SpellSchool::Necromancy,
            "1 action",
            10,
            Components::vsm("Burning incense"),
            "10 minutes",
            false,
            false,
            SpellEffect::Utility { description: "Grant the semblance of life and intelligence to a corpse of your choice, allowing it to answer up to 5 questions.".to_string() }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_speak_with_plants_spell() {
        let spell = Spell::new(
            "Speak with Plants",
            3,
            SpellSchool::Transmutation,
            "1 action",
            0,
            Components::vs(),
            "10 minutes",
            false,
            false,
            SpellEffect::Utility { description: "Imbue plants within 30 feet of you with limited sentience and animation, giving them the ability to communicate with you and follow simple commands.".to_string() }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::UtilityApplied { .. }
        ));
    }

    #[test]
    fn test_spirit_guardians_spell() {
        let spell = Spell::new(
            "Spirit Guardians",
            3,
            SpellSchool::Conjuration,
            "1 action",
            0,
            Components::vsm("A holy symbol"),
            "10 minutes",
            true,
            false,
            SpellEffect::AreaHazard {
                radius_feet: 15,
                save_ability: Some(Ability::Wisdom),
                condition: None,
                damage_dice: Some("3d8".to_string()),
                damage_type: Some(DamageType::Radiant),
                description: "".to_string(),
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::AreaHazardCreated { .. }
        ));
    }

    #[test]
    fn test_stinking_cloud_spell() {
        let spell = Spell::new(
            "Stinking Cloud",
            3,
            SpellSchool::Conjuration,
            "1 action",
            90,
            Components::vsm("A rotten egg or several skunk cabbage leaves"),
            "1 minute",
            true,
            false,
            SpellEffect::AreaHazard {
                radius_feet: 20,
                save_ability: Some(Ability::Constitution),
                condition: Some(crate::rules::conditions::Condition::Incapacitated),
                damage_dice: None,
                damage_type: None,
                description: "".to_string(),
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::AreaHazardCreated { .. }
        ));
    }

    #[test]
    fn test_tongues_spell() {
        let spell = Spell::new(
            "Tongues",
            3,
            SpellSchool::Divination,
            "1 action",
            0,
            Components::vsm("A small clay model of a ziggurat"),
            "1 hour",
            false,
            false,
            SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target understands any spoken language it hears, and any creature that knows at least one language and can hear the target understands what it says.".to_string()), duration: DurationType::DurationMinutes(60) }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_vampiric_touch_spell() {
        let spell = Spell::new(
            "Vampiric Touch",
            3,
            SpellSchool::Necromancy,
            "1 action",
            0,
            Components::vs(),
            "1 minute",
            true,
            false,
            SpellEffect::SpellAttack {
                damage_dice: "3d6".to_string(),
                damage_type: DamageType::Necrotic,
                damage_scaling: None,
                on_hit_effect: None,
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::AttackRoll { .. }));
    }

    #[test]
    fn test_water_breathing_spell() {
        let spell = Spell::new(
            "Water Breathing",
            3,
            SpellSchool::Transmutation,
            "1 action",
            30,
            Components::vsm("A short reed or piece of straw"),
            "24 hours",
            false,
            false,
            SpellEffect::MultiTargetApplyEffect {
                max_targets: 10,
                effect: ActiveEffect::Utility(
                    "Targets can breathe underwater for the duration.".to_string(),
                ),
                duration: DurationType::DurationMinutes(1440),
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_water_walk_spell() {
        let spell = Spell::new(
            "Water Walk",
            3,
            SpellSchool::Transmutation,
            "1 action",
            30,
            Components::vsm("A piece of cork"),
            "1 hour",
            false,
            false,
            SpellEffect::MultiTargetApplyEffect { max_targets: 10, effect: ActiveEffect::Utility("Targets can move across any liquid surface as if it were harmless solid ground.".to_string()), duration: DurationType::DurationMinutes(60) }
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_wind_wall_spell() {
        let spell = Spell::new(
            "Wind Wall",
            3,
            SpellSchool::Evocation,
            "1 action",
            120,
            Components::vsm("A tiny fan and a feather of exotic origin"),
            "1 minute",
            true,
            false,
            SpellEffect::AreaHazard {
                radius_feet: 50,
                save_ability: Some(Ability::Strength),
                condition: None,
                damage_dice: Some("3d8".to_string()),
                damage_type: Some(DamageType::Bludgeoning),
                description: "".to_string(),
            },
        );
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(3, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::AreaHazardCreated { .. }
        ));
    }
}
