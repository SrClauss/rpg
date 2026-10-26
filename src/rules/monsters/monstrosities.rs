use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::{create_dagger, DamageType};
use crate::rules::monsters::monster::{
    ActionType, CreatureSize, CreatureType, Monster, MonsterAction, MonsterSense,
};

fn create_monstrosity_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    )
}

fn create_monstrosity_base(id: &str, name: &str, size: CreatureSize, cr: f32, xp: u32, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Monster {
    let mut m = Monster::new(id, create_monstrosity_entity(name, str_, dex, con, int_, wis, cha), CreatureType::Monstrosity, size, cr, xp);
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_ankheg() -> Monster {
    let mut m = create_monstrosity_base("ankheg", "Ankheg", CreatureSize::Large, 2.0, 450, 17, 11, 13, 1, 10, 6);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_sense(MonsterSense::Tremorsense { range_feet: 60 });
    m.add_action(MonsterAction::new("Acid Spray", ActionType::Action, "Cuspe de ácido em linha de 30ft (Recharge 6).", None));
    m
}

pub fn create_basilisk() -> Monster {
    let mut m = create_monstrosity_base("basilisk", "Basilisk", CreatureSize::Medium, 3.0, 700, 16, 8, 15, 2, 8, 7);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Petrifying Gaze", "Olhar petrificante obriga salvaguarda de CON a 30ft para não virar pedra.");
    m
}

pub fn create_behir() -> Monster {
    let mut m = create_monstrosity_base("behir", "Behir", CreatureSize::Huge, 11.0, 7200, 23, 16, 18, 7, 14, 12);
    m.add_sense(MonsterSense::Darkvision { range_feet: 90 });
    m.resistances.damage_immunities.push(DamageType::Lightning);
    m.add_action(MonsterAction::new("Lightning Breath", ActionType::Action, "Relâmpago de 20ft em linha (Recharge 5-6).", None));
    m.add_action(MonsterAction::attack("Constrict", create_dagger()));
    m
}

pub fn create_bulette() -> Monster {
    let mut m = create_monstrosity_base("bulette", "Bulette", CreatureSize::Large, 5.0, 1800, 19, 11, 21, 2, 10, 5);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_sense(MonsterSense::Tremorsense { range_feet: 60 });
    m.add_action(MonsterAction::new("Deadly Leap", ActionType::Action, "Salta até 30ft esmagando alvos que falharem no teste de DEX.", None));
    m
}

pub fn create_centaur() -> Monster {
    let mut m = create_monstrosity_base("centaur", "Centaur", CreatureSize::Large, 2.0, 450, 18, 14, 14, 9, 13, 11);
    m.add_action(MonsterAction::attack("Pike", create_dagger()));
    m.add_action(MonsterAction::attack("Hooves", create_dagger()));
    m
}

pub fn create_chimera() -> Monster {
    let mut m = create_monstrosity_base("chimera", "Chimera", CreatureSize::Large, 6.0, 2300, 19, 11, 19, 3, 14, 10);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_action(MonsterAction::attack("Horns", create_dagger()));
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m.add_action(MonsterAction::new("Fire Breath", ActionType::Action, "Sopro de fogo da cabeça de dragão (Recharge 5-6).", None));
    m
}

pub fn create_cockatrice() -> Monster {
    let mut m = create_monstrosity_base("cockatrice", "Cockatrice", CreatureSize::Small, 0.5, 100, 6, 12, 12, 2, 13, 5);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Petrifying Touch", "Bicada obriga salvaguarda de CON para não ficar petrificado.");
    m
}

pub fn create_darkmantle() -> Monster {
    let mut m = create_monstrosity_base("darkmantle", "Darkmantle", CreatureSize::Small, 0.5, 100, 16, 12, 13, 2, 10, 5);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: true });
    m.add_action(MonsterAction::new("Darkness Aura", ActionType::Action, "Cria aura de escuridão mágica num raio de 15ft (Recharge 6).", None));
    m
}

pub fn create_displacer_beast() -> Monster {
    let mut m = create_monstrosity_base("displacer_beast", "Displacer Beast", CreatureSize::Large, 3.0, 700, 18, 15, 16, 6, 12, 8);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Displacement", "Ataques contra ela têm desvantagem enquanto a ilusão de deslocamento estiver ativa.");
    m.add_trait("Avoidance", "Se passar numa salvaguarda de DEX para meio dano, não sofre dano nenhum.");
    m.add_action(MonsterAction::attack("Tentacle", create_dagger()));
    m
}

pub fn create_doppelganger() -> Monster {
    let mut m = create_monstrosity_base("doppelganger", "Doppelganger", CreatureSize::Medium, 3.0, 700, 11, 18, 14, 11, 12, 14);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Shapechanger", "Metamorfoseia-se perfeitamente em qualquer humanoide.");
    m.add_trait("Read Thoughts", "Lê pensamentos superficiais de alvos a 60ft.");
    m.add_action(MonsterAction::attack("Slam", create_dagger()));
    m
}

pub fn create_drider() -> Monster {
    let mut m = create_monstrosity_base("drider", "Drider", CreatureSize::Large, 6.0, 2300, 16, 16, 18, 13, 14, 12);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.add_trait("Fey Ancestry", "Vantagem contra encantamentos e imune a sono.");
    m.add_trait("Spider Climb", "Escala superfícies.");
    m.add_action(MonsterAction::attack("Longsword", create_dagger()));
    m.add_action(MonsterAction::attack("Longbow", create_dagger()));
    m
}

pub fn create_gorgon() -> Monster {
    let mut m = create_monstrosity_base("gorgon", "Gorgon", CreatureSize::Large, 5.0, 1800, 20, 11, 18, 2, 12, 7);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_action(MonsterAction::attack("Gore", create_dagger()));
    m.add_action(MonsterAction::new("Petrifying Breath", ActionType::Action, "Gás petrificante em cone de 30ft (Recharge 5-6).", None));
    m
}

pub fn create_grick() -> Monster {
    let mut m = create_monstrosity_base("grick", "Grick", CreatureSize::Medium, 2.0, 450, 14, 14, 11, 3, 14, 5);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.resistances.nonmagical_physical_resistance = true;
    m.add_action(MonsterAction::attack("Tentacles", create_dagger()));
    m
}

pub fn create_griffon() -> Monster {
    let mut m = create_monstrosity_base("griffon", "Griffon", CreatureSize::Large, 2.0, 450, 18, 15, 16, 2, 13, 8);
    m.add_action(MonsterAction::attack("Beak", create_dagger()));
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m
}

pub fn create_harpy() -> Monster {
    let mut m = create_monstrosity_base("harpy", "Harpy", CreatureSize::Medium, 1.0, 200, 12, 13, 12, 7, 10, 13);
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m.add_action(MonsterAction::new("Luring Song", ActionType::Action, "Canto mágico atrai alvos a 300ft encantados (CD 11 WIS).", None));
    m
}

pub fn create_hippogriff() -> Monster { create_monstrosity_base("hippogriff", "Hippogriff", CreatureSize::Large, 1.0, 200, 17, 13, 13, 2, 12, 8) }

pub fn create_hydra() -> Monster {
    let mut m = create_monstrosity_base("hydra", "Hydra", CreatureSize::Huge, 8.0, 3900, 20, 12, 20, 2, 10, 7);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Multiple Heads", "Possui 5 cabeças e ganha reações extras por rodada.");
    m.add_trait("Reactive Heads", "Cresce 2 novas cabeças ao perder uma a menos que receba fogo.");
    m
}

pub fn create_kraken() -> Monster {
    let mut m = create_monstrosity_base("kraken", "Kraken", CreatureSize::Gargantuan, 23.0, 50000, 30, 11, 25, 22, 18, 20);
    m.add_sense(MonsterSense::Truesight { range_feet: 120 });
    m.resistances.damage_immunities.push(DamageType::Lightning);
    m.resistances.nonmagical_physical_immunity = true;
    m.add_trait("Siege Monster", "Causa o dobro de dano a objetos e estruturas.");
    m.set_legendary(3);
    m
}

pub fn create_lamia() -> Monster {
    let mut m = create_monstrosity_base("lamia", "Lamia", CreatureSize::Large, 4.0, 1100, 16, 13, 15, 14, 15, 16);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_action(MonsterAction::attack("Intoxicating Touch", create_dagger()));
    m
}

pub fn create_manticore() -> Monster {
    let mut m = create_monstrosity_base("manticore", "Manticore", CreatureSize::Large, 3.0, 700, 17, 16, 17, 7, 12, 8);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_action(MonsterAction::attack("Tail Spike", create_dagger()));
    m
}

pub fn create_medusa() -> Monster {
    let mut m = create_monstrosity_base("medusa", "Medusa", CreatureSize::Medium, 6.0, 2300, 10, 15, 16, 12, 13, 15);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Petrifying Gaze", "Olhar petrificante transforma em pedra quem falhar em CD 14 CON.");
    m.add_action(MonsterAction::attack("Snake Hair", create_dagger()));
    m
}

pub fn create_minotaur() -> Monster {
    let mut m = create_monstrosity_base("minotaur", "Minotaur", CreatureSize::Large, 3.0, 700, 18, 11, 16, 6, 16, 9);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Labyrinthine Recall", "Nunca se perde em labirintos.");
    m.add_trait("Charge", "Causa dano de chifres extra ao investir 10ft.");
    m.add_action(MonsterAction::attack("Greataxe", create_dagger()));
    m
}

pub fn create_naga() -> Monster {
    let mut m = create_monstrosity_base("guardian_naga", "Guardian Naga", CreatureSize::Large, 10.0, 5900, 19, 18, 16, 16, 19, 18);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Rejuvenation", "Se for morta, retorna à vida em 1d6 dias.");
    m.add_action(MonsterAction::attack("Spit Poison", create_dagger()));
    m
}

pub fn create_owlbear() -> Monster {
    let mut m = create_monstrosity_base("owlbear", "Owlbear", CreatureSize::Large, 3.0, 700, 20, 12, 17, 3, 12, 7);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_action(MonsterAction::attack("Beak", create_dagger()));
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m
}

pub fn create_purple_worm() -> Monster {
    let mut m = create_monstrosity_base("purple_worm", "Purple Worm", CreatureSize::Gargantuan, 15.0, 13000, 28, 7, 22, 1, 8, 4);
    m.add_sense(MonsterSense::Blindsight { range_feet: 30, blind_beyond: false });
    m.add_sense(MonsterSense::Tremorsense { range_feet: 60 });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Stinger", create_dagger()));
    m
}

pub fn create_remorhaz() -> Monster {
    let mut m = create_monstrosity_base("remorhaz", "Remorhaz", CreatureSize::Huge, 11.0, 7200, 24, 13, 21, 4, 10, 5);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_sense(MonsterSense::Tremorsense { range_feet: 60 });
    m.resistances.damage_immunities.push(DamageType::Fire);
    m.resistances.damage_immunities.push(DamageType::Cold);
    m.add_trait("Heated Body", "Causa 3d6 de fogo a quem o tocar em corpo a corpo.");
    m
}

pub fn create_roc() -> Monster { create_monstrosity_base("roc", "Roc", CreatureSize::Gargantuan, 11.0, 7200, 28, 10, 20, 3, 10, 9) }

pub fn create_rust_monster() -> Monster {
    let mut m = create_monstrosity_base("rust_monster", "Rust Monster", CreatureSize::Medium, 0.5, 100, 13, 12, 13, 2, 13, 6);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_sense(MonsterSense::Tremorsense { range_feet: 60 });
    m.add_trait("Iron Scent", "Sente a localização de metais ferrous a 30ft.");
    m.add_trait("Rust Metal", "Armas de ferro que o atingirem corroem permanentemente.");
    m.add_action(MonsterAction::attack("Antennae", create_dagger()));
    m
}

pub fn create_sphinx() -> Monster {
    let mut m = create_monstrosity_base("androsphinx", "Androsphinx", CreatureSize::Large, 15.0, 13000, 22, 10, 20, 16, 18, 23);
    m.add_sense(MonsterSense::Truesight { range_feet: 120 });
    m.add_trait("Inscrutable", "Imune a qualquer efeito que leia seus pensamentos ou emoções.");
    m.add_action(MonsterAction::new("Roar", ActionType::Action, "Rugido avassalador que aterroriza ou atordoa inimigos (3/Day).", None));
    m.set_legendary(3);
    m
}

pub fn create_tarrasque() -> Monster {
    let mut m = create_monstrosity_base("tarrasque", "Tarrasque", CreatureSize::Gargantuan, 30.0, 155000, 30, 11, 30, 3, 11, 11);
    m.add_sense(MonsterSense::Blindsight { range_feet: 120, blind_beyond: false });
    m.resistances.damage_immunities.push(DamageType::Fire);
    m.resistances.damage_immunities.push(DamageType::Poison);
    m.resistances.nonmagical_physical_immunity = true;
    m.add_trait("Reflective Carapace", "Reflete magias de projétil como Magic Missile e Ranged Spell Attacks com rolar 1d6 (no 6 redireciona).");
    m.add_trait("Siege Monster", "Causa o dobro de dano a estruturas.");
    m.set_legendary(3);
    m
}

pub fn create_unicorn() -> Monster {
    let mut m = create_monstrosity_base("unicorn", "Unicorn", CreatureSize::Large, 5.0, 1800, 18, 14, 15, 11, 17, 16);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Magic Resistance", "Vantagem contra magias.");
    m.add_action(MonsterAction::attack("Horn", create_dagger()));
    m.set_legendary(3);
    m
}

pub fn create_yuan_ti_pureblood() -> Monster {
    let mut m = create_monstrosity_base("yuan_ti_pureblood", "Yuan-ti Pureblood", CreatureSize::Medium, 1.0, 200, 11, 12, 11, 13, 12, 14);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.resistances.damage_immunities.push(DamageType::Poison);
    m.add_trait("Magic Resistance", "Vantagem contra magias.");
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monstrosities_creation() {
        let tarrasque = create_tarrasque();
        assert_eq!(tarrasque.id, "tarrasque");
        assert_eq!(tarrasque.challenge_rating, 30.0);
        assert!(tarrasque.legendary.is_some());
    }
}
