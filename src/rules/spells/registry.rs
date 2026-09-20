use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;

use super::spell::{
    create_cure_wounds, create_fire_bolt, create_fireball, Components, Spell, SpellEffect,
    SpellSchool,
};
use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;

/// Registro Global Estático de Magias (SRD 5.1 e Homebrew)
pub static GLOBAL_SPELLS: LazyLock<SpellRegistry> = LazyLock::new(|| {
    let mut reg = SpellRegistry::new();
    reg.load_default_srd_spells();
    reg
});

/// Catálogo Central de Magias
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpellRegistry {
    spells: HashMap<String, Spell>,
}

impl SpellRegistry {
    pub fn new() -> Self {
        Self {
            spells: HashMap::new(),
        }
    }

    /// Registra as magias iniciais do SRD 5.1
    pub fn load_default_srd_spells(&mut self) {
        crate::rules::spells::srd_cantrips::load_cantrips(self);
        crate::rules::spells::srd_level1::load_level1_spells(self);
        crate::rules::spells::srd_level2::load_level2_spells(self);
        crate::rules::spells::srd_level3::load_level3_spells(self);
        crate::rules::spells::srd_level4::load_level4_spells(self);
        crate::rules::spells::srd_level5::load_level5_spells(self);
        crate::rules::spells::srd_level6::load_level6_spells(self);
        crate::rules::spells::srd_level7::load_level7_spells(self);
        crate::rules::spells::srd_level8::load_level8_spells(self);
        crate::rules::spells::srd_level9::load_level9_spells(self);

        // Mantendo magias antigas para compatibilidade temporária
        self.register_spell(create_fire_bolt());
        self.register_spell(create_cure_wounds());
        self.register_spell(create_fireball());
    }

    /// Registra uma magia no catálogo (normaliza a chave em minúsculas)
    pub fn register_spell(&mut self, spell: Spell) {
        let key = spell.name.to_lowercase().replace(' ', "_");
        self.spells.insert(key, spell);
    }

    /// Busca uma magia global pelo nome/id
    pub fn get_spell(name: &str) -> Option<Spell> {
        let key = name.to_lowercase().replace(' ', "_");
        GLOBAL_SPELLS.spells.get(&key).cloned()
    }

    /// Retorna todos os nomes de magias registradas
    pub fn list_spell_names() -> Vec<String> {
        GLOBAL_SPELLS
            .spells
            .values()
            .map(|s| s.name.clone())
            .collect()
    }

    /// Carrega magias Homebrew a partir de uma string JSON
    pub fn load_homebrew_spells(&mut self, json_str: &str) -> Result<usize, String> {
        let list: Vec<Spell> = serde_json::from_str(json_str)
            .map_err(|e| format!("Erro ao desserializar magias homebrew: {}", e))?;
        let count = list.len();
        for spell in list {
            self.register_spell(spell);
        }
        Ok(count)
    }

    /// Carrega magias Homebrew a partir de uma pasta contendo scripts .hb (Rhai)
    pub fn load_rhai_homebrew_spells(&mut self, dir_path: &str) -> usize {
        let spells: Vec<Spell> = crate::rules::homebrew_loader::load_homebrew_from_dir(dir_path, "get_spell");
        let count = spells.len();
        for spell in spells {
            self.register_spell(spell);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_spells_srd() {
        let firebolt = SpellRegistry::get_spell("Fire Bolt");
        assert!(firebolt.is_some());
        assert_eq!(firebolt.unwrap().level, 0);

        let cure = SpellRegistry::get_spell("cure_wounds");
        assert!(cure.is_some());

        let fireball = SpellRegistry::get_spell("Fireball");
        assert!(fireball.is_some());
        assert_eq!(fireball.unwrap().level, 3);
    }

    #[test]
    fn test_load_homebrew_spells_json() {
        let mut registry = SpellRegistry::new();
        registry.load_default_srd_spells();

        let json_data = r#"[
            {
                "name": "Chuva de Asteroides Arcanos",
                "level": 4,
                "school": "Evocation",
                "casting_time": "1 action",
                "range_feet": 120,
                "components": {
                    "verbal": true,
                    "somatic": true,
                    "material": "Um fragmento de meteorito"
                },
                "duration": "Instantaneous",
                "requires_concentration": false,
                "is_ritual": false,
                "effect": {
                    "SaveThrow": {
                        "save_ability": "Dexterity",
                        "half_damage_on_save": true,
                        "damage_dice": "6d10",
                        "damage_type": "Fire",
                        "damage_scaling": null,
                        "fail_effect": null
                    }
                }
            }
        ]"#;

        let res = registry.load_homebrew_spells(json_data);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1);

        let spell = registry.spells.get("chuva_de_asteroides_arcanos");
        assert!(spell.is_some());
        assert_eq!(spell.unwrap().level, 4);
    }
}
