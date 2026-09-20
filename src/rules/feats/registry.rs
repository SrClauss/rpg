use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;

use super::feat::{Feat, FeatMechanicalEffect, FeatPrerequisites};
use crate::rules::character::entity::Entity;
use crate::rules::equipment::validator::{HomebrewValidator, ValidationReport};

/// Registro Global Estático de Talentos (SRD 5.1, 5e e Homebrew)
pub static GLOBAL_FEATS: LazyLock<FeatRegistry> = LazyLock::new(|| {
    let mut reg = FeatRegistry::new();
    reg.load_default_5e_feats();
    reg
});

/// Catálogo Central de Talentos
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatRegistry {
    feats: HashMap<String, Feat>,
}

impl FeatRegistry {
    pub fn new() -> Self {
        Self {
            feats: HashMap::new(),
        }
    }

    /// Registra os talentos padrão do 5e (Grappler, Alert, Sharpshooter, War Caster, Sentinel, Tough, Lucky, etc.)
    pub fn load_default_5e_feats(&mut self) {
        // 1. Grappler (SRD 5.1 Oficial)
        self.register_feat(Feat::new_mechanical(
            "grappler",
            "Grappler",
            "Você tem vantagem em jogadas de ataque contra uma criatura que esteja agarrando. Pode usar sua ação para imobilizar (Pin) a criatura agarrada, deixando ambos imobilizados (Restrained).",
            FeatPrerequisites {
                min_strength: Some(13),
                ..Default::default()
            },
            FeatMechanicalEffect {
                advantage_against_grappled: true,
                can_pin_grappled_target: true,
                ..Default::default()
            },
        ));

        // 2. Alert (Alerta)
        self.register_feat(Feat::new_mechanical(
            "alert",
            "Alert",
            "Você ganha +5 de bônus em rolagens de iniciativa, não pode ser surpreendido enquanto consciente e inimigos escondidos não ganham vantagem contra você.",
            FeatPrerequisites::default(),
            FeatMechanicalEffect {
                initiative_bonus: 5,
                cannot_be_surprised: true,
                denies_advantage_from_hidden_attackers: true,
                ..Default::default()
            },
        ));

        // 3. Sharpshooter (Atirador de Elite)
        self.register_feat(Feat::new_mechanical(
            "sharpshooter",
            "Sharpshooter",
            "Seus ataques à distância ignoram cobertura meia (CoverHalf) e três quartos (CoverThreeQuarters).",
            FeatPrerequisites::default(),
            FeatMechanicalEffect {
                ignore_half_cover: true,
                ignore_three_quarters_cover: true,
                ..Default::default()
            },
        ));

        // 4. Tough (Robusto)
        self.register_feat(Feat::new_mechanical(
            "tough",
            "Tough",
            "Seu máximo de pontos de vida aumenta em um valor igual ao dobro do seu nível quando você adquire este talento, e +2 PV adicionais a cada nível futuro.",
            FeatPrerequisites::default(),
            FeatMechanicalEffect {
                hp_per_level_bonus: 2,
                ..Default::default()
            },
        ));

        // 5. War Caster (Conjurador de Guerra)
        self.register_feat(Feat::new_mechanical(
            "war_caster",
            "War Caster",
            "Você tem vantagem em testes de resistência de Constituição para manter a concentração em uma magia quando sofre dano.",
            FeatPrerequisites {
                requires_spellcasting: true,
                ..Default::default()
            },
            FeatMechanicalEffect::default(),
        ));

        // 6. Sentinel (Sentinela)
        self.register_feat(Feat::new_mechanical(
            "sentinel",
            "Sentinel",
            "Quando você atinge uma criatura com um ataque de oportunidade, o deslocamento dela torna-se 0 até o final do turno.",
            FeatPrerequisites::default(),
            FeatMechanicalEffect::default(),
        ));

        // 7. Great Weapon Master (Mestre em Armas Grandes)
        self.register_feat(Feat::new_mechanical(
            "great_weapon_master",
            "Great Weapon Master",
            "Quando você obtém um acerto crítico com uma arma corpo a corpo ou reduz os PVs de uma criatura a 0, você pode fazer um ataque corpo a corpo adicional como ação bônus.",
            FeatPrerequisites::default(),
            FeatMechanicalEffect::default(),
        ));

        // 8. Mobile (Móvel)
        self.register_feat(Feat::new_mechanical(
            "mobile",
            "Mobile",
            "Seu deslocamento aumenta em 10 pés (3m). Quando você realiza uma ação de Corrida (Dash), terreno difícil não custa movimento extra.",
            FeatPrerequisites::default(),
            FeatMechanicalEffect::default(),
        ));

        // 9. Lucky (Sortudo)
        self.register_feat(Feat::new_mechanical(
            "lucky",
            "Lucky",
            "Você tem 3 pontos de sorte por dia para rolar d20 adicional em testes, ataques ou salvaguardas.",
            FeatPrerequisites::default(),
            FeatMechanicalEffect::default(),
        ));

        // 10. Resilient (Resiliente)
        self.register_feat(Feat::new_mechanical(
            "resilient",
            "Resilient",
            "Você ganha proficiência em rolagens de salvaguarda de um atributo à sua escolha.",
            FeatPrerequisites::default(),
            FeatMechanicalEffect::default(),
        ));
    }

    /// Registra um novo talento no catálogo
    pub fn register_feat(&mut self, feat: Feat) {
        self.feats.insert(feat.id.clone(), feat);
    }

    /// Busca um talento global pelo ID
    pub fn get_feat(id: &str) -> Option<Feat> {
        GLOBAL_FEATS.feats.get(id).cloned()
    }

    /// Retorna todos os IDs de talentos registrados
    pub fn list_feat_ids() -> Vec<String> {
        GLOBAL_FEATS.feats.keys().cloned().collect()
    }

    /// Carrega talentos Homebrew a partir de uma string JSON
    pub fn load_homebrew_feats(&mut self, json_str: &str) -> Result<usize, String> {
        let list: Vec<Feat> = serde_json::from_str(json_str)
            .map_err(|e| format!("Erro ao desserializar talentos homebrew: {}", e))?;
        let count = list.len();
        for feat in list {
            self.register_feat(feat);
        }
        Ok(count)
    }

    /// Valida se uma entidade atende aos pré-requisitos para adquirir um talento
    pub fn validate_prerequisites(feat: &Feat, entity: &Entity) -> ValidationReport {
        let warnings = Vec::new();
        let mut errors = Vec::new();

        if let Some(min_str) = feat.prerequisites.min_strength {
            if entity.attributes.strength.0 < min_str {
                errors.push(format!(
                    "Requer Força mínima {} (Atual: {}).",
                    min_str, entity.attributes.strength.0
                ));
            }
        }

        if let Some(min_dex) = feat.prerequisites.min_dexterity {
            if entity.attributes.dexterity.0 < min_dex {
                errors.push(format!(
                    "Requer Destreza mínima {} (Atual: {}).",
                    min_dex, entity.attributes.dexterity.0
                ));
            }
        }

        ValidationReport {
            is_valid: errors.is_empty(),
            item_id: feat.id.clone(),
            item_name: feat.name.clone(),
            validated_effect: feat.effect.clone(),
            warnings,
            errors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_feats_srd_and_5e() {
        let grappler = FeatRegistry::get_feat("grappler");
        assert!(grappler.is_some());
        assert_eq!(grappler.unwrap().name, "Grappler");

        let alert = FeatRegistry::get_feat("alert");
        assert!(alert.is_some());
        let alert_feat = alert.unwrap();
        assert_eq!(alert_feat.mechanical_details.unwrap().initiative_bonus, 5);

        let sharpshooter = FeatRegistry::get_feat("sharpshooter");
        assert!(sharpshooter.is_some());
    }

    #[test]
    fn test_load_homebrew_feats_json() {
        let mut registry = FeatRegistry::new();
        registry.load_default_5e_feats();

        let json_data = r#"[
            {
                "id": "telepata_corvos",
                "name": "Telepata de Corvos",
                "description": "O personagem consegue se comunicar mentalmente com corvos a até 1km de distância.",
                "prerequisites": {
                    "min_strength": null,
                    "min_dexterity": null,
                    "min_constitution": null,
                    "min_intelligence": 13,
                    "min_wisdom": null,
                    "min_charisma": null,
                    "requires_spellcasting": false,
                    "requires_armor_proficiency": null
                },
                "effect": {
                    "NarrativeCustom": {
                        "description": "Permite se comunicar mentalmente com corvos e ver através dos olhos deles por 1 minuto."
                    }
                },
                "mechanical_details": null
            }
        ]"#;

        let res = registry.load_homebrew_feats(json_data);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 1);

        let feat = registry.feats.get("telepata_corvos");
        assert!(feat.is_some());
        assert_eq!(feat.unwrap().name, "Telepata de Corvos");
    }
}
