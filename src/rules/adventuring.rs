use serde::{Deserialize, Serialize};
use super::character::entity::Entity;

/// Tipos de Movimentação no D&D 5e
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MovementType {
    Walk,
    Fly,
    Swim,
    Climb,
}

/// Velocidade de Movimentação em Pés (feet)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MovementSpeed {
    pub speed_type: MovementType,
    pub feet: u32,
}

impl MovementSpeed {
    pub fn new(speed_type: MovementType, feet: u32) -> Self {
        Self { speed_type, feet }
    }

    pub fn walk(feet: u32) -> Self {
        Self::new(MovementType::Walk, feet)
    }
}

/// Trait que define a capacidade de descanso para recuperação de atributos e saúde
pub trait Restable {
    /// Realiza um Descanso Longo (Long Rest, 8 horas):
    /// Conforme as regras do SRD D&D 5e, recupera todos os Pontos de Vida (PV) perdidos.
    fn long_rest(&mut self);
}

impl Restable for Entity {
    fn long_rest(&mut self) {
        self.current_hp = self.max_hp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::class::{Fighter, ClassVariant};
    use crate::rules::character::race::{Human, RaceVariant};
    use crate::rules::character::background::create_acolyte;

    #[test]
    fn test_long_rest_restores_hp() {
        let attrs = AbilityScores::new(14, 14, 14, 10, 10, 10);
        let mut hero = Entity::new("Fighter", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());

        // Reduz o HP do personagem
        hero.current_hp = 2;
        assert!(hero.current_hp < hero.max_hp);

        // Aplica o descanso longo
        hero.long_rest();

        // HP deve ter sido totalmente restaurado
        assert_eq!(hero.current_hp, hero.max_hp);
    }

    #[test]
    fn test_movement_speed_creation() {
        let speed = MovementSpeed::walk(30);
        assert_eq!(speed.speed_type, MovementType::Walk);
        assert_eq!(speed.feet, 30);
    }
}
