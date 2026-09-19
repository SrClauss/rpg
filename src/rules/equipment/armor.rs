use serde::{Deserialize, Serialize};

/// Categoria da Armadura
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmorCategory {
    Light,
    Medium,
    Heavy,
    Shield,
}

/// Representa uma Armadura ou Escudo Equipável (SRD 5.1 Página 63)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Armor {
    pub id: String,
    pub name: String,
    pub category: ArmorCategory,
    pub base_ac: i32,
    pub cost_gp: u32,
    pub weight_lb: u32,
    pub has_dex_limit: bool,
    pub dex_limit_max: i32,
    pub stealth_disadvantage: bool,
    pub strength_requirement: i32,
}

impl Armor {
    pub fn new(
        id: &str,
        name: &str,
        category: ArmorCategory,
        base_ac: i32,
        cost_gp: u32,
        weight_lb: u32,
        has_dex_limit: bool,
        dex_limit_max: i32,
        stealth_disadvantage: bool,
        strength_requirement: i32,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            category,
            base_ac,
            cost_gp,
            weight_lb,
            has_dex_limit,
            dex_limit_max,
            stealth_disadvantage,
            strength_requirement,
        }
    }
}

// --- ARMADURAS LEVES ---
pub fn create_padded_armor() -> Armor {
    Armor::new("padded", "Padded Armor", ArmorCategory::Light, 11, 5, 8, false, 0, true, 0)
}

pub fn create_leather_armor() -> Armor {
    Armor::new("leather", "Leather Armor", ArmorCategory::Light, 11, 10, 10, false, 0, false, 0)
}

pub fn create_studded_leather_armor() -> Armor {
    Armor::new("studded_leather", "Studded Leather Armor", ArmorCategory::Light, 12, 45, 13, false, 0, false, 0)
}

// --- ARMADURAS MÉDIAS ---
pub fn create_hide_armor() -> Armor {
    Armor::new("hide", "Hide Armor", ArmorCategory::Medium, 12, 10, 12, true, 2, false, 0)
}

pub fn create_chain_shirt() -> Armor {
    Armor::new("chain_shirt", "Chain Shirt", ArmorCategory::Medium, 13, 50, 20, true, 2, false, 0)
}

pub fn create_scale_mail() -> Armor {
    Armor::new("scale_mail", "Scale Mail", ArmorCategory::Medium, 14, 50, 45, true, 2, true, 0)
}

pub fn create_breastplate() -> Armor {
    Armor::new("breastplate", "Breastplate", ArmorCategory::Medium, 14, 400, 20, true, 2, false, 0)
}

pub fn create_half_plate() -> Armor {
    Armor::new("half_plate", "Half Plate Armor", ArmorCategory::Medium, 15, 750, 40, true, 2, true, 0)
}

// --- ARMADURAS PESADAS ---
pub fn create_ring_mail() -> Armor {
    Armor::new("ring_mail", "Ring Mail", ArmorCategory::Heavy, 14, 30, 40, true, 0, true, 0)
}

pub fn create_chain_mail() -> Armor {
    Armor::new("chain_mail", "Chain Mail", ArmorCategory::Heavy, 16, 75, 55, true, 0, true, 13)
}

pub fn create_splint_armor() -> Armor {
    Armor::new("splint", "Splint Armor", ArmorCategory::Heavy, 17, 200, 60, true, 0, true, 15)
}

pub fn create_plate_armor() -> Armor {
    Armor::new("plate", "Plate Armor", ArmorCategory::Heavy, 18, 1500, 65, true, 0, true, 15)
}

// --- ESCUDO ---
pub fn create_shield() -> Armor {
    Armor::new("shield", "Shield", ArmorCategory::Shield, 2, 10, 6, false, 0, false, 0)
}

/// Retorna todas as 13 armaduras oficiais do SRD 5.1
pub fn all_srd_armors() -> Vec<Armor> {
    vec![
        create_padded_armor(),
        create_leather_armor(),
        create_studded_leather_armor(),
        create_hide_armor(),
        create_chain_shirt(),
        create_scale_mail(),
        create_breastplate(),
        create_half_plate(),
        create_ring_mail(),
        create_chain_mail(),
        create_splint_armor(),
        create_plate_armor(),
        create_shield(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_13_srd_armors_count_and_properties() {
        let armors = all_srd_armors();
        assert_eq!(armors.len(), 13);

        let plate = armors.iter().find(|a| a.id == "plate").unwrap();
        assert_eq!(plate.base_ac, 18);
        assert_eq!(plate.strength_requirement, 15);
        assert!(plate.stealth_disadvantage);

        let leather = armors.iter().find(|a| a.id == "leather").unwrap();
        assert_eq!(leather.base_ac, 11);
        assert!(!leather.stealth_disadvantage);

        let shield = armors.iter().find(|a| a.id == "shield").unwrap();
        assert_eq!(shield.base_ac, 2);
    }
}
