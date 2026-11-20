use std::collections::HashMap;
use serde_json::{json, Value};
use crate::narrative::entity_store::{EntityDndStats, EntityType, WorldEntitySheet};
use crate::narrative::living_scene::{
    ActionsBudget, LivingScene, TurnParticipant,
};
use crate::state::{CampaignManifest, CampaignState};
use axum::http::StatusCode;

pub fn default_literary_style() -> String {
    "Fantasia Sombria".to_string()
}

pub fn default_player_name() -> String {
    "Aventureiro".to_string()
}

pub fn default_class_name() -> String {
    "Fighter".to_string()
}

pub fn default_creative_freedom() -> String {
    "balanced".to_string()
}

pub fn default_story_focus() -> String {
    "d20".to_string()
}

pub fn build_player_participant(
    player_id: &str,
    player_name: &str,
    default_class: &str,
    idx: usize,
    player_entities: &[WorldEntitySheet],
) -> TurnParticipant {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let sheet_opt = player_entities
        .iter()
        .find(|pe| pe.id == player_id || pe.name.eq_ignore_ascii_case(player_name));

    let mut max_hp = 12;
    let mut current_hp = 12;
    let mut armor_class = 14;
    let mut dex_mod = 2;
    let mut role_class = default_class.to_string();

    if let Some(sheet) = sheet_opt {
        if let Some(ref dnd) = sheet.dnd_stats {
            if dnd.max_hp > 0 {
                max_hp = dnd.max_hp;
            }
            current_hp = if dnd.current_hp > 0 {
                dnd.current_hp.min(max_hp)
            } else {
                max_hp
            };
            if dnd.armor_class > 0 {
                armor_class = dnd.armor_class;
            }
            if let Some(&dex) = dnd.ability_scores.get("DEX") {
                dex_mod = (dex - 10).div_euclid(2);
            }
        }
        if let Some(fs) = sheet.custom_flags.get("full_sheet") {
            if let Some(ac) = fs["vitals"]["armorClass"].as_i64().filter(|&a| a > 0) {
                armor_class = ac as i32;
            }
            if let Some(hp) = fs["vitals"]["hpMax"].as_i64().filter(|&h| h > 0) {
                max_hp = hp as i32;
                current_hp = current_hp.min(max_hp);
            }
            if let Some(init_m) = fs["vitals"]["initiativeModifier"].as_i64() {
                dex_mod = init_m as i32;
            }
            if let Some(cls) = fs["className"].as_str().filter(|s| !s.is_empty()) {
                role_class = cls.to_string();
            }
        }
        if let Some(cls) = sheet
            .custom_flags
            .get("class_name")
            .or_else(|| sheet.custom_flags.get("class"))
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
        {
            role_class = cls.to_string();
        }

        if armor_class <= 12 {
            let cls_lower = role_class.to_lowercase();
            if cls_lower.contains("guerreiro") || cls_lower.contains("fighter") || cls_lower.contains("palad") {
                armor_class = 18;
            } else if cls_lower.contains("clérig") || cls_lower.contains("cleric") {
                armor_class = 18;
            } else if cls_lower.contains("ranger") || cls_lower.contains("ladino") || cls_lower.contains("rogue") || cls_lower.contains("monk") {
                armor_class = 15;
            } else if cls_lower.contains("barb") {
                armor_class = 15;
            } else if cls_lower.contains("druid") || cls_lower.contains("bardo") || cls_lower.contains("bard") || cls_lower.contains("bruxo") || cls_lower.contains("warlock") {
                armor_class = 14;
            }
        }
    }

    let d20: i32 = rng.gen_range(1..=20);
    let initiative = d20 + dex_mod;
    let y_pos = (6 + (idx as i32 * 7)).min(36);

    TurnParticipant {
        id: player_id.to_string(),
        name: player_name.to_string(),
        is_player: true,
        is_hostile: false,
        initiative,
        initiative_roll: d20,
        initiative_modifier: dex_mod,
        current_hp,
        max_hp,
        armor_class,
        is_dead: current_hp <= 0,
        actions_budget: ActionsBudget::default(),
        position: Some((10, y_pos)),
        token_role: Some(role_class),
    }
}

pub fn parse_npc_multiplier_and_singularize(raw_name: &str) -> (String, usize) {
    let mut name = raw_name.trim().to_string();
    let mut count = 1usize;

    // Detecta padrões como "(3x)", "(2x)", "(4x)", "(x3)", "(x2)"
    for mult in 2..=6 {
        let pat1 = format!("({}x)", mult);
        let pat2 = format!("(x{})", mult);
        let pat3 = format!("({}X)", mult);
        if name.contains(&pat1) {
            count = mult;
            name = name.replace(&pat1, "").trim().to_string();
            break;
        } else if name.contains(&pat2) {
            count = mult;
            name = name.replace(&pat2, "").trim().to_string();
            break;
        } else if name.contains(&pat3) {
            count = mult;
            name = name.replace(&pat3, "").trim().to_string();
            break;
        }
    }

    let lower = name.to_lowercase();
    if count == 1 && !lower.contains('#') {
        if lower.starts_with("batedores ")
            || lower.starts_with("goblins ")
            || lower == "goblins"
            || lower == "batedores goblins"
        {
            count = 3;
        }
    }

    if count > 1 {
        name = name
            .replace("Batedores Goblins", "Batedor Goblin")
            .replace("batedores goblins", "Batedor Goblin")
            .replace("Batedores", "Batedor")
            .replace("Goblins", "Goblin")
            .replace("Guardas", "Guarda")
            .replace("Lobos", "Lobo")
            .replace("Esqueletos", "Esqueleto")
            .replace("Bandidos", "Bandido")
            .replace("Cultistas", "Cultista")
            .replace("Orcs", "Orc");
    }

    (name.trim().to_string(), count)
}

pub fn is_ally_npc_name(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("aliado")
        || lower.contains("aliad")
        || lower.contains("amig")
        || lower.contains("mentora")
        || lower.contains("mentor")
        || lower.contains("guardião")
        || lower.contains("guardiao")
        || lower.contains("companheiro")
        || lower.contains("companheira")
        || lower.contains("protetor")
        || lower.contains("protetora")
        || lower.contains("refém")
        || lower.contains("refem")
        || lower.contains("prisioneiro")
        || lower.contains("prisioneira")
        || lower.contains("caçador de goblins")
        || lower.contains("cacador de goblins")
        || lower.contains("goblin slayer")
        || lower.contains("sacerdotisa")
        || lower.contains("arqueira elfa")
        || lower.contains("xamã anão")
        || lower.contains("xama anao")
        || lower.contains("sacerdote lagarto")
}

pub fn calculate_dnd_monster_stats(name: &str, act_number: u32) -> (i32, i32, i32) {
    let lower = name.to_lowercase();
    if lower.contains("goblin") || lower.contains("kobold") || lower.contains("batedor") || lower.contains("scout") || lower.contains("lacaio") || lower.contains("minion") {
        (7, 14, 2)
    } else if lower.contains("esqueleto") || lower.contains("skeleton") {
        (13, 13, 2)
    } else if lower.contains("zumbi") || lower.contains("zombie") {
        (22, 8, -1)
    } else if lower.contains("lobo") || lower.contains("wolf") || lower.contains("cão") || lower.contains("cao") || lower.contains("hound") {
        (11, 13, 2)
    } else if lower.contains("orc") || lower.contains("hobgoblin") || lower.contains("bugbear") || lower.contains("bandido") {
        (15, 14, 1)
    } else {
        let base_hp = if act_number <= 1 { 8 } else { 12 };
        (base_hp, 13, 2)
    }
}

pub fn expand_and_build_npc_participants(
    active_npcs: &[String],
    act_number: u32,
    custom_monsters: &[Value],
    homebrew_bosses: &[Value],
    world_entities: &[WorldEntitySheet],
) -> Vec<TurnParticipant> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();
    let mut hostile_k = 0usize;

    for (idx, npc_raw) in active_npcs.iter().enumerate() {
        if is_ally_npc_name(npc_raw) {
            let d20: i32 = rng.gen_range(1..=20);
            let init_mod = 2;
            result.push(TurnParticipant {
                id: format!("npc_act{}_{}", act_number, idx + 1),
                name: npc_raw.clone(),
                is_player: false,
                is_hostile: false,
                initiative: d20 + init_mod,
                initiative_roll: d20,
                initiative_modifier: init_mod,
                current_hp: 24,
                max_hp: 24,
                armor_class: 16,
                is_dead: false,
                actions_budget: ActionsBudget::default(),
                position: Some((18, 20)),
                token_role: Some("Aliado".to_string()),
            });
            continue;
        }

        let (base_name, count) = parse_npc_multiplier_and_singularize(npc_raw);
        if count > 1 {
            let (m_hp, m_ac, m_init) = calculate_dnd_monster_stats(&base_name, act_number);
            for k in 0..count {
                let d20: i32 = rng.gen_range(1..=20);
                let x = (42 + ((hostile_k as i32 % 3) * 5)).min(56);
                let y = (8 + ((hostile_k as i32 * 10) % 28)).min(36);
                hostile_k += 1;

                result.push(TurnParticipant {
                    id: format!("npc_act{}_{}_{}", act_number, idx + 1, k + 1),
                    name: format!("{} #{}", base_name, k + 1),
                    is_player: false,
                    is_hostile: true,
                    initiative: d20 + m_init,
                    initiative_roll: d20,
                    initiative_modifier: m_init,
                    current_hp: m_hp,
                    max_hp: m_hp,
                    armor_class: m_ac,
                    is_dead: false,
                    actions_budget: ActionsBudget::default(),
                    position: Some((x, y)),
                    token_role: Some("Inimigo".to_string()),
                });
            }
            continue;
        }

        // Verifica se corresponde a um monstro customizado ou Boss Homebrew
        let norm_lower = base_name.to_lowercase();
        let matched_custom = custom_monsters
            .iter()
            .chain(homebrew_bosses.iter())
            .find(|m| {
                m["name"]
                    .as_str()
                    .map(|n| {
                        let nl = n.to_lowercase();
                        nl == norm_lower || norm_lower.contains(&nl) || nl.contains(&norm_lower)
                    })
                    .unwrap_or(false)
            });

        let matched_world_boss = world_entities.iter().find(|we| {
            we.custom_flags.contains_key("is_homebrew_boss")
                && (we.name.to_lowercase() == norm_lower
                    || norm_lower.contains(&we.name.to_lowercase())
                    || we.name.to_lowercase().contains(&norm_lower))
        });

        let is_boss_keyword = norm_lower.contains("chefe")
            || norm_lower.contains("boss")
            || norm_lower.contains("líder")
            || norm_lower.contains("lider")
            || norm_lower.contains("xamã")
            || norm_lower.contains("xama")
            || norm_lower.contains("rei")
            || norm_lower.contains("dragão")
            || norm_lower.contains("dragao");

        let (hp, ac, role, init_mod) = if let Some(cm) = matched_custom {
            let hp_v = cm["hp"].as_i64().unwrap_or(45) as i32;
            let ac_v = cm["ac"].as_i64().unwrap_or(15) as i32;
            (hp_v.max(15), ac_v.max(13), "Chefe", 3)
        } else if let Some(wb) = matched_world_boss {
            let dnd = wb.dnd_stats.as_ref();
            let hp_v = dnd.map(|d| d.max_hp).unwrap_or(45);
            let ac_v = dnd.map(|d| d.armor_class).unwrap_or(15);
            (hp_v.max(15), ac_v.max(13), "Chefe", 3)
        } else if is_boss_keyword {
            let boss_hp = if act_number <= 1 { 21 } else { 36 };
            (boss_hp, 15, "Chefe", 3)
        } else {
            let (m_hp, m_ac, m_init) = calculate_dnd_monster_stats(&base_name, act_number);
            (m_hp, m_ac, "Inimigo", m_init)
        };

        let d20: i32 = rng.gen_range(1..=20);
        let x = if role == "Chefe" {
            50
        } else {
            (42 + ((hostile_k as i32 % 3) * 5)).min(56)
        };
        let y = if role == "Chefe" {
            20
        } else {
            (8 + ((hostile_k as i32 * 10) % 28)).min(36)
        };
        hostile_k += 1;

        result.push(TurnParticipant {
            id: format!("npc_act{}_{}", act_number, idx + 1),
            name: base_name,
            is_player: false,
            is_hostile: true,
            initiative: d20 + init_mod,
            initiative_roll: d20,
            initiative_modifier: init_mod,
            current_hp: hp,
            max_hp: hp,
            armor_class: ac,
            is_dead: false,
            actions_budget: ActionsBudget::default(),
            position: Some((x, y)),
            token_role: Some(role.to_string()),
        });
    }

    result
}

pub fn ensure_tactical_scene_hydrated(
    scene: &mut LivingScene,
    player_entities: &[WorldEntitySheet],
) -> bool {
    let mut changed = false;

    if scene.obstacles.is_empty() {
        scene.obstacles = LivingScene::generate_default_obstacles(&scene.location, &scene.title);
        changed = true;
    }

    let needs_full_hydration = scene.participants.iter().any(|p| {
        p.max_hp <= 0
            || p.initiative_roll <= 0
            || p.token_role.is_none()
            || (!p.is_player
                && (p.name.contains("(3x)")
                    || p.name.contains("(2x)")
                    || p.name.contains("(4x)")
                    || p.name.contains("(x3)")
                    || p.name.contains("(x2)")))
    });

    if needs_full_hydration {
        let mut new_participants = Vec::new();
        let existing_players: Vec<TurnParticipant> = scene
            .participants
            .iter()
            .filter(|p| p.is_player)
            .cloned()
            .collect();

        for (i, p) in existing_players.iter().enumerate() {
            let mut hydrated = build_player_participant(&p.id, &p.name, "Fighter", i, player_entities);
            if p.current_hp > 0 && p.max_hp > 0 {
                hydrated.current_hp = p.current_hp;
                hydrated.max_hp = p.max_hp;
                hydrated.is_dead = p.is_dead;
            }
            new_participants.push(hydrated);
        }

        let npc_source_names: Vec<String> = if !scene.active_npcs.is_empty() {
            scene.active_npcs.clone()
        } else {
            scene
                .participants
                .iter()
                .filter(|p| !p.is_player)
                .map(|p| p.name.clone())
                .collect()
        };

        let expanded_npcs = expand_and_build_npc_participants(
            &npc_source_names,
            scene.chapter.max(1),
            &[],
            &[],
            &[],
        );
        new_participants.extend(expanded_npcs);

        // Ordena por iniciativa decrescente
        new_participants.sort_by(|a, b| b.initiative.cmp(&a.initiative));
        scene.participants = new_participants;
        scene.in_combat = scene.has_live_hostiles();

        if let Some(first_player_idx) = scene
            .participants
            .iter()
            .position(|p| p.is_player && !p.is_dead && p.current_hp > 0)
        {
            scene.current_participant_index = first_player_idx;
            scene.participants[first_player_idx].actions_budget.reset_turn();
        } else {
            scene.current_participant_index = 0;
        }

        changed = true;
    } else {
        // Mesmo em cena já hidratada, garante que current_participant_index nunca fique preso em um NPC ou jogador morto
        let current_is_valid_player = scene
            .participants
            .get(scene.current_participant_index)
            .map(|p| p.is_player && !p.is_dead && p.current_hp > 0)
            .unwrap_or(false);

        if !current_is_valid_player {
            if let Some(first_player_idx) = scene
                .participants
                .iter()
                .position(|p| p.is_player && !p.is_dead && p.current_hp > 0)
            {
                scene.current_participant_index = first_player_idx;
                scene.participants[first_player_idx].actions_budget.reset_turn();
                changed = true;
            }
        }
    }

    changed
}

pub fn hydrate_campaign_active_scene(camp: &mut CampaignState) {
    let player_entities = camp
        .get_entity_store()
        .and_then(|s| s.list_player_entities())
        .unwrap_or_default();

    let mut did_change = false;
    if let Some(ref mut scene) = camp.active_scene {
        did_change = ensure_tactical_scene_hydrated(scene, &player_entities);
    }
    if did_change {
        if let (Ok(store), Some(ref scene)) = (camp.get_entity_store(), camp.active_scene.as_ref()) {
            for np in scene.participants.iter().filter(|p| !p.is_player) {
                if store.get_world_entity(&np.id).ok().flatten().is_none() {
                    let mut sheet = WorldEntitySheet::new(
                        &np.id,
                        &np.name,
                        EntityType::Npc,
                        &format!("Combatente na cena '{}' ({})", scene.title, scene.location),
                    );
                    let mut dnd = EntityDndStats::default();
                    dnd.max_hp = np.max_hp;
                    dnd.current_hp = np.current_hp;
                    dnd.armor_class = np.armor_class;
                    sheet.dnd_stats = Some(dnd);
                    let _ = store.save_world_entity(&sheet);
                }
            }
        }
        let _ = camp.save_active_scene();
    }
}

pub fn validate_actor_turn_permission(
    manifest_opt: Option<&CampaignManifest>,
    actor_id: &str,
    req_player_id: Option<&str>,
    req_player_code: Option<&str>,
) -> Result<(), (StatusCode, Value)> {
    let manifest = match manifest_opt {
        Some(m) => m,
        None => return Ok(()),
    };

    let total_slots = manifest.player_slots.len();
    if total_slots <= 1 {
        return Ok(());
    }

    if req_player_id.is_none() && req_player_code.is_none() {
        return Ok(());
    }

    let requester_slot = manifest.player_slots.iter().find(|s| {
        if let Some(pid) = req_player_id {
            if s.player_id == pid {
                return true;
            }
        }
        if let Some(pcode) = req_player_code {
            if s.player_code.eq_ignore_ascii_case(pcode.trim()) {
                return true;
            }
        }
        false
    });

    let requester_slot = match requester_slot {
        Some(s) => s,
        None => {
            return Err((
                StatusCode::FORBIDDEN,
                json!({
                    "status": "error",
                    "code": "INVALID_PLAYER_CREDENTIALS",
                    "message": "Credenciais de jogador inválidas ou não encontradas na campanha."
                }),
            ));
        }
    };

    if requester_slot.is_host {
        return Ok(());
    }

    let is_owner = requester_slot.player_id == actor_id
        || requester_slot.player_name.eq_ignore_ascii_case(actor_id);

    if !is_owner {
        let owner_slot = manifest.player_slots.iter().find(|s| {
            s.player_id == actor_id || s.player_name.eq_ignore_ascii_case(actor_id)
        });
        let owner_name = owner_slot.map(|s| s.player_name.as_str()).unwrap_or("outro jogador");

        return Err((
            StatusCode::FORBIDDEN,
            json!({
                "status": "error",
                "code": "NOT_YOUR_TURN_OR_ACTOR",
                "message": format!(
                    "Não é a sua vez ou você não tem permissão para controlar este personagem. Este herói é controlado por '{}'.",
                    owner_name
                )
            }),
        ));
    }

    Ok(())
}

pub fn build_player_entity_from_sheet(
    player_id: &str,
    player_number: usize,
    payload: &Value,
) -> WorldEntitySheet {
    let name = payload["name"].as_str().unwrap_or("Aventureiro");
    let class_name = payload["className"].as_str().unwrap_or("Fighter");
    let race = payload["race"].as_str().unwrap_or("Humano");
    let level = payload["level"].as_i64().unwrap_or(1);
    let background = payload["background"].as_str().unwrap_or("Aventureiro");
    let subclass = payload["subclass"].as_str().unwrap_or("");

    let desc = if !subclass.is_empty() {
        format!(
            "Jogador #{}: {} — {} ({}) Nível {} ({})",
            player_number, race, class_name, subclass, level, background
        )
    } else {
        format!(
            "Jogador #{}: {} — {} Nível {} ({})",
            player_number, race, class_name, level, background
        )
    };

    let mut sheet = WorldEntitySheet::new(player_id, name, EntityType::Player, &desc);

    let vitals = &payload["vitals"];
    let abilities = &payload["abilities"];

    let mut scores = HashMap::new();
    scores.insert(
        "STR".to_string(),
        abilities["strength"].as_i64().unwrap_or(10) as i32,
    );
    scores.insert(
        "DEX".to_string(),
        abilities["dexterity"].as_i64().unwrap_or(10) as i32,
    );
    scores.insert(
        "CON".to_string(),
        abilities["constitution"].as_i64().unwrap_or(10) as i32,
    );
    scores.insert(
        "INT".to_string(),
        abilities["intelligence"].as_i64().unwrap_or(10) as i32,
    );
    scores.insert(
        "WIS".to_string(),
        abilities["wisdom"].as_i64().unwrap_or(10) as i32,
    );
    scores.insert(
        "CHA".to_string(),
        abilities["charisma"].as_i64().unwrap_or(10) as i32,
    );

    let mut proficiencies = Vec::new();
    if let Some(skills) = payload["skillProficiencies"].as_array() {
        for s in skills {
            if let Some(st) = s.as_str() {
                proficiencies.push(st.to_string());
            }
        }
    }
    if let Some(saves) = payload["savingThrowProficiencies"].as_array() {
        for sv in saves {
            if let Some(st) = sv.as_str() {
                proficiencies.push(format!("Save: {}", st));
            }
        }
    }

    let mut spell_slots_total = HashMap::new();
    let mut spell_slots_used = HashMap::new();
    if let Some(levels_obj) = payload["spellSlots"]["levels"].as_object() {
        for (k, v) in levels_obj {
            if let Ok(lvl) = k.parse::<u8>() {
                let total = v["total"].as_u64().unwrap_or(0) as u8;
                let used = v["used"].as_u64().unwrap_or(0) as u8;
                if total > 0 {
                    spell_slots_total.insert(lvl, total);
                    spell_slots_used.insert(lvl, used);
                }
            }
        }
    }

    let mut conditions = Vec::new();
    if let Some(conds) = vitals["conditions"].as_array() {
        for c in conds {
            if let Some(cs) = c.as_str() {
                conditions.push(cs.to_string());
            }
        }
    }

    let max_hp = vitals["hpMax"].as_i64().unwrap_or(10) as i32;
    let current_hp = vitals["hpCurrent"].as_i64().unwrap_or(max_hp as i64) as i32;
    let temp_hp = vitals["hpTemp"].as_i64().unwrap_or(0) as i32;
    let armor_class = vitals["armorClass"].as_i64().unwrap_or(10) as i32;

    sheet.dnd_stats = Some(EntityDndStats {
        armor_class,
        max_hp,
        current_hp,
        temporary_hp: temp_hp,
        ability_scores: scores,
        proficiencies,
        spell_slots_total,
        spell_slots_used,
        conditions,
        class_resources: HashMap::new(),
    });

    let xp = payload["experiencePoints"]
        .as_u64()
        .or_else(|| payload["xp"].as_u64())
        .unwrap_or(0) as u32;
    sheet.custom_flags.insert("xp".to_string(), json!(xp));
    sheet.custom_flags.insert("level".to_string(), json!(level));
    sheet
        .custom_flags
        .insert("class_name".to_string(), json!(class_name));
    sheet.custom_flags.insert("race".to_string(), json!(race));
    sheet
        .custom_flags
        .insert("background".to_string(), json!(background));
    if !subclass.is_empty() {
        sheet
            .custom_flags
            .insert("subclass".to_string(), json!(subclass));
    }
    if let Some(inv) = payload.get("inventory") {
        sheet
            .custom_flags
            .insert("inventory".to_string(), inv.clone());
    }
    sheet
        .custom_flags
        .insert("full_sheet".to_string(), payload.clone());

    sheet
}
