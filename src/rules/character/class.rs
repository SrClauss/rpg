use serde::{Deserialize, Serialize};
use super::attributes::Ability;

/// Dado de Vida da classe (Hit Dice)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HitDice {
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
}

/// Interface genérica que toda classe deve implementar
pub trait CharacterClass {
    fn get_name(&self) -> &str;
    fn get_hit_dice(&self) -> HitDice;
    fn get_primary_abilities(&self) -> Vec<Ability>;
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability>;
    fn get_class_features_summary(&self, level: u8) -> Vec<String>;
}

/// Estrutura Fixa: Bárbaro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Barbarian;

impl CharacterClass for Barbarian {
    fn get_name(&self) -> &str { "Barbarian" }
    fn get_hit_dice(&self) -> HitDice { HitDice::D12 }
    fn get_primary_abilities(&self) -> Vec<Ability> { vec![Ability::Strength] }
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability> { vec![Ability::Strength, Ability::Constitution] }
    fn get_class_features_summary(&self, _level: u8) -> Vec<String> {
        vec![
            "Fúria (Rage)".to_string(),
            "Defesa sem Armadura (10 + Mod. DES + Mod. CON)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Guerreiro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fighter;

impl CharacterClass for Fighter {
    fn get_name(&self) -> &str { "Fighter" }
    fn get_hit_dice(&self) -> HitDice { HitDice::D10 }
    fn get_primary_abilities(&self) -> Vec<Ability> { vec![Ability::Strength, Ability::Dexterity] }
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability> { vec![Ability::Strength, Ability::Constitution] }
    fn get_class_features_summary(&self, _level: u8) -> Vec<String> {
        vec![
            "Estilo de Luta (Fighting Style)".to_string(),
            "Retomada de Fôlego (Second Wind - Cura 1d10 + Nível)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Monge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monk;

impl CharacterClass for Monk {
    fn get_name(&self) -> &str { "Monk" }
    fn get_hit_dice(&self) -> HitDice { HitDice::D8 }
    fn get_primary_abilities(&self) -> Vec<Ability> { vec![Ability::Dexterity, Ability::Wisdom] }
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability> { vec![Ability::Strength, Ability::Dexterity] }
    fn get_class_features_summary(&self, _level: u8) -> Vec<String> {
        vec![
            "Defesa sem Armadura (10 + Mod. DES + Mod. SAB)".to_string(),
            "Artes Marciais (Martial Arts)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Ladino
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rogue;

impl CharacterClass for Rogue {
    fn get_name(&self) -> &str { "Rogue" }
    fn get_hit_dice(&self) -> HitDice { HitDice::D8 }
    fn get_primary_abilities(&self) -> Vec<Ability> { vec![Ability::Dexterity] }
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability> { vec![Ability::Dexterity, Ability::Intelligence] }
    fn get_class_features_summary(&self, _level: u8) -> Vec<String> {
        vec![
            "Especialização (Expertise)".to_string(),
            "Ataque Furtivo (Sneak Attack)".to_string(),
            "Gíria de Ladrão (Thieves' Cant)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Clérigo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cleric;

impl CharacterClass for Cleric {
    fn get_name(&self) -> &str { "Cleric" }
    fn get_hit_dice(&self) -> HitDice { HitDice::D8 }
    fn get_primary_abilities(&self) -> Vec<Ability> { vec![Ability::Wisdom] }
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability> { vec![Ability::Wisdom, Ability::Charisma] }
    fn get_class_features_summary(&self, _level: u8) -> Vec<String> {
        vec![
            "Conjuração Divina (Spellcasting)".to_string(),
            "Domínio Divino (Divine Domain)".to_string(),
            "Canalizar Divindade (Channel Divinity)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Druida
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Druid;

impl CharacterClass for Druid {
    fn get_name(&self) -> &str { "Druid" }
    fn get_hit_dice(&self) -> HitDice { HitDice::D8 }
    fn get_primary_abilities(&self) -> Vec<Ability> { vec![Ability::Wisdom] }
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability> { vec![Ability::Intelligence, Ability::Wisdom] }
    fn get_class_features_summary(&self, _level: u8) -> Vec<String> {
        vec![
            "Druídico (Druidic)".to_string(),
            "Conjuração da Natureza (Spellcasting)".to_string(),
            "Forma Selvagem (Wild Shape)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Feiticeiro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sorcerer;

impl CharacterClass for Sorcerer {
    fn get_name(&self) -> &str { "Sorcerer" }
    fn get_hit_dice(&self) -> HitDice { HitDice::D6 }
    fn get_primary_abilities(&self) -> Vec<Ability> { vec![Ability::Charisma] }
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability> { vec![Ability::Constitution, Ability::Charisma] }
    fn get_class_features_summary(&self, _level: u8) -> Vec<String> {
        vec![
            "Conjuração Inata (Spellcasting)".to_string(),
            "Origem Feiticeira (Sorcerous Origin)".to_string(),
            "Fonte de Magia & Pontos de Feitiçaria (Font of Magic)".to_string(),
            "Metamagia (Metamagic)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Bardo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bard;

impl CharacterClass for Bard {
    fn get_name(&self) -> &str { "Bard" }
    fn get_hit_dice(&self) -> HitDice { HitDice::D8 }
    fn get_primary_abilities(&self) -> Vec<Ability> { vec![Ability::Charisma] }
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability> { vec![Ability::Dexterity, Ability::Charisma] }
    fn get_class_features_summary(&self, _level: u8) -> Vec<String> {
        vec![
            "Conjuração Bárdica (Spellcasting)".to_string(),
            "Inspiração Bárdica (Bardic Inspiration)".to_string(),
            "Pau pra Toda Obra (Jack of All Trades)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Paladino
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paladin;

impl CharacterClass for Paladin {
    fn get_name(&self) -> &str { "Paladin" }
    fn get_hit_dice(&self) -> HitDice { HitDice::D10 }
    fn get_primary_abilities(&self) -> Vec<Ability> { vec![Ability::Strength, Ability::Charisma] }
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability> { vec![Ability::Wisdom, Ability::Charisma] }
    fn get_class_features_summary(&self, _level: u8) -> Vec<String> {
        vec![
            "Sentido Divino (Divine Sense)".to_string(),
            "Imposição de Mãos (Lay on Hands)".to_string(),
            "Destruição Divina (Divine Smite)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Patrulheiro (Ranger)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ranger;

impl CharacterClass for Ranger {
    fn get_name(&self) -> &str { "Ranger" }
    fn get_hit_dice(&self) -> HitDice { HitDice::D10 }
    fn get_primary_abilities(&self) -> Vec<Ability> { vec![Ability::Dexterity, Ability::Wisdom] }
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability> { vec![Ability::Strength, Ability::Dexterity] }
    fn get_class_features_summary(&self, _level: u8) -> Vec<String> {
        vec![
            "Inimigo Favorito (Favored Enemy)".to_string(),
            "Explorador Natural (Natural Explorer)".to_string(),
            "Estilo de Luta (Fighting Style)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Bruxo (Warlock)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warlock;

impl CharacterClass for Warlock {
    fn get_name(&self) -> &str { "Warlock" }
    fn get_hit_dice(&self) -> HitDice { HitDice::D8 }
    fn get_primary_abilities(&self) -> Vec<Ability> { vec![Ability::Charisma] }
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability> { vec![Ability::Wisdom, Ability::Charisma] }
    fn get_class_features_summary(&self, _level: u8) -> Vec<String> {
        vec![
            "Patrono Ultramundano (Otherworldly Patron)".to_string(),
            "Magia de Pacto (Pact Magic)".to_string(),
            "Invocações Místicas (Eldritch Invocations)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Mago
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wizard;

impl CharacterClass for Wizard {
    fn get_name(&self) -> &str { "Wizard" }
    fn get_hit_dice(&self) -> HitDice { HitDice::D6 }
    fn get_primary_abilities(&self) -> Vec<Ability> { vec![Ability::Intelligence] }
    fn get_saving_throw_proficiencies(&self) -> Vec<Ability> { vec![Ability::Intelligence, Ability::Wisdom] }
    fn get_class_features_summary(&self, _level: u8) -> Vec<String> {
        vec![
            "Conjuração Arcana (Spellcasting)".to_string(),
            "Recuperação Arcana (Arcane Recovery)".to_string(),
        ]
    }
}

/// A Variante Única de Classe (Abrange todas as 12 Classes Oficiais do SRD 5.1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassVariant {
    Barbarian(Barbarian),
    Bard(Bard),
    Cleric(Cleric),
    Druid(Druid),
    Fighter(Fighter),
    Monk(Monk),
    Paladin(Paladin),
    Ranger(Ranger),
    Rogue(Rogue),
    Sorcerer(Sorcerer),
    Warlock(Warlock),
    Wizard(Wizard),
}

impl ClassVariant {
    pub fn as_trait(&self) -> &dyn CharacterClass {
        match self {
            ClassVariant::Barbarian(b) => b,
            ClassVariant::Bard(bd) => bd,
            ClassVariant::Cleric(c) => c,
            ClassVariant::Druid(d) => d,
            ClassVariant::Fighter(f) => f,
            ClassVariant::Monk(m) => m,
            ClassVariant::Paladin(p) => p,
            ClassVariant::Ranger(rg) => rg,
            ClassVariant::Rogue(r) => r,
            ClassVariant::Sorcerer(s) => s,
            ClassVariant::Warlock(w) => w,
            ClassVariant::Wizard(wz) => wz,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_12_srd_classes() {
        let classes = vec![
            ClassVariant::Barbarian(Barbarian),
            ClassVariant::Bard(Bard),
            ClassVariant::Cleric(Cleric),
            ClassVariant::Druid(Druid),
            ClassVariant::Fighter(Fighter),
            ClassVariant::Monk(Monk),
            ClassVariant::Paladin(Paladin),
            ClassVariant::Ranger(Ranger),
            ClassVariant::Rogue(Rogue),
            ClassVariant::Sorcerer(Sorcerer),
            ClassVariant::Warlock(Warlock),
            ClassVariant::Wizard(Wizard),
        ];

        assert_eq!(classes.len(), 12);

        // Testa Bárbaro
        assert_eq!(classes[0].as_trait().get_name(), "Barbarian");
        assert_eq!(classes[0].as_trait().get_hit_dice(), HitDice::D12);

        // Testa Bardo
        assert_eq!(classes[1].as_trait().get_name(), "Bard");
        assert_eq!(classes[1].as_trait().get_hit_dice(), HitDice::D8);

        // Testa Paladino
        assert_eq!(classes[6].as_trait().get_name(), "Paladin");
        assert_eq!(classes[6].as_trait().get_hit_dice(), HitDice::D10);

        // Testa Mago
        assert_eq!(classes[11].as_trait().get_name(), "Wizard");
        assert_eq!(classes[11].as_trait().get_hit_dice(), HitDice::D6);
    }
}
