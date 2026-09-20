use super::advancement::Advancement;
use super::alignment::Alignment;
use super::attributes::AbilityScores;
use super::background::Background;
use super::class::ClassVariant;
use super::language::Language;
use super::multiclass::can_multiclass_into;
use super::race::{Race, RaceVariant, Size};
use super::subclass::{Subclass, SubclassFeature};
use crate::rules::character::active_effects::{ActiveEffect, AppliedEffect};
use crate::rules::conditions::{Condition, Disease, Madness, MadnessType};
use crate::rules::equipment::armor::Armor;
use crate::rules::equipment::magic_items::{MagicItem, MagicItemEffect};
use crate::rules::equipment::weapons::{DamageType, Weapon};
use crate::rules::spells::spellcaster::{ActiveConcentration, SpellSlots};
use serde::{Deserialize, Serialize};

/// Representa a Ficha Base de qualquer personagem (Entity) no jogo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub level: u8,
    pub xp: u32,
    pub base_max_hp: i32,
    pub current_hp: i32,
    pub temp_hp: i32,
    pub attributes: AbilityScores,
    pub classes: Vec<ClassVariant>,
    pub subclass: Option<Subclass>,
    pub race: RaceVariant,
    pub background: Background,
    pub alignment: Alignment,
    pub languages: Vec<Language>,
    pub has_inspiration: bool,
    pub equipped_armor: Option<Armor>,
    pub equipped_shield: Option<Armor>,
    pub equipped_weapons: Vec<Weapon>,
    pub active_conditions: Vec<Condition>,
    pub active_diseases: Vec<Disease>,
    pub active_madness: Vec<Madness>,
    pub spell_slots: Option<SpellSlots>,
    pub active_concentration: Option<ActiveConcentration>,
    pub applied_effects: Vec<AppliedEffect>,
    pub equipped_magic_items: Vec<MagicItem>,
}

impl Entity {
    pub fn new(
        name: &str,
        mut attributes: AbilityScores,
        primary_class: ClassVariant,
        race: RaceVariant,
        background: Background,
    ) -> Self {
        // Aplica os bônus raciais nos atributos base ANTES de calcular a vida
        let asi = race.as_trait().get_ability_score_increases();
        for (ability, bonus) in asi {
            match ability {
                super::attributes::Ability::Strength => {
                    attributes.strength.0 = (attributes.strength.0 as i8 + bonus) as u8
                }
                super::attributes::Ability::Dexterity => {
                    attributes.dexterity.0 = (attributes.dexterity.0 as i8 + bonus) as u8
                }
                super::attributes::Ability::Constitution => {
                    attributes.constitution.0 = (attributes.constitution.0 as i8 + bonus) as u8
                }
                super::attributes::Ability::Intelligence => {
                    attributes.intelligence.0 = (attributes.intelligence.0 as i8 + bonus) as u8
                }
                super::attributes::Ability::Wisdom => {
                    attributes.wisdom.0 = (attributes.wisdom.0 as i8 + bonus) as u8
                }
                super::attributes::Ability::Charisma => {
                    attributes.charisma.0 = (attributes.charisma.0 as i8 + bonus) as u8
                }
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
            level: 1,
            xp: 0,
            base_max_hp: starting_hp,
            current_hp: starting_hp,
            temp_hp: 0,
            attributes,
            classes: vec![primary_class],
            subclass: None,
            race,
            background,
            alignment: Alignment::TrueNeutral,
            languages: vec![Language::Common],
            has_inspiration: false,
            equipped_armor: None,
            equipped_shield: None,
            equipped_weapons: Vec::new(),
            active_conditions: Vec::new(),
            active_diseases: Vec::new(),
            active_madness: Vec::new(),
            spell_slots: None,
            active_concentration: None,
            applied_effects: Vec::new(),
            equipped_magic_items: Vec::new(),
        }
    }

    /// Contrair uma doença (SRD p. 199-200)
    pub fn contract_disease(&mut self, disease: Disease) {
        if !self.active_diseases.contains(&disease) {
            self.active_diseases.push(disease);
            if !self.active_conditions.contains(&Condition::Diseased) {
                self.active_conditions.push(Condition::Diseased);
            }
        }
    }

    /// Curar uma doença específica (ex: Restaurar Léssico / Herbalismo)
    pub fn cure_disease(&mut self, disease: &Disease) {
        self.active_diseases.retain(|d| d != disease);
        if self.active_diseases.is_empty() {
            self.active_conditions.retain(|&c| c != Condition::Diseased);
        }
    }

    /// Curar todas as doenças (ex: Magia Restauração Menor / Maior)
    pub fn cure_all_diseases(&mut self) {
        self.active_diseases.clear();
        self.active_conditions.retain(|&c| c != Condition::Diseased);
    }

    /// Verifica se possui uma doença específica
    pub fn has_disease(&self, disease: &Disease) -> bool {
        self.active_diseases.contains(disease)
    }

    /// Infligir Loucura (SRD p. 201-202)
    pub fn inflict_madness(&mut self, madness: Madness) {
        self.active_madness.push(madness);
        if !self.active_conditions.contains(&Condition::Madness) {
            self.active_conditions.push(Condition::Madness);
        }
    }

    /// Curar loucuras de um tipo específico (ex: Curta/Longa Duração com Restauração Menor, Indefinida com Restauração Maior)
    pub fn cure_madness(&mut self, madness_type: MadnessType) {
        self.active_madness
            .retain(|m| m.madness_type != madness_type);
        if self.active_madness.is_empty() {
            self.active_conditions.retain(|&c| c != Condition::Madness);
        }
    }

    /// Curar todas as loucuras
    pub fn cure_all_madness(&mut self) {
        self.active_madness.clear();
        self.active_conditions.retain(|&c| c != Condition::Madness);
    }

    /// Verifica se possui loucura de determinado tipo
    pub fn has_madness(&self, madness_type: MadnessType) -> bool {
        self.active_madness
            .iter()
            .any(|m| m.madness_type == madness_type)
    }

    /// Calcula o HP Máximo do personagem considerando a base e os efeitos ativos (ex: Aid)
    pub fn get_max_hp(&self) -> i32 {
        let mut total = self.base_max_hp;
        for applied in &self.applied_effects {
            if let ActiveEffect::IncreaseMaxHP(bonus) = applied.effect {
                total += bonus;
            }
        }
        for item in &self.equipped_magic_items {
            for effect in &item.effects {
                if let MagicItemEffect::Passive(ActiveEffect::IncreaseMaxHP(bonus)) = effect {
                    total += *bonus;
                }
            }
        }
        total
    }

    /// Adiciona um efeito ativo
    pub fn add_effect(&mut self, effect: AppliedEffect) {
        self.applied_effects.push(effect);
    }

    /// Adiciona XP ao personagem
    pub fn add_xp(&mut self, amount: u32) {
        self.xp = self.xp.saturating_add(amount);
    }

    /// Avança um nível do personagem (Evolução de Nível / Multiclasse)
    pub fn level_up(&mut self, new_class: ClassVariant, hp_increase: i32) -> Result<(), String> {
        if self.level >= 20 {
            return Err("Personagem já atingiu o nível máximo (Nível 20).".to_string());
        }

        // Se for multiclasse (classe diferente da primeira), valida pré-requisitos
        if let Some(first_class) = self.classes.first() {
            if first_class.as_trait().get_name() != new_class.as_trait().get_name() {
                can_multiclass_into(self, &new_class)?;
            }
        }

        self.level += 1;
        self.classes.push(new_class);

        // Adiciona pontos de vida (Aumento fixo/rolado + mod CON)
        let con_mod = self.attributes.constitution.modifier() as i32;
        let hp_gained = std::cmp::max(1, hp_increase + con_mod);
        self.base_max_hp += hp_gained;
        self.current_hp += hp_gained;

        // Atualiza a meta de XP se o XP atual for menor que o patamar do novo nível
        let req_xp = Advancement::xp_threshold(self.level);
        if self.xp < req_xp {
            self.xp = req_xp;
        }

        Ok(())
    }

    /// Concede Inspiração ao personagem (concedido pelo Mestre)
    pub fn grant_inspiration(&mut self) {
        self.has_inspiration = true;
    }

    /// Consome a Inspiração do personagem para obter Vantagem
    pub fn use_inspiration(&mut self) -> Result<(), String> {
        if !self.has_inspiration {
            return Err("Personagem não possui inspiração para usar.".to_string());
        }
        self.has_inspiration = false;
        Ok(())
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

    pub fn get_creature_type(&self) -> crate::rules::monsters::monster::CreatureType {
        // Personagens jogáveis (Entity baseada em raças) são primariamente Humanóides.
        // Raças específicas podem ser tratadas se necessário (ex: Centaur é Fey).
        let race_name = format!("{:?}", self.race);
        crate::rules::monsters::monster::CreatureType::Humanoid(Some(race_name))
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

        let mut final_ac = base_ac + shield_bonus;
        let mut min_ac = 0;

        for applied in &self.applied_effects {
            match applied.effect {
                ActiveEffect::BonusAC(bonus) => {
                    final_ac += bonus;
                }
                ActiveEffect::MinimumAC(min) if min > min_ac => {
                    min_ac = min;
                }
                _ => {}
            }
        }

        for item in &self.equipped_magic_items {
            for effect in &item.effects {
                if let MagicItemEffect::Passive(ActiveEffect::BonusAC(bonus)) = effect {
                    final_ac += *bonus;
                } else if let MagicItemEffect::Passive(ActiveEffect::MinimumAC(min)) = effect {
                    if *min > min_ac {
                        min_ac = *min;
                    }
                }
            }
        }

        std::cmp::max(final_ac, min_ac)
    }

    /// Equipa um item mágico (simplificado, futuramente pode validar limites de sintonização)
    pub fn equip_magic_item(&mut self, item: MagicItem) -> Result<(), String> {
        self.equipped_magic_items.push(item);
        Ok(())
    }

    /// Adiciona uma condição à entidade caso ela ainda não esteja ativa
    pub fn add_condition(&mut self, condition: Condition) {
        if !self.has_condition(&condition) {
            self.active_conditions.push(condition);
        }
    }

    /// Remove uma condição ativa da entidade e qualquer efeito contínuo que a conceda
    pub fn remove_condition(&mut self, condition: &Condition) {
        self.active_conditions.retain(|c| c != condition);
        // Remove de applied_effects
        self.applied_effects.retain(|e| match &e.effect {
            ActiveEffect::ApplyCondition(c) => c != condition,
            ActiveEffect::Multiple(effs) => !effs
                .iter()
                .any(|sub_e| matches!(sub_e, ActiveEffect::ApplyCondition(c) if c == condition)),
            _ => true,
        });
    }

    /// Verifica se a entidade possui determinada condição
    pub fn has_condition(&self, condition: &Condition) -> bool {
        if self.active_conditions.iter().any(|c| c == condition) {
            return true;
        }
        self.applied_effects.iter().any(|e| match &e.effect {
            ActiveEffect::ApplyCondition(c) => c == condition,
            ActiveEffect::Multiple(effs) => effs
                .iter()
                .any(|sub_e| matches!(sub_e, ActiveEffect::ApplyCondition(c) if c == condition)),
            _ => false,
        })
    }

    /// Processa efeitos que ocorrem no final do turno da Entidade
    pub fn process_end_of_turn(&mut self) {
        let mut effects_to_remove = Vec::new();
        let mut conditions_to_remove = Vec::new();

        // Fazemos a checagem coletando índices para evitar multiple borrows
        for (idx, e) in self.applied_effects.iter().enumerate() {
            if let ActiveEffect::RecurringSaveAtEndOfTurn(ability, dc, cond) = &e.effect {
                // Rola a salvaguarda
                let save = crate::rules::checks::roll_saving_throw(
                    self,
                    *ability,
                    false, // Considerar se tem proficiência seria ideal, por simplicidade usamos false, mas podemos recuperar do target
                    crate::dice::DiceMode::Normal, // Simplificado, ideal seria puxar do estado
                    Some(*dc),
                );

                if save.is_success.unwrap_or(false) {
                    effects_to_remove.push(idx);
                    conditions_to_remove.push(*cond);
                }
            }
        }

        // Remove do maior para o menor índice para não deslocar posições
        effects_to_remove.sort_by(|a, b| b.cmp(a));
        for idx in effects_to_remove {
            self.applied_effects.remove(idx);
        }

        for cond in conditions_to_remove {
            self.remove_condition(&cond);
        }
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
    use super::super::background::create_acolyte;
    use super::super::class::{Fighter, Wizard};
    use super::super::race::{Elf, Human};
    use super::*;

    #[test]
    fn test_entity_creation_hp_and_race_bonus() {
        // Base CON = 14. Elf doesn't increase CON. Human increases all by +1 (CON becomes 15, modifier still +2).
        let attrs = AbilityScores::new(16, 14, 14, 10, 10, 8);

        let fighter = Entity::new(
            "Bord",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        // Human adds +1 to all. CON goes from 14 to 15. Mod is still +2.
        // Fighter HitDice = D10 (10) + 2 = 12
        assert_eq!(fighter.get_max_hp(), 12);
        // Destreza vai de 14 para 15
        assert_eq!(fighter.get_max_hp(), 12);
        assert_eq!(fighter.current_hp, 12);

        let wizard = Entity::new(
            "Gandalf",
            attrs,
            ClassVariant::Wizard(Wizard),
            RaceVariant::Elf(Elf),
            create_acolyte(),
        );
        // Wizard HitDice = D6 (6) + 2 = 8
        assert_eq!(wizard.get_max_hp(), 8);
        assert_eq!(wizard.get_max_hp(), 8);
        // Destreza (Elf +2) vai de 14 para 16
        assert_eq!(wizard.attributes.dexterity.0, 16);
    }

    #[test]
    fn test_proficiency_bonus() {
        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut hero = Entity::new(
            "Hero",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

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
        use crate::rules::equipment::armor::{
            create_chain_shirt, create_leather_armor, create_plate_armor, create_shield,
        };

        // Base DEX 16 (+3 bonus). Human adds +1 -> DEX 17 (+3 bonus).
        let attrs = AbilityScores::new(10, 16, 10, 10, 10, 10);
        let mut hero = Entity::new(
            "Rogue",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

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
        let mut hero = Entity::new(
            "Hero",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

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
        use crate::rules::character::class::Rogue;
        use crate::rules::character::subclass::create_assassin_subclass;

        let attrs = AbilityScores::new(10, 16, 12, 14, 10, 8);
        let mut rogue = Entity::new(
            "Vax",
            attrs,
            ClassVariant::Rogue(Rogue),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

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

    #[test]
    fn test_entity_alignment_language_inspiration() {
        use crate::rules::character::active_effects::{ActiveEffect, AppliedEffect};
        use crate::rules::character::alignment::{Alignment, Ethics, Morality};
        use crate::rules::character::class::Fighter;
        use crate::rules::character::language::Language;

        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut hero = Entity::new(
            "Aragorn",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        // Testar Alinhamento
        hero.alignment = Alignment::new(Ethics::Lawful, Morality::Good);
        assert_eq!(hero.alignment.name(), "Lawful Good");

        // Testar Idiomas
        hero.languages.push(Language::Elvish);
        assert_eq!(hero.languages.len(), 2);
        assert!(hero.languages.contains(&Language::Common));
        assert!(hero.languages.contains(&Language::Elvish));

        // Testar Inspiração
        assert!(!hero.has_inspiration);
        assert!(hero.use_inspiration().is_err());

        hero.grant_inspiration();
        assert!(hero.has_inspiration);
        assert!(hero.use_inspiration().is_ok());
        assert!(!hero.has_inspiration);
    }

    #[test]
    fn test_entity_level_up_and_multiclass() {
        use crate::rules::character::class::{Fighter, Wizard};

        // STR 14, INT 14 -> Atributos adequados para Fighter / Wizard Multiclass
        let attrs = AbilityScores::new(14, 10, 12, 14, 10, 10);
        let mut hero = Entity::new(
            "Gandalftheda",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        assert_eq!(hero.level, 1);
        assert_eq!(hero.total_level(), 1);
        assert_eq!(hero.proficiency_bonus(), 2);
        let initial_hp = hero.get_max_hp();

        // Adiciona XP e sobe de nível na mesma classe (Fighter)
        hero.add_xp(300);
        assert_eq!(hero.xp, 300);
        let res = hero.level_up(ClassVariant::Fighter(Fighter), 6);
        assert!(res.is_ok());

        assert_eq!(hero.level, 2);
        // CON 12 (+1) + 6 HP = +7 HP
        assert_eq!(hero.get_max_hp(), initial_hp + 7);

        // Multiclasse para Wizard (exige INT 13+ e STR 13+ da classe base Fighter)
        let wizard_res = hero.level_up(ClassVariant::Wizard(Wizard), 4);
        assert!(wizard_res.is_ok());

        assert_eq!(hero.level, 3);
        assert_eq!(hero.classes.len(), 3);
        assert_eq!(hero.classes[2].as_trait().get_name(), "Wizard");
    }

    #[test]
    fn test_disease_and_madness_mechanics() {
        use crate::rules::character::class::Fighter;

        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut hero = Entity::new(
            "TestHero",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        // Test Contracting Disease
        assert!(!hero.has_disease(&Disease::CackleFever));
        hero.contract_disease(Disease::CackleFever);
        assert!(hero.has_disease(&Disease::CackleFever));
        assert!(hero.active_conditions.contains(&Condition::Diseased));

        // Test Inflicting Madness
        let madness = Madness {
            madness_type: MadnessType::ShortTerm,
            description: "Vivid hallucinations".to_string(),
            duration_minutes: Some(10),
        };
        hero.inflict_madness(madness);
        assert!(hero.has_madness(MadnessType::ShortTerm));
        assert!(hero.active_conditions.contains(&Condition::Madness));

        // Test Curing Disease
        hero.cure_disease(&Disease::CackleFever);
        assert!(!hero.has_disease(&Disease::CackleFever));
        assert!(!hero.active_conditions.contains(&Condition::Diseased));

        // Test Curing Madness
        hero.cure_madness(MadnessType::ShortTerm);
        assert!(!hero.has_madness(MadnessType::ShortTerm));
        assert!(!hero.active_conditions.contains(&Condition::Madness));
    }

    #[test]
    fn test_entity_equipped_magic_items() {
        use crate::rules::character::active_effects::ActiveEffect;
        use crate::rules::equipment::magic_items::{
            Attunement, MagicItem, MagicItemEffect, MagicItemType, Rarity,
        };

        let attrs = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let mut hero = Entity::new(
            "Hero",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let initial_ac = hero.get_armor_class(); // 10
        let initial_hp = hero.get_max_hp();

        let ring_of_protection = MagicItem::new_with_effects(
            "Ring of Protection",
            MagicItemType::Ring,
            Rarity::Rare,
            Attunement::Required,
            None,
            vec![MagicItemEffect::Passive(ActiveEffect::BonusAC(1))],
            "You gain a +1 bonus to AC.",
        );

        let amulet_of_health = MagicItem::new_with_effects(
            "Amulet of Health",
            MagicItemType::WondrousItem,
            Rarity::Rare,
            Attunement::Required,
            None,
            vec![MagicItemEffect::Passive(ActiveEffect::IncreaseMaxHP(10))],
            "Increases max HP.",
        );

        hero.equip_magic_item(ring_of_protection).unwrap();
        hero.equip_magic_item(amulet_of_health).unwrap();

        assert_eq!(hero.get_armor_class(), initial_ac + 1);
        assert_eq!(hero.get_max_hp(), initial_hp + 10);
    }
}
