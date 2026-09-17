use serde::{Deserialize, Serialize};

/// Condições do D&D 5e (Apêndice A do SRD)
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
