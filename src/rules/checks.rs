use super::character::attributes::Ability;
use super::character::entity::Entity;
use super::character::skills::Skill;
use crate::dice::{Dice, DiceMode, RollResult};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Resultado detalhado de um teste (Habilidade, Salvaguarda ou Perícia) comparado a uma Dificuldade (DC)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckResult {
    pub roll_result: RollResult,
    pub dc: Option<i32>,
    pub is_success: Option<bool>,
}

impl CheckResult {
    pub fn new(roll_result: RollResult, dc: Option<i32>) -> Self {
        let is_success = dc.map(|d| roll_result.total >= d);
        Self {
            roll_result,
            dc,
            is_success,
        }
    }
}

/// Realiza um Teste de Habilidade (Ability Check) para a Entidade.
/// Fórmula: 1d20 + Modificador de Atributo.
pub fn roll_ability_check(
    entity: &Entity,
    ability: Ability,
    mode: DiceMode,
    dc: Option<i32>,
) -> CheckResult {
    let modifier = entity.attributes.get_modifier(ability) as i32;
    let mut final_mode = mode;

    let mut has_advantage = false;
    let mut has_disadvantage = false;

    // Check base mode
    if mode == DiceMode::Advantage {
        has_advantage = true;
    }
    if mode == DiceMode::Disadvantage {
        has_disadvantage = true;
    }

    for e in &entity.applied_effects {
        if let crate::rules::character::active_effects::ActiveEffect::AdvantageOnAbilityCheck(a) =
            &e.effect
        {
            if *a == ability {
                has_advantage = true;
            }
        }
        if let crate::rules::character::active_effects::ActiveEffect::DisadvantageOnAbilityCheck(
            a,
        ) = &e.effect
        {
            if *a == ability {
                has_disadvantage = true;
            }
        }
    }

    if has_advantage && has_disadvantage {
        final_mode = DiceMode::Normal;
    } else if has_advantage {
        final_mode = DiceMode::Advantage;
    } else if has_disadvantage {
        final_mode = DiceMode::Disadvantage;
    }

    let dice = Dice::new(20, 1, modifier, final_mode);
    CheckResult::new(dice.roll(), dc)
}

/// Realiza um Teste de Salvaguarda (Saving Throw) para a Entidade.
/// Fórmula: 1d20 + Modificador de Atributo + (Bônus de Proficiência se for proficiente).
pub fn roll_saving_throw(
    entity: &Entity,
    ability: Ability,
    is_proficient: bool,
    mode: DiceMode,
    dc: Option<i32>,
) -> CheckResult {
    let mut modifier = entity.attributes.get_modifier(ability) as i32;
    if is_proficient {
        modifier += entity.proficiency_bonus();
    }

    let mut final_mode = mode;
    let mut has_advantage = false;
    let mut has_disadvantage = false;

    // Check base mode
    if mode == DiceMode::Advantage {
        has_advantage = true;
    }
    if mode == DiceMode::Disadvantage {
        has_disadvantage = true;
    }

    for e in &entity.applied_effects {
        if let crate::rules::character::active_effects::ActiveEffect::AdvantageOnSavingThrowAbility(a) = &e.effect {
            if *a == ability { has_advantage = true; }
        }
        if let crate::rules::character::active_effects::ActiveEffect::DisadvantageOnSavingThrowAbility(a) = &e.effect {
            if *a == ability { has_disadvantage = true; }
        }
    }

    if has_advantage && has_disadvantage {
        final_mode = DiceMode::Normal;
    } else if has_advantage {
        final_mode = DiceMode::Advantage;
    } else if has_disadvantage {
        final_mode = DiceMode::Disadvantage;
    }

    let dice = Dice::new(20, 1, modifier, final_mode);

    let mut bonus_str = String::new();
    let mut penalty_str = String::new();

    for eff in &entity.applied_effects {
        match &eff.effect {
            crate::rules::character::active_effects::ActiveEffect::BonusSavingThrow(d) => {
                if !bonus_str.is_empty() {
                    bonus_str.push('+');
                }
                bonus_str.push_str(d);
            }
            crate::rules::character::active_effects::ActiveEffect::PenaltySavingThrow(d) => {
                if !penalty_str.is_empty() {
                    penalty_str.push('+');
                }
                penalty_str.push_str(d);
            }
            crate::rules::character::active_effects::ActiveEffect::Multiple(effs) => {
                for e in effs {
                    match e {
                        crate::rules::character::active_effects::ActiveEffect::BonusSavingThrow(d) => {
                            if !bonus_str.is_empty() { bonus_str.push('+'); }
                            bonus_str.push_str(d);
                        }
                        crate::rules::character::active_effects::ActiveEffect::PenaltySavingThrow(d) => {
                            if !penalty_str.is_empty() { penalty_str.push('+'); }
                            penalty_str.push_str(d);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    let mut result = dice.roll();
    if !bonus_str.is_empty() {
        let b = crate::dice::Dice::from_str(&bonus_str)
            .unwrap_or_else(|_| Dice::new(0, 0, 0, DiceMode::Normal));
        let b_roll = b.roll();
        result.total += b_roll.total;
    }
    if !penalty_str.is_empty() {
        let p = crate::dice::Dice::from_str(&penalty_str)
            .unwrap_or_else(|_| Dice::new(0, 0, 0, DiceMode::Normal));
        let p_roll = p.roll();
        result.total -= p_roll.total;
    }

    CheckResult::new(result, dc)
}

/// Realiza um Teste de Perícia (Skill Check) para a Entidade.
/// Fórmula: 1d20 + Modificador do Atributo Base da Perícia + (Bônus de Proficiência se for proficiente).
pub fn roll_skill_check(
    entity: &Entity,
    skill: Skill,
    is_proficient: bool,
    mode: DiceMode,
    dc: Option<i32>,
) -> CheckResult {
    let base_ability = skill.base_ability();
    let mut modifier = entity.attributes.get_modifier(base_ability) as i32;
    if is_proficient {
        modifier += entity.proficiency_bonus();
    }
    let dice = Dice::new(20, 1, modifier, mode);
    CheckResult::new(dice.roll(), dc)
}

/// Calcula o Valor Passivo de um Teste/Perícia para a Entidade.
/// Fórmula SRD 5.1: 10 + Modificador + (Proficiência se houver) + (+5 se Vantagem, -5 se Desvantagem).
pub fn get_passive_score(
    entity: &Entity,
    ability: Ability,
    is_proficient: bool,
    mode: DiceMode,
) -> i32 {
    let mut score = 10 + entity.attributes.get_modifier(ability) as i32;
    if is_proficient {
        score += entity.proficiency_bonus();
    }
    match mode {
        DiceMode::Advantage => score += 5,
        DiceMode::Disadvantage => score -= 5,
        DiceMode::Normal => {}
    }
    score
}

/// Representa o resultado de um Conteste (Contest) entre duas entidades (ex: Agarrar/Empurrar).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContestResult {
    pub initiator_result: CheckResult,
    pub defender_result: CheckResult,
    pub is_initiator_winner: bool,
}

/// Realiza um Conteste de Atributo/Perícia entre duas entidades.
/// SRD 5.1 (p. 77): Se houver empate, a situação permanece idêntica ao estado anterior ao conteste.
pub fn roll_contest(initiator_result: CheckResult, defender_result: CheckResult) -> ContestResult {
    let is_initiator_winner =
        initiator_result.roll_result.total > defender_result.roll_result.total;
    ContestResult {
        initiator_result,
        defender_result,
        is_initiator_winner,
    }
}

/// Calcula o resultado de um Teste em Grupo (Group Check).
/// SRD 5.1 (p. 77): O grupo tem sucesso se pelo menos metade dos membros passarem no teste.
pub fn is_group_check_successful(results: &[CheckResult]) -> bool {
    if results.is_empty() {
        return false;
    }
    let successes = results
        .iter()
        .filter(|r| r.is_success.unwrap_or(false))
        .count();
    successes * 2 >= results.len()
}

/// Calcula a Capacidade de Carga (Carrying Capacity) em libras.
/// SRD 5.1 (p. 78): Força * 15.
pub fn carrying_capacity_lbs(strength_score: u8) -> u32 {
    strength_score as u32 * 15
}

/// Calcula a capacidade de Empurrar, Puxar ou Levantar (Push, Drag, or Lift) em libras.
/// SRD 5.1 (p. 78): Força * 30.
pub fn push_drag_lift_lbs(strength_score: u8) -> u32 {
    strength_score as u32 * 30
}

/// Realiza um Teste de Perícia com Atributo Alternativo (ex: Intimidação [Força]).
/// SRD 5.1 (p. 77): Variante de Regra.
pub fn roll_skill_check_with_custom_ability(
    entity: &Entity,
    _skill: Skill,
    custom_ability: Ability,
    is_proficient: bool,
    mode: DiceMode,
    dc: Option<i32>,
) -> CheckResult {
    let mut modifier = entity.attributes.get_modifier(custom_ability) as i32;
    if is_proficient {
        modifier += entity.proficiency_bonus();
    }
    let dice = Dice::new(20, 1, modifier, mode);
    CheckResult::new(dice.roll(), dc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::background::create_acolyte;
    use crate::rules::character::class::{ClassVariant, Fighter};
    use crate::rules::character::race::{Human, RaceVariant};

    #[test]
    fn test_ability_check() {
        let attrs = AbilityScores::new(16, 14, 12, 10, 8, 8); // STR 16 -> +3 (+1 Human = 17, +3)
        let hero = Entity::new(
            "Hero",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let res = roll_ability_check(&hero, Ability::Strength, DiceMode::Normal, Some(15));
        assert_eq!(res.roll_result.modifier, 3);
        assert_eq!(res.dc, Some(15));
    }

    #[test]
    fn test_saving_throw_with_proficiency() {
        let attrs = AbilityScores::new(14, 10, 10, 10, 10, 10); // STR 14 (+2, +1 Human = 15 -> +2)
        let hero = Entity::new(
            "Hero",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        // Mod (+2) + Proficiência Nível 1 (+2) = +4
        let res = roll_saving_throw(&hero, Ability::Strength, true, DiceMode::Normal, None);
        assert_eq!(res.roll_result.modifier, 4);
    }

    #[test]
    fn test_skill_check() {
        let attrs = AbilityScores::new(10, 16, 10, 10, 10, 10); // DEX 16 (+3, +1 Human = 17 -> +3)
        let hero = Entity::new(
            "Rogue",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        // Stealth utiliza DEX. Proficiente = +3 Mod + 2 Prof = +5
        let res = roll_skill_check(&hero, Skill::Stealth, true, DiceMode::Normal, Some(10));
        assert_eq!(res.roll_result.modifier, 5);
        assert!(res.is_success.unwrap() || !res.is_success.unwrap()); // Valida tipo
    }
}
