use serde::{Deserialize, Serialize};
use std::fmt;

/// Coordenada no grid 2D (1 unidade = 5 pés / 1.5m)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
    pub position: Position,
    pub terrain_type: TerrainType,
    pub description: String,
}

/// Tipo de Token no campo de batalha
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    Hero,
    Enemy,
    Neutral,
}

/// Token representando um personagem ou monstro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub name: String,
    pub token_type: TokenType,
    pub position: Position,
    pub hp: i32,
    pub max_hp: i32,
    pub statblock_id: Option<String>,
}

/// O campo de batalha completo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleGrid {
    pub width: usize,
    pub height: usize,
    pub theme: String,
    pub obstacles: Vec<Obstacle>,
    pub spawn_points_players: Vec<Position>,
    pub tokens: Vec<Token>,
}

impl BattleGrid {
    pub fn new(width: usize, height: usize, theme: String) -> Self {
        Self {
            width,
            height,
            theme,
            obstacles: Vec::new(),
            spawn_points_players: Vec::new(),
            tokens: Vec::new(),
        }
    }

    pub fn is_within_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && (pos.x as usize) < self.width && (pos.y as usize) < self.height
    }

    pub fn get_obstacle_at(&self, pos: &Position) -> Option<&Obstacle> {
        self.obstacles.iter().find(|o| o.position == *pos)
    }

    pub fn get_token_at(&self, pos: &Position) -> Option<&Token> {
        self.tokens.iter().find(|t| t.position == *pos)
    }
}

impl fmt::Display for BattleGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n=== BATTLEGRID: {} ({}x{}) ===", self.theme.to_uppercase(), self.width, self.height)?;
        
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position::new(x as i32, y as i32);
                let char_repr = if let Some(token) = self.get_token_at(&pos) {
                    match token.token_type {
                        TokenType::Hero => "H ",
                        TokenType::Enemy => "E ",
                        TokenType::Neutral => "N ",
                    }
                } else if let Some(obstacle) = self.get_obstacle_at(&pos) {
                    match obstacle.terrain_type {
                        TerrainType::Obstacle => "# ",
                        TerrainType::Difficult => "~ ",
                        TerrainType::CoverHalf => "=" ,
                        TerrainType::CoverThreeQuarters => "[" ,
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
