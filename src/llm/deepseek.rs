use crate::map::BattleGrid;
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;
use std::time::Duration;

pub struct DeepSeekClient {
    pub api_key: String,
    pub base_url: String,
    pub client: Client,
}

impl DeepSeekClient {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(45))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| "[https://api.deepseek.com](https://api.deepseek.com)".to_string()),
            client,
        }
    }

    /// Atua como Linter Literário: emite parecer crítico e diretrizes de reescrita para prosa
    pub async fn critique_text(
        &self,
        text_to_review: &str,
        guidelines: &str,
    ) -> Result<String, Box<dyn Error>> {
        let system_msg = "Você é um editor literário implacável e árbitro de coerência dramática. \
                          Sua missão é ler o rascunho de uma cena de RPG, identificar e podar impiedosamente: \
                          clichês óbvios, explicações didáticas artificiais (infodump), inconsistências psicológicas \
                          e ritmos truncados. Forneça uma análise concisa (entre 150 e 250 palavras) com instruções \
                          claras para que o escritor reescreva a cena em seu potencial máximo de maturidade, \
                          verossimilhança e densidade sensorial.";

        let user_content = format!(
            "DIRETRIZES DA CENA:\n{}\n\n---\nRASCUNHO PARA REVISÃO:\n{}",
            guidelines, text_to_review
        );

        self.chat_completion(system_msg, &user_content, 0.7, 1000)
            .await
    }

    /// Atua como Linter Técnico para scripts Rhai / regras (baixa temperatura, sem floreio literário)
    pub async fn lint_code(
        &self,
        code_to_review: &str,
        guidelines: &str,
    ) -> Result<String, Box<dyn Error>> {
        let system_msg = "Você é um analisador estático e linter estrito de código Rhai para motor D&D 5e em Rust. \
                          Avalie exclusivamente sintaxe, tipos, assinaturas de função e lógica mecânica. \
                          Seja objetivo e técnico. Se não houver erros, responda exatamente: SCRIPT_VALIDO";

        let user_content = format!(
            "REGRAS DE VALIDAÇÃO:\n{}\n\n---\nCÓDIGO PARA REVISÃO:\n{}",
            guidelines, code_to_review
        );

        self.chat_completion(system_msg, &user_content, 0.1, 800)
            .await
    }

    async fn chat_completion(
        &self,
        system_msg: &str,
        user_content: &str,
        temperature: f32,
        max_tokens: u32,
    ) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/chat/completions", self.base_url);

        let body = json!({
            "model": "deepseek-chat",
            "messages": [
                { "role": "system", "content": system_msg },
                { "role": "user", "content": user_content }
            ],
            "temperature": temperature,
            "max_tokens": max_tokens
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
            return Err(
                format!("Erro na API DeepSeek (Status {}): {}", status, response_text).into(),
            );
        }

        let res_json: Value = serde_json::from_str(&response_text)?;
        let content = res_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(content)
    }

    /// Schema JSON alinhado com BattleGrid (Grid Quadrado: x, y, squares)
    fn get_grid_tool_schema() -> Value {
        json!({
            "type": "function",
            "function": {
                "name": "generate_battle_grid",
                "description": "Gera um mapa de combate tático em grid quadrado de RPG de mesa (D&D 5e) com coordenadas (x, y), regiões de terreno e tokens.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "width": { "type": "integer", "description": "Largura do grid em quadros (ex: 8 a 15)" },
                        "height": { "type": "integer", "description": "Altura do grid em quadros (ex: 6 a 12)" },
                        "theme": { "type": "string", "description": "Tema/nome do ambiente (ex: 'Caverna dos Goblins')" },
                        "regions": {
                            "type": "array",
                            "description": "Regiões contínuas do terreno agrupando múltiplos quadros (x, y)",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "name": { "type": "string" },
                                    "terrain_type": {
                                        "type": "string",
                                        "enum": ["Obstacle", "Difficult", "CoverHalf", "CoverThreeQuarters"]
                                    },
                                    "description": { "type": "string" },
                                    "squares": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "x": { "type": "integer" },
                                                "y": { "type": "integer" }
                                            },
                                            "required": ["x", "y"]
                                        }
                                    }
                                },
                                "required": ["name", "terrain_type", "description", "squares"]
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
                                    "max_hp": { "type": "integer" }
                                },
                                "required": ["id", "name", "token_type", "position", "hp", "max_hp"]
                            }
                        }
                    },
                    "required": ["width", "height", "theme", "regions", "spawn_points_players", "tokens"]
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
                    "content": "Você é um Mestre de RPG especialista em D&D 5e. Quando solicitado para criar uma cena de combate, invoque SEMPRE a ferramenta `generate_battle_grid`. Cada quadro (x, y) representa 5 pés. Agrupe elementos do terreno em `regions` usando `squares: [{x, y}]`."
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
            return Err(format!(
                "Erro na API DeepSeek (Status {}): {}",
                status, response_text
            )
            .into());
        }

        let res_json: Value = serde_json::from_str(&response_text)?;
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
        let grid: BattleGrid = serde_json::from_value(args_val)?;

        Ok(grid)
    }

    pub async fn analyze_map_vision(
        &self,
        image_bytes: &[u8],
        prompt: &str,
    ) -> Result<Value, Box<dyn Error>> {
        let url = format!("{}/chat/completions", self.base_url);
        let base64_image =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_bytes);

        let system_prompt = "Você é um especialista em visão computacional e análise geométrica de mapas táticos de RPG (D&D 5e). \
        Analise a imagem do mapa com grade quadrada (coordenadas 'x,y' em cada célula). \
        Retorne estritamente um JSON com a chave raiz 'regions', onde cada item tem: \
        'name' (string), 'terrain_type' ('Obstacle' | 'Difficult' | 'CoverHalf' | 'CoverThreeQuarters' | 'Normal'), \
        'description' (string) e 'squares' (lista de { \"x\": int, \"y\": int }).";

        let body = json!({
            "model": "deepseek-chat",
            "messages": [
                { "role": "system", "content": system_prompt },
                {
                    "role": "user",
                    "content": [
                        { "type": "text", "text": prompt },
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
            return Err(format!(
                "Erro na API DeepSeek Vision (Status {}): {}",
                status, response_text
            )
            .into());
        }

        let res_json: Value = serde_json::from_str(&response_text)?;
        let content_str = res_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Resposta visual do DeepSeek sem conteúdo textual")?;

        let parsed_regions: Value = serde_json::from_str(content_str)?;
        Ok(parsed_regions)
    }
}