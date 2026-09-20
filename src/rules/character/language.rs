use serde::{Deserialize, Serialize};

/// Idiomas oficiais do SRD 5.1
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    // Standard Languages
    Common,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    Orc,

    // Exotic Languages
    Abyssal,
    Celestial,
    Draconic,
    DeepSpeech,
    Infernal,
    Primordial,
    Sylvan,
    Undercommon,

    // Secret / Class Languages
    ThievesCant,
    Druidic,

    // Homebrew / Custom
    Custom(String),
}

impl Language {
    pub fn name(&self) -> &str {
        match self {
            Language::Common => "Common",
            Language::Dwarvish => "Dwarvish",
            Language::Elvish => "Elvish",
            Language::Giant => "Giant",
            Language::Gnomish => "Gnomish",
            Language::Goblin => "Goblin",
            Language::Halfling => "Halfling",
            Language::Orc => "Orc",
            Language::Abyssal => "Abyssal",
            Language::Celestial => "Celestial",
            Language::Draconic => "Draconic",
            Language::DeepSpeech => "Deep Speech",
            Language::Infernal => "Infernal",
            Language::Primordial => "Primordial",
            Language::Sylvan => "Sylvan",
            Language::Undercommon => "Undercommon",
            Language::ThievesCant => "Thieves' Cant",
            Language::Druidic => "Druidic",
            Language::Custom(custom) => custom.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_names() {
        assert_eq!(Language::Common.name(), "Common");
        assert_eq!(Language::Draconic.name(), "Draconic");
        assert_eq!(Language::ThievesCant.name(), "Thieves' Cant");
        assert_eq!(
            Language::Custom("Thundercat".to_string()).name(),
            "Thundercat"
        );
    }
}
