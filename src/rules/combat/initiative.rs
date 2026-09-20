use crate::dice::{Dice, DiceMode};
use crate::rules::character::attributes::Ability;
use crate::rules::character::entity::Entity;
use serde::{Deserialize, Serialize};

/// Representa um participante na Fila de Iniciativa
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Combatant {
    pub id: String,
    pub name: String,
    pub initiative_score: i32,
    pub dex_modifier: i32,
}

/// Gerenciador da Fila de Iniciativa e Rodadas de Combate
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InitiativeTracker {
    pub combatants: Vec<Combatant>,
    pub current_turn_index: usize,
    pub round: u32,
    pub is_active: bool,
}

impl InitiativeTracker {
    pub fn new() -> Self {
        Self {
            combatants: Vec::new(),
            current_turn_index: 0,
            round: 0,
            is_active: false,
        }
    }

    /// Adiciona uma Entidade ao combate e rola sua Iniciativa (1d20 + Mod. DES)
    pub fn add_entity(&mut self, entity: &Entity, mode: DiceMode) {
        let dex_mod = entity.attributes.get_modifier(Ability::Dexterity) as i32;
        let dice = Dice::new(20, 1, dex_mod, mode);
        let roll_result = dice.roll();

        let combatant = Combatant {
            id: entity.name.clone(), // Usa o nome como ID único inicial
            name: entity.name.clone(),
            initiative_score: roll_result.total,
            dex_modifier: dex_mod,
        };

        self.combatants.push(combatant);
    }

    /// Adiciona um combatente com um valor de iniciativa pré-determinado
    pub fn add_combatant_custom(
        &mut self,
        id: &str,
        name: &str,
        initiative_score: i32,
        dex_modifier: i32,
    ) {
        self.combatants.push(Combatant {
            id: id.to_string(),
            name: name.to_string(),
            initiative_score,
            dex_modifier,
        });
    }

    /// Inicia o combate: ordena os combatentes em ordem decrescente de Iniciativa.
    /// Em caso de empate na Iniciativa, desempata pelo maior Modificador de Destreza.
    pub fn start_combat(&mut self) {
        self.combatants.sort_by(|a, b| {
            b.initiative_score
                .cmp(&a.initiative_score)
                .then_with(|| b.dex_modifier.cmp(&a.dex_modifier))
        });
        self.current_turn_index = 0;
        self.round = 1;
        self.is_active = !self.combatants.is_empty();
    }

    /// Retorna o combatente do turno atual
    pub fn current_combatant(&self) -> Option<&Combatant> {
        if !self.is_active || self.combatants.is_empty() {
            None
        } else {
            self.combatants.get(self.current_turn_index)
        }
    }

    /// Avança para o próximo turno. Quando a rodada termina, reinicia o índice e incrementa a rodada.
    pub fn next_turn(&mut self) -> Option<&Combatant> {
        if !self.is_active || self.combatants.is_empty() {
            return None;
        }

        self.current_turn_index += 1;
        if self.current_turn_index >= self.combatants.len() {
            self.current_turn_index = 0;
            self.round += 1;
        }

        self.current_combatant()
    }

    /// Remove um combatente do combate pelo ID
    pub fn remove_combatant(&mut self, id: &str) {
        if let Some(pos) = self.combatants.iter().position(|c| c.id == id) {
            self.combatants.remove(pos);
            if self.combatants.is_empty() {
                self.is_active = false;
                self.current_turn_index = 0;
            } else if pos <= self.current_turn_index && self.current_turn_index > 0 {
                self.current_turn_index -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::background::create_acolyte;
    use crate::rules::character::class::{ClassVariant, Fighter};
    use crate::rules::character::race::{Human, RaceVariant};

    #[test]
    fn test_initiative_order_and_turn_progression() {
        let mut tracker = InitiativeTracker::new();

        // Adiciona 3 combatentes customizados com pontuações fixas
        tracker.add_combatant_custom("1", "Rogue", 18, 4);
        tracker.add_combatant_custom("2", "Fighter", 15, 2);
        tracker.add_combatant_custom("3", "Wizard", 15, 1); // Empate de iniciativa com o Fighter, mas menor DES

        tracker.start_combat();

        assert_eq!(tracker.round, 1);
        assert_eq!(tracker.current_combatant().unwrap().name, "Rogue");

        // Turno 2: Fighter (desempatou pela DES maior 2 vs 1)
        let next1 = tracker.next_turn().unwrap();
        assert_eq!(next1.name, "Fighter");

        // Turno 3: Wizard
        let next2 = tracker.next_turn().unwrap();
        assert_eq!(next2.name, "Wizard");

        // Próximo turno deve passar para a Rodada 2 e voltar para o Rogue!
        let round2_first = tracker.next_turn().unwrap();
        assert_eq!(round2_first.name, "Rogue");
        assert_eq!(tracker.round, 2);
    }

    #[test]
    fn test_add_entity_initiative() {
        let attrs = AbilityScores::new(10, 16, 10, 10, 10, 10);
        let hero = Entity::new(
            "Hero",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let mut tracker = InitiativeTracker::new();
        tracker.add_entity(&hero, DiceMode::Normal);

        assert_eq!(tracker.combatants.len(), 1);
        assert_eq!(tracker.combatants[0].dex_modifier, 3);
    }
}
