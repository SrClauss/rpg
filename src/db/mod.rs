use serde_json::Value;
use crate::map::MapMetadata;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;
use std::error::Error;


#[derive(Clone)]
pub struct MapDatabase {
    db: Surreal<Db>,
}

impl MapDatabase {
    /// Inicializa a instância embutida do SurrealDB (em memória / kv-mem)
    pub async fn init() -> Result<Self, Box<dyn Error>> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("rpg").use_db("maps").await?;
        Ok(Self { db })
    }

    /// Limpa todos os mapas salvos da tabela 'map_metadata' no SurrealDB
    pub async fn clear_all_maps(&self) -> Result<(), Box<dyn Error>> {
        let _: Vec<Value> = self.db.delete("map_metadata").await?;
        Ok(())
    }



    /// Normaliza uma tag removendo acentos e convertendo para minúsculas
    fn normalize_tag(tag: &str) -> String {
        tag.to_lowercase()
            .chars()
            .map(|c| match c {
                'á' | 'à' | 'ã' | 'â' => 'a',
                'é' | 'ê' => 'e',
                'í' => 'i',
                'ó' | 'ô' | 'õ' => 'o',
                'ú' | 'ü' => 'u',
                'ç' => 'c',
                other => other,
            })
            .collect()
    }

    /// Salva ou atualiza os metadados de um mapa na tabela 'map_metadata'
    pub async fn save_map(&self, metadata: &MapMetadata) -> Result<Vec<Value>, Box<dyn Error>> {
        let mut normalized_meta = metadata.clone();
        normalized_meta.tags = normalized_meta
            .tags
            .iter()
            .map(|t| Self::normalize_tag(t))
            .collect();

        let json_val = serde_json::to_value(&normalized_meta)?;

        let created: Vec<Value> = self
            .db
            .create("map_metadata")
            .content(json_val)
            .await?;

        Ok(created)
    }


    /// Busca mapas no SurrealDB que contenham pelo menos uma das tags fornecidas (normalizadas)
    pub async fn search_maps_by_tags(&self, search_tags: &[String]) -> Result<Vec<MapMetadata>, Box<dyn Error>> {
        let normalized_search: Vec<String> = search_tags.iter().map(|t| Self::normalize_tag(t)).collect();

        let sql = "SELECT theme, description, tags, width_hexes, height_hexes FROM map_metadata";
        let mut response = self.db.query(sql).await?;
        let all_maps: Vec<Value> = response.take(0)?;

        let mut matched_maps = Vec::new();
        for val in all_maps {
            if let Ok(meta) = serde_json::from_value::<MapMetadata>(val) {
                let has_match = meta.tags.iter().any(|mt| {
                    let norm_mt = Self::normalize_tag(mt);
                    normalized_search.iter().any(|st| {
                        norm_mt.contains(st)
                            || st.contains(&norm_mt)
                            || norm_mt.split(&['_', '-', ' '][..]).any(|part| part == st || st.contains(part))
                    })
                });

                if has_match {
                    matched_maps.push(meta);
                }
            }
        }

        Ok(matched_maps)
    }








}
