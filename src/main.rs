mod db;
mod gemini;
mod grid;
mod llm;
mod overlay;


use dotenvy::dotenv;
use gemini::GeminiClient;
use grid::TokenType;
use llm::DeepSeekClient;
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

    let deepseek_client = DeepSeekClient::new(deepseek_key, deepseek_url);
    let gemini_client = GeminiClient::new(gemini_key);

    println!("[0/3] Inicializando SurrealDB (Embutido)...");
    let map_db = db::MapDatabase::init().await?;
    println!("      -> Instância do SurrealDB pronta!\n");

    // CENÁRIO: SALÃO DA TAVERNA PÓS-BRIGA
    let tavern_prompt = "Um salão de uma taverna onde acabou de ocorrer uma briga, com as cadeiras quebradas com mais ou menos 20m quadrados, uma viga de madeira caída e uma mesa central no meio do salão.";
    
    println!("\n>>> CENÁRIO: TAVERNA PÓS-BRIGA <<<");
    println!("[1/4] Solicitando prompt visual e tags ao DeepSeek...");
    println!("      Premissa: \"{}\"", tavern_prompt);

    let (mut tavern_grid, tavern_image_prompt, tavern_desc, tavern_tags) = deepseek_client.generate_grid(tavern_prompt).await?;
    println!("      Tema gerado: {}", tavern_grid.theme);
    println!("      Prompt Gemini (Top-Down): \"{}\"", tavern_image_prompt);
    println!("      Tags geradas: {:?}", tavern_tags);

    println!("\n[2/4] Gerando imagem limpa (Gemini) e aplicando overlay com coordenadas (q, r)...");
    match gemini_client.get_or_generate_battle_map_images(
        &tavern_image_prompt,
        &tavern_grid.theme,
        &tavern_desc,
        tavern_tags.clone(),
        tavern_grid.width,
        tavern_grid.height
    ).await {
        Ok(imgs) => {
            println!("      -> Imagem Original (limpa): {}", imgs.original_path);
            println!("      -> Imagem Gradeada (com rótulos q,r): {}", imgs.grid_path);
            println!("      -> Arquivo Metadados: {}", imgs.metadata_path);
            tavern_grid.map_image_url = Some(imgs.grid_path.clone());

            println!("\n[3/4] DeepSeek Visão lendo a imagem gradeada para mapear TODOS os terrenos nos hexágonos (q, r)...");
            if let Ok(grid_bytes) = tokio::fs::read(&imgs.grid_path).await {
                let vision_prompt = "Examine a imagem deste mapa de taverna com a grade de hexágonos rotulados (q, r). \
                Identifique todas as regiões de terrenos (viga caída, cadeiras quebradas, mesa central, chão limpo, entulhos) e mapeie a lista completa de hexágonos {q, r} para cada região.";
                
                match deepseek_client.analyze_map_vision(&grid_bytes, vision_prompt).await {
                    Ok(regions_val) => {
                        println!("      -> DeepSeek Visão mapeou com sucesso as regiões de terreno!");
                        // Grava no arquivo _metadata.json e no SurrealDB
                        let _ = gemini_client.save_map_regions(&imgs.metadata_path, regions_val, Some(&map_db)).await;
                    }
                    Err(e) => eprintln!("      -> Erro no mapeamento de visão: {}", e),
                }
            }

            println!("\n[4/4] Verificando metadados gravados em disco...");
            if let Ok(meta_json) = tokio::fs::read_to_string(&imgs.metadata_path).await {
                println!("--- CONTEÚDO DOS METADADOS GRAVADOS ({}) ---", imgs.metadata_path);
                println!("{}", meta_json);
            }
        }
        Err(e) => eprintln!("      -> Erro na geração da imagem: {}", e),
    }

    Ok(())
}




