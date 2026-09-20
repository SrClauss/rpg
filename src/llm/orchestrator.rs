use crate::llm::deepseek::DeepSeekClient;
use crate::llm::gemini::GeminiClient;
use crate::narrative::db::NarrativeDb;
use crate::narrative::telemetry::{EventLog, EntityStats};
use serde_json::Value;
use std::error::Error;

pub struct NarrativeOrchestrator {
    pub gemini: GeminiClient,
    pub deepseek: DeepSeekClient,
    pub db: NarrativeDb,
}

impl NarrativeOrchestrator {
    pub fn new(gemini_api_key: String, deepseek_api_key: String) -> Self {
        Self {
            gemini: GeminiClient::new(gemini_api_key),
            deepseek: DeepSeekClient::new(deepseek_api_key, None),
            db: NarrativeDb::new_in_memory().unwrap(),
        }
    }

    pub async fn process_player_action(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let system_instruction = "Você é o Mestre Narrador de RPG de Mesa (Gemini). Receba a ação do jogador e avance a história de forma imersiva. Se a ação do jogador exigir combate físico, uso de magias de dano ou testes de habilidade estritos (D&D 5e), você DEVE interromper a sua narração exatamente antes de resolver se deu certo ou errado. Em vez de resolver, emita EXATAMENTE a tag #regras seguida por um bloco JSON contendo 'action', 'actor' e 'target', e NADA MAIS. Exemplo: #regras {\"action\": \"attack\", \"actor\": \"Claus\", \"target\": \"Goblin\"}";

        println!("[ORQUESTRADOR] Enviando intenção do jogador para o Gemini...");
        let narrative = self.gemini.generate_narrative(prompt, system_instruction).await?;
        
        if let Some(idx) = narrative.find("#regras") {
            let pure_narrative = &narrative[..idx];
            println!("\n[GEMINI (Narrador)]: {}", pure_narrative.trim());
            
            let json_part = narrative[idx + 7..].trim();
            // Tenta remover crases de markdown (```json ... ```) se o Gemini tiver adicionado
            let clean_json = json_part.trim_matches('`').trim_start_matches("json").trim();

            println!("\n[ORQUESTRADOR] Interceptado Hook de Regras: {}", clean_json);
            
            let hook_payload: Value = serde_json::from_str(clean_json).unwrap_or(serde_json::json!({"action": "unknown", "actor": "unknown"}));
            
            println!("[ORQUESTRADOR] Invocando DeepSeek/Motor Tático para resolver a mecânica...");
            let ds_result = self.deepseek.resolve_mechanics(&hook_payload).await?;
            
            println!("[MOTOR RUST] Gravando resolução mecânica na Telemetria do SQLite...");
            let log = EventLog::new(
                "node_taverna", 
                "mechanic_resolution", 
                hook_payload["actor"].as_str().unwrap_or("Unknown"), 
                hook_payload["target"].as_str(), 
                "Resolvido pelo DeepSeek e Orquestrador", 
                Some(&ds_result.to_string())
            );
            log.insert(&self.db).unwrap();

            println!("[ORQUESTRADOR] Devolvendo resultados do DeepSeek para o Gemini concluir a narração...");
            let follow_up_prompt = format!("A ação mecânica interceptada foi resolvida pelo motor do jogo. O resultado em JSON bruto é: {}. Termine de narrar a cena baseando-se nestes dados matemáticos. Descreva de forma dramática o acerto ou erro, sem citar números cruamente.", ds_result);
            
            let final_narrative = self.gemini.generate_narrative(&follow_up_prompt, "Você é o Mestre Narrador. Leia o JSON mecânico e narre o desfecho.").await?;
            return Ok(format!("{}\n\n[GEMINI (Resolvendo a cena)]: {}", pure_narrative.trim(), final_narrative.trim()));
        }

        // Se não houve a tag #regras, o Gemini apenas contou uma história normal.
        println!("\n[GEMINI (Narrador)]: {}", narrative.trim());
        Ok(narrative)
    }
}
