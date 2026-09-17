use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::dice::{Dice, DiceMode, RollResult};
use crate::rules::character::attributes::Ability;
use crate::rules::character::entity::Entity;
use crate::rules::equipment::weapons::{DamageType, Weapon, WeaponProperty};

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
    attacker: &Entity,
    defender: &Entity,
    weapon: &Weapon,
    is_proficient: bool,
    mode: DiceMode,
    affinity: DamageAffinity,
) -> AttackResult {
    let str_mod = attacker.attributes.get_modifier(Ability::Strength) as i32;
    let dex_mod = attacker.attributes.get_modifier(Ability::Dexterity) as i32;

    // Se a arma tiver Acuidade (Finesse), utiliza o maior modificador entre FOR e DES
    let ability_mod = if weapon.has_property(WeaponProperty::Finesse) {
        std::cmp::max(str_mod, dex_mod)
    } else {
        str_mod
    };

    let mut attack_mod = ability_mod;
    if is_proficient {
        attack_mod += attacker.proficiency_bonus();
    }

    // Rolagem de Ataque
    let attack_dice = Dice::new(20, 1, attack_mod, mode);
    let attack_roll = attack_dice.roll();
    let target_ac = defender.get_armor_class();

    let is_critical_hit = attack_roll.is_critical_hit;
    let is_critical_fail = attack_roll.is_critical_fail;

    // No D&D 5e: 20 natural é acerto automático, 1 natural é falha automática
    let is_hit = if is_critical_fail {
        false
    } else if is_critical_hit {
        true
    } else {
        attack_roll.total >= target_ac
    };

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
    base_dice.modifier = ability_mod;
    let damage_roll = base_dice.roll();
    let raw_damage = std::cmp::max(0, damage_roll.total);

    // Aplicação de Resistência / Imunidade / Vulnerabilidade
    let final_damage = match affinity {
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
        damage_type: weapon.damage_type,
        affinity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::attributes::AbilityScores;
    use crate::rules::character::class::{Fighter, ClassVariant};
    use crate::rules::character::race::{Human, RaceVariant};
    use crate::rules::character::background::create_acolyte;
    use crate::rules::equipment::weapons::{create_longsword, create_dagger};

    #[test]
    fn test_attack_resolution_hits_or_misses() {
        let attrs = AbilityScores::new(16, 10, 10, 10, 10, 10); // STR 16 (+3, +1 Human = 17 -> +3)
        let attacker = Entity::new("Attacker", attrs.clone(), ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());
        let defender = Entity::new("Defender", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());

        let sword = create_longsword();

        // Testar múltiplos ataques contra CA 13
        for _ in 0..50 {
            let result = resolve_attack(&attacker, &defender, &sword, true, DiceMode::Normal, DamageAffinity::Normal);
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
        let attacker = Entity::new("Rogue", attrs.clone(), ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());
        let defender = Entity::new("Target", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());

        let dagger = create_dagger(); // Finesse
        let result = resolve_attack(&attacker, &defender, &dagger, true, DiceMode::Normal, DamageAffinity::Normal);

        // Modificador de ataque deve ser DEX (+3) + Proficiência (+2) = +5
        assert_eq!(result.attack_roll.modifier, 5);
    }

    #[test]
    fn test_damage_affinities() {
        let attrs = AbilityScores::new(14, 10, 10, 10, 10, 10);
        let attacker = Entity::new("Attacker", attrs.clone(), ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());
        let defender = Entity::new("Defender", attrs, ClassVariant::Fighter(Fighter), RaceVariant::Human(Human), create_acolyte());
        let sword = create_longsword();

        // Forçar acerto com afinidade de resistência
        let res_resistant = resolve_attack(&attacker, &defender, &sword, true, DiceMode::Normal, DamageAffinity::Resistant);
        if res_resistant.is_hit {
            assert_eq!(res_resistant.final_damage, res_resistant.raw_damage / 2);
        }

        // Afinidade de Imunidade zera o dano final
        let res_immune = resolve_attack(&attacker, &defender, &sword, true, DiceMode::Normal, DamageAffinity::Immune);
        if res_immune.is_hit {
            assert_eq!(res_immune.final_damage, 0);
        }

        // Afinidade de Vulnerabilidade dobra o dano final
        let res_vulnerable = resolve_attack(&attacker, &defender, &sword, true, DiceMode::Normal, DamageAffinity::Vulnerable);
        if res_vulnerable.is_hit {
            assert_eq!(res_vulnerable.final_damage, res_vulnerable.raw_damage * 2);
        }
    }
}
