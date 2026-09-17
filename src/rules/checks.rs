use serde::{Deserialize, Serialize};
use crate::dice::{Dice, DiceMode, RollResult};
use super::character::attributes::Ability;
use super::character::entity::Entity;
use super::character::skills::Skill;

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
pub fn roll_ability_check(entity: &Entity, ability: Ability, mode: DiceMode, dc: Option<i32>) -> CheckResult {
    let modifier = entity.attributes.get_modifier(ability) as i32;
    let dice = Dice::new(20, 1, modifier, mode);
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
    let dice = Dice::new(20, 1, modifier, mode);
    CheckResult::new(dice.roll(), dc)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::class::{Fighter, ClassVariant};
    use crate::rules::character::race::{Human, RaceVariant};
    use crate::rules::character::background::create_acolyte;

    #[test]
    fn test_ability_check() {
        let attrs = AbilityScores::new(16, 14, 12, 10, 8, 8); // STR 16 -> +3 (+1 Human = 17, +3)
        let hero = Entity::new("Hero", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());

        let res = roll_ability_check(&hero, Ability::Strength, DiceMode::Normal, Some(15));
        assert_eq!(res.roll_result.modifier, 3);
        assert_eq!(res.dc, Some(15));
    }

    #[test]
    fn test_saving_throw_with_proficiency() {
        let attrs = AbilityScores::new(14, 10, 10, 10, 10, 10); // STR 14 (+2, +1 Human = 15 -> +2)
        let hero = Entity::new("Hero", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());

        // Mod (+2) + Proficiência Nível 1 (+2) = +4
        let res = roll_saving_throw(&hero, Ability::Strength, true, DiceMode::Normal, None);
        assert_eq!(res.roll_result.modifier, 4);
    }

    #[test]
    fn test_skill_check() {
        let attrs = AbilityScores::new(10, 16, 10, 10, 10, 10); // DEX 16 (+3, +1 Human = 17 -> +3)
        let hero = Entity::new("Rogue", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());

        // Stealth utiliza DEX. Proficiente = +3 Mod + 2 Prof = +5
        let res = roll_skill_check(&hero, Skill::Stealth, true, DiceMode::Normal, Some(10));
        assert_eq!(res.roll_result.modifier, 5);
        assert!(res.is_success.unwrap() || !res.is_success.unwrap()); // Valida tipo
    }
}
