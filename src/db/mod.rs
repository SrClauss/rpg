use crate::map::BattleGrid;
use serde_json::Value;
use std::error::Error;
use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

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

    /// Limpa todos os mapas salvos da tabela 'battle_grids' no SurrealDB
    pub async fn clear_all_maps(&self) -> Result<(), Box<dyn Error>> {
        let _: Vec<Value> = self.db.delete("battle_grids").await?;
        Ok(())
    }

    /// Salva ou atualiza um mapa tático na tabela 'battle_grids'
    pub async fn save_grid(&self, grid: &BattleGrid) -> Result<Vec<Value>, Box<dyn Error>> {
        let json_val = serde_json::to_value(grid)?;
        let created: Vec<Value> = self.db.create("battle_grids").content(json_val).await?;
        Ok(created)
    }
}
