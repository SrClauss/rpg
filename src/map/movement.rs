use crate::dice::Dice;
use crate::map::grid::{BattleGrid, GridPosition, Posture, TerrainType};
use crate::rules::conditions::Condition;
use serde::{Deserialize, Serialize};

/// Relatório de Resultado do Movimento enviado para o Frontend e para a IA DM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementResult {
    pub token_id: String,
    pub original_position: GridPosition,
    pub final_position: GridPosition,
    pub distance_traveled_ft: f32,
    pub remaining_speed_ft: f32,
    pub is_halted: bool,
    pub halt_reason: Option<String>,
    pub applied_conditions: Vec<Condition>,
    pub events_log: Vec<String>,
}

/// Executa a movimentação passo a passo de um token ao longo de um trajeto no Grid Quadrado.
pub fn move_token(
    grid: &mut BattleGrid,
    token_id: &str,
    path: Vec<GridPosition>,
    dexterity_modifier: i32,
) -> Result<MovementResult, String> {
    let token_idx = grid
        .tokens
        .iter()
        .position(|t| t.id == token_id)
        .ok_or_else(|| format!("Token com id '{}' não foi encontrado no grid.", token_id))?;

    let original_position = grid.tokens[token_idx].position;
    let mut current_position = original_position;
    let mut remaining_speed = grid.tokens[token_idx].speed_ft;
    let mut distance_traveled = 0.0f32;
    let mut is_halted = false;
    let mut halt_reason = None;
    let mut applied_conditions = Vec::new();
    let mut events_log = Vec::new();

    for next_pos in path {
        if current_position == next_pos {
            continue;
        }

        // Garante que os passos são adjacentes no plano do grid (cardeal ou diagonal)
        if current_position.horizontal_distance(&next_pos) != 1 {
            return Err(format!(
                "Passo inválido de {:?} para {:?}: os quadros devem ser adjacentes.",
                current_position, next_pos
            ));
        }

        let (terrain, desc) = grid
            .get_terrain_at(&next_pos)
            .unwrap_or((TerrainType::Normal, "Chão Limpo".to_string()));

        // Calcula o custo base em pés via Teorema de Pitágoras (Arredondado para Cima: 5ft reto, 8ft diagonal)
        let base_cost = current_position.step_cost_ft(&next_pos);

        // Se for terreno difícil, o custo é dobrado
        let step_cost = match terrain {
            TerrainType::Difficult => base_cost * 2.0,
            TerrainType::Obstacle => {
                is_halted = true;
                halt_reason = Some(format!(
                    "Movimento bloqueado por obstáculo em {:?}.",
                    next_pos
                ));
                events_log.push(format!(
                    "Token bateu em obstáculo ({}) no quadro {:?}.",
                    desc, next_pos
                ));
                break;
            }
            _ => base_cost,
        };

        if remaining_speed < step_cost {
            is_halted = true;
            halt_reason = Some("Deslocamento insuficiente para continuar.".to_string());
            events_log.push(format!(
                "Deslocamento esgotado antes de entrar no quadro {:?}.",
                next_pos
            ));
            break;
        }

        // Aplica o movimento para o próximo quadro
        let drop_distance_ft = current_position.z - next_pos.z;
        remaining_speed -= step_cost;
        distance_traveled += step_cost;
        current_position = next_pos;

        // Se houver uma queda de altura (drop_distance_ft >= 10ft)
        if drop_distance_ft >= 10 {
            let fall_dice_count = std::cmp::min(20, drop_distance_ft / 10);
            let fall_dice_str = format!("{}d6", fall_dice_count);
            let mut fall_damage = 0;
            if let Ok(dice) = fall_dice_str.parse::<Dice>() {
                fall_damage = dice.roll().total;
            }

            grid.tokens[token_idx].hp = (grid.tokens[token_idx].hp - fall_damage).max(0);
            grid.tokens[token_idx].posture = Posture::Prone;
            applied_conditions.push(Condition::Prone);
            is_halted = true;
            halt_reason = Some(format!(
                "Caiu de uma altura de {} pés e sofreu {} de dano contundente.",
                drop_distance_ft, fall_damage
            ));
            events_log.push(format!(
                "Queda! Token caiu {} pés em {:?}, sofreu {} de dano de queda e ficou Prone.",
                drop_distance_ft, next_pos, fall_damage
            ));
            break;
        }

        // Verifica perigos no terreno (Hazards - ex: óleo, graxa, abrolhos)
        if let TerrainType::Hazard {
            save_dc,
            ref damage_dice,
            condition,
        } = terrain
        {
            events_log.push(format!(
                "Entrou em terreno de risco ({}) no quadro {:?}. Teste de Salvaguarda CD {}.",
                desc, next_pos, save_dc
            ));

            // Simula d20 + mod de destreza
            let d20 = Dice::d20().roll();
            let total_save = d20.total + dexterity_modifier;

            if total_save < save_dc as i32 {
                is_halted = true;
                events_log.push(format!(
                    "Falhou na Salvaguarda de Destreza (Rolou {} + {} = {} vs CD {}).",
                    d20.total, dexterity_modifier, total_save, save_dc
                ));

                if let Some(cond) = condition {
                    applied_conditions.push(cond);
                    if cond == Condition::Prone {
                        grid.tokens[token_idx].posture = Posture::Prone;
                        halt_reason = Some(format!(
                            "Escorregou e caiu (Prone) no quadro {:?}.",
                            next_pos
                        ));
                        events_log.push(format!("Token ficou agachado/deitado (Prone) no quadro {:?}. Movimento interrompido!", next_pos));
                    }
                }

                if damage_dice != "0" && !damage_dice.is_empty() {
                    if let Ok(dice) = damage_dice.parse::<Dice>() {
                        let rolled_damage = dice.roll().total;
                        grid.tokens[token_idx].hp =
                            (grid.tokens[token_idx].hp - rolled_damage).max(0);
                        events_log.push(format!(
                            "Recebeu {} de dano de perigo no terreno.",
                            rolled_damage
                        ));
                    }
                }

                break;
            } else {
                events_log.push(format!(
                    "Passou na Salvaguarda de Destreza (Rolou {} + {} = {} vs CD {}). Continua o movimento.",
                    d20.total, dexterity_modifier, total_save, save_dc
                ));
            }
        }
    }

    // Atualiza a posição final do token no grid
    grid.tokens[token_idx].position = current_position;

    Ok(MovementResult {
        token_id: token_id.to_string(),
        original_position,
        final_position: current_position,
        distance_traveled_ft: distance_traveled,
        remaining_speed_ft: remaining_speed,
        is_halted,
        halt_reason,
        applied_conditions,
        events_log,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::grid::{Obstacle, Token, TokenType};

    #[test]
    fn test_normal_movement() {
        let mut grid = BattleGrid::new(10, 10, "Arena".to_string());
        grid.tokens.push(Token {
            id: "hero".to_string(),
            name: "Guerreiro".to_string(),
            token_type: TokenType::Hero,
            position: GridPosition::new(0, 0),
            hp: 30,
            max_hp: 30,
            speed_ft: 30.0,
            statblock_id: None,
            posture: Posture::Standing,
            color_hex: "#00FF00".to_string(),
        });

        let path = vec![
            GridPosition::new(1, 0),
            GridPosition::new(2, 0),
            GridPosition::new(3, 0),
        ];

        let result = move_token(&mut grid, "hero", path, 2).unwrap();
        assert_eq!(result.final_position, GridPosition::new(3, 0));
        assert!((result.distance_traveled_ft - 15.0).abs() < 0.01);
        assert!((result.remaining_speed_ft - 15.0).abs() < 0.01);
        assert!(!result.is_halted);
    }

    #[test]
    fn test_movement_interrupted_by_hazard() {
        let mut grid = BattleGrid::new(10, 10, "Arena".to_string());
        grid.tokens.push(Token {
            id: "hero".to_string(),
            name: "Guerreiro".to_string(),
            token_type: TokenType::Hero,
            position: GridPosition::new(0, 0),
            hp: 30,
            max_hp: 30,
            speed_ft: 30.0,
            statblock_id: None,
            posture: Posture::Standing,
            color_hex: "#00FF00".to_string(),
        });

        grid.obstacles.push(Obstacle {
            position: GridPosition::new(1, 0),
            terrain_type: TerrainType::Hazard {
                save_dc: 99, // Força a falha
                damage_dice: "1d6".to_string(),
                condition: Some(Condition::Prone),
            },
            description: "Mancha de Óleo".to_string(),
        });

        let path = vec![GridPosition::new(1, 0), GridPosition::new(2, 0)];

        let result = move_token(&mut grid, "hero", path, 0).unwrap();
        assert_eq!(result.final_position, GridPosition::new(1, 0));
        assert!(result.is_halted);
        assert_eq!(grid.tokens[0].posture, Posture::Prone);
    }

    #[test]
    fn test_falling_damage_and_prone() {
        let mut grid = BattleGrid::new(10, 10, "Torre".to_string());
        grid.tokens.push(Token {
            id: "hero".to_string(),
            name: "Guerreiro".to_string(),
            token_type: TokenType::Hero,
            position: GridPosition::with_z(0, 0, 30), // Ledge em 30 pés de altura
            hp: 50,
            max_hp: 50,
            speed_ft: 30.0,
            statblock_id: None,
            posture: Posture::Standing,
            color_hex: "#00FF00".to_string(),
        });

        let path = vec![GridPosition::with_z(1, 0, 0)];

        let result = move_token(&mut grid, "hero", path, 0).unwrap();
        assert_eq!(result.final_position, GridPosition::with_z(1, 0, 0));
        assert!(result.is_halted);
        assert!(grid.tokens[0].hp < 50); // Sofreu dano de queda (3d6)
        assert_eq!(grid.tokens[0].posture, Posture::Prone);
    }

    #[test]
    fn test_falling_into_valley_negative_z() {
        let mut grid = BattleGrid::new(10, 10, "Ravina".to_string());
        grid.tokens.push(Token {
            id: "hero".to_string(),
            name: "Guerreiro".to_string(),
            token_type: TokenType::Hero,
            position: GridPosition::with_z(0, 0, 0), // Solo plano padrão (z = 0)
            hp: 50,
            max_hp: 50,
            speed_ft: 30.0,
            statblock_id: None,
            posture: Posture::Standing,
            color_hex: "#00FF00".to_string(),
        });

        let path = vec![GridPosition::with_z(1, 0, -20)];

        let result = move_token(&mut grid, "hero", path, 0).unwrap();
        assert_eq!(result.final_position, GridPosition::with_z(1, 0, -20));
        assert!(result.is_halted);
        assert!(grid.tokens[0].hp < 50); // Sofreu 2d6 de dano de queda ao cair no vale
        assert_eq!(grid.tokens[0].posture, Posture::Prone);
    }

    #[test]
    fn test_diagonal_pythagoras_movement() {
        let mut grid = BattleGrid::new(10, 10, "Arena".to_string());
        grid.tokens.push(Token {
            id: "hero".to_string(),
            name: "Guerreiro".to_string(),
            token_type: TokenType::Hero,
            position: GridPosition::new(0, 0),
            hp: 30,
            max_hp: 30,
            speed_ft: 30.0,
            statblock_id: None,
            posture: Posture::Standing,
            color_hex: "#00FF00".to_string(),
        });

        // Movimento na diagonal: (0,0) -> (1,1) -> (2,2)
        // Cada passo diagonal via Pitágoras em f32 = sqrt(5^2 + 5^2) = ~7.071068 pés
        let path = vec![GridPosition::new(1, 1), GridPosition::new(2, 2)];

        let result = move_token(&mut grid, "hero", path, 0).unwrap();
        assert_eq!(result.final_position, GridPosition::new(2, 2));
        assert!((result.distance_traveled_ft - 14.142136).abs() < 0.01); // 7.071 + 7.071 = 14.14 pés
        assert!((result.remaining_speed_ft - 15.857864).abs() < 0.01); // 30 - 14.14 = 15.86 pés
    }
}
