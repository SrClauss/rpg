use serde::{Deserialize, Serialize};

/// As 6 habilidades básicas (Ability Scores) do D&D 5e
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

/// Representa o valor de um Atributo (Ability Score) e fornece métodos para calcular o modificador.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbilityScore(pub u8);

impl AbilityScore {
    pub fn new(score: u8) -> Self {
        Self(score)
    }

    /// Calcula o modificador do atributo segundo a regra do D&D 5e.
    /// Modificador = (Valor - 10) / 2 (arredondado para baixo)
    pub fn modifier(&self) -> i8 {
        let diff = self.0 as i8 - 10;
        if diff < 0 && diff % 2 != 0 {
            diff / 2 - 1
        } else {
            diff / 2
        }
    }
}

/// Um conjunto completo de atributos para um personagem ou criatura.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbilityScores {
    pub strength: AbilityScore,
    pub dexterity: AbilityScore,
    pub constitution: AbilityScore,
    pub intelligence: AbilityScore,
    pub wisdom: AbilityScore,
    pub charisma: AbilityScore,
}

impl AbilityScores {
    pub fn new(str: u8, dex: u8, con: u8, int: u8, wis: u8, cha: u8) -> Self {
        Self {
            strength: AbilityScore::new(str),
            dexterity: AbilityScore::new(dex),
            constitution: AbilityScore::new(con),
            intelligence: AbilityScore::new(int),
            wisdom: AbilityScore::new(wis),
            charisma: AbilityScore::new(cha),
        }
    }

    /// Retorna o valor de um atributo específico.
    pub fn get_score(&self, ability: Ability) -> u8 {
        match ability {
            Ability::Strength => self.strength.0,
            Ability::Dexterity => self.dexterity.0,
            Ability::Constitution => self.constitution.0,
            Ability::Intelligence => self.intelligence.0,
            Ability::Wisdom => self.wisdom.0,
            Ability::Charisma => self.charisma.0,
        }
    }

    /// Retorna o modificador de um atributo específico.
    pub fn get_modifier(&self, ability: Ability) -> i8 {
        match ability {
            Ability::Strength => self.strength.modifier(),
            Ability::Dexterity => self.dexterity.modifier(),
            Ability::Constitution => self.constitution.modifier(),
            Ability::Intelligence => self.intelligence.modifier(),
            Ability::Wisdom => self.wisdom.modifier(),
            Ability::Charisma => self.charisma.modifier(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ability_modifiers() {
        assert_eq!(AbilityScore::new(1).modifier(), -5);
        assert_eq!(AbilityScore::new(8).modifier(), -1);
        assert_eq!(AbilityScore::new(9).modifier(), -1);
        assert_eq!(AbilityScore::new(10).modifier(), 0);
        assert_eq!(AbilityScore::new(11).modifier(), 0);
        assert_eq!(AbilityScore::new(12).modifier(), 1);
        assert_eq!(AbilityScore::new(20).modifier(), 5);
        assert_eq!(AbilityScore::new(30).modifier(), 10);
    }
}
