use super::{Attunement, MagicItem, MagicItemType, Rarity};

pub fn get_magic_scrolls() -> Vec<MagicItem> {
    vec![
        MagicItem::new(
            "Spell Scroll",
            MagicItemType::Scroll,
            Rarity::Common, // Rarity varies from Common (Cantrip) to Legendary (9th level)
            Attunement::None,
            "A spell scroll bears the words of a single spell. If the spell is on your class's spell list, you can read the scroll and cast the spell without providing material components.",
        ),
    ]
}
