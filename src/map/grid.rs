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

/// Aliases mantidos para compatibilidade com módulos legados
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

    /// Regra padrão D&D 5e para grid quadrado: cada passo adjacente (cardeal ou diagonal) custa 5 pés.
    pub fn step_cost_ft(&self, target: &GridPosition) -> f32 {
        let horiz = self.horizontal_distance(target) as f32 * 5.0;
        let climb = (target.z - self.z).max(0) as f32;
        horiz.max(climb)
    }

    /// Distância Euclidiana 2D exata em pés (usada para áreas de efeito circulares como Fireball)
    pub fn horizontal_distance_ft(&self, other: &GridPosition) -> f32 {
        let dx = (self.x - other.x).abs() as f32 * 5.0;
        let dy = (self.y - other.y).abs() as f32 * 5.0;
        (dx * dx + dy * dy).sqrt()
    }

    /// Distância 2D em quadros (Chebyshev — padrão D&D 5e)
    pub fn horizontal_distance(&self, other: &GridPosition) -> i32 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        std::cmp::max(dx, dy)
    }

    /// Distância 3D em quadros considerando elevação (1 quadro vertical = 5 pés)
    pub fn distance(&self, other: &GridPosition) -> i32 {
        let horizontal_quads = self.horizontal_distance(other);
        let vertical_quads = (self.z - other.z).abs() / 5;
        std::cmp::max(horizontal_quads, vertical_quads)
    }

    /// Retorna as 8 posições vizinhas adjacentes no mesmo plano z
    pub fn neighbors(&self) -> Vec<GridPosition> {
        vec![
            GridPosition::with_z(self.x, self.y - 1, self.z),
            GridPosition::with_z(self.x, self.y + 1, self.z),
            GridPosition::with_z(self.x + 1, self.y, self.z),
            GridPosition::with_z(self.x - 1, self.y, self.z),
            GridPosition::with_z(self.x + 1, self.y - 1, self.z),
            GridPosition::with_z(self.x - 1, self.y - 1, self.z),
            GridPosition::with_z(self.x + 1, self.y + 1, self.z),
            GridPosition::with_z(self.x - 1, self.y + 1, self.z),
        ]
    }
}

/// Tipos de terreno presentes no campo
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    Normal,
    Difficult,          // Deslocamento custa o dobro
    Obstacle,           // Bloqueia movimento e visão total
    CoverHalf,          // Concede +2 CA
    CoverThreeQuarters, // Concede +5 CA
    Hazard {
        save_dc: u8,
        damage_dice: String,
        condition: Option<crate::rules::conditions::Condition>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Posture {
    Standing,
    Prone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub position: GridPosition,
    pub terrain_type: TerrainType,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainRegion {
    pub name: String,
    pub terrain_type: TerrainType,
    pub description: String,
    pub squares: Vec<GridPosition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    Hero,
    Enemy,
    Neutral,
}

fn default_speed_ft() -> f32 {
    30.0
}

fn default_posture() -> Posture {
    Posture::Standing
}

fn default_color_hex() -> String {
    "#3F51B5".to_string()
}

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
        self.get_terrain_at(pos)
            .map(|(terrain_type, description)| Obstacle {
                position: *pos,
                terrain_type,
                description,
            })
    }

    pub fn get_token_at(&self, pos: &GridPosition) -> Option<&Token> {
        self.tokens.iter().find(|t| t.position == *pos && t.hp > 0)
    }

    /// Aplica um perigo de terreno em área CIRCULAR (distância Euclidiana em pés)
    pub fn apply_area_hazard(
        &mut self,
        center: GridPosition,
        radius_feet: u32,
        terrain_type: TerrainType,
        description: &str,
    ) -> usize {
        let max_radius = radius_feet as f32;
        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = GridPosition::with_z(x as i32, y as i32, center.z);
                if center.horizontal_distance_ft(&pos) <= max_radius {
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

    /// Sincroniza o BattleGrid com o estado ativo da LivingScene,
    /// convertendo obstáculos e posições de combatentes em quadros de grid (x, y)
    pub fn sync_with_living_scene(&mut self, scene: &crate::narrative::living_scene::LivingScene) {
        self.obstacles.clear();
        self.tokens.clear();

        for obs in &scene.obstacles {
            let grid_pos = obs.grid_position();
            let terrain_type = obs.to_terrain_type();
            self.obstacles.push(Obstacle {
                position: grid_pos,
                terrain_type,
                description: format!("{}: {}", obs.name, obs.mechanical_effect),
            });
        }

        for p in &scene.participants {
            if let Some((gx, gy)) = p.grid_position() {
                let token_type = if p.is_player {
                    TokenType::Hero
                } else if p.is_hostile {
                    TokenType::Enemy
                } else {
                    TokenType::Neutral
                };

                self.tokens.push(Token {
                    id: p.id.clone(),
                    name: p.name.clone(),
                    token_type,
                    position: GridPosition::new(gx, gy),
                    hp: p.current_hp,
                    max_hp: p.max_hp,
                    speed_ft: 30.0,
                    statblock_id: None,
                    posture: Posture::Standing,
                    color_hex: if p.is_player { "#eab308".to_string() } else { "#ef4444".to_string() },
                });
            }
        }
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
            "Legenda: H=Heroi, E=Inimigo, #=Parede, ~=Lama/Dificil, !=Perigo, .=Vazio"
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
        assert_eq!(neighbors.len(), 8);
        for n in neighbors {
            assert_eq!(center.distance(&n), 1);
            assert!((center.step_cost_ft(&n) - 5.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_circular_area_hazard() {
        let mut grid = BattleGrid::new(10, 10, "Arena".to_string());
        let center = GridPosition::new(5, 5);
        // Raio de 10 pés: (5,5) + 4 cardeais a 5ft + 4 diagonais a 7.07ft + 4 cardeais a 10ft = 13 quadros
        let affected = grid.apply_area_hazard(center, 10, TerrainType::Difficult, "Lama");
        assert_eq!(affected, 13);
    }

    #[test]
    fn test_is_within_bounds() {
        let grid = BattleGrid::new(10, 8, "Caverna".to_string());
        assert!(grid.is_within_bounds(&GridPosition::new(0, 0)));
        assert!(grid.is_within_bounds(&GridPosition::new(9, 7)));
        assert!(!grid.is_within_bounds(&GridPosition::new(-1, 0)));
        assert!(!grid.is_within_bounds(&GridPosition::new(10, 5)));
    }
}