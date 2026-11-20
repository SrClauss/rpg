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

    let mover_type = grid.tokens[token_idx].token_type;
    let original_position = grid.tokens[token_idx].position;
    let mut current_position = original_position;
    let mut remaining_speed = grid.tokens[token_idx].speed_ft;
    let mut distance_traveled = 0.0f32;
    let mut is_halted = false;
    let mut halt_reason = None;
    let mut applied_conditions = Vec::new();
    let mut events_log = Vec::new();

    let path_len = path.len();

    for (step_idx, next_pos) in path.into_iter().enumerate() {
        if current_position == next_pos {
            continue;
        }

        // 1. Limites do mapa
        if !grid.is_within_bounds(&next_pos) {
            is_halted = true;
            halt_reason = Some(format!(
                "Movimento bloqueado: quadro {:?} está fora dos limites do mapa.",
                next_pos
            ));
            events_log.push(halt_reason.clone().unwrap());
            break;
        }

        // 2. Adjacência (permite movimento horizontal de 1 quadro ou escalada/queda vertical no mesmo quadro)
        let horiz_dist = current_position.horizontal_distance(&next_pos);
        if horiz_dist > 1 {
            return Err(format!(
                "Passo inválido de {:?} para {:?}: os quadros devem ser adjacentes.",
                current_position, next_pos
            ));
        }

        // 3. Colisão com outros Tokens vivos
        let mut occupied_by_ally = false;
        if let Some(occupant) = grid.get_token_at(&next_pos) {
            if occupant.id != token_id {
                if occupant.token_type != mover_type {
                    is_halted = true;
                    halt_reason = Some(format!(
                        "Movimento bloqueado por inimigo ({}) em {:?}.",
                        occupant.name, next_pos
                    ));
                    events_log.push(halt_reason.clone().unwrap());
                    break;
                } else {
                    // Regra D&D 5e: pode atravessar espaço de aliado (custa terreno difícil),
                    // mas não pode encerrar o movimento no mesmo quadro.
                    if step_idx == path_len - 1 {
                        is_halted = true;
                        halt_reason = Some(format!(
                            "Não é possível encerrar o movimento no espaço ocupado pelo aliado ({}) em {:?}.",
                            occupant.name, next_pos
                        ));
                        events_log.push(halt_reason.clone().unwrap());
                        break;
                    }
                    occupied_by_ally = true;
                }
            }
        }

        // 4. Terreno e Obstáculos
        let (terrain, desc) = grid
            .get_terrain_at(&next_pos)
            .unwrap_or((TerrainType::Normal, "Chão Limpo".to_string()));

        if matches!(terrain, TerrainType::Obstacle) {
            is_halted = true;
            halt_reason = Some(format!(
                "Movimento bloqueado por obstáculo ({}) em {:?}.",
                desc, next_pos
            ));
            events_log.push(halt_reason.clone().unwrap());
            break;
        }

        // 5. Cálculo de Custo (5e padrão + subida vertical + terreno difícil/aliado)
        let base_cost = current_position.step_cost_ft(&next_pos);
        let is_difficult = matches!(terrain, TerrainType::Difficult) || occupied_by_ally;
        let step_cost = if is_difficult {
            base_cost * 2.0
        } else {
            base_cost
        };

        if remaining_speed < step_cost {
            is_halted = true;
            halt_reason = Some("Deslocamento insuficiente para continuar.".to_string());
            events_log.push(format!(
                "Deslocamento esgotado antes de entrar no quadro {:?} (precisava de {:.0}ft, restavam {:.0}ft).",
                next_pos, step_cost, remaining_speed
            ));
            break;
        }

        // 6. Aplica o movimento
        let drop_distance_ft = current_position.z - next_pos.z;
        remaining_speed -= step_cost;
        distance_traveled += step_cost;
        current_position = next_pos;

        // 7. Dano de queda (>= 10 pés)
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
                "Queda! Token caiu {} pés em {:?}, sofreu {} de dano e ficou Prone.",
                drop_distance_ft, next_pos, fall_damage
            ));
            break;
        }

        // 8. Perigos no terreno (Hazards)
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

            let d20 = Dice::d20().roll();
            let total_save = d20.total + dexterity_modifier;

            if total_save < save_dc as i32 {
                is_halted = true;
                events_log.push(format!(
                    "Falhou na Salvaguarda de Destreza ({} + {} = {} vs CD {}).",
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
                    "Passou na Salvaguarda de Destreza ({} + {} = {} vs CD {}). Continua o movimento.",
                    d20.total, dexterity_modifier, total_save, save_dc
                ));
            }
        }
    }

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

    fn hero_at(x: i32, y: i32, z: i32) -> Token {
        Token {
            id: "hero".to_string(),
            name: "Guerreiro".to_string(),
            token_type: TokenType::Hero,
            position: GridPosition::with_z(x, y, z),
            hp: 30,
            max_hp: 30,
            speed_ft: 30.0,
            statblock_id: None,
            posture: Posture::Standing,
            color_hex: "#00FF00".to_string(),
        }
    }

    #[test]
    fn test_diagonal_5e_movement() {
        let mut grid = BattleGrid::new(10, 10, "Arena".to_string());
        grid.tokens.push(hero_at(0, 0, 0));

        let path = vec![GridPosition::new(1, 1), GridPosition::new(2, 2)];
        let result = move_token(&mut grid, "hero", path, 0).unwrap();

        assert_eq!(result.final_position, GridPosition::new(2, 2));
        assert!((result.distance_traveled_ft - 10.0).abs() < 0.01);
        assert!((result.remaining_speed_ft - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_enemy_blocks_movement() {
        let mut grid = BattleGrid::new(10, 10, "Arena".to_string());
        grid.tokens.push(hero_at(0, 0, 0));
        grid.tokens.push(Token {
            id: "goblin".to_string(),
            name: "Goblin".to_string(),
            token_type: TokenType::Enemy,
            position: GridPosition::new(1, 0),
            hp: 7,
            max_hp: 7,
            speed_ft: 30.0,
            statblock_id: None,
            posture: Posture::Standing,
            color_hex: "#FF0000".to_string(),
        });

        let path = vec![GridPosition::new(1, 0), GridPosition::new(2, 0)];
        let result = move_token(&mut grid, "hero", path, 0).unwrap();

        assert!(result.is_halted);
        assert_eq!(result.final_position, GridPosition::new(0, 0));
    }

    #[test]
    fn test_vertical_climb_consumes_speed() {
        let mut grid = BattleGrid::new(10, 10, "Torre".to_string());
        grid.tokens.push(hero_at(0, 0, 0));

        // Tentar subir 40 pés de uma vez com speed 30ft deve bloquear
        let path = vec![GridPosition::with_z(1, 0, 40)];
        let result = move_token(&mut grid, "hero", path, 0).unwrap();

        assert!(result.is_halted);
        assert_eq!(result.final_position, GridPosition::with_z(0, 0, 0));
    }

    #[test]
    fn test_out_of_bounds_halts_movement() {
        let mut grid = BattleGrid::new(5, 5, "Sala".to_string());
        grid.tokens.push(hero_at(4, 0, 0));

        let path = vec![GridPosition::new(5, 0)];
        let result = move_token(&mut grid, "hero", path, 0).unwrap();

        assert!(result.is_halted);
        assert_eq!(result.final_position, GridPosition::new(4, 0));
    }
}