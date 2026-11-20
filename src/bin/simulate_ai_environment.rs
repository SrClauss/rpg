use dotenvy::dotenv;
use rpg_engine::llm::EnvironmentAgent;
use rpg_engine::narrative::entity_store::{EntityDndStats, EntityStore, EntityType, WorldEntitySheet};
use rpg_engine::narrative::living_scene::{ActionsBudget, LivingScene, SceneObjective, TurnParticipant};
use rpg_engine::state::{CampaignManifest, CampaignState};
use rpg_engine::narrative::skeleton::NarrativeSkeleton;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("================================================================================");
    println!("    SIMULAÇÃO DE TURNO HÍBRIDO: IA (GEMINI) + MOTOR DETERMINÍSTICO D&D 5E");
    println!("================================================================================");

    let gemini_key = env::var("GEMINI_API_KEY").unwrap_or_default();
    let agent = EnvironmentAgent::new(gemini_key);

    // 1. Criar ambiente de teste no disco temporário
    let temp_dir = std::env::temp_dir().join(format!("rpg_ai_sim_{}", uuid::Uuid::new_v4()));
    let campaign_id = "campanha_simulacao_ia";
    let camp_dir = temp_dir.join("data").join("campaigns").join(campaign_id);
    std::fs::create_dir_all(&camp_dir)?;

    let entity_store = EntityStore::new(&camp_dir)?;

    // 2. Criar a ficha do Herói
    let mut hero_sheet = WorldEntitySheet::new("player_1", "Thorne, o Ladino", EntityType::Player, "Rogue");
    let mut hero_dnd = EntityDndStats::default();
    hero_dnd.max_hp = 22;
    hero_dnd.current_hp = 22;
    hero_dnd.armor_class = 15;
    hero_dnd.ability_scores.insert("DEX".to_string(), 16);
    hero_dnd.ability_scores.insert("STR".to_string(), 10);
    hero_sheet.dnd_stats = Some(hero_dnd);
    entity_store.save_player_entity(&hero_sheet)?;

    // 3. Criar a ficha do Antagonista (NPC com memória de trauma)
    let mut cultist_sheet = WorldEntitySheet::new("cultist_boss", "Malakor, o Necromante", EntityType::Npc, "Necromante");
    let mut cult_dnd = EntityDndStats::default();
    cult_dnd.max_hp = 30;
    cult_dnd.current_hp = 30;
    cult_dnd.armor_class = 13;
    cultist_sheet.dnd_stats = Some(cult_dnd);
    entity_store.save_world_entity(&cultist_sheet)?;

    // 4. Montar a Cena Viva
    let objectives = vec![
        SceneObjective {
            id: "obj_ritual".to_string(),
            description: "Interromper o ritual proibido de Malakor antes que ele invoque o morto-vivo".to_string(),
            completed: false,
            completion_trigger: None,
        },
    ];

    let participants = vec![
        TurnParticipant {
            id: "player_1".to_string(),
            name: "Thorne, o Ladino".to_string(),
            is_player: true,
            is_hostile: false,
            initiative: 19,
            initiative_roll: 16,
            initiative_modifier: 3,
            current_hp: 22,
            max_hp: 22,
            armor_class: 15,
            is_dead: false,
            actions_budget: ActionsBudget::default(),
            position: Some((15, 10)),
            token_role: Some("Rogue".to_string()),
        },
        TurnParticipant {
            id: "cultist_boss".to_string(),
            name: "Malakor, o Necromante".to_string(),
            is_player: false,
            is_hostile: true,
            initiative: 12,
            initiative_roll: 11,
            initiative_modifier: 1,
            current_hp: 30,
            max_hp: 30,
            armor_class: 13,
            is_dead: false,
            actions_budget: ActionsBudget::default(),
            position: Some((35, 10)),
            token_role: Some("Boss".to_string()),
        },
        TurnParticipant {
            id: "skeleton_minion".to_string(),
            name: "Esqueleto Guardião".to_string(),
            is_player: false,
            is_hostile: true,
            initiative: 8,
            initiative_roll: 6,
            initiative_modifier: 2,
            current_hp: 13,
            max_hp: 13,
            armor_class: 13,
            is_dead: false,
            actions_budget: ActionsBudget::default(),
            position: Some((25, 10)),
            token_role: Some("Inimigo".to_string()),
        },
    ];

    let mut scene = LivingScene::new(
        campaign_id,
        "O Círculo de Sangue na Cripta",
        "Câmara Subterrânea de Sacrifícios",
        1,
        objectives,
        participants,
    );

    let mut state = CampaignState::new();
    state.manifest = Some(CampaignManifest {
        campaign_id: campaign_id.to_string(),
        room_code: "SIM1".to_string(),
        host_player_id: "player_1".to_string(),
        player_ids: vec!["player_1".to_string()],
        player_slots: vec![],
        premise: "Nas catacumbas esquecidas sob a capital, cultistas de Mumm-Ra realizam sacrifícios com sangue de necromantes para despertar um antigo flagelo cósmico.".to_string(),
        literary_style: "Dark Fantasy / Lovecraftian".to_string(),
        hidden_skeleton: NarrativeSkeleton::new("O Flagelo Oculto", "Dark Fantasy"),
        created_at: "2026-11-20".to_string(),
        status: "active".to_string(),
        custom_monsters: vec![],
        world_prologue: None,
    });

    println!("\n[ESTADO INICIAL DA CENA VIVA]");
    println!("📍 Local: {} | {}", scene.title, scene.location);
    println!("🎯 Objetivo: {}", scene.objectives[0].description);
    println!("👥 Participantes:");
    for p in &scene.participants {
        println!("   - {} (PV: {}/{}, CA: {}, Posição: {:?})", p.name, p.current_hp, p.max_hp, p.armor_class, p.position.unwrap());
    }

    // -------------------------------------------------------------
    // RODADA 1: Ação Criativa do Jogador (Via 1 / Fala em Linguagem Natural)
    // -------------------------------------------------------------
    let player_action = "Corro pelas sombras das colunas, saco minha adaga e salto pelas costas do Esqueleto Guardião tentando desarticular suas costelas com um golpe preciso!";
    println!("\n================================================================================");
    println!("👉 AÇÃO LIVRE DO JOGADOR (LINGUAGEM NATURAL):");
    println!("\"{}\"", player_action);
    println!("================================================================================");

    println!("\n⏳ Consultando Agente de Ambiente (Gemini) e acionando Motor Determinístico D&D 5e...");

    let turn_output = agent.process_turn(
        &mut scene,
        "player_1",
        "Thorne, o Ladino",
        "via1",
        player_action,
        Some("attack_dagger"),
        &mut state,
        &entity_store,
    ).await?;

    println!("\n================================================================================");
    println!("🎭 DECISÃO NARRATIVA & REATIVIDADE DO AMBIENTE (GERADA PELA IA):");
    println!("================================================================================");
    println!("{}", turn_output.narrative_reaction);

    println!("\n================================================================================");
    println!("⚙️ INTENÇÃO MECÂNICA DETECTADA & RESOLUÇÃO D&D 5E:");
    println!("================================================================================");
    if let Some(ref intent) = turn_output.mechanical_intent {
        println!("🔹 Intenção Mecânica: {}", serde_json::to_string_pretty(intent)?);
    }
    if let Some(ref rules) = turn_output.rules_result {
        println!("🔹 Resolução Mecânica: {}", serde_json::to_string_pretty(rules)?);
    }

    println!("\n================================================================================");
    println!("🗺️ NOVO ESTADO TÁTICO DOS COMBATENTES (APÓS A REAÇÃO DOS INIMIGOS):");
    println!("================================================================================");
    for p in &scene.participants {
        let status = if p.is_dead { "💀 MORTO" } else { "⚔️ VIVO" };
        println!("   - {} | PV: {}/{} | Posição: {:?} | Status: {}", p.name, p.current_hp, p.max_hp, p.position.unwrap(), status);
    }

    println!("\n================================================================================");
    println!("                     SIMULAÇÃO CONCLUÍDA COM SUCESSO!");
    println!("================================================================================");

    Ok(())
}
