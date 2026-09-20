use serde::{Deserialize, Serialize};

/// Níveis de Cobertura em Combate (Cover - SRD 5.1 p. 96)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cover {
    None,
    Half,
    ThreeQuarters,
    Total,
}

impl Cover {
    /// Bônus para Classe de Armadura (AC) concedido pela Cobertura
    pub fn ac_bonus(&self) -> i32 {
        match self {
            Cover::None => 0,
            Cover::Half => 2,
            Cover::ThreeQuarters => 5,
            Cover::Total => 0, // Cobertura total previne ser alvo direto
        }
    }

    /// Bônus em Salvaguardas de Destreza (Dexterity Saving Throws)
    pub fn dex_save_bonus(&self) -> i32 {
        match self {
            Cover::None => 0,
            Cover::Half => 2,
            Cover::ThreeQuarters => 5,
            Cover::Total => 0,
        }
    }

    /// Se a cobertura total impede que o alvo seja escolhido para ataque direto ou magia de alvo
    pub fn blocks_direct_target(&self) -> bool {
        matches!(self, Cover::Total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cover_bonuses() {
        assert_eq!(Cover::Half.ac_bonus(), 2);
        assert_eq!(Cover::ThreeQuarters.ac_bonus(), 5);
        assert_eq!(Cover::Half.dex_save_bonus(), 2);
        assert!(Cover::Total.blocks_direct_target());
    }
}
