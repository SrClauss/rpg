use serde::{Deserialize, Serialize};

/// Eixo Ético do Alinhamento (Leal, Neutro, Caótico)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ethics {
    Lawful,
    Neutral,
    Chaotic,
}

/// Eixo Moral do Alinhamento (Bom, Neutro, Mau)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Morality {
    Good,
    Neutral,
    Evil,
}

/// Alinhamento D&D 5e (combinação de Ética e Moralidade ou Não-Alinhado)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    TrueNeutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
    Unaligned,
}

impl Alignment {
    pub fn new(ethics: Ethics, morality: Morality) -> Self {
        match (ethics, morality) {
            (Ethics::Lawful, Morality::Good) => Alignment::LawfulGood,
            (Ethics::Neutral, Morality::Good) => Alignment::NeutralGood,
            (Ethics::Chaotic, Morality::Good) => Alignment::ChaoticGood,
            (Ethics::Lawful, Morality::Neutral) => Alignment::LawfulNeutral,
            (Ethics::Neutral, Morality::Neutral) => Alignment::TrueNeutral,
            (Ethics::Chaotic, Morality::Neutral) => Alignment::ChaoticNeutral,
            (Ethics::Lawful, Morality::Evil) => Alignment::LawfulEvil,
            (Ethics::Neutral, Morality::Evil) => Alignment::NeutralEvil,
            (Ethics::Chaotic, Morality::Evil) => Alignment::ChaoticEvil,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Alignment::LawfulGood => "Lawful Good",
            Alignment::NeutralGood => "Neutral Good",
            Alignment::ChaoticGood => "Chaotic Good",
            Alignment::LawfulNeutral => "Lawful Neutral",
            Alignment::TrueNeutral => "Neutral",
            Alignment::ChaoticNeutral => "Chaotic Neutral",
            Alignment::LawfulEvil => "Lawful Evil",
            Alignment::NeutralEvil => "Neutral Evil",
            Alignment::ChaoticEvil => "Chaotic Evil",
            Alignment::Unaligned => "Unaligned",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_creation() {
        let lg = Alignment::new(Ethics::Lawful, Morality::Good);
        assert_eq!(lg, Alignment::LawfulGood);
        assert_eq!(lg.name(), "Lawful Good");

        let cn = Alignment::new(Ethics::Chaotic, Morality::Neutral);
        assert_eq!(cn, Alignment::ChaoticNeutral);
        assert_eq!(cn.name(), "Chaotic Neutral");
    }
}
