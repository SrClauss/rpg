use super::{Attunement, MagicItem, MagicItemEffect, MagicItemType, Rarity};
use crate::rules::character::active_effects::{ActiveEffect, DurationType};

pub fn get_wondrous_items() -> Vec<MagicItem> {
    vec![
        MagicItem::new_with_effects(
            "Horn of Valhalla",
            MagicItemType::WondrousItem,
            Rarity::Rare,
            Attunement::None,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::SummonCreature("Berserker".to_string()),
                duration: DurationType::DurationHours(1),
            }],
            "You can blow this horn to summon berserkers from Ysgard to fight for you for 1 hour.",
        ),
        MagicItem::new_with_effects(
            "Portable Hole",
            MagicItemType::WondrousItem,
            Rarity::Rare,
            Attunement::None,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::GrantItemAction("Open Extradimensional Hole".to_string()),
                duration: DurationType::Instantaneous,
            }],
            "A fine black cloth that when unfolded on a solid surface creates an extradimensional hole 10 feet deep.",
        ),
        MagicItem::new_with_effects(
            "Robe of Eyes",
            MagicItemType::WondrousItem,
            Rarity::Rare,
            Attunement::Required,
            None,
            vec![
                MagicItemEffect::Passive(ActiveEffect::GrantDarkvision(120)),
                MagicItemEffect::Passive(ActiveEffect::SeeInvisible),
                MagicItemEffect::Passive(ActiveEffect::CannotBeSurprised),
            ],
            "You can see in all directions, have darkvision up to 120 feet, and see invisible creatures.",
        ),
        MagicItem::new_with_effects(
            "Rope of Entanglement",
            MagicItemType::WondrousItem,
            Rarity::Rare,
            Attunement::None,
            None,
            vec![MagicItemEffect::Activated {
                effect: ActiveEffect::GrantItemAction("Entangle Creature".to_string()),
                duration: DurationType::DurationMinutes(1),
            }],
            "A 30-foot rope that can be commanded to entangle a creature within 20 feet of you.",
        ),
    ]
}
