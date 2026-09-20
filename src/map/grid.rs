use serde::{Deserialize, Serialize};
use std::fmt;

/// Coordenada cartesiana em grid quadrado (x = coluna, y = linha, z = elevação em pés)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
    #[serde(default)]
    pub z: i32, // Elevação em pés (0 = nível do solo, 10 = 10ft de altura)
}

/// Alias para manter compatibilidade com nomes legados se necessário
pub type HexPosition = GridPosition;

/// 8 Direções Cardeais e Diagonais no Grid Quadrado
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridDirection {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

pub type HexDirection = GridDirection;

impl GridDirection {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            GridDirection::North => (0, -1),
            GridDirection::South => (0, 1),
            GridDirection::East => (1, 0),
            GridDirection::West => (-1, 0),
            GridDirection::NorthEast => (1, -1),
            GridDirection::NorthWest => (-1, -1),
            GridDirection::SouthEast => (1, 1),
            GridDirection::SouthWest => (-1, 1),
        }
    }
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y, z: 0 }
    }

    pub fn with_z(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Retorna a posição N passos em uma direção específica
    pub fn step_in_direction(&self, direction: GridDirection, steps: i32) -> Self {
        let (dx, dy) = direction.delta();
        Self {
            x: self.x + dx * steps,
            y: self.y + dy * steps,
            z: self.z,
        }
    }

    /// Calcula o custo exato de movimento em pés em f32 entre dois quadros adjacentes usando o Teorema de Pitágoras (5ft reto, ~7.07ft diagonal)
    pub fn step_cost_ft(&self, target: &GridPosition) -> f32 {
        let dx = (self.x - target.x).abs() as f32 * 5.0;
        let dy = (self.y - target.y).abs() as f32 * 5.0;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calcula a distância 2D exata em pés no plano do grid usando o Teorema de Pitágoras em f32
    pub fn horizontal_distance_ft(&self, other: &GridPosition) -> f32 {
        let dx = (self.x - other.x).abs() as f32 * 5.0;
        let dy = (self.y - other.y).abs() as f32 * 5.0;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calcula a distância em quadros (grid 2D)
    pub fn horizontal_distance(&self, other: &GridPosition) -> i32 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        std::cmp::max(dx, dy)
    }

    /// Calcula a distância 3D em quadros considerando elevação
    pub fn distance(&self, other: &GridPosition) -> i32 {
        let horizontal_quads = self.horizontal_distance(other);
        let vertical_quads = (self.z - other.z).abs() / 5;
        std::cmp::max(horizontal_quads, vertical_quads)
    }

    /// Retorna as 8 posições vizinhas adjacentes no mesmo plano z (cardeais + diagonais)
    pub fn neighbors(&self) -> Vec<GridPosition> {
        vec![
            GridPosition::with_z(self.x, self.y - 1, self.z), // N
            GridPosition::with_z(self.x, self.y + 1, self.z), // S
            GridPosition::with_z(self.x + 1, self.y, self.z), // E
            GridPosition::with_z(self.x - 1, self.y, self.z), // W
            GridPosition::with_z(self.x + 1, self.y - 1, self.z), // NE
            GridPosition::with_z(self.x - 1, self.y - 1, self.z), // NW
            GridPosition::with_z(self.x + 1, self.y + 1, self.z), // SE
            GridPosition::with_z(self.x - 1, self.y + 1, self.z), // SW
        ]
    }
}

/// Tipos de terreno presentes no campo
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    Normal,
    Difficult,          // Deslocamento custa o dobro
    Obstacle,           // Bloqueia movimento e visão total
    CoverHalf,          // Concede +2 CA (ex: barril)
    CoverThreeQuarters, // Concede +5 CA (ex: fenda)
    Hazard {
        save_dc: u8,
        damage_dice: String,
        condition: Option<crate::rules::conditions::Condition>,
    }, // Terreno perigoso (Ex: Óleo, Graxa)
}

/// Postura do personagem no grid (influencia linha de visão e cobertura)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Posture {
    Standing,
    Prone,
}

/// Obstáculo posicionado no mapa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub position: GridPosition,
    pub terrain_type: TerrainType,
    pub description: String,
}

/// Região de terreno contínua agrupando múltiplos quadros
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainRegion {
    pub name: String,
    pub terrain_type: TerrainType,
    pub description: String,
    pub squares: Vec<GridPosition>,
}

/// Tipo de Token no campo de batalha
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    Hero,
    Enemy,
    Neutral,
}

fn default_speed_ft() -> f32 {
    30.0
}

/// Token representando um personagem ou monstro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub name: String,
    pub token_type: TokenType,
    pub position: GridPosition,
    pub hp: i32,
    pub max_hp: i32,
    #[serde(default = "default_speed_ft")]
    pub speed_ft: f32,
    pub statblock_id: Option<String>,
    #[serde(default = "default_posture")]
    pub posture: Posture,
    #[serde(default = "default_color_hex")]
    pub color_hex: String,
}

fn default_posture() -> Posture {
    Posture::Standing
}

fn default_color_hex() -> String {
    "#3F51B5".to_string()
}

/// O campo de batalha completo em Grid Quadrado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleGrid {
    pub width: usize,
    pub height: usize,
    pub theme: String,
    #[serde(default)]
    pub regions: Vec<TerrainRegion>,
    #[serde(default)]
    pub obstacles: Vec<Obstacle>,
    pub spawn_points_players: Vec<GridPosition>,
    pub tokens: Vec<Token>,
}

impl BattleGrid {
    pub fn new(width: usize, height: usize, theme: String) -> Self {
        Self {
            width,
            height,
            theme,
            regions: Vec::new(),
            obstacles: Vec::new(),
            spawn_points_players: Vec::new(),
            tokens: Vec::new(),
        }
    }

    pub fn is_within_bounds(&self, pos: &GridPosition) -> bool {
        pos.x >= 0 && pos.y >= 0 && (pos.x as usize) < self.width && (pos.y as usize) < self.height
    }

    /// Retorna o tipo de terreno e descrição presente em uma posição do grid
    pub fn get_terrain_at(&self, pos: &GridPosition) -> Option<(TerrainType, String)> {
        if let Some(obs) = self.obstacles.iter().find(|o| o.position == *pos) {
            return Some((obs.terrain_type.clone(), obs.description.clone()));
        }
        for region in &self.regions {
            if region.squares.contains(pos) {
                return Some((region.terrain_type.clone(), region.description.clone()));
            }
        }

        None
    }

    pub fn get_obstacle_at(&self, pos: &GridPosition) -> Option<Obstacle> {
        if let Some((terrain_type, description)) = self.get_terrain_at(pos) {
            Some(Obstacle {
                position: *pos,
                terrain_type,
                description,
            })
        } else {
            None
        }
    }

    pub fn get_token_at(&self, pos: &GridPosition) -> Option<&Token> {
        self.tokens.iter().find(|t| t.position == *pos)
    }

    /// Aplica um perigo de terreno em área no mapa ao redor de uma posição central (Raio em pés)
    pub fn apply_area_hazard(
        &mut self,
        center: GridPosition,
        radius_feet: u32,
        terrain_type: TerrainType,
        description: &str,
    ) -> usize {
        let max_quad_dist = (radius_feet as i32) / 5;
        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = GridPosition::with_z(x as i32, y as i32, center.z);
                if center.horizontal_distance(&pos) <= max_quad_dist {
                    self.obstacles.retain(|o| o.position != pos);
                    self.obstacles.push(Obstacle {
                        position: pos,
                        terrain_type: terrain_type.clone(),
                        description: description.to_string(),
                    });
                    count += 1;
                }
            }
        }

        count
    }

    /// Lança um perigo de terreno a partir do conjurador em uma direção específica a uma distância fixa (em pés)
    pub fn cast_directional_hazard(
        &mut self,
        caster_pos: GridPosition,
        direction: GridDirection,
        distance_feet: u32,
        area_radius_feet: u32,
        terrain_type: TerrainType,
        description: &str,
    ) -> (GridPosition, usize) {
        let steps = (distance_feet as i32) / 5;
        let target_center = caster_pos.step_in_direction(direction, steps);
        let count =
            self.apply_area_hazard(target_center, area_radius_feet, terrain_type, description);
        (target_center, count)
    }
}

impl fmt::Display for BattleGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "\n=== BATTLEGRID QUADRADO: {} ({}x{}) ===",
            self.theme.to_uppercase(),
            self.width,
            self.height
        )?;

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = GridPosition::new(x as i32, y as i32);
                let char_repr = if let Some(token) = self.get_token_at(&pos) {
                    match token.token_type {
                        TokenType::Hero => "H ",
                        TokenType::Enemy => "E ",
                        TokenType::Neutral => "N ",
                    }
                } else if let Some((terrain_type, _)) = self.get_terrain_at(&pos) {
                    match terrain_type {
                        TerrainType::Obstacle => "# ",
                        TerrainType::Difficult => "~ ",
                        TerrainType::CoverHalf => "= ",
                        TerrainType::CoverThreeQuarters => "[ ",
                        TerrainType::Hazard { .. } => "! ",
                        TerrainType::Normal => ". ",
                    }
                } else {
                    ". "
                };
                write!(f, "{}", char_repr)?;
            }
            writeln!(f)?;
        }
        writeln!(
            f,
            "Legenda: H=Heroi, E=Inimigo, #=Parede, ~=Lama/Dificil, .=Vazio"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_distance_is_one() {
        let center = GridPosition::new(0, 0);
        let neighbors = center.neighbors();
        assert_eq!(neighbors.len(), 8); // 8 vizinhos no grid quadrado
        for n in neighbors {
            assert_eq!(center.distance(&n), 1);
        }
    }

    #[test]
    fn test_neighbors_count_and_values() {
        let pos = GridPosition::new(3, 3);
        let n = pos.neighbors();
        assert_eq!(n.len(), 8);
        assert!(n.contains(&GridPosition::new(3, 2))); // N
        assert!(n.contains(&GridPosition::new(3, 4))); // S
        assert!(n.contains(&GridPosition::new(4, 3))); // E
        assert!(n.contains(&GridPosition::new(2, 3))); // W
        assert!(n.contains(&GridPosition::new(4, 2))); // NE
        assert!(n.contains(&GridPosition::new(2, 2))); // NW
        assert!(n.contains(&GridPosition::new(4, 4))); // SE
        assert!(n.contains(&GridPosition::new(2, 4))); // SW
    }

    #[test]
    fn test_distance_calculation() {
        let p1 = GridPosition::new(0, 0);
        let p2 = GridPosition::new(2, 2);
        assert_eq!(p1.horizontal_distance(&p2), 2);

        let p3 = GridPosition::new(-1, -1);
        let p4 = GridPosition::new(1, 1);
        assert_eq!(p3.horizontal_distance(&p4), 2);
    }

    #[test]
    fn test_is_within_bounds() {
        let grid = BattleGrid::new(10, 8, "Caverna".to_string());
        assert!(grid.is_within_bounds(&GridPosition::new(0, 0)));
        assert!(grid.is_within_bounds(&GridPosition::new(9, 7)));
        assert!(!grid.is_within_bounds(&GridPosition::new(-1, 0)));
        assert!(!grid.is_within_bounds(&GridPosition::new(0, -1)));
        assert!(!grid.is_within_bounds(&GridPosition::new(10, 5)));
        assert!(!grid.is_within_bounds(&GridPosition::new(5, 8)));
    }

    #[test]
    fn test_cast_directional_hazard() {
        let mut grid = BattleGrid::new(20, 20, "Caverna".to_string());
        let caster_pos = GridPosition::new(5, 5);

        // Lança Grease na direção Leste a 60 pés (12 quadros de distância), com raio de 10 pés
        let (target_pos, affected) = grid.cast_directional_hazard(
            caster_pos,
            GridDirection::East,
            60,
            10,
            TerrainType::Hazard {
                save_dc: 13,
                damage_dice: "0".to_string(),
                condition: Some(crate::rules::conditions::Condition::Prone),
            },
            "Mancha de Graxa",
        );

        assert_eq!(target_pos, GridPosition::new(17, 5));
        assert!(affected > 0);
        let (terrain, desc) = grid.get_terrain_at(&target_pos).unwrap();
        assert_eq!(desc, "Mancha de Graxa");
        if let TerrainType::Hazard { save_dc, .. } = terrain {
            assert_eq!(save_dc, 13);
        } else {
            panic!("Deveria ser TerrainType::Hazard");
        }
    }
}
