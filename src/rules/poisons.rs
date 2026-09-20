use serde::{Deserialize, Serialize};

/// Tipo de Veneno quanto ao meio de aplicação (SRD 5.1 p. 203)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PoisonType {
    /// Contato (Contact) - Untado em objetos; afeta ao tocar com pele exposta.
    Contact,
    /// Ingerido (Ingested) - Engolido junto com comida ou bebida.
    Ingested,
    /// Inalado (Inhaled) - Pó ou gás liberado no ar; afeta ao respirar.
    Inhaled,
    /// Ferimento (Injury) - Aplicado em armas ou munições perfurantes ou cortantes.
    Injury,
}

/// Representa um Veneno no D&D 5e (SRD 5.1 p. 203-205)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Poison {
    pub name: String,
    pub poison_type: PoisonType,
    pub price_gp: u32,
    pub save_dc: u8,
    pub description: String,
}

impl Poison {
    pub fn new(
        name: &str,
        poison_type: PoisonType,
        price_gp: u32,
        save_dc: u8,
        description: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            poison_type,
            price_gp,
            save_dc,
            description: description.to_string(),
        }
    }

    /// Sangue de Assassino (Assassin's Blood) - Ingerido, 150 gp, CD 10 Con
    pub fn assassins_blood() -> Self {
        Self::new(
            "Sangue de Assassino (Assassin's Blood)",
            PoisonType::Ingested,
            150,
            10,
            "Resistência Con CD 10. Falha: 1d12 dano de veneno e envenenado por 24h. Sucesso: metade do dano e não fica envenenado.",
        )
    }

    /// Vapores de Othur Queimados (Burnt Othur Fumes) - Inalado, 500 gp, CD 13 Con
    pub fn burnt_othur_fumes() -> Self {
        Self::new(
            "Vapores de Othur Queimados (Burnt Othur Fumes)",
            PoisonType::Inhaled,
            500,
            13,
            "Resistência Con CD 13 ou toma 3d6 dano de veneno. Repete a cada turno (1d6 dano por falha). Três sucessos encerram o veneno.",
        )
    }

    /// Muco de Rastejante (Crawler Mucus) - Contato, 200 gp, CD 13 Con
    pub fn crawler_mucus() -> Self {
        Self::new(
            "Muco de Rastejante (Crawler Mucus)",
            PoisonType::Contact,
            200,
            13,
            "Resistência Con CD 13 ou fica envenenado e paralisado por 1 minuto. Repete o teste no final de cada turno.",
        )
    }

    /// Veneno Drow (Drow Poison) - Ferimento, 200 gp, CD 13 Con
    pub fn drow_poison() -> Self {
        Self::new(
            "Veneno Drow (Drow Poison)",
            PoisonType::Injury,
            200,
            13,
            "Resistência Con CD 13 ou envenenado por 1 hora. Se falhar por 5 ou mais, fica inconsciente.",
        )
    }

    /// Essência de Éter (Essence of Ether) - Inalado, 300 gp, CD 15 Con
    pub fn essence_of_ether() -> Self {
        Self::new(
            "Essência de Éter (Essence of Ether)",
            PoisonType::Inhaled,
            300,
            15,
            "Resistência Con CD 15 ou fica envenenado e inconsciente por 8 horas. Acorda se tomar dano ou for sacudido.",
        )
    }

    /// Malícia (Malice) - Inalado, 250 gp, CD 15 Con
    pub fn malice() -> Self {
        Self::new(
            "Malícia (Malice)",
            PoisonType::Inhaled,
            250,
            15,
            "Resistência Con CD 15 ou fica envenenado e cego por 1 hora.",
        )
    }

    /// Lágrimas da Meia-Noite (Midnight Tears) - Ingerido, 1500 gp, CD 17 Con
    pub fn midnight_tears() -> Self {
        Self::new(
            "Lágrimas da Meia-Noite (Midnight Tears)",
            PoisonType::Ingested,
            1500,
            17,
            "Sem efeito até a meia-noite. À meia-noite, resistência Con CD 17 ou toma 9d6 dano de veneno (metade no sucesso).",
        )
    }

    /// Óleo de Taggit (Oil of Taggit) - Contato, 400 gp, CD 13 Con
    pub fn oil_of_taggit() -> Self {
        Self::new(
            "Óleo de Taggit (Oil of Taggit)",
            PoisonType::Contact,
            400,
            13,
            "Resistência Con CD 13 ou fica envenenado e inconsciente por 24 horas. Acorda se tomar dano.",
        )
    }

    /// Tintura Pálida (Pale Tincture) - Ingerido, 250 gp, CD 16 Con
    pub fn pale_tincture() -> Self {
        Self::new(
            "Tintura Pálida (Pale Tincture)",
            PoisonType::Ingested,
            250,
            16,
            "Resistência Con CD 16 ou toma 1d6 dano de veneno e fica envenenado. Dano não pode ser curado até 7 sucessos seguidos no teste diário.",
        )
    }

    /// Veneno de Verme Púrpura (Purple Worm Poison) - Ferimento, 2000 gp, CD 19 Con
    pub fn purple_worm_poison() -> Self {
        Self::new(
            "Veneno de Verme Púrpura (Purple Worm Poison)",
            PoisonType::Injury,
            2000,
            19,
            "Resistência Con CD 19. Falha: 12d6 dano de veneno. Sucesso: metade do dano.",
        )
    }

    /// Veneno de Serpente (Serpent Venom) - Ferimento, 200 gp, CD 11 Con
    pub fn serpent_venom() -> Self {
        Self::new(
            "Veneno de Serpente (Serpent Venom)",
            PoisonType::Injury,
            200,
            11,
            "Resistência Con CD 11. Falha: 3d6 dano de veneno. Sucesso: metade do dano.",
        )
    }

    /// Torpor - Ingerido, 600 gp, CD 15 Con
    pub fn torpor() -> Self {
        Self::new(
            "Torpor",
            PoisonType::Ingested,
            600,
            15,
            "Resistência Con CD 15 ou fica envenenado e incapacitado por 4d6 horas.",
        )
    }

    /// Soro da Verdade (Truth Serum) - Ingerido, 150 gp, CD 11 Con
    pub fn truth_serum() -> Self {
        Self::new(
            "Soro da Verdade (Truth Serum)",
            PoisonType::Ingested,
            150,
            11,
            "Resistência Con CD 11 ou fica envenenado por 1 hora. Não consegue mentir intencionalmente (como na magia Zona da Verdade).",
        )
    }

    /// Veneno de Wyvern (Wyvern Poison) - Ferimento, 1200 gp, CD 15 Con
    pub fn wyvern_poison() -> Self {
        Self::new(
            "Veneno de Wyvern (Wyvern Poison)",
            PoisonType::Injury,
            1200,
            15,
            "Resistência Con CD 15. Falha: 7d6 dano de veneno. Sucesso: metade do dano.",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_poisons() {
        let drow = Poison::drow_poison();
        assert_eq!(drow.poison_type, PoisonType::Injury);
        assert_eq!(drow.save_dc, 13);
        assert_eq!(drow.price_gp, 200);

        let purple_worm = Poison::purple_worm_poison();
        assert_eq!(purple_worm.save_dc, 19);
        assert_eq!(purple_worm.price_gp, 2000);
    }
}
