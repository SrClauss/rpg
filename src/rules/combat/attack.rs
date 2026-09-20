use crate::dice::{Dice, DiceMode, RollResult};
use crate::rules::character::attributes::Ability;
use crate::rules::character::entity::Entity;
use crate::rules::equipment::weapons::{DamageType, Weapon, WeaponProperty};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Afinidade da Entidade a determinado tipo de dano
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageAffinity {
    Normal,
    Resistant,  // Metade do dano (arredondado para baixo)
    Immune,     // Dano zerado (0)
    Vulnerable, // Dobro do dano (2x)
}

/// Resultado da Resolução de um Ataque
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackResult {
    pub attack_roll: RollResult,
    pub target_ac: i32,
    pub is_hit: bool,
    pub is_critical_hit: bool,
    pub is_critical_fail: bool,
    pub raw_damage: i32,
    pub final_damage: i32,
    pub damage_type: DamageType,
    pub affinity: DamageAffinity,
}

/// Resolve um ataque completo de um Atacante contra um Defensor usando uma Arma.
pub fn resolve_attack(
    attacker: &mut Entity,
    defender: &mut Entity,
    weapon: &Weapon,
    is_proficient: bool,
    mode: DiceMode,
    affinity: DamageAffinity,
) -> AttackResult {
    let mut final_mode = mode;
    let mut has_advantage_effect = defender.applied_effects.iter().any(|e| {
        matches!(
            e.effect,
            crate::rules::character::active_effects::ActiveEffect::GrantsAdvantageToAttackers
        )
    });
    let mut has_disadvantage_effect = defender.applied_effects.iter().any(|e| {
        matches!(
            e.effect,
            crate::rules::character::active_effects::ActiveEffect::DisadvantageOnAttacksAgainstMe
        )
    });

    let is_ranged_attack = matches!(
        weapon.category,
        crate::rules::equipment::weapons::WeaponCategory::SimpleRanged
            | crate::rules::equipment::weapons::WeaponCategory::MartialRanged
    ) || weapon
        .properties
        .iter()
        .any(|p| matches!(p, WeaponProperty::Thrown(_, _)));

    let is_spell_attack = weapon.id.starts_with("spell_");

    if is_spell_attack {
        let has_spell_disadvantage = defender.applied_effects.iter().any(|e| {
            matches!(e.effect, crate::rules::character::active_effects::ActiveEffect::DisadvantageOnSpellAttacksAgainstMe)
        });
        has_disadvantage_effect = has_disadvantage_effect || has_spell_disadvantage;
    }

    let has_creature_type_disadvantage = attacker.applied_effects.iter().any(|e| {
        if let crate::rules::character::active_effects::ActiveEffect::DisadvantageOnAttacksAgainstCreatureType(ctype) = &e.effect {
            ctype == &defender.get_creature_type()
        } else {
            false
        }
    });
    has_disadvantage_effect = has_disadvantage_effect || has_creature_type_disadvantage;

    if has_advantage_effect && !has_disadvantage_effect {
        final_mode = DiceMode::Advantage;
    } else if has_disadvantage_effect && !has_advantage_effect {
        final_mode = DiceMode::Disadvantage;
    } else if has_advantage_effect && has_disadvantage_effect {
        final_mode = DiceMode::Normal;
    }

    let str_mod = attacker.attributes.get_modifier(Ability::Strength) as i32;
    let dex_mod = attacker.attributes.get_modifier(Ability::Dexterity) as i32;

    // Se a arma tiver Acuidade (Finesse), utiliza o maior modificador entre FOR e DES
    let is_ranged_weapon = matches!(
        weapon.category,
        crate::rules::equipment::weapons::WeaponCategory::SimpleRanged
            | crate::rules::equipment::weapons::WeaponCategory::MartialRanged
    );

    let ability_mod = if weapon.has_property(WeaponProperty::Finesse) {
        str_mod.max(dex_mod)
    } else if is_ranged_weapon {
        dex_mod
    } else {
        str_mod
    };

    // INVISIBILITY HOOK: Attacking breaks invisibility
    if attacker.has_condition(&crate::rules::conditions::Condition::Invisible) {
        attacker.remove_condition(&crate::rules::conditions::Condition::Invisible);
    }

    let mut attack_mod = ability_mod;
    if is_proficient {
        attack_mod += attacker.proficiency_bonus();
    }

    let mut magic_weapon_bonus = weapon.magic_bonus;
    for applied in &attacker.applied_effects {
        if let crate::rules::character::active_effects::ActiveEffect::MagicWeaponBonus(b) =
            &applied.effect
        {
            magic_weapon_bonus = std::cmp::max(magic_weapon_bonus, *b as i32);
        }
    }
    attack_mod += magic_weapon_bonus;

    let mut target_ac = defender.get_armor_class();
    if is_ranged_attack {
        for e in &defender.applied_effects {
            if let crate::rules::character::active_effects::ActiveEffect::BonusACAgainstRanged(b) =
                &e.effect
            {
                target_ac += b;
            }
        }
    }

    // Verifica se o defensor está sob o efeito de Sanctuary
    let is_sanctuary_blocked = defender
        .applied_effects
        .iter()
        .any(|e| e.source_name == "Sanctuary");
    if is_sanctuary_blocked {
        // Se falhar no teste de sabedoria contra a magia, o ataque é completamente bloqueado
        let wis_save = crate::rules::checks::roll_saving_throw(
            attacker,
            Ability::Wisdom,
            false,
            final_mode,
            Some(13),
        );
        if !wis_save.is_success.unwrap_or(false) {
            let mut result_roll = Dice::new(20, 1, attack_mod, final_mode).roll();
            result_roll.total = 0; // Ataque cancelado pelo Santuário
            return AttackResult {
                attack_roll: result_roll,
                target_ac,
                is_hit: false,
                is_critical_hit: false,
                is_critical_fail: false,
                raw_damage: 0,
                final_damage: 0,
                damage_type: weapon.damage_type,
                affinity,
            };
        }
    }

    // Rolagem de Ataque
    let attack_dice = Dice::new(20, 1, attack_mod, final_mode);
    let attack_roll = attack_dice.roll();
    // target_ac is already calculated

    let mut is_critical_hit = attack_roll.is_critical_hit;
    let is_critical_fail = attack_roll.is_critical_fail;

    // No D&D 5e: 20 natural é acerto automático, 1 natural é falha automática
    let mut is_hit = if is_critical_fail {
        false
    } else if is_critical_hit {
        true
    } else {
        attack_roll.total >= target_ac
    };

    // Imunidade a Acertos Críticos (ex: Adamantine Armor)
    if is_critical_hit {
        let is_immune_to_crit = defender.applied_effects.iter().any(|e| {
            matches!(
                e.effect,
                crate::rules::character::active_effects::ActiveEffect::ImmuneToCriticalHits
            )
        });
        if is_immune_to_crit {
            is_critical_hit = false;
        }
    }

    // MIRROR IMAGE HOOK
    if is_hit {
        // Find if defender has Mirror Image
        let mut image_index = None;
        let mut images_left = 0;
        for (i, applied) in defender.applied_effects.iter().enumerate() {
            if let crate::rules::character::active_effects::ActiveEffect::MirrorImage(count) =
                applied.effect
            {
                image_index = Some(i);
                images_left = count;
                break;
            }
        }

        if let Some(idx) = image_index {
            if images_left > 0 {
                // Roll to see if it hits an image instead.
                // 3 images: roll >= 6. 2 images: roll >= 8. 1 image: roll >= 11 on a d20.
                let target_roll = match images_left {
                    3 => 6,
                    2 => 8,
                    _ => 11,
                };
                let image_roll = Dice::new(20, 1, 0, DiceMode::Normal).roll();
                if image_roll.total >= target_roll {
                    // Hits image instead!
                    is_hit = false;

                    // Reduce image count or remove effect
                    if images_left == 1 {
                        defender.applied_effects.remove(idx);
                    } else {
                        defender.applied_effects[idx].effect =
                            crate::rules::character::active_effects::ActiveEffect::MirrorImage(
                                images_left - 1,
                            );
                    }
                }
            }
        }
    }

    if !is_hit {
        return AttackResult {
            attack_roll,
            target_ac,
            is_hit: false,
            is_critical_hit,
            is_critical_fail,
            raw_damage: 0,
            final_damage: 0,
            damage_type: weapon.damage_type,
            affinity,
        };
    }

    // Cálculo do Dano
    let mut base_dice = Dice::from_str(&weapon.damage_dice)
        .unwrap_or_else(|_| Dice::new(6, 1, 0, DiceMode::Normal));

    // No Acerto Crítico, a quantidade de dados de dano é dobrada!
    if is_critical_hit {
        base_dice.quantity *= 2;
    }

    // Adiciona o modificador de habilidade ao dano (mínimo de 0 dano bruto antes de afinidade)
    base_dice.modifier = ability_mod + magic_weapon_bonus;
    let damage_roll = base_dice.roll();
    let mut raw_damage = std::cmp::max(0, damage_roll.total);

    // Helper para processar efeitos aninhados
    fn process_effect_for_damage(
        effect: &crate::rules::character::active_effects::ActiveEffect,
        raw_damage: &mut i32,
        is_critical_hit: bool,
        defender_creature_type: &crate::rules::monsters::monster::CreatureType,
    ) {
        match effect {
            crate::rules::character::active_effects::ActiveEffect::NextWeaponAttackBonusDamage(
                dice_str,
                _dmg_type,
            ) => {
                let mut bonus_dice = Dice::from_str(dice_str)
                    .unwrap_or_else(|_| Dice::new(6, 1, 0, DiceMode::Normal));
                if is_critical_hit {
                    bonus_dice.quantity *= 2;
                }
                *raw_damage += std::cmp::max(0, bonus_dice.roll().total);
            }
            crate::rules::character::active_effects::ActiveEffect::WeaponDamageBonus(dice_str) => {
                let mut bonus_dice = Dice::from_str(dice_str)
                    .unwrap_or_else(|_| Dice::new(4, 1, 0, DiceMode::Normal));
                if is_critical_hit {
                    bonus_dice.quantity *= 2;
                }
                *raw_damage += std::cmp::max(0, bonus_dice.roll().total);
            }
            crate::rules::character::active_effects::ActiveEffect::WeaponDamagePenalty(
                dice_str,
            ) => {
                let penalty_dice = Dice::from_str(dice_str)
                    .unwrap_or_else(|_| Dice::new(4, 1, 0, DiceMode::Normal));
                *raw_damage -= std::cmp::max(0, penalty_dice.roll().total);
            }
            crate::rules::character::active_effects::ActiveEffect::ConditionalDamageAgainstCreature(
                target_type,
                dice_str,
                _dmg_type,
            ) => {
                if target_type == defender_creature_type {
                    let mut bonus_dice = Dice::from_str(dice_str)
                        .unwrap_or_else(|_| Dice::new(6, 1, 0, DiceMode::Normal));
                    if is_critical_hit {
                        bonus_dice.quantity *= 2;
                    }
                    *raw_damage += std::cmp::max(0, bonus_dice.roll().total);
                }
            }
            crate::rules::character::active_effects::ActiveEffect::BonusDamageOnCritical(
                dice_str,
                _dmg_type,
            ) => {
                if is_critical_hit {
                    let bonus_dice = Dice::from_str(dice_str)
                        .unwrap_or_else(|_| Dice::new(6, 1, 0, DiceMode::Normal));
                    // Usually this is flat or extra dice that aren't doubled, we just roll it.
                    *raw_damage += std::cmp::max(0, bonus_dice.roll().total);
                }
            }
            crate::rules::character::active_effects::ActiveEffect::Multiple(effects) => {
                for sub in effects {
                    process_effect_for_damage(sub, raw_damage, is_critical_hit, defender_creature_type);
                }
            }
            _ => {}
        }
    }

    let defender_ctype = defender.get_creature_type();

    // Weapon's own specific active effects
    for effect in &weapon.magic_effects {
        process_effect_for_damage(effect, &mut raw_damage, is_critical_hit, &defender_ctype);
    }

    // Extra damage from Smites or Hunter's Mark
    for applied in &attacker.applied_effects {
        process_effect_for_damage(
            &applied.effect,
            &mut raw_damage,
            is_critical_hit,
            &defender_ctype,
        );
    }
    raw_damage = std::cmp::max(0, raw_damage);

    let mut final_damage_type = weapon.damage_type;
    for effect in &weapon.magic_effects {
        if let crate::rules::character::active_effects::ActiveEffect::OverrideDamageType(dt) =
            effect
        {
            final_damage_type = *dt;
        }
    }

    let mut current_affinity = affinity;

    // Verifica Resistência e Vulnerabilidade nativa pelo ActiveEffect
    let has_native_resistance = defender.applied_effects.iter().any(|e| {
        if let crate::rules::character::active_effects::ActiveEffect::Resistance(types) = &e.effect
        {
            types.contains(&weapon.damage_type)
        } else {
            false
        }
    });

    let has_native_vulnerability = defender.applied_effects.iter().any(|e| {
        if let crate::rules::character::active_effects::ActiveEffect::Vulnerability(types) =
            &e.effect
        {
            types.contains(&weapon.damage_type)
        } else {
            false
        }
    });

    if current_affinity == DamageAffinity::Normal {
        if has_native_vulnerability && has_native_resistance {
            current_affinity = DamageAffinity::Normal; // Se anulam
        } else if has_native_vulnerability {
            current_affinity = DamageAffinity::Vulnerable;
        } else if has_native_resistance {
            current_affinity = DamageAffinity::Resistant;
        }
    }

    // Verifica Imunidade/Resistência a ataques não mágicos
    let is_magical_attack = magic_weapon_bonus > 0 || weapon.id.starts_with("spell_");
    let is_non_magical_physical = !is_magical_attack
        && matches!(
            weapon.damage_type,
            DamageType::Bludgeoning | DamageType::Piercing | DamageType::Slashing
        );

    if is_non_magical_physical {
        let has_non_magical_resistance = defender.applied_effects.iter().any(|e| {
            matches!(
                e.effect,
                crate::rules::character::active_effects::ActiveEffect::ResistanceToNonMagicalDamage
            )
        });
        let has_non_magical_immunity = defender.applied_effects.iter().any(|e| {
            matches!(
                e.effect,
                crate::rules::character::active_effects::ActiveEffect::ImmuneToNonMagicalDamage
            )
        });

        if has_non_magical_immunity {
            current_affinity = DamageAffinity::Immune;
        } else if has_non_magical_resistance && current_affinity == DamageAffinity::Normal {
            current_affinity = DamageAffinity::Resistant;
        }
    }

    // Aplicação de Resistência / Imunidade / Vulnerabilidade
    let final_damage = match current_affinity {
        DamageAffinity::Normal => raw_damage,
        DamageAffinity::Resistant => raw_damage / 2,
        DamageAffinity::Immune => 0,
        DamageAffinity::Vulnerable => raw_damage * 2,
    };

    AttackResult {
        attack_roll,
        target_ac,
        is_hit: true,
        is_critical_hit,
        is_critical_fail,
        raw_damage,
        final_damage,
        damage_type: final_damage_type,
        affinity: current_affinity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::background::create_acolyte;
    use crate::rules::character::class::{ClassVariant, Fighter};
    use crate::rules::character::race::{Human, RaceVariant};
    use crate::rules::equipment::weapons::{create_dagger, create_longsword};

    #[test]
    fn test_attack_resolution_hits_or_misses() {
        let attrs = AbilityScores::new(16, 10, 10, 10, 10, 10); // STR 16 (+3, +1 Human = 17 -> +3)
        let mut attacker = Entity::new(
            "Attacker",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut defender = Entity::new(
            "Defender",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let sword = create_longsword();

        // Testar múltiplos ataques contra CA 13
        for _ in 0..50 {
            let result = resolve_attack(
                &mut attacker,
                &mut defender,
                &sword,
                true,
                DiceMode::Normal,
                DamageAffinity::Normal,
            );
            if result.is_hit {
                assert!(result.final_damage > 0);
            } else {
                assert_eq!(result.final_damage, 0);
            }
        }
    }

    #[test]
    fn test_finesse_weapon_uses_dexterity() {
        let attrs = AbilityScores::new(8, 16, 10, 10, 10, 10); // DEX 16 (+3, +1 Human = 17 -> +3), STR 8 (-1)
        let mut attacker = Entity::new(
            "Rogue",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut defender = Entity::new(
            "Target",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );

        let dagger = create_dagger(); // Finesse
        let result = resolve_attack(
            &mut attacker,
            &mut defender,
            &dagger,
            true,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );

        // Modificador de ataque deve ser DEX (+3) + Proficiência (+2) = +5
        assert_eq!(result.attack_roll.modifier, 5);
    }

    #[test]
    fn test_damage_affinities() {
        let attrs = AbilityScores::new(14, 10, 10, 10, 10, 10);
        let mut attacker = Entity::new(
            "Attacker",
            attrs.clone(),
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let mut defender = Entity::new(
            "Defender",
            attrs,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            create_acolyte(),
        );
        let sword = create_longsword();

        // Forçar acerto com afinidade de resistência
        let res_resistant = resolve_attack(
            &mut attacker,
            &mut defender,
            &sword,
            true,
            DiceMode::Normal,
            DamageAffinity::Resistant,
        );
        if res_resistant.is_hit {
            assert_eq!(res_resistant.final_damage, res_resistant.raw_damage / 2);
        }

        // Afinidade de Imunidade zera o dano final
        let res_immune = resolve_attack(
            &mut attacker,
            &mut defender,
            &sword,
            true,
            DiceMode::Normal,
            DamageAffinity::Immune,
        );
        if res_immune.is_hit {
            assert_eq!(res_immune.final_damage, 0);
        }

        // Afinidade de Vulnerabilidade dobra o dano final
        let res_vulnerable = resolve_attack(
            &mut attacker,
            &mut defender,
            &sword,
            true,
            DiceMode::Normal,
            DamageAffinity::Vulnerable,
        );
        if res_vulnerable.is_hit {
            assert_eq!(res_vulnerable.final_damage, res_vulnerable.raw_damage * 2);
        }
    }
}
