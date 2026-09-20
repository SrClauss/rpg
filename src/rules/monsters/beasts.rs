use crate::rules::character::attributes::AbilityScores;
use crate::rules::character::background::create_acolyte;
use crate::rules::character::class::{ClassVariant, Fighter};
use crate::rules::character::entity::Entity;
use crate::rules::character::race::{Human, RaceVariant};
use crate::rules::equipment::weapons::create_dagger;
use crate::rules::monsters::monster::{CreatureSize, CreatureType, Monster, MonsterAction, MonsterSense};

fn create_beast_entity(name: &str, str_: u8, dex: u8, con: u8, int_: u8, wis: u8, cha: u8) -> Entity {
    Entity::new(
        name,
        AbilityScores::new(str_, dex, con, int_, wis, cha),
        ClassVariant::Fighter(Fighter),
        RaceVariant::Human(Human),
        create_acolyte(),
    )
}

pub fn create_ape() -> Monster {
    let mut m = Monster::new("ape", create_beast_entity("Ape", 16, 14, 14, 6, 12, 7), CreatureType::Beast, CreatureSize::Medium, 0.5, 100);
    m.add_action(MonsterAction::attack("Fist", create_dagger()));
    m.add_action(MonsterAction::attack("Rock", create_dagger()));
    m
}

pub fn create_baboon() -> Monster {
    let mut m = Monster::new("baboon", create_beast_entity("Baboon", 8, 14, 11, 4, 12, 6), CreatureType::Beast, CreatureSize::Small, 0.0, 10);
    m.add_trait("Pack Tactics", "Vantagem em ataques se um aliado estiver a 5ft do alvo.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_badger() -> Monster {
    let mut m = Monster::new("badger", create_beast_entity("Badger", 4, 11, 12, 2, 12, 5), CreatureType::Beast, CreatureSize::Tiny, 0.0, 10);
    m.add_sense(MonsterSense::Darkvision { range_feet: 30 });
    m.add_sense(MonsterSense::PassivePerception(11));
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_black_bear() -> Monster {
    let mut m = Monster::new("black_bear", create_beast_entity("Black Bear", 15, 10, 14, 2, 12, 7), CreatureType::Beast, CreatureSize::Medium, 0.5, 100);
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m
}

pub fn create_boar() -> Monster {
    let mut m = Monster::new("boar", create_beast_entity("Boar", 13, 11, 12, 2, 9, 5), CreatureType::Beast, CreatureSize::Medium, 0.25, 50);
    m.add_trait("Charge", "Se mover 20ft em linha reta antes de atacar com Tusk, causa 1d6 extra.");
    m.add_trait("Relentless", "Se sofrer 7 ou menos de dano que o reduziria a 0 HP, fica com 1 HP.");
    m.add_action(MonsterAction::attack("Tusk", create_dagger()));
    m
}

pub fn create_brown_bear() -> Monster {
    let mut m = Monster::new("brown_bear", create_beast_entity("Brown Bear", 19, 10, 16, 2, 13, 7), CreatureType::Beast, CreatureSize::Large, 1.0, 200);
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m
}

pub fn create_constrictor_snake() -> Monster {
    let mut m = Monster::new("constrictor_snake", create_beast_entity("Constrictor Snake", 15, 14, 12, 1, 10, 3), CreatureType::Beast, CreatureSize::Large, 0.25, 50);
    m.add_sense(MonsterSense::Blindsight { range_feet: 10, blind_beyond: true });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Constrict", create_dagger()));
    m
}

pub fn create_crocodile() -> Monster {
    let mut m = Monster::new("crocodile", create_beast_entity("Crocodile", 15, 10, 13, 2, 10, 5), CreatureType::Beast, CreatureSize::Large, 0.5, 100);
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_dire_wolf() -> Monster {
    let mut m = Monster::new("dire_wolf", create_beast_entity("Dire Wolf", 17, 15, 15, 3, 12, 7), CreatureType::Beast, CreatureSize::Large, 1.0, 200);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Pack Tactics", "Vantagem em jogadas de ataque se tiver aliado a 5ft do alvo.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_draft_horse() -> Monster {
    let mut m = Monster::new("draft_horse", create_beast_entity("Draft Horse", 18, 12, 13, 2, 11, 7), CreatureType::Beast, CreatureSize::Large, 0.25, 50);
    m.add_action(MonsterAction::attack("Hooves", create_dagger()));
    m
}

pub fn create_eagle() -> Monster {
    let mut m = Monster::new("eagle", create_beast_entity("Eagle", 6, 15, 10, 2, 14, 7), CreatureType::Beast, CreatureSize::Small, 0.0, 10);
    m.add_action(MonsterAction::attack("Talons", create_dagger()));
    m
}

pub fn create_elephant() -> Monster {
    let mut m = Monster::new("elephant", create_beast_entity("Elephant", 22, 9, 17, 3, 11, 6), CreatureType::Beast, CreatureSize::Huge, 4.0, 1100);
    m.add_action(MonsterAction::attack("Gore", create_dagger()));
    m.add_action(MonsterAction::attack("Stomp", create_dagger()));
    m
}

pub fn create_giant_ape() -> Monster {
    let mut m = Monster::new("giant_ape", create_beast_entity("Giant Ape", 23, 14, 18, 7, 12, 7), CreatureType::Beast, CreatureSize::Huge, 7.0, 2900);
    m.add_action(MonsterAction::attack("Fist", create_dagger()));
    m.add_action(MonsterAction::attack("Rock", create_dagger()));
    m
}

pub fn create_giant_badger() -> Monster {
    let mut m = Monster::new("giant_badger", create_beast_entity("Giant Badger", 13, 10, 15, 2, 12, 5), CreatureType::Beast, CreatureSize::Medium, 0.25, 50);
    m.add_sense(MonsterSense::Darkvision { range_feet: 30 });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m
}

pub fn create_giant_bat() -> Monster {
    let mut m = Monster::new("giant_bat", create_beast_entity("Giant Bat", 15, 16, 11, 2, 12, 6), CreatureType::Beast, CreatureSize::Large, 0.25, 50);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: true });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_giant_boar() -> Monster {
    let mut m = Monster::new("giant_boar", create_beast_entity("Giant Boar", 17, 10, 16, 2, 7, 5), CreatureType::Beast, CreatureSize::Large, 2.0, 450);
    m.add_trait("Relentless", "Se sofrer 10 ou menos dano reduzindo a 0 HP, fica com 1 HP.");
    m.add_action(MonsterAction::attack("Tusk", create_dagger()));
    m
}

pub fn create_giant_centipede() -> Monster {
    let mut m = Monster::new("giant_centipede", create_beast_entity("Giant Centipede", 5, 14, 12, 1, 7, 3), CreatureType::Beast, CreatureSize::Small, 0.125, 25);
    m.add_sense(MonsterSense::Blindsight { range_feet: 30, blind_beyond: true });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_giant_constrictor_snake() -> Monster {
    let mut m = Monster::new("giant_constrictor_snake", create_beast_entity("Giant Constrictor Snake", 19, 14, 12, 1, 10, 3), CreatureType::Beast, CreatureSize::Huge, 2.0, 450);
    m.add_sense(MonsterSense::Blindsight { range_feet: 10, blind_beyond: true });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Constrict", create_dagger()));
    m
}

pub fn create_giant_crab() -> Monster {
    let mut m = Monster::new("giant_crab", create_beast_entity("Giant Crab", 13, 15, 11, 1, 9, 3), CreatureType::Beast, CreatureSize::Medium, 0.125, 25);
    m.add_sense(MonsterSense::Blindsight { range_feet: 30, blind_beyond: true });
    m.add_action(MonsterAction::attack("Claw", create_dagger()));
    m
}

pub fn create_giant_crocodile() -> Monster {
    let mut m = Monster::new("giant_crocodile", create_beast_entity("Giant Crocodile", 21, 9, 17, 2, 10, 7), CreatureType::Beast, CreatureSize::Huge, 5.0, 1800);
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Tail", create_dagger()));
    m
}

pub fn create_giant_eagle() -> Monster {
    let mut m = Monster::new("giant_eagle", create_beast_entity("Giant Eagle", 16, 17, 13, 8, 14, 10), CreatureType::Beast, CreatureSize::Large, 1.0, 200);
    m.add_action(MonsterAction::attack("Beak", create_dagger()));
    m.add_action(MonsterAction::attack("Talons", create_dagger()));
    m
}

pub fn create_giant_elk() -> Monster {
    let mut m = Monster::new("giant_elk", create_beast_entity("Giant Elk", 19, 16, 14, 7, 14, 10), CreatureType::Beast, CreatureSize::Huge, 2.0, 450);
    m.add_action(MonsterAction::attack("Ram", create_dagger()));
    m.add_action(MonsterAction::attack("Hooves", create_dagger()));
    m
}

pub fn create_giant_fire_beetle() -> Monster {
    let mut m = Monster::new("giant_fire_beetle", create_beast_entity("Giant Fire Beetle", 8, 10, 12, 1, 7, 3), CreatureType::Beast, CreatureSize::Small, 0.0, 10);
    m.add_sense(MonsterSense::Blindsight { range_feet: 30, blind_beyond: true });
    m.add_trait("Illumination", "Emite luz brilhante num raio de 10ft e penumbra por mais 10ft.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_giant_frog() -> Monster {
    let mut m = Monster::new("giant_frog", create_beast_entity("Giant Frog", 12, 13, 11, 2, 10, 3), CreatureType::Beast, CreatureSize::Medium, 0.25, 50);
    m.add_sense(MonsterSense::Darkvision { range_feet: 30 });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_giant_goat() -> Monster {
    let mut m = Monster::new("giant_goat", create_beast_entity("Giant Goat", 17, 11, 14, 3, 12, 6), CreatureType::Beast, CreatureSize::Large, 0.5, 100);
    m.add_action(MonsterAction::attack("Ram", create_dagger()));
    m
}

pub fn create_giant_hyena() -> Monster {
    let mut m = Monster::new("giant_hyena", create_beast_entity("Giant Hyena", 16, 14, 14, 2, 12, 7), CreatureType::Beast, CreatureSize::Large, 1.0, 200);
    m.add_trait("Rampage", "Ao reduzir uma criatura a 0 HP, pode mover metade do deslocamento e fazer ataque de mordida como ação bônus.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_giant_lizard() -> Monster {
    let mut m = Monster::new("giant_lizard", create_beast_entity("Giant Lizard", 15, 12, 13, 2, 10, 5), CreatureType::Beast, CreatureSize::Large, 0.25, 50);
    m.add_sense(MonsterSense::Darkvision { range_feet: 30 });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_giant_octopus() -> Monster {
    let mut m = Monster::new("giant_octopus", create_beast_entity("Giant Octopus", 17, 13, 13, 4, 10, 4), CreatureType::Beast, CreatureSize::Large, 1.0, 200);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_action(MonsterAction::attack("Tentacles", create_dagger()));
    m
}

pub fn create_giant_owl() -> Monster {
    let mut m = Monster::new("giant_owl", create_beast_entity("Giant Owl", 13, 15, 12, 8, 13, 10), CreatureType::Beast, CreatureSize::Large, 0.25, 50);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.add_trait("Flyby", "Não provoca ataques de oportunidade ao voar fora do alcance de um inimigo.");
    m.add_action(MonsterAction::attack("Talons", create_dagger()));
    m
}

pub fn create_giant_poisonous_snake() -> Monster {
    let mut m = Monster::new("giant_poisonous_snake", create_beast_entity("Giant Poisonous Snake", 10, 18, 13, 2, 10, 3), CreatureType::Beast, CreatureSize::Medium, 0.25, 50);
    m.add_sense(MonsterSense::Blindsight { range_feet: 10, blind_beyond: true });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_giant_rat() -> Monster {
    let mut m = Monster::new("giant_rat", create_beast_entity("Giant Rat", 7, 15, 11, 2, 10, 4), CreatureType::Beast, CreatureSize::Small, 0.125, 25);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Pack Tactics", "Vantagem em ataques com aliado a 5ft do alvo.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_giant_raven() -> Monster {
    let mut m = Monster::new("giant_raven", create_beast_entity("Giant Raven", 6, 14, 10, 2, 13, 6), CreatureType::Beast, CreatureSize::Tiny, 0.0, 10);
    m.add_action(MonsterAction::attack("Beak", create_dagger()));
    m
}

pub fn create_giant_scorpion() -> Monster {
    let mut m = Monster::new("giant_scorpion", create_beast_entity("Giant Scorpion", 15, 13, 15, 1, 9, 3), CreatureType::Beast, CreatureSize::Large, 3.0, 700);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: true });
    m.add_action(MonsterAction::attack("Claw", create_dagger()));
    m.add_action(MonsterAction::attack("Stinger", create_dagger()));
    m
}

pub fn create_giant_shark() -> Monster {
    let mut m = Monster::new("giant_shark", create_beast_entity("Giant Shark", 23, 11, 21, 1, 10, 5), CreatureType::Beast, CreatureSize::Huge, 5.0, 1800);
    m.add_sense(MonsterSense::Blindsight { range_feet: 60, blind_beyond: true });
    m.add_trait("Blood Frenzy", "Vantagem em ataques contra criaturas que não estejam com HP cheio.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_giant_spider() -> Monster {
    let mut m = Monster::new("giant_spider", create_beast_entity("Giant Spider", 14, 16, 12, 2, 11, 4), CreatureType::Beast, CreatureSize::Large, 1.0, 200);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_sense(MonsterSense::PassivePerception(10));
    m.add_trait("Spider Climb", "Escala superfícies sem teste.");
    m.add_trait("Web Sense", "Sente vibrações na teia.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Web", create_dagger()));
    m
}

pub fn create_giant_toad() -> Monster {
    let mut m = Monster::new("giant_toad", create_beast_entity("Giant Toad", 15, 13, 13, 2, 10, 3), CreatureType::Beast, CreatureSize::Large, 1.0, 200);
    m.add_sense(MonsterSense::Darkvision { range_feet: 30 });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Swallow", create_dagger()));
    m
}

pub fn create_giant_vulture() -> Monster {
    let mut m = Monster::new("giant_vulture", create_beast_entity("Giant Vulture", 15, 12, 15, 6, 12, 7), CreatureType::Beast, CreatureSize::Large, 1.0, 200);
    m.add_trait("Pack Tactics", "Vantagem em ataques com aliado a 5ft do alvo.");
    m.add_action(MonsterAction::attack("Multiattack", create_dagger()));
    m
}

pub fn create_giant_wasp() -> Monster {
    let mut m = Monster::new("giant_wasp", create_beast_entity("Giant Wasp", 10, 14, 10, 1, 10, 3), CreatureType::Beast, CreatureSize::Medium, 0.5, 100);
    m.add_action(MonsterAction::attack("Sting", create_dagger()));
    m
}

pub fn create_giant_weasel() -> Monster {
    let mut m = Monster::new("giant_weasel", create_beast_entity("Giant Weasel", 11, 16, 10, 4, 12, 5), CreatureType::Beast, CreatureSize::Medium, 0.125, 25);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_giant_wolf_spider() -> Monster {
    let mut m = Monster::new("giant_wolf_spider", create_beast_entity("Giant Wolf Spider", 12, 16, 13, 3, 12, 4), CreatureType::Beast, CreatureSize::Medium, 0.25, 50);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Spider Climb", "Escala superfícies sem teste.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_goat() -> Monster {
    let mut m = Monster::new("goat", create_beast_entity("Goat", 12, 10, 11, 2, 10, 5), CreatureType::Beast, CreatureSize::Medium, 0.0, 10);
    m.add_action(MonsterAction::attack("Ram", create_dagger()));
    m
}

pub fn create_hawk() -> Monster {
    let mut m = Monster::new("hawk", create_beast_entity("Hawk", 5, 16, 8, 2, 14, 6), CreatureType::Beast, CreatureSize::Tiny, 0.0, 10);
    m.add_action(MonsterAction::attack("Talons", create_dagger()));
    m
}

pub fn create_hunter_shark() -> Monster {
    let mut m = Monster::new("hunter_shark", create_beast_entity("Hunter Shark", 18, 13, 15, 1, 10, 4), CreatureType::Beast, CreatureSize::Large, 2.0, 450);
    m.add_sense(MonsterSense::Blindsight { range_feet: 30, blind_beyond: true });
    m.add_trait("Blood Frenzy", "Vantagem em ataques contra alvos feridos.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_hyena() -> Monster {
    let mut m = Monster::new("hyena", create_beast_entity("Hyena", 11, 12, 11, 2, 12, 5), CreatureType::Beast, CreatureSize::Medium, 0.0, 10);
    m.add_trait("Pack Tactics", "Vantagem em ataques com aliado a 5ft do alvo.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_jackal() -> Monster {
    let mut m = Monster::new("jackal", create_beast_entity("Jackal", 8, 15, 11, 3, 12, 6), CreatureType::Beast, CreatureSize::Small, 0.0, 10);
    m.add_trait("Pack Tactics", "Vantagem em ataques com aliado a 5ft do alvo.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_killer_whale() -> Monster {
    let mut m = Monster::new("killer_whale", create_beast_entity("Killer Whale", 19, 10, 13, 3, 12, 7), CreatureType::Beast, CreatureSize::Huge, 3.0, 700);
    m.add_sense(MonsterSense::Blindsight { range_feet: 120, blind_beyond: true });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_lion() -> Monster {
    let mut m = Monster::new("lion", create_beast_entity("Lion", 17, 15, 13, 3, 12, 8), CreatureType::Beast, CreatureSize::Large, 1.0, 200);
    m.add_trait("Pack Tactics", "Vantagem em ataques com aliado a 5ft.");
    m.add_trait("Pounce", "Se mover 20ft antes de atacar com Claws, derruba Prone.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m
}

pub fn create_lizard() -> Monster {
    let mut m = Monster::new("lizard", create_beast_entity("Lizard", 2, 11, 10, 1, 8, 3), CreatureType::Beast, CreatureSize::Tiny, 0.0, 10);
    m.add_sense(MonsterSense::Darkvision { range_feet: 30 });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_mule() -> Monster {
    let mut m = Monster::new("mule", create_beast_entity("Mule", 14, 10, 13, 2, 10, 5), CreatureType::Beast, CreatureSize::Medium, 0.125, 25);
    m.add_action(MonsterAction::attack("Hooves", create_dagger()));
    m
}

pub fn create_octopus() -> Monster {
    let mut m = Monster::new("octopus", create_beast_entity("Octopus", 4, 15, 11, 3, 10, 4), CreatureType::Beast, CreatureSize::Small, 0.0, 10);
    m.add_sense(MonsterSense::Darkvision { range_feet: 30 });
    m.add_action(MonsterAction::attack("Tentacles", create_dagger()));
    m
}

pub fn create_owl() -> Monster {
    let mut m = Monster::new("owl", create_beast_entity("Owl", 3, 13, 8, 2, 12, 7), CreatureType::Beast, CreatureSize::Tiny, 0.0, 10);
    m.add_sense(MonsterSense::Darkvision { range_feet: 120 });
    m.add_trait("Flyby", "Não provoca ataque de oportunidade ao voar fora do alcance.");
    m.add_action(MonsterAction::attack("Talons", create_dagger()));
    m
}

pub fn create_panther() -> Monster {
    let mut m = Monster::new("panther", create_beast_entity("Panther", 14, 15, 10, 3, 14, 7), CreatureType::Beast, CreatureSize::Medium, 0.25, 50);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Pounce", "Se mover 20ft antes de atacar, derruba Prone.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m
}

pub fn create_phase_spider() -> Monster {
    let mut m = Monster::new("phase_spider", create_beast_entity("Phase Spider", 15, 15, 12, 6, 10, 6), CreatureType::Beast, CreatureSize::Large, 3.0, 700);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Ethereal Jaunt", "Ação Bônus para se mover para o Plano Etéreo.");
    m.add_trait("Spider Climb", "Escala superfícies sem teste.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_plesiosaurus() -> Monster {
    let mut m = Monster::new("plesiosaurus", create_beast_entity("Plesiosaurus", 18, 15, 16, 2, 12, 5), CreatureType::Beast, CreatureSize::Large, 2.0, 450);
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_poisonous_snake() -> Monster {
    let mut m = Monster::new("poisonous_snake", create_beast_entity("Poisonous Snake", 2, 16, 11, 1, 10, 3), CreatureType::Beast, CreatureSize::Tiny, 0.125, 25);
    m.add_sense(MonsterSense::Blindsight { range_feet: 10, blind_beyond: true });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_pony() -> Monster {
    let mut m = Monster::new("pony", create_beast_entity("Pony", 15, 10, 13, 2, 11, 7), CreatureType::Beast, CreatureSize::Medium, 0.125, 25);
    m.add_action(MonsterAction::attack("Hooves", create_dagger()));
    m
}

pub fn create_reef_shark() -> Monster {
    let mut m = Monster::new("reef_shark", create_beast_entity("Reef Shark", 14, 13, 13, 1, 10, 4), CreatureType::Beast, CreatureSize::Medium, 0.5, 100);
    m.add_sense(MonsterSense::Blindsight { range_feet: 30, blind_beyond: true });
    m.add_trait("Pack Tactics", "Vantagem em ataques com aliado a 5ft.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_rhinoceros() -> Monster {
    let mut m = Monster::new("rhinoceros", create_beast_entity("Rhinoceros", 21, 8, 15, 2, 12, 6), CreatureType::Beast, CreatureSize::Large, 2.0, 450);
    m.add_action(MonsterAction::attack("Gore", create_dagger()));
    m
}

pub fn create_riding_horse() -> Monster {
    let mut m = Monster::new("riding_horse", create_beast_entity("Riding Horse", 16, 10, 12, 2, 11, 7), CreatureType::Beast, CreatureSize::Large, 0.25, 50);
    m.add_action(MonsterAction::attack("Hooves", create_dagger()));
    m
}

pub fn create_saber_toothed_tiger() -> Monster {
    let mut m = Monster::new("saber_toothed_tiger", create_beast_entity("Saber-Toothed Tiger", 18, 14, 15, 3, 12, 8), CreatureType::Beast, CreatureSize::Large, 2.0, 450);
    m.add_sense(MonsterSense::PassivePerception(13));
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m
}

pub fn create_scorpion() -> Monster {
    let mut m = Monster::new("scorpion", create_beast_entity("Scorpion", 2, 11, 10, 1, 8, 2), CreatureType::Beast, CreatureSize::Tiny, 0.0, 10);
    m.add_sense(MonsterSense::Blindsight { range_feet: 10, blind_beyond: true });
    m.add_action(MonsterAction::attack("Stinger", create_dagger()));
    m
}

pub fn create_sea_horse() -> Monster {
    let mut m = Monster::new("sea_horse", create_beast_entity("Sea Horse", 1, 12, 8, 1, 10, 2), CreatureType::Beast, CreatureSize::Tiny, 0.0, 0);
    m.add_trait("Water Breathing", "Pode respirar apenas em baixo d'água.");
    m
}

pub fn create_spider() -> Monster {
    let mut m = Monster::new("spider", create_beast_entity("Spider", 2, 14, 8, 1, 10, 2), CreatureType::Beast, CreatureSize::Tiny, 0.0, 10);
    m.add_sense(MonsterSense::Darkvision { range_feet: 30 });
    m.add_trait("Spider Climb", "Escala superfícies sem teste.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_tiger() -> Monster {
    let mut m = Monster::new("tiger", create_beast_entity("Tiger", 17, 15, 14, 3, 12, 8), CreatureType::Beast, CreatureSize::Large, 1.0, 200);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Claws", create_dagger()));
    m
}

pub fn create_tyrannosaurus_rex() -> Monster {
    let mut m = Monster::new("tyrannosaurus_rex", create_beast_entity("Tyrannosaurus Rex", 25, 10, 19, 2, 12, 9), CreatureType::Beast, CreatureSize::Huge, 8.0, 3900);
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m.add_action(MonsterAction::attack("Tail", create_dagger()));
    m
}

pub fn create_vulture() -> Monster {
    let mut m = Monster::new("vulture", create_beast_entity("Vulture", 7, 10, 13, 2, 12, 4), CreatureType::Beast, CreatureSize::Medium, 0.0, 10);
    m.add_trait("Pack Tactics", "Vantagem em ataques com aliado a 5ft.");
    m.add_action(MonsterAction::attack("Beak", create_dagger()));
    m
}

pub fn create_warhorse() -> Monster {
    let mut m = Monster::new("warhorse", create_beast_entity("Warhorse", 18, 12, 13, 2, 12, 7), CreatureType::Beast, CreatureSize::Large, 0.5, 100);
    m.add_action(MonsterAction::attack("Hooves", create_dagger()));
    m
}

pub fn create_weasel() -> Monster {
    let mut m = Monster::new("weasel", create_beast_entity("Weasel", 3, 16, 8, 2, 12, 3), CreatureType::Beast, CreatureSize::Tiny, 0.0, 10);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

pub fn create_wolf() -> Monster {
    let mut m = Monster::new("wolf", create_beast_entity("Wolf", 12, 15, 12, 3, 12, 6), CreatureType::Beast, CreatureSize::Medium, 0.25, 50);
    m.add_sense(MonsterSense::Darkvision { range_feet: 60 });
    m.add_trait("Pack Tactics", "Vantagem em jogadas de ataque se tiver aliado a 5ft do alvo.");
    m.add_action(MonsterAction::attack("Bite", create_dagger()));
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beasts_creation() {
        let ape = create_ape();
        assert_eq!(ape.id, "ape");

        let wolf = create_wolf();
        assert_eq!(wolf.id, "wolf");

        let trex = create_tyrannosaurus_rex();
        assert_eq!(trex.id, "tyrannosaurus_rex");
        assert_eq!(trex.challenge_rating, 8.0);
    }
}
