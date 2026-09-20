use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use crate::rules::spells::registry::SpellRegistry;
use crate::rules::spells::spell::{Components, Spell, SpellEffect, SpellSchool};

/// Registra todas as magias de Nível 7 do SRD
pub fn load_level7_spells(reg: &mut SpellRegistry) {
    // 1. Arcane Sword
    reg.register_spell(Spell::new(
        "Arcane Sword",
        7,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::vsm("A miniature platinum sword with a grip and pommel of copper and zinc, worth 250 gp"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Create a sword of pure force that can attack enemies.".to_string() },
    ));

    // 2. Delayed Blast Fireball
    reg.register_spell(Spell::new(
        "Delayed Blast Fireball",
        7,
        SpellSchool::Evocation,
        "1 action",
        150,
        Components::vsm("A tiny ball of bat guano and sulfur"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Dexterity, half_damage_on_save: true, damage_dice: Some("12d6".to_string()), damage_type: Some(DamageType::Fire), damage_scaling: Some("1d6".to_string()), fail_effect: None },
    ));

    // 3. Divine Word
    reg.register_spell(Spell::new(
        "Divine Word",
        7,
        SpellSchool::Evocation,
        "1 bonus action",
        30,
        Components::v(),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Charisma, half_damage_on_save: false, damage_dice: None, damage_type: None, damage_scaling: None, fail_effect: Some(Box::new(SpellEffect::Utility { description: "Suffer effects based on current hit points (deafened, blinded, stunned, or killed instantly), and celestials/elementals/fey/fiends are banished.".to_string() })) },
    ));

    // 4. Etherealness
    reg.register_spell(Spell::new(
        "Etherealness",
        7,
        SpellSchool::Transmutation,
        "1 action",
        0,
        Components::vs(),
        "8 hours",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Step into the Ethereal Plane, moving through objects and creatures.".to_string() },
    ));

    // 5. Finger of Death
    reg.register_spell(Spell::new(
        "Finger of Death",
        7,
        SpellSchool::Necromancy,
        "1 action",
        60,
        Components::vs(),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Constitution, half_damage_on_save: true, damage_dice: Some("7d8+30".to_string()), damage_type: Some(DamageType::Necrotic), damage_scaling: None, fail_effect: None },
    ));

    // 6. Fire Storm
    reg.register_spell(Spell::new(
        "Fire Storm",
        7,
        SpellSchool::Evocation,
        "1 action",
        150,
        Components::vs(),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::SaveThrow { save_ability: Ability::Dexterity, half_damage_on_save: true, damage_dice: Some("7d10".to_string()), damage_type: Some(DamageType::Fire), damage_scaling: None, fail_effect: None },
    ));

    // 7. Forcecage
    reg.register_spell(Spell::new(
        "Forcecage",
        7,
        SpellSchool::Evocation,
        "1 action",
        100,
        Components::vsm("Ruby dust worth 1,500 gp"),
        "1 hour",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Trap creatures in an immobile, invisible, cube-shaped prison composed of magical force.".to_string() },
    ));

    // 8. Mirage Arcane
    reg.register_spell(Spell::new(
        "Mirage Arcane",
        7,
        SpellSchool::Illusion,
        "10 minutes",
        120,
        Components::vs(),
        "10 days",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Make terrain in a 1-mile area look, sound, smell, and even feel like some other sort of terrain.".to_string() },
    ));

    // 9. Plane Shift
    reg.register_spell(Spell::new(
        "Plane Shift",
        7,
        SpellSchool::Conjuration,
        "1 action",
        0,
        Components::vsm("A forked, metal rod worth at least 250 gp, attuned to a particular plane of existence"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Transport yourself or a creature to another plane of existence.".to_string() },
    ));

    // 10. Prismatic Spray
    reg.register_spell(Spell::new(
        "Prismatic Spray",
        7,
        SpellSchool::Evocation,
        "1 action",
        0,
        Components::vs(),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Eight rays of light flash out, applying random damage or effects based on a d8 roll.".to_string() },
    ));

    // 11. Resurrection
    reg.register_spell(Spell::new(
        "Resurrection",
        7,
        SpellSchool::Necromancy,
        "1 hour",
        0,
        Components::vsm("A diamond worth at least 1,000 gp, which the spell consumes"),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Return a dead creature to life, curing poisons and nonmagical diseases, and restoring all hit points.".to_string() },
    ));

    // 12. Reverse Gravity
    reg.register_spell(Spell::new(
        "Reverse Gravity",
        7,
        SpellSchool::Transmutation,
        "1 action",
        100,
        Components::vsm("A lodestone and iron filings"),
        "1 minute",
        true,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Reverse gravity in a 50-foot-radius, 100-foot-high cylinder, causing creatures and objects to fall upward.".to_string() },
    ));

    // 13. Sequester
    reg.register_spell(Spell::new(
        "Sequester",
        7,
        SpellSchool::Transmutation,
        "1 action",
        0,
        Components::vsm("A powder composed of diamond, emerald, ruby, and sapphire dust worth at least 5,000 gp, which the spell consumes"),
        "Until dispelled",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Keep a willing creature or an object safe, making it invisible and immune to divination magic.".to_string() },
    ));

    // 14. Simulacrum
    reg.register_spell(Spell::new(
        "Simulacrum",
        7,
        SpellSchool::Illusion,
        "12 hours",
        0,
        Components::vsm("Snow or ice in quantities sufficient to made a life-size copy of the duplicated creature; some hair, fingernail clippings, or other piece of that creature's body placed inside the snow or ice; and powdered ruby worth 1,500 gp, sprinkled over the duplicate and consumed by the spell"),
        "Until dispelled",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Create an illusory duplicate of a beast or humanoid that acts as your ally.".to_string() },
    ));

    // 15. Symbol
    reg.register_spell(Spell::new(
        "Symbol",
        7,
        SpellSchool::Abjuration,
        "1 minute",
        0,
        Components::vsm("Mercury, phosphorus, and powdered diamond and opal with a total value of at least 1,000 gp, which the spell consumes"),
        "Until dispelled",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Inscribe a magical glyph that harms creatures when triggered.".to_string() },
    ));

    // 16. Teleport
    reg.register_spell(Spell::new(
        "Teleport",
        7,
        SpellSchool::Conjuration,
        "1 action",
        10,
        Components::v(),
        "Instantaneous",
        false,  // Concentration
        false, // Ritual
        SpellEffect::Utility { description: "Instantly transport yourself and up to eight willing creatures to a destination you select.".to_string() },
    ));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arcane_sword_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Arcane Sword").expect("Arcane Sword not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_delayed_blast_fireball_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Delayed Blast Fireball").expect("Delayed Blast Fireball not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_divine_word_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Divine Word").expect("Divine Word not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_etherealness_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Etherealness").expect("Etherealness not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_finger_of_death_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Finger of Death").expect("Finger of Death not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_fire_storm_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Fire Storm").expect("Fire Storm not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_forcecage_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Forcecage").expect("Forcecage not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_mirage_arcane_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Mirage Arcane").expect("Mirage Arcane not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_plane_shift_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Plane Shift").expect("Plane Shift not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_prismatic_spray_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Prismatic Spray").expect("Prismatic Spray not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_resurrection_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Resurrection").expect("Resurrection not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_reverse_gravity_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Reverse Gravity").expect("Reverse Gravity not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_sequester_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Sequester").expect("Sequester not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_simulacrum_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Simulacrum").expect("Simulacrum not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_symbol_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Symbol").expect("Symbol not found");
        assert_eq!(spell.level, 7);
    }
    #[test]
    fn test_teleport_spell() {
        let mut reg = SpellRegistry::new();
        load_level7_spells(&mut reg);
        let spell = SpellRegistry::get_spell("Teleport").expect("Teleport not found");
        assert_eq!(spell.level, 7);
    }
}
