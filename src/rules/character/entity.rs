use super::attributes::AbilityScores;
use super::class::ClassVariant;
use super::race::{RaceVariant, Race, Size};
use super::background::Background;
use super::subclass::{Subclass, SubclassFeature};
use crate::rules::equipment::armor::Armor;
use crate::rules::equipment::weapons::{Weapon, DamageType};
use crate::rules::conditions::Condition;
use crate::rules::spells::spellcaster::{SpellSlots, ActiveConcentration};
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
    pub subclass: Option<Subclass>,
    pub race: RaceVariant,
    pub background: Background,
    pub equipped_armor: Option<Armor>,
    pub equipped_shield: Option<Armor>,
    pub equipped_weapons: Vec<Weapon>,
    pub active_conditions: Vec<Condition>,
    pub spell_slots: Option<SpellSlots>,
    pub active_concentration: Option<ActiveConcentration>,
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
            subclass: None,
            race,
            background,
            equipped_armor: None,
            equipped_shield: None,
            equipped_weapons: Vec::new(),
            active_conditions: Vec::new(),
            spell_slots: None,
            active_concentration: None,
        }
    }

    /// Define a subclasse do personagem se o nível mínimo for atingido
    pub fn set_subclass(&mut self, subclass: Subclass) -> Result<(), String> {
        if self.level < subclass.unlock_level {
            return Err(format!(
                "Personagem precisa ser nível {} para escolher a subclasse {}, nível atual é {}",
                subclass.unlock_level, subclass.name, self.level
            ));
        }
        self.subclass = Some(subclass);
        Ok(())
    }

    /// Retorna as habilidades da subclasse ativas para o nível do personagem
    pub fn active_subclass_features(&self) -> Vec<&SubclassFeature> {
        match &self.subclass {
            Some(sc) => sc.active_features(self.level),
            None => Vec::new(),
        }
    }

    /// Retorna o nível total do personagem (soma de todos os níveis de classe ou campo level).
    pub fn total_level(&self) -> u8 {
        std::cmp::max(self.level, self.classes.len() as u8)
    }

    /// Retorna o Bônus de Proficiência baseado no Nível Total
    /// Regra: ceil(Level / 4) + 1
    pub fn proficiency_bonus(&self) -> i32 {
        let lvl = self.total_level() as f32;
        (lvl / 4.0).ceil() as i32 + 1
    }

    /// Calcula dinamicamente a Classe de Armadura (CA) da Entidade.
    /// Sem armadura: 10 + Modificador de Destreza.
    /// Com armadura: CA Base da Armadura + Modificador de Destreza (respeitando limites da armadura).
    /// Bônus de Escudo é somado se equipado.
    pub fn get_armor_class(&self) -> i32 {
        let dex_mod = self.attributes.dexterity.modifier() as i32;

        let base_ac = match &self.equipped_armor {
            Some(armor) => {
                let applicable_dex = if armor.has_dex_limit {
                    std::cmp::min(dex_mod, armor.dex_limit_max)
                } else {
                    dex_mod
                };
                armor.base_ac + applicable_dex
            }
            None => 10 + dex_mod,
        };

        let shield_bonus = match &self.equipped_shield {
            Some(shield) => shield.base_ac,
            None => 0,
        };

        base_ac + shield_bonus
    }

    /// Adiciona uma condição à entidade caso ela ainda não esteja ativa
    pub fn add_condition(&mut self, condition: Condition) {
        if !self.has_condition(&condition) {
            self.active_conditions.push(condition);
        }
    }

    /// Remove uma condição ativa da entidade
    pub fn remove_condition(&mut self, condition: &Condition) {
        self.active_conditions.retain(|c| c != condition);
    }

    /// Verifica se a entidade possui determinada condição
    pub fn has_condition(&self, condition: &Condition) -> bool {
        self.active_conditions.iter().any(|c| c == condition)
    }

    /// Verifica se a entidade está incapacitada por qualquer condição ativa
    pub fn is_incapacitated(&self) -> bool {
        self.active_conditions.iter().any(|c| c.is_incapacitating())
    }

    // --- Métodos de Traços Raciais ---

    pub fn speed(&self) -> u8 {
        self.race.as_trait().get_speed()
    }

    pub fn size(&self) -> Size {
        self.race.as_trait().get_size()
    }

    pub fn has_darkvision(&self) -> bool {
        self.race.as_trait().has_darkvision()
    }

    pub fn has_fey_ancestry(&self) -> bool {
        self.race.as_trait().has_fey_ancestry()
    }

    pub fn has_lucky(&self) -> bool {
        self.race.as_trait().has_lucky()
    }

    pub fn has_gnome_cunning(&self) -> bool {
        self.race.as_trait().has_gnome_cunning()
    }

    pub fn has_relentless_endurance(&self) -> bool {
        self.race.as_trait().has_relentless_endurance()
    }

    pub fn get_damage_resistances(&self) -> Vec<DamageType> {
        self.race.as_trait().get_damage_resistances()
    }

    pub fn get_racial_traits_summary(&self) -> Vec<String> {
        self.race.as_trait().get_traits_summary()
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

    #[test]
    fn test_armor_class_calculation() {
        use crate::rules::equipment::armor::{create_leather_armor, create_chain_shirt, create_plate_armor, create_shield};

        // Base DEX 16 (+3 bonus). Human adds +1 -> DEX 17 (+3 bonus).
        let attrs = AbilityScores::new(10, 16, 10, 10, 10, 10);
        let mut hero = Entity::new("Rogue", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());

        // Unarmored: 10 + 3 = 13
        assert_eq!(hero.get_armor_class(), 13);

        // Light Armor (Leather 11 + 3 DEX = 14)
        hero.equipped_armor = Some(create_leather_armor());
        assert_eq!(hero.get_armor_class(), 14);

        // Medium Armor (Chain Shirt 13 + DEX (max +2) = 15)
        hero.equipped_armor = Some(create_chain_shirt());
        assert_eq!(hero.get_armor_class(), 15);

        // Heavy Armor (Plate 18 + DEX (max 0) = 18)
        hero.equipped_armor = Some(create_plate_armor());
        assert_eq!(hero.get_armor_class(), 18);

        // Plate + Shield (+2) = 20
        hero.equipped_shield = Some(create_shield());
        assert_eq!(hero.get_armor_class(), 20);
    }

    #[test]
    fn test_entity_conditions() {
        use crate::rules::conditions::Condition;

        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut hero = Entity::new("Hero", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());

        assert!(!hero.has_condition(&Condition::Poisoned));
        assert!(!hero.is_incapacitated());

        hero.add_condition(Condition::Poisoned);
        assert!(hero.has_condition(&Condition::Poisoned));
        assert!(!hero.is_incapacitated()); // Envenenado não incapacita por si só

        hero.add_condition(Condition::Stunned);
        assert!(hero.has_condition(&Condition::Stunned));
        assert!(hero.is_incapacitated()); // Atordoado incapacita!

        hero.remove_condition(&Condition::Stunned);
        assert!(!hero.has_condition(&Condition::Stunned));
        assert!(!hero.is_incapacitated());
    }

    #[test]
    fn test_entity_assassin_subclass() {
        use crate::rules::character::subclass::create_assassin_subclass;
        use crate::rules::character::class::Rogue;

        let attrs = AbilityScores::new(10, 16, 12, 14, 10, 8);
        let mut rogue = Entity::new("Vax", attrs, ClassVariant::Rogue(Rogue), RaceVariant::Human(Human), create_acolyte());

        let assassin = create_assassin_subclass();

        // Tentar definir subclasse no nível 1 (deve falhar pois Assassin exige nível 3)
        let res = rogue.set_subclass(assassin.clone());
        assert!(res.is_err());
        assert_eq!(rogue.active_subclass_features().len(), 0);

        // Subir para o Nível 3 e vincular a subclasse Assassin
        rogue.level = 3;
        let res_lvl3 = rogue.set_subclass(assassin);
        assert!(res_lvl3.is_ok());

        // Deve retornar 2 características ativas no nível 3 (Bonus Proficiencies e Assassinate)
        let features = rogue.active_subclass_features();
        assert_eq!(features.len(), 2);
        assert_eq!(features[0].name, "Bonus Proficiencies");
        assert_eq!(features[1].name, "Assassinate");
    }
}

