use crate::map::BattleGrid;
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
                "description": "Gera um mapa de combate tático hexagonal de RPG de mesa (D&D 5e) com coordenadas axiais (q, r), obstáculos, tokens e prompt de imagem visual.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "width": { "type": "integer", "description": "Largura do grid em células (ex: 8 a 15)" },
                        "height": { "type": "integer", "description": "Altura do grid em células (ex: 6 a 12)" },
                        "theme": { "type": "string", "description": "Tema/nome do ambiente (ex: 'Caverna dos Goblins', 'Ponte de Madeira em Ruínas')" },
                        "gemini_image_prompt": {
                            "type": "string",
                            "description": "Prompt detalhado em inglês para IA de imagem (Gemini). OBRIGATÓRIO incluir termos de perspectiva superior pura e mapa limpo: 'Strict 90-degree direct overhead top-down view, 2D RPG battlemap texture, flat bird-eye aerial perspective, no 3D horizon, no perspective tilt, seamless tabletop map wallpaper, GRIDLESS, NO GRIDS, clean floor (a grade será desenhada depois)'."
                        },
                        "image_description": {
                            "type": "string",
                            "description": "Descrição textual concisa sobre a geografia visual do mapa (ex: 'Caverna vulcânica com um rio de lava cruzando o centro e pedras ao redor')."
                        },
                        "image_tags": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Lista de palavras-chave que descrevem o mapa para indexação (ex: ['caverna', 'lava', 'vulcão', 'pedras'])."
                        },
                        "regions": {
                            "type": "array",
                            "description": "Regiões geográficas contínuas do terreno que agrupam múltiplos hexágonos (ex: 'rio_de_lava', 'barricada_de_barris', 'estalagmites')",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "name": { "type": "string", "description": "Nome da região (ex: 'rio_central', 'barrilada')" },
                                    "terrain_type": {
                                        "type": "string",
                                        "enum": ["Obstacle", "Difficult", "CoverHalf", "CoverThreeQuarters"]
                                    },
                                    "description": { "type": "string" },
                                    "hexes": {
                                        "type": "array",
                                        "description": "Lista de hexágonos axiais (q, r) pertencentes a esta região",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "q": { "type": "integer" },
                                                "r": { "type": "integer" }
                                            },
                                            "required": ["q", "r"]
                                        }
                                    }
                                },
                                "required": ["name", "terrain_type", "description", "hexes"]
                            }
                        },
                        "spawn_points_players": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "q": { "type": "integer" },
                                    "r": { "type": "integer" }
                                },
                                "required": ["q", "r"]
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
                                            "q": { "type": "integer" },
                                            "r": { "type": "integer" }
                                        },
                                        "required": ["q", "r"]
                                    },
                                    "hp": { "type": "integer" },
                                    "max_hp": { "type": "integer" },
                                    "speed_ft": { "type": "integer", "description": "Deslocamento em pés (padrão 30)" },
                                    "statblock_id": { "type": "string" }
                                },
                                "required": ["id", "name", "token_type", "position", "hp", "max_hp"]
                            }
                        }
                    },
                    "required": ["width", "height", "theme", "gemini_image_prompt", "image_description", "image_tags", "regions", "spawn_points_players", "tokens"]
                }
            }
        })
    }

    pub async fn generate_grid(&self, prompt: &str) -> Result<(BattleGrid, String, String, Vec<String>), Box<dyn Error>> {
        let url = format!("{}/chat/completions", self.base_url);

        let body = json!({
            "model": "deepseek-chat",
            "messages": [
                {
                    "role": "system",
                    "content": "Você é um Mestre de RPG especialista em D&D 5e. Quando solicitado para criar uma cena de combate, você DEVE SEMPRE invocar a ferramenta `generate_battle_grid`. O hexágono é a unidade atômica base de D&D (5ft). Você DEVE mapear os elementos geográficos do terreno agrupando os hexágonos por regiões (ex: 'rio_central', 'barricada', 'estalagmites'), atribuindo o tipo de terreno e a lista de hexágonos {q, r} pertencentes a cada região. Adicionalmente, no campo `gemini_image_prompt` você DEVE SEMPRE escrever um prompt em inglês que comece com 'Strict 90-degree direct overhead top-down view, 2D D&D battlemap, GRIDLESS, NO GRIDS' para garantir que a arte venha sem grade (a grade matemática será sobreposta programaticamente depois)."
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

        let args_val: Value = serde_json::from_str(arguments_str)?;

        let image_prompt = args_val["gemini_image_prompt"]
            .as_str()
            .unwrap_or("Top-down fantasy RPG battlemap")
            .to_string();

        let image_description = args_val["image_description"]
            .as_str()
            .unwrap_or("Battle map")
            .to_string();

        let image_tags = args_val["image_tags"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|v| v.as_str().unwrap_or("").to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>();

        let grid: BattleGrid = serde_json::from_value(args_val)?;

        Ok((grid, image_prompt, image_description, image_tags))
    }

    /// Envia a imagem do mapa com o overlay contendo as coordenadas (q, r) impressas para o modelo visual do DeepSeek/LLM
    /// para analisar e extrair as regiões físicas de terreno associadas a cada hexágono.
    pub async fn analyze_map_vision(
        &self,
        image_bytes: &[u8],
        prompt: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/chat/completions", self.base_url);
        let base64_image = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_bytes);

        let system_prompt = "Você é um especialista em visão computacional e análise geométrica de mapas táticos de RPG (D&D 5e). \
        Sua tarefa é analisar a imagem do mapa que possui uma grade de hexágonos com as coordenadas 'q,r' visivelmente estampadas no centro de cada célula. \
        Identifique TODOS os tipos de terrenos e objetos presentes no mapa (ex: cadeiras quebradas, viga de madeira caída, mesa central, escombros, área limpa, poços, paredes, água). \
        Para cada tipo de elemento visual, crie uma entrada na lista 'regions' contendo: \
        1. 'name': Nome descritivo da região (ex: 'viga_de_madeira_caida', 'mesas_e_cadeiras_quebradas', 'chao_limpo_da_taverna'). \
        2. 'terrain_type': Tipo tático (um entre: 'Obstacle', 'Difficult', 'CoverHalf', 'CoverThreeQuarters', 'Normal'). \
        3. 'description': Descrição do elemento na imagem. \
        4. 'hexes': Lista contendo TODAS as coordenadas { \"q\": int, \"r\": int } estampadas na imagem que cobrem aquele elemento. \
        Retorne estritamente um JSON com a chave raiz 'regions'.";


        let body = json!({
            "model": "deepseek-chat",
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": prompt
                        },
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": format!("data:image/png;base64,{}", base64_image)
                            }
                        }
                    ]
                }
            ],
            "response_format": { "type": "json_object" }
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
            return Err(format!("Erro na API DeepSeek Vision (Status {}): {}", status, response_text).into());
        }

        let res_json: Value = serde_json::from_str(&response_text)?;
        let content_str = res_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Resposta visual do DeepSeek sem conteúdo textual")?;

        let parsed_regions: Value = serde_json::from_str(content_str)?;
        Ok(parsed_regions)
    }
}


