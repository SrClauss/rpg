use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::dice::{Dice, DiceMode};
use crate::rules::character::attributes::Ability;
use crate::rules::character::entity::Entity;
use crate::rules::checks::{roll_saving_throw, CheckResult};
use crate::rules::combat::attack::{resolve_attack, AttackResult, DamageAffinity};
use crate::rules::equipment::weapons::{DamageType, Weapon};
use crate::rules::spells::spell::{Spell, SpellEffect};
use crate::rules::spells::spellcaster::ActiveConcentration;

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
    FailedNoSlot,
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

/// Executa a conjuração de uma magia do Conjurador contra um Alvo
pub fn cast_spell(
    caster: &mut Entity,
    target: Option<&mut Entity>,
    spell: &Spell,
    spellcasting_ability: Ability,
    is_target_save_proficient: bool,
    mode: DiceMode,
    target_affinity: DamageAffinity,
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
    let outcome = match &spell.effect {
        SpellEffect::SpellAttack { damage_dice, damage_type } => {
            let target_entity = target.expect("Ataque Mágico requer um alvo válido");
            
            // Cria uma arma virtual representando o Ataque Mágico
            let spell_weapon = Weapon::new(&spell.name, damage_dice, *damage_type, vec![]);
            
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
                target_entity.current_hp = std::cmp::max(0, target_entity.current_hp - attack_res.final_damage);
            }
            
            CastOutcome::AttackRoll(attack_res)
        }
        SpellEffect::SaveThrow {
            save_ability,
            half_damage_on_save,
            damage_dice,
            damage_type: _,
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

            target_entity.current_hp = std::cmp::min(target_entity.max_hp, target_entity.current_hp + heal_amount);

            CastOutcome::Healed { amount: heal_amount }
        }
        SpellEffect::Utility { description } => CastOutcome::UtilityApplied {
            description: description.clone(),
        },
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
    use crate::rules::character::class::{Wizard, Fighter, ClassVariant};
    use crate::rules::character::race::{Human, RaceVariant};
    use crate::rules::character::background::create_acolyte;
    use crate::rules::spells::spell::{create_fire_bolt, create_cure_wounds, create_fireball};
    use crate::rules::spells::spellcaster::SpellSlots;

    #[test]
    fn test_cast_cantrip_no_slot_required() {
        let attrs = AbilityScores::new(10, 10, 10, 16, 10, 10); // INT 16 (+3, +1 Human = 17 -> +3)
        let mut mage = Entity::new("Mage", attrs.clone(), ClassVariant::Wizard(Wizard), RaceVariant::Human(Human), create_acolyte());
        let mut goblin = Entity::new("Goblin", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());

        let firebolt = create_fire_bolt();
        let res = cast_spell(&mut mage, Some(&mut goblin), &firebolt, Ability::Intelligence, false, DiceMode::Normal, DamageAffinity::Normal);

        assert!(!res.slot_consumed);
        assert_eq!(res.spell_save_dc, 13); // 8 + 2 (prof) + 3 (mod)
    }

    #[test]
    fn test_cast_level1_spell_consumes_slot() {
        let attrs = AbilityScores::new(10, 10, 10, 16, 10, 10);
        let mut mage = Entity::new("Mage", attrs.clone(), ClassVariant::Wizard(Wizard), RaceVariant::Human(Human), create_acolyte());
        let mut ally = Entity::new("Ally", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());

        let mut slots = SpellSlots::new();
        slots.set_slots(1, 2);
        mage.spell_slots = Some(slots);

        ally.current_hp = 2; // Alvo ferido

        let cure = create_cure_wounds();
        let res = cast_spell(&mut mage, Some(&mut ally), &cure, Ability::Intelligence, false, DiceMode::Normal, DamageAffinity::Normal);

        assert!(res.slot_consumed);
        assert_eq!(mage.spell_slots.as_ref().unwrap().slots[0].current, 1);
        assert!(ally.current_hp > 2); // Vida restaurada
    }

    #[test]
    fn test_cast_fireball_save_roll() {
        let attrs = AbilityScores::new(10, 14, 10, 16, 10, 10);
        let mut mage = Entity::new("Mage", attrs.clone(), ClassVariant::Wizard(Wizard), RaceVariant::Human(Human), create_acolyte());
        let mut target = Entity::new("Target", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());

        let mut slots = SpellSlots::new();
        slots.set_slots(3, 1);
        mage.spell_slots = Some(slots);

        let fireball = create_fireball();
        let res = cast_spell(&mut mage, Some(&mut target), &fireball, Ability::Intelligence, true, DiceMode::Normal, DamageAffinity::Normal);

        assert!(res.slot_consumed);
        if let CastOutcome::SaveRoll { dc, final_damage, .. } = res.outcome {
            assert_eq!(dc, 13);
            assert!(final_damage >= 0);
        } else {
            panic!("Esperava resultado do tipo SaveRoll");
        }
    }
}
