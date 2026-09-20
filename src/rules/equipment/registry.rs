use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;

use super::armor::{all_srd_armors, Armor};
use super::magic_items::{all_srd_magic_items, MagicItem};
use super::weapons::{all_srd_weapons, Weapon};

/// Registro Global Estático de Equipamentos (SRD 5.1 e Homebrew)
pub static GLOBAL_EQUIPMENT: LazyLock<EquipmentRegistry> = LazyLock::new(|| {
    let mut reg = EquipmentRegistry::new();
    reg.load_srd_defaults();
    reg
});

/// Catálogo Central de Armas e Armaduras
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EquipmentRegistry {
    weapons: HashMap<String, Weapon>,
    armors: HashMap<String, Armor>,
    magic_items: HashMap<String, MagicItem>,
}

impl EquipmentRegistry {
    pub fn new() -> Self {
        Self {
            weapons: HashMap::new(),
            armors: HashMap::new(),
            magic_items: HashMap::new(),
        }
    }

    /// Carrega as 37 armas e 13 armaduras padrão do SRD 5.1
    pub fn load_srd_defaults(&mut self) {
        for armor in all_srd_armors() {
            self.armors.insert(armor.id.clone(), armor);
        }
        for weapon in all_srd_weapons() {
            self.weapons.insert(weapon.id.clone(), weapon);
        }
        for item in all_srd_magic_items() {
            // Using lowercase name as ID for magic items
            let id = item.name.to_lowercase().replace(" ", "_").replace(",", "");
            self.magic_items.insert(id, item);
        }
    }

    /// Registra uma nova arma no catálogo
    pub fn register_weapon(&mut self, weapon: Weapon) {
        self.weapons.insert(weapon.id.clone(), weapon);
    }

    /// Registra uma nova armadura no catálogo
    pub fn register_armor(&mut self, armor: Armor) {
        self.armors.insert(armor.id.clone(), armor);
    }

    /// Busca uma arma no registro global pelo ID (ex: "lance", "longsword")
    pub fn get_weapon(id: &str) -> Option<Weapon> {
        GLOBAL_EQUIPMENT.weapons.get(id).cloned()
    }

    /// Busca uma armadura no registro global pelo ID (ex: "plate", "leather")
    pub fn get_armor(id: &str) -> Option<Armor> {
        GLOBAL_EQUIPMENT.armors.get(id).cloned()
    }

    /// Busca um item mágico no registro global pelo ID
    pub fn get_magic_item(id: &str) -> Option<MagicItem> {
        GLOBAL_EQUIPMENT.magic_items.get(id).cloned()
    }

    /// Retorna todos os IDs de armas registradas
    pub fn list_weapon_ids() -> Vec<String> {
        GLOBAL_EQUIPMENT.weapons.keys().cloned().collect()
    }

    /// Retorna todos os IDs de armaduras registradas
    pub fn list_armor_ids() -> Vec<String> {
        GLOBAL_EQUIPMENT.armors.keys().cloned().collect()
    }

    /// Retorna todos os IDs de itens mágicos registrados
    pub fn list_magic_item_ids() -> Vec<String> {
        GLOBAL_EQUIPMENT.magic_items.keys().cloned().collect()
    }

    /// Carrega armas homebrew a partir de uma string JSON
    pub fn load_homebrew_weapons(&mut self, json_str: &str) -> Result<usize, String> {
        let list: Vec<Weapon> = serde_json::from_str(json_str)
            .map_err(|e| format!("Erro ao desserializar armas homebrew: {}", e))?;
        let count = list.len();
        for weapon in list {
            self.register_weapon(weapon);
        }
        Ok(count)
    }

    /// Carrega armaduras homebrew a partir de uma string JSON
    pub fn load_homebrew_armors(&mut self, json_str: &str) -> Result<usize, String> {
        let list: Vec<Armor> = serde_json::from_str(json_str)
            .map_err(|e| format!("Erro ao desserializar armaduras homebrew: {}", e))?;
        let count = list.len();
        for armor in list {
            self.register_armor(armor);
        }
        Ok(count)
    }

    /// Carrega armas Homebrew a partir de uma pasta contendo scripts .hb (Rhai)
    pub fn load_rhai_homebrew_weapons(&mut self, dir_path: &str) -> usize {
        let weapons: Vec<Weapon> =
            crate::rules::homebrew_loader::load_homebrew_from_dir(dir_path, "get_weapon");
        let count = weapons.len();
        for weapon in weapons {
            self.register_weapon(weapon);
        }
        count
    }

    /// Carrega armaduras Homebrew a partir de uma pasta contendo scripts .hb (Rhai)
    pub fn load_rhai_homebrew_armors(&mut self, dir_path: &str) -> usize {
        let armors: Vec<Armor> =
            crate::rules::homebrew_loader::load_homebrew_from_dir(dir_path, "get_armor");
        let count = armors.len();
        for armor in armors {
            self.register_armor(armor);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_equipment_registry_srd_defaults() {
        // Testar busca estática de armas por ID no recurso global
        let longsword = EquipmentRegistry::get_weapon("longsword");
        assert!(longsword.is_some());
        let dagger = EquipmentRegistry::get_weapon("dagger");
        assert!(dagger.is_some());
        let lance = EquipmentRegistry::get_weapon("lance");
        assert!(lance.is_some());

        // Testar busca estática de armaduras por ID no recurso global
        let plate = EquipmentRegistry::get_armor("plate");
        assert!(plate.is_some());
        let shield = EquipmentRegistry::get_armor("shield");
        assert!(shield.is_some());

        // Testar contagem de IDs
        assert_eq!(EquipmentRegistry::list_weapon_ids().len(), 37);
        assert_eq!(EquipmentRegistry::list_armor_ids().len(), 13);
    }

    #[test]
    fn test_global_equipment_registry_magic_items() {
        let magic_items = EquipmentRegistry::list_magic_item_ids();
        assert!(magic_items.len() > 50);

        let adamantine = EquipmentRegistry::get_magic_item("adamantine_armor");
        assert!(adamantine.is_some());

        let ammo = EquipmentRegistry::get_magic_item("ammunition_+1");
        assert!(ammo.is_some());
        assert_eq!(ammo.unwrap().name, "Ammunition, +1");
    }

    #[test]
    fn test_homebrew_weapon_json_loading() {
        let mut registry = EquipmentRegistry::new();
        registry.load_srd_defaults();

        let homebrew_json = r#"[
            {
                "id": "laser_sword",
                "name": "Laser Sword",
                "category": "MartialMelee",
                "damage_dice": "2d8",
                "damage_type": "Radiant",
                "cost_gp": 500,
                "weight_lb": 2,
                "properties": ["Finesse", "Light"]
            }
        ]"#;

        let res = registry.load_homebrew_weapons(homebrew_json);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1);

        let laser_sword = registry.weapons.get("laser_sword");
        assert!(laser_sword.is_some());
        let item = laser_sword.unwrap();
        assert_eq!(item.name, "Laser Sword");
        assert_eq!(item.damage_dice, "2d8");
    }
}
