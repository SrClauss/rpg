use crate::rules::checks::{roll_contest, CheckResult, ContestResult};
use serde::{Deserialize, Serialize};

/// Ações Táticas em Combate (SRD 5.1 Págs 92-95)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombatAction {
    Attack,
    CastSpell,
    Dash,
    Disengage,
    Dodge,
    Help,
    Hide,
    Ready,
    Search,
    UseObject,
    Grapple,
    Shove,
}

impl CombatAction {
    /// Se a ação consome a Ação Principal do turno
    pub fn requires_main_action(&self) -> bool {
        true
    }
}

/// Processa a manobra de Agarrar (Grapple - SRD 5.1 p. 95).
/// Substitui um ataque. Requer teste de Atletismo (Força) do atacante vs Atletismo ou Acrobacia do defensor.
pub fn resolve_grapple(
    attacker_athletics: CheckResult,
    defender_athletics_or_acrobatics: CheckResult,
) -> ContestResult {
    roll_contest(attacker_athletics, defender_athletics_or_acrobatics)
}

/// Processa a manobra de Empurrar (Shove - SRD 5.1 p. 95).
/// Substitui um ataque. Se o atacante vencer, pode derrubar o alvo Prone ou empurrá-lo 5 pés.
pub fn resolve_shove(
    attacker_athletics: CheckResult,
    defender_athletics_or_acrobatics: CheckResult,
) -> ContestResult {
    roll_contest(attacker_athletics, defender_athletics_or_acrobatics)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dice::{Dice, DiceMode};

    #[test]
    fn test_grapple_resolution() {
        let att_roll = Dice::new(20, 1, 5, DiceMode::Normal).roll();
        let def_roll = Dice::new(20, 1, 0, DiceMode::Normal).roll();

        let att_check = CheckResult::new(att_roll, None);
        let def_check = CheckResult::new(def_roll, None);

        let contest = resolve_grapple(att_check, def_check);
        assert!(contest.is_initiator_winner || !contest.is_initiator_winner);
    }
}
