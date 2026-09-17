use serde::{Deserialize, Serialize};
use std::fmt;

/// Coordenada axial em grid hexagonal (q = coluna, r = linha)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HexPosition {
    pub q: i32,
    pub r: i32,
}

impl HexPosition {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    /// Calcula a distância em hexágonos (distância cúbica)
    pub fn distance(&self, other: &HexPosition) -> i32 {
        let dq = self.q - other.q;
        let dr = self.r - other.r;
        let ds = (-self.q - self.r) - (-other.q - other.r);
        (dq.abs() + dr.abs() + ds.abs()) / 2
    }

    /// Retorna as 6 posições vizinhas adjacentes
    pub fn neighbors(&self) -> Vec<HexPosition> {
        vec![
            HexPosition::new(self.q + 1, self.r),
            HexPosition::new(self.q - 1, self.r),
            HexPosition::new(self.q, self.r + 1),
            HexPosition::new(self.q, self.r - 1),
            HexPosition::new(self.q + 1, self.r - 1),
            HexPosition::new(self.q - 1, self.r + 1),
        ]
    }
}

/// Tipos de terreno presentes no campo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    Normal,
    Difficult,          // Deslocamento custa o dobro
    Obstacle,           // Bloqueia movimento e visão total
    CoverHalf,          // Concede +2 CA (ex: barril)
    CoverThreeQuarters, // Concede +5 CA (ex: fenda)
}

/// Obstáculo posicionado no mapa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub position: HexPosition,
    pub terrain_type: TerrainType,
    pub description: String,
}

/// Região de terreno contínua agrupando múltiplos hexágonos (ex: rio_central, floresta_densa)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainRegion {
    pub name: String,
    pub terrain_type: TerrainType,
    pub description: String,
    pub hexes: Vec<HexPosition>,
}

/// Tipo de Token no campo de batalha
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    Hero,
    Enemy,
    Neutral,
}

fn default_speed_ft() -> i32 {
    30
}

/// Token representando um personagem ou monstro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub name: String,
    pub token_type: TokenType,
    pub position: HexPosition,
    pub hp: i32,
    pub max_hp: i32,
    #[serde(default = "default_speed_ft")]
    pub speed_ft: i32,
    pub statblock_id: Option<String>,
}

/// O campo de batalha completo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleGrid {
    pub width: usize,
    pub height: usize,
    pub theme: String,
    #[serde(default)]
    pub regions: Vec<TerrainRegion>,
    #[serde(default)]
    pub obstacles: Vec<Obstacle>,
    pub spawn_points_players: Vec<HexPosition>,
    pub tokens: Vec<Token>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub map_image_url: Option<String>,
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
            map_image_url: None,
        }
    }

    pub fn is_within_bounds(&self, pos: &HexPosition) -> bool {
        pos.q >= 0 && pos.r >= 0 && (pos.q as usize) < self.width && (pos.r as usize) < self.height
    }

    /// Retorna o tipo de terreno e descrição presente em uma posição hexagonal (seja por região ou obstáculo)
    pub fn get_terrain_at(&self, pos: &HexPosition) -> Option<(TerrainType, String)> {
        if let Some(obs) = self.obstacles.iter().find(|o| o.position == *pos) {
            return Some((obs.terrain_type, obs.description.clone()));
        }
        for region in &self.regions {
            if region.hexes.contains(pos) {
                return Some((region.terrain_type, region.description.clone()));
            }
        }
        None
    }

    pub fn get_obstacle_at(&self, pos: &HexPosition) -> Option<Obstacle> {
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

    pub fn get_token_at(&self, pos: &HexPosition) -> Option<&Token> {
        self.tokens.iter().find(|t| t.position == *pos)
    }
}

impl fmt::Display for BattleGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n=== BATTLEGRID HEXAGONAL: {} ({}x{}) ===", self.theme.to_uppercase(), self.width, self.height)?;
        
        for r in 0..self.height {
            // Deslocamento visual para linhas ímpares
            if r % 2 != 0 {
                write!(f, " ")?;
            }
            for q in 0..self.width {
                let pos = HexPosition::new(q as i32, r as i32);
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
                        TerrainType::Normal => ". ",
                    }
                } else {
                    ". "
                };
                write!(f, "{}", char_repr)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Legenda: H=Heroi, E=Inimigo, #=Parede, ~=Lama/Dificil, .=Vazio")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_distance_is_one() {
        let center = HexPosition::new(0, 0);
        let neighbors = center.neighbors();
        assert_eq!(neighbors.len(), 6);
        for n in neighbors {
            assert_eq!(center.distance(&n), 1, "Distance from {:?} to neighbor {:?} should be 1", center, n);
        }
    }

    #[test]
    fn test_neighbors_count_and_values() {
        let pos = HexPosition::new(3, 3);
        let n = pos.neighbors();
        assert_eq!(n.len(), 6);
        assert!(n.contains(&HexPosition::new(4, 3)));
        assert!(n.contains(&HexPosition::new(2, 3)));
        assert!(n.contains(&HexPosition::new(3, 4)));
        assert!(n.contains(&HexPosition::new(3, 2)));
        assert!(n.contains(&HexPosition::new(4, 2)));
        assert!(n.contains(&HexPosition::new(2, 4)));
    }

    #[test]
    fn test_distance_calculation() {
        let p1 = HexPosition::new(0, 0);
        let p2 = HexPosition::new(2, 1);
        assert_eq!(p1.distance(&p2), 3);

        let p3 = HexPosition::new(-1, -1);
        let p4 = HexPosition::new(1, 1);
        assert_eq!(p3.distance(&p4), 4);
    }

    #[test]
    fn test_is_within_bounds() {
        let grid = BattleGrid::new(10, 8, "Caverna".to_string());
        assert!(grid.is_within_bounds(&HexPosition::new(0, 0)));
        assert!(grid.is_within_bounds(&HexPosition::new(9, 7)));
        assert!(!grid.is_within_bounds(&HexPosition::new(-1, 0)));
        assert!(!grid.is_within_bounds(&HexPosition::new(0, -1)));
        assert!(!grid.is_within_bounds(&HexPosition::new(10, 5)));
        assert!(!grid.is_within_bounds(&HexPosition::new(5, 8)));
    }
}


