mod grid;
mod llm;

use dotenvy::dotenv;
use grid::BattleGrid;
use llm::DeepSeekClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("DEEPSEEK_API_KEY")
        .expect("Variável de ambiente DEEPSEEK_API_KEY não configurada no .env");
    let base_url = env::var("DEEPSEEK_BASE_URL").ok();

    let client = DeepSeekClient::new(api_key, base_url);

    let prompt = "O grupo de 2 heróis entra em uma emboscada em uma ponte de madeira em ruínas sobre um desfiladeiro. Dois goblins atacam do outro lado da ponte.";

    println!("Conectando ao DeepSeek LLM para gerar o BattleGrid procedural...");
    println!("Premissa: \"{}\"\n", prompt);

    match client.generate_grid(prompt).await {
        Ok(grid) => {
            println!("BattleGrid gerado com sucesso via Function Calling!");
            println!("{}", grid);

            println!("\nDump do JSON Estruturado:");
            println!("{}", serde_json::to_string_pretty(&grid)?);
        }
        Err(err) => {
            eprintln!("Erro ao gerar o BattleGrid: {}", err);
        }
    }

    Ok(())
}
