use crate::rules::character::attributes::Ability;
use crate::rules::character::language::Language;
use crate::rules::character::race::Size;
use crate::rules::equipment::armor::ArmorCategory;
use crate::rules::equipment::weapons::DamageType;
use crate::rules::monsters::monster::CreatureType;
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
    /// Bônus na Classe de Armadura apenas contra ataques à distância (ex: Arrow-Catching Shield)
    BonusACAgainstRanged(i32),
    /// Bônus extra rolado em testes de habilidade (ex: Guidance dá "1d4")
    BonusAbilityCheck(String),
    /// Bônus extra rolado em salvaguardas (ex: Bless dá "1d4")
    BonusSavingThrow(String),
    /// Bônus extra rolado em jogadas de ataque (ex: Bless dá "1d4")
    BonusAttackRoll(String),
    /// Resistência a danos específicos (ex: Blade Ward)
    Resistance(Vec<DamageType>),
    /// Vulnerabilidade a danos específicos (ex: Armor of Vulnerability)
    Vulnerability(Vec<DamageType>),
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
    /// Transforma qualquer acerto crítico recebido num acerto normal (ex: Adamantine Armor)
    ImmuneToCriticalHits,
    /// Resistência a danos contundentes, cortantes e perfurantes de ataques não-mágicos
    ResistanceToNonMagicalDamage,
    /// Imunidade a danos contundentes, cortantes e perfurantes de ataques não-mágicos
    ImmuneToNonMagicalDamage,
    /// Concede conhecimento de um idioma (ex: Demon Armor)
    LearnLanguage(Language),
    /// Substitui o ataque desarmado padrão por outro dado e tipo de dano (ex: Demon Armor)
    OverrideUnarmedStrike(String, DamageType),
    /// Desvantagem em rolagens de ataque contra criaturas específicas (ex: Demon Armor)
    AdvantageOnAttacksAgainstCreatureType(CreatureType),
    DisadvantageOnAttacksAgainstCreatureType(CreatureType),
    /// Desvantagem em testes de salvaguarda contra criaturas específicas (ex: Demon Armor)
    DisadvantageOnSavingThrowsAgainstCreatureType(CreatureType),
    /// Vantagem contra uma feature/mecânica específica (ex: Dragon Scale Mail)
    AdvantageAgainstFeature(String),
    /// Reduz movimento forçado (ex: Dwarven Plate)
    ReduceForcedMovement(u32),
    /// Concede proficiência passiva em uma categoria de armadura (ex: Elven Chain)
    GrantsArmorProficiency(ArmorCategory),
    /// Ação ativável para alterar aparência do item
    ActionChangeAppearance,
    /// Maldição que redireciona ataques à distância para o alvo se estiverem na distância em pés definida
    CurseRedirectRangedAttacksToMe(u32),
    /// Ataques com magias têm desvantagem contra o portador (ex: Spellguard Shield)
    DisadvantageOnSpellAttacksAgainstMe,
    /// Vantagem em testes de salvaguarda contra magias (ex: Spellguard Shield)
    AdvantageOnSavingThrowsAgainstSpells,
    /// Remove desvantagem em testes de Furtividade impostos por armaduras (ex: Mithral Armor)
    RemoveStealthDisadvantage,
    /// Remove requisitos de Força de armaduras (ex: Mithral Armor)
    RemoveStrengthRequirement,
    /// Aumenta o Max HP por nível (ex: Berserker Axe)
    IncreaseMaxHpPerLevel(u32),
    /// Dano extra contra um tipo específico de criatura (ex: Dragon Slayer, Mace of Disruption)
    ConditionalDamageAgainstCreature(
        crate::rules::monsters::monster::CreatureType,
        String,
        DamageType,
    ),
    /// Sobrescreve o tipo base de dano da arma (ex: Sun Blade converte tudo para Radiante)
    OverrideDamageType(DamageType),
    /// Dano bônus aplicado apenas em Acerto Crítico (ex: Vicious Weapon)
    BonusDamageOnCritical(String, DamageType),

    // Novas Variantes para Consumíveis / Poções
    /// Restaura Pontos de Vida (ex: "2d4+2" para Potion of Healing)
    RestoreHitPoints(String),
    /// Substitui um Atributo Base (ex: Força 21 para Potion of Giant Strength)
    OverrideAttribute(Ability, u32),
    /// Concede Velocidade de Escalada
    GainClimbingSpeed,
    /// Concede Velocidade de Voo
    GainFlyingSpeed,
    /// Altera a Categoria de Tamanho (ex: Enlarge/Reduce)
    ChangeSize(Size),
    /// Invoca uma criatura (ex: Djinni)
    SummonCreature(String),

    // Novas Variantes para Anéis (Rings) e Equipamentos Utilitários
    /// Transforma uma falha no saving throw em sucesso
    SucceedFailedSavingThrow(Ability),
    /// Ignora penalidades de Terreno Difícil
    IgnoreDifficultTerrain,
    /// Imunidade a leitura de mentes, detecção de alinhamento/verdade
    ImmuneToDivination,
    /// Regenera HP ao longo do tempo (ex: "1d6", 10 min)
    RegenerateHP(String, u32),
    /// Capacidade de armazenar N níveis de magia
    SpellStoring(u8),
    /// Proteção contra clima extremo (muito frio ou muito calor)
    EndureExtremeClimate,
    /// Capacidade de andar sobre líquidos
    WaterWalking,
    /// Concede uma ação específica de item (ex: "Shoot ball lightning")
    GrantItemAction(String),
    /// Sofre dano imediato (ex: poção de veneno)
    TakeDamage(String, DamageType),

    // Novas Variantes para Varinhas, Cajados, Cetros e Itens Maravilhosos
    /// Bônus para rolagens de ataque mágico (ex: Wand of the War Mage)
    BonusSpellAttack(String),
    /// Ignora meia cobertura (ex: Wand of the War Mage)
    IgnoreHalfCover,
    /// Vantagem em rolagem de Iniciativa
    AdvantageOnInitiative,
    /// Reflete magias de encantamento se passar na resistência
    ReflectEnchantmentSpells,
    /// Absorve até N níveis de magia lançados contra o usuário
    AbsorbSpellEnergy(u8),
    /// Concede visão no escuro em pés
    GrantDarkvision(u32),
    /// Consegue enxergar seres invisíveis
    SeeInvisible,
    /// Não pode ser surpreendido
    CannotBeSurprised,
    /// Permite interceptar um ataque à distância direcionado a um aliado adjacente (ex: Arrow-Catching Shield)
    InterceptRangedAttack,
    /// Permite rolar os dados novamente N vezes por dia
    RerollDice(u32),
    /// Aplica uma condição automaticamente ao receber dano em combate (ex: Berserker Axe)
    ApplyConditionWhenDamaged(crate::rules::conditions::Condition),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppliedEffect {
    pub effect: ActiveEffect,
    pub duration: DurationType,
    pub source_name: String, // Nome da magia ou habilidade que gerou o efeito
}
