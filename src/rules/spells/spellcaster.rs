use crate::dice::DiceMode;
use crate::rules::character::attributes::Ability;
use crate::rules::character::entity::Entity;
use crate::rules::checks::{roll_saving_throw, CheckResult};
use serde::{Deserialize, Serialize};

/// Representa a quantidade de Espaços de Magia (Spell Slots) por nível (1 a 9)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SpellSlotLevel {
    pub max: u8,
    pub current: u8,
}

impl SpellSlotLevel {
    pub fn new(max: u8) -> Self {
        Self { max, current: max }
    }
}

/// Gerenciador de Espaços de Magia do Conjurador
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SpellSlots {
    /// Slots dos níveis 1 a 9 (índices 0 a 8)
    pub slots: [SpellSlotLevel; 9],
}

impl SpellSlots {
    pub fn new() -> Self {
        Self {
            slots: [SpellSlotLevel::default(); 9],
        }
    }

    /// Configura a quantidade de slots para um determinado nível (1-9)
    pub fn set_slots(&mut self, level: u8, max: u8) {
        if (1..=9).contains(&level) {
            self.slots[(level - 1) as usize] = SpellSlotLevel::new(max);
        }
    }

    /// Verifica se possui pelo menos 1 slot disponível para o nível solicitado (Truques / Nível 0 sempre possuem slot)
    pub fn has_slot(&self, level: u8) -> bool {
        if level == 0 {
            return true;
        }
        if (1..=9).contains(&level) {
            self.slots[(level - 1) as usize].current > 0
        } else {
            false
        }
    }

    /// Consome 1 slot do nível especificado (1-9)
    pub fn consume_slot(&mut self, level: u8) -> bool {
        if level == 0 {
            return true; // Truques não consomem slot
        }
        if (1..=9).contains(&level) {
            let slot = &mut self.slots[(level - 1) as usize];
            if slot.current > 0 {
                slot.current -= 1;
                return true;
            }
        }
        false
    }

    /// Restaura todos os slots de magia ao máximo (Descanso Longo)
    pub fn recover_all(&mut self) {
        for slot in &mut self.slots {
            slot.current = slot.max;
        }
    }
}

/// Rastreia uma Magia que exige Concentração ativa
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActiveConcentration {
    pub spell_name: String,
    pub spell_level: u8,
}

impl Entity {
    /// Ao sofrer dano, se a Entidade tiver uma magia em Concentração ativa, deve realizar
    /// um Teste de Salvaguarda de Constituição (CD = max(10, dano / 2)).
    /// Se falhar, perde a concentração!
    pub fn check_concentration_on_damage(
        &mut self,
        damage_taken: i32,
        is_con_save_proficient: bool,
    ) -> Option<CheckResult> {
        if self.active_concentration.is_none() || damage_taken <= 0 {
            return None;
        }

        let dc = std::cmp::max(10, damage_taken / 2);
        let check_result = roll_saving_throw(
            self,
            Ability::Constitution,
            is_con_save_proficient,
            DiceMode::Normal,
            Some(dc),
        );

        if let Some(is_success) = check_result.is_success {
            if !is_success {
                self.active_concentration = None; // Perdeu a concentração!
            }
        }

        Some(check_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::background::create_acolyte;
    use crate::rules::character::class::{ClassVariant, Wizard};
    use crate::rules::character::race::{Human, RaceVariant};

    #[test]
    fn test_spell_slots_management() {
        let mut slots = SpellSlots::new();
        slots.set_slots(1, 4); // Wizard Lvl 1: 4 slots de 1º Nível
        slots.set_slots(2, 2);

        assert!(slots.has_slot(0)); // Truque
        assert!(slots.has_slot(1));
        assert_eq!(slots.slots[0].current, 4);

        assert!(slots.consume_slot(1));
        assert_eq!(slots.slots[0].current, 3);

        slots.recover_all();
        assert_eq!(slots.slots[0].current, 4);
    }

    #[test]
    fn test_concentration_check_on_damage() {
        let attrs = AbilityScores::new(10, 10, 14, 16, 10, 10); // CON 14 (+2, +1 Human = 15 -> +2)
        let mut wizard = Entity::new(
            "Mage",
            attrs,
            ClassVariant::Wizard(Wizard),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        wizard.active_concentration = Some(ActiveConcentration {
            spell_name: "Fly".to_string(),
            spell_level: 3,
        });

        // Dano de 10 -> CD = 10
        let res = wizard.check_concentration_on_damage(10, false);
        assert!(res.is_some());
        assert_eq!(res.unwrap().dc, Some(10));
    }
}
