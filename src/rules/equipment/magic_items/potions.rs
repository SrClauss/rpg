use super::{Attunement, MagicItem, MagicItemEffect, MagicItemType, Rarity};
use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::character::attributes::Ability;
use crate::rules::character::race::Size;
use crate::rules::conditions::Condition;
use crate::rules::equipment::weapons::DamageType;

pub fn get_magic_potions() -> Vec<MagicItem> {
    vec![
        MagicItem::new_with_effects(
            "Oil of Etherealness",
            MagicItemType::Potion,
            Rarity::Rare,
            Attunement::None,
            None,
            vec![MagicItemEffect::CastSpell {
                spell_name: "Etherealness".to_string(),
                cast_level: 7,
                save_dc: None,
                attack_bonus: None,
            }],
            "When applied, you gain the effect of the etherealness spell for 1 hour.",
        ),
        MagicItem::new_with_effects(
            "Oil of Sharpness",
            MagicItemType::Potion,
            Rarity::VeryRare,
            Attunement::None,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::GrantItemAction("Buff Weapon +3 for 1 hour".to_string()),
                duration: DurationType::DurationHours(1),
            }],
            "When applied to a slashing or piercing weapon, it grants a +3 bonus to attack and damage rolls for 1 hour.",
        ),
        MagicItem::new_with_effects(
            "Oil of Slipperiness",
            MagicItemType::Potion,
            Rarity::Uncommon,
            Attunement::None,
            None,
            vec![MagicItemEffect::CastSpell {
                spell_name: "Freedom of Movement".to_string(),
                cast_level: 4,
                save_dc: None,
                attack_bonus: None,
            }],
            "When applied, you gain the effect of the freedom of movement spell for 8 hours.",
        ),
        MagicItem::new_with_effects(
            "Philter of Love",
            MagicItemType::Potion,
            Rarity::Uncommon,
            Attunement::None,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::ApplyCondition(Condition::Charmed),
                duration: DurationType::DurationHours(1),
            }],
            "When you drink this, you become charmed by the next creature you see within 10 minutes. Lasts 1 hour.",
        ),
        MagicItem::new_with_effects(
            "Potion of Animal Friendship",
            MagicItemType::Potion,
            Rarity::Uncommon,
            Attunement::None,
            None,
            vec![MagicItemEffect::CastSpell {
                spell_name: "Animal Friendship".to_string(),
                cast_level: 1,
                save_dc: Some(13),
                attack_bonus: None,
            }],
            "When you drink this, you can cast the animal friendship spell (DC 13) for 1 hour.",
        ),
        MagicItem::new_with_effects(
            "Potion of Climbing",
            MagicItemType::Potion,
            Rarity::Common,
            Attunement::None,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::GainClimbingSpeed,
                duration: DurationType::DurationHours(1),
            }],
            "When you drink this, you gain a climbing speed equal to your walking speed for 1 hour.",
        ),
        MagicItem::new_with_effects(
            "Potion of Diminution",
            MagicItemType::Potion,
            Rarity::Rare,
            Attunement::None,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::ChangeSize(Size::Tiny),
                duration: DurationType::DurationHours(1), // Simplified 1d4
            }],
            "When you drink this, you suffer the 'reduce' effect of the enlarge/reduce spell for 1d4 hours.",
        ),
        MagicItem::new_with_effects(
            "Potion of Flying",
            MagicItemType::Potion,
            Rarity::VeryRare,
            Attunement::None,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::GainFlyingSpeed,
                duration: DurationType::DurationHours(1),
            }],
            "When you drink this, you gain a flying speed equal to your walking speed for 1 hour.",
        ),
        MagicItem::new_with_effects(
            "Potion of Gaseous Form",
            MagicItemType::Potion,
            Rarity::Rare,
            Attunement::None,
            None,
            vec![MagicItemEffect::CastSpell {
                spell_name: "Gaseous Form".to_string(),
                cast_level: 3,
                save_dc: None,
                attack_bonus: None,
            }],
            "When you drink this, you gain the effect of the gaseous form spell for 1 hour.",
        ),
        MagicItem::new_with_effects(
            "Potion of Giant Strength",
            MagicItemType::Potion,
            Rarity::Uncommon,
            Attunement::None,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::OverrideAttribute(Ability::Strength, 21), // Hill Giant by default
                duration: DurationType::DurationHours(1),
            }],
            "When you drink this, your Strength score changes for 1 hour. Varies by type of giant.",
        ),
        MagicItem::new_with_effects(
            "Potion of Growth",
            MagicItemType::Potion,
            Rarity::Uncommon,
            Attunement::None,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::ChangeSize(Size::Large),
                duration: DurationType::DurationHours(1), // Simplified 1d4
            }],
            "When you drink this, you gain the 'enlarge' effect of the enlarge/reduce spell for 1d4 hours.",
        ),
        MagicItem::new_with_effects(
            "Potion of Healing",
            MagicItemType::Potion,
            Rarity::Common,
            Attunement::None,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::RestoreHitPoints("2d4+2".to_string()),
                duration: DurationType::Instantaneous,
            }],
            "When you drink this, you regain hit points. Varies by rarity.",
        ),
        MagicItem::new_with_effects(
            "Potion of Invisibility",
            MagicItemType::Potion,
            Rarity::VeryRare,
            Attunement::None,
            None,
            vec![MagicItemEffect::CastSpell {
                spell_name: "Invisibility".to_string(),
                cast_level: 2,
                save_dc: None,
                attack_bonus: None,
            }],
            "When you drink this, you become invisible for 1 hour, or until you attack or cast a spell.",
        ),
        MagicItem::new_with_effects(
            "Potion of Mind Reading",
            MagicItemType::Potion,
            Rarity::Rare,
            Attunement::None,
            None,
            vec![MagicItemEffect::CastSpell {
                spell_name: "Detect Thoughts".to_string(),
                cast_level: 2,
                save_dc: None,
                attack_bonus: None,
            }],
            "When you drink this, you gain the effect of the detect thoughts spell (save DC 13).",
        ),
        MagicItem::new_with_effects(
            "Potion of Poison",
            MagicItemType::Potion,
            Rarity::Uncommon,
            Attunement::None,
            None,
            vec![
                MagicItemEffect::Activated {
                    effect: ActiveEffect::TakeDamage("3d6".to_string(), DamageType::Poison),
                    duration: DurationType::Instantaneous,
                },
                MagicItemEffect::Activated {
                    effect: ActiveEffect::ApplyCondition(Condition::Poisoned),
                    duration: DurationType::DurationHours(1),
                }
            ],
            "Looks, smells, and tastes like a potion of healing. Actually poison.",
        ),
        MagicItem::new_with_effects(
            "Potion of Resistance",
            MagicItemType::Potion,
            Rarity::Uncommon,
            Attunement::None,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::Resistance(vec![DamageType::Fire]), // Default to Fire, usually varies
                duration: DurationType::DurationHours(1),
            }],
            "When you drink this, you gain resistance to one type of damage for 1 hour.",
        ),
        MagicItem::new_with_effects(
            "Potion of Speed",
            MagicItemType::Potion,
            Rarity::VeryRare,
            Attunement::None,
            None,
            vec![MagicItemEffect::CastSpell {
                spell_name: "Haste".to_string(),
                cast_level: 3,
                save_dc: None,
                attack_bonus: None,
            }],
            "When you drink this, you gain the effect of the haste spell for 1 minute.",
        ),
        MagicItem::new_with_effects(
            "Potion of Water Breathing",
            MagicItemType::Potion,
            Rarity::Uncommon,
            Attunement::None,
            None,
            vec![MagicItemEffect::CastSpell {
                spell_name: "Water Breathing".to_string(),
                cast_level: 3,
                save_dc: None,
                attack_bonus: None,
            }],
            "You can breathe underwater for 1 hour after drinking this.",
        ),
    ]
}
