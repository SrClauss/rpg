use dotenvy::dotenv;
use rpg_engine::llm::NarrativeOrchestrator;
use rpg_engine::narrative::beat::{BeatType, EntityMutationType, Premise};
use rpg_engine::state::CampaignState;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let gemini_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY não encontrada no .env");
    let deepseek_key = env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY não encontrada no .env");

    let orchestrator = NarrativeOrchestrator::new(gemini_key, deepseek_key);

    // 1. Criar o CampaignState com a Premissa de Santa Vitória do Sul
    let mut state = CampaignState::new();
    state.premise = Some(Premise {
        genre: "Drama psicológico / Mistério costeiro".to_string(),
        tone_directives: vec![
            "A maresia e o vento sul corroem tudo".to_string(),
            "A vila é cúmplice de mentiras antigas mantidas por Valério".to_string(),
            "Sem melodrama; foco no silêncio e peso moral".to_string(),
        ],
        initial_hook: "Laura herda a casa e a cômoda de 42 gavetas de segredos do tio Valério.".to_string(),
        world_name: "Santa Vitória do Sul".to_string(),
    });

    // 2. Iniciar o Beat 1 (Nível Macro)
    let beat_id = state.start_new_beat(
        "O Confronto na Sacristia",
        BeatType::Dialogue,
    );
    println!(">>> [CAMPAIGN STATE] Beat 1 Iniciado (ID: {})", beat_id);

    // 3. Ação do Jogador (Nível Micro)
    let player_action = "Eu pego a chave da gaveta do Padre Vicente, coloco o laudo balístico da fratura do joelho sobre as hóstias na sacristia e digo: 'O Doutor Valério morreu, Padre. Mas o seu joelho ainda lembra do que aconteceu na noite do incêndio. Vamos conversar sobre o perdão dos seus pecados ou prefere que a vila inteira leia o que está aqui?'";

    println!("\n>>> [JOGADOR] Ação executada:\n{}\n", player_action);

    // 4. Executar o fluxo completo integrado (Gemini -> Rust Rules -> DeepSeek Linter -> Gemini Polished)
    let narracao_final = orchestrator.process_player_action(player_action, &mut state).await?;

    println!("\n==================================================");
    println!("RESPOSTA FINAL DO SISTEMA (PROSA POLIDA):");
    println!("==================================================");
    println!("{}", narracao_final);

    // 5. Fechamento e Mutação do Estado no Rust
    state.record_mutation(
        "Padre Vicente",
        EntityMutationType::Npc,
        "relationship_status",
        "Neutra / Distante",
        "Chantageado / Abalado",
        "Confrontado por Laura com laudo da fratura balística",
    ).unwrap();

    state.close_current_beat("Laura confrontou o Padre Vicente na sacristia, descobrindo que o incêndio do cartório acobertou disputas de terras.").unwrap();

    println!("\n>>> [CAMPAIGN STATE] Histórico de mutação do Padre Vicente:");
    for m in state.get_entity_mutation_history("Padre Vicente") {
        println!("    - Campo: {} | De: '{}' -> Para: '{}' (Motivo: {})", 
            m.field_changed, m.old_value, m.new_value, m.reason);
    }

    Ok(())
}
