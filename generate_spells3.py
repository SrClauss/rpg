import sys

file_path = "/home/claus/src/rpg/src/rules/spells/srd_level3.rs"

spells = [
    ("Dispel Magic", "Abjuration", "1 action", 120, "vs()", "Instantaneous", False, 'SpellEffect::Utility { description: "Choose one creature, object, or magical effect. Any spell of 3rd level or lower on the target ends. For higher levels, make an ability check.".to_string() }'),
    ("Fear", "Illusion", "1 action", 30, "vsm(\"A white feather or the heart of a hen\")", "1 minute", True, 'SpellEffect::AreaHazard { radius_feet: 30, save_ability: Some(Ability::Wisdom), condition: Some(crate::rules::conditions::Condition::Frightened), damage_dice: None, damage_type: None, description: "".to_string() }'),
    ("Feign Death", "Necromancy", "1 action", 0, "vsm(\"A pinch of graveyard dirt\")", "1 hour", False, 'SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target appears dead and is blinded/incapacitated, but has resistance to all damage except psychic.".to_string()), duration: DurationType::DurationMinutes(60) }'),
    ("Fireball", "Evocation", "1 action", 150, "vsm(\"A tiny ball of bat guano and sulfur\")", "Instantaneous", False, 'SpellEffect::AreaHazard { radius_feet: 20, save_ability: Some(Ability::Dexterity), condition: None, damage_dice: Some("8d6".to_string()), damage_type: Some(DamageType::Fire), description: "".to_string() }'),
    ("Fly", "Transmutation", "1 action", 0, "vsm(\"A wing feather from any bird\")", "10 minutes", True, 'SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target gains a flying speed of 60 feet.".to_string()), duration: DurationType::DurationMinutes(10) }'),
    ("Gaseous Form", "Transmutation", "1 action", 0, "vsm(\"A bit of gauze and a wisp of smoke\")", "1 hour", True, 'SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target transforms into a misty cloud. Has fly speed 10 ft, resistance to nonmagical damage, and advantage on STR/DEX/CON saves.".to_string()), duration: DurationType::DurationMinutes(60) }'),
    ("Glyph of Warding", "Abjuration", "1 hour", 0, "vsm(\"Incense and powdered diamond worth at least 200 gp, which the spell consumes\")", "Until dispelled", False, 'SpellEffect::Utility { description: "Inscribes a glyph that harms other creatures either with explosive runes (5d8 damage) or a stored spell up to 3rd level.".to_string() }'),
    ("Haste", "Transmutation", "1 action", 30, "vsm(\"A shaving of licorice root\")", "1 minute", True, 'SpellEffect::Compound(vec![SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target speed is doubled, gains +2 AC, advantage on DEX saves, and an additional action each turn.".to_string()), duration: DurationType::DurationMinutes(1) }])'),
    ("Hypnotic Pattern", "Illusion", "1 action", 120, "sm(\"A glowing stick of incense or a crystal vial filled with phosphorescent material\")", "1 minute", True, 'SpellEffect::AreaHazard { radius_feet: 30, save_ability: Some(Ability::Wisdom), condition: Some(crate::rules::conditions::Condition::Charmed), damage_dice: None, damage_type: None, description: "".to_string() }'),
    ("Lightning Bolt", "Evocation", "1 action", 100, "vsm(\"A bit of fur and a rod of amber, crystal, or glass\")", "Instantaneous", False, 'SpellEffect::AreaHazard { radius_feet: 5, save_ability: Some(Ability::Dexterity), condition: None, damage_dice: Some("8d6".to_string()), damage_type: Some(DamageType::Lightning), description: "".to_string() }'),
    ("Magic Circle", "Abjuration", "1 minute", 10, "vsm(\"Holy water or powdered silver and iron worth at least 100 gp, which the spell consumes\")", "1 hour", False, 'SpellEffect::Utility { description: "Creates a 10-foot radius cylinder of magical energy. Creatures of chosen types can\'t willingly enter by nonmagical means.".to_string() }'),
    ("Major Image", "Illusion", "1 action", 120, "vsm(\"A bit of fleece\")", "10 minutes", True, 'SpellEffect::Utility { description: "Creates an illusion of an object, creature, or phenomenon no larger than a 20-foot cube, including sounds, smells, and temperature.".to_string() }'),
    ("Mass Healing Word", "Evocation", "1 bonus action", 60, "v()", "Instantaneous", False, 'SpellEffect::MultiTargetSpellAttack { projectiles_count: 6, damage_dice: "1d4".to_string(), damage_type: DamageType::Radiant, auto_hit: true }'),
    ("Meld into Stone", "Transmutation", "1 action", 0, "vs()", "8 hours", False, 'SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target steps into a stone object or surface large enough to fully contain it. Cannot see out, has advantage on hearing checks.".to_string()), duration: DurationType::DurationMinutes(480) }'),
    ("Nondetection", "Abjuration", "1 action", 0, "vsm(\"A pinch of diamond dust worth 25 gp sprinkled over the target, which the spell consumes\")", "8 hours", False, 'SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target is hidden from divination magic and cannot be targeted by such spells or perceived through magical scrying sensors.".to_string()), duration: DurationType::DurationMinutes(480) }'),
    ("Phantom Steed", "Illusion", "1 minute", 30, "vs()", "1 hour", False, 'SpellEffect::SummonCreature { monster_id: "Phantom Steed".to_string() }'),
    ("Protection from Energy", "Abjuration", "1 action", 0, "vs()", "1 hour", True, 'SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target has resistance to one damage type: acid, cold, fire, lightning, or thunder.".to_string()), duration: DurationType::DurationMinutes(60) }'),
    ("Remove Curse", "Abjuration", "1 action", 0, "vs()", "Instantaneous", False, 'SpellEffect::Utility { description: "At your touch, all curses on one creature or object end.".to_string() }'),
    ("Revivify", "Necromancy", "1 action", 0, "vsm(\"Diamonds worth 300 gp, which the spell consumes\")", "Instantaneous", False, 'SpellEffect::Utility { description: "You touch a creature that has died within the last minute. That creature returns to life with 1 hit point.".to_string() }'),
    ("Sending", "Evocation", "1 action", 9999, "vsm(\"A short piece of fine copper wire\")", "1 round", False, 'SpellEffect::Utility { description: "Send a short message of 25 words or less to a creature with which you are familiar. They can reply.".to_string() }'),
    ("Sleet Storm", "Conjuration", "1 action", 150, "vsm(\"A pinch of dust and a few drops of water\")", "1 minute", True, 'SpellEffect::AreaHazard { radius_feet: 40, save_ability: Some(Ability::Dexterity), condition: Some(crate::rules::conditions::Condition::Prone), damage_dice: None, damage_type: None, description: "".to_string() }'),
    ("Slow", "Transmutation", "1 action", 120, "vsm(\"A drop of molasses\")", "1 minute", True, 'SpellEffect::MultiTargetApplyEffect { max_targets: 6, effect: ActiveEffect::Utility("Target speed halved, -2 to AC and DEX saves, and cannot use reactions. Can only use action or bonus action, not both. Spells might take 2 turns to cast.".to_string()), duration: DurationType::DurationMinutes(1) }'),
    ("Speak with Dead", "Necromancy", "1 action", 10, "vsm(\"Burning incense\")", "10 minutes", False, 'SpellEffect::Utility { description: "Grant the semblance of life and intelligence to a corpse of your choice, allowing it to answer up to 5 questions.".to_string() }'),
    ("Speak with Plants", "Transmutation", "1 action", 0, "vs()", "10 minutes", False, 'SpellEffect::Utility { description: "Imbue plants within 30 feet of you with limited sentience and animation, giving them the ability to communicate with you and follow simple commands.".to_string() }'),
    ("Spirit Guardians", "Conjuration", "1 action", 0, "vsm(\"A holy symbol\")", "10 minutes", True, 'SpellEffect::AreaHazard { radius_feet: 15, save_ability: Some(Ability::Wisdom), condition: None, damage_dice: Some("3d8".to_string()), damage_type: Some(DamageType::Radiant), description: "".to_string() }'),
    ("Stinking Cloud", "Conjuration", "1 action", 90, "vsm(\"A rotten egg or several skunk cabbage leaves\")", "1 minute", True, 'SpellEffect::AreaHazard { radius_feet: 20, save_ability: Some(Ability::Constitution), condition: Some(crate::rules::conditions::Condition::Incapacitated), damage_dice: None, damage_type: None, description: "".to_string() }'),
    ("Tongues", "Divination", "1 action", 0, "vsm(\"A small clay model of a ziggurat\")", "1 hour", False, 'SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target understands any spoken language it hears, and any creature that knows at least one language and can hear the target understands what it says.".to_string()), duration: DurationType::DurationMinutes(60) }'),
    ("Vampiric Touch", "Necromancy", "1 action", 0, "vs()", "1 minute", True, 'SpellEffect::SpellAttack { damage_dice: "3d6".to_string(), damage_type: DamageType::Necrotic, damage_scaling: None, on_hit_effect: None }'),
    ("Water Breathing", "Transmutation", "1 action", 30, "vsm(\"A short reed or piece of straw\")", "24 hours", False, 'SpellEffect::MultiTargetApplyEffect { max_targets: 10, effect: ActiveEffect::Utility("Targets can breathe underwater for the duration.".to_string()), duration: DurationType::DurationMinutes(1440) }'),
    ("Water Walk", "Transmutation", "1 action", 30, "vsm(\"A piece of cork\")", "1 hour", False, 'SpellEffect::MultiTargetApplyEffect { max_targets: 10, effect: ActiveEffect::Utility("Targets can move across any liquid surface as if it were harmless solid ground.".to_string()), duration: DurationType::DurationMinutes(60) }'),
    ("Wind Wall", "Evocation", "1 action", 120, "vsm(\"A tiny fan and a feather of exotic origin\")", "1 minute", True, 'SpellEffect::AreaHazard { radius_feet: 50, save_ability: Some(Ability::Strength), condition: None, damage_dice: Some("3d8".to_string()), damage_type: Some(DamageType::Bludgeoning), description: "".to_string() }'),
]

# Construct the code to append
code_to_insert = ""
for i, spell in enumerate(spells):
    spell_id = 11 + i
    code_to_insert += f"""    // {spell_id}. {spell[0]}
    reg.register_spell(Spell::new(
        "{spell[0]}",
        3,
        SpellSchool::{spell[1]},
        "{spell[2]}",
        {spell[3]},
        Components::{spell[4]},
        "{spell[5]}",
        {'true' if spell[6] else 'false'}, // Concentration
        false, // Ritual
        {spell[7]}
    ));

"""

# Construct the tests
tests_to_insert = ""
for spell in spells:
    test_name = "test_" + spell[0].lower().replace(" ", "_").replace("/", "_").replace("'", "") + "_spell"
    outcome_check = "UtilityApplied"
    
    if "SummonCreature" in spell[7] or "Summon {" in spell[7]:
        outcome_check = "Summoned"
    elif "AreaHazard" in spell[7]:
        outcome_check = "AreaHazardCreated"
    elif "SpellAttack" in spell[7] and "MultiTarget" not in spell[7]:
        outcome_check = "DamageDealt"
    elif "MultiTargetSpellAttack" in spell[7]:
        outcome_check = "Healed"
        
    tests_to_insert += f"""    #[test]
    fn {test_name}() {{
        let spell = Spell::new(
            "{spell[0]}",
            3,
            SpellSchool::{spell[1]},
            "{spell[2]}",
            {spell[3]},
            Components::{spell[4]},
            "{spell[5]}",
            {'true' if spell[6] else 'false'},
            false,
            {spell[7]}
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
        assert!(matches!(outcome.outcome, CastOutcome::{outcome_check} {{ .. }}));
    }}

"""

with open(file_path, "r") as f:
    content = f.read()

target1 = """        SpellEffect::Utility {
            description: "A 60-foot sphere of bright light spreading from a point. Dispels magical darkness created by spells of 3rd level or lower.".to_string(),
        },
    ));

}"""

replace1 = """        SpellEffect::Utility {
            description: "A 60-foot sphere of bright light spreading from a point. Dispels magical darkness created by spells of 3rd level or lower.".to_string(),
        },
    ));

""" + code_to_insert + "}\n"

if target1 in content:
    content = content.replace(target1, replace1)
else:
    # Try another way
    parts = content.split("    // 10. Daylight")
    if len(parts) > 1:
        p2 = parts[1].split("\n}", 1)
        if len(p2) > 1:
            content = parts[0] + "    // 10. Daylight" + p2[0] + "\n\n" + code_to_insert + "}\n" + p2[1]

target2 = "    }\n}"
replace2 = "    }\n\n" + tests_to_insert + "}"
if target2 in content:
    content = content.replace(target2, replace2)
else:
    parts = content.rsplit("    }\n}", 1)
    if len(parts) == 2:
        content = parts[0] + "    }\n\n" + tests_to_insert + "}\n" + parts[1]

with open(file_path, "w") as f:
    f.write(content)

print("Injected successfully!")
