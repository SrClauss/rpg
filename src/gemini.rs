use base64::Engine;
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;
use std::time::Duration;
use tokio::fs;
use tokio::time::timeout;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapMetadata {
    #[serde(default)]
    pub id: String,
    pub theme: String,
    pub description: String,
    pub tags: Vec<String>,
    pub width_hexes: usize,
    pub height_hexes: usize,
    #[serde(default)]
    pub regions: Option<Value>,
}


#[derive(Debug, Clone)]
pub struct BattleMapImages {
    pub original_path: String,
    pub grid_path: String,
    pub metadata_path: String,
}


pub struct GeminiClient {
    api_key: String,
    client: Client,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::builder()
                .timeout(Duration::from_secs(20))
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }

    /// Gera uma imagem através da API do Gemini (`gemini-3.1-flash-image`) com timeout de 15s.
    /// Retorna os bytes PNG decodificados ou um Erro em caso de falha.
    pub async fn generate_image(&self, prompt: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.1-flash-image:generateContent?key={}",
            self.api_key
        );

        let body = json!({
            "contents": [
                {
                    "parts": [
                        {
                            "text": prompt
                        }
                    ]
                }
            ],
            "generationConfig": {
                "responseModalities": ["IMAGE"]
            }
        });

        // Timeout estrito de 15 segundos para a requisição
        let response_res = timeout(
            Duration::from_secs(15),
            self.client.post(&url).json(&body).send(),
        )
        .await;

        let response = match response_res {
            Ok(res) => res?,
            Err(_) => return Err("Timeout de 15s excedido na geração de imagem com Gemini".into()),
        };

        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(format!(
                "Erro na API do Gemini (Status {}): {}",
                status, response_text
            )
            .into());
        }

        let res_json: Value = serde_json::from_str(&response_text)?;

        // Extrai a string base64 dos candidatos retornados
        let parts = res_json["candidates"][0]["content"]["parts"]
            .as_array()
            .ok_or("Resposta do Gemini não contém partes válidas em candidatos")?;

        for part in parts {
            if let Some(inline_data) = part.get("inlineData") {
                if let Some(base64_str) = inline_data["data"].as_str() {
                    let engine = base64::engine::general_purpose::STANDARD;
                    let bytes = engine.decode(base64_str)?;
                    return Ok(bytes);
                }
            }
        }

        Err("Nenhum dado de imagem inlineData retornado pela API do Gemini".into())
    }

    /// Obtém ou gera o par de imagens do campo de batalha:
    /// 1. `original_path`: Imagem limpa gerada pelo Gemini (`map_{hash}_original.png`).
    /// 2. `grid_path`: Imagem com o overlay do grid hexagonal desenhado pelo Rust (`map_{hash}_gradeado.png`).
    pub async fn get_or_generate_battle_map_images(
        &self,
        prompt: &str,
        theme: &str,
        description: &str,
        tags: Vec<String>,
        width_hexes: usize,
        height_hexes: usize,
    ) -> Result<BattleMapImages, Box<dyn Error>> {
        fs::create_dir_all("./maps/cache").await?;

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        prompt.hash(&mut hasher);
        theme.hash(&mut hasher);
        let hash_id = format!("{:x}", hasher.finish());

        let sanitized_theme = theme
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect::<String>();

        let original_path = format!("./maps/cache/map_{}_{}_original.png", sanitized_theme, hash_id);
        let grid_path = format!("./maps/cache/map_{}_{}_gradeado.png", sanitized_theme, hash_id);
        let metadata_path = format!("./maps/cache/map_{}_{}_metadata.json", sanitized_theme, hash_id);

        if fs::metadata(&original_path).await.is_ok() && fs::metadata(&grid_path).await.is_ok() && fs::metadata(&metadata_path).await.is_ok() {
            println!("      -> [CACHE HIT] Ambas as imagens (original e gradeada) reutilizadas do cache!");
            return Ok(BattleMapImages {
                original_path,
                grid_path,
                metadata_path,
            });
        }


        println!("      -> [CACHE MISS] Gerando imagem limpa via Gemini...");
        let scaled_prompt = format!(
            "{} This terrain represents an area of {} by {} D&D units (each unit is a 5ft / 1.5m space). Ensure the scale of objects, trees, and obstacles matches this perspective.",
            prompt, width_hexes, height_hexes
        );
        let image_bytes = self.generate_image(&scaled_prompt).await?;
        fs::write(&original_path, &image_bytes).await?;
        println!("      -> Imagem Original salva em: {}", original_path);

        println!("      -> Desenhando overlay do Grid Hexagonal em Rust...");
        let grid_overlay_bytes = crate::overlay::draw_hex_grid_overlay(&image_bytes, width_hexes, height_hexes)?;
        fs::write(&grid_path, &grid_overlay_bytes).await?;
        println!("      -> Imagem Gradeada salva em: {}", grid_path);

        // Salvar metadados
        let metadata = MapMetadata {
            id: format!("map_{}_{}", sanitized_theme, hash_id),
            theme: theme.to_string(),
            description: description.to_string(),
            tags,
            width_hexes,
            height_hexes,
            regions: None,
        };
        let meta_json = serde_json::to_string_pretty(&metadata)?;
        fs::write(&metadata_path, meta_json).await?;
        println!("      -> Metadados salvos em: {}", metadata_path);

        Ok(BattleMapImages {
            original_path,
            grid_path,
            metadata_path,
        })
    }

    /// Atualiza o arquivo de metadados do mapa adicionando a estrutura 'regions' com o mapeamento tático de terrenos
    /// e sincroniza o documento unificado com o SurrealDB.
    pub async fn save_map_regions(
        &self,
        metadata_path: &str,
        regions: Value,
        db: Option<&crate::db::MapDatabase>,
    ) -> Result<(), Box<dyn Error>> {
        if let Ok(content) = fs::read_to_string(metadata_path).await {
            let mut metadata: MapMetadata = serde_json::from_str(&content)?;
            // Se o JSON de resposta contiver a chave "regions", usamos seu valor, caso contrário usamos a estrutura completa
            let regions_value = if let Some(inner) = regions.get("regions") {
                inner.clone()
            } else {
                regions
            };
            metadata.regions = Some(regions_value);

            let updated_json = serde_json::to_string_pretty(&metadata)?;
            fs::write(metadata_path, updated_json).await?;
            println!("      -> Mapeamento de regiões unificado no metadados: {}", metadata_path);

            if let Some(database) = db {
                let _ = database.save_map(&metadata).await?;
                println!("      -> Metadados do mapa indexados no SurrealDB com sucesso!");
            }
        }
        Ok(())
    }




    /// Busca no cache de mapas usando as tags fornecidas
    pub async fn search_maps_by_tag(&self, search_tags: &[String]) -> Result<Vec<MapMetadata>, Box<dyn Error>> {
        let mut results = Vec::new();
        let mut dir = match fs::read_dir("./maps/cache").await {
            Ok(d) => d,
            Err(_) => return Ok(results), // Diretório não existe, cache vazio
        };

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path).await {
                    if let Ok(meta) = serde_json::from_str::<MapMetadata>(&content) {
                        // Verifica se há interseção de tags (pelo menos uma tag em comum, ignorando case)
                        let has_match = search_tags.iter().any(|st| {
                            meta.tags.iter().any(|mt| mt.eq_ignore_ascii_case(st))
                        });
                        if has_match {
                            results.push(meta);
                        }
                    }
                }
            }
        }
        Ok(results)
    }
}

