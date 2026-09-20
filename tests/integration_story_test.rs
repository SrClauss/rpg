use rpg_engine::narrative::{
    NarrativeDb, NarrativeNode, NarrativeEdge, NodeType, ContextualMemory, EventLog, EntityStats
};
use rpg_engine::rules::monsters::registry::BestiaryRegistry;

#[test]
fn test_full_story_integration() {
    // ---------------------------------------------------------
    // 1. INICIALIZAÇÃO DO MUNDO
    // ---------------------------------------------------------
    // Cria banco em memória vazio com todas as tabelas e índices
    let db = NarrativeDb::new_in_memory().expect("Falha ao criar o DB SQLite");
    
    // Inicializa os monstros (DeepSeek/Rules Engine)
    let bestiary = BestiaryRegistry::new();

    // ---------------------------------------------------------
    // 2. CRIAÇÃO DO GRAFO NARRATIVO (O MAPA)
    // ---------------------------------------------------------
    let tavern_node = NarrativeNode::new(NodeType::Scene, "Taverna do Javali Saltitante", "Cheiro de cerveja e suor.");
    tavern_node.insert(&db).unwrap();

    let forest_node = NarrativeNode::new(NodeType::Combat, "Trilha Escura na Floresta", "Árvores altas bloqueiam a luz.");
    forest_node.insert(&db).unwrap();

    // Conecta Taverna à Floresta
    let edge_to_forest = NarrativeEdge::new(
        tavern_node.id, 
        forest_node.id, 
        "Viajar para a floresta escutando os rumores", 
        Some("player.level >= 1")
    );
    edge_to_forest.insert(&db).unwrap();

    // ---------------------------------------------------------
    // 3. INTERAÇÃO 1: SEM BATALHA (DIÁLOGO)
    // ---------------------------------------------------------
    // O Jogador fala com o Bartender.
    let dialog_memory = ContextualMemory::new(
        tavern_node.id,
        "npc",
        "Bartender",
        "dialogue",
        "O Herói Claus perguntou sobre Goblins. Eu avisei que estão na Trilha Escura.",
        Some("friendly"),
    );
    dialog_memory.record(&db).unwrap();

    // DeepSeek loga cada fala atômica na Telemetria
    let dialog_log = EventLog::new(
        &tavern_node.id.to_string(),
        "dialogue_action",
        "Claus",
        Some("Bartender"),
        "Perguntou sobre os perigos locais",
        Some(r#"{"topic": "goblins", "skill_check": "persuasion", "roll": 18}"#),
    );
    dialog_log.insert(&db).unwrap();

    // ---------------------------------------------------------
    // 4. INTERAÇÃO 2: ENCONTRO COM LUTA
    // ---------------------------------------------------------
    // Jogador viaja e encontra o monstro. Motor busca do Bestiário.
    let goblin = bestiary.get_monster("goblin").expect("Goblin não encontrado no SRD!");
    
    // Simulando Turno 1: Jogador ataca o Goblin e causa 6 de dano slashing.
    let combat_log = EventLog::new(
        &forest_node.id.to_string(),
        "combat_roll",
        "Claus",
        Some("Goblin"),
        "Claus atacou com espada longa e acertou. Causou 6 de dano.",
        Some(r#"{"roll": 15, "damage": 6, "damage_type": "slashing", "weapon": "longsword"}"#),
    );
    combat_log.insert(&db).unwrap();

    // Motor Tático atualiza a vida do Goblin em memória (não coberto no SQLite pq é volátil)
    let _goblin_hp_remaining = goblin.entity.base_max_hp - 6; // Sobra 1 de HP

    // Mas a telemetria agregada grava os danos para a vida toda!
    let mut claus_stats = EntityStats::get_or_create(&db, "Claus").unwrap();
    claus_stats.add_damage_dealt(&db, 6).unwrap();

    let mut goblin_stats = EntityStats::get_or_create(&db, "Goblin").unwrap();
    goblin_stats.total_damage_taken += 6;
    goblin_stats.save(&db).unwrap();

    // Turno 2: Jogador ataca novamente e mata o Goblin.
    let fatal_log = EventLog::new(
        &forest_node.id.to_string(),
        "combat_roll",
        "Claus",
        Some("Goblin"),
        "Claus atacou o Goblin, esmagando-o com um golpe crítico letal.",
        Some(r#"{"roll": 20, "damage": 8, "is_critical": true}"#),
    );
    fatal_log.insert(&db).unwrap();

    claus_stats.add_damage_dealt(&db, 8).unwrap();
    claus_stats.enemies_defeated += 1; // +1 Kill
    claus_stats.critical_hits_scored += 1;
    claus_stats.save(&db).unwrap();

    // Grava na Memória Contextual que o Herói matou um Goblin
    let death_memory = ContextualMemory::new(
        forest_node.id,
        "player",
        "Claus",
        "combat_victory",
        "Eu derrotei um Goblin perigoso na trilha escura, salvando a região.",
        Some("triumphant"),
    );
    death_memory.record(&db).unwrap();


    // ---------------------------------------------------------
    // 5. ASSERÇÕES E VALIDAÇÕES DO LLM (A auditoria)
    // ---------------------------------------------------------
    // Verificando a memória do Bartender
    let bartender_memories = ContextualMemory::fetch_for_entity(&db, "Bartender").unwrap();
    assert_eq!(bartender_memories.len(), 1);
    assert_eq!(bartender_memories[0].emotional_state.as_deref(), Some("friendly"));

    // Verificando a memória do Herói
    let claus_memories = ContextualMemory::fetch_for_entity(&db, "Claus").unwrap();
    assert_eq!(claus_memories.len(), 1);
    assert!(claus_memories[0].context_summary.contains("derrotei um Goblin"));

    // Verificando a Telemetria Atômica (EventLogs)
    let logs_count: i32 = db.conn.query_row("SELECT COUNT(*) FROM EventLogs", [], |row| row.get(0)).unwrap();
    assert_eq!(logs_count, 3); // 1 Dialogo, 2 Ataques

    let crit_log_raw: String = db.conn.query_row(
        "SELECT raw_json_data FROM EventLogs WHERE description LIKE '%crítico letal%'", 
        [], 
        |row| row.get(0)
    ).unwrap();
    assert!(crit_log_raw.contains(r#""is_critical": true"#));

    // Verificando as Estatísticas Agregadas
    let final_claus_stats = EntityStats::get_or_create(&db, "Claus").unwrap();
    assert_eq!(final_claus_stats.total_damage_dealt, 14); // 6 + 8
    assert_eq!(final_claus_stats.enemies_defeated, 1);
    assert_eq!(final_claus_stats.critical_hits_scored, 1);

    let final_goblin_stats = EntityStats::get_or_create(&db, "Goblin").unwrap();
    assert_eq!(final_goblin_stats.total_damage_taken, 6); // Teste parou de contar pq ele morreu rs.
}
