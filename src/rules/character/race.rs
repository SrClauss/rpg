use super::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use serde::{Deserialize, Serialize};

/// Tamanho da criatura (Size)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

/// Interface para Raças do D&D 5e
pub trait Race {
    fn get_speed(&self) -> u8;
    fn get_size(&self) -> Size;
    /// Retorna uma lista de Tuplas (Atributo, Bônus)
    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)>;

    // Métodos para Traços Raciais Especiais (Com valores padrão false/vazio)
    fn has_darkvision(&self) -> bool {
        false
    }
    fn has_fey_ancestry(&self) -> bool {
        false
    }
    fn has_lucky(&self) -> bool {
        false
    }
    fn has_gnome_cunning(&self) -> bool {
        false
    }
    fn has_relentless_endurance(&self) -> bool {
        false
    }
    fn get_damage_resistances(&self) -> Vec<DamageType> {
        vec![]
    }
    fn get_traits_summary(&self) -> Vec<String> {
        vec![]
    }
}

/// Estrutura Fixa: Humano
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Human;

impl Race for Human {
    fn get_speed(&self) -> u8 {
        30
    }
    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![
            (Ability::Strength, 1),
            (Ability::Dexterity, 1),
            (Ability::Constitution, 1),
            (Ability::Intelligence, 1),
            (Ability::Wisdom, 1),
            (Ability::Charisma, 1),
        ]
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec!["Humano: +1 em todos os atributos".to_string()]
    }
}

/// Estrutura Fixa: Anão Base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dwarf;

impl Race for Dwarf {
    fn get_speed(&self) -> u8 {
        25
    }
    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![(Ability::Constitution, 2)]
    }

    fn has_darkvision(&self) -> bool {
        true
    }
    fn get_damage_resistances(&self) -> Vec<DamageType> {
        vec![DamageType::Poison]
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec![
            "Resiliência Anã (Vantagem & Resistência a Veneno)".to_string(),
            "Visão no Escuro (60ft)".to_string(),
            "Especialização em Rochas (Stonecunning)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Anão da Colina (Hill Dwarf)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HillDwarf;

impl Race for HillDwarf {
    fn get_speed(&self) -> u8 {
        25
    }
    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![(Ability::Constitution, 2), (Ability::Wisdom, 1)]
    }

    fn has_darkvision(&self) -> bool {
        true
    }
    fn get_damage_resistances(&self) -> Vec<DamageType> {
        vec![DamageType::Poison]
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec![
            "Resiliência Anã (Vantagem & Resistência a Veneno)".to_string(),
            "Visão no Escuro (60ft)".to_string(),
            "Tenacidade Anã (+1 PV por nível)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Elfo Base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Elf;

impl Race for Elf {
    fn get_speed(&self) -> u8 {
        30
    }
    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![(Ability::Dexterity, 2)]
    }

    fn has_darkvision(&self) -> bool {
        true
    }
    fn has_fey_ancestry(&self) -> bool {
        true
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec![
            "Ancestralidade Feérica (Vantagem vs Encantamento, imunidade a Sono Mágico)"
                .to_string(),
            "Visão no Escuro (60ft)".to_string(),
            "Transe (Descanso em 4 horas)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Alto Elfo (High Elf)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighElf;

impl Race for HighElf {
    fn get_speed(&self) -> u8 {
        30
    }
    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![(Ability::Dexterity, 2), (Ability::Intelligence, 1)]
    }

    fn has_darkvision(&self) -> bool {
        true
    }
    fn has_fey_ancestry(&self) -> bool {
        true
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec![
            "Ancestralidade Feérica (Vantagem vs Encantamento, imunidade a Sono Mágico)"
                .to_string(),
            "Visão no Escuro (60ft)".to_string(),
            "Treinamento com Armas Élficas".to_string(),
            "Truque Adicional de Mago".to_string(),
        ]
    }
}

/// Estrutura Fixa: Pequenino Base (Halfling)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Halfling;

impl Race for Halfling {
    fn get_speed(&self) -> u8 {
        25
    }
    fn get_size(&self) -> Size {
        Size::Small
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![(Ability::Dexterity, 2)]
    }

    fn has_lucky(&self) -> bool {
        true
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec![
            "Sortudo (Rerolar 1s naturais em d20)".to_string(),
            "Destemido (Vantagem vs Amedrontado)".to_string(),
            "Agilidade Pequenina".to_string(),
        ]
    }
}

/// Estrutura Fixa: Pequenino Pés-Leves (Lightfoot Halfling)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightfootHalfling;

impl Race for LightfootHalfling {
    fn get_speed(&self) -> u8 {
        25
    }
    fn get_size(&self) -> Size {
        Size::Small
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![(Ability::Dexterity, 2), (Ability::Charisma, 1)]
    }

    fn has_lucky(&self) -> bool {
        true
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec![
            "Sortudo (Rerolar 1s naturais em d20)".to_string(),
            "Destemido (Vantagem vs Amedrontado)".to_string(),
            "Furtividade Natural (Pode se esconder atrás de criaturas maiores)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Gnomo Base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gnome;

impl Race for Gnome {
    fn get_speed(&self) -> u8 {
        25
    }
    fn get_size(&self) -> Size {
        Size::Small
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![(Ability::Intelligence, 2)]
    }

    fn has_darkvision(&self) -> bool {
        true
    }
    fn has_gnome_cunning(&self) -> bool {
        true
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec![
            "Astúcia de Gnomo (Vantagem em Salvaguardas INT/SAB/CAR vs Magia)".to_string(),
            "Visão no Escuro (60ft)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Gnomo das Rochas (Rock Gnome)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RockGnome;

impl Race for RockGnome {
    fn get_speed(&self) -> u8 {
        25
    }
    fn get_size(&self) -> Size {
        Size::Small
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![(Ability::Intelligence, 2), (Ability::Constitution, 1)]
    }

    fn has_darkvision(&self) -> bool {
        true
    }
    fn has_gnome_cunning(&self) -> bool {
        true
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec![
            "Astúcia de Gnomo (Vantagem em Salvaguardas INT/SAB/CAR vs Magia)".to_string(),
            "Visão no Escuro (60ft)".to_string(),
            "Conhecimento de Engenheiro (Artificer's Lore)".to_string(),
            "Engenhoca (Tinker)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Draconato (Dragonborn)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dragonborn;

impl Race for Dragonborn {
    fn get_speed(&self) -> u8 {
        30
    }
    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![(Ability::Strength, 2), (Ability::Charisma, 1)]
    }

    fn get_damage_resistances(&self) -> Vec<DamageType> {
        vec![DamageType::Fire] // Padrão ancestralidade vermelha
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec![
            "Ancestralidade Dracônica (Resistência a Fogo)".to_string(),
            "Arma de Sopro (Breath Weapon)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Meio-Elfo (Half-Elf)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HalfElf;

impl Race for HalfElf {
    fn get_speed(&self) -> u8 {
        30
    }
    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![
            (Ability::Charisma, 2),
            (Ability::Dexterity, 1),
            (Ability::Constitution, 1),
        ]
    }

    fn has_darkvision(&self) -> bool {
        true
    }
    fn has_fey_ancestry(&self) -> bool {
        true
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec![
            "Ancestralidade Feérica (Vantagem vs Encantamento, imunidade a Sono Mágico)"
                .to_string(),
            "Visão no Escuro (60ft)".to_string(),
            "Versatilidade em Perícias (2 perícias à escolha)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Meio-Orc (Half-Orc)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HalfOrc;

impl Race for HalfOrc {
    fn get_speed(&self) -> u8 {
        30
    }
    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![(Ability::Strength, 2), (Ability::Constitution, 1)]
    }

    fn has_darkvision(&self) -> bool {
        true
    }
    fn has_relentless_endurance(&self) -> bool {
        true
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec![
            "Resistência Incansável (Fica com 1 HP ao ser reduzido a 0 HP uma vez/descanso)"
                .to_string(),
            "Ataques Selvagens (Dado extra de dano no acerto crítico)".to_string(),
            "Ameaçador (Proficiência em Intimidação)".to_string(),
            "Visão no Escuro (60ft)".to_string(),
        ]
    }
}

/// Estrutura Fixa: Tiefling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tiefling;

impl Race for Tiefling {
    fn get_speed(&self) -> u8 {
        30
    }
    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn get_ability_score_increases(&self) -> Vec<(Ability, i8)> {
        vec![(Ability::Charisma, 2), (Ability::Intelligence, 1)]
    }

    fn has_darkvision(&self) -> bool {
        true
    }
    fn get_damage_resistances(&self) -> Vec<DamageType> {
        vec![DamageType::Fire]
    }

    fn get_traits_summary(&self) -> Vec<String> {
        vec![
            "Resistência Infernal (Resistência a dano de Fogo)".to_string(),
            "Legado Infernal (Magia Thaumaturgy)".to_string(),
            "Visão no Escuro (60ft)".to_string(),
        ]
    }
}

/// A Variante Única de Raça
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RaceVariant {
    Human(Human),
    Dwarf(Dwarf),
    HillDwarf(HillDwarf),
    Elf(Elf),
    HighElf(HighElf),
    Halfling(Halfling),
    LightfootHalfling(LightfootHalfling),
    Gnome(Gnome),
    RockGnome(RockGnome),
    Dragonborn(Dragonborn),
    HalfElf(HalfElf),
    HalfOrc(HalfOrc),
    Tiefling(Tiefling),
}

impl RaceVariant {
    pub fn as_trait(&self) -> &dyn Race {
        match self {
            RaceVariant::Human(h) => h,
            RaceVariant::Dwarf(d) => d,
            RaceVariant::HillDwarf(hd) => hd,
            RaceVariant::Elf(e) => e,
            RaceVariant::HighElf(he) => he,
            RaceVariant::Halfling(hf) => hf,
            RaceVariant::LightfootHalfling(lh) => lh,
            RaceVariant::Gnome(g) => g,
            RaceVariant::RockGnome(rg) => rg,
            RaceVariant::Dragonborn(db) => db,
            RaceVariant::HalfElf(he) => he,
            RaceVariant::HalfOrc(ho) => ho,
            RaceVariant::Tiefling(t) => t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srd_races_traits() {
        let elf = RaceVariant::HighElf(HighElf);
        assert_eq!(elf.as_trait().get_speed(), 30);
        assert_eq!(elf.as_trait().get_size(), Size::Medium);
        assert!(elf.as_trait().has_darkvision());
        assert!(elf.as_trait().has_fey_ancestry());

        let dwarf = RaceVariant::HillDwarf(HillDwarf);
        assert_eq!(dwarf.as_trait().get_speed(), 25);
        assert!(dwarf.as_trait().has_darkvision());
        assert_eq!(
            dwarf.as_trait().get_damage_resistances(),
            vec![DamageType::Poison]
        );

        let halfling = RaceVariant::LightfootHalfling(LightfootHalfling);
        assert_eq!(halfling.as_trait().get_size(), Size::Small);
        assert!(halfling.as_trait().has_lucky());

        let gnome = RaceVariant::RockGnome(RockGnome);
        assert_eq!(gnome.as_trait().get_size(), Size::Small);
        assert!(gnome.as_trait().has_gnome_cunning());

        let half_orc = RaceVariant::HalfOrc(HalfOrc);
        assert!(half_orc.as_trait().has_relentless_endurance());

        let tiefling = RaceVariant::Tiefling(Tiefling);
        assert_eq!(
            tiefling.as_trait().get_damage_resistances(),
            vec![DamageType::Fire]
        );
    }
}
