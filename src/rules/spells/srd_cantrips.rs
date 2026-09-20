use crate::rules::character::active_effects::{ActiveEffect, DurationType};
use crate::rules::character::attributes::Ability;
use crate::rules::equipment::weapons::DamageType;
use crate::rules::spells::registry::SpellRegistry;
use crate::rules::spells::spell::{Components, Spell, SpellEffect, SpellSchool};

pub fn load_cantrips(reg: &mut SpellRegistry) {
    reg.register_spell(Spell::new(
        "Acid Splash",
        0,
        SpellSchool::Conjuration,
        "1 action",
        60,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: false,
            damage_dice: Some("1d6".to_string()),
            damage_type: Some(DamageType::Acid),
            damage_scaling: Some("cantrip".to_string()),
            fail_effect: None,
        },
    ));

    reg.register_spell(Spell::new(
        "Blade Ward",
        0,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vs(),
        "1 round",
        false,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Resistance(vec![
                DamageType::Bludgeoning,
                DamageType::Piercing,
                DamageType::Slashing,
            ]),
            duration: DurationType::UntilEndOfNextTurn,
        },
    ));

    reg.register_spell(Spell::new(
        "Chill Touch",
        0,
        SpellSchool::Necromancy,
        "1 action",
        120,
        Components::vs(),
        "1 round",
        false,
        false,
        SpellEffect::Compound(vec![
            SpellEffect::SpellAttack {
                damage_dice: "1d8".to_string(),
                damage_type: DamageType::Necrotic,
                damage_scaling: Some("cantrip".to_string()),
                on_hit_effect: None,
            },
            SpellEffect::ApplyEffect {
                effect: ActiveEffect::PreventHealing,
                duration: DurationType::UntilEndOfNextTurn,
            },
        ]),
    ));

    reg.register_spell(Spell::new(
        "Dancing Lights",
        0,
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vsm("A bit of phosphorus or wychwood, or a glowworm"),
        "1 minute",
        true,
        false,
        SpellEffect::Utility { description: "Você cria até quatro luzes do tamanho de tochas em alcance, fazendo com que pareçam tochas, lanternas ou esferas luminosas que pairam no ar pela duração. Elas iluminam com luz fraca um raio de 10 pés. Como uma ação bônus em seu turno, você pode movê-las até 60 pés para um novo ponto.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Druidcraft",
        0,
        SpellSchool::Transmutation,
        "1 action",
        30,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::Utility { description: "Você cria um efeito mágico natural inofensivo como prever o tempo por 24h, fazer uma flor desabrochar ou acender/apagar uma pequena fogueira/vela.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Eldritch Blast",
        0,
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::SpellAttack {
            damage_dice: "1d10".to_string(),
            damage_type: DamageType::Force,
            damage_scaling: Some("cantrip_beams".to_string()),
            on_hit_effect: None,
        },
    ));

    reg.register_spell(Spell::new(
        "Fire Bolt",
        0,
        SpellSchool::Evocation,
        "1 action",
        120,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::SpellAttack {
            damage_dice: "1d10".to_string(),
            damage_type: DamageType::Fire,
            damage_scaling: Some("cantrip".to_string()),
            on_hit_effect: None,
        },
    ));

    reg.register_spell(Spell::new(
        "Guidance",
        0,
        SpellSchool::Divination,
        "1 action",
        0,
        Components::vs(),
        "1 minute",
        true,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::BonusAbilityCheck("1d4".to_string()),
            duration: DurationType::Concentration,
        },
    ));

    reg.register_spell(Spell::new(
        "Light",
        0,
        SpellSchool::Evocation,
        "1 action",
        0,
        Components::vsm("A firefly or phosphorescent moss"),
        "1 hour",
        false,
        false,
        SpellEffect::Compound(vec![
            SpellEffect::Utility { description: "O objeto tocado emite 20 pés de luz brilhante e mais 20 pés de meia luz. Se direcionado a um objeto vestido por criatura hostil, requer SaveThrow.".to_string() },
            SpellEffect::SaveThrow { save_ability: Ability::Dexterity, half_damage_on_save: false, damage_dice: None, damage_type: None, damage_scaling: None, fail_effect: None }
        ])
    ));

    reg.register_spell(Spell::new(
        "Mage Hand",
        0,
        SpellSchool::Conjuration,
        "1 action",
        30,
        Components::vs(),
        "1 minute",
        false,
        false,
        SpellEffect::Utility { description: "Uma mão espectral flutuante surge no ponto escolhido. Pode manipular objetos, abrir portas destrancadas ou guardar itens (peso máx 10 lbs). Não ataca.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Mending",
        0,
        SpellSchool::Transmutation,
        "1 minute",
        0,
        Components::vsm("Two lodestones"),
        "Instantaneous",
        false,
        false,
        SpellEffect::Utility { description: "Repara um rasgo ou quebra de até 1 pé em qualquer dimensão em um objeto (ex: chave partida, manto rasgado). Não restaura a magia de itens mágicos.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Message",
        0,
        SpellSchool::Transmutation,
        "1 action",
        120,
        Components::vsm("A short piece of copper wire"),
        "1 round",
        false,
        false,
        SpellEffect::Utility { description: "Você aponta seu dedo e sussurra. O alvo ouve a mensagem e pode responder sussurrando em resposta, atravessando superfícies menores que pedras espessas.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Minor Illusion",
        0,
        SpellSchool::Illusion,
        "1 action",
        30,
        Components::vsm("A bit of fleece"),
        "1 minute",
        false,
        false,
        SpellEffect::Compound(vec![
            SpellEffect::Utility {
                description: "Cria um som ou uma imagem de um objeto de até um cubo de 5 pés."
                    .to_string(),
            },
            SpellEffect::SaveThrow {
                save_ability: Ability::Intelligence,
                half_damage_on_save: false,
                damage_dice: None,
                damage_type: None,
                damage_scaling: None,
                fail_effect: None,
            },
        ]),
    ));

    reg.register_spell(Spell::new(
        "Poison Spray",
        0,
        SpellSchool::Conjuration,
        "1 action",
        10,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Constitution,
            half_damage_on_save: false,
            damage_dice: Some("1d12".to_string()),
            damage_type: Some(DamageType::Poison),
            damage_scaling: Some("cantrip".to_string()),
            fail_effect: None,
        },
    ));

    reg.register_spell(Spell::new(
        "Prestidigitation",
        0,
        SpellSchool::Transmutation,
        "1 action",
        10,
        Components::vs(),
        "Up to 1 hour",
        false,
        false,
        SpellEffect::Utility { description: "Pequeno truque inofensivo: esquenta/esfria/tempera comida, cria faísca, limpa ou suja superfície de até 1 pé, ou cria imagem pequena que dura até o fim do turno.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Produce Flame",
        0,
        SpellSchool::Conjuration,
        "1 action",
        30,
        Components::vs(),
        "10 minutes",
        false,
        false,
        SpellEffect::Compound(vec![
            SpellEffect::Utility { description: "Uma chama tremeluzente surge na sua mão e emite 10 pés de luz brilhante e 10 pés penumbra. Não queima você ou seus equipamentos.".to_string() },
            SpellEffect::SpellAttack { damage_dice: "1d8".to_string(), damage_type: DamageType::Fire, damage_scaling: Some("cantrip".to_string()), on_hit_effect: None }
        ])
    ));

    reg.register_spell(Spell::new(
        "Ray of Frost",
        0,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::Compound(vec![
            SpellEffect::SpellAttack {
                damage_dice: "1d8".to_string(),
                damage_type: DamageType::Cold,
                damage_scaling: Some("cantrip".to_string()),
                on_hit_effect: None,
            },
            SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility("Speed reduced by 10 feet".to_string()),
                duration: DurationType::UntilEndOfNextTurn,
            },
        ]),
    ));

    reg.register_spell(Spell::new(
        "Resistance",
        0,
        SpellSchool::Abjuration,
        "1 action",
        0,
        Components::vsm("A miniature cloak"),
        "1 minute",
        true,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::BonusSavingThrow("1d4".to_string()),
            duration: DurationType::Concentration,
        },
    ));

    reg.register_spell(Spell::new(
        "Sacred Flame",
        0,
        SpellSchool::Evocation,
        "1 action",
        60,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::SaveThrow {
            save_ability: Ability::Dexterity,
            half_damage_on_save: false,
            damage_dice: Some("1d8".to_string()),
            damage_type: Some(DamageType::Radiant),
            damage_scaling: Some("cantrip".to_string()),
            fail_effect: None,
        },
    ));

    reg.register_spell(Spell::new(
        "Shillelagh",
        0,
        SpellSchool::Transmutation,
        "1 bonus action",
        0,
        Components::vsm("Mistletoe, a shamrock leaf, and a club or quarterstaff"),
        "1 minute",
        false,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility(
                "Weapon damage becomes 1d8, uses Spellcasting Ability".to_string(),
            ),
            duration: DurationType::DurationMinutes(1),
        },
    ));

    reg.register_spell(Spell::new(
        "Shocking Grasp",
        0,
        SpellSchool::Evocation,
        "1 action",
        0,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::Compound(vec![
            SpellEffect::SpellAttack {
                damage_dice: "1d8".to_string(),
                damage_type: DamageType::Lightning,
                damage_scaling: Some("cantrip".to_string()),
                on_hit_effect: None,
            },
            SpellEffect::ApplyEffect {
                effect: ActiveEffect::Utility(
                    "Target cannot take reactions until start of its next turn".to_string(),
                ),
                duration: DurationType::UntilEndOfNextTurn,
            },
        ]),
    ));

    reg.register_spell(Spell::new(
        "Spare the Dying",
        0,
        SpellSchool::Necromancy,
        "1 action",
        0,
        Components::vs(),
        "Instantaneous",
        false,
        false,
        SpellEffect::Utility { description: "Você toca uma criatura viva que esteja com 0 pontos de vida. Ela se torna estável (não requer mais os salvaguardas contra morte).".to_string() }
    ));

    reg.register_spell(Spell::new(
        "Thaumaturgy",
        0,
        SpellSchool::Transmutation,
        "1 action",
        30,
        Components::vs(),
        "Up to 1 minute",
        false,
        false,
        SpellEffect::Utility { description: "Você manifesta uma maravilha menor: altera o som da sua voz (3x mais alto), causa tremores inofensivos no chão, altera chamas ou abre portas violentamente.".to_string() }
    ));

    reg.register_spell(Spell::new(
        "true Strike",
        0,
        SpellSchool::Divination,
        "1 action",
        30,
        Components::new(false, true, None),
        "1 round",
        true,
        false,
        SpellEffect::ApplyEffect {
            effect: ActiveEffect::Utility(
                "Advantage on your first attack roll against target in the next turn".to_string(),
            ),
            duration: DurationType::Concentration,
        },
    ));

    reg.register_spell(Spell::new(
        "Vicious Mockery",
        0,
        SpellSchool::Enchantment,
        "1 action",
        60,
        Components::v(),
        "Instantaneous",
        false,
        false,
        SpellEffect::Compound(vec![
            SpellEffect::SaveThrow { save_ability: Ability::Wisdom, half_damage_on_save: false, damage_dice: Some("1d4".to_string()), damage_type: Some(DamageType::Psychic), damage_scaling: Some("cantrip".to_string()), fail_effect: None },
            SpellEffect::ApplyEffect { effect: ActiveEffect::Utility("Target has Disadvantage on the next attack roll it makes before end of its next turn".to_string()), duration: DurationType::UntilEndOfNextTurn }
        ])
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dice::DiceMode;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::background::create_acolyte;
    use crate::rules::character::class::{ClassVariant, Fighter, Wizard};
    use crate::rules::character::entity::Entity;
    use crate::rules::character::race::{Human, RaceVariant};
    use crate::rules::combat::attack::DamageAffinity;
    use crate::rules::spells::casting::cast_spell;

    fn create_test_entities() -> (Entity, Entity) {
        let attrs_caster = AbilityScores::new(10, 14, 12, 16, 10, 8); // INT mod +3
        let attrs_target = AbilityScores::new(10, 10, 10, 10, 10, 10);
        let caster = Entity::new(
            "Caster",
            attrs_caster,
            ClassVariant::Wizard(Wizard),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let target = Entity::new(
            "Target",
            attrs_target,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        (caster, target)
    }

    #[test]
    fn test_acid_splash() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Acid Splash").unwrap();
        let initial_hp = target.current_hp;
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
        assert!(target.current_hp <= initial_hp);
    }

    #[test]
    fn test_blade_ward() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Blade Ward").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_chill_touch() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Chill Touch").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_dancing_lights() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Dancing Lights").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_druidcraft() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Druidcraft").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_eldritch_blast() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Eldritch Blast").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_fire_bolt() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Fire Bolt").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_guidance() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Guidance").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_light() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Light").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_mage_hand() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Mage Hand").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_mending() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Mending").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_message() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Message").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_minor_illusion() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Minor Illusion").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_poison_spray() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Poison Spray").unwrap();
        let initial_hp = target.current_hp;
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
        assert!(target.current_hp <= initial_hp);
    }

    #[test]
    fn test_prestidigitation() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Prestidigitation").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_produce_flame() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Produce Flame").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_ray_of_frost() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Ray of Frost").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_resistance() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Resistance").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_sacred_flame() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Sacred Flame").unwrap();
        let initial_hp = target.current_hp;
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
        assert!(target.current_hp <= initial_hp);
    }

    #[test]
    fn test_shillelagh() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Shillelagh").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_shocking_grasp() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Shocking Grasp").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_spare_the_dying() {
        let (mut caster, mut target) = create_test_entities();
        target.current_hp = 0;
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Spare the Dying").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_thaumaturgy() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Thaumaturgy").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }

    #[test]
    fn test_true_strike() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("True Strike").unwrap();
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Wisdom,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
        assert!(!target.applied_effects.is_empty());
    }

    #[test]
    fn test_vicious_mockery() {
        let (mut caster, mut target) = create_test_entities();
        let mut reg = SpellRegistry::new();
        load_cantrips(&mut reg);
        let spell = SpellRegistry::get_spell("Vicious Mockery").unwrap();
        let _initial_hp = target.current_hp;
        let res = cast_spell(
            &mut caster,
            Some(&mut target),
            &spell,
            Ability::Charisma,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );
        assert!(!res.slot_consumed);
    }
}
