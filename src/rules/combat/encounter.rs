use crate::dice::DiceMode;
use crate::rules::character::entity::Entity;
use crate::rules::combat::initiative::InitiativeTracker;
use crate::rules::monsters::monster::Monster;
use serde::{Deserialize, Serialize};

/// Condições de Ambiente e Terreno do Encontro
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentCondition {
    BrightLight,
    DirectSunlight,
    DimLight,
    Darkness,
    DifficultTerrain,
    Custom(String),
}

/// Resumo de Regras Ativas de um Participante do Combate (Formatado para a IA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterRuleSummary {
    pub participant_name: String,
    pub traits: Vec<String>,
    pub equipped_items: Vec<String>,
    pub active_conditions: Vec<String>,
}

/// Gerenciador do Contexto de Regras do Encontro (Active Encounter Context)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterContext {
    pub title: String,
    pub environment: Vec<EnvironmentCondition>,
    pub initiative_tracker: InitiativeTracker,
    pub active_rules: Vec<EncounterRuleSummary>,
    pub is_finished: bool,
}

impl EncounterContext {
    pub fn new(title: &str, environment: Vec<EnvironmentCondition>) -> Self {
        Self {
            title: title.to_string(),
            environment,
            initiative_tracker: InitiativeTracker::new(),
            active_rules: Vec::new(),
            is_finished: false,
        }
    }

    /// Adiciona um Jogador (Entity) ao encontro
    pub fn add_player(&mut self, entity: &Entity, extra_traits: Vec<String>, mode: DiceMode) {
        self.initiative_tracker.add_entity(entity, mode);

        let weapons: Vec<String> = entity
            .equipped_weapons
            .iter()
            .map(|w| w.name.clone())
            .collect();
        let conditions: Vec<String> = entity
            .active_conditions
            .iter()
            .map(|c| format!("{:?}", c))
            .collect();

        self.active_rules.push(EncounterRuleSummary {
            participant_name: entity.name.clone(),
            traits: extra_traits,
            equipped_items: weapons,
            active_conditions: conditions,
        });
    }

    /// Adiciona um Monstro ao encontro extraindo automaticamente seus traços e armas
    pub fn add_monster(&mut self, monster: &Monster, mode: DiceMode) {
        self.initiative_tracker.add_entity(&monster.entity, mode);

        let traits: Vec<String> = monster
            .traits
            .iter()
            .map(|t| format!("{}: {}", t.name, t.description))
            .collect();
        let weapons: Vec<String> = monster
            .actions
            .iter()
            .filter_map(|a| a.weapon.as_ref().map(|w| w.name.clone()))
            .collect();
        let conditions: Vec<String> = monster
            .entity
            .active_conditions
            .iter()
            .map(|c| format!("{:?}", c))
            .collect();

        self.active_rules.push(EncounterRuleSummary {
            participant_name: monster.entity.name.clone(),
            traits,
            equipped_items: weapons,
            active_conditions: conditions,
        });
    }

    /// Inicia a batalha ordenando a fila de iniciativa
    pub fn start_encounter(&mut self) {
        self.initiative_tracker.start_combat();
    }

    /// Compila todo o quadro de regras ativas em uma String pronta para ser injetada no System Prompt da IA
    pub fn compile_prompt_context_for_ai(&self) -> String {
        let mut prompt = format!("### CONTEXTO DE ENCONTRO ATIVO: {}\n", self.title);
        prompt.push_str(&format!(
            "- Rodada Atual: {}\n",
            self.initiative_tracker.round
        ));
        prompt.push_str(&format!("- Ambiente: {:?}\n\n", self.environment));
        prompt.push_str("#### REGRAS E TRAÇOS DOS PARTICIPANTES:\n");

        for rule in &self.active_rules {
            prompt.push_str(&format!("* **{}**:\n", rule.participant_name));
            if !rule.traits.is_empty() {
                prompt.push_str(&format!("  - Traços: {}\n", rule.traits.join(" | ")));
            }
            if !rule.equipped_items.is_empty() {
                prompt.push_str(&format!("  - Itens: {}\n", rule.equipped_items.join(", ")));
            }
            if !rule.active_conditions.is_empty() {
                prompt.push_str(&format!(
                    "  - Condições: {}\n",
                    rule.active_conditions.join(", ")
                ));
            }
        }

        if let Some(current) = self.initiative_tracker.current_combatant() {
            prompt.push_str(&format!("\n👉 TURNO ATUAL: {}\n", current.name));
        }

        prompt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::background::create_acolyte;
    use crate::rules::character::class::{ClassVariant, Fighter};
    use crate::rules::character::race::{Human, RaceVariant};
    use crate::rules::equipment::weapons::create_dagger;
    use crate::rules::monsters::monster::{CreatureSize, CreatureType};

    #[test]
    fn test_encounter_context_compilation() {
        let mut encounter = EncounterContext::new(
            "Emboscada na Floresta",
            vec![
                EnvironmentCondition::DimLight,
                EnvironmentCondition::DifficultTerrain,
            ],
        );

        // Adiciona um jogador
        let attrs = AbilityScores::new(16, 14, 14, 10, 10, 10);
        let hero = Entity::new(
            "Bord",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        encounter.add_player(
            &hero,
            vec!["Ação Surto (1/descanso)".to_string()],
            DiceMode::Normal,
        );

        // Adiciona um monstro (Goblin)
        let base_entity = Entity::new(
            "Goblin",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut goblin = Monster::new(
            "goblin",
            base_entity,
            CreatureType::Humanoid(Some("goblinoid".to_string())),
            CreatureSize::Small,
            0.25,
            50,
        );
        goblin.add_trait("Nimble Escape", "Pode usar Disengage/Hide como Ação Bônus");
        goblin.add_action(crate::rules::monsters::monster::MonsterAction::attack(
            "Scimitar",
            create_dagger(),
        ));

        encounter.add_monster(&goblin, DiceMode::Normal);
        encounter.start_encounter();

        let ai_prompt = encounter.compile_prompt_context_for_ai();

        assert!(ai_prompt.contains("Emboscada na Floresta"));
        assert!(ai_prompt.contains("Bord"));
        assert!(ai_prompt.contains("Goblin"));
        assert!(ai_prompt.contains("Nimble Escape"));
        assert!(ai_prompt.contains("TURNO ATUAL"));
    }
}
