use super::character::entity::Entity;
use serde::{Deserialize, Serialize};

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
    /// Realiza um Descanso Curto (Short Rest, 1 hora):
    /// Recupera HP gastando Dados de Vida + Modificador de Constituição.
    fn short_rest(&mut self, hit_dice_spent: u8) -> Result<i32, String>;

    /// Realiza um Descanso Longo (Long Rest, 8 horas):
    /// Recupera todos os Pontos de Vida (PV) perdidos e renova slots de magia.
    fn long_rest(&mut self);
}

impl Restable for Entity {
    fn short_rest(&mut self, hit_dice_spent: u8) -> Result<i32, String> {
        if hit_dice_spent == 0 {
            return Err("Nenhum dado de vida gasto.".to_string());
        }
        let con_mod = self.attributes.constitution.modifier() as i32;
        // Dado de vida fixo aproximado ou rolado (ex: metade da média + mod CON por dado)
        let hit_dice_val = self
            .classes
            .first()
            .map_or(6, |c| c.as_trait().get_hit_dice() as i32);
        let hp_per_die = std::cmp::max(1, (hit_dice_val / 2 + 1) + con_mod);
        let total_healed = hp_per_die * (hit_dice_spent as i32);

        self.current_hp = std::cmp::min(self.get_max_hp(), self.current_hp + total_healed);
        Ok(total_healed)
    }

    fn long_rest(&mut self) {
        self.current_hp = self.get_max_hp();
        if let Some(slots) = &mut self.spell_slots {
            slots.recover_all();
        }
    }
}

/// Ritmo de Viagem (Travel Pace - SRD 5.1 p. 84-85)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TravelPace {
    Fast,
    Normal,
    Slow,
}

impl TravelPace {
    pub fn distance_per_minute_feet(&self) -> u32 {
        match self {
            TravelPace::Fast => 400,
            TravelPace::Normal => 300,
            TravelPace::Slow => 200,
        }
    }

    pub fn distance_per_hour_miles(&self) -> f32 {
        match self {
            TravelPace::Fast => 4.0,
            TravelPace::Normal => 3.0,
            TravelPace::Slow => 2.0,
        }
    }

    pub fn distance_per_day_miles(&self) -> u32 {
        match self {
            TravelPace::Fast => 30,
            TravelPace::Normal => 24,
            TravelPace::Slow => 18,
        }
    }

    /// Efeito no teste de Percepção Passiva visual (Fast = -5 de penalidade)
    pub fn passive_perception_penalty(&self) -> i32 {
        match self {
            TravelPace::Fast => -5,
            TravelPace::Normal | TravelPace::Slow => 0,
        }
    }

    /// Se permite usar Furtividade durante a viagem (apenas Slow)
    pub fn allows_stealth(&self) -> bool {
        matches!(self, TravelPace::Slow)
    }
}

/// Níveis de Iluminação (SRD 5.1 p. 86)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LightLevel {
    BrightLight,
    DimLight,
    Darkness,
}

/// Calcula a distância de Salto em Distância (Long Jump - SRD 5.1 p. 85).
/// Com corrida prévia de 10ft = pés iguais ao valor de Força.
/// Sem corrida = metade.
pub fn calculate_long_jump_feet(strength_score: u8, running_start: bool) -> u32 {
    let base = strength_score as u32;
    if running_start {
        base
    } else {
        base / 2
    }
}

/// Calcula a distância de Salto em Altura (High Jump - SRD 5.1 p. 85).
/// Com corrida prévia = 3 + mod FOR em pés (mínimo 0ft).
/// Sem corrida = metade.
pub fn calculate_high_jump_feet(strength_modifier: i8, running_start: bool) -> u32 {
    let base = std::cmp::max(0, 3 + strength_modifier as i32) as u32;
    if running_start {
        base
    } else {
        base / 2
    }
}

/// Calcula o tempo máximo de retenção de respiração em minutos (SRD 5.1 p. 86).
/// 1 + mod CON minutos (mínimo 30 segundos / 0.5 minutos).
pub fn max_breath_duration_minutes(constitution_modifier: i8) -> f32 {
    let minutes = 1 + constitution_modifier as i32;
    if minutes < 1 {
        0.5
    } else {
        minutes as f32
    }
}

/// Calcula quantos turnos uma criatura sem ar pode sobreviver antes de cair para 0 PV (SRD 5.1 p. 86).
/// Rodadas iguais a mod CON (mínimo 1 turno).
pub fn suffocation_rounds_remaining(constitution_modifier: i8) -> u32 {
    std::cmp::max(1, constitution_modifier as i32) as u32
}

/// Calcula o dano de queda (SRD 5.1):
/// 1d6 de dano concussivo por cada 10 pés de queda (máximo 20d6, ou seja 200 pés).
pub fn calculate_fall_damage(feet_fallen: u32) -> (i32, bool) {
    if feet_fallen < 10 {
        return (0, false);
    }
    let dice_count = std::cmp::min(20, feet_fallen / 10);
    // Dano fixo médio de 3.5 por d6 (3.5 * dice_count) ou rolado
    let damage = (dice_count * 7) / 2;
    // Qualquer queda de 10 pés ou mais aplica a condição Prone ao tocar o chão
    (damage as i32, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::background::create_acolyte;
    use crate::rules::character::class::{ClassVariant, Fighter};
    use crate::rules::character::race::{Human, RaceVariant};

    #[test]
    fn test_long_rest_restores_hp() {
        let attrs = AbilityScores::new(14, 14, 14, 10, 10, 10);
        let mut hero = Entity::new(
            "Fighter",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        // Reduz o HP do personagem
        hero.current_hp = 2;
        assert!(hero.current_hp < hero.get_max_hp());

        // Aplica o descanso longo
        hero.long_rest();

        // HP deve ter sido totalmente restaurado
        assert_eq!(hero.current_hp, hero.get_max_hp());
    }

    #[test]
    fn test_movement_speed_creation() {
        let speed = MovementSpeed::walk(30);
        assert_eq!(speed.speed_type, MovementType::Walk);
        assert_eq!(speed.feet, 30);
    }
}
