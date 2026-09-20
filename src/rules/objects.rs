// Constantes de Pontos de Vida (HP) e Classe de Armadura (CA) para Objetos (SRD 5.1 p. 203)

// ==========================================
// PONTOS DE VIDA DE OBJETOS FRÁGEIS (Fragile Object HP)
// ==========================================
/// Objeto Miúdo Frágil (ex: Garrafa, Fechadura) - 1d4 (média: 2)
pub const HP_TINY_FRAGILE: i32 = 2;

/// Objeto Pequeno Frágil (ex: Baú, Alaúde) - 1d6 (média: 3)
pub const HP_SMALL_FRAGILE: i32 = 3;

/// Objeto Médio Frágil (ex: Barril, Lustre) - 1d8 (média: 4)
pub const HP_MEDIUM_FRAGILE: i32 = 4;

/// Objeto Grande Frágil (ex: Carroça, Janela 3x3m) - 1d10 (média: 5)
pub const HP_LARGE_FRAGILE: i32 = 5;

// ==========================================
// PONTOS DE VIDA DE OBJETOS RESILIENTES (Resilient Object HP)
// ==========================================
/// Objeto Miúdo Resiliente (ex: Garrafa, Fechadura) - 2d4 (média: 5)
pub const HP_TINY_RESILIENT: i32 = 5;

/// Objeto Pequeno Resiliente (ex: Baú, Alaúde) - 3d6 (média: 10)
pub const HP_SMALL_RESILIENT: i32 = 10;

/// Objeto Médio Resiliente (ex: Barril, Lustre) - 4d8 (média: 18)
pub const HP_MEDIUM_RESILIENT: i32 = 18;

/// Objeto Grande Resiliente (ex: Carroça, Janela 3x3m) - 5d10 (média: 27)
pub const HP_LARGE_RESILIENT: i32 = 27;

// ==========================================
// CLASSE DE ARMADURA DE OBJETOS POR MATERIAL (Object Armor Class)
// ==========================================
/// Pano, Papel, Corda (Cloth, Paper, Rope)
pub const AC_CLOTH_PAPER_ROPE: u8 = 11;

/// Cristal, Vidro, Gelo (Crystal, Glass, Ice)
pub const AC_CRYSTAL_GLASS_ICE: u8 = 13;

/// Madeira, Osso (Wood, Bone)
pub const AC_WOOD_BONE: u8 = 15;

/// Pedra (Stone)
pub const AC_STONE: u8 = 17;

/// Ferro, Aço (Iron, Steel)
pub const AC_IRON_STEEL: u8 = 19;

/// Mithral
pub const AC_MITHRAL: u8 = 21;

/// Adamante (Adamantine)
pub const AC_ADAMANTINE: u8 = 23;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_constants() {
        assert_eq!(HP_TINY_FRAGILE, 2);
        assert_eq!(HP_SMALL_RESILIENT, 10);
        assert_eq!(AC_IRON_STEEL, 19);
        assert_eq!(AC_ADAMANTINE, 23);
    }
}
