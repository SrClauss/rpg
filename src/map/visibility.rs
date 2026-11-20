use crate::map::grid::{BattleGrid, GridPosition, Posture, TerrainType};

/// Realiza a interpolação linear 3D entre duas coordenadas no Grid Quadrado
pub fn grid_line(start: GridPosition, end: GridPosition) -> Vec<GridPosition> {
    let distance = start.distance(&end);
    if distance == 0 {
        return vec![start];
    }

    let mut line = Vec::with_capacity((distance + 1) as usize);

    let s_x = start.x as f32;
    let s_y = start.y as f32;
    let s_z = start.z as f32;

    let e_x = end.x as f32;
    let e_y = end.y as f32;
    let e_z = end.z as f32;

    for i in 0..=distance {
        let t = i as f32 / distance as f32;
        let x = (s_x + (e_x - s_x) * t).round() as i32;
        let y = (s_y + (e_y - s_y) * t).round() as i32;
        // Arredonda elevação para múltiplos de 5 pés (altura padrão de células do grid)
        let z = (((s_z + (e_z - s_z) * t) / 5.0).round() as i32) * 5;

        line.push(GridPosition::with_z(x, y, z));
    }

    line
}

pub use grid_line as hex_line;

/// Verifica se há Linha de Visão (LoS) 3D desobstruída entre o observador e um alvo.
pub fn has_line_of_sight(
    grid: &BattleGrid,
    observer_pos: GridPosition,
    target_pos: GridPosition,
    target_posture: Posture,
) -> bool {
    let line = grid_line(observer_pos, target_pos);

    for square in line.iter().skip(1) {
        if *square == target_pos {
            if let Some((TerrainType::Obstacle, _)) = grid.get_terrain_at(square) {
                return false;
            }
            break;
        }

        if let Some((terrain, _)) = grid.get_terrain_at(square) {
            match terrain {
                TerrainType::Obstacle => return false,
                TerrainType::CoverHalf | TerrainType::CoverThreeQuarters
                    if target_posture == Posture::Prone =>
                {
                    return false;
                }
                _ => {}
            }
        }
    }

    true
}

/// Calcula todos os quadros visíveis no plano z do observador dentro do alcance.
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
                let target_posture = grid
                    .get_token_at(&pos)
                    .map(|t| t.posture)
                    .unwrap_or(Posture::Standing);

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
    fn test_line_of_sight_blocked_by_obstacle() {
        let mut grid = BattleGrid::new(10, 10, "Masmorra".to_string());
        grid.obstacles.push(Obstacle {
            position: GridPosition::new(1, 0),
            terrain_type: TerrainType::Obstacle,
            description: "Parede de Pedra".to_string(),
        });

        assert!(!has_line_of_sight(
            &grid,
            GridPosition::new(0, 0),
            GridPosition::new(2, 0),
            Posture::Standing
        ));
    }

    #[test]
    fn test_3d_line_of_sight_over_low_wall() {
        let mut grid = BattleGrid::new(10, 10, "Pátio".to_string());
        // Muro no chão (z = 0)
        grid.obstacles.push(Obstacle {
            position: GridPosition::with_z(1, 0, 0),
            terrain_type: TerrainType::Obstacle,
            description: "Muro Baixo".to_string(),
        });

        // Observador no topo de uma torre (z = 20) olhando para alvo no alto (z = 20) vê por cima do muro
        assert!(has_line_of_sight(
            &grid,
            GridPosition::with_z(0, 0, 20),
            GridPosition::with_z(2, 0, 20),
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

        assert!(has_line_of_sight(&grid, observer, target, Posture::Standing));
        assert!(!has_line_of_sight(&grid, observer, target, Posture::Prone));
    }
}