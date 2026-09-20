use rpg_engine::narrative::{
    NarrativeDb, NarrativeNode, NarrativeEdge, NodeType, ContextualMemory, EventLog, EntityStats
};
use rpg_engine::rules::monsters::registry::BestiaryRegistry;

fn main() {
    println!("============================================================");
    println!("          DEMONSTRAÇÃO: GRAFO NARRATIVO & TELEMETRIA         ");
    println!("============================================================\n");

    let db = NarrativeDb::new_in_memory().expect("Falha ao criar o DB SQLite");
    let bestiary = BestiaryRegistry::new();

    println!("[SISTEMA] Banco de Dados Inicializado.");
    println!("[SISTEMA] Bestiário Carregado com {} monstros.\n", bestiary.list_monster_ids().len());

    // ---------------------------------------------------------
    // CENA 1
    // ---------------------------------------------------------
    let tavern_node = NarrativeNode::new(NodeType::Scene, "Taverna do Javali Saltitante", "Cheiro de cerveja e suor. Bardos tocam ao fundo.");
    tavern_node.insert(&db).unwrap();
    println!(">> NOVA CENA: {}", tavern_node.title);
    println!(">> DESCRIÇÃO: {}\n", tavern_node.description);

    println!("[AÇÃO] Claus fala com o Bartender...");
    let dialog_memory = ContextualMemory::new(
        tavern_node.id,
        "npc",
        "Bartender",
        "dialogue",
        "O Herói Claus perguntou sobre Goblins. Eu avisei que estão na Trilha Escura.",
        Some("friendly"),
    );
    dialog_memory.record(&db).unwrap();

    let dialog_log = EventLog::new(
        &tavern_node.id.to_string(),
        "dialogue_action",
        "Claus",
        Some("Bartender"),
        "Perguntou sobre os perigos locais",
        Some(r#"{"topic": "goblins", "skill_check": "persuasion", "roll": 18}"#),
    );
    dialog_log.insert(&db).unwrap();
    println!("[TELEMETRIA] Log salvo no banco! (Rolagem de Persuasão: 18)\n");

    // ---------------------------------------------------------
    // CENA 2
    // ---------------------------------------------------------
    let forest_node = NarrativeNode::new(NodeType::Combat, "Trilha Escura na Floresta", "Árvores altas bloqueiam a luz. Você ouve rosnados.");
    forest_node.insert(&db).unwrap();
    println!(">> NOVA CENA: {}", forest_node.title);
    println!(">> DESCRIÇÃO: {}\n", forest_node.description);

    let goblin = bestiary.get_monster("goblin").expect("Goblin não encontrado no SRD!");
    println!("[SISTEMA] Instanciando monstro: {} (HP Inicial: {})\n", goblin.entity.name, goblin.entity.base_max_hp);

    println!("[AÇÃO] Turno 1: Claus ataca o Goblin!");
    let combat_log = EventLog::new(
        &forest_node.id.to_string(),
        "combat_roll",
        "Claus",
        Some("Goblin"),
        "Claus atacou com espada longa e acertou. Causou 6 de dano.",
        Some(r#"{"roll": 15, "damage": 6, "damage_type": "slashing", "weapon": "longsword"}"#),
    );
    combat_log.insert(&db).unwrap();

    let mut claus_stats = EntityStats::get_or_create(&db, "Claus").unwrap();
    claus_stats.add_damage_dealt(&db, 6).unwrap();

    println!("[AÇÃO] Turno 2: Claus ataca o Goblin novamente!");
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
    claus_stats.enemies_defeated += 1;
    claus_stats.critical_hits_scored += 1;
    claus_stats.save(&db).unwrap();

    let death_memory = ContextualMemory::new(
        forest_node.id,
        "player",
        "Claus",
        "combat_victory",
        "Eu derrotei um Goblin perigoso na trilha escura, salvando a região.",
        Some("triumphant"),
    );
    death_memory.record(&db).unwrap();
    println!("[TELEMETRIA] Combate finalizado. Goblin derrotado.\n");

    // ---------------------------------------------------------
    // RESULTADOS DO BANCO DE DADOS
    // ---------------------------------------------------------
    println!("============================================================");
    println!("          O QUE O BANCO DE DADOS (IA) GRAVOU?               ");
    println!("============================================================\n");

    let bartender_memories = ContextualMemory::fetch_for_entity(&db, "Bartender").unwrap();
    println!("🧠 MEMÓRIA DO BARTENDER:");
    println!("   Resumo: '{}'", bartender_memories[0].context_summary);
    println!("   Estado Emocional em relação a Claus: {}\n", bartender_memories[0].emotional_state.as_deref().unwrap_or("Neutro"));

    let claus_memories = ContextualMemory::fetch_for_entity(&db, "Claus").unwrap();
    println!("🧠 MEMÓRIA DE CLAUS:");
    println!("   Resumo: '{}'", claus_memories[0].context_summary);
    println!("   Estado Emocional: {}\n", claus_memories[0].emotional_state.as_deref().unwrap_or("Neutro"));

    let final_claus_stats = EntityStats::get_or_create(&db, "Claus").unwrap();
    println!("📊 ESTATÍSTICAS PERMANENTES DE CLAUS:");
    println!("   Total Dano Causado: {}", final_claus_stats.total_damage_dealt);
    println!("   Inimigos Derrotados: {}", final_claus_stats.enemies_defeated);
    println!("   Críticos Executados: {}\n", final_claus_stats.critical_hits_scored);

    let mut stmt = db.conn.prepare("SELECT timestamp, description, raw_json_data FROM EventLogs").unwrap();
    let logs: Vec<(String, String)> = stmt.query_map([], |row| {
        Ok((row.get(1)?, row.get(2)?))
    }).unwrap().map(|r| r.unwrap()).collect();

    println!("📜 EVENT LOGS BRUTOS (Telemetria do Motor):");
    for (desc, json) in logs {
        println!("   - DESC: {}", desc);
        println!("     JSON: {}", json);
    }
}
