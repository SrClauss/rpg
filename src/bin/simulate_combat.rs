use rpg_engine::dice::DiceMode;
use rpg_engine::rules::character::attributes::{Ability, AbilityScore, AbilityScores};
use rpg_engine::rules::character::entity::Entity;
use rpg_engine::rules::character::race::{RaceVariant, Human};
use rpg_engine::rules::character::class::{ClassVariant, Paladin, Wizard};
use rpg_engine::rules::character::background::Background;
use rpg_engine::rules::character::skills::Skill;
use rpg_engine::rules::combat::attack::{resolve_attack, DamageAffinity};
use rpg_engine::rules::equipment::registry::EquipmentRegistry;
use rpg_engine::rules::equipment::armor::{create_plate_armor, create_leather_armor, create_shield};
use rpg_engine::rules::spells::registry::SpellRegistry;
use rpg_engine::rules::spells::spellcaster::SpellSlots;
use rpg_engine::rules::spells::casting::cast_spell;

fn main() {
    println!("================================================================================");
    println!("   DUELO ÉPICO D&D 5E: PALADINO SAGRADO vs OGRO XAMÃ (OGRE MAGE / ONI)");
    println!("================================================================================");

    // 1. Criar o Herói: Valerius (Paladino Humano Nível 3)
    let paladin_scores = AbilityScores {
        strength: AbilityScore(16),     // Mod +3
        dexterity: AbilityScore(10),    // Mod 0
        constitution: AbilityScore(14), // Mod +2
        intelligence: AbilityScore(10), // Mod 0
        wisdom: AbilityScore(12),       // Mod +1
        charisma: AbilityScore(16),     // Mod +3
    };
    let paladin_bg = Background::new("Nobre", vec![Skill::Athletics, Skill::Persuasion]);
    let mut hero = Entity::new(
        "Valerius, o Paladino",
        paladin_scores,
        ClassVariant::Paladin(Paladin),
        RaceVariant::Human(Human),
        paladin_bg,
    );
    hero.base_max_hp = 35;
    hero.current_hp = 35;
    hero.equipped_armor = Some(create_plate_armor()); // CA 18
    hero.equipped_shield = Some(create_shield());     // +2 -> CA 20

    let mut hero_slots = SpellSlots::new();
    hero_slots.set_slots(1, 3);
    hero.spell_slots = Some(hero_slots);

    // 2. Criar o Monstro: Kor'gath, o Ogro Xamã (Ogre Mage / Feiticeiro da Tribo)
    let ogre_scores = AbilityScores {
        strength: AbilityScore(18),     // Mod +4
        dexterity: AbilityScore(12),    // Mod +1
        constitution: AbilityScore(16), // Mod +3
        intelligence: AbilityScore(14), // Mod +2 (Capaz de conjuração arcana)
        wisdom: AbilityScore(12),       // Mod +1
        charisma: AbilityScore(14),     // Mod +2
    };
    let ogre_bg = Background::new("Xamã de Sangue", vec![Skill::Arcana, Skill::Intimidation]);
    let mut monster = Entity::new(
        "Kor'gath, o Ogro Xamã",
        ogre_scores,
        ClassVariant::Wizard(Wizard),
        RaceVariant::Human(Human),
        ogre_bg,
    );
    monster.base_max_hp = 45;
    monster.current_hp = 45;
    monster.equipped_armor = Some(create_leather_armor()); // CA 12

    // Concede slots de magia de 1º e 2º Círculo ao Ogro Xamã
    let mut ogre_slots = SpellSlots::new();
    ogre_slots.set_slots(1, 4); // 4 slots de nível 1 (Magic Missile, Burning Hands)
    ogre_slots.set_slots(2, 2); // 2 slots de nível 2 (Scorching Ray)
    monster.spell_slots = Some(ogre_slots);

    // 3. Armas
    let longsword = EquipmentRegistry::get_weapon("longsword").expect("Longsword deve existir");
    let greatclub = EquipmentRegistry::get_weapon("greatclub").expect("Greatclub deve existir");

    println!("\n[ESTADO INICIAL DOS COMBATENTES]");
    println!("🛡️  HERÓI:   {} | PV: {}/{} | CA: {} | Slots: 3x Nível 1 (Cure Wounds)", hero.name, hero.current_hp, hero.base_max_hp, hero.get_armor_class());
    println!("👹 INIMIGO: {} | PV: {}/{} | CA: {} | Slots: 4x Lvl 1, 2x Lvl 2 (Magias Ofensivas)", monster.name, monster.current_hp, monster.base_max_hp, monster.get_armor_class());
    println!("--------------------------------------------------------------------------------");

    let mut round = 1;

    while hero.current_hp > 0 && monster.current_hp > 0 && round <= 12 {
        println!("\n=======================================================");
        println!("                    RODADA {}", round);
        println!("=======================================================");

        // --- Turno do Herói ---
        println!("\n[TURNO DO HERÓI: {}]", hero.name);

        let has_hero_slot = hero.spell_slots.as_ref().map(|s| s.has_slot(1)).unwrap_or(false);
        if hero.current_hp <= 15 && has_hero_slot {
            println!("✨ {} percebe o perigo arcano e conjura [Cure Wounds] em si mesmo!", hero.name);
            let cure_spell = SpellRegistry::get_spell("cure_wounds").expect("Cure Wounds deve existir");
            
            let mut caster_clone = hero.clone();
            let outcome = cast_spell(
                &mut caster_clone,
                Some(&mut hero),
                &cure_spell,
                Ability::Charisma,
                false,
                DiceMode::Normal,
                DamageAffinity::Normal,
            );
            hero.spell_slots = caster_clone.spell_slots;
            println!("   🌿 Efeito da Magia: {:?} -> PV restaurados! Atual: {}/{}", outcome.outcome, hero.current_hp, hero.base_max_hp);
        } else {
            println!("⚔️  {} avança golpeando com a {}", hero.name, longsword.name);
            let attack_res = resolve_attack(
                &mut hero,
                &mut monster,
                &longsword,
                true,
                DiceMode::Normal,
                DamageAffinity::Normal,
            );

            let roll = attack_res.attack_roll.selected_roll;
            let total = attack_res.attack_roll.total;
            let hit_str = if attack_res.is_critical_hit {
                "💥 ACERTO CRÍTICO NATURAL 20!"
            } else if attack_res.is_hit {
                "✅ ACERTOU!"
            } else {
                "❌ ERROU!"
            };

            println!("   🎲 Rolagem: d20({}) + Mod({}) = {} vs CA {} -> {}", roll, attack_res.attack_roll.modifier, total, attack_res.target_ac, hit_str);
            if attack_res.is_hit {
                monster.current_hp = (monster.current_hp - attack_res.final_damage).max(0);
                println!("   🩸 Dano Infligido: {} pontos cortantes! PV restante do Ogro: {}/{}", attack_res.final_damage, monster.current_hp, monster.base_max_hp);
            }
        }

        if monster.current_hp <= 0 {
            println!("\n🏆 O Ogro Xamã não suporta a fúria sagrada e cai morto! VITÓRIA DO HERÓI!");
            break;
        }

        // --- Turno do Ogro Xamã (IA Tática de Magia vs Ataque Físico) ---
        println!("\n[TURNO DO MONSTRO: {}]", monster.name);

        let has_lvl2 = monster.spell_slots.as_ref().map(|s| s.has_slot(2)).unwrap_or(false);
        let has_lvl1 = monster.spell_slots.as_ref().map(|s| s.has_slot(1)).unwrap_or(false);

        if has_lvl2 && round <= 2 {
            // Usa magia de 2º nível potente (Scorching Ray)
            println!("🔥 {} entoa um cântico gutural de sangue e conjura [Scorching Ray] (Raio Ardente)!", monster.name);
            let ray_spell = SpellRegistry::get_spell("scorching_ray").unwrap_or_else(|| {
                SpellRegistry::get_spell("magic_missile").expect("Magic Missile existe")
            });

            let mut caster_clone = monster.clone();
            let outcome = cast_spell(
                &mut caster_clone,
                Some(&mut hero),
                &ray_spell,
                Ability::Intelligence,
                false,
                DiceMode::Normal,
                DamageAffinity::Normal,
            );
            monster.spell_slots = caster_clone.spell_slots;
            println!("   💥 Efeito Arcano: {:?}! PV restante do Herói: {}/{}", outcome.outcome, hero.current_hp, hero.base_max_hp);
        } else if has_lvl1 && (round == 3 || round == 4) {
            // Usa projéteis mágicos que não erram (Magic Missile)
            println!("🔮 {} canaliza energia arcana e dispara [Magic Missile] (Mísseis Mágicos infalíveis)!", monster.name);
            let mm_spell = SpellRegistry::get_spell("magic_missile").expect("Magic Missile existe");

            let mut caster_clone = monster.clone();
            let outcome = cast_spell(
                &mut caster_clone,
                Some(&mut hero),
                &mm_spell,
                Ability::Intelligence,
                false,
                DiceMode::Normal,
                DamageAffinity::Normal,
            );
            monster.spell_slots = caster_clone.spell_slots;
            println!("   ⚡ Dano Mágico Inevitável: {:?}! PV restante do Herói: {}/{}", outcome.outcome, hero.current_hp, hero.base_max_hp);
        } else {
            // Se acabar os slots ou preferir corpo-a-corpo, golpeia com a Clava
            println!("👹 {} balança sua pesada {} contra {}", monster.name, greatclub.name, hero.name);

            let ogre_atk = resolve_attack(
                &mut monster,
                &mut hero,
                &greatclub,
                true,
                DiceMode::Normal,
                DamageAffinity::Normal,
            );

            let ogre_roll = ogre_atk.attack_roll.selected_roll;
            let ogre_total = ogre_atk.attack_roll.total;
            let ogre_hit_str = if ogre_atk.is_critical_hit {
                "💥 ACERTO CRÍTICO!"
            } else if ogre_atk.is_hit {
                "✅ ACERTOU!"
            } else {
                "🛡️ BLOQUEADO PELA ARMADURA DE PLACAS!"
            };

            println!("   🎲 Rolagem: d20({}) + Mod({}) = {} vs CA {} -> {}", ogre_roll, ogre_atk.attack_roll.modifier, ogre_total, ogre_atk.target_ac, ogre_hit_str);
            if ogre_atk.is_hit {
                hero.current_hp = (hero.current_hp - ogre_atk.final_damage).max(0);
                println!("   🩸 Dano Sofrido: {} pontos de contusão! PV restante do Herói: {}/{}", ogre_atk.final_damage, hero.current_hp, hero.base_max_hp);
            }
        }

        if hero.current_hp <= 0 {
            println!("\n💀 {} foi destruído pela magia ancestral e força do Ogro Xamã! O Herói sucumbiu!", hero.name);
            break;
        }

        round += 1;
    }

    println!("\n================================================================================");
    println!("                           FIM DO DUELO");
    println!("================================================================================");
}
