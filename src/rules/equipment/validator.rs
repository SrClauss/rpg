use super::armor::Armor;
use super::weapons::{Weapon, WeaponProperty};
use crate::dice::Dice;
use crate::rules::character::attributes::Ability;
use crate::rules::conditions::Condition;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Classificação do Efeito do Item Homebrew
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidatedEffect {
    /// Efeito mecânico estruturado que o Rust executa nativamente
    Mechanical {
        effect_name: String,
        description: String,
        save_dc: Option<u8>,
        save_ability: Option<Ability>,
        condition: Option<Condition>,
    },
    /// Efeito narrativo/customizado em texto livre repassado para a IA DM interpretar
    NarrativeCustom { description: String },
    /// Item sem efeito especial descritivo
    None,
}

/// Relatório de Validação de um Item Homebrew
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub item_id: String,
    pub item_name: String,
    pub validated_effect: ValidatedEffect,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Validador Central de Equipamentos Homebrew
pub struct HomebrewValidator;

impl HomebrewValidator {
    /// Valida uma arma Homebrew
    pub fn validate_weapon(weapon: &Weapon) -> ValidationReport {
        let mut warnings = Vec::new();
        let mut errors = Vec::new();

        // 1. Validação de ID e Nome
        if weapon.id.trim().is_empty() {
            errors.push("ID do item não pode ser vazio.".to_string());
        }
        if weapon.name.trim().is_empty() {
            errors.push("Nome do item não pode ser vazio.".to_string());
        }

        // 2. Validação do Dado de Dano (se não for dano 0)
        if weapon.damage_dice != "0"
            && weapon.damage_dice != "1"
            && Dice::from_str(&weapon.damage_dice).is_err()
        {
            warnings.push(format!(
                "Notação de dado de dano '{}' não padrão. Será padronizada para 1d6 se inválida.",
                weapon.damage_dice
            ));
        }

        // 3. Classificação de Efeitos Especiais
        let validated_effect = Self::classify_weapon_effect(weapon);

        ValidationReport {
            is_valid: errors.is_empty(),
            item_id: weapon.id.clone(),
            item_name: weapon.name.clone(),
            validated_effect,
            warnings,
            errors,
        }
    }

    /// Classifica o efeito especial da arma em mecânico ou narrativo/customizado
    fn classify_weapon_effect(weapon: &Weapon) -> ValidatedEffect {
        if weapon.has_property(WeaponProperty::Special) {
            match weapon.id.as_str() {
                "lance" => ValidatedEffect::Mechanical {
                    effect_name: "Lance Special".to_string(),
                    description: "Desvantagem a 5ft; exige 2 mãos se não estiver montado.".to_string(),
                    save_dc: None,
                    save_ability: None,
                    condition: None,
                },
                "net" => ValidatedEffect::Mechanical {
                    effect_name: "Net Entangle".to_string(),
                    description: "Aplica a condição Restrained no acerto.".to_string(),
                    save_dc: Some(10),
                    save_ability: Some(Ability::Strength),
                    condition: Some(Condition::Restrained),
                },
                _ => ValidatedEffect::NarrativeCustom {
                    description: format!("Efeito especial customizado da arma {}: regras narrativas delegadas para a IA DM.", weapon.name),
                },
            }
        } else {
            ValidatedEffect::None
        }
    }

    /// Classifica uma descrição de efeito em texto livre
    pub fn classify_description_text(description: &str) -> ValidatedEffect {
        let desc_lower = description.to_lower();

        // Verifica palavras-chave mecânicas reconhecidas pelo engine
        if desc_lower.contains("dano")
            || desc_lower.contains("damage")
            || desc_lower.contains("cd")
            || desc_lower.contains("dc")
        {
            ValidatedEffect::Mechanical {
                effect_name: "Custom Mechanical Effect".to_string(),
                description: description.to_string(),
                save_dc: None,
                save_ability: None,
                condition: None,
            }
        } else if !description.trim().is_empty() {
            ValidatedEffect::NarrativeCustom {
                description: description.to_string(),
            }
        } else {
            ValidatedEffect::None
        }
    }
}

// Helper para converter em minusculo sem alocacao extra excessiva
trait ToLowerExt {
    fn to_lower(&self) -> String;
}

impl ToLowerExt for str {
    fn to_lower(&self) -> String {
        self.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::equipment::weapons::{
        create_dagger, create_lance, create_net, DamageType, WeaponCategory,
    };

    #[test]
    fn test_validate_srd_weapons() {
        let dagger = create_dagger();
        let report = HomebrewValidator::validate_weapon(&dagger);
        assert!(report.is_valid);
        assert_eq!(report.validated_effect, ValidatedEffect::None);

        let lance = create_lance();
        let lance_report = HomebrewValidator::validate_weapon(&lance);
        assert!(lance_report.is_valid);
        assert!(matches!(
            lance_report.validated_effect,
            ValidatedEffect::Mechanical { .. }
        ));
    }

    #[test]
    fn test_validate_custom_narrative_homebrew_weapon() {
        let custom_weapon = Weapon::new(
            "espada_amnesia",
            "Espada da Amnésia",
            WeaponCategory::MartialMelee,
            "1d8",
            DamageType::Slashing,
            100,
            3,
            vec![WeaponProperty::Special],
        );

        let report = HomebrewValidator::validate_weapon(&custom_weapon);
        assert!(report.is_valid);
        assert!(matches!(
            report.validated_effect,
            ValidatedEffect::NarrativeCustom { .. }
        ));
    }

    #[test]
    fn test_classify_description_text() {
        let narrative = HomebrewValidator::classify_description_text(
            "Faz o alvo esquecer quem ele é e voltar 2 turnos no tempo",
        );
        assert!(matches!(narrative, ValidatedEffect::NarrativeCustom { .. }));

        let mechanical = HomebrewValidator::classify_description_text(
            "Causa 2d6 de dano de Ácido adicional e exige CD 14",
        );
        assert!(matches!(mechanical, ValidatedEffect::Mechanical { .. }));
    }
}
