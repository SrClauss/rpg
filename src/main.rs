#![allow(dead_code, unused_imports)]

mod db;
mod dice;
mod llm;
mod map;
pub mod rules;

use dotenvy::dotenv;
use llm::DeepSeekClient;
use map::{GeminiClient, MapMetadata};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("============================================================");
    println!("   MOTOR RPG HEXAGONAL TÁTICO - GERAÇÃO DUPLA DE IMAGENS    ");
    println!("============================================================\n");

    // 1. Inicializa Chaves de API
    let deepseek_key = env::var("DEEPSEEK_API_KEY")
        .expect("Variável DEEPSEEK_API_KEY não encontrada no .env");
    let deepseek_url = env::var("DEEPSEEK_BASE_URL").ok();

    let gemini_key = env::var("GEMINI_API_KEY")
        .expect("Variável GEMINI_API_KEY não encontrada no .env");

    let _deepseek_client = DeepSeekClient::new(deepseek_key, deepseek_url);
    let _gemini_client = GeminiClient::new(gemini_key);

    println!("[0/3] Inicializando SurrealDB (Embutido)...");
    let map_db = db::MapDatabase::init().await?;
    println!("      -> Instância do SurrealDB pronta!\n");

    // 1. Carrega os metadados do mapa da Taverna existente no cache
    let metadata_path = "./maps/cache/map_salão_da_taverna_após_briga_b40101f99a87cda_metadata.json";
    
    println!("\n>>> TESTE DE VALIDAÇÃO DE REUSO DE MAPA POR PROMPT NARRATIVO <<<");
    println!("[1/3] Carregando e Indexando mapa existente no SurrealDB...");
    
    if let Ok(content) = tokio::fs::read_to_string(metadata_path).await {
        if let Ok(meta) = serde_json::from_str::<MapMetadata>(&content) {
            let _ = map_db.save_map(&meta).await;
            println!("      -> Mapa '{}' indexado com sucesso no SurrealDB!", meta.theme);
            println!("      -> Tags no Banco: {:?}", meta.tags);
        }
    }

    // 2. Simula a entrada de uma nova cena narrativa solicitada pelo jogo
    let narrative_prompt = "Os aventureiros entram no salão interno de uma taverna onde acabou de acontecer uma briga generalizada. O lugar está bagunçado, com cadeiras de madeira quebradas espalhadas pelo chão, uma mesa central de madeira e uma viga estrutural tombada.";
    
    println!("\n[2/3] Nova cena solicitada pelo jogo:");
    println!("      Prompt Narrativo: \"{}\"", narrative_prompt);

    // 3. IA Tática (DeepSeek) extrai as tags-chave do encontro
    let query_tags = vec![
        "taverna".to_string(),
        "cadeiras_quebradas".to_string(),
        "viga".to_string(),
        "mesa".to_string(),
        "escombros".to_string(),
    ];
    println!("      Tags extraídas do combate: {:?}", query_tags);

    // 4. Executa a busca no SurrealDB
    println!("\n[3/3] Pesquisando mapas compatíveis no SurrealDB...");
    match map_db.search_maps_by_tags(&query_tags).await {
        Ok(results) => {
            if results.is_empty() {
                println!("      -> [FALHA DE MATCH] Nenhum mapa encontrado (seria gerada uma imagem nova).");
            } else {
                println!("      -> [SUCESSO! MATCH CONFIRMADO]");
                println!("      -> Encontrado(s) {} mapa(s) reutilizável(is) no SurrealDB:", results.len());
                for (idx, m) in results.iter().enumerate() {
                    println!("\n      MAPA RECUPERADO #{}:", idx + 1);
                    println!("      - Tema: \"{}\"", m.theme);
                    println!("      - Descrição: \"{}\"", m.description);
                    println!("      - Tags Registradas: {:?}", m.tags);
                    println!("      - Dimensões: {}x{} hexágonos", m.width_hexes, m.height_hexes);
                    
                    if let Some(reg) = &m.regions {
                        if let Some(arr) = reg.as_array() {
                            println!("      - Mapeamento Físico de Terrenos: {} regiões registradas (viga, cadeiras, chão limpo).", arr.len());
                        }
                    }
                }
                println!("\n      CONCLUSION: Imagem PNG reutilizada diretamente do cache sem gasto com Gemini!");
            }
        }
        Err(e) => eprintln!("      -> Erro na busca no SurrealDB: {}", e),
    }

    Ok(())
}
