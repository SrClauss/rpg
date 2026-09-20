use super::{Attunement, MagicItem, MagicItemEffect, MagicItemType, Rarity};
use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::character::language::Language;
use crate::rules::equipment::armor::{Armor, ArmorCategory};
use crate::rules::equipment::weapons::DamageType;
use crate::rules::monsters::monster::CreatureType;

pub fn get_magic_armor() -> Vec<MagicItem> {
    vec![
        MagicItem::new_with_effects(
            "Adamantine Armor",
            MagicItemType::Armor,
            Rarity::Uncommon,
            Attunement::None,
            None,
            vec![MagicItemEffect::Passive(ActiveEffect::ImmuneToCriticalHits)],
            "Any critical hit against you becomes a normal hit.",
        ),
        MagicItem::new_with_effects(
            "Animated Shield",
            MagicItemType::Armor,
            Rarity::VeryRare,
            Attunement::Required,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::BonusAC(2),
                duration: DurationType::DurationMinutes(1),
            }],
            "You can use a bonus action to animate the shield. It protects you as if you were wielding it, leaving your hands free, for 1 minute.",
        ),
        MagicItem::new_with_effects(
            "Armor of Invulnerability",
            MagicItemType::Armor,
            Rarity::Legendary,
            Attunement::Required,
            None,
            vec![
                MagicItemEffect::Passive(ActiveEffect::ResistanceToNonMagicalDamage),
                MagicItemEffect::Activated {
                    effect: ActiveEffect::ImmuneToNonMagicalDamage,
                    duration: DurationType::DurationMinutes(10),
                },
            ],
            "You have resistance to nonmagical damage. You can use an action to gain immunity to nonmagical damage for 10 minutes (once per dawn).",
        ),

        MagicItem::new_with_effects(
            "Armor, +1",
            MagicItemType::Armor,
            Rarity::Rare,
            Attunement::None,
            None,
            vec![MagicItemEffect::Passive(ActiveEffect::BonusAC(1))],
            "You have a +1 bonus to AC while wearing this armor.",
        ),
        MagicItem::new_with_effects(
            "Armor, +2",
            MagicItemType::Armor,
            Rarity::VeryRare,
            Attunement::None,
            None,
            vec![MagicItemEffect::Passive(ActiveEffect::BonusAC(2))],
            "You have a +2 bonus to AC while wearing this armor.",
        ),
        MagicItem::new_with_effects(
            "Armor, +3",
            MagicItemType::Armor,
            Rarity::Legendary,
            Attunement::None,
            None,
            vec![MagicItemEffect::Passive(ActiveEffect::BonusAC(3))],
            "You have a +3 bonus to AC while wearing this armor.",
        ),
        MagicItem::new_with_effects(
            "Arrow-Catching Shield",
            MagicItemType::Armor,
            Rarity::Rare,
            Attunement::Required,
            None,
            vec![
                MagicItemEffect::Passive(ActiveEffect::BonusACAgainstRanged(2)),
                MagicItemEffect::Passive(ActiveEffect::InterceptRangedAttack),
            ],
            "+2 bonus to AC against ranged attacks. You can use a reaction to become the target of a ranged attack that would hit an ally within 5 feet.",
        ),
        MagicItem::new_with_effects(
            "Demon Armor",
            MagicItemType::Armor,
            Rarity::VeryRare,
            Attunement::Required,
            None,
            vec![
                MagicItemEffect::Passive(ActiveEffect::BonusAC(1)),
                MagicItemEffect::Passive(ActiveEffect::LearnLanguage(Language::Abyssal)),
                MagicItemEffect::Passive(ActiveEffect::OverrideUnarmedStrike("1d8".to_string(), DamageType::Slashing)),
                MagicItemEffect::Passive(ActiveEffect::DisadvantageOnAttacksAgainstCreatureType(CreatureType::Fiend)),
                MagicItemEffect::Passive(ActiveEffect::DisadvantageOnSavingThrowsAgainstCreatureType(CreatureType::Fiend)),
            ],
            "+1 bonus to AC, understand Abyssal, unarmed strikes use 1d8 slashing. Cursed: Disadvantage on attacks against demons and saving throws against their spells.",
        ),

        MagicItem::new_with_effects(
            "Dwarven Plate",
            MagicItemType::Armor,
            Rarity::VeryRare,
            Attunement::None,
            None,
            vec![
                MagicItemEffect::Passive(ActiveEffect::BonusAC(2)),
                MagicItemEffect::Passive(ActiveEffect::ReduceForcedMovement(10)),
            ],
            "+2 bonus to AC. If an effect moves you against your will, the distance is reduced by 10 feet.",
        ),
        MagicItem::new_with_effects(
            "Elven Chain",
            MagicItemType::Armor,
            Rarity::Rare,
            Attunement::None,
            None,
            vec![
                MagicItemEffect::Passive(ActiveEffect::BonusAC(1)),
                MagicItemEffect::Passive(ActiveEffect::GrantsArmorProficiency(ArmorCategory::Medium)),
            ],
            "+1 bonus to AC. You are considered proficient with this armor even if you lack proficiency with medium armor.",
        ),
        MagicItem::new_with_effects(
            "Glamoured Studded Leather",
            MagicItemType::Armor,
            Rarity::Rare,
            Attunement::None,
            None,
            vec![
                MagicItemEffect::Passive(ActiveEffect::BonusAC(1)),
                MagicItemEffect::Passive(ActiveEffect::ActionChangeAppearance),
            ],
            "+1 bonus to AC. You can use a bonus action to change the armor's appearance to assume the form of normal clothing or other armor.",
        ),

        MagicItem::new_with_effects(
            "Shield of Missile Attraction",
            MagicItemType::Armor,
            Rarity::Rare,
            Attunement::Required,
            None,
            vec![
                MagicItemEffect::Passive(ActiveEffect::Resistance(vec![DamageType::Piercing])),
                MagicItemEffect::Passive(ActiveEffect::CurseRedirectRangedAttacksToMe(10)),
            ],
            "Resistance to ranged weapon attacks. Cursed: Ranged attacks against targets within 10 feet target you instead.",
        ),
        MagicItem::new_with_effects(
            "Shield, +1",
            MagicItemType::Armor,
            Rarity::Uncommon,
            Attunement::None,
            None,
            vec![MagicItemEffect::Passive(ActiveEffect::BonusAC(1))],
            "+1 bonus to AC.",
        ),
        MagicItem::new_with_effects(
            "Shield, +2",
            MagicItemType::Armor,
            Rarity::Rare,
            Attunement::None,
            None,
            vec![MagicItemEffect::Passive(ActiveEffect::BonusAC(2))],
            "+2 bonus to AC.",
        ),
        MagicItem::new_with_effects(
            "Shield, +3",
            MagicItemType::Armor,
            Rarity::VeryRare,
            Attunement::None,
            None,
            vec![MagicItemEffect::Passive(ActiveEffect::BonusAC(3))],
            "+3 bonus to AC.",
        ),
        MagicItem::new_with_effects(
            "Spellguard Shield",
            MagicItemType::Armor,
            Rarity::VeryRare,
            Attunement::Required,
            None,
            vec![
                MagicItemEffect::Passive(ActiveEffect::AdvantageOnSavingThrowsAgainstSpells),
                MagicItemEffect::Passive(ActiveEffect::DisadvantageOnSpellAttacksAgainstMe),
            ],
            "Advantage on saving throws against spells and magical effects. Spell attacks have disadvantage against you.",
        ),
    ]
}

pub fn create_armor_of_resistance(damage_type: DamageType) -> MagicItem {
    MagicItem::new_with_effects(
        &format!("Armor of Resistance ({:?})", damage_type),
        MagicItemType::Armor,
        Rarity::Rare,
        Attunement::Required,
        None,
        vec![MagicItemEffect::Passive(ActiveEffect::Resistance(vec![
            damage_type,
        ]))],
        &format!("You have resistance to {:?} damage.", damage_type),
    )
}

pub fn create_armor_of_vulnerability(resist_type: DamageType) -> MagicItem {
    let mut vulnerabilities = vec![];
    if resist_type != DamageType::Bludgeoning {
        vulnerabilities.push(DamageType::Bludgeoning);
    }
    if resist_type != DamageType::Piercing {
        vulnerabilities.push(DamageType::Piercing);
    }
    if resist_type != DamageType::Slashing {
        vulnerabilities.push(DamageType::Slashing);
    }

    MagicItem::new_with_effects(
        &format!("Armor of Vulnerability ({:?})", resist_type),
        MagicItemType::Armor,
        Rarity::Rare,
        Attunement::Required,
        None,
        vec![
            MagicItemEffect::Passive(ActiveEffect::Resistance(vec![resist_type])),
            MagicItemEffect::Passive(ActiveEffect::Vulnerability(vulnerabilities)),
        ],
        &format!("You have resistance to {:?} damage and vulnerability to the other two physical damage types. Cursed.", resist_type),
    )
}

pub fn create_dragon_scale_mail(dragon_type: &str, damage_type: DamageType) -> MagicItem {
    MagicItem::new_with_effects(
        &format!("Dragon Scale Mail ({})", dragon_type),
        MagicItemType::Armor,
        Rarity::VeryRare,
        Attunement::Required,
        None,
        vec![
            MagicItemEffect::Passive(ActiveEffect::BonusAC(1)),
            MagicItemEffect::Passive(ActiveEffect::AdvantageAgainstFeature("Frightful Presence and Breath Weapons of Dragons".to_string())),
            MagicItemEffect::Passive(ActiveEffect::Resistance(vec![damage_type])),
        ],
        &format!("+1 bonus to AC, advantage against Frightful Presence and breath weapons of dragons, resistance to {:?} damage.", damage_type),
    )
}

pub fn create_mithral_armor(base_armor: Armor) -> MagicItem {
    MagicItem::new_with_effects(
        &format!("Mithral {}", base_armor.name),
        MagicItemType::Armor,
        Rarity::Uncommon,
        Attunement::None,
        None,
        vec![
            MagicItemEffect::Passive(ActiveEffect::RemoveStealthDisadvantage),
            MagicItemEffect::Passive(ActiveEffect::RemoveStrengthRequirement),
        ],
        "Does not impose disadvantage on Stealth checks. No Strength requirement.",
    )
}
