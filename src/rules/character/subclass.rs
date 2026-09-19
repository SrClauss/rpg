use serde::{Deserialize, Serialize};

/// Efeitos mecânicos ou descritivos de uma característica de subclasse
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureEffect {
    AdvantageOnUnactedTargets,
    AutoCriticalOnSurprisedTargets,
    ToolProficiency(String),
    Custom(String),
}

/// Característica de uma subclasse concedida em determinado nível
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubclassFeature {
    pub level: u8,
    pub name: String,
    pub description: String,
    pub effects: Vec<FeatureEffect>,
}

/// Subclasse ou Arquétipo (ex: Assassin para Rogue)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subclass {
    pub id: String,
    pub name: String,
    pub parent_class: String,
    pub unlock_level: u8,
    pub description: String,
    pub features: Vec<SubclassFeature>,
}

impl Subclass {
    pub fn new(id: &str, name: &str, parent_class: &str, unlock_level: u8, description: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            parent_class: parent_class.to_string(),
            unlock_level,
            description: description.to_string(),
            features: Vec::new(),
        }
    }

    pub fn with_feature(mut self, feature: SubclassFeature) -> Self {
        self.features.push(feature);
        self
    }

    /// Retorna todas as características ativas para o nível atual do personagem
    pub fn active_features(&self, current_level: u8) -> Vec<&SubclassFeature> {
        self.features
            .iter()
            .filter(|f| f.level <= current_level)
            .collect()
    }
}

/// Construtor auxiliar para a Subclasse Assassin (Archetype de PHB/Rogue)
pub fn create_assassin_subclass() -> Subclass {
    Subclass::new(
        "assassin",
        "Assassin",
        "Rogue",
        3,
        "Focuses on the grim art of death, stealth, poison, and disguise.",
    )
    .with_feature(SubclassFeature {
        level: 3,
        name: "Bonus Proficiencies".to_string(),
        description: "Gains proficiency with the disguise kit and poisoner's kit.".to_string(),
        effects: vec![
            FeatureEffect::ToolProficiency("Disguise Kit".to_string()),
            FeatureEffect::ToolProficiency("Poisoner's Kit".to_string()),
        ],
    })
    .with_feature(SubclassFeature {
        level: 3,
        name: "Assassinate".to_string(),
        description: "Advantage on attack rolls against creatures that haven't taken a turn yet. Any hit against a surprised creature is a critical hit.".to_string(),
        effects: vec![
            FeatureEffect::AdvantageOnUnactedTargets,
            FeatureEffect::AutoCriticalOnSurprisedTargets,
        ],
    })
    .with_feature(SubclassFeature {
        level: 9,
        name: "Infiltration Expertise".to_string(),
        description: "Can create false identities with established background histories.".to_string(),
        effects: vec![FeatureEffect::Custom("Infiltration Expertise".to_string())],
    })
    .with_feature(SubclassFeature {
        level: 13,
        name: "Impostor".to_string(),
        description: "Can unerringly mimic another person's speech, writing, and behavior.".to_string(),
        effects: vec![FeatureEffect::Custom("Impostor".to_string())],
    })
    .with_feature(SubclassFeature {
        level: 17,
        name: "Death Strike".to_string(),
        description: "When hitting a surprised creature, force a CON save or double damage.".to_string(),
        effects: vec![FeatureEffect::Custom("Death Strike".to_string())],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assassin_subclass_features_by_level() {
        let assassin = create_assassin_subclass();
        assert_eq!(assassin.name, "Assassin");
        assert_eq!(assassin.parent_class, "Rogue");
        assert_eq!(assassin.unlock_level, 3);

        // Nível 2 (Ainda não desbloqueou as habilidades da subclasse)
        let lvl2_features = assassin.active_features(2);
        assert_eq!(lvl2_features.len(), 0);

        // Nível 3 (Recebe Bonus Proficiencies + Assassinate)
        let lvl3_features = assassin.active_features(3);
        assert_eq!(lvl3_features.len(), 2);
        assert_eq!(lvl3_features[0].name, "Bonus Proficiencies");
        assert_eq!(lvl3_features[1].name, "Assassinate");

        // Nível 20 (Recebe todas as 5 habilidades)
        let lvl20_features = assassin.active_features(20);
        assert_eq!(lvl20_features.len(), 5);
    }

    #[test]
    fn test_assassin_serde_json() {
        let assassin = create_assassin_subclass();
        let json = serde_json::to_string(&assassin).expect("Deve serializar para JSON");
        assert!(json.contains("Assassinate"));
        assert!(json.contains("Disguise Kit"));

        let deserialized: Subclass = serde_json::from_str(&json).expect("Deve deserializar do JSON");
        assert_eq!(deserialized.id, "assassin");
        assert_eq!(deserialized.features.len(), 5);
    }
}
