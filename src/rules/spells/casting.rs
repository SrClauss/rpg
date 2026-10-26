use crate::dice::{Dice, DiceMode};
use crate::rules::character::attributes::Ability;
use crate::rules::character::entity::Entity;
use crate::rules::checks::{roll_saving_throw, CheckResult};
use crate::rules::combat::attack::{resolve_attack, AttackResult, DamageAffinity};
use crate::rules::equipment::weapons::Weapon;
use crate::rules::spells::spell::{Spell, SpellEffect};
use crate::rules::spells::spellcaster::ActiveConcentration;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Resultado detalhado do resultado de uma conjuração
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CastOutcome {
    AttackRoll(AttackResult),
    SaveRoll {
        dc: i32,
        save_result: CheckResult,
        raw_damage: i32,
        final_damage: i32,
    },
    Healed {
        amount: i32,
    },
    UtilityApplied {
        description: String,
    },
    AreaHazardCreated {
        radius_feet: u32,
        save_dc: i32,
        affected_hexes_count: usize,
    },
    EffectApplied,
    CompoundOutcome(Vec<CastOutcome>),
    SleepApplied {
        total_hp_pool: i32,
    },
    CreatureSummoned {
        monster_id: String,
    },
    Summoned {
        creature_name: String,
        count: u32,
    },
    FailedNoSlot,
    FailedMissingChoice,
    ConditionCured {
        condition: crate::rules::conditions::Condition,
    },
    ConditionNotCured,
}

/// Resultado completo da Conjuração de uma Magia
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastSpellResult {
    pub spell_name: String,
    pub spell_level: u8,
    pub slot_consumed: bool,
    pub spell_save_dc: i32,
    pub spell_attack_bonus: i32,
    pub outcome: CastOutcome,
}

pub fn cast_spell(
    caster: &mut Entity,
    target: Option<&mut Entity>,
    spell: &Spell,
    spellcasting_ability: Ability,
    is_target_save_proficient: bool,
    mode: DiceMode,
    target_affinity: DamageAffinity,
) -> CastSpellResult {
    cast_spell_with_choice(
        caster,
        target,
        spell,
        spellcasting_ability,
        is_target_save_proficient,
        mode,
        target_affinity,
        None,
    )
}

/// Executa a conjuração de uma magia do Conjurador contra um Alvo, permitindo seleção de modo
#[allow(clippy::too_many_arguments)]
pub fn cast_spell_with_choice(
    caster: &mut Entity,
    mut target: Option<&mut Entity>,
    spell: &Spell,
    spellcasting_ability: Ability,
    is_target_save_proficient: bool,
    mode: DiceMode,
    target_affinity: DamageAffinity,
    choice_index: Option<usize>,
) -> CastSpellResult {
    let spellcasting_mod = caster.attributes.get_modifier(spellcasting_ability) as i32;
    let spell_save_dc = 8 + caster.proficiency_bonus() + spellcasting_mod;
    let spell_attack_bonus = caster.proficiency_bonus() + spellcasting_mod;

    // 1. Verificação e Consumo de Espaço de Magia (Spell Slot)
    let slot_consumed = if spell.level > 0 {
        match &mut caster.spell_slots {
            Some(slots) => {
                if !slots.has_slot(spell.level) {
                    return CastSpellResult {
                        spell_name: spell.name.clone(),
                        spell_level: spell.level,
                        slot_consumed: false,
                        spell_save_dc,
                        spell_attack_bonus,
                        outcome: CastOutcome::FailedNoSlot,
                    };
                }
                slots.consume_slot(spell.level)
            }
            None => {
                return CastSpellResult {
                    spell_name: spell.name.clone(),
                    spell_level: spell.level,
                    slot_consumed: false,
                    spell_save_dc,
                    spell_attack_bonus,
                    outcome: CastOutcome::FailedNoSlot,
                }
            }
        }
    } else {
        false // Truques não consomem slots
    };

    // 2. Aplicação de Concentração se exigida pela magia
    if spell.requires_concentration {
        caster.active_concentration = Some(ActiveConcentration {
            spell_name: spell.name.clone(),
            spell_level: spell.level,
        });
    }

    // 3. Resolução dos Efeitos da Magia
    let mut effect_to_eval = &spell.effect;
    if let SpellEffect::Choice(options) = effect_to_eval {
        if let Some(idx) = choice_index {
            if idx < options.len() {
                effect_to_eval = &options[idx].1;
            } else {
                return CastSpellResult {
                    spell_name: spell.name.clone(),
                    spell_level: spell.level,
                    slot_consumed,
                    spell_save_dc,
                    spell_attack_bonus,
                    outcome: CastOutcome::FailedMissingChoice,
                };
            }
        } else {
            return CastSpellResult {
                spell_name: spell.name.clone(),
                spell_level: spell.level,
                slot_consumed,
                spell_save_dc,
                spell_attack_bonus,
                outcome: CastOutcome::FailedMissingChoice,
            };
        }
    }

    // INVISIBILITY HOOK: Casting any spell breaks invisibility
    if caster.has_condition(&crate::rules::conditions::Condition::Invisible) {
        caster.remove_condition(&crate::rules::conditions::Condition::Invisible);
    }

    let outcome = match effect_to_eval {
        SpellEffect::SpellAttack {
            damage_dice,
            damage_type,
            damage_scaling: _,
            on_hit_effect,
        } => {
            let target_entity = target.expect("Ataque Mágico requer um alvo válido");

            // Cria uma arma virtual representando o Ataque Mágico
            let spell_weapon = Weapon::spell_attack(&spell.name, damage_dice, *damage_type);

            // Resolve como rolagem de ataque
            let attack_res = resolve_attack(
                caster,
                target_entity,
                &spell_weapon,
                true,
                mode,
                target_affinity,
            );

            // Aplica o dano se acertar
            if attack_res.is_hit {
                target_entity.current_hp =
                    std::cmp::max(0, target_entity.current_hp - attack_res.final_damage);

                if let Some(effect) = on_hit_effect {
                    let sub_spell = Spell {
                        name: spell.name.clone(),
                        level: spell.level,
                        school: spell.school,
                        casting_time: spell.casting_time.clone(),
                        range_feet: spell.range_feet,
                        components: spell.components.clone(),
                        duration: spell.duration.clone(),
                        requires_concentration: false,
                        is_ritual: spell.is_ritual,
                        effect: *effect.clone(),
                    };
                    // Recursively apply the on hit effect
                    cast_spell(
                        caster,
                        Some(target_entity),
                        &sub_spell,
                        spellcasting_ability,
                        is_target_save_proficient,
                        mode,
                        target_affinity,
                    );
                }
            }

            CastOutcome::AttackRoll(attack_res)
        }
        SpellEffect::SaveThrow {
            save_ability,
            half_damage_on_save,
            damage_dice,
            damage_type: _,
            damage_scaling: _,
            fail_effect,
        } => {
            let target_entity = target.expect("Magia com Salvaguarda requer um alvo válido");
            let save_result = roll_saving_throw(
                target_entity,
                *save_ability,
                is_target_save_proficient,
                mode,
                Some(spell_save_dc),
            );

            let mut raw_damage = 0;
            if let Some(dice_str) = damage_dice {
                if let Ok(dice) = Dice::from_str(dice_str) {
                    raw_damage = dice.roll().total;
                }
            }

            let is_success = save_result.is_success.unwrap_or(false);
            let damage_after_save = if is_success {
                if *half_damage_on_save {
                    raw_damage / 2
                } else {
                    0
                }
            } else {
                raw_damage
            };

            let final_damage = match target_affinity {
                DamageAffinity::Normal => damage_after_save,
                DamageAffinity::Resistant => damage_after_save / 2,
                DamageAffinity::Immune => 0,
                DamageAffinity::Vulnerable => damage_after_save * 2,
            };

            target_entity.current_hp = std::cmp::max(0, target_entity.current_hp - final_damage);

            if !is_success {
                if let Some(effect_box) = fail_effect {
                    if let SpellEffect::ApplyEffect { effect, duration } = &**effect_box {
                        let applied = crate::rules::character::active_effects::AppliedEffect {
                            effect: effect.clone(),
                            duration: duration.clone(),
                            source_name: spell.name.clone(),
                        };
                        target_entity.add_effect(applied);

                        if let crate::rules::character::active_effects::ActiveEffect::Utility(
                            desc,
                        ) = effect
                        {
                            if desc.contains("Prone") {
                                target_entity
                                    .add_condition(crate::rules::conditions::Condition::Prone);
                            }
                            if desc.contains("Incapacitated") {
                                target_entity.add_condition(
                                    crate::rules::conditions::Condition::Incapacitated,
                                );
                            }
                            if desc.contains("Restrained") {
                                target_entity
                                    .add_condition(crate::rules::conditions::Condition::Restrained);
                            }
                        }
                    }
                }
            }

            CastOutcome::SaveRoll {
                dc: spell_save_dc,
                save_result,
                raw_damage,
                final_damage,
            }
        }
        SpellEffect::Healing { healing_dice } => {
            let target_entity = target.expect("Magia de Cura requer um alvo válido");
            let mut heal_amount = 0;
            if let Ok(mut dice) = Dice::from_str(healing_dice) {
                dice.modifier = spellcasting_mod;
                heal_amount = std::cmp::max(1, dice.roll().total);
            }

            target_entity.current_hp = std::cmp::min(
                target_entity.get_max_hp(),
                target_entity.current_hp + heal_amount,
            );

            CastOutcome::Healed {
                amount: heal_amount,
            }
        }
        SpellEffect::MultiTargetHealing {
            max_targets: _,
            healing_dice,
        } => {
            // Em uma engine com grid/times, aplicaríamos nos N alvos aliados.
            // Aqui aplicamos no alvo providenciado.
            let mut heal_amount = 0;
            if let Ok(mut dice) = Dice::from_str(healing_dice) {
                dice.modifier = spellcasting_mod;
                heal_amount = std::cmp::max(1, dice.roll().total);
            }
            if let Some(target_entity) = target {
                target_entity.current_hp = std::cmp::min(
                    target_entity.get_max_hp(),
                    target_entity.current_hp + heal_amount,
                );
            }
            CastOutcome::Healed {
                amount: heal_amount,
            }
        }
        SpellEffect::Utility { description } => CastOutcome::UtilityApplied {
            description: description.clone(),
        },
        SpellEffect::CureCondition(conditions) => {
            let target_entity = target.expect("Cura de Condição requer um alvo válido");
            let mut cured_cond = None;

            for cond in conditions {
                if target_entity.has_condition(cond) {
                    target_entity.remove_condition(cond);
                    cured_cond = Some(*cond);
                    break; // Remove only one
                }
            }

            if let Some(c) = cured_cond {
                CastOutcome::ConditionCured { condition: c }
            } else {
                CastOutcome::ConditionNotCured
            }
        }
        SpellEffect::AreaHazard {
            radius_feet,
            save_ability: _,
            condition: _,
            damage_dice: _,
            damage_type: _,
            description: _,
        } => {
            let hex_radius = (*radius_feet as i32) / 5;
            let hexes_count = (3 * hex_radius * (hex_radius + 1) + 1) as usize;
            CastOutcome::AreaHazardCreated {
                radius_feet: *radius_feet,
                save_dc: spell_save_dc,
                affected_hexes_count: hexes_count,
            }
        }
        SpellEffect::ApplyEffect { effect, duration } => {
            let target_entity = target.expect("Aplicaçao de efeito requer um alvo válido");
            let applied = crate::rules::character::active_effects::AppliedEffect {
                effect: effect.clone(),
                duration: duration.clone(),
                source_name: spell.name.clone(),
            };
            target_entity.add_effect(applied);
            CastOutcome::EffectApplied
        }
        SpellEffect::MultiTargetApplyEffect {
            max_targets: _,
            effect,
            duration,
        } => {
            // Em combate real com grid, selecionaria os N alvos mais próximos
            // Aqui aplicamos no alvo atual, simulando que é um dos alvos
            if let Some(target_entity) = target {
                let applied = crate::rules::character::active_effects::AppliedEffect {
                    effect: effect.clone(),
                    duration: duration.clone(),
                    source_name: spell.name.clone(),
                };
                target_entity.add_effect(applied);
                // No caso do Aid, temos que aumentar o HP Atual pelo bônus recém adicionado
                if let crate::rules::character::active_effects::ActiveEffect::IncreaseMaxHP(bonus) =
                    effect
                {
                    target_entity.current_hp += *bonus;
                }
            }
            CastOutcome::EffectApplied
        }
        SpellEffect::Summon {
            creature_name,
            count,
        } => CastOutcome::Summoned {
            creature_name: creature_name.clone(),
            count: *count,
        },
        SpellEffect::SummonCreature { monster_id } => CastOutcome::Summoned {
            creature_name: monster_id.clone(),
            count: 1,
        },
        SpellEffect::MultiTargetSpellAttack {
            projectiles_count,
            damage_dice,
            damage_type: _,
            auto_hit,
        } => {
            let target_entity = target.expect("MultiTargetSpellAttack requer um alvo válido");
            let mut total_damage = 0;
            if *auto_hit {
                for _ in 0..*projectiles_count {
                    if let Ok(dice) = Dice::from_str(damage_dice) {
                        total_damage += dice.roll().total;
                    }
                }
                target_entity.current_hp =
                    std::cmp::max(0, target_entity.current_hp - total_damage);
            }
            CastOutcome::Healed {
                amount: total_damage,
            } // Reutilizando variante para repasse de dano total
        }
        SpellEffect::Sleep {
            pool_dice,
            radius_feet: _,
        } => {
            let mut total_hp_pool = 0;
            if let Ok(dice) = Dice::from_str(pool_dice) {
                total_hp_pool = dice.roll().total;
            }
            // Em uma implementação de grid real, buscaríamos todos os alvos na área.
            // Aqui, aplicamos diretamente no alvo se os PVs dele forem menores que a pool.
            if let Some(target_entity) = target {
                if target_entity.current_hp <= total_hp_pool {
                    target_entity.add_condition(crate::rules::conditions::Condition::Unconscious);
                }
            }
            CastOutcome::SleepApplied { total_hp_pool }
        }
        SpellEffect::Compound(effects) => {
            let mut outcomes = Vec::new();
            for sub_effect in effects {
                let sub_spell = Spell {
                    name: spell.name.clone(),
                    level: spell.level,
                    school: spell.school,
                    casting_time: spell.casting_time.clone(),
                    range_feet: spell.range_feet,
                    components: spell.components.clone(),
                    duration: spell.duration.clone(),
                    requires_concentration: false,
                    is_ritual: spell.is_ritual,
                    effect: sub_effect.clone(),
                };
                let sub_target = target.as_deref_mut();
                let sub_res = cast_spell(
                    caster,
                    sub_target,
                    &sub_spell,
                    spellcasting_ability,
                    is_target_save_proficient,
                    mode,
                    target_affinity,
                );
                outcomes.push(sub_res.outcome);
            }
            CastOutcome::CompoundOutcome(outcomes)
        }
        SpellEffect::Choice(_) => {
            // Se chegou aqui, não foi processado no pré-filtro ou está num Compound
            CastOutcome::FailedMissingChoice
        }
    };

    CastSpellResult {
        spell_name: spell.name.clone(),
        spell_level: spell.level,
        slot_consumed,
        spell_save_dc,
        spell_attack_bonus,
        outcome,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::background::create_acolyte;
    use crate::rules::character::class::{ClassVariant, Fighter, Wizard};
    use crate::rules::character::race::{Human, RaceVariant};
    use crate::rules::spells::spell::{create_cure_wounds, create_fire_bolt, create_fireball};
    use crate::rules::spells::spellcaster::SpellSlots;

    #[test]
    fn test_cast_cantrip_no_slot_required() {
        let attrs = AbilityScores::new(10, 10, 10, 16, 10, 10); // INT 16 (+3, +1 Human = 17 -> +3)
        let mut mage = Entity::new(
            "Mage",
            attrs.clone(),
            ClassVariant::Wizard(Wizard),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut goblin = Entity::new(
            "Goblin",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let firebolt = create_fire_bolt();
        let res = cast_spell(
            &mut mage,
            Some(&mut goblin),
            &firebolt,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );

        assert!(!res.slot_consumed);
        assert_eq!(res.spell_save_dc, 13); // 8 + 2 (prof) + 3 (mod)
    }

    #[test]
    fn test_cast_level1_spell_consumes_slot() {
        let attrs = AbilityScores::new(10, 10, 10, 16, 10, 10);
        let mut mage = Entity::new(
            "Mage",
            attrs.clone(),
            ClassVariant::Wizard(Wizard),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut ally = Entity::new(
            "Ally",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let mut slots = SpellSlots::new();
        slots.set_slots(1, 2);
        mage.spell_slots = Some(slots);

        ally.current_hp = 2; // Alvo ferido

        let cure = create_cure_wounds();
        let res = cast_spell(
            &mut mage,
            Some(&mut ally),
            &cure,
            Ability::Intelligence,
            false,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );

        assert!(res.slot_consumed);
        assert_eq!(mage.spell_slots.as_ref().unwrap().slots[0].current, 1);
        assert!(ally.current_hp > 2); // Vida restaurada
    }

    #[test]
    fn test_cast_fireball_save_roll() {
        let attrs = AbilityScores::new(10, 14, 10, 16, 10, 10);
        let mut mage = Entity::new(
            "Mage",
            attrs.clone(),
            ClassVariant::Wizard(Wizard),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut target = Entity::new(
            "Target",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let mut slots = SpellSlots::new();
        slots.set_slots(3, 1);
        mage.spell_slots = Some(slots);

        let fireball = create_fireball();
        let res = cast_spell(
            &mut mage,
            Some(&mut target),
            &fireball,
            Ability::Intelligence,
            true,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );

        assert!(res.slot_consumed);
        if let CastOutcome::SaveRoll {
            dc, final_damage, ..
        } = res.outcome
        {
            assert_eq!(dc, 13);
            assert!(final_damage >= 0);
        } else {
            panic!("Esperava resultado do tipo SaveRoll");
        }
    }
}
