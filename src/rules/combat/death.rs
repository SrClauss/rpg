use crate::dice::{Dice, DiceMode};
use crate::rules::character::entity::Entity;
use crate::rules::conditions::Condition;
use serde::{Deserialize, Serialize};

/// Estado de Salvaguarda de Morte para Entidades com 0 Pontos de Vida (SRD 5.1 p. 98)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeathSaveState {
    pub successes: u8,
    pub failures: u8,
    pub is_stable: bool,
    pub is_dead: bool,
}

impl DeathSaveState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Rola uma salvaguarda de morte (1d20 puro)
    /// 10+: Sucesso (3 sucessos = Estabilizado)
    /// <10: Falha (3 falhas = Morte)
    /// 1 Natural: 2 Falhas
    /// 20 Natural: Recupera 1 HP e acorda!
    pub fn roll_death_save(&mut self, entity: &mut Entity) -> (u8, String) {
        if self.is_dead || self.is_stable || entity.current_hp > 0 {
            return (
                0,
                "Entidade não precisa rolar salvaguarda de morte.".to_string(),
            );
        }

        let dice = Dice::new(20, 1, 0, DiceMode::Normal);
        let roll = dice.roll();
        let val = roll.total;

        if val == 20 {
            entity.current_hp = 1;
            entity.remove_condition(&Condition::Unconscious);
            self.is_stable = true;
            return (
                val as u8,
                "20 Natural! Entidade recuperou 1 HP e voltou à consciência!".to_string(),
            );
        }

        if val == 1 {
            self.failures += 2;
        } else if val >= 10 {
            self.successes += 1;
        } else {
            self.failures += 1;
        }

        if self.failures >= 3 {
            self.is_dead = true;
            (
                val as u8,
                "3 Falhas acumuladas. A entidade faleceu.".to_string(),
            )
        } else if self.successes >= 3 {
            self.is_stable = true;
            (
                val as u8,
                "3 Sucessos acumulados. A entidade está estabilizada.".to_string(),
            )
        } else {
            (
                val as u8,
                format!(
                    "Salvaguarda de morte: {}. Sucessos: {}, Falhas: {}",
                    val, self.successes, self.failures
                ),
            )
        }
    }
}

/// Processa dano recebido verificando morte instantânea (SRD 5.1 p. 98).
/// Se o dano excedente após reduzir a 0 PV for maior ou igual ao PV Máximo, resulta em Morte Instantânea.
pub fn apply_damage_with_instant_death_check(
    entity: &mut Entity,
    damage: i32,
    death_state: &mut DeathSaveState,
) -> String {
    if damage <= 0 {
        return "Nenhum dano aplicado.".to_string();
    }

    let previous_hp = entity.current_hp;
    entity.current_hp = std::cmp::max(0, entity.current_hp - damage);

    if entity.current_hp == 0 {
        let excess_damage = damage - previous_hp;
        if excess_damage >= entity.get_max_hp() {
            death_state.is_dead = true;
            entity.current_hp = 0;
            // Opcional: Adicionar a condição `Dead`
            return format!(
                "Morte Instantânea! Dano excedente ({}) >= HP Máximo ({}).",
                excess_damage,
                entity.get_max_hp()
            );
        } else if previous_hp > 0 {
            entity.add_condition(Condition::Unconscious);
            return "Entidade caiu para 0 HP e ficou inconsciente.".to_string();
        } else {
            // Se já estava com 0 HP e sofreu dano, ganha 1 falha na salvaguarda de morte (ou 2 se for acerto crítico)
            death_state.failures += 1;
            if death_state.failures >= 3 {
                death_state.is_dead = true;
                return "Dano sofrido com 0 HP. 3ª falha em Death Save: Entidade faleceu."
                    .to_string();
            }
            return format!(
                "Dano sofrido com 0 HP! Falha em Death Save (Total: {}).",
                death_state.failures
            );
        }
    }

    format!(
        "Dano de {} aplicado. HP restante: {}/{}",
        damage,
        entity.current_hp,
        entity.get_max_hp()
    )
}

/// Nocautear uma Criatura (Knocking a Creature Out - SRD 5.1 p. 98).
/// Quando um atacante reduz uma criatura a 0 PV com um ataque corpo a corpo, pode optar por nocauteá-la (estável e inconsciente).
pub fn knock_out_creature(entity: &mut Entity, death_state: &mut DeathSaveState) {
    entity.current_hp = 0;
    entity.add_condition(Condition::Unconscious);
    death_state.is_stable = true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::background::create_acolyte;
    use crate::rules::character::class::{ClassVariant, Fighter};
    use crate::rules::character::race::{Human, RaceVariant};

    #[test]
    fn test_instant_death() {
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut hero = Entity::new(
            "Fighter",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        hero.base_max_hp = 12;
        hero.current_hp = 5;

        let mut ds = DeathSaveState::new();

        // Recebe 17 de dano -> Excedente = 17 - 5 = 12 (igual ao max_hp)
        let msg = apply_damage_with_instant_death_check(&mut hero, 17, &mut ds);
        assert!(ds.is_dead);
        assert!(msg.contains("Morte Instantânea"));
    }

    #[test]
    fn test_knock_out() {
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut hero = Entity::new(
            "Enemy",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut ds = DeathSaveState::new();

        knock_out_creature(&mut hero, &mut ds);
        assert_eq!(hero.current_hp, 0);
        assert!(ds.is_stable);
        assert!(!ds.is_dead);
    }
}
