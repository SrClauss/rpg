use super::attributes::AbilityScores;
use super::class::ClassVariant;
use super::race::{RaceVariant, Race};
use super::background::Background;
use serde::{Deserialize, Serialize};

/// Representa a Ficha Base de qualquer personagem (Entity) no jogo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub level: u8,
    pub max_hp: i32,
    pub current_hp: i32,
    pub attributes: AbilityScores,
    pub classes: Vec<ClassVariant>,
    pub race: RaceVariant,
    pub background: Background,
}

impl Entity {
    pub fn new(name: &str, mut attributes: AbilityScores, primary_class: ClassVariant, race: RaceVariant, background: Background) -> Self {
        // Aplica os bônus raciais nos atributos base ANTES de calcular a vida
        let asi = race.as_trait().get_ability_score_increases();
        for (ability, bonus) in asi {
            match ability {
                super::attributes::Ability::Strength => attributes.strength.0 = (attributes.strength.0 as i8 + bonus) as u8,
                super::attributes::Ability::Dexterity => attributes.dexterity.0 = (attributes.dexterity.0 as i8 + bonus) as u8,
                super::attributes::Ability::Constitution => attributes.constitution.0 = (attributes.constitution.0 as i8 + bonus) as u8,
                super::attributes::Ability::Intelligence => attributes.intelligence.0 = (attributes.intelligence.0 as i8 + bonus) as u8,
                super::attributes::Ability::Wisdom => attributes.wisdom.0 = (attributes.wisdom.0 as i8 + bonus) as u8,
                super::attributes::Ability::Charisma => attributes.charisma.0 = (attributes.charisma.0 as i8 + bonus) as u8,
            }
        }

        // PV Nível 1: Valor máximo do Dado de Vida da primeira classe + Modificador de Constituição (já modificado pela raça)
        let hit_dice_val = primary_class.as_trait().get_hit_dice() as i32;
        let con_mod = attributes.constitution.modifier() as i32;
        let starting_hp = hit_dice_val + con_mod;

        // Um personagem nunca pode começar com 0 ou menos de vida
        let starting_hp = std::cmp::max(1, starting_hp);

        Self {
            name: name.to_string(),
            level: 1, // Atualmente fixo em 1 por classe, pode evoluir depois para calcular soma
            max_hp: starting_hp,
            current_hp: starting_hp,
            attributes,
            classes: vec![primary_class],
            race,
            background,
        }
    }

    /// Retorna o nível total do personagem (soma de todos os níveis de classe). 
    /// Temporariamente fixado em 1 por item do array.
    pub fn total_level(&self) -> u8 {
        self.classes.len() as u8
    }

    /// Retorna o Bônus de Proficiência baseado no Nível Total
    /// Regra: ceil(Level / 4) + 1
    pub fn proficiency_bonus(&self) -> i32 {
        let lvl = self.total_level() as f32;
        (lvl / 4.0).ceil() as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::class::{Fighter, Wizard};
    use super::super::race::{Elf, Human};
    use super::super::background::create_acolyte;

    #[test]
    fn test_entity_creation_hp_and_race_bonus() {
        // Base CON = 14. Elf doesn't increase CON. Human increases all by +1 (CON becomes 15, modifier still +2).
        let attrs = AbilityScores::new(16, 14, 14, 10, 10, 8); 
        
        let fighter = Entity::new("Bord", attrs.clone(), ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());
        // Human adds +1 to all. CON goes from 14 to 15. Mod is still +2.
        // Fighter HitDice = D10 (10) + 2 = 12
        assert_eq!(fighter.max_hp, 12);
        // Destreza vai de 14 para 15
        assert_eq!(fighter.attributes.dexterity.0, 15);
        
        let wizard = Entity::new("Gandalf", attrs, ClassVariant::Wizard(Wizard), RaceVariant::Elf(Elf), create_acolyte());
        // Wizard HitDice = D6 (6) + 2 = 8
        assert_eq!(wizard.max_hp, 8);
        // Destreza (Elf +2) vai de 14 para 16
        assert_eq!(wizard.attributes.dexterity.0, 16);
    }

    #[test]
    fn test_proficiency_bonus() {
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut hero = Entity::new("Hero", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());
        
        assert_eq!(hero.proficiency_bonus(), 2); // Nível 1

        // Simulando Nível 5 (adicionando 4 classes fake)
        hero.classes.push(ClassVariant::Fighter(Fighter));
        hero.classes.push(ClassVariant::Fighter(Fighter));
        hero.classes.push(ClassVariant::Fighter(Fighter));
        hero.classes.push(ClassVariant::Fighter(Fighter));
        
        assert_eq!(hero.total_level(), 5);
        assert_eq!(hero.proficiency_bonus(), 3); // Nível 5 = +3
    }
}
