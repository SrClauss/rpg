use crate::map::grid::{BattleGrid, GridPosition, Posture, TerrainType};

/// Realiza a interpolação linear entre duas coordenadas em Grid Quadrado (Raycasting / Bresenham-like Line)
pub fn grid_line(start: GridPosition, end: GridPosition) -> Vec<GridPosition> {
    let distance = start.distance(&end);
    if distance == 0 {
        return vec![start];
    }

    let mut line = Vec::with_capacity((distance + 1) as usize);

    let s_x = start.x as f32;
    let s_y = start.y as f32;

    let e_x = end.x as f32;
    let e_y = end.y as f32;

    for i in 0..=distance {
        let t = i as f32 / distance as f32;
        let x = (s_x + (e_x - s_x) * t).round() as i32;
        let y = (s_y + (e_y - s_y) * t).round() as i32;

        line.push(GridPosition::with_z(x, y, start.z));
    }

    line
}

pub use grid_line as hex_line;

/// Verifica se há Linha de Visão (LoS) desobstruída entre o observador e um alvo.
/// Leva em consideração a postura do alvo e o tipo de obstáculo/cobertura no caminho.
pub fn has_line_of_sight(
    grid: &BattleGrid,
    observer_pos: GridPosition,
    target_pos: GridPosition,
    target_posture: Posture,
) -> bool {
    let line = grid_line(observer_pos, target_pos);

    // O traçado inclui origem e destino; inspecionamos os quadros intermediários
    for square in line.iter().skip(1) {
        if *square == target_pos {
            // Se o próprio quadro do alvo for um obstáculo total, não é visível
            if let Some((TerrainType::Obstacle, _)) = grid.get_terrain_at(square) {
                return false;
            }
            break;
        }

        if let Some((terrain, _)) = grid.get_terrain_at(square) {
            match terrain {
                TerrainType::Obstacle => {
                    // Obstáculo alto bloqueia visão independentemente da postura
                    return false;
                }
                TerrainType::CoverHalf | TerrainType::CoverThreeQuarters
                    if target_posture == Posture::Prone =>
                {
                    // Cobertura baixa/média bloqueia visão SE o alvo estiver deitado (Prone)
                    return false;
                }
                _ => {}
            }
        }
    }

    true
}

/// Calcula todos os quadros visíveis a partir de um observador no raio máximo de alcance (em quadros).
pub fn calculate_fov(
    grid: &BattleGrid,
    observer_pos: GridPosition,
    _observer_posture: Posture,
    range: i32,
) -> Vec<GridPosition> {
    let mut visible_squares = Vec::new();

    for x in (observer_pos.x - range)..=(observer_pos.x + range) {
        for y in (observer_pos.y - range)..=(observer_pos.y + range) {
            let pos = GridPosition::with_z(x, y, observer_pos.z);
            if observer_pos.distance(&pos) <= range && grid.is_within_bounds(&pos) {
                let target_posture = if let Some(token) = grid.get_token_at(&pos) {
                    token.posture
                } else {
                    Posture::Standing
                };

                if has_line_of_sight(grid, observer_pos, pos, target_posture) {
                    visible_squares.push(pos);
                }
            }
        }
    }

    visible_squares
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::grid::Obstacle;

    #[test]
    fn test_grid_line_generation() {
        let start = GridPosition::new(0, 0);
        let end = GridPosition::new(3, 0);
        let line = grid_line(start, end);

        assert_eq!(line.len(), 4);
        assert_eq!(line[0], GridPosition::new(0, 0));
        assert_eq!(line[3], GridPosition::new(3, 0));
    }

    #[test]
    fn test_line_of_sight_blocked_by_obstacle() {
        let mut grid = BattleGrid::new(10, 10, "Masmorra".to_string());
        grid.obstacles.push(Obstacle {
            position: GridPosition::new(1, 0),
            terrain_type: TerrainType::Obstacle,
            description: "Parede de Pedra".to_string(),
        });

        let observer = GridPosition::new(0, 0);
        let target = GridPosition::new(2, 0);

        assert!(!has_line_of_sight(
            &grid,
            observer,
            target,
            Posture::Standing
        ));
    }

    #[test]
    fn test_line_of_sight_prone_cover_interaction() {
        let mut grid = BattleGrid::new(10, 10, "Sala".to_string());
        grid.obstacles.push(Obstacle {
            position: GridPosition::new(1, 0),
            terrain_type: TerrainType::CoverHalf,
            description: "Mesa de Madeira".to_string(),
        });

        let observer = GridPosition::new(0, 0);
        let target = GridPosition::new(2, 0);

        // Se o alvo está em pé (Standing), fica visível através da cobertura baixa
        assert!(has_line_of_sight(
            &grid,
            observer,
            target,
            Posture::Standing
        ));

        // Se o alvo está agachado/deitado (Prone), fica escondido atrás da mureta
        assert!(!has_line_of_sight(&grid, observer, target, Posture::Prone));
    }
}
