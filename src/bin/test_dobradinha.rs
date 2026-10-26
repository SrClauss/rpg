use dotenvy::dotenv;
use rpg_engine::llm::NarrativeOrchestrator;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let gemini_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY não encontrada no .env");
    let deepseek_key = env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY não encontrada no .env");

    let orchestrator = NarrativeOrchestrator::new(gemini_key, deepseek_key);

    let context = "Uma taverna portuária chamada 'O Farol Torto'. Chuva torrencial lá fora. O jogador é um ladino procurado pela guarda que precisa convencer um capitão contrabandista a levá-lo no barco antes da maré cheia.";
    let player_action = "Eu sento na mesa do capitão sem pedir licença, mostro uma moeda antiga de ouro marcada com o brasão da guilda e digo: 'Você não faz perguntas, e este ouro paga a viagem e o seu silêncio.'";

    println!("==================================================");
    println!("TESTE DO MOTOR: DUAL-PASS REFINEMENT PIPELINE");
    println!("==================================================");

    let resultado = orchestrator.generate_refined_narrative(context, player_action).await?;

    println!("\n==================================================");
    println!("RESULTADO FINAL DEFINITIVO:");
    println!("==================================================");
    println!("{}", resultado);

    Ok(())
}
