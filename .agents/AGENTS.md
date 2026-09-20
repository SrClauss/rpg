# RPG Engine - Workspace Rules for AI Agents

## Magic Items Factories
When generating or outfitting characters with certain dynamic magic items, you must use their specific factory functions instead of searching for them in the standard `get_magic_armor()` catalogs. These items require specific parameters (like base armor types or damage types) to properly hook into the combat engine's `ActiveEffect` mechanics.

Factory functions are located in `src/rules/equipment/magic_items/armor.rs`:
- `create_armor_of_resistance(damage_type: DamageType) -> MagicItem`
- `create_armor_of_vulnerability(resist_type: DamageType) -> MagicItem`
- `create_dragon_scale_mail(dragon_type: &str, damage_type: DamageType) -> MagicItem`
- `create_mithral_armor(base_armor: Armor) -> MagicItem`
