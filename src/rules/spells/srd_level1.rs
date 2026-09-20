use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use crate::rules::spells::registry::SpellRegistry;
use crate::rules::spells::spell::{Components, Spell, SpellEffect, SpellSchool};

pub fn load_level1_spells(reg: &mut SpellRegistry) {
    reg.register_spell(Spell::new(
        "Alarm",
        1,
        SpellSchool::Abjuration,
        "1 minute",
        30,
        Components::vsm("A tiny bell and a piece of fine silver wire"),
        "8 hours",
        false,
        true,
        SpellEffect::Utility { description: "Coloca um alarme contra intrusos em uma área cúbica de 20 pés. Até 8 horas. Emite som de sino ou alerta mental se uma criatura Minúscula ou maior entrar na área (ignorando criaturas designadas).".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Armor of Agathys",
        1,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vsm("A cup of water"),
        "1 hour",
        false,
        false,
        SpellEffect::Compound(vec![
            SpellEffect::ApplyEffect {
                effect: ActiveEffect::TemporaryHP(5),
                duration: DurationType::DurationHours(1),
            },
            SpellEffect::ApplyEffect {
                effect: ActiveEffect::RetaliateMelee("5".to_string(), DamageType::Cold),
                duration: DurationType::DurationHours(1),
            },
        ]),
    ));

    reg.register_spell(Spell::new(
        "Arms of Hadar",
        1,
        SpellSchool::Conjuration,
        "1 action",
        0,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Strength,
            half_damage_on_save: true,
            damage_dice: Some("2d6".to_string()),
            damage_type: Some(DamageType::Necrotic),
            damage_scaling: Some("spell_slot".to_string()),
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility(
                    "Cannot take reactions until its next turn".to_string(),
                ),
                duration: DurationType::UntilEndOfNextTurn,
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Bane",
        1,
        SpellSchool::Enchantment,
        "1 action",
        30,
        Components::vsm("A drop of blood"),
        "1 minute",
        true,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Charisma,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Multiple(vec![
                    ActiveEffect::PenaltyAttackRoll("1d4".to_string()),
                    ActiveEffect::PenaltySavingThrow("1d4".to_string()),
                ]),
                duration: DurationType::Concentration,
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Bless",
        1,
        SpellSchool::Enchantment,
        "1 action",
        30,
        Components::vsm("A sprinkling of holy water"),
        "1 minute",
        true,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Multiple(vec![
                ActiveEffect::BonusAttackRoll("1d4".to_string()),
                ActiveEffect::BonusSavingThrow("1d4".to_string()),
            ]),
            duration: DurationType::Concentration,
        },
    ));

    reg.register_spell(Spell::new(
        "Burning Hands",
        1,
        SpellSchool::Evocation,
        "1 action",
        15, // 15 ft cone
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: true,
            damage_dice: Some("3d6".to_string()),
            damage_type: Some(DamageType::Fire),
            damage_scaling: Some("spell_slot".to_string()),
            fail_effect: None,
        },
    ));

    reg.register_spell(Spell::new(
        "Charm Person",
        1,
        SpellSchool::Enchantment,
        "1 action",
        30,
        Components::vs(),
        "1 hour",
        false,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Wisdom,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::ApplyCondition(crate::rules::conditions::Condition::Charmed),
                duration: DurationType::DurationHours(1),
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Command",
        1,
        SpellSchool::Enchantment,
        "1 action",
        60,
        Components::v(),
        "1 round",
        false,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Wisdom,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::ApplyCondition(
                    crate::rules::conditions::Condition::Incapacitated,
                ),
                duration: DurationType::UntilEndOfNextTurn,
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Comprehend Languages",
        1,
        SpellSchool::Divination,
        "1 action",
        0,
        Components::vsm("A pinch of soot and salt"),
        "1 hour",
        false,
        true, // Ritual
        SpellEffect::Utility { description: "You understand any spoken language that you hear. Moreover, when you touch the surface of any written language, you can understand it.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Create or Destroy Water",
        1,
        SpellSchool::Transmutation,
        "1 action",
        30,
        Components::vsm("A drop of water if creating water or a few grains of sand if destroying it"),
        "Instantaneous",
        false,
        true, // Ritual
        SpellEffect::Utility { description: "Create Water: Creates up to 10 gallons of clean water in an open container, or falls as rain in a 30-foot cube. Destroy Water: Destroys up to 10 gallons of water in an open container, or destroys fog in a 30-foot cube.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Cure Wounds",
        1,
        SpellSchool::Evocation,
        "1 action",
        0,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::Healing {
            healing_dice: "1d8".to_string(),
        },
    ));

    reg.register_spell(Spell::new(
        "Detect Evil and Good",
        1,
        SpellSchool::Divination,
        "1 action",
        0,
        Components::vs(),
        "10 minutes",
        true,
        false,
        SpellEffect::Utility { description: "You know if there is an aberration, celestial, elemental, fey, fiend, or undead within 30 feet of you, as well as where the creature is located. Similarly, you know if there is a place or object within 30 feet of you that has been magically consecrated or desecrated.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Detect Magic",
        1,
        SpellSchool::Divination,
        "1 action",
        0,
        Components::vs(),
        "10 minutes",
        true,
        true,
        SpellEffect::Utility { description: "You see a faint aura around any visible magical creature or object in the area, and you learn its school of magic, if any.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Detect Poison and Disease",
        1,
        SpellSchool::Divination,
        "1 action",
        0,
        Components::vsm("A yew leaf"),
        "10 minutes",
        true,
        true,
        SpellEffect::Utility { description: "You can sense the presence and location of poisons, poisonous creatures, and diseases within 30 feet of you. You also identify the kind of poison, poisonous creature, or disease in each case.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Disguise Self",
        1,
        SpellSchool::Illusion,
        "1 action",
        0,
        Components::vs(),
        "1 hour",
        false,
        false,
        SpellEffect::Utility { description: "You make yourself--including your clothing, armor, weapons, and other belongings on your person--look different until the spell ends or until you use your action to dismiss it. You can seem 1 foot shorter or taller and can appear thin, fat, or in between. To discern that you are disguised, a creature can use its action to inspect your appearance and must succeed on an Intelligence (Investigation) check against your spell save DC.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Divine Favor",
        1,
        SpellSchool::Evocation,
        "1 bonus action",
        0,
        Components::vs(),
        "1 minute",
        true,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::BonusDamage("1d4".to_string(), DamageType::Radiant),
            duration: DurationType::Concentration,
        },
    ));

    reg.register_spell(Spell::new(
        "Entangle",
        1,
        SpellSchool::Conjuration,
        "1 action",
        90,
        Components::vs(),
        "1 minute",
        true,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Strength,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::ApplyCondition(
                    crate::rules::conditions::Condition::Restrained,
                ),
                duration: DurationType::Concentration,
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Expeditious Retreat",
        1,
        SpellSchool::Transmutation,
        "1 bonus action",
        0,
        Components::vs(),
        "10 minutes",
        true,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility("Allows Dash action as a bonus action".to_string()),
            duration: DurationType::Concentration,
        },
    ));

    reg.register_spell(Spell::new(
        "Faerie Fire",
        1,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::v(),
        "1 minute",
        true,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::GrantsAdvantageToAttackers,
                duration: DurationType::Concentration,
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Feather Fall",
        1,
        SpellSchool::Transmutation,
        "1 reaction",
        60,
        Components::vm("A small feather or a piece of down"),
        "1 minute",
        false,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility(
                "Rate of descent slows to 60 feet per round. Takes no falling damage.".to_string(),
            ),
            duration: DurationType::DurationMinutes(1),
        },
    ));

    reg.register_spell(Spell::new(
        "Find Familiar",
        1,
        SpellSchool::Conjuration,
        "1 hour",
        10,
        Components::vsm("10gp worth of charcoal, incense, and herbs burned in a brass brazier"),
        "Instantaneous",
        false,
        true, // Ritual
        SpellEffect::SummonCreature {
            monster_id: "familiar".to_string(),
        },
    ));

    reg.register_spell(Spell::new(
        "Fog Cloud",
        1,
        SpellSchool::Conjuration,
        "1 action",
        120,
        Components::vs(),
        "1 hour",
        true,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility(
                "Creates a 20-foot-radius sphere of fog. The area is heavily obscured.".to_string(),
            ),
            duration: DurationType::Concentration,
        },
    ));

    reg.register_spell(Spell::new(
        "Goodberry",
        1,
        SpellSchool::Transmutation,
        "1 action",
        0,
        Components::vsm("A sprig of mistletoe"),
        "Instantaneous",
        false,
        false,
        SpellEffect::Healing {
            healing_dice: "1d1".to_string(),
        },
    ));

    reg.register_spell(Spell::new(
        "Grease",
        1,
        SpellSchool::Conjuration,
        "1 action",
        60,
        Components::vsm("A bit of pork rine or butter"),
        "1 minute",
        false,
        false,
        SpellEffect::AreaHazard {
            radius_feet: 10,
            save_ability: Some(Ability::Dexterity),
            condition: Some(crate::rules::conditions::Condition::Prone),
            damage_dice: None,
            damage_type: None,
            description: "Slick grease covers the ground in a 10-foot square centered on a point within range and turns it into difficult terrain for the duration.".to_string(),
        }
    ));

    reg.register_spell(Spell::new(
        "Guiding Bolt",
        1,
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vs(),
        "1 round",
        false,
        false,
        SpellEffect::Compound(vec![
            SpellEffect::SpellAttack { damage_dice: "4d6".to_string(), damage_type: DamageType::Radiant, damage_scaling: Some("spell_slot".to_string()), on_hit_effect: None },
            SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Next attack roll made against this target before end of your next turn has Advantage.".to_string()), duration: DurationType::UntilEndOfNextTurn }
        ])
    ));

    reg.register_spell(Spell::new(
        "Healing Word",
        1,
        SpellSchool::Evocation,
        "1 bonus action",
        60,
        Components::v(),
        "Instantaneous",
        false,
        false,
        SpellEffect::Healing {
            healing_dice: "1d4".to_string(),
        },
    ));

    reg.register_spell(Spell::new(
        "Hellish Rebuke",
        1,
        SpellSchool::Evocation,
        "1 reaction",
        60,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: true,
            damage_dice: Some("2d10".to_string()),
            damage_type: Some(DamageType::Fire),
            damage_scaling: Some("spell_slot".to_string()),
            fail_effect: None,
        },
    ));

    reg.register_spell(Spell::new(
        "Heroism",
        1,
        SpellSchool::Enchantment,
        "1 action",
        0,
        Components::vs(),
        "1 minute",
        true,
        false,
        SpellEffect::Compound(vec![
            SpellEffect::ApplyEffect {
                effect: ActiveEffect::ImmuneToCondition(
                    crate::rules::conditions::Condition::Frightened,
                ),
                duration: DurationType::Concentration,
            },
            SpellEffect::ApplyEffect {
                effect: ActiveEffect::TemporaryHP(3), // Modificador de conjuração típico
                duration: DurationType::Concentration,
            },
        ]),
    ));

    reg.register_spell(Spell::new(
        "Hideous Laughter",
        1,
        SpellSchool::Enchantment,
        "1 action",
        30,
        Components::vsm("Tiny tarts and a feather"),
        "1 minute",
        true,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Wisdom,
            half_damage_on_save: false,
            damage_dice: None,
            damage_type: None,
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Multiple(vec![
                    ActiveEffect::ApplyCondition(crate::rules::conditions::Condition::Prone),
                    ActiveEffect::ApplyCondition(
                        crate::rules::conditions::Condition::Incapacitated,
                    ),
                ]),
                duration: DurationType::Concentration,
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Hunter's Mark",
        1,
        SpellSchool::Divination,
        "1 bonus action",
        90,
        Components::v(),
        "1 hour",
        true,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::BonusDamage("1d6".to_string(), DamageType::Slashing),
            duration: DurationType::Concentration,
        },
    ));

    reg.register_spell(Spell::new(
        "Identify",
        1,
        SpellSchool::Divination,
        "1 minute",
        0,
        Components::vsm("A pearl worth at least 100 gp and an owl feather"),
        "Instantaneous",
        false,
        true, // Ritual
        SpellEffect::Utility {
            description:
                "You learn if an item has magical properties, its spells if any, and its charges."
                    .to_string(),
        },
    ));

    reg.register_spell(Spell::new(
        "Illusory Script",
        1,
        SpellSchool::Illusion,
        "1 minute",
        0,
        Components::vsm("A lead-based ink worth at least 10 gp"),
        "10 days",
        false,
        true, // Ritual
        SpellEffect::Utility { description: "You write on paper/parchment and imbue it with a potent illusion. To undesignated creatures, the script appears written in an unintelligible or different script.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Inflict Wounds",
        1,
        SpellSchool::Necromancy,
        "1 action",
        0,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::SpellAttack {
            damage_dice: "3d10".to_string(),
            damage_type: DamageType::Necrotic,
            damage_scaling: Some("spell_slot".to_string()),
            on_hit_effect: None,
        },
    ));

    reg.register_spell(Spell::new(
        "Jump",
        1,
        SpellSchool::Transmutation,
        "1 action",
        0,
        Components::vsm("A grasshopper's hind leg"),
        "1 minute",
        false,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility(
                "Target's jump distance is tripled for the duration.".to_string(),
            ),
            duration: DurationType::DurationMinutes(1),
        },
    ));

    reg.register_spell(Spell::new(
        "Longstrider",
        1,
        SpellSchool::Transmutation,
        "1 action",
        0,
        Components::vsm("A pinch of dirt"),
        "1 hour",
        false,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility(
                "Target's movement speed increases by 10 feet for the duration.".to_string(),
            ),
            duration: DurationType::DurationHours(1),
        },
    ));

    reg.register_spell(Spell::new(
        "Mage Armor",
        1,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vsm("A piece of cured leather"),
        "8 hours",
        false,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::BonusAC(3), // CA base vira 13 + Dex (bônus equivalente a +3 em relação a 10)
            duration: DurationType::DurationHours(8),
        },
    ));

    reg.register_spell(Spell::new(
        "Magic Missile",
        1,
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::MultiTargetSpellAttack {
            projectiles_count: 3,
            damage_dice: "1d4+1".to_string(),
            damage_type: DamageType::Force,
            auto_hit: true,
        },
    ));

    reg.register_spell(Spell::new(
        "Protection from Evil and Good",
        1,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vsm("Holy water or powdered silver and iron"),
        "10 minutes",
        true,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::AdvantageOnSavingThrow,
            duration: DurationType::Concentration,
        },
    ));

    reg.register_spell(Spell::new(
        "Purify Food and Drink",
        1,
        SpellSchool::Transmutation,
        "1 action",
        10,
        Components::vs(),
        "Instantaneous",
        false,
        true, // Ritual
        SpellEffect::Utility { description: "All nonmagical food and drink within a 5-foot-radius sphere centered on a point of your choice is purified and rendered free of poison and disease.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Sanctuary",
        1,
        SpellSchool::Abjuration,
        "1 bonus action",
        30,
        Components::vsm("A small silver mirror"),
        "1 minute",
        false,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility("Any creature who targets the warded creature with an attack or harmful spell must first make a Wisdom saving throw.".to_string()),
            duration: DurationType::DurationMinutes(1)
        }
    ));

    reg.register_spell(Spell::new(
        "Shield",
        1,
        SpellSchool::Abjuration,
        "1 reaction",
        0,
        Components::vs(),
        "1 round",
        false,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::BonusAC(5),
            duration: DurationType::UntilEndOfNextTurn,
        },
    ));

    reg.register_spell(Spell::new(
        "Shield of Faith",
        1,
        SpellSchool::Abjuration,
        "1 bonus action",
        60,
        Components::vsm("A small parchment with a bit of holy text written on it"),
        "10 minutes",
        true,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::BonusAC(2),
            duration: DurationType::Concentration,
        },
    ));

    reg.register_spell(Spell::new(
        "Silent Image",
        1,
        SpellSchool::Illusion,
        "1 action",
        60,
        Components::vsm("A bit of fleece"),
        "10 minutes",
        true,
        false,
        SpellEffect::Utility { description: "You create the image of an object, a creature, or some other visible phenomenon that is no larger than a 15-foot cube. The image is purely visual.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Sleep",
        1,
        SpellSchool::Enchantment,
        "1 action",
        90,
        Components::vsm("A pinch of fine sand, rose petals, or a cricket"),
        "1 minute",
        false,
        false,
        SpellEffect::Sleep {
            pool_dice: "5d8".to_string(),
            radius_feet: 20,
        },
    ));

    reg.register_spell(Spell::new(
        "Speak with Animals",
        1,
        SpellSchool::Divination,
        "1 action",
        0,
        Components::vs(),
        "10 minutes",
        false,
        true, // Ritual
        SpellEffect::Utility { description: "You gain the ability to comprehend and verbally communicate with beasts for the duration.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Thunderwave",
        1,
        SpellSchool::Evocation,
        "1 action",
        15, // 15ft cube
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Constitution,
            half_damage_on_save: true,
            damage_dice: Some("2d8".to_string()),
            damage_type: Some(DamageType::Thunder),
            damage_scaling: Some("spell_slot".to_string()),
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility("Pushed 10 feet away from the caster.".to_string()),
                duration: DurationType::Instantaneous,
            })),
        },
    ));

    reg.register_spell(Spell::new(
        "Unseen Servant",
        1,
        SpellSchool::Conjuration,
        "1 action",
        60,
        Components::vsm("A piece of string and a bit of wood"),
        "1 hour",
        false,
        true, // Ritual
        SpellEffect::SummonCreature {
            monster_id: "unseen_servant".to_string(),
        },
    ));

    reg.register_spell(Spell::new(
        "Witch Bolt",
        1,
        SpellSchool::Evocation,
        "1 action",
        30,
        Components::vsm("A twig from a tree that has been struck by lightning"),
        "1 minute",
        true,
        false,
        SpellEffect::SpellAttack {
            damage_dice: "1d12".to_string(),
            damage_type: DamageType::Lightning,
            damage_scaling: Some("spell_slot".to_string()),
            on_hit_effect: None,
        },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dice::DiceMode;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::background::create_acolyte;
    use crate::rules::character::class::{ClassVariant, Fighter, Wizard};
    use crate::rules::character::entity::Entity;
    use crate::rules::character::race::{Human, RaceVariant};
    use crate::rules::combat::attack::DamageAffinity;
    use crate::rules::spells::casting::cast_spell;
    use crate::rules::spells::spellcaster::SpellSlots;

    fn create_test_entities() -> (Entity, Entity) {
        let attrs_caster = AbilityScores::new(10, 14, 12, 16, 10, 8); // INT mod +3
        let attrs_target = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut caster = Entity::new(
            "Caster",
            attrs_caster,
            ClassVariant::Wizard(Wizard),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut slots = SpellSlots::new();
        slots.set_slots(1, 4);
        caster.spell_slots = Some(slots);
        let target = Entity::new(
            "Target",
            attrs_target,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        (caster, target)
    }

    #[test]
    fn test_alarm() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Alarm").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_armor_of_agathys() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Armor of Agathys").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_arms_of_hadar() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Arms of Hadar").unwrap();
        let initial_hp = target.current_hp;
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(target.current_hp <= initial_hp);
    }

    #[test]
    fn test_bane() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Bane").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_bless() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Bless").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_burning_hands() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Burning Hands").unwrap();
        let initial_hp = target.current_hp;
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(target.current_hp <= initial_hp);
    }

    #[test]
    fn test_charm_person() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Charm Person").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_command() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Command").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_comprehend_languages() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Comprehend Languages").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_create_or_destroy_water() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Create or Destroy Water").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_cure_wounds() {
        let (mut caster, mut target) = create_test_entities();
        target.current_hp = 5;
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Cure Wounds").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(target.current_hp > 5);
    }

    #[test]
    fn test_detect_evil_and_good() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Detect Evil and Good").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_detect_magic() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Detect Magic").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_detect_poison_and_disease() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Detect Poison and Disease").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_disguise_self() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Disguise Self").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_divine_favor() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Divine Favor").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_entangle() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Entangle").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_expeditious_retreat() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Expeditious Retreat").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_faerie_fire() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Faerie Fire").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_feather_fall() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Feather Fall").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_find_familiar() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Find Familiar").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_fog_cloud() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Fog Cloud").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_goodberry() {
        let (mut caster, mut target) = create_test_entities();
        target.current_hp = 5;
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Goodberry").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(target.current_hp > 5);
    }

    #[test]
    fn test_grease() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Grease").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_guiding_bolt() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Guiding Bolt").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_healing_word() {
        let (mut caster, mut target) = create_test_entities();
        target.current_hp = 5;
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Healing Word").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(target.current_hp > 5);
    }

    #[test]
    fn test_hellish_rebuke() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Hellish Rebuke").unwrap();
        let initial_hp = target.current_hp;
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(target.current_hp <= initial_hp);
    }

    #[test]
    fn test_heroism() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Heroism").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_hideous_laughter() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Hideous Laughter").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_hunters_mark() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Hunter's Mark").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_identify() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Identify").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_illusory_script() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Illusory Script").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_inflict_wounds() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Inflict Wounds").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_jump() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Jump").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Strength,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_longstrider() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Longstrider").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Dexterity,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_mage_armor() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Mage Armor").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_magic_missile() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Magic Missile").unwrap();
        let initial_hp = target.current_hp;
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(target.current_hp < initial_hp);
    }

    #[test]
    fn test_protection_from_evil_and_good() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Protection from Evil and Good").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_purify_food_and_drink() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Purify Food and Drink").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_sanctuary() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Sanctuary").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_shield() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Shield").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_shield_of_faith() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Shield of Faith").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_silent_image() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Silent Image").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_sleep() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Sleep").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(target.has_condition(&crate::rules::conditions::Condition::Unconscious));
    }

    #[test]
    fn test_speak_with_animals() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Speak with Animals").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_thunderwave() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Thunderwave").unwrap();
        let initial_hp = target.current_hp;
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
        assert!(target.current_hp <= initial_hp);
    }

    #[test]
    fn test_unseen_servant() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Unseen Servant").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }

    #[test]
    fn test_witch_bolt() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_level1_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Witch Bolt").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(res.slot_consumed);
    }
}
