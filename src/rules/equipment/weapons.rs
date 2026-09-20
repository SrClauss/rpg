use serde::{Deserialize, Serialize};

/// Tipos de dano no D&D 5e
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    Slashing,
    Piercing,
    Bludgeoning,
    Acid,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Poison,
    Psychic,
    Radiant,
    Thunder,
}

/// Categorias de Armas (SRD 5.1 Página 65)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponCategory {
    SimpleMelee,
    SimpleRanged,
    MartialMelee,
    MartialRanged,
}

/// Propriedades Especiais de Armas (SRD 5.1 Página 64-65)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponProperty {
    Finesse,              // Acuidade (pode usar FOR ou DES)
    Heavy,                // Pesada (Desvantagem para criaturas Pequenas)
    Light,                // Leve (Usada para combate com duas armas)
    TwoHanded,            // Duas mãos
    Versatile(String),    // Versátil (dano com duas mãos ex: "1d10")
    Thrown(u32, u32),     // Arremesso (alcance normal, alcance longo)
    Reach,                // Alcance (10 pés)
    Ammunition(u32, u32), // Munição (alcance normal, alcance longo)
    Loading,              // Recarga
    Special,              // Regras especiais (ex: Lança de Montaria / Redes)
}

/// Representa uma Arma Equipável
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Weapon {
    pub id: String,
    pub name: String,
    pub category: WeaponCategory,
    pub damage_dice: String,
    pub damage_type: DamageType,
    pub cost_gp: u32,
    pub weight_lb: u32,
    pub properties: Vec<WeaponProperty>,
}

impl Weapon {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &str,
        name: &str,
        category: WeaponCategory,
        damage_dice: &str,
        damage_type: DamageType,
        cost_gp: u32,
        weight_lb: u32,
        properties: Vec<WeaponProperty>,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            category,
            damage_dice: damage_dice.to_string(),
            damage_type,
            cost_gp,
            weight_lb,
            properties,
        }
    }

    /// Helper para criar um ataque mágico virtual como Arma
    pub fn spell_attack(name: &str, damage_dice: &str, damage_type: DamageType) -> Self {
        Self {
            id: format!("spell_{}", name.to_lowercase().replace(' ', "_")),
            name: name.to_string(),
            category: WeaponCategory::SimpleRanged,
            damage_dice: damage_dice.to_string(),
            damage_type,
            cost_gp: 0,
            weight_lb: 0,
            properties: Vec::new(),
        }
    }

    pub fn has_property(&self, prop: WeaponProperty) -> bool {
        self.properties.contains(&prop)
    }
}

// --- ARMAS SIMPLES DE CORPO A CORPO (10) ---
pub fn create_club() -> Weapon {
    Weapon::new(
        "club",
        "Club",
        WeaponCategory::SimpleMelee,
        "1d4",
        DamageType::Bludgeoning,
        1,
        2,
        vec![WeaponProperty::Light],
    )
}

pub fn create_dagger() -> Weapon {
    Weapon::new(
        "dagger",
        "Dagger",
        WeaponCategory::SimpleMelee,
        "1d4",
        DamageType::Piercing,
        2,
        1,
        vec![
            WeaponProperty::Finesse,
            WeaponProperty::Light,
            WeaponProperty::Thrown(20, 60),
        ],
    )
}

pub fn create_greatclub() -> Weapon {
    Weapon::new(
        "greatclub",
        "Greatclub",
        WeaponCategory::SimpleMelee,
        "1d8",
        DamageType::Bludgeoning,
        1,
        10,
        vec![WeaponProperty::TwoHanded],
    )
}

pub fn create_handaxe() -> Weapon {
    Weapon::new(
        "handaxe",
        "Handaxe",
        WeaponCategory::SimpleMelee,
        "1d6",
        DamageType::Slashing,
        5,
        2,
        vec![WeaponProperty::Light, WeaponProperty::Thrown(20, 60)],
    )
}

pub fn create_javelin() -> Weapon {
    Weapon::new(
        "javelin",
        "Javelin",
        WeaponCategory::SimpleMelee,
        "1d6",
        DamageType::Piercing,
        1,
        2,
        vec![WeaponProperty::Thrown(30, 120)],
    )
}

pub fn create_light_hammer() -> Weapon {
    Weapon::new(
        "light_hammer",
        "Light Hammer",
        WeaponCategory::SimpleMelee,
        "1d4",
        DamageType::Bludgeoning,
        2,
        2,
        vec![WeaponProperty::Light, WeaponProperty::Thrown(20, 60)],
    )
}

pub fn create_mace() -> Weapon {
    Weapon::new(
        "mace",
        "Mace",
        WeaponCategory::SimpleMelee,
        "1d6",
        DamageType::Bludgeoning,
        5,
        4,
        vec![],
    )
}

pub fn create_quarterstaff() -> Weapon {
    Weapon::new(
        "quarterstaff",
        "Quarterstaff",
        WeaponCategory::SimpleMelee,
        "1d6",
        DamageType::Bludgeoning,
        1,
        4,
        vec![WeaponProperty::Versatile("1d8".to_string())],
    )
}

pub fn create_sickle() -> Weapon {
    Weapon::new(
        "sickle",
        "Sickle",
        WeaponCategory::SimpleMelee,
        "1d4",
        DamageType::Slashing,
        1,
        2,
        vec![WeaponProperty::Light],
    )
}

pub fn create_spear() -> Weapon {
    Weapon::new(
        "spear",
        "Spear",
        WeaponCategory::SimpleMelee,
        "1d6",
        DamageType::Piercing,
        1,
        3,
        vec![
            WeaponProperty::Thrown(20, 60),
            WeaponProperty::Versatile("1d8".to_string()),
        ],
    )
}

// --- ARMAS SIMPLES À DISTÂNCIA (4) ---
pub fn create_light_crossbow() -> Weapon {
    Weapon::new(
        "light_crossbow",
        "Crossbow, Light",
        WeaponCategory::SimpleRanged,
        "1d8",
        DamageType::Piercing,
        25,
        5,
        vec![
            WeaponProperty::Ammunition(80, 320),
            WeaponProperty::Loading,
            WeaponProperty::TwoHanded,
        ],
    )
}

pub fn create_dart() -> Weapon {
    Weapon::new(
        "dart",
        "Dart",
        WeaponCategory::SimpleRanged,
        "1d4",
        DamageType::Piercing,
        1,
        1,
        vec![WeaponProperty::Finesse, WeaponProperty::Thrown(20, 60)],
    )
}

pub fn create_shortbow() -> Weapon {
    Weapon::new(
        "shortbow",
        "Shortbow",
        WeaponCategory::SimpleRanged,
        "1d6",
        DamageType::Piercing,
        25,
        2,
        vec![
            WeaponProperty::Ammunition(80, 320),
            WeaponProperty::TwoHanded,
        ],
    )
}

pub fn create_sling() -> Weapon {
    Weapon::new(
        "sling",
        "Sling",
        WeaponCategory::SimpleRanged,
        "1d4",
        DamageType::Bludgeoning,
        1,
        0,
        vec![WeaponProperty::Ammunition(30, 120)],
    )
}

// --- ARMAS MARCIAIS DE CORPO A CORPO (18) ---
pub fn create_battleaxe() -> Weapon {
    Weapon::new(
        "battleaxe",
        "Battleaxe",
        WeaponCategory::MartialMelee,
        "1d8",
        DamageType::Slashing,
        10,
        4,
        vec![WeaponProperty::Versatile("1d10".to_string())],
    )
}

pub fn create_flail() -> Weapon {
    Weapon::new(
        "flail",
        "Flail",
        WeaponCategory::MartialMelee,
        "1d8",
        DamageType::Bludgeoning,
        10,
        2,
        vec![],
    )
}

pub fn create_glaive() -> Weapon {
    Weapon::new(
        "glaive",
        "Glaive",
        WeaponCategory::MartialMelee,
        "1d10",
        DamageType::Slashing,
        20,
        6,
        vec![
            WeaponProperty::Heavy,
            WeaponProperty::Reach,
            WeaponProperty::TwoHanded,
        ],
    )
}

pub fn create_greataxe() -> Weapon {
    Weapon::new(
        "greataxe",
        "Greataxe",
        WeaponCategory::MartialMelee,
        "1d12",
        DamageType::Slashing,
        30,
        7,
        vec![WeaponProperty::Heavy, WeaponProperty::TwoHanded],
    )
}

pub fn create_greatsword() -> Weapon {
    Weapon::new(
        "greatsword",
        "Greatsword",
        WeaponCategory::MartialMelee,
        "2d6",
        DamageType::Slashing,
        50,
        6,
        vec![WeaponProperty::Heavy, WeaponProperty::TwoHanded],
    )
}

pub fn create_halberd() -> Weapon {
    Weapon::new(
        "halberd",
        "Halberd",
        WeaponCategory::MartialMelee,
        "1d10",
        DamageType::Slashing,
        20,
        6,
        vec![
            WeaponProperty::Heavy,
            WeaponProperty::Reach,
            WeaponProperty::TwoHanded,
        ],
    )
}

pub fn create_lance() -> Weapon {
    Weapon::new(
        "lance",
        "Lance",
        WeaponCategory::MartialMelee,
        "1d12",
        DamageType::Piercing,
        10,
        6,
        vec![WeaponProperty::Reach, WeaponProperty::Special],
    )
}

pub fn create_longsword() -> Weapon {
    Weapon::new(
        "longsword",
        "Longsword",
        WeaponCategory::MartialMelee,
        "1d8",
        DamageType::Slashing,
        15,
        3,
        vec![WeaponProperty::Versatile("1d10".to_string())],
    )
}

pub fn create_maul() -> Weapon {
    Weapon::new(
        "maul",
        "Maul",
        WeaponCategory::MartialMelee,
        "2d6",
        DamageType::Bludgeoning,
        10,
        10,
        vec![WeaponProperty::Heavy, WeaponProperty::TwoHanded],
    )
}

pub fn create_morningstar() -> Weapon {
    Weapon::new(
        "morningstar",
        "Morningstar",
        WeaponCategory::MartialMelee,
        "1d8",
        DamageType::Piercing,
        15,
        4,
        vec![],
    )
}

pub fn create_pike() -> Weapon {
    Weapon::new(
        "pike",
        "Pike",
        WeaponCategory::MartialMelee,
        "1d10",
        DamageType::Piercing,
        5,
        18,
        vec![
            WeaponProperty::Heavy,
            WeaponProperty::Reach,
            WeaponProperty::TwoHanded,
        ],
    )
}

pub fn create_rapier() -> Weapon {
    Weapon::new(
        "rapier",
        "Rapier",
        WeaponCategory::MartialMelee,
        "1d8",
        DamageType::Piercing,
        25,
        2,
        vec![WeaponProperty::Finesse],
    )
}

pub fn create_scimitar() -> Weapon {
    Weapon::new(
        "scimitar",
        "Scimitar",
        WeaponCategory::MartialMelee,
        "1d6",
        DamageType::Slashing,
        25,
        3,
        vec![WeaponProperty::Finesse, WeaponProperty::Light],
    )
}

pub fn create_shortsword() -> Weapon {
    Weapon::new(
        "shortsword",
        "Shortsword",
        WeaponCategory::MartialMelee,
        "1d6",
        DamageType::Piercing,
        10,
        2,
        vec![WeaponProperty::Finesse, WeaponProperty::Light],
    )
}

pub fn create_trident() -> Weapon {
    Weapon::new(
        "trident",
        "Trident",
        WeaponCategory::MartialMelee,
        "1d6",
        DamageType::Piercing,
        5,
        4,
        vec![
            WeaponProperty::Thrown(20, 60),
            WeaponProperty::Versatile("1d8".to_string()),
        ],
    )
}

pub fn create_war_pick() -> Weapon {
    Weapon::new(
        "war_pick",
        "War Pick",
        WeaponCategory::MartialMelee,
        "1d8",
        DamageType::Piercing,
        5,
        2,
        vec![],
    )
}

pub fn create_warhammer() -> Weapon {
    Weapon::new(
        "warhammer",
        "Warhammer",
        WeaponCategory::MartialMelee,
        "1d8",
        DamageType::Bludgeoning,
        15,
        2,
        vec![WeaponProperty::Versatile("1d10".to_string())],
    )
}

pub fn create_whip() -> Weapon {
    Weapon::new(
        "whip",
        "Whip",
        WeaponCategory::MartialMelee,
        "1d4",
        DamageType::Slashing,
        2,
        3,
        vec![WeaponProperty::Finesse, WeaponProperty::Reach],
    )
}

// --- ARMAS MARCIAIS À DISTÂNCIA (5) ---
pub fn create_blowgun() -> Weapon {
    Weapon::new(
        "blowgun",
        "Blowgun",
        WeaponCategory::MartialRanged,
        "1",
        DamageType::Piercing,
        10,
        1,
        vec![WeaponProperty::Ammunition(25, 100), WeaponProperty::Loading],
    )
}

pub fn create_hand_crossbow() -> Weapon {
    Weapon::new(
        "hand_crossbow",
        "Crossbow, Hand",
        WeaponCategory::MartialRanged,
        "1d6",
        DamageType::Piercing,
        75,
        3,
        vec![
            WeaponProperty::Ammunition(30, 120),
            WeaponProperty::Light,
            WeaponProperty::Loading,
        ],
    )
}

pub fn create_heavy_crossbow() -> Weapon {
    Weapon::new(
        "heavy_crossbow",
        "Crossbow, Heavy",
        WeaponCategory::MartialRanged,
        "1d10",
        DamageType::Piercing,
        50,
        18,
        vec![
            WeaponProperty::Ammunition(100, 400),
            WeaponProperty::Heavy,
            WeaponProperty::Loading,
            WeaponProperty::TwoHanded,
        ],
    )
}

pub fn create_longbow() -> Weapon {
    Weapon::new(
        "longbow",
        "Longbow",
        WeaponCategory::MartialRanged,
        "1d8",
        DamageType::Piercing,
        50,
        2,
        vec![
            WeaponProperty::Ammunition(150, 600),
            WeaponProperty::Heavy,
            WeaponProperty::TwoHanded,
        ],
    )
}

pub fn create_net() -> Weapon {
    Weapon::new(
        "net",
        "Net",
        WeaponCategory::MartialRanged,
        "0",
        DamageType::Bludgeoning,
        1,
        3,
        vec![WeaponProperty::Special, WeaponProperty::Thrown(5, 15)],
    )
}

/// Retorna todas as 37 armas oficiais do SRD 5.1
pub fn all_srd_weapons() -> Vec<Weapon> {
    vec![
        create_club(),
        create_dagger(),
        create_greatclub(),
        create_handaxe(),
        create_javelin(),
        create_light_hammer(),
        create_mace(),
        create_quarterstaff(),
        create_sickle(),
        create_spear(),
        create_light_crossbow(),
        create_dart(),
        create_shortbow(),
        create_sling(),
        create_battleaxe(),
        create_flail(),
        create_glaive(),
        create_greataxe(),
        create_greatsword(),
        create_halberd(),
        create_lance(),
        create_longsword(),
        create_maul(),
        create_morningstar(),
        create_pike(),
        create_rapier(),
        create_scimitar(),
        create_shortsword(),
        create_trident(),
        create_war_pick(),
        create_warhammer(),
        create_whip(),
        create_blowgun(),
        create_hand_crossbow(),
        create_heavy_crossbow(),
        create_longbow(),
        create_net(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_37_srd_weapons_count() {
        let weapons = all_srd_weapons();
        assert_eq!(weapons.len(), 37);

        let dagger = weapons.iter().find(|w| w.id == "dagger").unwrap();
        assert_eq!(dagger.damage_dice, "1d4");
        assert!(dagger.has_property(WeaponProperty::Finesse));

        let longsword = weapons.iter().find(|w| w.id == "longsword").unwrap();
        assert_eq!(longsword.damage_dice, "1d8");
        assert!(longsword.has_property(WeaponProperty::Versatile("1d10".to_string())));

        let heavy_cb = weapons.iter().find(|w| w.id == "heavy_crossbow").unwrap();
        assert_eq!(heavy_cb.damage_dice, "1d10");
        assert!(heavy_cb.has_property(WeaponProperty::Ammunition(100, 400)));
    }
}
