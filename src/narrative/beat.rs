use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Premissa da campanha — consultada a cada novo Beat para manter
/// o tom, gênero e diretrizes narrativas consistentes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Premise {
    /// Gênero narrativo (ex: "Fantasia sombria de horror cósmico")
    pub genre: String,
    /// Diretrizes de tom e tropos literários
    /// (ex: ["tragédia anunciada", "plot twist traição"])
    pub tone_directives: Vec<String>,
    /// Gancho narrativo de abertura da campanha
    pub initial_hook: String,
    /// Nome do mundo/cenário
    pub world_name: String,
}

/// Tipo do Beat (Pulso Narrativo)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BeatType {
    Exploration,
    Combat,
    Dialogue,
    Rest,
    Cinematic,
    Custom(String),
}

/// Um Beat representa um pulso narrativo discreto.
/// A narrativa é dividida em Beats sequenciais, e ao final de cada um
/// as consequências são aplicadas nas entidades do mundo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beat {
    pub id: Uuid,
    /// Número sequencial do Beat na campanha
    pub index: u32,
    pub title: String,
    /// Resumo narrativo gerado pela IA ao fechar o Beat
    pub summary: String,
    pub beat_type: BeatType,
    /// Mutações aplicadas nas entidades ao fim deste Beat
    pub entity_mutations: Vec<EntityMutation>,
    pub timestamp: String,
}

impl Beat {
    pub fn new(index: u32, title: &str, beat_type: BeatType) -> Self {
        Beat {
            id: Uuid::new_v4(),
            index,
            title: title.to_string(),
            summary: String::new(),
            beat_type,
            entity_mutations: Vec::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Tipo de entidade que sofreu mutação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityMutationType {
    Character,
    Object,
    Location,
    Npc,
}

/// Registro de mutação de uma entidade ao fim de um Beat.
/// Garante rastreabilidade total: qual campo mudou, de que valor para qual,
/// e o motivo narrativo (ex: "Perdeu ente querido no ataque goblin").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMutation {
    pub entity_name: String,
    pub entity_type: EntityMutationType,
    /// ID do Beat em que a mutação ocorreu
    pub beat_id: Uuid,
    /// Campo que foi alterado (ex: "hp", "emotional_state", "location")
    pub field_changed: String,
    pub old_value: String,
    pub new_value: String,
    /// Motivo narrativo da mudança
    pub reason: String,
}

impl EntityMutation {
    pub fn new(
        entity_name: &str,
        entity_type: EntityMutationType,
        beat_id: Uuid,
        field_changed: &str,
        old_value: &str,
        new_value: &str,
        reason: &str,
    ) -> Self {
        EntityMutation {
            entity_name: entity_name.to_string(),
            entity_type,
            beat_id,
            field_changed: field_changed.to_string(),
            old_value: old_value.to_string(),
            new_value: new_value.to_string(),
            reason: reason.to_string(),
        }
    }
}
