use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::spells::spell::SpellEffect;
use serde::{Deserialize, Serialize};

pub mod armor;
pub mod potions;
pub mod rings;
pub mod scrolls;
pub mod wands_staffs_rods;
pub mod weapons;
pub mod wondrous_items;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
    Artifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Attunement {
    None,
    Required,
    RequiresClass(String),
    RequiresAlignment(String),
    RequiresSpellcaster,
    RequiresDwarf,
    RequiresElf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MagicItemType {
    Armor,
    Weapon,
    Potion,
    Ring,
    Wand,
    Staff,
    Rod,
    WondrousItem,
    Scroll,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MagicItemEffect {
    /// Efeito passivo constante enquanto equipado/sintonizado
    Passive(ActiveEffect),

    /// O item permite conjurar uma magia específica
    CastSpell {
        spell_name: String,
        cast_level: u8,
        save_dc: Option<i32>,
        attack_bonus: Option<i32>,
    },

    /// Efeito ativado temporário
    Activated {
        effect: ActiveEffect,
        duration: DurationType,
    },

    /// Efeito aplicado quando um ataque com a arma atinge o alvo
    OnWeaponHit(ActiveEffect),

    /// Efeito aplicado on hit similar a uma magia (SpellEffect)
    OnWeaponHitSpell(SpellEffect),

    /// Efeito narrativo / utilitário
    Utility(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MagicItem {
    pub name: String,
    pub item_type: MagicItemType,
    pub rarity: Rarity,
    pub attunement: Attunement,
    pub max_charges: Option<u32>,
    pub effects: Vec<MagicItemEffect>,
    pub description: String,
}

impl MagicItem {
    pub fn new(
        name: &str,
        item_type: MagicItemType,
        rarity: Rarity,
        attunement: Attunement,
        description: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            item_type,
            rarity,
            attunement,
            max_charges: None,
            effects: Vec::new(),
            description: description.to_string(),
        }
    }

    pub fn new_with_effects(
        name: &str,
        item_type: MagicItemType,
        rarity: Rarity,
        attunement: Attunement,
        max_charges: Option<u32>,
        effects: Vec<MagicItemEffect>,
        description: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            item_type,
            rarity,
            attunement,
            max_charges,
            effects,
            description: description.to_string(),
        }
    }
}

pub fn all_srd_magic_items() -> Vec<MagicItem> {
    let mut items = Vec::new();
    items.extend(armor::get_magic_armor());
    items.extend(potions::get_magic_potions());
    items.extend(rings::get_magic_rings());
    items.extend(scrolls::get_magic_scrolls());
    items.extend(wands_staffs_rods::get_magic_wands_staffs_rods());
    items.extend(weapons::get_magic_weapons());
    items.extend(wondrous_items::get_wondrous_items());
    items
}
