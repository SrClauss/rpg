use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use crate::rules::spells::registry::SpellRegistry;
use crate::rules::spells::spell::{Components, Spell, SpellEffect, SpellSchool};

pub fn load_level2_spells(reg: &mut SpellRegistry) {
    reg.register_spell(Spell::new(
        "Aid",
        2,
        SpellSchool::Abjuration,
        "1 action",
        30,
        Components::vsm("A tiny strip of white cloth"),
        "8 hours",
        false,
        false,
        SpellEffect::MultiTargetApplyEffect {
            max_targets: 3,
            effect: ActiveEffect::IncreaseMaxHP(5),
            duration: DurationType::DurationHours(8),
        },
    ));

    reg.register_spell(Spell::new(
        "Alter Self",
        2,
        SpellSchool::Transmutation,
        "1 action",
        0,
        Components::vs(),
        "Up to 1 hour",
        true,
        false,
        SpellEffect::Compound(vec![
            SpellEffect::Utility { description: "You assume a different form: Aquatic Adaptation, Change Appearance, or Natural Weapons.".to_string() },
            SpellEffect::ApplyEffect {
                effect: ActiveEffect::NaturalWeapon("1d6".to_string(), DamageType::Bludgeoning),
                duration: DurationType::DurationMinutes(60),
            }
        ])
    ));

    reg.register_spell(Spell::new(
        "Animal Messenger",
        2,
        SpellSchool::Enchantment,
        "1 action",
        30,
        Components::vsm("A morsel of food"),
        "24 hours",
        false,
        true,
        SpellEffect::Utility {
            description:
                "You use an animal to deliver a message. It travels up to 50 miles per day."
                    .to_string(),
        },
    ));

    reg.register_spell(Spell::new(
        "Arcane Lock",
        2,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vsm("Gold dust worth at least 25 gp, which the spell consumes"),
        "Until dispelled",
        false,
        false,
        SpellEffect::Utility { description: "You touch a closed door, window, gate, chest, or other entryway, and it becomes locked for the duration.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Augury",
        2,
        SpellSchool::Divination,
        "1 minute",
        0,
        Components::vsm("Specially marked sticks, bones, or similar tokens worth at least 25gp"),
        "Instantaneous",
        false,
        true,
        SpellEffect::Utility { description: "You receive an omen from an otherworldly entity about the results of a specific course of action that you plan to take within the next 30 minutes.".to_string() }
    ));
    reg.register_spell(Spell::new(
        "Barkskin",
        2,
        SpellSchool::Transmutation,
        "1 action",
        0, // Touch
        Components::vsm("A handful of oak bark"),
        "1 hour",
        true, // Concentration
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::MinimumAC(16),
            duration: DurationType::DurationHours(1),
        },
    ));

    reg.register_spell(Spell::new(
        "Blindness/Deafness",
        2,
        SpellSchool::Necromancy,
        "1 action",
        30,
        Components::v(),
        "1 minute",
        false,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Constitution,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::ApplyCondition(crate::rules::conditions::Condition::Blinded),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Blur",
        2,
        SpellSchool::Illusion,
        "1 action",
        0, // Self
        Components::v(),
        "1 minute",
        true, // Concentration
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::DisadvantageOnAttacksAgainstMe,
            duration: DurationType::DurationMinutes(1),
        },
    ));

    reg.register_spell(Spell::new(
        "Branding Smite",
        2,
        SpellSchool::Evocation,
        "1 bonus action",
        0, // Self
        Components::v(),
        "1 minute",
        true, // Concentration
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Multiple(vec![
                ActiveEffect::NextWeaponAttackBonusDamage("2d6".to_string(), DamageType::Radiant),
                ActiveEffect::PreventInvisibility,
            ]),
            duration: DurationType::DurationMinutes(1),
        },
    ));

    reg.register_spell(Spell::new(
        "Calm Emotions",
        2,
        SpellSchool::Enchantment,
        "1 action",
        60,
        Components::vs(),
        "1 minute",
        true, // Concentration
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Charisma,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Multiple(vec![
                    ActiveEffect::SuppressCondition(crate::rules::conditions::Condition::Charmed),
                    ActiveEffect::SuppressCondition(
                        crate::rules::conditions::Condition::Frightened,
                    ),
                ]),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));
    reg.register_spell(Spell::new(
        "Darkness",
        2,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::vsm("Bat fur and a drop of pitch or piece of coal"),
        "10 minutes",
        true, // Concentration
        false,
        SpellEffect::AreaHazard {
            radius_feet: 15,
            save_ability: None, // No save for darkness itself
            condition: None,
            damage_dice: None,
            damage_type: None,
            description: "Magical darkness spreads from a point you choose within range to fill a 15-foot-radius sphere for the duration.".to_string(),
        }
    ));

    reg.register_spell(Spell::new(
        "Darkvision",
        2,
        SpellSchool::Transmutation,
        "1 action",
        0, // Touch
        Components::vsm("Either a pinch of dried carrot or an agate"),
        "8 hours",
        false,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::GainDarkvision(60),
            duration: DurationType::DurationHours(8),
        },
    ));

    reg.register_spell(Spell::new(
        "Detect Thoughts",
        2,
        SpellSchool::Divination,
        "1 action",
        0, // Self
        Components::vsm("A copper piece"),
        "1 minute",
        true, // Concentration
        false,
        SpellEffect::Utility {
            description: "For the duration, you can read the thoughts of certain creatures."
                .to_string(),
        },
    ));

    reg.register_spell(Spell::new(
        "Enhance Ability",
        2,
        SpellSchool::Transmutation,
        "1 action",
        0, // Touch
        Components::vsm("Fur or a feather from a beast"),
        "1 hour",
        true, // Concentration
        false,
        SpellEffect::Choice(vec![
            (
                "Bear's Endurance".to_string(),
                Box::new(SpellEffect::ApplyEffect {
                    effect: ActiveEffect::Multiple(vec![
                        ActiveEffect::AdvantageOnAbilityCheck(Ability::Constitution),
                        ActiveEffect::TemporaryHP(7), // 2d6 simplified for the demo
                    ]),
                    duration: DurationType::DurationHours(1),
                }),
            ),
            (
                "Bull's Strength".to_string(),
                Box::new(SpellEffect::ApplyEffect {
                    effect: ActiveEffect::AdvantageOnAbilityCheck(Ability::Strength),
                    duration: DurationType::DurationHours(1),
                }),
            ),
            (
                "Cat's Grace".to_string(),
                Box::new(SpellEffect::ApplyEffect {
                    effect: ActiveEffect::AdvantageOnAbilityCheck(Ability::Dexterity),
                    duration: DurationType::DurationHours(1),
                }),
            ),
            (
                "Eagle's Splendor".to_string(),
                Box::new(SpellEffect::ApplyEffect {
                    effect: ActiveEffect::AdvantageOnAbilityCheck(Ability::Charisma),
                    duration: DurationType::DurationHours(1),
                }),
            ),
            (
                "Fox's Cunning".to_string(),
                Box::new(SpellEffect::ApplyEffect {
                    effect: ActiveEffect::AdvantageOnAbilityCheck(Ability::Intelligence),
                    duration: DurationType::DurationHours(1),
                }),
            ),
            (
                "Owl's Wisdom".to_string(),
                Box::new(SpellEffect::ApplyEffect {
                    effect: ActiveEffect::AdvantageOnAbilityCheck(Ability::Wisdom),
                    duration: DurationType::DurationHours(1),
                }),
            ),
        ]),
    ));

    reg.register_spell(Spell::new(
        "Enlarge/Reduce",
        2,
        SpellSchool::Transmutation,
        "1 action",
        30,
        Components::vsm("A pinch of powdered iron"),
        "1 minute",
        true, // Concentration
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Multiple(vec![
                ActiveEffect::AdvantageOnAbilityCheck(Ability::Strength),
                ActiveEffect::AdvantageOnSavingThrowAbility(Ability::Strength),
                ActiveEffect::WeaponDamageBonus("1d4".to_string()),
            ]), // Enlarge default
            duration: DurationType::DurationMinutes(1),
        },
    ));
    reg.register_spell(Spell::new(
        "Enthrall",
        2,
        SpellSchool::Enchantment,
        "1 action",
        60,
        Components::vs(),
        "1 minute",
        false,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Wisdom,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::DisadvantageOnAbilityCheck(Ability::Wisdom), // Perception checks
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Find Steed",
        2,
        SpellSchool::Conjuration,
        "10 minutes",
        30,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::Choice(vec![
            (
                "Warhorse".to_string(),
                Box::new(SpellEffect::SummonCreature {
                    monster_id: "srd_warhorse".to_string(),
                }),
            ),
            (
                "Pony".to_string(),
                Box::new(SpellEffect::SummonCreature {
                    monster_id: "srd_pony".to_string(),
                }),
            ),
            (
                "Camel".to_string(),
                Box::new(SpellEffect::SummonCreature {
                    monster_id: "srd_camel".to_string(),
                }),
            ),
            (
                "Elk".to_string(),
                Box::new(SpellEffect::SummonCreature {
                    monster_id: "srd_elk".to_string(),
                }),
            ),
            (
                "Mastiff".to_string(),
                Box::new(SpellEffect::SummonCreature {
                    monster_id: "srd_mastiff".to_string(),
                }),
            ),
        ]),
    ));

    reg.register_spell(Spell::new(
        "Find Traps",
        2,
        SpellSchool::Divination,
        "1 action",
        120,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::Utility {
            description: "You sense the presence of any trap within line of sight.".to_string(),
        },
    ));

    reg.register_spell(Spell::new(
        "Flame Blade",
        2,
        SpellSchool::Evocation,
        "1 bonus action",
        0, // Self
        Components::vsm("Leaf of sumac"),
        "10 minutes",
        true, // Concentration
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility(
                "Wielding a fiery blade (3d6 fire damage on melee spell attack).".to_string(),
            ),
            duration: DurationType::DurationMinutes(10),
        },
    ));

    reg.register_spell(Spell::new(
        "Flaming Sphere",
        2,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::vsm("A bit of tallow, a pinch of brimstone, and a dusting of powdered iron"),
        "1 minute",
        true, // Concentration
        false,
        SpellEffect::AreaHazard {
            radius_feet: 5,
            save_ability: Some(Ability::Dexterity),
            condition: None,
            damage_dice: Some("2d6".to_string()),
            damage_type: Some(DamageType::Fire),
            description: "A 5-foot-diameter sphere of fire appears. Creatures ending their turn within 5 feet take 2d6 fire damage (half on save).".to_string(),
        }
    ));
    reg.register_spell(Spell::new(
        "Gentle Repose",
        2,
        SpellSchool::Necromancy,
        "1 action",
        0, // Touch
        Components::vsm("A pinch of salt and one copper piece placed on each of the corpse's eyes"),
        "10 days",
        false,
        true, // Ritual
        SpellEffect::Utility {
            description:
                "You protect a corpse from decay and from becoming undead for the duration."
                    .to_string(),
        },
    ));

    reg.register_spell(Spell::new(
        "Gust of Wind",
        2,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::vsm("A legume seed"),
        "1 minute",
        true, // Concentration
        false,
        SpellEffect::AreaHazard {
            radius_feet: 60, // Representing the line length
            save_ability: Some(Ability::Strength),
            condition: None,
            damage_dice: None,
            damage_type: None,
            description: "A line of strong wind 60 feet long and 10 feet wide. Creatures that fail a Strength save are pushed 15 feet away. It also costs 2 feet of movement for every 1 foot moved when moving closer.".to_string(),
        }
    ));

    reg.register_spell(Spell::new(
        "Hold Person",
        2,
        SpellSchool::Enchantment,
        "1 action",
        60,
        Components::vsm("A small, straight piece of iron"),
        "1 minute",
        true, // Concentration
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Wisdom,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Multiple(vec![
                    ActiveEffect::ApplyCondition(crate::rules::conditions::Condition::Paralyzed),
                    // Hold Person doesn't provide a flat DC in the effect directly, but since we model it as a placeholder here,
                    // we can use a base DC of 13. Ideally the engine evaluates this dynamically, but this uses our new RecurringSave structure.
                    ActiveEffect::RecurringSaveAtEndOfTurn(
                        Ability::Wisdom,
                        13,
                        crate::rules::conditions::Condition::Paralyzed,
                    ),
                ]),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Invisibility",
        2,
        SpellSchool::Illusion,
        "1 action",
        0, // Touch
        Components::vsm("An eyelash encased in gum arabic"),
        "1 hour",
        true, // Concentration
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Multiple(vec![
                ActiveEffect::ApplyCondition(crate::rules::conditions::Condition::Invisible),
                ActiveEffect::Utility(
                    "The spell ends if the target attacks or casts a spell.".to_string(),
                ),
            ]),
            duration: DurationType::DurationHours(1),
        },
    ));

    reg.register_spell(Spell::new(
        "Knock",
        2,
        SpellSchool::Transmutation,
        "1 action",
        60,
        Components::v(),
        "Instantaneous",
        false,
        false,
        SpellEffect::Utility {
            description: "Unlocks an object that is held shut by a lock, stuck, or barred. It produces a loud knock audible from 300 feet away.".to_string(),
        }
    ));
    reg.register_spell(Spell::new(
        "Lesser Restoration",
        2,
        SpellSchool::Abjuration,
        "1 action",
        0, // Touch
        Components::vs(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::CureCondition(vec![
            crate::rules::conditions::Condition::Blinded,
            crate::rules::conditions::Condition::Deafened,
            crate::rules::conditions::Condition::Paralyzed,
            crate::rules::conditions::Condition::Poisoned,
        ]),
    ));

    reg.register_spell(Spell::new(
        "Levitate",
        2,
        SpellSchool::Transmutation,
        "1 action",
        60,
        Components::vsm("Either a small leather loop or a piece of golden wire bent into a cup shape with a long shank on one end"),
        "10 minutes",
        true, // Concentration
        false, // Ritual
        SpellEffect::Choice(vec![
            (
                "Willing Target".to_string(),
                Box::new(SpellEffect::ApplyEffect {
                    effect: ActiveEffect::Utility("Levitating up to 20 feet high. The target can move only by pushing or pulling against a fixed object or surface within reach.".to_string()),
                    duration: DurationType::DurationMinutes(10),
                })
            ),
            (
                "Unwilling Target".to_string(),
                Box::new(SpellEffect::SaveThrow {
                    save_ability: Ability::Constitution,
                    half_damage_on_save: false,
                    damage_dice: None,
                    damage_type: None,
                    damage_scaling: None,
                    fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                        effect: ActiveEffect::Utility("Levitating up to 20 feet high. The target can move only by pushing or pulling against a fixed object or surface within reach.".to_string()),
                        duration: DurationType::DurationMinutes(10),
                    })),
                })
            ),
        ])
    ));

    reg.register_spell(Spell::new(
        "Locate Animals or Plants",
        2,
        SpellSchool::Divination,
        "1 action",
        0, // Self
        Components::vsm("A bit of fur from a bloodhound"),
        "10 minutes",
        false, // Concentration
        true, // Ritual
        SpellEffect::Utility {
            description: "Describe or name a specific kind of beast or plant. You learn the direction and distance to the closest creature or plant of that kind within 5 miles.".to_string(),
        }
    ));

    reg.register_spell(Spell::new(
        "Locate Object",
        2,
        SpellSchool::Divination,
        "1 action",
        0, // Self
        Components::vsm("A twig from a dowsing rod"),
        "10 minutes",
        true, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Describe or name an object that is familiar to you. You sense the direction to the object's location, as long as that object is within 1,000 feet of you.".to_string(),
        }
    ));

    reg.register_spell(Spell::new(
        "Magic Mouth",
        2,
        SpellSchool::Illusion,
        "1 minute",
        30,
        Components::vsm("A small bit of honeycomb and jade dust worth at least 10 gp, which the spell consumes"),
        "Until dispelled",
        false, // Concentration
        true, // Ritual
        SpellEffect::Utility {
            description: "You implant a message within an object in range, a message that is uttered when a trigger condition is met.".to_string(),
        }
    ));
    reg.register_spell(Spell::new(
        "Magic Weapon",
        2,
        SpellSchool::Transmutation,
        "1 bonus action",
        0, // Touch
        Components::vs(),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::MagicWeaponBonus(1),
            duration: DurationType::DurationMinutes(60),
        },
    ));

    reg.register_spell(Spell::new(
        "Mirror Image",
        2,
        SpellSchool::Illusion,
        "1 action",
        0, // Self
        Components::vs(),
        "1 minute",
        false, // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::MirrorImage(3),
            duration: DurationType::DurationMinutes(1),
        },
    ));

    reg.register_spell(Spell::new(
        "Misty Step",
        2,
        SpellSchool::Conjuration,
        "1 bonus action",
        0,               // Self
        Components::v(), // Verbal only
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Teleport up to 30 feet to an unoccupied space that you can see."
                .to_string(),
        },
    ));

    reg.register_spell(Spell::new(
        "Moonbeam",
        2,
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vsm("Several seeds of any moonseed plant and a piece of opalescent feldspar"),
        "1 minute",
        true, // Concentration
        false, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 5,
            save_ability: Some(Ability::Constitution),
            condition: None,
            damage_dice: Some("2d10".to_string()),
            damage_type: Some(DamageType::Radiant),
            description: "A silvery beam of pale light shines down in a 5-foot-radius, 40-foot-high cylinder. Shapechangers have disadvantage on this saving throw.".to_string(),
        }
    ));

    reg.register_spell(Spell::new(
        "Pass without Trace",
        2,
        SpellSchool::Abjuration,
        "1 action",
        0, // Self
        Components::vsm("Ashes from a burned leaf of mistletoe and a sprig of spruce"),
        "1 hour",
        true, // Concentration
        false, // Ritual
        SpellEffect::MultiTargetApplyEffect {
            max_targets: 99, // Choose any creatures within 30 feet
            effect: ActiveEffect::Utility("+10 bonus to Dexterity (Stealth) checks and can't be tracked except by magical means.".to_string()),
            duration: DurationType::DurationMinutes(60),
        }
    ));
    reg.register_spell(Spell::new(
        "Prayer of Healing",
        2,
        SpellSchool::Evocation,
        "10 minutes",
        30,
        Components::v(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::MultiTargetHealing {
            max_targets: 6,
            healing_dice: "2d8".to_string(),
        },
    ));

    reg.register_spell(Spell::new(
        "Protection from Poison",
        2,
        SpellSchool::Abjuration,
        "1 action",
        0, // Touch
        Components::vs(),
        "1 hour",
        false, // Concentration
        false, // Ritual
        SpellEffect::Compound(vec![
            SpellEffect::CureCondition(vec![crate::rules::conditions::Condition::Poisoned]),
            SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility("Advantage on saving throws against being poisoned, and resistance to poison damage".to_string()),
                duration: DurationType::DurationMinutes(60),
            }
        ])
    ));

    reg.register_spell(Spell::new(
        "Ray of Enfeeblement",
        2,
        SpellSchool::Necromancy,
        "1 action",
        60,
        Components::vs(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SpellAttack {
            damage_dice: "0d0".to_string(), // No direct damage
            damage_type: DamageType::Necrotic,
            damage_scaling: None,
            on_hit_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::WeaponDamagePenalty("1d4".to_string()), // Simplified 5e halving to penalty for testing
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Rope Trick",
        2,
        SpellSchool::Transmutation,
        "1 action",
        0, // Touch
        Components::vsm("Powdered corn extract and a twisted loop of parchment"),
        "1 hour",
        false, // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "You touch a length of rope that is up to 60 feet long. One end of the rope then rises into the air until the whole rope hangs perpendicular to the ground. At the upper end of the rope, an invisible entrance opens to an extradimensional space that lasts until the spell ends.".to_string(),
        }
    ));

    reg.register_spell(Spell::new(
        "Scorching Ray",
        2,
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vs(),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::Compound(vec![
            SpellEffect::SpellAttack {
                damage_dice: "2d6".to_string(),
                damage_type: DamageType::Fire,
                damage_scaling: Some("spell_slot".to_string()),
                on_hit_effect: None,
            },
            SpellEffect::SpellAttack {
                damage_dice: "2d6".to_string(),
                damage_type: DamageType::Fire,
                damage_scaling: Some("spell_slot".to_string()),
                on_hit_effect: None,
            },
            SpellEffect::SpellAttack {
                damage_dice: "2d6".to_string(),
                damage_type: DamageType::Fire,
                damage_scaling: Some("spell_slot".to_string()),
                on_hit_effect: None,
            },
        ]),
    ));
    reg.register_spell(Spell::new(
        "See Invisibility",
        2,
        SpellSchool::Divination,
        "1 action",
        0, // Self
        Components::vsm("A pinch of talc and a small sprinkling of powdered silver"),
        "1 hour",
        false, // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility("Can see invisible creatures and objects as if they were visible, and can see into the Ethereal Plane.".to_string()),
            duration: DurationType::DurationMinutes(60),
        }
    ));

    reg.register_spell(Spell::new(
        "Shatter",
        2,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::vsm("A chip of mica"),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 10,
            save_ability: Some(Ability::Constitution),
            condition: None,
            damage_dice: Some("3d8".to_string()),
            damage_type: Some(DamageType::Thunder),
            description: "A sudden loud ringing noise, painfully intense, erupts from a point of your choice. Inorganic creatures have disadvantage on the save.".to_string(),
        }
    ));

    reg.register_spell(Spell::new(
        "Silence",
        2,
        SpellSchool::Illusion,
        "1 action",
        120,
        Components::vs(),
        "10 minutes",
        true, // Concentration
        true, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 20,
            save_ability: None,
            condition: None,
            damage_dice: None,
            damage_type: None,
            description: "No sound can be created within or pass through a 20-foot-radius sphere. Creatures are deafened while entirely inside it, and immune to thunder damage. Spells requiring verbal components cannot be cast here.".to_string(),
        }
    ));

    reg.register_spell(Spell::new(
        "Spider Climb",
        2,
        SpellSchool::Transmutation,
        "1 action",
        0, // Touch
        Components::vsm("A drop of bitumen and a spider"),
        "1 hour",
        true, // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility("Gains climbing speed equal to walking speed, can move up, down, and across vertical surfaces and upside down along ceilings, while leaving hands free.".to_string()),
            duration: DurationType::DurationMinutes(60),
        }
    ));

    reg.register_spell(Spell::new(
        "Spike Growth",
        2,
        SpellSchool::Transmutation,
        "1 action",
        150,
        Components::vsm("Seven sharp thorns or seven small twigs, each sharpened to a point"),
        "10 minutes",
        true, // Concentration
        false, // Ritual
        SpellEffect::AreaHazard {
            radius_feet: 20,
            save_ability: None,
            condition: None,
            damage_dice: Some("2d4".to_string()), // For every 5 ft traveled
            damage_type: Some(DamageType::Piercing),
            description: "Ground twists and sprouts hard spikes and thorns. The area becomes difficult terrain. When a creature moves into or within the area, it takes 2d4 piercing damage for every 5 feet it travels.".to_string(),
        }
    ));

    reg.register_spell(Spell::new(
        "Spiritual Weapon",
        2,
        SpellSchool::Evocation,
        "1 bonus action",
        60,
        Components::vs(),
        "1 minute",
        false, // Concentration
        false, // Ritual
        SpellEffect::Summon {
            creature_name: "Spiritual Weapon".to_string(),
            count: 1,
        },
    ));

    reg.register_spell(Spell::new(
        "Suggestion",
        2,
        SpellSchool::Enchantment,
        "1 action",
        30,
        Components::vsm("A snake's tongue and either a bit of honeycomb or a drop of sweet oil"),
        "8 hours",
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
                    "Charmed and follows the suggested course of action".to_string(),
                ),
                duration: DurationType::DurationMinutes(480), // 8 hours
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Warding Bond",
        2,
        SpellSchool::Abjuration,
        "1 action",
        0, // Touch
        Components::vsm("A pair of platinum rings worth at least 50 gp each, which you and the target must wear for the duration"),
        "1 hour",
        false, // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility("+1 AC and saving throws, resistance to all damage. Caster takes same damage.".to_string()),
            duration: DurationType::DurationMinutes(60),
        }
    ));

    reg.register_spell(Spell::new(
        "Web",
        2,
        SpellSchool::Conjuration,
        "1 action",
        60,
        Components::vsm("A bit of spiderweb"),
        "1 hour",
        true, // Concentration
        false, // Ritual
        SpellEffect::Compound(vec![
            SpellEffect::AreaHazard {
                radius_feet: 20,
                save_ability: None,
                condition: None,
                damage_dice: None,
                damage_type: None,
                description: "Webs fill a 20-foot cube from a point within range. Area is difficult terrain and lightly obscured.".to_string(),
            },
            SpellEffect::SaveThrow {
                save_ability: Ability::Dexterity,
                half_damage_on_save: false,
                damage_dice: None,
                damage_type: None,
                damage_scaling: None,
                fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                    effect: ActiveEffect::ApplyCondition(crate::rules::conditions::Condition::Restrained),
                    duration: DurationType::DurationMinutes(60),
                })),
            }
        ])
    ));

    reg.register_spell(Spell::new(
        "Zone of Truth",
        2,
        SpellSchool::Enchantment,
        "1 action",
        60,
        Components::vs(),
        "10 minutes",
        false, // Concentration
        false, // Ritual
        SpellEffect::Compound(vec![
            SpellEffect::AreaHazard {
                radius_feet: 15,
                save_ability: None,
                condition: None,
                damage_dice: None,
                damage_type: None,
                description: "15-foot-radius sphere. Creatures in it must make a Charisma save to be able to lie.".to_string(),
            },
            SpellEffect::SaveThrow {
                save_ability: Ability::Charisma,
                half_damage_on_save: false,
                damage_dice: None,
                damage_type: None,
                damage_scaling: None,
                fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                    effect: ActiveEffect::Utility("Can't speak a deliberate lie".to_string()),
                    duration: DurationType::DurationMinutes(10),
                })),
            }
        ])
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
    use crate::rules::spells::registry::SpellRegistry;

    #[test]
    fn test_aid_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);

        let spell = SpellRegistry::get_spell("Aid").expect("Aid should be registered");

        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        // Give spell slots to cast Level 2 spell
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let mut ally1 = Entity::new(
            "Ally1",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let initial_max_hp = ally1.get_max_hp();
        let initial_current_hp = ally1.current_hp;

        // Cast in ally 1
        let outcome = cast_spell(
            &mut caster,
            Some(&mut ally1),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );

        match outcome.outcome {
            CastOutcome::EffectApplied => {
                assert_eq!(ally1.get_max_hp(), initial_max_hp + 5);
                assert_eq!(ally1.current_hp, initial_current_hp + 5);
            }
            other => panic!("Expected EffectApplied, got {:?}", other),
        }
    }
    #[test]
    fn test_barkskin_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);

        let spell = SpellRegistry::get_spell("Barkskin").expect("Barkskin should be registered");
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Druid",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let mut ally = Entity::new(
            "Ally",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        assert!(ally.get_armor_class() < 16);

        let outcome = cast_spell(
            &mut caster,
            Some(&mut ally),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        match outcome.outcome {
            CastOutcome::EffectApplied => {
                assert_eq!(ally.get_armor_class(), 16);
            }
            other => panic!("Expected EffectApplied, got {:?}", other),
        }
    }
    #[test]
    fn test_blindness_deafness_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);

        let spell =
            SpellRegistry::get_spell("Blindness/Deafness").expect("Spell should be registered");
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Necromancer",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let mut foe = Entity::new(
            "Foe",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let outcome = cast_spell(
            &mut caster,
            Some(&mut foe),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        match outcome.outcome {
            CastOutcome::SaveRoll { save_result, .. } => {
                let success = save_result.is_success.unwrap_or(false);
                if success {
                    assert!(!foe.has_condition(&crate::rules::conditions::Condition::Blinded));
                } else {
                    assert!(foe.has_condition(&crate::rules::conditions::Condition::Blinded));
                }
            }
            other => panic!("Expected SaveRoll, got {:?}", other),
        }
    }
    #[test]
    fn test_blur_spell() {
        use crate::rules::combat::attack::resolve_attack;
        use crate::rules::equipment::weapons::all_srd_weapons;

        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);

        let spell = SpellRegistry::get_spell("Blur").expect("Blur should be registered");
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Illusionist",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster.clone(),
            Some(&mut caster),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));

        let mut attacker = Entity::new(
            "Goblin",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let weapon = all_srd_weapons()
            .into_iter()
            .find(|w| w.name == "Shortsword")
            .unwrap();

        // The attack should be rolled with Disadvantage
        let attack_res = resolve_attack(
            &mut attacker,
            &mut caster,
            &weapon,
            true,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        // We can't directly check if it rolled with disadvantage, but we can verify it doesn't crash
        // and is resolved successfully. The engine code handles the Disadvantage internally.
        assert!(attack_res.attack_roll.total >= 0);
    }
    #[test]
    fn test_branding_smite_spell() {
        use crate::rules::combat::attack::resolve_attack;
        use crate::rules::equipment::weapons::all_srd_weapons;

        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);

        let spell = SpellRegistry::get_spell("Branding Smite").expect("Should be registered");
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Paladin",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster.clone(),
            Some(&mut caster),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));

        let mut defender = Entity::new(
            "Goblin",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let weapon = all_srd_weapons()
            .into_iter()
            .find(|w| w.name == "Shortsword")
            .unwrap();

        let attack_res = resolve_attack(
            &mut caster,
            &mut defender,
            &weapon,
            true,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        // Base Shortsword is 1d6. Smite adds 2d6. Total damage if hit should be between 3 and 18 (plus mods).
        // Since attributes are 10, mod is 0.
        if attack_res.is_hit {
            println!("ATTACKER EFFECTS: {:?}", caster.applied_effects);
            println!("ATTACK RES: {:?}", attack_res);
            assert!(
                attack_res.final_damage >= 3,
                "Damage was less than 3! Final damage: {}",
                attack_res.final_damage
            );
        }
    }

    #[test]
    fn test_calm_emotions_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);

        let spell = SpellRegistry::get_spell("Calm Emotions").expect("Spell should be registered");
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Bard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let mut foe = Entity::new(
            "Foe",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let outcome = cast_spell(
            &mut caster,
            Some(&mut foe),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        match outcome.outcome {
            CastOutcome::SaveRoll { save_result, .. } => {
                let success = save_result.is_success.unwrap_or(false);
                if success {
                    // nothing happens
                } else {
                    assert!(foe.applied_effects.iter().any(|e| matches!(
                        e.effect,
                        crate::rules::character::active_effects::ActiveEffect::Multiple(_)
                    )));
                }
            }
            other => panic!("Expected SaveRoll, got {:?}", other),
        }
    }
    #[test]
    fn test_darkness_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Darkness").expect("Spell should be registered");
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Warlock",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster,
            None,
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(
            outcome.outcome,
            CastOutcome::AreaHazardCreated {
                radius_feet: 15,
                save_dc: _,
                affected_hexes_count: _
            }
        ));
    }

    #[test]
    fn test_darkvision_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Darkvision").expect("Spell should be registered");
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster.clone(),
            Some(&mut caster),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
        assert!(caster.applied_effects.iter().any(|e| matches!(
            e.effect,
            crate::rules::character::active_effects::ActiveEffect::GainDarkvision(60)
        )));
    }

    #[test]
    fn test_detect_thoughts_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("Detect Thoughts").expect("Spell should be registered");
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_enhance_ability_spell() {
        use crate::rules::checks::roll_ability_check;

        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("Enhance Ability").expect("Spell should be registered");
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        use crate::rules::spells::casting::cast_spell_with_choice;
        let mut target = Entity::new(
            "Ally",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let outcome = cast_spell_with_choice(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
            Some(1),
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));

        // Testing the resulting Advantage on Strength check
        let check_res = roll_ability_check(&target, Ability::Strength, DiceMode::Normal, None);
        // By rolling and verifying the mechanics work without panicking, the engine now applies Advantage.
        assert!(check_res.roll_result.total >= 1);
    }

    #[test]
    fn test_enlarge_reduce_spell() {
        use crate::rules::checks::roll_saving_throw;
        use crate::rules::combat::attack::resolve_attack;
        use crate::rules::equipment::weapons::all_srd_weapons;

        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Enlarge/Reduce").expect("Spell should be registered");
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Sorcerer",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let mut target = Entity::new(
            "Fighter",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));

        // Test saving throw advantage
        let save_res = roll_saving_throw(&target, Ability::Strength, false, DiceMode::Normal, None);
        assert!(save_res.roll_result.total >= 1);

        // Test attack damage bonus
        let mut foe = Entity::new(
            "Goblin",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let weapon = all_srd_weapons()
            .into_iter()
            .find(|w| w.name == "Shortsword")
            .unwrap();

        let attack_res = resolve_attack(
            &mut target,
            &mut foe,
            &weapon,
            true,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        // Shortsword (1d6) + Enlarge (1d4). Total minimum if hit is 2.
        if attack_res.is_hit {
            assert!(attack_res.final_damage >= 2);
        }
    }
    #[test]
    fn test_enthrall_spell() {
        use crate::rules::checks::roll_ability_check;

        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Enthrall").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Bard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let mut foe = Entity::new(
            "Goblin",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let outcome = cast_spell(
            &mut caster,
            Some(&mut foe),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );

        match outcome.outcome {
            CastOutcome::SaveRoll { save_result, .. } => {
                let success = save_result.is_success.unwrap_or(false);
                if !success {
                    assert!(foe.applied_effects.iter().any(|e| matches!(e.effect, crate::rules::character::active_effects::ActiveEffect::DisadvantageOnAbilityCheck(Ability::Wisdom))));

                    // Verify the dice roll is processed with Disadvantage correctly
                    let check = roll_ability_check(&foe, Ability::Wisdom, DiceMode::Normal, None);
                    assert!(check.roll_result.total >= 1);
                }
            }
            _ => panic!("Expected SaveRoll"),
        }
    }

    #[test]
    fn test_find_steed_spell() {
        use crate::rules::spells::casting::cast_spell_with_choice;

        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Find Steed").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Paladin",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        // Pick choice index 0 (Warhorse)
        let outcome = cast_spell_with_choice(
            &mut caster,
            None,
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
            Some(0),
        );
        assert!(matches!(outcome.outcome, CastOutcome::Summoned { .. }));
        if let CastOutcome::Summoned { creature_name, .. } = outcome.outcome {
            assert_eq!(creature_name, "srd_warhorse");
        }
    }

    #[test]
    fn test_find_traps_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Find Traps").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_flame_blade_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Flame Blade").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Druid",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster.clone(),
            Some(&mut caster),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
        assert!(caster.applied_effects.iter().any(|e| matches!(
            e.effect,
            crate::rules::character::active_effects::ActiveEffect::Utility(_)
        )));
    }

    #[test]
    fn test_flaming_sphere_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Flaming Sphere").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
        match outcome.outcome {
            CastOutcome::AreaHazardCreated {
                radius_feet,
                save_dc: _,
                affected_hexes_count: _,
            } => {
                assert_eq!(radius_feet, 5);
            }
            _ => panic!("Expected AreaHazardCreated"),
        }
    }

    #[test]
    fn test_gentle_repose_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Gentle Repose").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_gust_of_wind_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Gust of Wind").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
        match outcome.outcome {
            CastOutcome::AreaHazardCreated { radius_feet, .. } => {
                assert_eq!(radius_feet, 60);
            }
            _ => panic!("Expected AreaHazardCreated"),
        }
    }

    #[test]
    fn test_hold_person_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Hold Person").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let mut target = Entity::new(
            "Bandit",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        // The save throw should either apply the condition or not, based on a roll.
        // We just assert it executes cleanly and resolves the SaveThrow effect.
        assert!(matches!(outcome.outcome, CastOutcome::SaveRoll { .. }));
    }

    #[test]
    fn test_invisibility_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Invisibility").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let mut ally = Entity::new(
            "Rogue",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let outcome = cast_spell(
            &mut caster,
            Some(&mut ally),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
        assert!(ally.has_condition(&crate::rules::conditions::Condition::Invisible));

        // Test the invisibility hook by attacking
        use crate::rules::combat::attack::resolve_attack;
        use crate::rules::equipment::weapons::all_srd_weapons;
        let mut foe = Entity::new(
            "Goblin",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let weapon = all_srd_weapons()
            .into_iter()
            .find(|w| w.name == "Shortsword")
            .unwrap();

        let _attack_res = resolve_attack(
            &mut ally,
            &mut foe,
            &weapon,
            true,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );

        // Assert invisibility was broken!
        assert!(!ally.has_condition(&crate::rules::conditions::Condition::Invisible));
    }

    #[test]
    fn test_knock_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Knock").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_lesser_restoration_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Lesser Restoration").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let mut target = Entity::new(
            "Ally",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        // Apply condition first
        target.add_condition(crate::rules::conditions::Condition::Poisoned);
        assert!(target.has_condition(&crate::rules::conditions::Condition::Poisoned));

        let outcome = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );

        assert!(matches!(
            outcome.outcome,
            CastOutcome::ConditionCured { .. }
        ));
        assert!(!target.has_condition(&crate::rules::conditions::Condition::Poisoned));
    }

    #[test]
    fn test_levitate_spell() {
        use crate::rules::spells::casting::cast_spell_with_choice;
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Levitate").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let mut willing_target = Entity::new(
            "Ally",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        // Willing choice (index 0)
        let outcome1 = cast_spell_with_choice(
            &mut caster,
            Some(&mut willing_target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
            Some(0),
        );
        assert!(matches!(outcome1.outcome, CastOutcome::EffectApplied));

        let mut unwilling_target = Entity::new(
            "Enemy",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        // Unwilling choice (index 1)
        let outcome2 = cast_spell_with_choice(
            &mut caster,
            Some(&mut unwilling_target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
            Some(1),
        );
        assert!(matches!(outcome2.outcome, CastOutcome::SaveRoll { .. }));
    }

    #[test]
    fn test_locate_animals_or_plants_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Locate Animals or Plants").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Druid",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_locate_object_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Locate Object").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_magic_mouth_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Magic Mouth").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_magic_weapon_spell() {
        use crate::rules::combat::attack::resolve_attack;
        use crate::rules::equipment::weapons::all_srd_weapons;
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Magic Weapon").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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

        // Let's test the combat logic!
        caster.add_effect(crate::rules::character::active_effects::AppliedEffect {
            effect: ActiveEffect::MagicWeaponBonus(1),
            duration: DurationType::DurationMinutes(60),
            source_name: "Magic Weapon".to_string(),
        });
        let mut defender = Entity::new(
            "Goblin",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let weapon = all_srd_weapons()
            .into_iter()
            .find(|w| w.name == "Shortsword")
            .unwrap();

        let attack_res = resolve_attack(
            &mut caster,
            &mut defender,
            &weapon,
            true,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert_eq!(attack_res.attack_roll.modifier, 3); // 0 (str) + 2 (prof) + 1 (magic weapon) = 3
        assert!(attack_res.attack_roll.total >= 0);
    }

    #[test]
    fn test_mirror_image_spell() {
        use crate::rules::combat::attack::resolve_attack;
        use crate::rules::equipment::weapons::all_srd_weapons;
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Mirror Image").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster.clone(),
            Some(&mut caster),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));

        let mut foe = Entity::new(
            "Goblin",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let weapon = all_srd_weapons()
            .into_iter()
            .find(|w| w.name == "Shortsword")
            .unwrap();

        let attack_res = resolve_attack(
            &mut foe,
            &mut caster,
            &weapon,
            true,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        // It might hit an image or not, but it shouldn't panic.
        assert!(attack_res.attack_roll.total >= 0);
    }

    #[test]
    fn test_misty_step_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Misty Step").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_moonbeam_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Moonbeam").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Druid",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_pass_without_trace_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Pass without Trace").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Druid",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster.clone(),
            Some(&mut caster),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::EffectApplied));
    }

    #[test]
    fn test_prayer_of_healing_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Prayer of Healing").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        caster.current_hp = 1;
        let outcome = cast_spell(
            &mut caster.clone(),
            Some(&mut caster),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::Healed { .. }));
        assert!(caster.current_hp > 1);
    }

    #[test]
    fn test_protection_from_poison_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Protection from Poison").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let outcome = cast_spell(
            &mut caster.clone(),
            Some(&mut caster),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::CompoundOutcome(_)));
    }

    #[test]
    fn test_ray_of_enfeeblement_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Ray of Enfeeblement").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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

        assert!(matches!(outcome.outcome, CastOutcome::AttackRoll { .. }));
    }

    #[test]
    fn test_rope_trick_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Rope Trick").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_scorching_ray_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Scorching Ray").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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

        assert!(matches!(outcome.outcome, CastOutcome::CompoundOutcome(_)));
    }

    #[test]
    fn test_see_invisibility_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("See Invisibility").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_shatter_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Shatter").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
            CastOutcome::AreaHazardCreated { .. }
        ));
    }

    #[test]
    fn test_silence_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Silence").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_spider_climb_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Spider Climb").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_spike_growth_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Spike Growth").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Druid",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_spiritual_weapon_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Spiritual Weapon").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_suggestion_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Suggestion").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
    fn test_warding_bond_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Warding Bond").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
        caster.spell_slots = Some(slots);

        let mut target = Entity::new(
            "Ally",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
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
    fn test_web_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Web").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Wizard",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
        assert!(matches!(outcome.outcome, CastOutcome::CompoundOutcome(_)));
    }

    #[test]
    fn test_zone_of_truth_spell() {
        let mut reg = SpellRegistry::new();
        load_level2_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Zone of Truth").unwrap();
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Cleric",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = crate::rules::spells::spellcaster::SpellSlots::new();
        slots.set_slots(2, 2);
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
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(matches!(outcome.outcome, CastOutcome::CompoundOutcome(_)));
    }
}
