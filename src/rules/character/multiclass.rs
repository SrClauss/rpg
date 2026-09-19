use serde::{Deserialize, Serialize};
use super::attributes::AbilityScores;
use super::class::ClassVariant;
use super::entity::Entity;
use crate::rules::spells::spellcaster::{SpellSlotLevel, SpellSlots};

/// Valida os pré-requisitos de atributo para Multiclasse (SRD 5.1 Página 56)
pub fn check_multiclass_prerequisites(attrs: &AbilityScores, class: &ClassVariant) -> Result<(), String> {
    let str = attrs.strength.0;
    let dex = attrs.dexterity.0;
    let int = attrs.intelligence.0;
    let wis = attrs.wisdom.0;
    let cha = attrs.charisma.0;

    let satisfied = match class {
        ClassVariant::Barbarian(_) => str >= 13,
        ClassVariant::Bard(_) => cha >= 13,
        ClassVariant::Cleric(_) => wis >= 13,
        ClassVariant::Druid(_) => wis >= 13,
        ClassVariant::Fighter(_) => str >= 13 || dex >= 13,
        ClassVariant::Monk(_) => dex >= 13 && wis >= 13,
        ClassVariant::Paladin(_) => str >= 13 && cha >= 13,
        ClassVariant::Ranger(_) => dex >= 13 && wis >= 13,
        ClassVariant::Rogue(_) => dex >= 13,
        ClassVariant::Sorcerer(_) => cha >= 13,
        ClassVariant::Warlock(_) => cha >= 13,
        ClassVariant::Wizard(_) => int >= 13,
    };

    if satisfied {
        Ok(())
    } else {
        Err(format!(
            "Atributos insuficientes para multiclasse em {}. Exige valor mínimo de 13 no(s) atributo(s) da classe.",
            class.as_trait().get_name()
        ))
    }
}

/// Valida se uma entidade pode fazer multiclasse em uma nova classe (valida tanto a classe atual quanto a nova)
pub fn can_multiclass_into(entity: &Entity, target_class: &ClassVariant) -> Result<(), String> {
    // 1. Deve satisfazer a classe atual
    if let Some(primary) = entity.classes.first() {
        check_multiclass_prerequisites(&entity.attributes, primary)?;
    }
    // 2. Deve satisfazer a classe alvo
    check_multiclass_prerequisites(&entity.attributes, target_class)?;
    Ok(())
}

/// Tabela Oficial de Conjuração Multiclasse (SRD 5.1 Página 58)
/// Retorna os Espaços de Magia (Spell Slots) por Nível Efetivo de Conjurador (1 a 20)
pub fn multiclass_spell_slots_table(caster_level: u8) -> Option<SpellSlots> {
    if caster_level == 0 {
        return None;
    }

    let tuple = match caster_level {
        1 => (2, 0, 0, 0, 0, 0, 0, 0, 0),
        2 => (3, 0, 0, 0, 0, 0, 0, 0, 0),
        3 => (4, 2, 0, 0, 0, 0, 0, 0, 0),
        4 => (4, 3, 0, 0, 0, 0, 0, 0, 0),
        5 => (4, 3, 2, 0, 0, 0, 0, 0, 0),
        6 => (4, 3, 3, 0, 0, 0, 0, 0, 0),
        7 => (4, 3, 3, 1, 0, 0, 0, 0, 0),
        8 => (4, 3, 3, 2, 0, 0, 0, 0, 0),
        9 => (4, 3, 3, 3, 1, 0, 0, 0, 0),
        10 => (4, 3, 3, 3, 2, 0, 0, 0, 0),
        11 => (4, 3, 3, 3, 2, 1, 0, 0, 0),
        12 => (4, 3, 3, 3, 2, 1, 0, 0, 0),
        13 => (4, 3, 3, 3, 2, 1, 1, 0, 0),
        14 => (4, 3, 3, 3, 2, 1, 1, 0, 0),
        15 => (4, 3, 3, 3, 2, 1, 1, 1, 0),
        16 => (4, 3, 3, 3, 2, 1, 1, 1, 0),
        17 => (4, 3, 3, 3, 2, 1, 1, 1, 1),
        18 => (4, 3, 3, 3, 3, 1, 1, 1, 1),
        19 => (4, 3, 3, 3, 3, 2, 1, 1, 1),
        _ => (4, 3, 3, 3, 3, 2, 2, 1, 1),
    };

    Some(SpellSlots {
        slots: [
            SpellSlotLevel::new(tuple.0),
            SpellSlotLevel::new(tuple.1),
            SpellSlotLevel::new(tuple.2),
            SpellSlotLevel::new(tuple.3),
            SpellSlotLevel::new(tuple.4),
            SpellSlotLevel::new(tuple.5),
            SpellSlotLevel::new(tuple.6),
            SpellSlotLevel::new(tuple.7),
            SpellSlotLevel::new(tuple.8),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::character::class::{Barbarian, Cleric, Fighter, Wizard};

    #[test]
    fn test_multiclass_prerequisites() {
        let attrs = AbilityScores::new(14, 10, 10, 10, 14, 10);
        let fighter = ClassVariant::Fighter(Fighter);
        let cleric = ClassVariant::Cleric(Cleric);
        let wizard = ClassVariant::Wizard(Wizard);

        // Fighter exige STR ou DEX 13 (STR 14 OK)
        assert!(check_multiclass_prerequisites(&attrs, &fighter).is_ok());

        // Cleric exige WIS 13 (WIS 14 OK)
        assert!(check_multiclass_prerequisites(&attrs, &cleric).is_ok());

        // Wizard exige INT 13 (INT 10 Fails)
        assert!(check_multiclass_prerequisites(&attrs, &wizard).is_err());
    }

    #[test]
    fn test_multiclass_spell_slots_table() {
        let slots_lvl1 = multiclass_spell_slots_table(1).unwrap();
        assert_eq!(slots_lvl1.slots[0].max, 2);
        assert_eq!(slots_lvl1.slots[1].max, 0);

        let slots_lvl5 = multiclass_spell_slots_table(5).unwrap();
        assert_eq!(slots_lvl5.slots[0].max, 4);
        assert_eq!(slots_lvl5.slots[1].max, 3);
        assert_eq!(slots_lvl5.slots[2].max, 2);

        let slots_lvl20 = multiclass_spell_slots_table(20).unwrap();
        assert_eq!(slots_lvl20.slots[8].max, 1);
    }
}
