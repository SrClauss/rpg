use crate::grid::BattleGrid;
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

pub struct DeepSeekClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl DeepSeekClient {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.deepseek.com".to_string()),
            client: Client::new(),
        }
    }

    /// Retorna o schema JSON da ferramenta `generate_battle_grid`
    fn get_grid_tool_schema() -> Value {
        json!({
            "type": "function",
            "function": {
                "name": "generate_battle_grid",
                "description": "Gera um mapa de combate tático de RPG de mesa (D&D 5e) com dimensões, obstáculos e tokens de heróis e monstros baseados na premissa narrativa fornecida.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "width": { "type": "integer", "description": "Largura do grid em células (ex: 8 a 15)" },
                        "height": { "type": "integer", "description": "Altura do grid em células (ex: 6 a 12)" },
                        "theme": { "type": "string", "description": "Tema/nome do ambiente (ex: 'Caverna dos Goblins', 'Taverna em Chamas')" },
                        "obstacles": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "position": {
                                        "type": "object",
                                        "properties": {
                                            "x": { "type": "integer" },
                                            "y": { "type": "integer" }
                                        },
                                        "required": ["x", "y"]
                                    },
                                    "terrain_type": {
                                        "type": "string",
                                        "enum": ["Obstacle", "Difficult", "CoverHalf", "CoverThreeQuarters"]
                                    },
                                    "description": { "type": "string" }
                                },
                                "required": ["position", "terrain_type", "description"]
                            }
                        },
                        "spawn_points_players": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "x": { "type": "integer" },
                                    "y": { "type": "integer" }
                                },
                                "required": ["x", "y"]
                            }
                        },
                        "tokens": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "id": { "type": "string" },
                                    "name": { "type": "string" },
                                    "token_type": {
                                        "type": "string",
                                        "enum": ["Hero", "Enemy", "Neutral"]
                                    },
                                    "position": {
                                        "type": "object",
                                        "properties": {
                                            "x": { "type": "integer" },
                                            "y": { "type": "integer" }
                                        },
                                        "required": ["x", "y"]
                                    },
                                    "hp": { "type": "integer" },
                                    "max_hp": { "type": "integer" },
                                    "statblock_id": { "type": "string" }
                                },
                                "required": ["id", "name", "token_type", "position", "hp", "max_hp"]
                            }
                        }
                    },
                    "required": ["width", "height", "theme", "obstacles", "spawn_points_players", "tokens"]
                }
            }
        })
    }

    pub async fn generate_grid(&self, prompt: &str) -> Result<BattleGrid, Box<dyn Error>> {
        let url = format!("{}/chat/completions", self.base_url);

        let body = json!({
            "model": "deepseek-chat",
            "messages": [
                {
                    "role": "system",
                    "content": "Você é um Mestre de RPG especialista em D&D 5e. Quando solicitado para criar uma cena de combate, você DEVE SEMPRE invocar a ferramenta `generate_battle_grid` fornecendo um mapa tático equilibrado e coerente com a narrativa."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "tools": [Self::get_grid_tool_schema()],
            "tool_choice": {
                "type": "function",
                "function": { "name": "generate_battle_grid" }
            }
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(format!("Erro na API DeepSeek (Status {}): {}", status, response_text).into());
        }

        let res_json: Value = serde_json::from_str(&response_text)?;
        
        // Extrai os argumentos da Tool Call chamada pelo LLM
        let tool_calls = res_json["choices"][0]["message"]["tool_calls"]
            .as_array()
            .ok_or("LLM não retornou tool_calls válidos")?;

        if tool_calls.is_empty() {
            return Err("LLM retornou tool_calls vazio".into());
        }

        let arguments_str = tool_calls[0]["function"]["arguments"]
            .as_str()
            .ok_or("Argumentos da função não são uma string válida")?;

        let grid: BattleGrid = serde_json::from_str(arguments_str)?;

        Ok(grid)
    }
}
