use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DurationType {
    Instantaneous,
    UntilEndOfNextTurn,
    DurationRounds(u32),
    DurationMinutes(u32),
    DurationHours(u32),
    Concentration,
    Permanent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActiveEffect {
    /// Bônus fixo na Classe de Armadura (ex: Shield dá +5)
    BonusAC(i32),
    /// Bônus extra rolado em testes de habilidade (ex: Guidance dá "1d4")
    BonusAbilityCheck(String),
    /// Bônus extra rolado em salvaguardas (ex: Bless dá "1d4")
    BonusSavingThrow(String),
    /// Bônus extra rolado em jogadas de ataque (ex: Bless dá "1d4")
    BonusAttackRoll(String),
    /// Resistência a danos específicos (ex: Blade Ward)
    Resistance(Vec<DamageType>),
    /// Impede qualquer tipo de cura (ex: Chill Touch)
    PreventHealing,
    /// Efeito puramente descritivo de utilidade
    Utility(String),
    /// Pontos de vida temporários (ex: Armor of Agathys)
    TemporaryHP(i32),
    /// Causa dano de volta em quem atacar corpo a corpo
    RetaliateMelee(String, DamageType),
    /// Penalidade em ataques (ex: Bane)
    PenaltyAttackRoll(String),
    /// Múltiplos efeitos
    Multiple(Vec<ActiveEffect>),
    /// Bônus de dano em ataques armados (ex: Divine Favor)
    BonusDamage(String, DamageType),
    /// Imunidade a condições específicas (ex: Heroism imuniza contra Medo/Frightened)
    ImmuneToCondition(crate::rules::conditions::Condition),
    /// Vantagem em jogadas de ataque
    AdvantageOnAttacks,
    /// Vantagem em testes de salvaguarda contra tipos de criaturas ou efeitos
    AdvantageOnSavingThrow,
    /// Aplica uma condição específica enquanto durar o efeito
    ApplyCondition(crate::rules::conditions::Condition),
    /// Força os ataques direcionados contra o portador do efeito a terem vantagem
    GrantsAdvantageToAttackers,
    /// Penalidade em testes de salvaguarda (ex: Bane)
    PenaltySavingThrow(String),
    /// Aumenta o HP Máximo e o HP Atual temporariamente (ex: Aid)
    IncreaseMaxHP(i32),
    /// Transforma ataques desarmados em armas naturais com dano e tipo específicos (ex: Alter Self)
    NaturalWeapon(String, DamageType),
    /// Fornece um valor mínimo para a Classe de Armadura (ex: Barkskin)
    MinimumAC(i32),
    /// Força ataques contra o portador a terem desvantagem (ex: Blur)
    DisadvantageOnAttacksAgainstMe,
    /// Dano extra no próximo acerto de ataque com arma (ex: Branding Smite)
    NextWeaponAttackBonusDamage(String, DamageType),
    /// Impede o alvo de receber os benefícios de estar invisível (ex: Branding Smite)
    PreventInvisibility,
    /// Suprime (não cura) uma condição específica (ex: Calm Emotions)
    SuppressCondition(crate::rules::conditions::Condition),
    /// Fornece visão no escuro num raio específico em pés
    GainDarkvision(u32),
    /// Concede Vantagem em testes de habilidade de um atributo específico
    AdvantageOnAbilityCheck(Ability),
    /// Concede Vantagem em testes de salvaguarda de um atributo específico
    AdvantageOnSavingThrowAbility(Ability),
    /// Concede Desvantagem em testes de habilidade de um atributo específico
    DisadvantageOnAbilityCheck(Ability),
    /// Concede Desvantagem em testes de salvaguarda de um atributo específico
    DisadvantageOnSavingThrowAbility(Ability),
    /// Concede um dado extra de dano (contínuo) para todo ataque com arma (ex: Enlarge 1d4)
    WeaponDamageBonus(String),
    /// Subtrai um dado de dano (contínuo) para todo ataque com arma (ex: Reduce 1d4)
    WeaponDamagePenalty(String),
    /// Força uma salvaguarda no final de todo turno do alvo. Se passar, remove uma Condição específica. (ex: Hold Person)
    RecurringSaveAtEndOfTurn(Ability, i32, crate::rules::conditions::Condition),
    /// Fornece imagens espelhadas do alvo (ex: Mirror Image)
    MirrorImage(u8),
    /// Bônus mágico concedido a rolagens de ataque e dano com a arma (ex: Magic Weapon)
    MagicWeaponBonus(u8),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppliedEffect {
    pub effect: ActiveEffect,
    pub duration: DurationType,
    pub source_name: String, // Nome da magia ou habilidade que gerou o efeito
}
