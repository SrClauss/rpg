use serde_json::{json, Value};

/// Presets e modelos pré-configurados para campanhas temáticas específicas (testes e demonstração)
pub struct CampaignPresets;

impl CampaignPresets {
    pub fn get_subclass_overrides(premise_lower: &str) -> Vec<Value> {
        let mut overrides = Vec::new();
        if premise_lower.contains("mumm-ra") || premise_lower.contains("mumm ra") {
            overrides.push(json!({
                "parent_class": "Warlock",
                "replaces": "O Grande Antigo",
                "name": "O Ser Eterno (Pacto de Mumm-Ra)",
                "description": "Canaliza os Antigos Espíritos do Mal. Permite transformar a carne mortal na Forma Eterna com aumento de CA e dano necrótico.",
                "special_ability": "Invocação dos Antigos Espíritos do Mal"
            }));
        }
        overrides
    }

    pub fn get_custom_spells(premise_lower: &str) -> Vec<Value> {
        let mut spells = Vec::new();
        if premise_lower.contains("plasma") || (premise_lower.contains("magia") && premise_lower.contains("feiticeiro")) {
            spells.push(json!({
                "name": "Rajada de Plasma",
                "level": 1,
                "classes": ["Sorcerer", "Wizard"],
                "school": "Evocação",
                "description": "Dispara um feixe de pura energia iônica instável que atinge o alvo causando 3d8 de dano de fogo e força."
            }));
        }
        spells
    }

    pub fn get_custom_monsters(premise_lower: &str) -> Vec<Value> {
        let mut monsters = Vec::new();
        if premise_lower.contains("espelho") || premise_lower.contains("obsidiana") || premise_lower.contains("doppelg") {
            monsters.push(json!({
                "name": "O Espelho de Obsidiana (Demônio Doppelgänger)",
                "type": "Corruptor Médio (Metamorfo), Neutro e Mau",
                "cr": "3",
                "ac": 14,
                "hp": 52,
                "speed": "9 m",
                "stats": { "str": 11, "dex": 18, "con": 14, "int": 11, "wis": 12, "cha": 16 },
                "abilities": ["A Prisão de Vidro", "Ler Pensamentos", "Metamorfo Profano"],
                "actions": ["Ações Múltiplas (2 ataques)", "Golpe Enganoso (+6, 1d6+4 + 3d6 psíquico)", "Sussurros do Abismo (Coroa da Loucura CD 13)"],
                "description": "Entidade enigmática envolta em sombras e reflexos distorcidos."
            }));
        }
        if premise_lower.contains("snowee") || premise_lower.contains("princesa corrompida") {
            monsters.push(json!({
                "name": "Snowee, a Princesa Corrompida",
                "type": "Humanoide Médio (Corrompido), Caótico e Neutro",
                "cr": "9",
                "ac": 16,
                "hp": 135,
                "speed": "9 m",
                "stats": { "str": 10, "dex": 16, "con": 16, "int": 14, "wis": 12, "cha": 20 },
                "abilities": ["Imunidades: Dano gélido, Enfeitiçado, Amedrontado", "Aura do Coração Congelado (1d6 gélido e 1/2 vel)"],
                "actions": ["Lâmina de Gelo (+7, 2d8+3 + 1d8 gélido)", "Sopro de Estilhaços (Cone 9m, CD 15, 8d8)", "Ações Lendárias (3/turno)"],
                "description": "Figura trágica ligada ao frio sobrenatural que assola as montanhas."
            }));
        }
        if premise_lower.contains("dunga") {
            monsters.push(json!({
                "name": "Dunga, o Algoz Sorridente",
                "type": "Humanoide Pequeno (Goblinoide), Caótico e Mau",
                "cr": "11",
                "ac": 18,
                "hp": 110,
                "speed": "15 m, escalada 15 m",
                "stats": { "str": 12, "dex": 22, "con": 14, "int": 10, "wis": 16, "cha": 8 },
                "abilities": ["Ataque Furtivo (6d6)", "Frenesi Sanguinário", "Evasão", "Esquiva Sobrenatural"],
                "actions": ["Ataques Múltiplos (3 adagas serrilhadas)", "Adaga Serrilhada (+10, 1d4+6 + 2d6 necrótico)"],
                "description": "Caçador implacável das sombras com sorriso gélido e armas envenenadas."
            }));
        }
        monsters
    }
}
