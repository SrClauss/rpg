use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::{create_dagger, DamageType};
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, Monster, MonsterAction, MonsterSense, UsageLimit,
};

/// 1. Aboleth (CR 10) - SRD 5.1 p. 261
pub fn create_aboleth() -> Monster {
    let attrs = AbilityScores::new(21, 9, 15, 18, 15, 18);
    let entity = Entity::new(
        "Aboleth",
        attrs,
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    );

    let mut aboleth = Monster::new("aboleth", entity, CreatureType::Aberration, CreatureSize::Large, 10.0, 5900);
    aboleth.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    aboleth.add_sense(MonsterSense::PassivePerception(20));

    aboleth.add_trait("Amphibious", "Pode respirar ar e água.");
    aboleth.add_trait(
        "Mucous Cloud",
        "Enquanto submerso, criatura a 5ft que tocar ou atingir em corpo a corpo deve passar em CD 14 CON ou não conseguir respirar fora d'água por 1d4 horas.",
    );
    aboleth.add_trait(
        "Probing Telepathy",
        "Se telepatia for usada, descobre o maior desejo da criatura se ela sofrer dano ou falhar na salvaguarda.",
    );

    aboleth.add_action(MonsterAction::attack("Tentacle", create_dagger()));
    aboleth.add_action(MonsterAction::attack("Tail", create_dagger()));
    aboleth.add_action(
        MonsterAction::new(
            "Enslave",
            ActionType::Action,
            "Alvo a 30ft deve passar em CD 14 WIS ou fica encantado sob controle do Aboleth.",
            None,
        )
        .with_usage(UsageLimit::PerDay(3)),
    );

    aboleth.set_legendary(3);
    aboleth
}

/// 2. Cloaker (CR 8) - SRD 5.1 p. 268
pub fn create_cloaker() -> Monster {
    let attrs = AbilityScores::new(17, 15, 12, 13, 12, 14);
    let entity = Entity::new(
        "Cloaker",
        attrs,
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    );

    let mut cloaker = Monster::new("cloaker", entity, CreatureType::Aberration, CreatureSize::Large, 8.0, 3900);
    cloaker.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    cloaker.add_sense(MonsterSense::PassivePerception(11));

    cloaker.add_trait(
        "Damage Transfer",
        "Enquanto agarrado/anexado a uma criatura, transfere metade do dano recebido para ela.",
    );
    cloaker.add_trait("False Appearance", "Indistingüível de um manto de couro escuro enquanto imóvel.");
    cloaker.add_trait("Light Sensitivity", "Desvantagem em ataques e Percepção sob luz brilhante.");

    cloaker.add_action(MonsterAction::attack("Bite", create_dagger()));
    cloaker.add_action(MonsterAction::attack("Tail", create_dagger()));
    cloaker.add_action(MonsterAction::new(
        "Moan",
        ActionType::Action,
        "Grito aterrorizante: criaturas a 60ft devem passar em CD 13 WIS ou ficam Frightened.",
        None,
    ));
    cloaker.add_action(MonsterAction::new(
        "Phantasmal Duplicates",
        ActionType::Action,
        "Cria 3 duplicatas ilusórias (similar à magia Mirror Image).",
        None,
    ).with_usage(UsageLimit::PerDay(3)));

    cloaker
}

/// 3. Gibbering Mouther (CR 2) - SRD 5.1 p. 288
pub fn create_gibbering_mouther() -> Monster {
    let attrs = AbilityScores::new(10, 8, 16, 3, 10, 6);
    let entity = Entity::new(
        "Gibbering Mouther",
        attrs,
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    );

    let mut mouther = Monster::new("gibbering_mouther", entity, CreatureType::Aberration, CreatureSize::Medium, 2.0, 450);
    mouther.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    mouther.add_sense(MonsterSense::PassivePerception(10));

    mouther.add_trait(
        "Gibbering",
        "Tagarela incoerentemente: criaturas a 20ft no início do turno devem passar em CD 10 WIS ou sofrem confusão.",
    );
    mouther.add_trait("Aberrant Ground", "O chão num raio de 10ft se torna terreno difícil viscoso.");

    mouther.add_action(MonsterAction::attack("Bites", create_dagger()));
    mouther.add_action(MonsterAction::new(
        "Blinding Spittle",
        ActionType::Action,
        "Cuspe cegante a 15ft: CD 13 DEX save ou fica Blinded por 1 minuto (Recharge 5-6).",
        None,
    ));

    mouther
}

/// 4. Otyugh (CR 5) - SRD 5.1 p. 339
pub fn create_otyugh() -> Monster {
    let attrs = AbilityScores::new(16, 11, 19, 6, 13, 6);
    let entity = Entity::new(
        "Otyugh",
        attrs,
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    );

    let mut otyugh = Monster::new("otyugh", entity, CreatureType::Aberration, CreatureSize::Large, 5.0, 1800);
    otyugh.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    otyugh.add_sense(MonsterSense::PassivePerception(11));

    otyugh.add_trait(
        "Limited Telepathy",
        "Transmite mensagens e imagens simples telepaticamente a 120ft.",
    );

    otyugh.add_action(MonsterAction::attack("Bite", create_dagger()));
    otyugh.add_action(MonsterAction::attack("Tentacle", create_dagger()));
    otyugh.add_action(MonsterAction::new(
        "Tentacle Slam",
        ActionType::Action,
        "Bate duas criaturas agarradas uma contra a outra causando 2d6+3 de dano.",
        None,
    ));

    otyugh
}

/// 5. Roper (CR 5) - SRD 5.1 p. 342
pub fn create_roper() -> Monster {
    let attrs = AbilityScores::new(18, 8, 17, 7, 16, 6);
    let entity = Entity::new(
        "Roper",
        attrs,
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    );

    let mut roper = Monster::new("roper", entity, CreatureType::Aberration, CreatureSize::Large, 5.0, 1800);
    roper.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    roper.add_sense(MonsterSense::PassivePerception(16));

    roper.add_trait("False Appearance", "Indistingüível de uma estalagmite natural enquanto imóvel.");
    roper.add_trait("Grasping Tendrils", "Possui 6 tendões que podem agarrar alvos a até 50ft.");
    roper.add_trait("Spider Climb", "Pode escalar superfícies difíceis sem teste.");

    roper.add_action(MonsterAction::attack("Bite", create_dagger()));
    roper.add_action(MonsterAction::attack("Tendril", create_dagger()));
    roper.add_action(MonsterAction::new(
        "Reel",
        ActionType::BonusAction,
        "Puxa até duas criaturas agarradas pelos tendões em até 25ft em sua direção.",
        None,
    ));

    roper
}

/// 6. Will-o'-Wisp (CR 2) - SRD 5.1 p. 358
pub fn create_will_o_wisp() -> Monster {
    let attrs = AbilityScores::new(1, 28, 10, 13, 14, 11);
    let entity = Entity::new(
        "Will-o'-Wisp",
        attrs,
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    );

    let mut wisp = Monster::new("will_o_wisp", entity, CreatureType::Aberration, CreatureSize::Tiny, 2.0, 450);
    wisp.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    wisp.add_sense(MonsterSense::PassivePerception(12));

    wisp.resistances.damage_immunities.push(DamageType::Lightning);
    wisp.resistances.damage_immunities.push(DamageType::Poison);

    wisp.add_trait("Consume Life", "Ação bônus para matar criatura com 0 HP a 5ft e recuperar 2d6 HP.");
    wisp.add_trait("Ephemeral", "Pode passar por espaço de outras criaturas e objetos como terreno difícil.");
    wisp.add_trait("Invisibility", "Pode se tornar invisível como ação bônus.");

    wisp.add_action(MonsterAction::attack("Shock", create_dagger()));

    wisp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aberrations_creation() {
        let aboleth = create_aboleth();
        assert_eq!(aboleth.id, "aboleth");
        assert_eq!(aboleth.challenge_rating, 10.0);
        assert!(aboleth.legendary.is_some());

        let cloaker = create_cloaker();
        assert_eq!(cloaker.id, "cloaker");
        assert_eq!(cloaker.challenge_rating, 8.0);

        let mouther = create_gibbering_mouther();
        assert_eq!(mouther.id, "gibbering_mouther");

        let otyugh = create_otyugh();
        assert_eq!(otyugh.id, "otyugh");

        let roper = create_roper();
        assert_eq!(roper.id, "roper");

        let wisp = create_will_o_wisp();
        assert_eq!(wisp.id, "will_o_wisp");
        assert_eq!(wisp.size, CreatureSize::Tiny);
    }
}
