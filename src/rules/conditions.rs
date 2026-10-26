use serde::{Deserialize, Serialize};

/// Doenças do D&D 5e (SRD 5.1 p. 199-200)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Disease {
    /// Febre do Riso (SRD p. 199) - Afeta humanoides exceto gnomos
    CackleFever,
    /// Peste dos Esgotos (SRD p. 199) - Causa fadiga e impede recuperação de exaustão em descanso longo
    SewerPlague,
    /// Apodrecimento da Visão (SRD p. 200) - Reduz visão a cada descanso longo até cegueira completa (estágio 1-5)
    SightRot { stage: u8 },
    /// Doença customizada com nome e descrição de sintomas
    Custom { name: String, description: String },
}

/// Tipos de Loucura no D&D 5e (SRD 5.1 p. 201-202)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MadnessType {
    /// Curta Duração (1d10 minutos)
    ShortTerm,
    /// Longa Duração (1d10 x 10 horas)
    LongTerm,
    /// Indefinida (Até ser curada por magia como Greater Restoration)
    Indefinite,
}

/// Efeito de Loucura (SRD 5.1 p. 201-202)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Madness {
    pub madness_type: MadnessType,
    pub description: String,
    pub duration_minutes: Option<u32>,
}

/// Condições do D&D 5e (Apêndice A do SRD + SRD p. 199-202)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Condition {
    Blinded,
    Charmed,
    Deafened,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
    Exhaustion(u8), // Nível de Exaustão (1-6)
    Diseased,
    Madness,
    Berserk,
    Surprised,
}

impl Condition {
    /// Retorna se a condição impede a entidade de tomar ações/reações (Incapacitado)
    pub fn is_incapacitating(&self) -> bool {
        matches!(
            self,
            Condition::Incapacitated
                | Condition::Paralyzed
                | Condition::Petrified
                | Condition::Stunned
                | Condition::Unconscious
        )
    }
}
