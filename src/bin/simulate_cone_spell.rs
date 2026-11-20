use rpg_engine::dice::DiceMode;
use rpg_engine::map::grid::{BattleGrid, GridDirection, GridPosition, TerrainType};
use rpg_engine::rules::character::attributes::{Ability, AbilityScore, AbilityScores};
use rpg_engine::rules::character::background::Background;
use rpg_engine::rules::character::class::{ClassVariant, Wizard, Fighter};
use rpg_engine::rules::character::entity::Entity;
use rpg_engine::rules::character::race::{RaceVariant, Human};
use rpg_engine::rules::character::skills::Skill;
use rpg_engine::rules::combat::attack::DamageAffinity;
use rpg_engine::rules::spells::casting::cast_spell;
use rpg_engine::rules::spells::registry::SpellRegistry;
use rpg_engine::rules::spells::spellcaster::SpellSlots;

fn main() {
    println!("================================================================================");
    println!("     SIMULAÇÃO DE MAGIA EM CONE: BURNING HANDS (MÃOS FLAMEJANTES)");
    println!("                     SRD 5.1 + TABULEIRO TÁTICO");
    println!("================================================================================");

    // 1. O Conjurador: Mago Arcano Elric
    let mage_scores = AbilityScores {
        strength: AbilityScore(8),      // -1
        dexterity: AbilityScore(14),    // +2
        constitution: AbilityScore(13), // +1
        intelligence: AbilityScore(16), // +3 (Modificador de Conjuração)
        wisdom: AbilityScore(12),       // +1
        charisma: AbilityScore(10),     // 0
    };
    let mage_bg = Background::new("Sábio Arcano", vec![Skill::Arcana, Skill::History]);
    let mut mage = Entity::new(
        "Elric, o Mago de Guerra",
        mage_scores,
        ClassVariant::Wizard(Wizard),
        RaceVariant::Human(Human),
        mage_bg,
    );
    mage.base_max_hp = 18;
    mage.current_hp = 18;

    let mut slots = SpellSlots::new();
    slots.set_slots(1, 4); // 4 slots de nível 1
    mage.spell_slots = Some(slots);

    // DC de Salvaguarda do Mago: 8 + Proficiência(+2) + INT(+3) = DC 13
    let spellcasting_mod = mage.attributes.get_modifier(Ability::Intelligence) as i32;
    let spell_save_dc = 8 + mage.proficiency_bonus() + spellcasting_mod;

    // 2. Os Alvos dentro da área do Cone (Três Goblins Emboscadores)
    struct EnemyTarget {
        name: String,
        pos: GridPosition,
        entity: Entity,
        in_cone: bool,
    }

    let make_goblin = |name: &str, dex: u8, hp: i32, pos: GridPosition, in_cone: bool| -> EnemyTarget {
        let scores = AbilityScores {
            strength: AbilityScore(8),
            dexterity: AbilityScore(dex),
            constitution: AbilityScore(10),
            intelligence: AbilityScore(10),
            wisdom: AbilityScore(8),
            charisma: AbilityScore(8),
        };
        let bg = Background::new("Ladrão de Caverna", vec![Skill::Stealth]);
        let mut ent = Entity::new(
            name,
            scores,
            ClassVariant::Fighter(Fighter),
            RaceVariant::Human(Human),
            bg,
        );
        ent.base_max_hp = hp;
        ent.current_hp = hp;
        EnemyTarget {
            name: name.to_string(),
            pos,
            entity: ent,
            in_cone,
        }
    };

    let mut enemies = vec![
        make_goblin("Goblin Ágil #1", 16, 12, GridPosition::new(11, 10), true), // DES 16 (+3)
        make_goblin("Goblin Distraído #2", 10, 10, GridPosition::new(12, 11), true), // DES 10 (0)
        make_goblin("Goblin Lento #3", 8, 14, GridPosition::new(13, 9), true), // DES 8 (-1)
        make_goblin("Goblin Arqueiro #4 (Fora do Cone)", 14, 9, GridPosition::new(15, 15), false), // Fora da área
    ];

    // 3. Montar o Campo de Batalha e Trajetória do Cone
    let mut grid = BattleGrid::new(20, 20, "Caverna Subterrânea".to_string());
    let mage_pos = GridPosition::new(10, 10);
    let cone_direction = GridDirection::East;
    let cone_length_ft = 15; // Cone de 15 pés

    println!("\n📍 [POSICIONAMENTO TÁTICO]");
    println!("🧙 Conjurador: {} em {:?} (CD de Salvaguarda: {})", mage.name, mage_pos, spell_save_dc);
    println!("Direção da Conjuração: {:?} (Cone de {} pés se expandindo para o Leste)", cone_direction, cone_length_ft);
    println!("\nInimigos no Cenário:");
    for e in &enemies {
        let tag = if e.in_cone { "🔥 DENTRO DO CONE" } else { "🛡️ FORA DA ÁREA" };
        println!("   - {} em {:?} | PV: {} | Destreza: {} | {}", e.name, e.pos, e.entity.current_hp, e.entity.attributes.dexterity.0, tag);
    }

    // 4. Conjurando "Burning Hands"
    let spell = SpellRegistry::get_spell("Burning Hands").expect("Burning Hands deve estar registrada no SRD");
    println!("\n================================================================================");
    println!("💥 {} junta os polegares e projeta uma leque de labaredas com [Burning Hands]!", mage.name);
    println!("   Regra: 3d6 de Fogo em cone de 15ft. Salvaguarda de Destreza (CD {}) para metade do dano.", spell_save_dc);
    println!("================================================================================");

    // Marcação do terreno com chamas persistentes via BattleGrid
    let (_, cells_ignited) = grid.cast_directional_hazard(
        mage_pos,
        cone_direction,
        15,
        10,
        TerrainType::Hazard {
            save_dc: spell_save_dc as u8,
            damage_dice: "1d4".to_string(),
            condition: None,
        },
        "Brasas incandescentes e labaredas no chão de pedra",
    );
    println!("🔥 O solo de {} células do grid foi incendiado pelo sopro de chamas!", cells_ignited);

    // 5. Resolução da magia criatura por criatura dentro do Cone
    println!("\n[RESOLUÇÃO DE SALVAGUARDAS E DANO NOS ALVOS]:\n");

    for enemy in enemies.iter_mut() {
        if !enemy.in_cone {
            println!("⏩ {} está a mais de 15 pés da abertura do cone e não foi atingido.", enemy.name);
            continue;
        }

        let mut mage_clone = mage.clone();
        let outcome = cast_spell(
            &mut mage_clone,
            Some(&mut enemy.entity),
            &spell,
            Ability::Intelligence,
            false, // não proficiente em save DEX
            DiceMode::Normal,
            DamageAffinity::Normal,
        );

        match outcome.outcome {
            rpg_engine::rules::spells::casting::CastOutcome::SaveRoll { dc: _, save_result, raw_damage: _, final_damage } => {
                let d20 = save_result.roll_result.selected_roll;
                let mod_val = save_result.roll_result.modifier;
                let total = save_result.roll_result.total;
                let passed = save_result.is_success.unwrap_or(false);

                if passed {
                    println!(
                        "⚡ {}: Rola Salvaguarda DES d20({}) + {} = {} vs CD {} => ✅ PASSOU! (Sofre metade do dano)",
                        enemy.name, d20, mod_val, total, spell_save_dc
                    );
                    println!(
                        "   🩸 Dano Reduzido: {} PV de Fogo. PV Restante: {}/{}",
                        final_damage, enemy.entity.current_hp, enemy.entity.base_max_hp
                    );
                } else {
                    println!(
                        "💥 {}: Rola Salvaguarda DES d20({}) + {} = {} vs CD {} => ❌ FALHOU! (Engolido pelas chamas)",
                        enemy.name, d20, mod_val, total, spell_save_dc
                    );
                    let death_str = if enemy.entity.current_hp <= 0 { "💀 QUEIMADO ATÉ CINZAS!" } else { "🩸 FERIDO GRAVEMENTE!" };
                    println!(
                        "   🔥 Dano Total: {} PV de Fogo. PV Restante: {}/{} => {}",
                        final_damage, enemy.entity.current_hp, enemy.entity.base_max_hp, death_str
                    );
                }
            }
            other => println!("Resultado inesperado: {:?}", other),
        }
        println!("--------------------------------------------------------------------------------");
    }

    println!("\n[ESTADO FINAL DO CAMPO DE BATALHA]");
    for e in &enemies {
        let status = if e.entity.current_hp <= 0 { "💀 MORTO" } else { "⚔️ SOBREVIVEU" };
        println!("   - {} | PV: {}/{} | Status: {}", e.name, e.entity.current_hp, e.entity.base_max_hp, status);
    }

    println!("\n================================================================================");
    println!("                     SIMULAÇÃO CONCLUÍDA COM SUCESSO!");
    println!("================================================================================");
}
