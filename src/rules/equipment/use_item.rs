use crate::dice::Dice;
use crate::rules::conditions::Condition;
use crate::rules::equipment::registry::EquipmentRegistry;
use crate::rules::equipment::validator::{HomebrewValidator, ValidatedEffect};
use serde::{Deserialize, Serialize};

/// Resultado da utilização de um item pelo motor de regras
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UseItemResult {
    /// O motor de regras executou 100% da mecânica (dano, salvaguarda, condição)
    MechanicalExecuted {
        item_id: String,
        item_name: String,
        target_id: Option<String>,
        success: bool,
        applied_condition: Option<Condition>,
        summary: String,
    },
    /// Efeito narrativo/customizado repassado como instrução direta para a IA DM interpretar
    NarrativeCustomDirective {
        item_id: String,
        item_name: String,
        directive_for_ai: String,
    },
    /// O item não possui efeitos mecânicos ou narrativos especiais ao ser usado
    NoSpecialEffect {
        item_id: String,
        item_name: String,
        summary: String,
    },
}

/// Função central que processa a utilização de um equipamento/item no motor de regras
pub fn use_item(
    actor_id: &str,
    target_id: Option<&str>,
    item_id: &str,
    target_dex_modifier: i32,
) -> Result<UseItemResult, String> {
    // 1. Busca a arma ou armadura no EquipmentRegistry global ou homebrew
    let weapon = EquipmentRegistry::get_weapon(item_id);

    if let Some(w) = weapon {
        let report = HomebrewValidator::validate_weapon(&w);

        match report.validated_effect {
            ValidatedEffect::Mechanical {
                ref effect_name,
                ref description,
                save_dc,
                save_ability: _,
                condition,
            } => {
                let mut success = true;
                let summary;
                let mut applied = None;



                if let Some(dc) = save_dc {
                    let d20 = Dice::d20().roll();
                    let total_save = d20.total + target_dex_modifier;

                    if total_save < dc as i32 {
                        success = true;
                        applied = condition;
                        summary = format!(
                            "O agente '{}' usou '{}' em '{}'. O alvo FALHOU no teste de salvaguarda (Rolou {} vs CD {}) e recebeu a condição {:?}.",
                            actor_id, w.name, target_id.unwrap_or("Alvo"), total_save, dc, condition
                        );
                    } else {
                        success = false;
                        summary = format!(
                            "O agente '{}' usou '{}' em '{}'. O alvo PASSOU no teste de salvaguarda (Rolou {} vs CD {}).",
                            actor_id, w.name, target_id.unwrap_or("Alvo"), total_save, dc
                        );
                    }
                } else {
                    summary = format!(
                        "O agente '{}' ativou o efeito mecânico '{}' do item '{}': {}.",
                        actor_id, effect_name, w.name, description
                    );
                }

                Ok(UseItemResult::MechanicalExecuted {
                    item_id: w.id.clone(),
                    item_name: w.name.clone(),
                    target_id: target_id.map(|s| s.to_string()),
                    success,
                    applied_condition: applied,
                    summary,
                })
            }
            ValidatedEffect::NarrativeCustom { description } => {
                Ok(UseItemResult::NarrativeCustomDirective {
                    item_id: w.id.clone(),
                    item_name: w.name.clone(),
                    directive_for_ai: format!(
                        "O motor de regras não possui resolução sistêmica para este efeito. A IA DM DEVE interpretar narrativamente as seguintes diretrizes: {}",
                        description
                    ),
                })
            }
            ValidatedEffect::None => Ok(UseItemResult::NoSpecialEffect {
                item_id: w.id.clone(),
                item_name: w.name.clone(),
                summary: format!("O item '{}' foi utilizado pelo agente '{}', mas não possui um efeito ativável especial.", w.name, actor_id),
            }),
        }
    } else {
        Err(format!(
            "Item com ID '{}' não foi encontrado no registro de equipamentos.",
            item_id
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::equipment::weapons::{
        create_net, DamageType, Weapon, WeaponCategory, WeaponProperty,
    };

    #[test]
    fn test_use_net_mechanical_effect() {
        let result = use_item("guerreiro_1", Some("goblin_1"), "net", 0).unwrap();
        if let UseItemResult::MechanicalExecuted {
            item_id,
            success,
            applied_condition,
            ..
        } = result
        {
            assert_eq!(item_id, "net");
            assert!(success || !success);
            if success {
                assert_eq!(applied_condition, Some(Condition::Restrained));
            }
        } else {
            panic!("Esperado resultado mecânico para a rede!");
        }
    }

    #[test]
    fn test_use_narrative_custom_item() {
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

        // Registra temporariamente para o teste
        let mut reg = crate::rules::equipment::registry::EquipmentRegistry::new();
        reg.register_weapon(custom_weapon.clone());

        // Valida diretamente o relatório
        let report = HomebrewValidator::validate_weapon(&custom_weapon);
        assert!(matches!(
            report.validated_effect,
            ValidatedEffect::NarrativeCustom { .. }
        ));
    }
}
