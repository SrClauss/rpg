use crate::dice::DiceMode;
use crate::narrative::entity_store::EntityStore;
use crate::narrative::living_scene::LivingScene;
use crate::rules::character::attributes::{Ability, AbilityScore, AbilityScores};
use crate::rules::character::background::Background;
use crate::rules::character::class::ClassVariant;
use crate::rules::character::entity::Entity;
use crate::rules::character::race::RaceVariant;
use crate::rules::character::skills::Skill;
use crate::rules::checks::{roll_ability_check, roll_skill_check, CheckResult};
use crate::rules::combat::attack::{resolve_attack, AttackResult, DamageAffinity};
use crate::rules::equipment::registry::EquipmentRegistry;
use crate::state::CampaignState;
use rand::Rng;
use serde_json::{json, Value};

/// Interpretador e Executor Determinístico de Regras D&D 5e
pub struct RulesResolver;

impl RulesResolver {
    /// Auxiliar para criar entidade base de teste/representação
    pub fn create_default_entity(name: &str) -> Entity {
        let scores = AbilityScores {
            strength: AbilityScore(14),
            dexterity: AbilityScore(14),
            constitution: AbilityScore(14),
            intelligence: AbilityScore(12),
            wisdom: AbilityScore(12),
            charisma: AbilityScore(14),
        };

        let background = Background::new("Criminal", vec![Skill::Deception, Skill::Stealth]);

        Entity::new(
            name,
            scores,
            ClassVariant::Rogue(crate::rules::character::class::Rogue),
            RaceVariant::Human(crate::rules::character::race::Human),
            background,
        )
    }

    pub fn create_entity_from_store(name: &str, actor_id: Option<&str>, store: Option<&EntityStore>) -> Entity {
        if let Some(st) = store {
            if let Ok(players) = st.list_player_entities() {
                if let Some(sheet) = players.iter().find(|p| {
                    actor_id.map(|id| p.id == id).unwrap_or(false)
                        || p.name.eq_ignore_ascii_case(name)
                }) {
                    if let Some(ref dnd) = sheet.dnd_stats {
                        let str_v = *dnd.ability_scores.get("STR").unwrap_or(&14) as u8;
                        let dex_v = *dnd.ability_scores.get("DEX").unwrap_or(&14) as u8;
                        let con_v = *dnd.ability_scores.get("CON").unwrap_or(&14) as u8;
                        let int_v = *dnd.ability_scores.get("INT").unwrap_or(&12) as u8;
                        let wis_v = *dnd.ability_scores.get("WIS").unwrap_or(&12) as u8;
                        let cha_v = *dnd.ability_scores.get("CHA").unwrap_or(&14) as u8;

                        let scores = AbilityScores {
                            strength: AbilityScore(str_v),
                            dexterity: AbilityScore(dex_v),
                            constitution: AbilityScore(con_v),
                            intelligence: AbilityScore(int_v),
                            wisdom: AbilityScore(wis_v),
                            charisma: AbilityScore(cha_v),
                        };
                        let background = Background::new("Aventureiro", vec![Skill::Perception, Skill::Athletics]);
                        let mut ent = Entity::new(
                            &sheet.name,
                            scores,
                            ClassVariant::Fighter(crate::rules::character::class::Fighter),
                            RaceVariant::Human(crate::rules::character::race::Human),
                            background,
                        );
                        ent.base_max_hp = dnd.max_hp.max(10);
                        ent.current_hp = dnd.current_hp.max(1);
                        return ent;
                    }
                }
            }
        }
        Self::create_default_entity(name)
    }

    /// Resolve uma intenção mecânica estruturada (gerada pela IA ou recebida via #regras)
    pub fn resolve(payload: &Value, state: &mut CampaignState) -> Value {
        Self::resolve_with_scene(payload, state, None, None)
    }

    /// Resolve uma intenção mecânica operando diretamente sobre a `LivingScene` e o `EntityStore`
    pub fn resolve_with_scene(
        payload: &Value,
        state: &mut CampaignState,
        scene: Option<&mut LivingScene>,
        entity_store: Option<&EntityStore>,
    ) -> Value {
        let action = payload["action"].as_str().unwrap_or("unknown");

        match action {
            "skill_check" => Self::resolve_skill_check(payload, state, entity_store),
            "ability_check" => Self::resolve_ability_check(payload, state, entity_store),
            "attack" => Self::resolve_attack_with_scene(payload, state, scene, entity_store),
            _ => json!({
                "status": "error",
                "message": format!("Ação mecânica '{}' não reconhecida", action)
            }),
        }
    }

    fn resolve_skill_check(
        payload: &Value,
        state: &mut CampaignState,
        entity_store: Option<&EntityStore>,
    ) -> Value {
        let skill_str = payload["skill"].as_str().unwrap_or("Persuasion");
        let dc = payload["dc"].as_i64().map(|v| v as i32);
        let actor_name = payload["actor"].as_str().unwrap_or("Jogador");
        let actor_id = payload["actor_id"].as_str();
        let is_proficient = payload["is_proficient"].as_bool().unwrap_or(true);

        let skill = match skill_str.to_lowercase().as_str() {
            "athletics" | "atletismo" => Skill::Athletics,
            "acrobatics" | "acrobacia" => Skill::Acrobatics,
            "sleightofhand" | "sleight_of_hand" | "prestidigitação" => Skill::SleightOfHand,
            "stealth" | "furtividade" => Skill::Stealth,
            "arcana" | "arcanismo" => Skill::Arcana,
            "history" | "história" => Skill::History,
            "investigation" | "investigação" => Skill::Investigation,
            "nature" | "natureza" => Skill::Nature,
            "religion" | "religião" => Skill::Religion,
            "animalhandling" | "animal_handling" | "adestrar animais" => Skill::AnimalHandling,
            "insight" | "intuição" => Skill::Insight,
            "medicine" | "medicina" => Skill::Medicine,
            "perception" | "percepção" => Skill::Perception,
            "survival" | "sobrevivência" => Skill::Survival,
            "deception" | "enganação" => Skill::Deception,
            "intimidation" | "intimidação" => Skill::Intimidation,
            "performance" | "atuação" => Skill::Performance,
            _ => Skill::Persuasion,
        };

        let fallback_store = state.get_entity_store().ok();
        let effective_store = entity_store.or(fallback_store.as_ref());
        let entity = Self::create_entity_from_store(actor_name, actor_id, effective_store);
        let check: CheckResult = roll_skill_check(&entity, skill, is_proficient, DiceMode::Normal, dc);

        json!({
            "status": "success",
            "action": "skill_check",
            "actor": actor_name,
            "skill": format!("{:?}", skill),
            "d20_roll": check.roll_result.selected_roll,
            "modifier": check.roll_result.modifier,
            "total": check.roll_result.total,
            "dc": dc,
            "is_success": check.is_success.unwrap_or(true),
            "is_critical": check.roll_result.is_critical_hit
        })
    }

    fn resolve_ability_check(
        payload: &Value,
        state: &mut CampaignState,
        entity_store: Option<&EntityStore>,
    ) -> Value {
        let ability_str = payload["ability"].as_str().unwrap_or("Strength");
        let dc = payload["dc"].as_i64().map(|v| v as i32);
        let actor_name = payload["actor"].as_str().unwrap_or("Jogador");
        let actor_id = payload["actor_id"].as_str();

        let ability = match ability_str.to_lowercase().as_str() {
            "dexterity" | "dex" | "destreza" => Ability::Dexterity,
            "constitution" | "con" | "constituição" => Ability::Constitution,
            "intelligence" | "int" | "inteligência" => Ability::Intelligence,
            "wisdom" | "wis" | "sabedoria" => Ability::Wisdom,
            "charisma" | "cha" | "carisma" => Ability::Charisma,
            _ => Ability::Strength,
        };

        let fallback_store = state.get_entity_store().ok();
        let effective_store = entity_store.or(fallback_store.as_ref());
        let entity = Self::create_entity_from_store(actor_name, actor_id, effective_store);
        let check = roll_ability_check(&entity, ability, DiceMode::Normal, dc);

        json!({
            "status": "success",
            "action": "ability_check",
            "actor": actor_name,
            "ability": format!("{:?}", ability),
            "d20_roll": check.roll_result.selected_roll,
            "modifier": check.roll_result.modifier,
            "total": check.roll_result.total,
            "dc": dc,
            "is_success": check.is_success.unwrap_or(true)
        })
    }

    pub fn resolve_attack_with_scene(
        payload: &Value,
        state: &mut CampaignState,
        scene: Option<&mut LivingScene>,
        entity_store: Option<&EntityStore>,
    ) -> Value {
        let attacker_name = payload["actor"].as_str().unwrap_or("Atacante");
        let attacker_id = payload["actor_id"].as_str();
        let requested_target = payload["target"].as_str().unwrap_or("");
        let is_spell = payload["is_spell"].as_bool().unwrap_or(false);

        let fallback_store = state.get_entity_store().ok();
        let effective_store = entity_store.or(fallback_store.as_ref());

        // Obtém bônus real de atributo do jogador no EntityStore
        let mut best_stat_mod: i32 = 3;
        let mut prof_bonus: i32 = 2;
        if let Some(st) = effective_store {
            if let Ok(players) = st.list_player_entities() {
                if let Some(sheet) = players.iter().find(|p| {
                    attacker_id.map(|id| p.id == id).unwrap_or(false)
                        || p.name.eq_ignore_ascii_case(attacker_name)
                }) {
                    if let Some(ref dnd) = sheet.dnd_stats {
                        let str_s = *dnd.ability_scores.get("STR").unwrap_or(&14);
                        let dex_s = *dnd.ability_scores.get("DEX").unwrap_or(&14);
                        let int_s = *dnd.ability_scores.get("INT").unwrap_or(&12);
                        let wis_s = *dnd.ability_scores.get("WIS").unwrap_or(&12);
                        let cha_s = *dnd.ability_scores.get("CHA").unwrap_or(&12);
                        let max_score = *[str_s, dex_s, int_s, wis_s, cha_s].iter().max().unwrap_or(&16);
                        best_stat_mod = (max_score - 10).div_euclid(2).max(2);
                    }
                    if let Some(p_bon) = sheet
                        .custom_flags
                        .get("full_sheet")
                        .and_then(|fs| fs.get("proficiencyBonus"))
                        .and_then(|v| v.as_i64())
                    {
                        prof_bonus = (p_bon as i32).max(2);
                    }
                }
            }
        }
        let atk_mod = best_stat_mod + prof_bonus;

        // Tenta usar a LivingScene passada por parâmetro ou a de `state.active_scene`
        let active_scene_opt: Option<&mut LivingScene> = match scene {
            Some(s) => Some(s),
            None => state.active_scene.as_mut(),
        };

        if let Some(sc) = active_scene_opt {
            let attacker_pos = sc
                .participants
                .iter()
                .find(|p| {
                    attacker_id.map(|id| p.id == id).unwrap_or(false)
                        || p.name.eq_ignore_ascii_case(attacker_name)
                })
                .and_then(|p| p.position)
                .unwrap_or((10, 20));

            let req_lower = requested_target.trim().to_lowercase();
            // 1. Procura inimigo hostil vivo especificado pelo nome/ID/#número
            let mut target_idx = if !req_lower.is_empty()
                && req_lower != "alvo"
                && req_lower != "id_do_alvo"
                && req_lower != "inimigo"
            {
                sc.participants.iter().position(|p| {
                    !p.is_player
                        && p.is_hostile
                        && !p.is_dead
                        && p.current_hp > 0
                        && (p.id.to_lowercase() == req_lower
                            || p.name.to_lowercase().contains(&req_lower)
                            || req_lower.contains(&p.name.to_lowercase())
                            || (req_lower.contains("#1") && p.name.contains("#1"))
                            || (req_lower.contains("#2") && p.name.contains("#2"))
                            || (req_lower.contains("#3") && p.name.contains("#3")))
                })
            } else {
                None
            };

            // 2. Se não encontrou por nome específico, escolhe o inimigo hostil vivo mais próximo do atacante
            if target_idx.is_none() {
                target_idx = sc
                    .participants
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| !p.is_player && p.is_hostile && !p.is_dead && p.current_hp > 0)
                    .min_by_key(|(_, p)| {
                        let (px, py) = p.position.unwrap_or((45, 20));
                        let dx = px - attacker_pos.0;
                        let dy = py - attacker_pos.1;
                        dx * dx + dy * dy
                    })
                    .map(|(idx, _)| idx);
            }

            // 3. Fallback: qualquer inimigo hostil na cena
            if target_idx.is_none() {
                target_idx = sc
                    .participants
                    .iter()
                    .position(|p| !p.is_player && p.is_hostile);
            }

            if let Some(idx) = target_idx {
                let mut rng = rand::thread_rng();
                let d20: i32 = rng.gen_range(1..=20);
                let is_crit = d20 == 20;
                let is_fumble = d20 == 1;
                let total_hit = d20 + atk_mod;

                let enemy = &mut sc.participants[idx];
                let enemy_ac = if enemy.armor_class > 0 { enemy.armor_class } else { 13 };
                let is_hit = !is_fumble && (is_crit || total_hit >= enemy_ac);

                let final_damage = if is_hit {
                    let dice_count = if is_crit { 2 } else { 1 };
                    let mut roll_sum = 0;
                    for _ in 0..dice_count {
                        if is_spell {
                            roll_sum += rng.gen_range(1..=10);
                        } else {
                            roll_sum += rng.gen_range(1..=8);
                        }
                    }
                    (roll_sum + best_stat_mod).max(1)
                } else {
                    0
                };

                if is_hit {
                    enemy.current_hp = (enemy.current_hp - final_damage).max(0);
                    if enemy.current_hp <= 0 {
                        enemy.current_hp = 0;
                        enemy.is_dead = true;
                    }
                }

                let enemy_id = enemy.id.clone();
                let enemy_name = enemy.name.clone();
                let enemy_hp_remaining = enemy.current_hp;
                let enemy_max_hp = enemy.max_hp.max(12);
                let enemy_killed = enemy.is_dead;

                // Sincroniza HP no EntityStore se a entidade existir lá
                if let Some(st) = effective_store {
                    if let Ok(Some(mut w_sheet)) = st.get_world_entity(&enemy_id) {
                        if let Some(ref mut dnd) = w_sheet.dnd_stats {
                            dnd.current_hp = enemy_hp_remaining;
                        }
                        let _ = st.save_world_entity(&w_sheet);
                    }
                }

                return json!({
                    "status": "success",
                    "action": "attack",
                    "attacker": attacker_name,
                    "target": enemy_name,
                    "target_id": enemy_id,
                    "d20_roll": d20,
                    "modifier": atk_mod,
                    "total_to_hit": total_hit,
                    "total": total_hit,
                    "target_ac": enemy_ac,
                    "dc": enemy_ac,
                    "is_hit": is_hit,
                    "is_success": is_hit,
                    "is_critical": is_crit,
                    "damage": final_damage,
                    "target_hp_remaining": enemy_hp_remaining,
                    "target_max_hp": enemy_max_hp,
                    "target_killed": enemy_killed
                });
            }
        }

        // Fallback quando não há LivingScene ativa (ex: testes unitários isolados)
        let target_name = if requested_target.is_empty() { "Alvo" } else { requested_target };
        let mut attacker = Self::create_default_entity(attacker_name);
        let mut target = Self::create_default_entity(target_name);

        let dagger = EquipmentRegistry::get_weapon("dagger").unwrap_or_else(|| {
            crate::rules::equipment::weapons::Weapon::new(
                "dagger",
                "Adaga",
                crate::rules::equipment::weapons::WeaponCategory::SimpleMelee,
                "1d4",
                crate::rules::equipment::weapons::DamageType::Piercing,
                2,
                1,
                vec![],
            )
        });

        let attack_result: AttackResult = resolve_attack(
            &mut attacker,
            &mut target,
            &dagger,
            true,
            DiceMode::Normal,
            DamageAffinity::Normal,
        );

        json!({
            "status": "success",
            "action": "attack",
            "attacker": attacker_name,
            "target": target_name,
            "target_id": "target_default",
            "d20_roll": attack_result.attack_roll.selected_roll,
            "modifier": attack_result.attack_roll.modifier,
            "total_to_hit": attack_result.attack_roll.total,
            "total": attack_result.attack_roll.total,
            "target_ac": attack_result.target_ac,
            "dc": attack_result.target_ac,
            "is_hit": attack_result.is_hit,
            "is_success": attack_result.is_hit,
            "is_critical": attack_result.is_critical_hit,
            "damage": attack_result.final_damage,
            "target_hp_remaining": target.current_hp,
            "target_max_hp": target.base_max_hp,
            "target_killed": target.current_hp <= 0
        })
    }
}
