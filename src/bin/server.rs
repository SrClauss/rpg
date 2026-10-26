use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post, delete},
    Router,
};
use dotenvy::dotenv;
use futures_util::{SinkExt, StreamExt};
use rpg_engine::auth::{UserPublic, UserStore};
use rpg_engine::llm::{
    EnvironmentAgent, NarrativeOrchestrator, SceneCloser, SkeletonAgent,
};
use rpg_engine::narrative::chronicle::{ChronicleEntry, ChronicleEntryType};
use rpg_engine::narrative::entity_store::{EntityDndStats, EntityType, WorldEntitySheet};
use rpg_engine::narrative::living_scene::{
    ActionsBudget, LivingScene, SceneObjective, SceneResolutionReason, TurnParticipant,
};
use rpg_engine::state::{
    generate_4letter_player_code, generate_4letter_room_code, CampaignManifest, CampaignState,
    PlayerSlot,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct ServerState {
    rooms: Arc<RwLock<HashMap<String, Arc<RwLock<CampaignState>>>>>, // room_code -> CampaignState
    campaigns: Arc<RwLock<HashMap<String, Arc<RwLock<CampaignState>>>>>, // campaign_id -> CampaignState
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<String>>>>, // room_code -> broadcast::Sender
    skeleton_agent: Arc<SkeletonAgent>,
    environment_agent: Arc<EnvironmentAgent>,
    orchestrator: Arc<NarrativeOrchestrator>,
    user_store: Arc<UserStore>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnalyzePremiseRequest {
    premise: String,
    #[serde(default = "default_literary_style")]
    literary_style: String,
    #[serde(default = "default_creative_freedom")]
    ai_creative_freedom: String,
    #[serde(default)]
    dramatic_focus_slider: Option<u32>,
}

fn default_literary_style() -> String {
    "Fantasia Sombria".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateCampaignRequest {
    premise: String,
    literary_style: String,
    #[serde(default = "default_player_name")]
    host_player_name: String,
    #[serde(default = "default_class_name")]
    host_class: String,
    #[serde(default)]
    custom_monsters: Vec<Value>,
    #[serde(default = "default_creative_freedom")]
    ai_creative_freedom: String,
    #[serde(default = "default_story_focus")]
    story_focus: String,
    #[serde(default)]
    dramatic_focus_slider: Option<u32>,
    #[serde(default)]
    party: Option<Vec<Value>>,
}

fn default_player_name() -> String {
    "Aventureiro".to_string()
}

fn default_class_name() -> String {
    "Fighter".to_string()
}

fn default_creative_freedom() -> String {
    "balanced".to_string()
}

fn default_story_focus() -> String {
    "d20".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
struct JoinCampaignRequest {
    #[serde(default)]
    campaign_id: Option<String>,
    room_code: String,
    #[serde(default = "default_player_name")]
    player_name: String,
    #[serde(default = "default_class_name")]
    class_name: String,
    #[serde(default)]
    player_code: Option<String>,
    #[serde(default)]
    is_single_terminal: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddLocalPlayerRequest {
    campaign_id: String,
    #[serde(default = "default_player_name")]
    player_name: String,
    #[serde(default = "default_class_name")]
    class_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct StartSceneRequest {
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    location: Option<String>,
    #[serde(default)]
    objectives: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClaimItemRequest {
    item_id: String,
    #[serde(default = "default_player_name")]
    player_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SceneMoveRequest {
    actor_id: String,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct EndTurnRequest {
    #[serde(default)]
    actor_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TurnActionRequest {
    via_type: String, // "via1", "via2", "via3"
    action_id: Option<String>,
    player_text: Option<String>,
    character_name: String,
    class_name: String,
    actor_id: Option<String>,
    campaign_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TurnActionResponse {
    status: String,
    requires_mechanic: bool,
    detected_intent: String,
    roll_result: Option<Value>,
    narrative_feedback: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("============================================================");
    println!("   RPG D&D 5E ENGINE — SERVIDOR MULTIPLAYER (AXUM + WS)    ");
    println!("============================================================");

    let gemini_key = env::var("GEMINI_API_KEY").unwrap_or_else(|_| "dummy_key".to_string());
    let deepseek_key = env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| "dummy_key".to_string());

    let skeleton_agent = Arc::new(SkeletonAgent::new(gemini_key.clone()));
    let environment_agent = Arc::new(EnvironmentAgent::new(gemini_key.clone()));
    let orchestrator = Arc::new(NarrativeOrchestrator::new(gemini_key, deepseek_key));
    let user_store = Arc::new(UserStore::new("data/users"));

    let server_state = ServerState {
        rooms: Arc::new(RwLock::new(HashMap::new())),
        campaigns: Arc::new(RwLock::new(HashMap::new())),
        channels: Arc::new(RwLock::new(HashMap::new())),
        skeleton_agent,
        environment_agent,
        orchestrator,
        user_store,
    };

    // Cria sala padrão de demonstração / legado para testes diretos
    {
        let mut default_camp = CampaignState::with_id_and_room("default_campaign", "0000");
        default_camp.start_new_beat(
            "A Cripta dos Reis Esquecidos",
            rpg_engine::narrative::beat::BeatType::Exploration,
        );
        let arc_camp = Arc::new(RwLock::new(default_camp));
        server_state.rooms.write().await.insert("0000".to_string(), arc_camp.clone());
        server_state.campaigns.write().await.insert("default_campaign".to_string(), arc_camp);
        let (tx, _) = broadcast::channel(100);
        server_state.channels.write().await.insert("0000".to_string(), tx);
    }

    // Estado inicial limpo: o estado do mundo só é restaurado se o usuário importar explicitamente um Arquivo de Campanha (.json)
    println!(">>> [STARTUP] Servidor pronto em estado inicial limpo (sem restauração automática de campanhas passadas).");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/state", get(legacy_get_state))
        .route("/api/character", post(receive_character))
        .route("/api/action/turn", post(legacy_turn_action))
        // Rotas do Motor de Campanha Multiplayer (suporta /api/campaign e /api/campaigns)
        .route("/api/campaign/create", post(create_campaign))
        .route("/api/campaign/analyze-premise", post(analyze_premise))
        .route("/api/campaign/join", post(join_campaign))
        .route("/api/campaign/{id}/state", get(get_campaign_state))
        .route("/api/campaigns/{id}/state", get(get_campaign_state))
        .route("/api/campaign/{id}/chronicle", get(get_campaign_chronicle))
        .route("/api/campaigns/{id}/chronicle", get(get_campaign_chronicle))
        .route("/api/campaign/{id}/entities", get(get_campaign_entities))
        .route("/api/campaigns/{id}/entities", get(get_campaign_entities))
        .route("/api/campaign/{id}/scene/start", post(start_living_scene))
        .route("/api/campaigns/{id}/scene/start", post(start_living_scene))
        .route("/api/campaign/{id}/scene/claim_item", post(claim_scene_item))
        .route("/api/campaigns/{id}/scene/claim_item", post(claim_scene_item))
        .route("/api/campaign/{id}/scene/resolve", post(resolve_scene))
        .route("/api/campaigns/{id}/scene/resolve", post(resolve_scene))
        .route("/api/campaign/{id}/scene/move", post(move_scene_participant))
        .route("/api/campaigns/{id}/scene/move", post(move_scene_participant))
        .route("/api/campaign/{id}/scene/end-turn", post(end_scene_turn))
        .route("/api/campaigns/{id}/scene/end-turn", post(end_scene_turn))
        .route("/api/campaign/{id}/action/turn", post(process_campaign_turn))
        .route("/api/campaigns/{id}/action/turn", post(process_campaign_turn))
        .route("/api/campaign/{id}/turn", post(process_campaign_turn))
        .route("/api/campaigns/{id}/turn", post(process_campaign_turn))
        // Modo Um Terminal e Gerenciamento de Slots Presenciais
        .route("/api/campaign/{id}/slots", get(get_campaign_slots))
        .route("/api/campaigns/{id}/slots", get(get_campaign_slots))
        .route("/api/campaign/add-local-player", post(add_local_player))
        .route("/api/campaign/{id}/sync-party", post(sync_campaign_party))
        .route("/api/campaigns/{id}/sync-party", post(sync_campaign_party))
        .route("/api/campaign/{id}", delete(delete_saved_campaign))
        // T12: Gravação Externa e Retomada de Campanhas
        .route("/api/campaigns/saved", get(list_saved_campaigns))
        .route("/api/campaign/{id}/export", get(export_campaign_bundle))
        .route("/api/campaign/import", post(import_campaign_bundle))
        // T13: Catálogo e Painel de Histórias Pré-Construídas
        .route("/api/premises/saved", get(list_saved_premises))
        // Rotas SaaS de Autenticação (Argon2id)
        .route("/api/auth/register", post(auth_register))
        .route("/api/auth/login", post(auth_login))
        .route("/api/auth/me", get(auth_me))
        .route("/api/auth/logout", post(auth_logout))
        // Rotas Exclusivas do Painel Administrativo SaaS
        .route("/api/admin/auth/login", post(admin_auth_login))
        .route("/api/admin/metrics", get(admin_get_metrics))
        .route("/api/admin/users", get(admin_list_users))
        .route("/api/admin/premises", post(admin_save_premise))
        .route("/api/admin/premises/preview", post(admin_preview_premise))
        .route("/api/admin/premises/{id}", delete(admin_delete_premise))
        // T14: Chat da Party
        .route("/api/campaign/{id}/chat", post(send_party_chat))
        // T15: Modal de Auditoria Completa
        .route("/api/campaign/{id}/audit", get(get_campaign_audit))
        // WebSockets para sincronização de sala e chat em tempo real
        .route("/ws/{room_code}", get(ws_handler))
        .layer(cors)
        .with_state(server_state);

    let addr = "0.0.0.0:3001";
    println!("Servidor HTTP e WebSocket ouvindo em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "online",
            "engine": "rpg-engine-rust",
            "rules": "D&D 5e Deterministic",
            "multiplayer": "WebSockets Active",
            "version": "0.2.0"
        })),
    )
}

async fn analyze_premise(
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
        Ok(analysis) => (
            StatusCode::OK,
            Json(json!({
                "status": "ok",
                "analysis": analysis
            })),
        ),
        Err(e) => {
            eprintln!("[PREMISE] Erro na análise da premissa: {}. Utilizando fallback determinístico.", e);
            let fallback = rpg_engine::llm::skeleton_agent::SkeletonAgent::deterministic_premise_analysis(&req.premise, slider);
            (
                StatusCode::OK,
                Json(json!({
                    "status": "ok",
                    "analysis": fallback
                })),
            )
        }
    }
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|val| val.to_str().ok())
        .and_then(|auth| {
            if auth.starts_with("Bearer ") {
                Some(auth[7..].trim().to_string())
            } else {
                Some(auth.trim().to_string())
            }
        })
}

async fn create_campaign(
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
        .unwrap_or_else(|_| rpg_engine::narrative::skeleton::NarrativeSkeleton::new(&enriched_premise, &req.literary_style));

    let manifest = CampaignManifest {
        campaign_id: campaign_id.clone(),
        room_code: room_code.clone(),
        host_player_id: host_id.clone(),
        player_ids: player_ids.clone(),
        player_slots: player_slots.clone(),
        premise: enriched_premise.clone(),
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

                let mut scores = std::collections::HashMap::new();
                let stats_obj = hero.get("ability_scores").or_else(|| hero.get("stats"));
                if let Some(obj) = stats_obj.and_then(|v| v.as_object()) {
                    for (k, v) in obj {
                        if let Some(val) = v.as_i64() {
                            scores.insert(k.to_uppercase(), val as i32);
                        }
                    }
                }
                if !scores.is_empty() {
                    dnd_stats.ability_scores = scores;
                }

                hero_sheet.dnd_stats = Some(dnd_stats);
                hero_sheet.custom_flags.insert("race".to_string(), json!(hero_race));
                hero_sheet.custom_flags.insert("class".to_string(), json!(slot.class_name));
                if let Some(inv) = hero.get("inventory").and_then(|v| v.as_array()) {
                    hero_sheet.custom_flags.insert("inventory".to_string(), json!(inv));
                }
                let _ = entity_store.save_player_entity(&hero_sheet);
                println!(">>> [CAMPAIGN] Herói pré-campanha registrado no EntityStore: {} (Classe: {}, ID: {})", slot.player_name, slot.class_name, slot.player_id);
            }
        } else {
            let mut host_sheet = WorldEntitySheet::new(
                &host_id,
                &req.host_player_name,
                EntityType::Player,
                &format!("Host da mesa jogando como {}", req.host_class),
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

    // 6. Gera prólogo literário denso, mitológico e cinematográfico ("A História por trás da História")
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

    // Registra prólogo inicial denso na Crônica
    if let Ok(mut chronicle) = campaign.get_chronicle_book() {
        let mut initial_entry = rpg_engine::narrative::chronicle::ChronicleEntry::new(
            current_act_number,
            &format!("Prólogo — Ato {}: {}", current_act_number, current_act_title),
            rpg_engine::narrative::chronicle::ChronicleEntryType::LiteraryChapter,
            &dense_prologue,
            Vec::new(),
            player_ids.clone(),
        );
        let _ = chronicle.append_entry(&mut initial_entry);
    }

    // 7. Gera automaticamente a primeira Cena Viva (Ato I) com participantes, iniciativas reais e NPCs expandidos
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

fn build_player_participant(
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

fn parse_npc_multiplier_and_singularize(raw_name: &str) -> (String, usize) {
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

fn is_ally_npc_name(name: &str) -> bool {
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

fn calculate_dnd_monster_stats(name: &str, act_number: u32) -> (i32, i32, i32) {
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

fn expand_and_build_npc_participants(
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

/// Garante que uma LivingScene (inclusive carregada de disco de versão anterior) tenha HP real, CA, iniciativas D20+mod, obstáculos no canvas, inimigos expandidos e turno apontando para um jogador vivo
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

fn hydrate_campaign_active_scene(camp: &mut CampaignState) {
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

/// Helper para buscar campanha em memória viva ou recarregar do disco sob demanda
async fn get_or_load_campaign(state: &ServerState, campaign_id: &str) -> Option<Arc<RwLock<CampaignState>>> {
    {
        let campaigns = state.campaigns.read().await;
        if let Some(c) = campaigns.get(campaign_id) {
            return Some(c.clone());
        }
    }

    // Tenta carregar do disco em data/campaigns/{campaign_id}
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

                state.rooms.write().await.insert(room_code.clone(), arc_c.clone());
                state.campaigns.write().await.insert(campaign_id.to_string(), arc_c.clone());

                let mut channels = state.channels.write().await;
                channels.entry(room_code.clone()).or_insert_with(|| broadcast::channel(100).0);

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

/// Helper para buscar ou carregar campanha a partir do room_code
async fn get_or_load_campaign_by_room(state: &ServerState, room_code: &str) -> Option<Arc<RwLock<CampaignState>>> {
    {
        let rooms = state.rooms.read().await;
        if let Some(c) = rooms.get(room_code) {
            return Some(c.clone());
        }
    }

    // Busca nas pastas de campanhas salvas em disco pelo manifest.json
    if let Ok(entries) = std::fs::read_dir("data/campaigns") {
        for entry in entries.flatten() {
            let manifest_path = entry.path().join("manifest.json");
            if manifest_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                    if let Ok(manifest_val) = serde_json::from_str::<Value>(&content) {
                        if manifest_val["room_code"].as_str().map(|r| r.eq_ignore_ascii_case(room_code)).unwrap_or(false) {
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

async fn join_campaign(
    State(state): State<ServerState>,
    Json(req): Json<JoinCampaignRequest>,
) -> (StatusCode, Json<Value>) {
    let req_room_code = req.room_code.trim().to_uppercase();

    // 1. Tenta por campaign_id se fornecido
    let mut campaign_arc = if let Some(ref req_camp_id) = req.campaign_id {
        get_or_load_campaign(&state, req_camp_id).await
    } else {
        None
    };

    // 2. Se não encontrou, busca por room_code
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

    // Se o UUID foi informado, valida se coincide com o UUID real da campanha
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

    // CASO A: O jogador informou um player_code existente (reconexão / retomada de slot presencial ↔ online)
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
                    "message": format!("Slot do Jogador #{} ({}) reconectado com sucesso!", slot_clone.player_number, slot_clone.player_name)
                })),
            );
        }
    }

    // CASO A2: Retomada direta de campanha existente no terminal ou quando a sala já possui slots criados (ex: dono da sala reconectando ou modo um terminal)
    let is_single_term = req.is_single_terminal.unwrap_or(false);
    if let Some(ref m) = camp.manifest {
        if !m.player_slots.is_empty() && (is_single_term || m.player_slots.len() >= 6) {
            // Reutiliza o primeiro slot (Host) por padrão
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

    // CASO B: Novo jogador entrando na sala
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

    // Salva ficha viva do jogador convidado
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

    // Notifica a sala via WebSocket se houver canal aberto
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

async fn get_campaign_slots(
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

async fn add_local_player(
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

async fn get_campaign_state(
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

    // Sanitizado: NÃO envia o hidden_skeleton para a interface (spoilers protegidos)
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

async fn get_campaign_chronicle(
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

async fn get_campaign_entities(
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

async fn start_living_scene(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(req): Json<StartSceneRequest>,
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

    let mut participants = Vec::new();
    let existing_players = camp
        .get_entity_store()
        .and_then(|s| s.list_player_entities())
        .unwrap_or_default();

    if let Some(ref m) = camp.manifest {
        for (i, p_id) in m.player_ids.iter().enumerate() {
            let slot_opt = m.player_slots.iter().find(|s| &s.player_id == p_id);
            let p_name = existing_players
                .iter()
                .find(|pe| &pe.id == p_id)
                .map(|pe| pe.name.clone())
                .or_else(|| slot_opt.map(|s| s.player_name.clone()))
                .unwrap_or_else(|| format!("Jogador {}", i + 1));
            let p_class = slot_opt
                .map(|s| s.class_name.as_str())
                .unwrap_or("Fighter");

            participants.push(build_player_participant(
                p_id,
                &p_name,
                p_class,
                i,
                &existing_players,
            ));
        }
    }

    let premise = camp
        .manifest
        .as_ref()
        .map(|m| m.premise.clone())
        .unwrap_or_default();
    let literary_style = camp
        .manifest
        .as_ref()
        .map(|m| m.literary_style.clone())
        .unwrap_or_default();
    let (act_number, act_title, act_synopsis, key_milestones) = camp
        .manifest
        .as_ref()
        .and_then(|m| m.hidden_skeleton.current_act())
        .map(|a| {
            (
                a.act_number,
                a.title.clone(),
                a.synopsis.clone(),
                a.expected_milestones.clone(),
            )
        })
        .unwrap_or_else(|| (1, "Ato I".to_string(), premise.clone(), Vec::new()));

    let latest_chronicle_prose = camp
        .get_chronicle_book()
        .and_then(|b| b.get_all_entries())
        .ok()
        .and_then(|entries| entries.last().map(|e| e.narrative_prose.clone()))
        .unwrap_or_else(|| act_synopsis.clone());

    let mut new_scene = state
        .environment_agent
        .generate_dynamic_scene(
            &campaign_id,
            &premise,
            &literary_style,
            act_number,
            &act_title,
            &act_synopsis,
            &key_milestones,
            &latest_chronicle_prose,
            participants,
        )
        .await;

    // Permite override explícito apenas se o cliente enviou objetivos/título customizados específicos
    if let Some(custom_title) = req.title.filter(|t| !t.trim().is_empty()) {
        new_scene.title = custom_title;
    }
    if let Some(custom_loc) = req.location.filter(|l| !l.trim().is_empty()) {
        new_scene.location = custom_loc;
    }
    if !req.objectives.is_empty() {
        new_scene.objectives = req
            .objectives
            .into_iter()
            .enumerate()
            .map(|(i, desc)| SceneObjective {
                id: format!("obj_{}", i + 1),
                description: desc,
                completed: false,
                completion_trigger: None,
            })
            .collect();
    }

    let custom_monsters = camp
        .manifest
        .as_ref()
        .map(|m| m.custom_monsters.clone())
        .unwrap_or_default();
    let homebrew_bosses = camp
        .manifest
        .as_ref()
        .map(|m| m.hidden_skeleton.homebrew_bosses.clone())
        .unwrap_or_default();
    let world_entities = camp
        .get_entity_store()
        .and_then(|s| s.list_world_entities())
        .unwrap_or_default();

    let npc_participants = expand_and_build_npc_participants(
        &new_scene.active_npcs,
        act_number,
        &custom_monsters,
        &homebrew_bosses,
        &world_entities,
    );

    if let Ok(store) = camp.get_entity_store() {
        for np in &npc_participants {
            if store.get_world_entity(&np.id).ok().flatten().is_none() {
                let mut sheet = WorldEntitySheet::new(
                    &np.id,
                    &np.name,
                    EntityType::Npc,
                    &format!("Entidade ativa na cena '{}' ({})", new_scene.title, new_scene.location),
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

    new_scene.participants.extend(npc_participants);
    ensure_tactical_scene_hydrated(&mut new_scene, &existing_players);

    camp.active_scene = Some(new_scene.clone());
    let _ = camp.save_active_scene();

    // Notifica via WebSocket
    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&camp.room_code) {
        let _ = tx.send(
            json!({
                "type": "SCENE_STARTED",
                "scene": new_scene
            })
            .to_string(),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Cena Viva dinâmica iniciada com sucesso a partir do Esqueleto Narrativo",
            "scene": new_scene
        })),
    )
}

async fn move_scene_participant(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(req): Json<SceneMoveRequest>,
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
    hydrate_campaign_active_scene(&mut camp);
    let room_code = camp.room_code.clone();

    let scene = match camp.active_scene.as_mut() {
        Some(s) => s,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "status": "error", "message": "Nenhuma cena ativa para movimentação" })),
            )
        }
    };

    let in_combat = scene.in_combat && scene.has_live_hostiles();
    let p_idx_opt = scene
        .participants
        .iter()
        .position(|p| p.id == req.actor_id || p.name.eq_ignore_ascii_case(&req.actor_id))
        .or_else(|| {
            let cur = scene.current_participant_index;
            if scene.participants.get(cur).map(|p| p.is_player).unwrap_or(false) {
                Some(cur)
            } else {
                scene.participants.iter().position(|p| p.is_player && !p.is_dead)
            }
        });

    let p_idx = match p_idx_opt {
        Some(idx) => idx,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "status": "error", "message": "Participante não encontrado na cena" })),
            )
        }
    };

    let target_x = req.x.clamp(0, 60);
    let target_y = req.y.clamp(0, 40);
    let (cur_x, cur_y) = scene.participants[p_idx].position.unwrap_or((10, 20));
    let dx = (target_x - cur_x) as f64;
    let dy = (target_y - cur_y) as f64;
    let dist_ft = (dx * dx + dy * dy).sqrt().round() as u32;

    if !in_combat {
        scene.participants[p_idx].position = Some((target_x, target_y));
        scene.participants[p_idx].actions_budget.movement_remaining_ft = 30;
    } else {
        let rem = scene.participants[p_idx].actions_budget.movement_remaining_ft;
        if rem == 0 && dist_ft > 0 {
            let scene_clone = scene.clone();
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Sem deslocamento restante neste turno (0 ft). Use Disparada (Dash) ou encerre seu turno.",
                    "active_scene": scene_clone
                })),
            );
        }

        if dist_ft <= rem {
            scene.participants[p_idx].position = Some((target_x, target_y));
            scene.participants[p_idx].actions_budget.movement_remaining_ft = rem.saturating_sub(dist_ft);
        } else if dist_ft > 0 {
            let ratio = (rem as f64) / (dist_ft as f64);
            let clamped_x = (cur_x as f64 + dx * ratio).round() as i32;
            let clamped_y = (cur_y as f64 + dy * ratio).round() as i32;
            scene.participants[p_idx].position = Some((clamped_x.clamp(0, 60), clamped_y.clamp(0, 40)));
            scene.participants[p_idx].actions_budget.movement_remaining_ft = 0;
        }
    }

    let scene_clone = scene.clone();
    let _ = camp.save_active_scene();

    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&room_code) {
        let _ = tx.send(
            json!({
                "type": "SCENE_UPDATED",
                "active_scene": scene_clone
            })
            .to_string(),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "active_scene": scene_clone
        })),
    )
}

async fn end_scene_turn(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(req): Json<EndTurnRequest>,
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
    hydrate_campaign_active_scene(&mut camp);
    let room_code = camp.room_code.clone();

    let entity_store = match camp.get_entity_store() {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": e.to_string() })),
            )
        }
    };

    let mut enemy_logs = Vec::new();
    let mut enemy_turns = Vec::new();

    if let Some(ref mut scene) = camp.active_scene {
        if let Some(ref aid) = req.actor_id {
            if let Some(idx) = scene
                .participants
                .iter()
                .position(|p| &p.id == aid || p.name.eq_ignore_ascii_case(aid))
            {
                scene.current_participant_index = idx;
            }
        }

        let prev_actor_name = scene
            .current_participant()
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "Herói".to_string());
        let prev_actor_id = scene
            .current_participant()
            .map(|p| p.id.clone())
            .unwrap_or_else(|| "player_1".to_string());

        let (logs, turns, victory_achieved) =
            EnvironmentAgent::advance_to_next_player_and_run_npcs(scene, &entity_store);
        enemy_logs = logs;
        enemy_turns = turns;

        if !enemy_logs.is_empty() || victory_achieved {
            let mut reaction = String::from("⏭️ **Turno encerrado pelo jogador.**\n\n🎲 **[ROLAGENS TÁTICAS DOS INIMIGOS E ALIADOS]:**\n");
            for l in &enemy_logs {
                reaction.push_str(&format!("- {}\n", l));
            }
            if victory_achieved {
                reaction.push_str(
                    "\n🏆 **[VITÓRIA TÁTICA! Todos os inimigos hostis foram abatidos! O campo de batalha está seguro para exploração e saque.]**",
                );
            }
            let turn_rec = rpg_engine::narrative::living_scene::SceneInteractionTurn {
                turn_number: scene.turn_history.len() as u32 + 1,
                actor_id: prev_actor_id,
                actor_name: prev_actor_name,
                via_type: "end_turn".to_string(),
                player_input: "Encerrar Turno".to_string(),
                mechanical_intent: None,
                rules_result: Some(json!({
                    "status": "success",
                    "action": "end_turn",
                    "enemy_turns": enemy_turns
                })),
                environment_reaction: reaction,
            };
            scene.record_turn(turn_rec);
        }
    }

    let _ = camp.save_active_scene();
    let updated_players = entity_store.list_player_entities().unwrap_or_default();
    let scene_clone = camp.active_scene.clone();

    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&room_code) {
        let _ = tx.send(
            json!({
                "type": "SCENE_UPDATED",
                "active_scene": scene_clone,
                "player_entities": updated_players,
                "enemy_turns": enemy_turns,
            })
            .to_string(),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "active_scene": scene_clone,
            "player_entities": updated_players,
            "enemy_turns": enemy_turns,
            "enemy_combat_log": enemy_logs,
        })),
    )
}

async fn claim_scene_item(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(req): Json<ClaimItemRequest>,
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

    let mut claimed_item_clone = None;
    let mut scene_chapter = 1;
    let mut scene_title = String::new();

    if let Some(ref mut scene) = camp.active_scene {
        if let Some(item) = scene.scene_loot.iter_mut().find(|i| i.id == req.item_id) {
            if item.is_claimed {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "status": "error", "message": "Item já foi reivindicado anteriormente." })),
                );
            }

            item.is_claimed = true;
            item.claimed_by = Some(req.player_name.clone());

            claimed_item_clone = Some(item.clone());
            scene_chapter = scene.chapter;
            scene_title = scene.title.clone();
        }
    }

    if let Some(claimed_item) = claimed_item_clone {
        // Salva a cena atualizada em disco
        let _ = camp.save_active_scene();

        // Atualiza inventário na ficha do jogador no EntityStore
        if let Ok(store) = camp.get_entity_store() {
            if let Ok(mut players) = store.list_player_entities() {
                if let Some(p_sheet) = players.iter_mut().find(|p| p.name == req.player_name) {
                    let mut inv = p_sheet
                        .custom_flags
                        .get("inventory")
                        .and_then(|v| v.as_array().cloned())
                        .unwrap_or_default();
                    inv.push(json!({
                        "name": claimed_item.name,
                        "quantity": 1,
                        "weightLbs": 1,
                        "isEquipped": false,
                        "rarity": claimed_item.rarity,
                        "description": claimed_item.description
                    }));
                    p_sheet.custom_flags.insert("inventory".to_string(), json!(inv));
                    let _ = store.save_player_entity(p_sheet);
                }
            }
        }

        // Registra no livro da crônica
        if let Ok(mut book) = camp.get_chronicle_book() {
            let note = format!(
                "🎒 O herói {} encontrou e coletou o item: '{}' ({}) - {}!",
                req.player_name,
                claimed_item.name,
                claimed_item.rarity,
                claimed_item.description
            );
            let mut entry = ChronicleEntry::new(
                scene_chapter,
                &scene_title,
                ChronicleEntryType::MechanicalAction,
                &note,
                vec![],
                vec![req.player_name.clone()],
            );
            let _ = book.append_entry(&mut entry);
        }

        let active_scene_clone = camp.active_scene.clone();

        // Notifica todos os participantes conectados via WebSocket
        let channels = state.channels.read().await;
        if let Some(tx) = channels.get(&room_code) {
            let _ = tx.send(
                json!({
                    "type": "ITEM_CLAIMED",
                    "item": claimed_item,
                    "player_name": req.player_name,
                    "active_scene": active_scene_clone
                })
                .to_string(),
            );
        }

        return (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": format!("Item '{}' coletado com sucesso por {}!", claimed_item.name, req.player_name),
                "item": claimed_item,
                "active_scene": active_scene_clone
            })),
        );
    }

    (
        StatusCode::NOT_FOUND,
        Json(json!({ "status": "error", "message": "Item ou cena ativa não encontrados" })),
    )
}

async fn resolve_scene(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(payload): Json<Value>,
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
    hydrate_campaign_active_scene(&mut camp);

    if camp.active_scene.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "status": "error", "message": "Nenhuma cena ativa para concluir" })),
        );
    }

    let reason_str = payload["reason"].as_str().unwrap_or("objectives_completed");
    let force_close = payload["force"].as_bool().unwrap_or(false);
    let reason = match reason_str {
        "party_departed" | "flee" => SceneResolutionReason::PartyDeparted,
        "party_defeated" => SceneResolutionReason::PartyDefeated,
        _ => SceneResolutionReason::ObjectivesCompleted,
    };

    // Trava de Batalha: não permite concluir a cena enquanto houver inimigos hostis vivos em combate (exceto derrota do grupo ou force_close)
    if let Some(ref sc) = camp.active_scene {
        if sc.in_combat
            && sc.has_live_hostiles()
            && reason != SceneResolutionReason::PartyDefeated
            && !force_close
        {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "status": "error",
                    "message": "Combate ativo em andamento! Derrote todos os inimigos hostis ou realize uma ação mecânica de Fuga / Retirada Tática (CD 12) antes de sair da cena.",
                    "active_scene": sc
                })),
            );
        }
    }

    let mut active_scene = camp.active_scene.take().unwrap();

    let resolution = match SceneCloser::close_and_consolidate(
        &mut active_scene,
        reason,
        &mut camp,
        &state.orchestrator,
        &state.skeleton_agent,
    )
    .await
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("[SCENE_RESOLVE] Erro ao consolidar cena: {}", e);
            camp.active_scene = Some(active_scene);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": e.to_string() })),
            );
        }
    };

    camp.active_scene = None;
    let _ = camp.save_active_scene();

    let room_code = camp.room_code.clone();
    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&room_code) {
        let _ = tx.send(
            json!({
                "type": "SCENE_RESOLVED",
                "resolution": resolution,
                "active_scene": null,
            })
            .to_string(),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Cena concluída e consolidada na Crônica!",
            "resolution": resolution,
        })),
    )
}

async fn process_campaign_turn(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
    Json(req): Json<TurnActionRequest>,
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
    hydrate_campaign_active_scene(&mut camp);
    let room_code = camp.room_code.clone();

    if camp.active_scene.is_none() {
        let existing_players = camp
            .get_entity_store()
            .and_then(|s| s.list_player_entities())
            .unwrap_or_default();
        let obj = vec![SceneObjective {
            id: "obj_explore".to_string(),
            description: "Investigar e avançar pelo local atual".to_string(),
            completed: false,
            completion_trigger: None,
        }];
        let p_id = req.actor_id.clone().unwrap_or_else(|| "player_1".to_string());
        let part = vec![build_player_participant(
            &p_id,
            &req.character_name,
            &req.class_name,
            0,
            &existing_players,
        )];
        let mut fallback_scene = LivingScene::new(&campaign_id, "Exploração", "Local Atual", 1, obj, part);
        fallback_scene.in_combat = false;
        camp.active_scene = Some(fallback_scene);
    }

    let mut active_scene = camp.active_scene.take().unwrap();
    let entity_store = camp.get_entity_store().unwrap();
    let actor_id = req.actor_id.unwrap_or_else(|| "player_1".to_string());
    let player_input = req.player_text.unwrap_or_else(|| "Ação de turno".to_string());

    // Marca ação consumida se ID fornecido
    if let Some(ref act_id) = req.action_id {
        if !active_scene.consumed_scenario_options.contains(act_id) {
            active_scene.consumed_scenario_options.push(act_id.clone());
        }
    }

    let turn_output = match state
        .environment_agent
        .process_turn(
            &mut active_scene,
            &actor_id,
            &req.character_name,
            &req.via_type,
            &player_input,
            req.action_id.as_deref(),
            &mut camp,
            &entity_store,
        )
        .await
    {
        Ok(out) => out,
        Err(e) => {
            eprintln!("[TURN] Erro no processamento de turno: {}", e);
            camp.active_scene = Some(active_scene);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": e.to_string() })),
            );
        }
    };

    // Revelação orgânica de loot oculto na exploração
    let player_text_lower = player_input.to_lowercase();
    let is_searching = player_text_lower.contains("investig")
        || player_text_lower.contains("procur")
        || player_text_lower.contains("examin")
        || player_text_lower.contains("vasculh")
        || player_text_lower.contains("saque")
        || player_text_lower.contains("baú")
        || player_text_lower.contains("bau")
        || player_text_lower.contains("corpo")
        || player_text_lower.contains("fenda")
        || player_text_lower.contains("abrir")
        || req.action_id.as_deref().map(|s| s.contains("investig") || s.contains("procur")).unwrap_or(false);

    let is_check_success = turn_output.rules_result.as_ref()
        .and_then(|r| r.get("is_success"))
        .and_then(|b| b.as_bool())
        .unwrap_or(true);

    let mut final_reaction = turn_output.narrative_reaction.clone();
    if is_searching && is_check_success {
        if let Some(hidden_item) = active_scene.scene_loot.iter_mut().find(|item| item.is_hidden && !item.is_claimed) {
            hidden_item.is_hidden = false;
            let note = format!(
                "\n\n✨ [DESCOBERTA / ITEM ENCONTRADO]: Durante a investigação atenta do local, você descobriu: **{}**! ({})\n*(O item agora está visível nas descobertas da cena e pode ser coletado pelo grupo).* ",
                hidden_item.name, hidden_item.description
            );
            final_reaction.push_str(&note);
            println!(">>> [CENA VIVA] Item oculto revelado por investigação: {}", hidden_item.name);
        }
    }

    // Atualiza opções situacionais fora de combate para a nova rodada
    let active_options_count = active_scene.scenario_options.iter().filter(|o| {
        let id = o["id"].as_str().unwrap_or("");
        !active_scene.consumed_scenario_options.contains(&id.to_string())
    }).count();

    let act_boss = camp.manifest.as_ref()
        .and_then(|m| m.hidden_skeleton.current_act())
        .and_then(|a| a.boss_name.as_deref())
        .unwrap_or("o Chefe da Região");

    let wants_advance = (!active_scene.in_combat || !active_scene.has_live_hostiles())
        && (req.action_id.as_deref().map(|id| id.contains("advance") || id.contains("boss_confront") || id.contains("unlock_sector")).unwrap_or(false)
            || player_text_lower.contains("avançar para o próximo ato")
            || player_text_lower.contains("concluir cena"));

    if active_options_count < 2 {
        let round_num = active_scene.turn_history.len() + 1;
        active_scene.scenario_options.push(json!({
            "id": format!("scen_opt_boss_confront_{}", round_num),
            "title": format!("Confrontar as Patrulhas e Buscar {}", act_boss),
            "description": format!("Seguir os rastros de destruição e energia deixados para trás, marchando diretamente em direção a {}.", act_boss),
            "actionCost": "action",
            "skillCheckSuggested": { "skill": "Athletics", "dcEstimate": 13 },
            "tacticalBenefit": "Inicia o confronto decisivo com iniciativa vantajosa."
        }));
        active_scene.scenario_options.push(json!({
            "id": format!("scen_opt_unlock_sector_{}", round_num),
            "title": "Forçar passagem mecânica para o próximo setor",
            "description": "Alavancar as engrenagens emperradas e romper o bloqueio para transpor o setor.",
            "actionCost": "action",
            "skillCheckSuggested": { "skill": "Athletics", "dcEstimate": 12 },
            "tacticalBenefit": "Destrava o avanço definitivo e escapa das patrulhas menores."
        }));
        active_scene.scenario_options.push(json!({
            "id": format!("scen_opt_fortify_{}", round_num),
            "title": "Organizar emboscada e atrair os guardas",
            "description": "Criar uma distração com reflexos e som para encurralar as patrulhas.",
            "actionCost": "bonus_action",
            "skillCheckSuggested": { "skill": "Stealth", "dcEstimate": 12 },
            "tacticalBenefit": "Surpreende o inimigo na primeira rodada de combate."
        }));
    }

    // Checa se a cena foi concluída ou solicitou avanço decisivo (respeitando a Trava de Batalha)
    let mut scene_closure_result = None;
    let mut scene_resolution_final = turn_output.scene_resolution.clone();
    if wants_advance && scene_resolution_final.is_none() {
        println!(">>> [CENA VIVA] Jogador acionou avanço após combate concluído. Concluindo cena...");
        scene_resolution_final = Some(SceneResolutionReason::ObjectivesCompleted);
    }

    if let Some(reason) = scene_resolution_final {
        println!(">>> [CENA VIVA] Condição de término detectada: {:?}. Acionando SceneCloser...", reason);
        match SceneCloser::close_and_consolidate(
            &mut active_scene,
            reason,
            &mut camp,
            &state.orchestrator,
            &state.skeleton_agent,
        )
        .await
        {
            Ok(res) => {
                scene_closure_result = Some(res);
            }
            Err(e) => {
                eprintln!("[SCENE_CLOSER] Erro no fechamento da cena: {}", e);
            }
        }
    } else {
        camp.active_scene = Some(active_scene.clone());
        let _ = camp.save_active_scene();
    }

    let updated_players = entity_store.list_player_entities().unwrap_or_default();

    // Broadcast da atualização para todos os jogadores na sala
    let channels = state.channels.read().await;
    if let Some(tx) = channels.get(&room_code) {
        let _ = tx.send(
            json!({
                "type": "TURN_RESOLVED",
                "actor_id": actor_id,
                "actor_name": req.character_name,
                "via_type": req.via_type,
                "narrative_reaction": final_reaction,
                "rules_result": turn_output.rules_result,
                "active_scene": camp.active_scene,
                "player_entities": updated_players,
                "scene_closed": scene_closure_result,
            })
            .to_string(),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "narrative_reaction": final_reaction,
            "rules_result": turn_output.rules_result,
            "active_scene": camp.active_scene,
            "player_entities": updated_players,
            "scene_closed": scene_closure_result,
        })),
    )
}

async fn ws_handler(
    Path(room_code): Path<String>,
    ws: WebSocketUpgrade,
    State(state): State<ServerState>,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| handle_socket(socket, room_code, state))
}

async fn handle_socket(socket: WebSocket, room_code: String, state: ServerState) {
    let (mut sender, mut receiver) = socket.split();

    // Obtém ou cria o canal de broadcast da sala
    let rx_opt = {
        let mut channels = state.channels.write().await;
        let tx = channels
            .entry(room_code.clone())
            .or_insert_with(|| broadcast::channel(100).0);
        Some(tx.subscribe())
    };

    let mut rx = rx_opt.unwrap();

    // Envia estado inicial da sala para o cliente recém-conectado (buscando em memória ou carregando do disco sob demanda)
    if let Some(camp_arc) = get_or_load_campaign_by_room(&state, &room_code).await {
        let camp = camp_arc.read().await;
        let player_slots = camp.manifest.as_ref().map(|m| m.player_slots.clone()).unwrap_or_default();
        let init_msg = json!({
            "type": "INITIAL_SYNC",
            "campaign_id": camp.campaign_id,
            "room_code": camp.room_code,
            "player_slots": player_slots,
            "active_scene": camp.active_scene,
        });
        let _ = sender.send(Message::Text(init_msg.to_string().into())).await;
    }

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg_text) = rx.recv().await {
            if sender.send(Message::Text(msg_text.into())).await.is_err() {
                break;
            }
        }
    });

    let room_code_clone = room_code.clone();
    let channels_clone = state.channels.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                let channels = channels_clone.read().await;
                if let Some(tx) = channels.get(&room_code_clone) {
                    let _ = tx.send(text.to_string());
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

// Handlers legados mantidos para compatibilidade com os painéis existentes
async fn legacy_get_state(State(state): State<ServerState>) -> (StatusCode, Json<Value>) {
    let rooms = state.rooms.read().await;
    if let Some(camp_arc) = rooms.get("0000") {
        let camp = camp_arc.read().await;
        let current_beat = camp.current_beat();
        return (
            StatusCode::OK,
            Json(json!({
                "current_beat_index": camp.current_beat_index,
                "beat_title": current_beat.map(|b| b.title.clone()).unwrap_or_default(),
                "beat_summary": current_beat.map(|b| b.summary.clone()).unwrap_or_default(),
            })),
        );
    }

    (StatusCode::OK, Json(json!({ "current_beat_index": 0 })))
}

fn build_player_entity_from_sheet(
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

#[derive(Serialize, Deserialize, Debug)]
struct SyncPartyRequest {
    party: Vec<Value>,
}

async fn sync_campaign_party(
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

    // Se houver cena ativa, sincroniza nomes, atributos e iniciativas reais dos jogadores na ordem de turno
    if let Some(ref mut scene) = camp.active_scene {
        let mut player_participants: Vec<TurnParticipant> = Vec::new();
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
        let non_players: Vec<TurnParticipant> = scene
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

async fn receive_character(
    State(state): State<ServerState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let name = payload["name"].as_str().unwrap_or("Aventureiro");
    let class_name = payload["className"].as_str().unwrap_or("Fighter");
    let req_campaign_id = payload["campaign_id"].as_str().map(|s| s.to_string());

    println!(">>> [BACKEND] Ficha sincronizada: {} ({})", name, class_name);

    // Se houver uma campanha associada (ou uma campanha ativa no servidor), persiste a ficha real no EntityStore e atualiza o slot
    let camp_arc_opt = {
        let campaigns = state.campaigns.read().await;
        if let Some(ref cid) = req_campaign_id {
            campaigns.get(cid).cloned()
        } else {
            campaigns
                .iter()
                .find(|(k, _)| *k != "default_campaign")
                .map(|(_, v)| v.clone())
        }
    };

    if let Some(camp_arc) = camp_arc_opt {
        let mut camp = camp_arc.write().await;
        let mut target_player_id = payload["player_id"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_default();
        let mut target_player_num = 1;

        if let Some(ref mut m) = camp.manifest {
            if let Some(slot) = m
                .player_slots
                .iter_mut()
                .find(|s| (!target_player_id.is_empty() && s.player_id == target_player_id) || s.player_name == name)
            {
                slot.player_name = name.to_string();
                slot.class_name = class_name.to_string();
                target_player_id = slot.player_id.clone();
                target_player_num = slot.player_number;
            } else if let Some(first_slot) = m.player_slots.first_mut() {
                first_slot.player_name = name.to_string();
                first_slot.class_name = class_name.to_string();
                target_player_id = first_slot.player_id.clone();
                target_player_num = first_slot.player_number;
            }
        }
        let _ = camp.save_manifest();

        if !target_player_id.is_empty() {
            if let Ok(store) = camp.get_entity_store() {
                let sheet = build_player_entity_from_sheet(&target_player_id, target_player_num, &payload);
                let _ = store.save_player_entity(&sheet);
            }
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": format!("Ficha do herói '{}' sincronizada com o motor de regras D&D 5e.", name),
            "registered_class": class_name
        })),
    )
}

async fn legacy_turn_action(
    State(state): State<ServerState>,
    Json(req): Json<TurnActionRequest>,
) -> (StatusCode, Json<TurnActionResponse>) {
    let rooms = state.rooms.read().await;
    let camp_arc = rooms.get("0000").cloned();
    drop(rooms);

    if let Some(camp_arc) = camp_arc {
        let mut camp = camp_arc.write().await;
        if req.via_type == "via3" {
            let text = req.player_text.clone().unwrap_or_default();
            let lower = text.to_lowercase();
            let is_mechanic = lower.contains("atacar")
                || lower.contains("chutar")
                || lower.contains("empurrar")
                || lower.contains("esconder")
                || lower.contains("arrombar");

            if is_mechanic {
                let mechanic_payload = json!({
                    "action": "ability_check",
                    "ability": "Strength",
                    "actor": req.character_name,
                    "dc": 14
                });
                let resolution = rpg_engine::rules::resolver::RulesResolver::resolve(&mechanic_payload, &mut camp);
                return (
                    StatusCode::OK,
                    Json(TurnActionResponse {
                        status: "resolved".to_string(),
                        requires_mechanic: true,
                        detected_intent: "Manobra Tática / Ação Improvisada (#regras)".to_string(),
                        roll_result: Some(resolution),
                        narrative_feedback: format!("Mecânica resolvida deterministicamente para '{}'.", text),
                    }),
                );
            }
        }
    }

    (
        StatusCode::OK,
        Json(TurnActionResponse {
            status: "resolved".to_string(),
            requires_mechanic: false,
            detected_intent: "Ação Processada".to_string(),
            roll_result: None,
            narrative_feedback: "Ação de turno processada com sucesso.".to_string(),
        }),
    )
}

// =========================================================================
// T11: Gravação e Biblioteca de Premissas (data/premises/*.json)
// =========================================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SavedPremise {
    id: String,
    title: String,
    premise: String,
    literary_style: String,
    ai_creative_freedom: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    custom_monsters: Vec<Value>,
    created_at: String,
}

async fn list_saved_premises() -> (StatusCode, Json<Value>) {
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

async fn save_premise(Json(payload): Json<Value>) -> (StatusCode, Json<Value>) {
    let dir = "data/premises";
    if let Err(e) = std::fs::create_dir_all(dir) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "status": "error", "message": e.to_string() })),
        );
    }

    let id = payload["id"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let title = payload["title"]
        .as_str()
        .unwrap_or("Premissa Sem Título")
        .to_string();

    let mut full_payload = payload.clone();
    full_payload["id"] = json!(id);
    if full_payload.get("created_at").is_none() {
        full_payload["created_at"] = json!(chrono::Utc::now().to_rfc3339());
    }

    // Normalização e extração de Tags
    let raw_tags: Vec<String> = full_payload
        .get("tags")
        .and_then(|t| t.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();

    let mut tags = Vec::new();
    for t in raw_tags {
        for word in t.split_whitespace() {
            let clean = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
            if !clean.is_empty() && !tags.contains(&clean) {
                tags.push(clean);
            }
        }
    }

    if tags.is_empty() {
        let title_lower = title.to_lowercase();
        let style_lower = full_payload.get("literary_style").and_then(|s| s.as_str()).unwrap_or("").to_lowercase();
        let premise_lower = full_payload.get("premise").and_then(|s| s.as_str()).unwrap_or("").to_lowercase();
        let full_text = format!("{} {} {}", title_lower, style_lower, premise_lower);
        tags = rpg_engine::llm::skeleton_agent::SkeletonAgent::extract_deterministic_tags(&full_text);
    }
    let mut unique_tags = Vec::new();
    for tag in tags {
        if !unique_tags.contains(&tag) {
            unique_tags.push(tag);
        }
    }
    full_payload["tags"] = json!(unique_tags);

    let file_path = format!("{}/{}.json", dir, id);
    match serde_json::to_string_pretty(&full_payload) {
        Ok(json_str) => {
            if let Err(e) = std::fs::write(&file_path, json_str) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "status": "error", "message": e.to_string() })),
                );
            }
            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "id": id,
                    "title": title,
                    "message": "Premissa gravada com sucesso na biblioteca."
                })),
            )
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "status": "error", "message": e.to_string() })),
        ),
    }
}

async fn delete_saved_premise(axum::extract::Path(id): axum::extract::Path<String>) -> (StatusCode, Json<Value>) {
    let dir = "data/premises";
    let file_path = format!("{}/{}.json", dir, id);
    if std::path::Path::new(&file_path).exists() {
        if let Err(e) = std::fs::remove_file(&file_path) {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": e.to_string() })),
            );
        }
        (
            StatusCode::OK,
            Json(json!({ "status": "success", "message": "História pré-construída excluída com sucesso." })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "status": "error", "message": "História não encontrada." })),
        )
    }
}

// =========================================================================
// T12: Gravação Externa e Retomada de Campanhas
// =========================================================================

async fn list_saved_campaigns(State(state): State<ServerState>) -> (StatusCode, Json<Value>) {
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

async fn export_campaign_bundle(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
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

async fn import_campaign_bundle(
    State(state): State<ServerState>,
    Json(bundle): Json<Value>,
) -> (StatusCode, Json<Value>) {
    // Permite tanto bundle completo { manifest: { ... } } quanto o próprio manifest direto { campaign_id: ..., room_code: ... }
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

    // Salva manifest.json
    let _ = std::fs::write(
        format!("{}/manifest.json", base_dir),
        serde_json::to_string_pretty(manifest_val).unwrap_or_default(),
    );

    // Salva active_scene.json se existir
    if let Some(scene) = bundle.get("active_scene").filter(|s| !s.is_null()) {
        let _ = std::fs::write(
            format!("{}/active_scene.json", base_dir),
            serde_json::to_string_pretty(scene).unwrap_or_default(),
        );
    }

    // Salva chronicle_log.jsonl
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

    // Salva players
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

    // Salva world_entities
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

    // Registra na memória viva do servidor
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

async fn delete_saved_campaign(
    Path(campaign_id): Path<String>,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    let dir = format!("data/campaigns/{}", campaign_id);
    let path = std::path::Path::new(&dir);

    // Remove da memória viva se estiver carregada
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

    // Se o diretório existe no disco, exclui a campanha e seus dados
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

// =========================================================================
// T14: Chat da Party (HTTP fallback + WebSocket broadcast)
// =========================================================================

#[derive(Serialize, Deserialize, Debug)]
struct SendChatMessageRequest {
    sender_id: String,
    sender_name: String,
    text: String,
    #[serde(default)]
    is_action: bool,
}

async fn send_party_chat(
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

// =========================================================================
// T15: Modal de Auditoria Completa
// =========================================================================

async fn get_campaign_audit(
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

// =========================================================================
// T16: SaaS Autenticação (Argon2id) & Painel Administrativo
// =========================================================================

#[derive(Serialize, Deserialize, Debug)]
struct RegisterRequest {
    email: String,
    name: String,
    password: String,
    #[serde(default)]
    role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginRequest {
    email: String,
    password: String,
}

async fn auth_register(
    State(state): State<ServerState>,
    Json(req): Json<RegisterRequest>,
) -> (StatusCode, Json<Value>) {
    match state
        .user_store
        .register(&req.email, &req.name, &req.password, req.role.as_deref())
        .await
    {
        Ok((user, token)) => (
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "token": token,
                "user": user,
                "message": "Conta criada com sucesso."
            })),
        ),
        Err(err) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "status": "error", "message": err })),
        ),
    }
}

async fn auth_login(
    State(state): State<ServerState>,
    Json(req): Json<LoginRequest>,
) -> (StatusCode, Json<Value>) {
    match state.user_store.login(&req.email, &req.password).await {
        Ok((user, token)) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "token": token,
                "user": user,
                "message": "Login realizado com sucesso."
            })),
        ),
        Err(err) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "status": "error", "message": err })),
        ),
    }
}

async fn admin_auth_login(
    State(state): State<ServerState>,
    Json(req): Json<LoginRequest>,
) -> (StatusCode, Json<Value>) {
    match state.user_store.login_admin(&req.email, &req.password).await {
        Ok((user, token)) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "token": token,
                "user": user,
                "message": "Acesso administrativo autenticado com sucesso."
            })),
        ),
        Err(err) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "status": "error", "message": err })),
        ),
    }
}

async fn auth_me(
    headers: HeaderMap,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    let token = match extract_bearer_token(&headers) {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "status": "error", "message": "Token não fornecido." })),
            );
        }
    };

    match state.user_store.get_user_by_token(&token).await {
        Some(user) => (StatusCode::OK, Json(json!({ "status": "ok", "user": user }))),
        None => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "status": "error", "message": "Sessão inválida ou expirada." })),
        ),
    }
}

async fn auth_logout(
    headers: HeaderMap,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    if let Some(token) = extract_bearer_token(&headers) {
        state.user_store.logout(&token).await;
    }
    (StatusCode::OK, Json(json!({ "status": "ok", "message": "Logout realizado com sucesso." })))
}

async fn require_admin(headers: &HeaderMap, state: &ServerState) -> Result<UserPublic, (StatusCode, Json<Value>)> {
    let token = match extract_bearer_token(headers) {
        Some(t) => t,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "status": "error", "message": "Autenticação requerida." })),
            ));
        }
    };

    match state.user_store.get_user_by_token(&token).await {
        Some(user) => {
            if user.role == "admin" {
                Ok(user)
            } else {
                Err((
                    StatusCode::FORBIDDEN,
                    Json(json!({ "status": "error", "message": "Acesso restrito ao Painel Administrativo SaaS." })),
                ))
            }
        }
        None => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "status": "error", "message": "Sessão inválida." })),
        )),
    }
}

async fn admin_get_metrics(
    headers: HeaderMap,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    if let Err(res) = require_admin(&headers, &state).await {
        return res;
    }

    let users = state.user_store.list_users().await;
    let total_users = users.len();
    let total_hosts = users.iter().filter(|u| u.role == "host" || u.role == "admin").count();
    let total_players = users.iter().filter(|u| u.role == "player").count();

    let rooms = state.rooms.read().await;
    let active_rooms = rooms.len();
    drop(rooms);

    let mut saved_campaigns_count = 0;
    if let Ok(entries) = std::fs::read_dir("data/campaigns") {
        saved_campaigns_count = entries.flatten().count();
    }

    let mut saved_premises_count = 0;
    if let Ok(entries) = std::fs::read_dir("data/premises") {
        saved_premises_count = entries.flatten().count();
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "metrics": {
                "total_users": total_users,
                "total_hosts": total_hosts,
                "total_players": total_players,
                "active_rooms": active_rooms,
                "saved_campaigns": saved_campaigns_count,
                "saved_premises": saved_premises_count,
                "saas_tier": "Alpha Multi-Tenant"
            }
        })),
    )
}

async fn admin_list_users(
    headers: HeaderMap,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    if let Err(res) = require_admin(&headers, &state).await {
        return res;
    }

    let users = state.user_store.list_users().await;
    (StatusCode::OK, Json(json!({ "status": "ok", "users": users })))
}

async fn admin_save_premise(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    if let Err(res) = require_admin(&headers, &state).await {
        return res;
    }

    let dir = "data/premises";
    if let Err(e) = std::fs::create_dir_all(dir) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "status": "error", "message": e.to_string() })),
        );
    }

    let id = payload["id"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let title = payload["title"]
        .as_str()
        .unwrap_or("Premissa Sem Título")
        .to_string();

    let raw_premise = payload["premise"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let literary_style = payload["literary_style"]
        .as_str()
        .unwrap_or("Fantasia Sombria")
        .to_string();

    // Processo Revisor Editorial de IA (Duas Vias Gemini + DeepSeek)
    let should_refine = payload["auto_refine_ai"].as_bool().unwrap_or(true);
    let final_premise = if should_refine && !raw_premise.trim().is_empty() {
        println!(">>> [ADMIN_AI] Refinando premissa '{}' via processo editorial de IA...", title);
        match state.orchestrator.refine_premise(&title, &raw_premise, &literary_style).await {
            Ok(refined) => {
                println!(">>> [ADMIN_AI] Premissa refinada com sucesso!");
                refined
            }
            Err(e) => {
                println!(">>> [ADMIN_AI] Fallback de refinamento: {}. Usando premissa original.", e);
                raw_premise.clone()
            }
        }
    } else {
        raw_premise.clone()
    };

    let mut full_payload = payload.clone();
    full_payload["id"] = json!(id);
    full_payload["premise"] = json!(final_premise);
    full_payload["raw_premise_submitted"] = json!(raw_premise);
    full_payload["literary_style"] = json!(literary_style);
    full_payload["ai_refined"] = json!(should_refine);

    // Se o payload já traz entidades da prévia, usa-as diretamente; caso contrário, roda a análise
    let analysis_opt = if full_payload.get("custom_monsters").is_none() || full_payload["custom_monsters"].as_array().map_or(true, |a| a.is_empty()) {
        let combined_for_analysis = format!("{}\n\nREGISTROS E ENTIDADES HOMEBREW ORIGINAIS:\n{}", final_premise, raw_premise);
        let analysis = match state.skeleton_agent.analyze_premise_requirements(&combined_for_analysis, &literary_style, 10).await {
            Ok(v) => v,
            Err(_) => SkeletonAgent::deterministic_premise_analysis(&combined_for_analysis, 10),
        };
        if let Some(monsters) = analysis.get("custom_monsters") {
            full_payload["custom_monsters"] = monsters.clone();
        }
        if let Some(overrides) = analysis.get("subclass_overrides") {
            full_payload["subclass_overrides"] = overrides.clone();
        }
        if let Some(spells) = analysis.get("custom_spells") {
            full_payload["custom_spells"] = spells.clone();
        }
        Some(analysis)
    } else {
        None
    };

    if full_payload.get("created_at").is_none() {
        full_payload["created_at"] = json!(chrono::Utc::now().to_rfc3339());
    }

    // Normalização e extração de Tags no Admin
    let mut raw_tags: Vec<String> = full_payload
        .get("tags")
        .and_then(|t| t.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();

    if raw_tags.is_empty() {
        if let Some(ref analysis) = analysis_opt {
            if let Some(analysis_tags) = analysis.get("tags").and_then(|t| t.as_array()) {
                for at in analysis_tags {
                    if let Some(s) = at.as_str() {
                        let trimmed = s.trim().to_string();
                        if !trimmed.is_empty() && !raw_tags.contains(&trimmed) {
                            raw_tags.push(trimmed);
                        }
                    }
                }
            }
        }
    }

    let mut tags = Vec::new();
    for t in raw_tags {
        for word in t.split_whitespace() {
            let clean = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
            if !clean.is_empty() && !tags.contains(&clean) {
                tags.push(clean);
            }
        }
    }

    if tags.is_empty() {
        let title_lower = title.to_lowercase();
        let style_lower = literary_style.to_lowercase();
        let premise_lower = raw_premise.to_lowercase();
        let full_text = format!("{} {} {}", title_lower, style_lower, premise_lower);
        tags = rpg_engine::llm::skeleton_agent::SkeletonAgent::extract_deterministic_tags(&full_text);
    }
    let mut unique_tags = Vec::new();
    for tag in tags {
        if !unique_tags.contains(&tag) {
            unique_tags.push(tag);
        }
    }
    full_payload["tags"] = json!(unique_tags);

    let file_path = format!("{}/{}.json", dir, id);
    match serde_json::to_string_pretty(&full_payload) {
        Ok(json_str) => {
            if let Err(e) = std::fs::write(&file_path, json_str) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "status": "error", "message": e.to_string() })),
                );
            }
            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "id": id,
                    "title": title,
                    "premise": final_premise,
                    "ai_refined": should_refine,
                    "message": "Premissa revisada por IA e gravada com sucesso no catálogo oficial."
                })),
            )
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "status": "error", "message": e.to_string() })),
        ),
    }
}

async fn admin_preview_premise(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    if let Err(res) = require_admin(&headers, &state).await {
        return res;
    }

    let title = payload["title"]
        .as_str()
        .unwrap_or("Premissa Sem Título")
        .to_string();

    let raw_premise = payload["premise"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let literary_style = payload["literary_style"]
        .as_str()
        .unwrap_or("Fantasia Sombria")
        .to_string();

    let slider = payload["dramatic_focus_slider"]
        .as_u64()
        .unwrap_or(10) as u32;

    let should_refine = payload["auto_refine_ai"].as_bool().unwrap_or(true);

    let refined_premise = if should_refine && !raw_premise.trim().is_empty() {
        println!(">>> [ADMIN_AI] Gerando prévia de refinamento para '{}'...", title);
        match state.orchestrator.refine_premise(&title, &raw_premise, &literary_style).await {
            Ok(refined) => refined,
            Err(e) => {
                println!(">>> [ADMIN_AI] Fallback de prévia: {}. Mantendo original.", e);
                raw_premise.clone()
            }
        }
    } else {
        raw_premise.clone()
    };

    // Analisa a composição resultante e overrides/homebrew considerando tanto o texto refinado quanto o bruto submetido
    let combined_for_analysis = format!("{}\n\nREGISTROS E ENTIDADES HOMEBREW ORIGINAIS:\n{}", refined_premise, raw_premise);
    let analysis = match state.skeleton_agent.analyze_premise_requirements(&combined_for_analysis, &literary_style, slider).await {
        Ok(v) => v,
        Err(_) => SkeletonAgent::deterministic_premise_analysis(&combined_for_analysis, slider),
    };

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "title": title,
            "raw_premise": raw_premise,
            "refined_premise": refined_premise,
            "literary_style": literary_style,
            "dramatic_focus_slider": slider,
            "ai_refined": should_refine,
            "analysis": analysis
        })),
    )
}

async fn admin_delete_premise(
    headers: HeaderMap,
    State(state): State<ServerState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> (StatusCode, Json<Value>) {
    if let Err(res) = require_admin(&headers, &state).await {
        return res;
    }

    let dir = "data/premises";
    let file_path = format!("{}/{}.json", dir, id);
    if std::path::Path::new(&file_path).exists() {
        if let Err(e) = std::fs::remove_file(&file_path) {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": e.to_string() })),
            );
        }
        (
            StatusCode::OK,
            Json(json!({ "status": "success", "message": "História pré-construída excluída do catálogo oficial." })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "status": "error", "message": "História não encontrada." })),
        )
    }
}

