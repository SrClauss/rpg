use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{delete, get, post},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

use crate::api::helpers::{
    build_player_entity_from_sheet, build_player_participant,
    ensure_tactical_scene_hydrated, expand_and_build_npc_participants,
    hydrate_campaign_active_scene,
};
use crate::api::models::{
    AddLocalPlayerRequest, AnalyzePremiseRequest, CreateCampaignRequest, JoinCampaignRequest,
    SendChatMessageRequest, SyncPartyRequest,
};
use crate::api::state::{extract_bearer_token, ServerState};
use crate::llm::skeleton_agent::SkeletonAgent;
use crate::narrative::entity_store::{EntityDndStats, EntityType, WorldEntitySheet};
use crate::state::{
    generate_4letter_player_code, generate_4letter_room_code, CampaignManifest, CampaignState,
    PlayerSlot,
};

pub fn campaign_router() -> Router<ServerState> {
    Router::new()
        // Criação e Pré-análise
        .route("/api/campaign/create", post(create_campaign))
        .route("/api/campaigns/create", post(create_campaign))
        .route("/api/campaign/analyze-premise", post(analyze_premise))
        .route("/api/campaigns/analyze-premise", post(analyze_premise))
        // Entrada na sala
        .route("/api/campaign/join", post(join_campaign))
        .route("/api/campaigns/join", post(join_campaign))
        // Estado, Crônica e Entidades
        .route("/api/campaigns/{id}/state", get(get_campaign_state))
        .route("/api/campaigns/{id}/chronicle", get(get_campaign_chronicle))
        .route("/api/campaigns/{id}/entities", get(get_campaign_entities))
        // Slots e gerenciamento presencial
        .route("/api/campaigns/{id}/slots", get(get_campaign_slots))
        .route("/api/campaign/add-local-player", post(add_local_player))
        .route("/api/campaigns/add-local-player", post(add_local_player))
        .route("/api/campaigns/{id}/sync-party", post(sync_campaign_party))
        .route("/api/campaigns/{id}", delete(delete_saved_campaign))
        // Persistência, Catálogo e Bundles
        .route("/api/campaigns/saved", get(list_saved_campaigns))
        .route("/api/campaigns/{id}/export", get(export_campaign_bundle))
        .route("/api/campaign/import", post(import_campaign_bundle))
        .route("/api/campaigns/import", post(import_campaign_bundle))
        .route("/api/premises/saved", get(list_saved_premises))
        // Chat da Party e Auditoria
        .route("/api/campaigns/{id}/chat", post(send_party_chat))
        .route("/api/campaigns/{id}/audit", get(get_campaign_audit))
}

pub async fn get_or_load_campaign(
    state: &ServerState,
    campaign_id: &str,
) -> Option<Arc<RwLock<CampaignState>>> {
    {
        let campaigns = state.campaigns.read().await;
        if let Some(c) = campaigns.get(campaign_id) {
            return Some(c.clone());
        }
    }

    let dir = format!("data/campaigns/{}", campaign_id);
    let manifest_path = format!("{}/manifest.json", dir);
    if std::path::Path::new(&manifest_path).exists() {
        if let Ok(content) = std::fs::read_to_string(&manifest_path) {
            if let Ok(manifest_val) = serde_json::from_str::<Value>(&content) {
                let room_code = manifest_val["room_code"]
                    .as_str()
                    .unwrap_or_else(|| &campaign_id[..4.min(campaign_id.len())])
                    .to_uppercase();

                let mut loaded_camp = CampaignState::with_id_and_room(campaign_id, &room_code);
                let _ = loaded_camp.load_manifest();
                let _ = loaded_camp.load_active_scene();
                hydrate_campaign_active_scene(&mut loaded_camp);
                let arc_c = Arc::new(RwLock::new(loaded_camp));

                state
                    .rooms
                    .write()
                    .await
                    .insert(room_code.clone(), arc_c.clone());
                state
                    .campaigns
                    .write()
                    .await
                    .insert(campaign_id.to_string(), arc_c.clone());

                let mut channels = state.channels.write().await;
                channels
                    .entry(room_code.clone())
                    .or_insert_with(|| broadcast::channel(100).0);

                println!(
                    ">>> [CAMPAIGN] Campanha restaurada e hidratada sob demanda do disco: ID={}, Sala=#{}",
                    campaign_id, room_code
                );
                return Some(arc_c);
            }
        }
    }

    None
}

pub async fn get_or_load_campaign_by_room(
    state: &ServerState,
    room_code: &str,
) -> Option<Arc<RwLock<CampaignState>>> {
    {
        let rooms = state.rooms.read().await;
        if let Some(c) = rooms.get(room_code) {
            return Some(c.clone());
        }
    }

    if let Ok(entries) = std::fs::read_dir("data/campaigns") {
        for entry in entries.flatten() {
            let manifest_path = entry.path().join("manifest.json");
            if manifest_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                    if let Ok(manifest_val) = serde_json::from_str::<Value>(&content) {
                        if manifest_val["room_code"]
                            .as_str()
                            .map(|r| r.eq_ignore_ascii_case(room_code))
                            .unwrap_or(false)
                        {
                            if let Some(cid) = entry.file_name().to_str() {
                                return get_or_load_campaign(state, cid).await;
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

pub async fn analyze_premise(
    State(state): State<ServerState>,
    Json(req): Json<AnalyzePremiseRequest>,
) -> (StatusCode, Json<Value>) {
    println!(">>> [PREMISE] Analisando premissa para requisitos e composição do grupo...");
    let slider = req.dramatic_focus_slider.unwrap_or(10);
    match state
        .skeleton_agent
        .analyze_premise_requirements(&req.premise, &req.literary_style, slider)
        .await
    {
        Ok(analysis) => {
            println!(">>> [PREMISE] Análise estruturada concluída via IA com sucesso!");
            (StatusCode::OK, Json(json!({ "status": "ok", "analysis": analysis })))
        }
        Err(e) => {
            println!(">>> [PREMISE] Fallback para gerador determinístico: {}", e);
            let fallback = SkeletonAgent::deterministic_premise_analysis(&req.premise, slider);
            (
                StatusCode::OK,
                Json(json!({
                    "status": "ok",
                    "fallback_mode": true,
                    "analysis": fallback
                })),
            )
        }
    }
}

pub async fn create_campaign(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Json(req): Json<CreateCampaignRequest>,
) -> (StatusCode, Json<Value>) {
    // Autenticação opcional: Host pode ter conta SaaS ou entrar anonimamente via UUID de convite
    let auth_user_opt = if let Some(token) = extract_bearer_token(&headers) {
        state.user_store.get_user_by_token(&token).await
    } else {
        None
    };

    if let Some(ref auth_user) = auth_user_opt {
        println!(">>> [CAMPAIGN] Host autenticado '{}' ({}, plano: {}) criando nova sessão...", auth_user.name, auth_user.email, auth_user.plan);
    } else {
        println!(">>> [CAMPAIGN] Host anônimo criando sessão com UUID de convite...");
    }

    let campaign_id = uuid::Uuid::new_v4().to_string();
    let room_code = generate_4letter_room_code();
    let host_id = format!("player_host_{}", &uuid::Uuid::new_v4().to_string()[..6]);
    let host_code = generate_4letter_player_code();

    // 1. Determina composição completa do grupo (1 a 4 heróis)
    let mut player_slots = Vec::new();
    let mut player_ids = Vec::new();

    if let Some(ref party) = req.party.as_ref().filter(|p| !p.is_empty()) {
        for (i, hero) in party.iter().enumerate() {
            let p_num = i + 1;
            let hero_name = hero["name"]
                .as_str()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or(if i == 0 { &req.host_player_name } else { "Herói" })
                .to_string();
            let hero_class = hero["class"]
                .as_str()
                .or_else(|| hero["character_class"].as_str())
                .filter(|s| !s.trim().is_empty())
                .unwrap_or(if i == 0 { &req.host_class } else { "Fighter" })
                .to_string();
            let p_id = if i == 0 {
                host_id.clone()
            } else {
                format!("player_{}_{}", p_num, &uuid::Uuid::new_v4().to_string()[..6])
            };
            let p_code = if i == 0 {
                host_code.clone()
            } else {
                generate_4letter_player_code()
            };

            let slot = PlayerSlot {
                player_number: p_num,
                player_id: p_id.clone(),
                player_name: hero_name,
                class_name: hero_class,
                player_code: p_code,
                is_host: i == 0,
            };

            player_slots.push(slot);
            player_ids.push(p_id);
        }
    } else {
        let host_slot = PlayerSlot {
            player_number: 1,
            player_id: host_id.clone(),
            player_name: req.host_player_name.clone(),
            class_name: req.host_class.clone(),
            player_code: host_code.clone(),
            is_host: true,
        };
        player_slots.push(host_slot);
        player_ids.push(host_id.clone());
    }

    // 2. Constrói premissa enriquecida com os monstros customizados se informados ou extraídos da premissa
    let mut effective_monsters = req.custom_monsters.clone();
    if effective_monsters.is_empty() {
        let analysis = state
            .skeleton_agent
            .analyze_premise_requirements(&req.premise, &req.literary_style, req.dramatic_focus_slider.unwrap_or(10))
            .await
            .unwrap_or_else(|_| SkeletonAgent::deterministic_premise_analysis(&req.premise, req.dramatic_focus_slider.unwrap_or(10)));
        if let Some(monsters) = analysis.get("custom_monsters").and_then(|m| m.as_array()) {
            effective_monsters = monsters.clone();
        }
    }

    let mut enriched_premise = req.premise.clone();
    if !effective_monsters.is_empty() {
        enriched_premise.push_str("\n\n[MONSTROS / ANTAGONISTAS HOMEBREW REGISTRADOS NA PREMISSA]:\n");
        for (i, m) in effective_monsters.iter().enumerate() {
            let m_name = m["name"].as_str().unwrap_or("Monstro Homebrew");
            let m_cr = m["challenge_rating"].as_str().or_else(|| m["cr"].as_str()).unwrap_or("?");
            let m_desc = m["description"].as_str().unwrap_or("");
            enriched_premise.push_str(&format!(
                "- Monstro #{}: {} (ND: {}). Detalhes / Poderes: {}\n",
                i + 1, m_name, m_cr, m_desc
            ));
        }
    }

    // 3. Foco dramático ajustado pelo slider contínuo (1 a 20) ou rolagem D20
    let effective_focus = if let Some(slider_val) = req.dramatic_focus_slider {
        println!(">>> [CAMPAIGN] Slider Dramático Contínuo D20: {}/20", slider_val);
        if slider_val <= 6 {
            "roleplay"
        } else if slider_val >= 14 {
            "combat"
        } else {
            "balanced"
        }
    } else if req.story_focus == "d20" {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let secret_roll: u32 = rng.gen_range(1..=20);
        println!(">>> [SEGREDO DO MESTRE - NÃO MOSTRADO AO GRUPO] Rolagem oculta D20 do Destino: {}/20 (<=10: Foco Roleplay, >=11: Foco Batalhas)", secret_roll);
        if secret_roll <= 10 {
            "roleplay"
        } else {
            "combat"
        }
    } else {
        req.story_focus.as_str()
    };

    // 4. Gera o Esqueleto Narrativo Oculto via IA com o conhecimento dos monstros, arcos e foco dramático
    let hidden_skeleton = state
        .skeleton_agent
        .generate_skeleton(&enriched_premise, &req.literary_style, &req.ai_creative_freedom, effective_focus)
        .await
        .unwrap_or_else(|_| crate::narrative::skeleton::NarrativeSkeleton::new(&enriched_premise, &req.literary_style));

    let manifest = CampaignManifest {
        campaign_id: campaign_id.clone(),
        room_code: room_code.clone(),
        host_player_id: host_id.clone(),
        player_ids: player_ids.clone(),
        player_slots: player_slots.clone(),
        premise: req.premise.clone(),
        literary_style: req.literary_style.clone(),
        hidden_skeleton: hidden_skeleton.clone(),
        created_at: chrono::Utc::now().to_rfc3339(),
        status: "active".to_string(),
        custom_monsters: effective_monsters.clone(),
        world_prologue: None,
    };

    let mut campaign = CampaignState::with_id_and_room(&campaign_id, &room_code);
    campaign.manifest = Some(manifest.clone());
    let _ = campaign.save_manifest();

    // 5. Cria fichas de todos os heróis do grupo e dos Monstros no EntityStore
    if let Ok(entity_store) = campaign.get_entity_store() {
        if let Some(ref party) = req.party.as_ref().filter(|p| !p.is_empty()) {
            for (i, hero) in party.iter().enumerate() {
                let slot = &player_slots[i];
                let hero_race = hero["race"].as_str().unwrap_or("Humano").to_string();
                let mut hero_sheet = WorldEntitySheet::new(
                    &slot.player_id,
                    &slot.player_name,
                    EntityType::Player,
                    &format!("Herói {} ({}), jogador {}", slot.class_name, hero_race, slot.player_number),
                );

                let mut dnd_stats = EntityDndStats::default();
                let vitals_hp = hero.get("vitals").and_then(|v| v.get("hpMax")).and_then(|v| v.as_i64());
                let vitals_curr_hp = hero.get("vitals").and_then(|v| v.get("hpCurrent")).and_then(|v| v.as_i64());
                let direct_hp = hero["hp"].as_i64().or_else(|| hero["max_hp"].as_i64());
                if let Some(hp) = vitals_hp.or(direct_hp) {
                    dnd_stats.max_hp = hp as i32;
                    dnd_stats.current_hp = vitals_curr_hp.unwrap_or(hp) as i32;
                }

                let vitals_ac = hero.get("vitals").and_then(|v| v.get("armorClass")).and_then(|v| v.as_i64());
                let direct_ac = hero["ac"].as_i64().or_else(|| hero["armor_class"].as_i64());
                let extracted_ac = vitals_ac.or(direct_ac).map(|a| a as i32).unwrap_or(0);
                dnd_stats.armor_class = if extracted_ac > 0 {
                    extracted_ac
                } else {
                    match slot.class_name.to_lowercase().as_str() {
                        "fighter" | "guerreiro" | "paladin" | "paladino" => 18,
                        "cleric" | "clérigo" | "clerigo" => 18,
                        "barbarian" | "bárbaro" | "barbaro" => 15,
                        "ranger" | "patrulheiro" | "rogue" | "ladino" | "monk" | "monge" => 15,
                        "druid" | "druida" | "bard" | "bardo" | "warlock" | "bruxo" => 14,
                        _ => 12,
                    }
                };

                let vitals_init = hero.get("vitals").and_then(|v| v.get("initiativeModifier")).and_then(|v| v.as_i64());
                let direct_init = hero["initiative_modifier"].as_i64();
                if let Some(init) = vitals_init.or(direct_init) {
                    dnd_stats.ability_scores.insert("DEX".to_string(), 10 + (init as i32 * 2));
                }

                hero_sheet.dnd_stats = Some(dnd_stats);
                hero_sheet.custom_flags.insert("class_name".to_string(), json!(slot.class_name));
                hero_sheet.custom_flags.insert("full_sheet".to_string(), hero.clone());
                let _ = entity_store.save_player_entity(&hero_sheet);
            }
        } else {
            let mut host_sheet = WorldEntitySheet::new(
                &host_id,
                &req.host_player_name,
                EntityType::Player,
                &format!("Líder do Grupo / Anfitrião ({})", req.host_class),
            );
            host_sheet.dnd_stats = Some(EntityDndStats::default());
            let _ = entity_store.save_player_entity(&host_sheet);
        }

        for (idx, m) in effective_monsters.iter().enumerate() {
            let m_name = m["name"].as_str().unwrap_or("Monstro Homebrew");
            let m_id = format!("custom_boss_{}_{}", idx + 1, m_name.to_lowercase().replace(' ', "_"));
            let m_desc = m["description"].as_str().unwrap_or("Monstro Homebrew da Campanha");

            let mut monster_sheet = WorldEntitySheet::new(
                &m_id,
                m_name,
                EntityType::Npc,
                m_desc,
            );

            let hp = m["hp"].as_i64().unwrap_or(100) as i32;
            let ac = m["ac"].as_i64().unwrap_or(15) as i32;
            let mut scores = std::collections::HashMap::new();

            let stats_obj = m.get("stats").or_else(|| m.get("ability_scores"));
            if let Some(s) = stats_obj.and_then(|v| v.as_object()) {
                for (k, v) in s {
                    if let Some(val) = v.as_i64() {
                        scores.insert(k.to_uppercase(), val as i32);
                    }
                }
            } else {
                scores.insert("STR".to_string(), m["str"].as_i64().unwrap_or(10) as i32);
                scores.insert("DEX".to_string(), m["dex"].as_i64().unwrap_or(10) as i32);
                scores.insert("CON".to_string(), m["con"].as_i64().unwrap_or(10) as i32);
                scores.insert("INT".to_string(), m["int"].as_i64().unwrap_or(10) as i32);
                scores.insert("WIS".to_string(), m["wis"].as_i64().unwrap_or(10) as i32);
                scores.insert("CHA".to_string(), m["cha"].as_i64().unwrap_or(10) as i32);
            }

            let mut dnd_stats = EntityDndStats::default();
            dnd_stats.max_hp = hp;
            dnd_stats.current_hp = hp;
            dnd_stats.armor_class = ac;
            dnd_stats.ability_scores = scores;

            monster_sheet.dnd_stats = Some(dnd_stats);
            monster_sheet.custom_flags.insert("is_homebrew_boss".to_string(), json!(true));
            monster_sheet.custom_flags.insert("homebrew_raw".to_string(), m.clone());

            let _ = entity_store.save_world_entity(&monster_sheet);
            println!(">>> [CAMPAIGN] Monstro Homebrew (Jogador/Premissa) registrado no EntityStore: {} (ID: {})", m_name, m_id);
        }

        let effective_names: std::collections::HashSet<String> = effective_monsters.iter()
            .filter_map(|m| m.get("name").and_then(|n| n.as_str()).map(|s| s.trim().to_lowercase()))
            .collect();

        for (idx, b) in hidden_skeleton.homebrew_bosses.iter().enumerate() {
            let b_name = b["name"].as_str().unwrap_or("Boss Homebrew da IA");
            let norm_name = b_name.trim().to_lowercase();
            if effective_names.contains(&norm_name) {
                println!(">>> [CAMPAIGN] Boss Homebrew da IA ignorado por duplicidade com monstro da premissa: {}", b_name);
                continue;
            }
            let b_id = format!("ai_boss_{}_{}", idx + 1, norm_name.replace(' ', "_"));
            let b_desc = b["description"].as_str().unwrap_or("Boss Homebrew gerado autonomamente pelo Mestre");

            let mut boss_sheet = WorldEntitySheet::new(
                &b_id,
                b_name,
                EntityType::Npc,
                b_desc,
            );

            let hp = b["hp"].as_i64().unwrap_or(120) as i32;
            let ac = b["ac"].as_i64().unwrap_or(16) as i32;
            let mut scores = std::collections::HashMap::new();
            scores.insert("STR".to_string(), b["str"].as_i64().unwrap_or(12) as i32);
            scores.insert("DEX".to_string(), b["dex"].as_i64().unwrap_or(14) as i32);
            scores.insert("CON".to_string(), b["con"].as_i64().unwrap_or(14) as i32);
            scores.insert("INT".to_string(), b["int"].as_i64().unwrap_or(14) as i32);
            scores.insert("WIS".to_string(), b["wis"].as_i64().unwrap_or(12) as i32);
            scores.insert("CHA".to_string(), b["cha"].as_i64().unwrap_or(16) as i32);

            let mut dnd_stats = EntityDndStats::default();
            dnd_stats.max_hp = hp;
            dnd_stats.current_hp = hp;
            dnd_stats.armor_class = ac;
            dnd_stats.ability_scores = scores;

            boss_sheet.dnd_stats = Some(dnd_stats);
            boss_sheet.custom_flags.insert("is_homebrew_boss".to_string(), json!(true));
            boss_sheet.custom_flags.insert("created_by_ai".to_string(), json!(true));
            boss_sheet.custom_flags.insert("homebrew_raw".to_string(), b.clone());

            let _ = entity_store.save_world_entity(&boss_sheet);
            println!(">>> [CAMPAIGN] Boss Homebrew (IA Autônomo) registrado no EntityStore: {} (ID: {})", b_name, b_id);
        }
    }

    let prologue_instruction = "Você é o Mestre Escritor, Historiador Cósmico e Mitólogo de Ficção de Alta Fantasia D&D 5e (Gemini Flash-Lite). \
Sua missão inegociável é escrever uma GRANDE INTRODUÇÃO HISTÓRICA E WORLDBUILDING ÉPICO DO MUNDO (um texto longo, cadenciado, denso e substancial, com 6 a 8 parágrafos profundos). \
O leitor PRECISA entender a totalidade do mundo, seu mapa geopolítico, a sociedade, as lendas ancestrais e a decadência das eras ANTES de qualquer aproximação dos heróis. \
NUNCA comece focando direto em uma caverna, emboscada ou masmorra rasa. Comece com a magnitude do mundo e a história pregressa dos continentes! \
\
ESTRUTURA OBRIGATÓRIA E SEQUENCIAL DO WORLDBUILDING: \
1. O CONTINENTE, A GEOPOLÍTICA E AS ERAS PASSADAS: \
   Dê nome ao reino ou continente. Estabeleça a história ancestral: quem governava este mundo há séculos ou milênios? Quais impérios prosperaram, quais deuses ou entidades cósmicas caminharam sobre estas terras e que catástrofe ou cisão quebrou a era dourada deixando as cicatrizes que moldam a geografia e as fronteiras políticas atuais. \
2. A SOCIEDADE, AS CIDADES E A TENSÃO COTIDIANA: \
   Explique a estrutura social e a geopolítica do presente: como os reinos, cortes decadentes, guildas mercantes e santuários religiosos coexistem ou rivalizam. O que o povo comum nas tabernas e vilarejos sussurra com medo? Qual é a atmosfera de paranoia, declínio econômico, praga espiritual ou tirania que corrói os alicerces da civilização? \
3. A HISTÓRIA OCULTA POR TRÁS DA PREMISSA & AS FORÇAS EM MOVIMENTO: \
   Aprofunde a premissa fornecida: não a resuma, mas expanda o lore do mundo ao redor dela. Quais forças antagônicas, pactos esquecidos, conspirações da nobreza ou poderes sombrios estão tramando nas sombras? Por que os exércitos ou regentes falharam em conter a corrupção? \
4. A COMITIVA DOS HERÓIS NA TAPEÇARIA DO MUNDO: \
   Introduza nominalmente CADA UM dos heróis da comitiva fornecidos (com suas respectivas classes e papéis). Explique como suas origens no reino, seus juramentos, suas feridas pregressas ou seus laços de honra colidiram com o destino do continente. Por que este grupo específico de aventureiros aceitou carregar o fardo desta saga quando outros recuaram? \
5. O LIMIAR DO ATO I — A CHEGADA AO PALCO DA CRISE: \
   Somente agora, com a grandiosidade e a tragédia do mundo perfeitamente estabelecidas, a câmera narrativa desce para o cenário imediato do Ato I. Descreva o peso do silêncio, o clima implacável e o momento tenso em que os heróis cruzam a fronteira e avistam os primeiros indícios diretos da ameaça iminente. \
\
DIRETRIZ DE ESTILO: \
Escreva prosa literária de alta densidade em português brasileiro culto, rica em vocabulário sensorial, metáforas mitológicas, cadência épica e maturidade dramática. NUNCA use quebras metalinguísticas, notas de rodapé, tópicos numerados ou saudações. Entregue uma verdadeira crônica de abertura de livro de fantasia épica.";

    let current_act_number = hidden_skeleton
        .current_act()
        .map(|a| a.act_number)
        .unwrap_or(1);
    let current_act_title = hidden_skeleton
        .current_act()
        .map(|a| a.title.clone())
        .unwrap_or_else(|| "Ato I: O Despertar".to_string());
    let current_act_synopsis = hidden_skeleton
        .current_act()
        .map(|a| a.synopsis.clone())
        .unwrap_or_else(|| req.premise.clone());

    let mut party_summary = String::new();
    for slot in &player_slots {
        party_summary.push_str(&format!("- {} (Classe: {})\n", slot.player_name, slot.class_name));
    }

    let prologue_prompt = format!(
        "LORE COMPLETO & PREMISSA DA CAMPANHA:\n{}\n\nESTILO / TOM LITERÁRIO:\n{}\n\nPRIMEIRO ATO PLANEJADO: {} — {}\n\nCOMITIVA DE AVENTUREIROS (Nomear e integrar nominalmente TODOS):\n{}\n\nEscreva agora a Grande Crônica de Abertura e Worldbuilding do Mundo, contextualizando profundamente a história do reino, a geopolítica e as eras passadas antes de convergir para a chegada dos heróis ao Primeiro Ato.",
        req.premise,
        req.literary_style,
        current_act_title,
        current_act_synopsis,
        party_summary
    );

    let party_names = player_slots.iter().map(|s| s.player_name.as_str()).collect::<Vec<_>>().join(", ");
    let dense_prologue = match state.skeleton_agent.gemini.generate_narrative(&prologue_prompt, prologue_instruction).await {
        Ok(text) if !text.trim().is_empty() => text,
        _ => format!(
            "Nas eras em que os primeiros impérios ergueram suas torres de mármore e ferro sobre estas terras, o equilíbrio entre as linhagens mortais e as forças ancestrais era guardado por juramentos sagrados. Séculos se passaram desde a Grande Ruptura, e as fronteiras dos reinos outrora gloriosos agora sangram sob o peso da decadência política, da ganância dos nobres e dos rumores de maquinações ocultas que operam nas sombras das cortes.\n\n\
Hoje, o povo comum vive à sombra de um medo tácito. As estradas mercantes tornaram-se traiçoeiras, os templos ecoam preces silenciadas e os governantes ignoram os sinais celestiais de que uma tempestade irrevogável se aproxima. É neste cenário de tensão e fragilidade que os eventos de '{}' ganham forma, impulsionados pela premissa que ecoa pelos vales: {}.\n\n\
Perante este abismo, uma comitiva improvável foi convocada pelos desígnios do destino. Os heróis {} carregam o peso de suas próprias convicções, classes e legados, unindo forças em um juramento silencioso para desvendar os segredos e enfrentar as trevas que os exércitos do reino não ousam confrontar.\n\n\
Agora, ao cruzarem os limites conhecidos da civilização e ingressarem no limiar de '{}', o vento frio sopra sobre suas lâminas e mantos. O mundo ao redor pode ignorar o que está prestes a se desatar, mas para este grupo de aventureiros, o primeiro passo da jornada acaba de ser dado.",
            current_act_title,
            req.premise,
            party_names,
            current_act_title
        )
    };

    campaign.world_prologue = Some(dense_prologue.clone());
    if let Some(ref mut m) = campaign.manifest {
        m.world_prologue = Some(dense_prologue.clone());
    }
    let _ = campaign.save_manifest();

    if let Ok(mut chronicle) = campaign.get_chronicle_book() {
        let mut initial_entry = crate::narrative::chronicle::ChronicleEntry::new(
            current_act_number,
            &format!("Prólogo — Ato {}: {}", current_act_number, current_act_title),
            crate::narrative::chronicle::ChronicleEntryType::LiteraryChapter,
            &dense_prologue,
            Vec::new(),
            player_ids.clone(),
        );
        let _ = chronicle.append_entry(&mut initial_entry);
    }

    let existing_players = campaign
        .get_entity_store()
        .and_then(|s| s.list_player_entities())
        .unwrap_or_default();

    let mut initial_participants = Vec::new();
    for (i, slot) in player_slots.iter().enumerate() {
        initial_participants.push(build_player_participant(
            &slot.player_id,
            &slot.player_name,
            &slot.class_name,
            i,
            &existing_players,
        ));
    }

    let mut first_scene = state
        .environment_agent
        .generate_dynamic_scene(
            &campaign_id,
            &enriched_premise,
            &req.literary_style,
            current_act_number,
            &current_act_title,
            &current_act_synopsis,
            &hidden_skeleton.current_act().map(|a| a.expected_milestones.clone()).unwrap_or_default(),
            &dense_prologue,
            initial_participants,
        )
        .await;

    let world_entities = campaign
        .get_entity_store()
        .and_then(|s| s.list_world_entities())
        .unwrap_or_default();

    let npc_participants = expand_and_build_npc_participants(
        &first_scene.active_npcs,
        current_act_number,
        &effective_monsters,
        &hidden_skeleton.homebrew_bosses,
        &world_entities,
    );

    if let Ok(store) = campaign.get_entity_store() {
        for np in &npc_participants {
            if store.get_world_entity(&np.id).ok().flatten().is_none() {
                let mut sheet = WorldEntitySheet::new(
                    &np.id,
                    &np.name,
                    EntityType::Npc,
                    &format!("Combatente na cena '{}' ({})", first_scene.title, first_scene.location),
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

    first_scene.participants.extend(npc_participants);
    ensure_tactical_scene_hydrated(&mut first_scene, &existing_players);

    campaign.active_scene = Some(first_scene);
    let _ = campaign.save_active_scene();

    let arc_camp = Arc::new(RwLock::new(campaign));
    state.rooms.write().await.insert(room_code.clone(), arc_camp.clone());
    state.campaigns.write().await.insert(campaign_id.clone(), arc_camp);

    let (tx, _) = broadcast::channel(100);
    state.channels.write().await.insert(room_code.clone(), tx);

    println!(
        ">>> [CAMPAIGN] Campanha criada com sucesso! ID: {}, RoomCode: {}, Grupo: {} heróis, Ato 1: {}",
        campaign_id, room_code, player_slots.len(), current_act_title
    );

    (
        StatusCode::CREATED,
        Json(json!({
            "status": "created",
            "campaign_id": campaign_id,
            "room_code": room_code,
            "player_id": host_id,
            "player_number": 1,
            "player_code": host_code,
            "player_slots": player_slots,
            "is_host": true,
            "current_act_number": current_act_number,
            "current_act_title": current_act_title,
            "message": "Campanha iniciada com grupo completo e esqueleto narrativo oculto no servidor."
        })),
    )
}

pub async fn join_campaign(
    State(state): State<ServerState>,
    Json(req): Json<JoinCampaignRequest>,
) -> (StatusCode, Json<Value>) {
    let req_room_code = req.room_code.trim().to_uppercase();

    let mut campaign_arc = if let Some(ref req_camp_id) = req.campaign_id {
        get_or_load_campaign(&state, req_camp_id).await
    } else {
        None
    };

    if campaign_arc.is_none() {
        campaign_arc = get_or_load_campaign_by_room(&state, &req_room_code).await;
    }

    let campaign_arc = match campaign_arc {
        Some(c) => c,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "status": "error",
                    "message": format!("Campanha ou código de sala '{}' não encontrado", req_room_code)
                })),
            )
        }
    };

    let mut camp = campaign_arc.write().await;

    if let Some(ref req_camp_id) = req.campaign_id {
        if camp.campaign_id != *req_camp_id {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "O UUID da campanha informado não confere com o código da sala."
                })),
            );
        }
    }

    let campaign_id = camp.campaign_id.clone();
    let room_code = camp.room_code.clone();

    if let Some(ref code) = req.player_code {
        let clean_code = code.trim().to_uppercase();
        let existing_slot = camp.manifest.as_ref().and_then(|m| {
            m.player_slots.iter().find(|s| s.player_code.to_uppercase() == clean_code).cloned()
        });
        if let Some(slot_clone) = existing_slot {
            let all_slots = camp.manifest.as_ref().map(|m| m.player_slots.clone()).unwrap_or_default();
            return (
                StatusCode::OK,
                Json(json!({
                    "status": "rejoined",
                    "campaign_id": campaign_id,
                    "room_code": room_code,
                    "player_id": slot_clone.player_id,
                    "player_number": slot_clone.player_number,
                    "player_code": slot_clone.player_code,
                    "player_slots": all_slots,
                    "is_host": slot_clone.is_host,
                    "message": format!("Reconexão bem-sucedida como Jogador #{} ({}).", slot_clone.player_number, slot_clone.player_name)
                })),
            );
        }
    }

    let is_single_term = req.is_single_terminal.unwrap_or(false);
    if let Some(ref m) = camp.manifest {
        if !m.player_slots.is_empty() && (is_single_term || m.player_slots.len() >= 6) {
            let first_slot = m.player_slots[0].clone();
            let all_slots = m.player_slots.clone();
            return (
                StatusCode::OK,
                Json(json!({
                    "status": "rejoined",
                    "campaign_id": campaign_id,
                    "room_code": room_code,
                    "player_id": first_slot.player_id,
                    "player_number": first_slot.player_number,
                    "player_code": first_slot.player_code,
                    "player_slots": all_slots,
                    "is_host": first_slot.is_host,
                    "message": format!("Retomando sessão na sala #{} como Jogador #{} ({}).", room_code, first_slot.player_number, first_slot.player_name)
                })),
            );
        }
    }

    let (next_player_number, new_player_id, new_player_code, all_slots) = {
        let m = match camp.manifest.as_mut() {
            Some(m) => m,
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "status": "error", "message": "Manifesto da campanha não encontrado" })),
                )
            }
        };

        if m.player_slots.len() >= 6 {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "status": "error", "message": "Sala cheia (capacidade máxima de 6 jogadores atingida)" })),
            );
        }

        let next_player_number = m.player_slots.len() + 1;
        let new_player_id = format!("player_guest_{}_{}", next_player_number, uuid::Uuid::new_v4().to_string()[..6].to_string());
        let new_player_code = generate_4letter_player_code();

        let new_slot = PlayerSlot {
            player_number: next_player_number,
            player_id: new_player_id.clone(),
            player_name: req.player_name.clone(),
            class_name: req.class_name.clone(),
            player_code: new_player_code.clone(),
            is_host: false,
        };

        m.player_slots.push(new_slot);
        m.player_ids.push(new_player_id.clone());
        (next_player_number, new_player_id, new_player_code, m.player_slots.clone())
    };

    let _ = camp.save_manifest();

    if let Ok(store) = camp.get_entity_store() {
        let mut player_sheet = WorldEntitySheet::new(
            &new_player_id,
            &req.player_name,
            EntityType::Player,
            &format!("Jogador #{} ({})", next_player_number, req.class_name),
        );
        player_sheet.dnd_stats = Some(EntityDndStats::default());
        let _ = store.save_player_entity(&player_sheet);
    }

    let total_players = all_slots.len();

    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&room_code) {
        let _ = tx.send(
            json!({
                "type": "PLAYER_JOINED",
                "player_id": new_player_id,
                "player_number": next_player_number,
                "player_name": req.player_name,
                "class_name": req.class_name,
                "total_players": total_players,
                "player_slots": all_slots
            })
            .to_string(),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "joined",
            "campaign_id": campaign_id,
            "room_code": room_code,
            "player_id": new_player_id,
            "player_number": next_player_number,
            "player_code": new_player_code,
            "player_slots": all_slots,
            "is_host": false,
            "message": format!("Bem-vindo à sala! Você é o Jogador #{} ({}) com código de acesso: {}.", next_player_number, req.player_name, new_player_code)
        })),
    )
}

pub async fn get_campaign_slots(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    let camp_arc = match get_or_load_campaign(&state, &campaign_id).await {
        Some(c) => c,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "status": "error", "message": "Campanha não encontrada" })),
            )
        }
    };

    let camp = camp_arc.read().await;
    let slots = camp.manifest.as_ref().map(|m| m.player_slots.clone()).unwrap_or_default();
    (
        StatusCode::OK,
        Json(json!({
            "campaign_id": camp.campaign_id,
            "room_code": camp.room_code,
            "player_slots": slots
        })),
    )
}

pub async fn add_local_player(
    State(state): State<ServerState>,
    Json(req): Json<AddLocalPlayerRequest>,
) -> (StatusCode, Json<Value>) {
    let camp_arc = match get_or_load_campaign(&state, &req.campaign_id).await {
        Some(c) => c,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "status": "error", "message": "Campanha não encontrada" })),
            )
        }
    };

    let mut camp = camp_arc.write().await;
    let campaign_id = camp.campaign_id.clone();
    let room_code = camp.room_code.clone();

    let (player_num, new_id, new_code, all_slots) = {
        let m = match camp.manifest.as_mut() {
            Some(m) => m,
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "status": "error", "message": "Manifesto ausente" })),
                )
            }
        };

        if m.player_slots.len() >= 6 {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "status": "error", "message": "Limite de 6 jogadores no terminal atingido" })),
            );
        }

        let player_num = m.player_slots.len() + 1;
        let new_id = format!("player_local_{}_{}", player_num, uuid::Uuid::new_v4().to_string()[..6].to_string());
        let new_code = generate_4letter_player_code();

        let slot = PlayerSlot {
            player_number: player_num,
            player_id: new_id.clone(),
            player_name: req.player_name.clone(),
            class_name: req.class_name.clone(),
            player_code: new_code.clone(),
            is_host: false,
        };

        m.player_slots.push(slot);
        m.player_ids.push(new_id.clone());
        (player_num, new_id, new_code, m.player_slots.clone())
    };

    let _ = camp.save_manifest();

    if let Ok(store) = camp.get_entity_store() {
        let mut player_sheet = WorldEntitySheet::new(
            &new_id,
            &req.player_name,
            EntityType::Player,
            &format!("Jogador local #{}: {}", player_num, req.class_name),
        );
        player_sheet.dnd_stats = Some(EntityDndStats::default());
        let _ = store.save_player_entity(&player_sheet);
    }

    (
        StatusCode::CREATED,
        Json(json!({
            "status": "created",
            "campaign_id": campaign_id,
            "room_code": room_code,
            "player_id": new_id,
            "player_number": player_num,
            "player_code": new_code,
            "player_slots": all_slots
        })),
    )
}

pub async fn get_campaign_state(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    let camp_arc = match get_or_load_campaign(&state, &campaign_id).await {
        Some(c) => c,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "status": "error", "message": "Campanha não encontrada" })),
            )
        }
    };

    {
        let mut camp_w = camp_arc.write().await;
        hydrate_campaign_active_scene(&mut camp_w);
    }

    let camp = camp_arc.read().await;
    let manifest = camp.manifest.as_ref();

    let current_act_title = manifest
        .and_then(|m| m.hidden_skeleton.current_act())
        .map(|a| a.title.clone())
        .unwrap_or_else(|| "Ato Inicial".to_string());

    let act_number = manifest
        .and_then(|m| m.hidden_skeleton.current_act())
        .map(|a| a.act_number)
        .unwrap_or(1);

    let world_prologue = camp
        .world_prologue
        .clone()
        .or_else(|| manifest.and_then(|m| m.world_prologue.clone()))
        .or_else(|| {
            camp.get_chronicle_book()
                .ok()
                .and_then(|book| book.get_all_entries().ok())
                .and_then(|entries| {
                    entries
                        .into_iter()
                        .find(|e| e.scene_title.to_lowercase().contains("prólogo"))
                        .map(|e| e.narrative_prose)
                })
        });

    let player_entities = camp
        .get_entity_store()
        .and_then(|s| s.list_player_entities())
        .unwrap_or_default();

    (
        StatusCode::OK,
        Json(json!({
            "campaign_id": camp.campaign_id,
            "room_code": camp.room_code,
            "premise": manifest.map(|m| m.premise.clone()).unwrap_or_default(),
            "literary_style": manifest.map(|m| m.literary_style.clone()).unwrap_or_default(),
            "host_player_id": manifest.map(|m| m.host_player_id.clone()).unwrap_or_default(),
            "player_ids": manifest.map(|m| m.player_ids.clone()).unwrap_or_default(),
            "current_act_number": act_number,
            "current_act_title": current_act_title,
            "world_prologue": world_prologue,
            "active_scene": camp.active_scene,
            "player_entities": player_entities,
        })),
    )
}

pub async fn get_campaign_chronicle(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    let camp_arc = match get_or_load_campaign(&state, &campaign_id).await {
        Some(c) => c,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "status": "error", "message": "Campanha não encontrada" })),
            )
        }
    };

    let camp = camp_arc.read().await;
    let entries = match camp.get_chronicle_book() {
        Ok(book) => book.get_all_entries().unwrap_or_default(),
        Err(_) => Vec::new(),
    };

    (StatusCode::OK, Json(json!({ "entries": entries })))
}

pub async fn get_campaign_entities(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    let camp_arc = match get_or_load_campaign(&state, &campaign_id).await {
        Some(c) => c,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "status": "error", "message": "Campanha não encontrada" })),
            )
        }
    };

    let camp = camp_arc.read().await;
    let world_entities = camp.get_entity_store().and_then(|s| s.list_world_entities()).unwrap_or_default();
    let player_entities = camp.get_entity_store().and_then(|s| s.list_player_entities()).unwrap_or_default();

    (
        StatusCode::OK,
        Json(json!({
            "world_entities": world_entities,
            "player_entities": player_entities
        })),
    )
}

pub async fn sync_campaign_party(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(req): Json<SyncPartyRequest>,
) -> (StatusCode, Json<Value>) {
    let camp_arc = match get_or_load_campaign(&state, &campaign_id).await {
        Some(c) => c,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "status": "error", "message": "Campanha não encontrada" })),
            )
        }
    };

    let mut camp = camp_arc.write().await;
    let room_code = camp.room_code.clone();

    let mut updated_slots = Vec::new();
    let mut updated_ids = Vec::new();
    let existing_slots = camp
        .manifest
        .as_ref()
        .map(|m| m.player_slots.clone())
        .unwrap_or_default();

    for (idx, hero) in req.party.iter().enumerate() {
        let player_num = idx + 1;
        let hero_name = hero["name"].as_str().unwrap_or("Aventureiro").to_string();
        let hero_class = hero["className"].as_str().unwrap_or("Fighter").to_string();

        let (player_id, player_code, is_host) = if let Some(existing) = existing_slots.get(idx) {
            (
                existing.player_id.clone(),
                existing.player_code.clone(),
                existing.is_host,
            )
        } else {
            let new_id = format!(
                "player_{}_{}",
                player_num,
                uuid::Uuid::new_v4().to_string()[..6].to_string()
            );
            let new_code = generate_4letter_player_code();
            (new_id, new_code, idx == 0)
        };

        updated_slots.push(PlayerSlot {
            player_number: player_num,
            player_id: player_id.clone(),
            player_name: hero_name,
            class_name: hero_class,
            player_code,
            is_host,
        });
        updated_ids.push(player_id);
    }

    if let Some(ref mut m) = camp.manifest {
        if !updated_ids.is_empty() {
            m.host_player_id = updated_ids[0].clone();
            m.player_ids = updated_ids.clone();
            m.player_slots = updated_slots.clone();
        }
    }
    let _ = camp.save_manifest();

    let mut saved_entities = Vec::new();
    if let Ok(store) = camp.get_entity_store() {
        let _ = store.clear_player_entities();
        for (idx, hero) in req.party.iter().enumerate() {
            let slot = &updated_slots[idx];
            let sheet = build_player_entity_from_sheet(&slot.player_id, slot.player_number, hero);
            let _ = store.save_player_entity(&sheet);
            saved_entities.push(sheet);
        }
    }

    if let Some(ref mut scene) = camp.active_scene {
        let mut player_participants: Vec<crate::narrative::living_scene::TurnParticipant> = Vec::new();
        for (idx, slot) in updated_slots.iter().enumerate() {
            let mut built = build_player_participant(
                &slot.player_id,
                &slot.player_name,
                &slot.class_name,
                idx,
                &saved_entities,
            );
            if let Some(prev) = scene.participants.iter().find(|p| p.id == slot.player_id) {
                if prev.initiative_roll > 0 {
                    built.initiative_roll = prev.initiative_roll;
                    built.initiative = built.initiative_roll + built.initiative_modifier;
                }
                if let Some((px, py)) = prev.position {
                    if px >= 5 && py >= 4 {
                        built.position = Some((px, py));
                    }
                }
            }
            player_participants.push(built);
        }
        let non_players: Vec<crate::narrative::living_scene::TurnParticipant> = scene
            .participants
            .iter()
            .filter(|p| !p.is_player)
            .cloned()
            .collect();
        let mut combined = player_participants;
        combined.extend(non_players);
        combined.sort_by(|a, b| b.initiative.cmp(&a.initiative));
        scene.participants = combined;
        ensure_tactical_scene_hydrated(scene, &saved_entities);
        let _ = camp.save_active_scene();
    }

    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&room_code) {
        let _ = tx.send(
            json!({
                "type": "PARTY_SYNCED",
                "player_slots": updated_slots,
                "player_entities": saved_entities
            })
            .to_string(),
        );
    }

    println!(
        ">>> [BACKEND] Party sincronizada na campanha {}: {} herói(s)",
        campaign_id,
        updated_slots.len()
    );

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "campaign_id": campaign_id,
            "player_slots": updated_slots,
            "player_entities": saved_entities
        })),
    )
}

pub async fn delete_saved_campaign(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    let dir = format!("data/campaigns/{}", campaign_id);
    let path = std::path::Path::new(&dir);

    let room_code = {
        let mut campaigns = state.campaigns.write().await;
        if let Some(camp_arc) = campaigns.remove(&campaign_id) {
            let camp = camp_arc.read().await;
            Some(camp.room_code.clone())
        } else {
            None
        }
    };

    if let Some(ref r_code) = room_code {
        state.rooms.write().await.remove(r_code);
        state.channels.write().await.remove(r_code);
    }

    if path.exists() {
        if let Err(e) = std::fs::remove_dir_all(&dir) {
            eprintln!("[CAMPAIGN_DELETE] Erro ao remover diretório {}: {}", dir, e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": format!("Erro ao excluir pasta da campanha: {}", e) })),
            );
        }
    } else if room_code.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "status": "error", "message": "Campanha não encontrada" })),
        );
    }

    println!(">>> [CAMPAIGN] Campanha excluída permanentemente: ID={}", campaign_id);

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "campaign_id": campaign_id,
            "message": "Campanha excluída permanentemente com sucesso."
        })),
    )
}

pub async fn list_saved_campaigns(State(_state): State<ServerState>) -> (StatusCode, Json<Value>) {
    let dir = "data/campaigns";
    let _ = std::fs::create_dir_all(dir);

    let mut list = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let camp_id = entry.file_name().to_string_lossy().to_string();
                let manifest_path = format!("{}/{}/manifest.json", dir, camp_id);
                if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                    if let Ok(manifest) = serde_json::from_str::<CampaignManifest>(&content) {
                        let active_scene_path = format!("{}/{}/active_scene.json", dir, camp_id);
                        let has_active_scene = std::path::Path::new(&active_scene_path).exists();
                        list.push(json!({
                            "campaign_id": manifest.campaign_id,
                            "room_code": manifest.room_code,
                            "premise": manifest.premise,
                            "literary_style": manifest.literary_style,
                            "current_act_number": manifest.hidden_skeleton.current_act().map(|a| a.act_number).unwrap_or(1),
                            "current_act_title": manifest.hidden_skeleton.current_act().map(|a| a.title.clone()).unwrap_or_else(|| "Ato I".to_string()),
                            "player_ids": manifest.player_ids,
                            "created_at": manifest.created_at,
                            "has_active_scene": has_active_scene
                        }));
                    }
                }
            }
        }
    }

    (StatusCode::OK, Json(json!({ "campaigns": list })))
}

pub async fn export_campaign_bundle(
    Path(campaign_id): Path<String>,
    State(_state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    let dir = format!("data/campaigns/{}", campaign_id);
    let manifest_path = format!("{}/manifest.json", dir);

    let manifest = match std::fs::read_to_string(&manifest_path) {
        Ok(c) => match serde_json::from_str::<Value>(&c) {
            Ok(v) => v,
            Err(_) => return (StatusCode::NOT_FOUND, Json(json!({ "status": "error", "message": "Manifesto inválido" }))),
        },
        Err(_) => return (StatusCode::NOT_FOUND, Json(json!({ "status": "error", "message": "Campanha não encontrada" }))),
    };

    let active_scene = std::fs::read_to_string(format!("{}/active_scene.json", dir))
        .ok()
        .and_then(|c| serde_json::from_str::<Value>(&c).ok());

    let mut chronicle_entries = Vec::new();
    if let Ok(c) = std::fs::read_to_string(format!("{}/chronicle_log.jsonl", dir)) {
        for line in c.lines() {
            if let Ok(entry) = serde_json::from_str::<Value>(line) {
                chronicle_entries.push(entry);
            }
        }
    }

    let mut players = Vec::new();
    let players_dir = format!("{}/entities/players", dir);
    if let Ok(entries) = std::fs::read_dir(&players_dir) {
        for entry in entries.flatten() {
            if let Ok(c) = std::fs::read_to_string(entry.path()) {
                if let Ok(val) = serde_json::from_str::<Value>(&c) {
                    players.push(val);
                }
            }
        }
    }

    let mut world_entities = Vec::new();
    let world_dir = format!("{}/entities/world", dir);
    if let Ok(entries) = std::fs::read_dir(&world_dir) {
        for entry in entries.flatten() {
            if let Ok(c) = std::fs::read_to_string(entry.path()) {
                if let Ok(val) = serde_json::from_str::<Value>(&c) {
                    world_entities.push(val);
                }
            }
        }
    }

    let bundle = json!({
        "format": "rpg_engine_campaign_backup_v1",
        "exported_at": chrono::Utc::now().to_rfc3339(),
        "campaign_id": campaign_id,
        "manifest": manifest,
        "active_scene": active_scene,
        "chronicle_entries": chronicle_entries,
        "players": players,
        "world_entities": world_entities
    });

    (StatusCode::OK, Json(bundle))
}

pub async fn import_campaign_bundle(
    State(state): State<ServerState>,
    Json(bundle): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let manifest_val = if bundle.get("manifest").is_some() {
        &bundle["manifest"]
    } else if bundle.get("campaign_id").is_some() || bundle.get("premise").is_some() {
        &bundle
    } else {
        return (StatusCode::BAD_REQUEST, Json(json!({ "status": "error", "message": "Arquivo de backup inválido: falta manifesto ou identificador da campanha" })));
    };

    let campaign_id = manifest_val["campaign_id"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let room_code = manifest_val["room_code"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            format!("{:04}", rng.gen_range(1000..9999))
        });

    let base_dir = format!("data/campaigns/{}", campaign_id);
    let players_dir = format!("{}/entities/players", base_dir);
    let world_dir = format!("{}/entities/world", base_dir);

    if let Err(e) = std::fs::create_dir_all(&players_dir) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "status": "error", "message": e.to_string() })));
    }
    if let Err(e) = std::fs::create_dir_all(&world_dir) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "status": "error", "message": e.to_string() })));
    }

    let _ = std::fs::write(
        format!("{}/manifest.json", base_dir),
        serde_json::to_string_pretty(manifest_val).unwrap_or_default(),
    );

    if let Some(scene) = bundle.get("active_scene").filter(|s| !s.is_null()) {
        let _ = std::fs::write(
            format!("{}/active_scene.json", base_dir),
            serde_json::to_string_pretty(scene).unwrap_or_default(),
        );
    }

    if let Some(entries) = bundle["chronicle_entries"].as_array() {
        let mut log_content = String::new();
        for entry in entries {
            if let Ok(line) = serde_json::to_string(entry) {
                log_content.push_str(&line);
                log_content.push('\n');
            }
        }
        let _ = std::fs::write(format!("{}/chronicle_log.jsonl", base_dir), log_content);
    }

    if let Some(players) = bundle["players"].as_array() {
        for p in players {
            if let Some(pid) = p["id"].as_str() {
                let _ = std::fs::write(
                    format!("{}/{}.json", players_dir, pid),
                    serde_json::to_string_pretty(p).unwrap_or_default(),
                );
            }
        }
    }

    if let Some(entities) = bundle["world_entities"].as_array() {
        for e in entities {
            if let Some(eid) = e["id"].as_str() {
                let _ = std::fs::write(
                    format!("{}/{}.json", world_dir, eid),
                    serde_json::to_string_pretty(e).unwrap_or_default(),
                );
            }
        }
    }

    let mut camp = CampaignState::with_id_and_room(&campaign_id, &room_code);
    let _ = camp.load_manifest();
    let _ = camp.load_active_scene();
    let arc_camp = Arc::new(RwLock::new(camp));

    state.rooms.write().await.insert(room_code.clone(), arc_camp.clone());
    state.campaigns.write().await.insert(campaign_id.clone(), arc_camp);

    let mut channels = state.channels.write().await;
    channels.entry(room_code.clone()).or_insert_with(|| broadcast::channel(100).0);

    println!(
        ">>> [CAMPAIGN] Campanha importada com sucesso: ID={}, Sala=#{}",
        campaign_id, room_code
    );

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "campaign_id": campaign_id,
            "room_code": room_code,
            "message": "Campanha restaurada e sala aberta com sucesso."
        })),
    )
}

pub async fn list_saved_premises() -> (StatusCode, Json<Value>) {
    let dir = "data/premises";
    let _ = std::fs::create_dir_all(dir);

    let mut list = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(val) = serde_json::from_str::<Value>(&content) {
                        list.push(val);
                    }
                }
            }
        }
    }

    (StatusCode::OK, Json(json!({ "premises": list })))
}

pub async fn send_party_chat(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(req): Json<SendChatMessageRequest>,
) -> (StatusCode, Json<Value>) {
    let camp_arc = match get_or_load_campaign(&state, &campaign_id).await {
        Some(c) => c,
        None => return (StatusCode::NOT_FOUND, Json(json!({ "status": "error", "message": "Campanha não encontrada" }))),
    };
    let room_code = camp_arc.read().await.room_code.clone();

    let chat_msg = json!({
        "type": "CHAT_MESSAGE",
        "sender_id": req.sender_id,
        "sender_name": req.sender_name,
        "text": req.text,
        "is_action": req.is_action,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&room_code) {
        let _ = tx.send(chat_msg.to_string());
    }

    (StatusCode::OK, Json(json!({ "status": "sent" })))
}

pub async fn get_campaign_audit(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    let camp_arc = match get_or_load_campaign(&state, &campaign_id).await {
        Some(c) => c,
        None => return (StatusCode::NOT_FOUND, Json(json!({ "status": "error", "message": "Campanha não encontrada" }))),
    };

    let camp = camp_arc.read().await;
    let manifest = camp.manifest.clone();
    let active_scene = camp.active_scene.clone();

    let chronicle_entries = match camp.get_chronicle_book() {
        Ok(book) => book.get_all_entries().unwrap_or_default(),
        Err(_) => Vec::new(),
    };

    let world_entities = camp.get_entity_store().and_then(|s| s.list_world_entities()).unwrap_or_default();
    let player_entities = camp.get_entity_store().and_then(|s| s.list_player_entities()).unwrap_or_default();

    (
        StatusCode::OK,
        Json(json!({
            "campaign_id": camp.campaign_id,
            "room_code": camp.room_code,
            "manifest": manifest,
            "active_scene": active_scene,
            "chronicle_entries": chronicle_entries,
            "world_entities": world_entities,
            "player_entities": player_entities,
        })),
    )
}
