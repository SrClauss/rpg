use serde::{Deserialize, Serialize};

/// Tabela Oficial de Experiência (XP) e Níveis do SRD 5.1 (Página 56)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Advancement;

impl Advancement {
    /// Retorna a quantidade de XP total necessária para atingir um determinado nível (1 a 20)
    pub fn xp_threshold(level: u8) -> u32 {
        match level {
            1 => 0,
            2 => 300,
            3 => 900,
            4 => 2_700,
            5 => 6_500,
            6 => 14_000,
            7 => 23_000,
            8 => 34_000,
            9 => 48_000,
            10 => 64_000,
            11 => 85_000,
            12 => 100_000,
            13 => 120_000,
            14 => 140_000,
            15 => 165_000,
            16 => 195_000,
            17 => 225_000,
            18 => 265_000,
            19 => 305_000,
            20 => 355_000,
            _ if level > 20 => 355_000,
            _ => 0,
        }
    }

    /// Retorna o nível correspondente a um determinado total de XP acumulado
    pub fn level_for_xp(xp: u32) -> u8 {
        for lvl in (1..=20).rev() {
            if xp >= Self::xp_threshold(lvl) {
                return lvl;
            }
        }
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xp_thresholds() {
        assert_eq!(Advancement::xp_threshold(1), 0);
        assert_eq!(Advancement::xp_threshold(2), 300);
        assert_eq!(Advancement::xp_threshold(5), 6500);
        assert_eq!(Advancement::xp_threshold(20), 355000);
    }

    #[test]
    fn test_level_for_xp() {
        assert_eq!(Advancement::level_for_xp(0), 1);
        assert_eq!(Advancement::level_for_xp(299), 1);
        assert_eq!(Advancement::level_for_xp(300), 2);
        assert_eq!(Advancement::level_for_xp(1000), 3);
        assert_eq!(Advancement::level_for_xp(400000), 20);
    }
}
