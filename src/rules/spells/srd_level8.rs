use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use crate::rules::spells::registry::SpellRegistry;
use crate::rules::spells::spell::{Components, Spell, SpellEffect, SpellSchool};

/// Registra todas as magias de Nível 8 do SRD
pub fn load_level8_spells(reg: &mut SpellRegistry) {
    // 1. Animal Shapes
    reg.register_spell(Spell::new(
        "Animal Shapes",
        8,
        SpellSchool::Transmutation,
        "1 action",
        30,
        Components::vs(),
        "24 hours",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Transform any number of willing creatures into Large or smaller beasts of CR 4 or lower.".to_string() },
    ));

    // 2. Antimagic Field
    reg.register_spell(Spell::new(
        "Antimagic Field",
        8,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vsm("A pinch of powdered iron or iron filings"),
        "1 hour",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Create a 10-foot-radius sphere where magic does not function."
                .to_string(),
        },
    ));

    // 3. Antipathy Sympathy
    reg.register_spell(Spell::new(
        "Antipathy Sympathy",
        8,
        SpellSchool::Enchantment,
        "1 hour",
        60,
        Components::vsm("Either a lump of alum soaked in vinegar or a drop of honey"),
        "10 days",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Attract or repel a specific kind of intelligent creature to or from a target object or area.".to_string() },
    ));

    // 4. Clone
    reg.register_spell(Spell::new(
        "Clone",
        8,
        SpellSchool::Necromancy,
        "1 hour",
        0,
        Components::vsm("A diamond worth at least 1,000 gp and at least 1 cubic inch of flesh of the creature that is to be cloned, which the spell consumes, and a vessel worth at least 2,000 gp that has a sealable lid and is large enough to hold a Medium creature, such as a huge urn, coffin, mud-filled cyst in the ground, or crystal container filled with salt water"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Grow an inert duplicate of a living creature as a safeguard against death.".to_string() },
    ));

    // 5. Control Weather
    reg.register_spell(Spell::new(
        "Control Weather",
        8,
        SpellSchool::Transmutation,
        "10 minutes",
        0,
        Components::vsm("Burning incense and bits of earth and wood mixed in water"),
        "8 hours",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility {
            description: "Take control of the weather within 5 miles of you for the duration."
                .to_string(),
        },
    ));

    // 6. Earthquake
    reg.register_spell(Spell::new(
        "Earthquake",
        8,
        SpellSchool::Evocation,
        "1 action",
        500,
        Components::vsm("A pinch of dirt, a piece of rock, and a lump of clay"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Create a seismic disturbance in a 100-foot-radius circle, dealing bludgeoning damage and causing various terrain effects.".to_string() },
    ));

    // 7. Feeblemind
    reg.register_spell(Spell::new(
        "Feeblemind",
        8,
        SpellSchool::Enchantment,
        "1 action",
        150,
        Components::vsm("A handful of clay, crystal, glass, or mineral spheres"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Intelligence, half_damage_on_save: false, damage_dice: Some("4d6".to_string()), damage_type: Some(DamageType::Psychic), damage_scaling: None, fail_effect: Some(Box::new(SpellEffect::Utility { description: "Target's Intelligence and Charisma scores become 1. They can't cast spells, activate magic items, understand language, or communicate in any intelligible way.".to_string() })) },
    ));

    // 8. Holy Aura
    reg.register_spell(Spell::new(
        "Holy Aura",
        8,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vsm("A tiny reliquary worth at least 1,000 gp containing a sacred relic, such as a scrap of cloth from a saint's robe or a piece of parchment from a holy text"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Provide advantage on all saving throws and impose disadvantage on attack rolls against you and up to 8 allies. Fiends and undead that hit a protected creature with a melee attack may be blinded.".to_string() },
    ));

    // 9. Incendiary Cloud
    reg.register_spell(Spell::new(
        "Incendiary Cloud",
        8,
        SpellSchool::Conjuration,
        "1 action",
        150,
        Components::vs(),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: true,
            damage_dice: Some("10d8".to_string()),
            damage_type: Some(DamageType::Fire),
            damage_scaling: None,
            fail_effect: None,
        },
    ));

    // 10. Maze
    reg.register_spell(Spell::new(
        "Maze",
        8,
        SpellSchool::Conjuration,
        "1 action",
        60,
        Components::vs(),
        "10 minutes",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Banish a creature into a labyrinthine demiplane. They can escape with a DC 20 Intelligence check.".to_string() },
    ));

    // 11. Mind Blank
    reg.register_spell(Spell::new(
        "Mind Blank",
        8,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vs(),
        "24 hours",
        false,  // Concentration
        false, // Ritual
        SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Immune to psychic damage, any effect that would sense its emotions or read its thoughts, divination spells, and the charmed condition.".to_string()), duration: DurationType::DurationHours(24) },
    ));

    // 12. Power Word Stun
    reg.register_spell(Spell::new(
        "Power Word Stun",
        8,
        SpellSchool::Enchantment,
        "1 action",
        60,
        Components::v(),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "If the target has 150 hit points or fewer, it is stunned. Otherwise, the spell has no effect.".to_string() },
    ));

    // 13. Sunburst
    reg.register_spell(Spell::new(
        "Sunburst",
        8,
        SpellSchool::Evocation,
        "1 action",
        150,
        Components::vsm("Fire and a piece of sunstone"),
        "Instantaneous",
        false, // Concentration
        false, // Ritual
        SpellEffect::SaveThrow {
            save_ability: Ability::Constitution,
            half_damage_on_save: true,
            damage_dice: Some("12d6".to_string()),
            damage_type: Some(DamageType::Radiant),
            damage_scaling: None,
            fail_effect: Some(Box::new(SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility("Blinded for 1 minute.".to_string()),
                duration: DurationType::DurationMinutes(1),
            })),
        },
    ));

    // 14. Tsunami
    reg.register_spell(Spell::new(
        "Tsunami",
        8,
        SpellSchool::Conjuration,
        "1 minute",
        120,
        Components::vs(),
        "6 rounds",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Create a wall of water that moves and crashes into creatures, dealing massive bludgeoning damage and pushing them.".to_string() },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animal_shapes_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Animal Shapes").expect("Animal Shapes not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_antimagic_field_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Antimagic Field").expect("Antimagic Field not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_antipathy_sympathy_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("Antipathy Sympathy").expect("Antipathy Sympathy not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_clone_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Clone").expect("Clone not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_control_weather_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Control Weather").expect("Control Weather not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_earthquake_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Earthquake").expect("Earthquake not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_feeblemind_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Feeblemind").expect("Feeblemind not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_holy_aura_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Holy Aura").expect("Holy Aura not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_incendiary_cloud_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell =
            SpellRegistry::get_spell("Incendiary Cloud").expect("Incendiary Cloud not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_maze_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Maze").expect("Maze not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_mind_blank_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Mind Blank").expect("Mind Blank not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_power_word_stun_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Power Word Stun").expect("Power Word Stun not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_sunburst_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Sunburst").expect("Sunburst not found");
        assert_eq!(spell.level, 8);
    }
    #[test]
    fn test_tsunami_spell() {
        let mut reg = SpellRegistry::new();
        load_level8_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Tsunami").expect("Tsunami not found");
        assert_eq!(spell.level, 8);
    }
}
