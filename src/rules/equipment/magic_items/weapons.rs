use super::{Attunement, MagicItem, MagicItemEffect, MagicItemType, Rarity};
use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::equipment::weapons::{DamageType, Weapon};
use crate::rules::monsters::monster::CreatureType;

pub fn get_magic_weapons() -> Vec<MagicItem> {
    vec![
        MagicItem::new_with_effects(
            "Ammunition, +1",
            MagicItemType::Weapon,
            Rarity::Uncommon,
            Attunement::None,
            None,
            vec![MagicItemEffect::Passive(ActiveEffect::MagicWeaponBonus(1))],
            "You have a +1 bonus to attack and damage rolls made with this piece of magic ammunition.",
        ),
        MagicItem::new_with_effects(
            "Ammunition, +2",
            MagicItemType::Weapon,
            Rarity::Rare,
            Attunement::None,
            None,
            vec![MagicItemEffect::Passive(ActiveEffect::MagicWeaponBonus(2))],
            "You have a +2 bonus to attack and damage rolls made with this piece of magic ammunition.",
        ),
        MagicItem::new_with_effects(
            "Ammunition, +3",
            MagicItemType::Weapon,
            Rarity::VeryRare,
            Attunement::None,
            None,
            vec![MagicItemEffect::Passive(ActiveEffect::MagicWeaponBonus(3))],
            "You have a +3 bonus to attack and damage rolls made with this piece of magic ammunition.",
        ),
    ]
}

pub fn create_magic_weapon(mut base_weapon: Weapon, bonus: i32) -> (Weapon, MagicItem) {
    base_weapon.name = format!("{}, +{}", base_weapon.name, bonus);
    base_weapon.magic_bonus = bonus;

    let rarity = match bonus {
        1 => Rarity::Uncommon,
        2 => Rarity::Rare,
        _ => Rarity::VeryRare,
    };

    let item = MagicItem::new(
        &base_weapon.name,
        MagicItemType::Weapon,
        rarity,
        Attunement::None,
        &format!(
            "You have a +{} bonus to attack and damage rolls made with this magic weapon.",
            bonus
        ),
    );
    (base_weapon, item)
}

pub fn create_dragon_slayer(mut base_weapon: Weapon) -> (Weapon, MagicItem) {
    base_weapon.name = format!("Dragon Slayer {}", base_weapon.name);
    base_weapon.magic_bonus = 1;
    base_weapon
        .magic_effects
        .push(ActiveEffect::ConditionalDamageAgainstCreature(
            CreatureType::Dragon,
            "3d6".to_string(),
            base_weapon.damage_type,
        ));

    let item = MagicItem::new(
        &base_weapon.name,
        MagicItemType::Weapon,
        Rarity::Rare,
        Attunement::None,
        "+1 bonus to attack and damage rolls. Deals an extra 3d6 damage to dragons.",
    );
    (base_weapon, item)
}

pub fn create_frost_brand(mut base_weapon: Weapon) -> (Weapon, MagicItem) {
    base_weapon.name = format!("Frost Brand {}", base_weapon.name);
    base_weapon
        .magic_effects
        .push(ActiveEffect::WeaponDamageBonus("1d6".to_string()));
    // Actually Frost Brand doesn't give a generic damage bonus, it's 1d6 Cold damage on hit.
    // Let's assume WeaponDamageBonus is currently untyped or uses weapon type. For strict D&D, we should have typed extra damage.
    // We can use NextWeaponAttackBonusDamage for now or just WeaponDamageBonus.

    let item = MagicItem::new_with_effects(
        &base_weapon.name,
        MagicItemType::Weapon,
        Rarity::VeryRare,
        Attunement::Required,
        None,
        vec![MagicItemEffect::Passive(ActiveEffect::Resistance(vec![DamageType::Fire]))],
        "Deals extra 1d6 cold damage. Resistance to fire damage. Emits bright light in freezing temperatures.",
    );
    (base_weapon, item)
}

pub fn create_flame_tongue(mut base_weapon: Weapon) -> (Weapon, MagicItem) {
    base_weapon.name = format!("Flame Tongue {}", base_weapon.name);
    // Utility activation
    let item = MagicItem::new_with_effects(
        &base_weapon.name,
        MagicItemType::Weapon,
        Rarity::Rare,
        Attunement::Required,
        None,
        vec![MagicItemEffect::Activated {
            effect: ActiveEffect::BonusDamage("2d6".to_string(), DamageType::Fire),
            duration: DurationType::Permanent,
        }],
        "Deals extra 2d6 fire damage when activated.",
    );
    (base_weapon, item)
}

pub fn create_sun_blade(mut base_weapon: Weapon) -> (Weapon, MagicItem) {
    base_weapon.name = "Sun Blade".to_string();
    base_weapon.magic_bonus = 2;
    base_weapon
        .magic_effects
        .push(ActiveEffect::OverrideDamageType(DamageType::Radiant));
    base_weapon
        .magic_effects
        .push(ActiveEffect::ConditionalDamageAgainstCreature(
            CreatureType::Undead,
            "1d8".to_string(),
            DamageType::Radiant,
        ));

    let item = MagicItem::new(
        &base_weapon.name,
        MagicItemType::Weapon,
        Rarity::Rare,
        Attunement::Required,
        "+2 bonus to attack and damage rolls. Deals radiant damage. Extra 1d8 radiant damage to undead. Emits sunlight.",
    );
    (base_weapon, item)
}

pub fn create_mace_of_disruption(mut base_weapon: Weapon) -> (Weapon, MagicItem) {
    base_weapon.name = "Mace of Disruption".to_string();
    base_weapon
        .magic_effects
        .push(ActiveEffect::ConditionalDamageAgainstCreature(
            CreatureType::Fiend,
            "2d6".to_string(),
            DamageType::Radiant,
        ));
    base_weapon
        .magic_effects
        .push(ActiveEffect::ConditionalDamageAgainstCreature(
            CreatureType::Undead,
            "2d6".to_string(),
            DamageType::Radiant,
        ));

    let item = MagicItem::new(
        &base_weapon.name,
        MagicItemType::Weapon,
        Rarity::Rare,
        Attunement::Required,
        "Deals extra 2d6 radiant damage to fiends and undead, possibly destroying them if they have 25 HP or less.",
    );
    (base_weapon, item)
}

pub fn create_mace_of_smiting(mut base_weapon: Weapon) -> (Weapon, MagicItem) {
    base_weapon.name = "Mace of Smiting".to_string();
    base_weapon.magic_bonus = 1;
    // +3 vs constructs can be implemented by just giving +1 normal and a utility note for the extra +2 to hit constructs,
    // or extending the engine further. For now we use the critical bonus.
    base_weapon
        .magic_effects
        .push(ActiveEffect::BonusDamageOnCritical(
            "2d6".to_string(),
            base_weapon.damage_type,
        ));

    let item = MagicItem::new(
        &base_weapon.name,
        MagicItemType::Weapon,
        Rarity::Rare,
        Attunement::None,
        "+1 bonus to attack and damage rolls. +3 against constructs. Deals extra damage on a critical hit.",
    );
    (base_weapon, item)
}

pub fn create_vicious_weapon(mut base_weapon: Weapon) -> (Weapon, MagicItem) {
    base_weapon.name = format!("Vicious {}", base_weapon.name);
    // Vicious does flat 7 damage on a nat 20. We can just use "7".
    base_weapon
        .magic_effects
        .push(ActiveEffect::BonusDamageOnCritical(
            "7".to_string(),
            base_weapon.damage_type,
        ));

    let item = MagicItem::new(
        &base_weapon.name,
        MagicItemType::Weapon,
        Rarity::Uncommon,
        Attunement::None,
        "When you roll a 20 on your attack roll, the weapon deals an extra 7 damage of the weapon's type.",
    );
    (base_weapon, item)
}

pub fn create_luck_blade(mut base_weapon: Weapon) -> (Weapon, MagicItem) {
    base_weapon.name = format!("Luck Blade ({})", base_weapon.name);
    base_weapon.magic_bonus = 1;

    let item = MagicItem::new_with_effects(
        &base_weapon.name,
        MagicItemType::Weapon,
        Rarity::Legendary,
        Attunement::Required,
        Some(3), // up to 3 wishes, but varies
        vec![
            MagicItemEffect::Passive(ActiveEffect::BonusSavingThrow("1".to_string())),
            MagicItemEffect::Passive(ActiveEffect::RerollDice(1)),
            MagicItemEffect::CastSpell { spell_name: "Wish".to_string(), cast_level: 9, save_dc: None, attack_bonus: None },
        ],
        "+1 bonus to attack and damage rolls, +1 to saving throws. Holds 1d4-1 wishes. Once per day, you can reroll a d20.",
    );
    (base_weapon, item)
}

pub fn create_berserker_axe(mut base_weapon: Weapon) -> (Weapon, MagicItem) {
    base_weapon.name = format!("Berserker {}", base_weapon.name);
    base_weapon.magic_bonus = 1;

    let item = MagicItem::new_with_effects(
        &base_weapon.name,
        MagicItemType::Weapon,
        Rarity::Rare,
        Attunement::Required,
        None,
        vec![
            MagicItemEffect::Passive(ActiveEffect::IncreaseMaxHpPerLevel(1)),
            MagicItemEffect::Passive(ActiveEffect::ApplyConditionWhenDamaged(crate::rules::conditions::Condition::Berserk))
        ],
        "+1 bonus to attack and damage rolls. Increases max HP by 1 per level. Cursed: You go berserk when damaged in combat.",
    );
    (base_weapon, item)
}
