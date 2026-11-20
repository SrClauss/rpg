use rpg_engine::narrative::chronicle::{ChronicleBook, ChronicleEntry, ChronicleEntryType};
use rpg_engine::narrative::entity_store::{EntityStore, EntityType, TargetSpecificModifier, WorldEntitySheet};
use rpg_engine::narrative::living_scene::{LivingScene, SceneObjective, SceneResolutionReason, TurnParticipant, ActionsBudget};
use rpg_engine::narrative::skeleton::NarrativeSkeleton;

#[tokio::test]
async fn test_full_campaign_living_scene_flow() {
    let test_dir = std::env::temp_dir().join(format!("rpg_test_{}", uuid::Uuid::new_v4()));
    let campaign_id = "test_camp_123";
    let campaign_path = test_dir.join("data").join("campaigns").join(campaign_id);
    std::fs::create_dir_all(&campaign_path).unwrap();

    // 1. Esqueleto Narrativo Oculto
    let mut skeleton = NarrativeSkeleton::new(
        "Inverno eterno e lobisomens assolando as vilas",
        "Fantasia Sombria",
    );
    assert_eq!(skeleton.expected_acts.len(), 0);
    skeleton.expected_acts.push(rpg_engine::narrative::skeleton::SkeletonNode {
        id: "act_1".to_string(),
        act_number: 1,
        title: "O Primeiro Ataque".to_string(),
        synopsis: "Os lobisomens cercam a estalagem".to_string(),
        expected_milestones: vec!["Proteger a estalagem".to_string()],
        completed: false,
        arc_name: None,
        arc_number: None,
        boss_name: None,
        battle_intensity: None,
    });
    assert_eq!(skeleton.current_act().unwrap().title, "O Primeiro Ataque");

    // 2. Livro da Crônica Indexado O(1)
    let mut chronicle = ChronicleBook::new(&campaign_path).unwrap();
    let mut entry1 = ChronicleEntry::new(
        1,
        "Ataque à Estalagem",
        ChronicleEntryType::LiteraryChapter,
        "A lua sangrenta iluminava os telhados nevados enquanto uivos ecoavam.",
        Vec::new(),
        vec!["joao".to_string()],
    );
    let event_id_1 = chronicle.append_entry(&mut entry1).unwrap();
    assert_eq!(event_id_1, "evt_0001");

    // Teste de busca O(1)
    let fetched = chronicle.get_entry_by_id(&event_id_1).unwrap().unwrap();
    assert_eq!(fetched.scene_title, "Ataque à Estalagem");
    assert_eq!(fetched.involved_entities, vec!["joao"]);

    // 3. Ficha Viva de Entidade com Ponteiro de Memória
    let entity_store = EntityStore::new(&campaign_path).unwrap();
    let mut joao = WorldEntitySheet::new("joao", "João da Silva", EntityType::Npc, "Moleiro local traumatizado");

    // Adiciona impacto referenciando o evento 1
    joao.add_impact(
        &event_id_1,
        "Viu a esposa ser levada pelos lobisomens",
        "Trauma severo com bestas lupinas",
        vec![TargetSpecificModifier {
            target_actor_id: "player_rogue".to_string(),
            dc_adjustment: 5,
            dice_mode: Some(rpg_engine::dice::DiceMode::Disadvantage),
            reason: "Desconfiança do ladino que fugiu".to_string(),
        }],
    );
    entity_store.save_world_entity(&joao).unwrap();

    // Testa modificador contra o ladino
    let loaded_joao = entity_store.get_world_entity("joao").unwrap().unwrap();
    let (dc_adj, dice_mode) = loaded_joao.get_modifiers_against("player_rogue");
    assert_eq!(dc_adj, 5);
    assert_eq!(dice_mode, Some(rpg_engine::dice::DiceMode::Disadvantage));

    // Contra outro jogador não deve haver penalidade
    let (dc_adj_other, dice_mode_other) = loaded_joao.get_modifiers_against("player_paladin");
    assert_eq!(dc_adj_other, 0);
    assert_eq!(dice_mode_other, None);

    // 4. Cena Viva com Prevenção de Loops via Objetivos
    let objectives = vec![
        SceneObjective {
            id: "obj_barricade".to_string(),
            description: "Reforçar as portas da estalagem".to_string(),
            completed: false,
            completion_trigger: None,
        },
        SceneObjective {
            id: "obj_pacify".to_string(),
            description: "Acalmar João em choque".to_string(),
            completed: false,
            completion_trigger: None,
        },
    ];

    let participants = vec![
        TurnParticipant {
            id: "player_paladin".to_string(),
            name: "Paladino Arthur".to_string(),
            is_player: true,
            is_hostile: false,
            initiative: 14,
            initiative_roll: 12,
            initiative_modifier: 2,
            current_hp: 14,
            max_hp: 14,
            armor_class: 16,
            is_dead: false,
            actions_budget: ActionsBudget::default(),
            position: Some((10, 10)),
            token_role: Some("Paladin".to_string()),
        },
        TurnParticipant {
            id: "joao".to_string(),
            name: "João".to_string(),
            is_player: false,
            is_hostile: false,
            initiative: 8,
            initiative_roll: 8,
            initiative_modifier: 0,
            current_hp: 10,
            max_hp: 10,
            armor_class: 11,
            is_dead: false,
            actions_budget: ActionsBudget::default(),
            position: Some((18, 20)),
            token_role: Some("Aliado".to_string()),
        },
    ];

    let mut scene = LivingScene::new(
        campaign_id,
        "Defesa da Estalagem",
        "Estalagem da Encruzilhada",
        1,
        objectives,
        participants,
    );

    assert_eq!(scene.participants[0].id, "player_paladin");
    assert_eq!(scene.check_completion(), None);

    // Cumpre os objetivos
    scene.mark_objective_completed("obj_barricade");
    assert_eq!(scene.check_completion(), None);

    scene.mark_objective_completed("obj_pacify");
    assert_eq!(scene.check_completion(), Some(SceneResolutionReason::ObjectivesCompleted));

    // Fecha a cena viva
    scene.close_scene(SceneResolutionReason::ObjectivesCompleted, "A estalagem foi protegida e João recuperou os sentidos.");
    assert!(!scene.is_active);

    // Anexa resumo no livro da crônica
    let mut scene_entry = ChronicleEntry::new(
        1,
        "Defesa da Estalagem - Resumo",
        ChronicleEntryType::LivingSceneSummary,
        scene.summary.as_ref().unwrap(),
        Vec::new(),
        vec!["player_paladin".to_string(), "joao".to_string()],
    );
    let event_id_2 = chronicle.append_entry(&mut scene_entry).unwrap();
    assert_eq!(event_id_2, "evt_0002");

    // Verifica que ambas as entradas estão indexadas e recuperáveis
    let all = chronicle.get_all_entries().unwrap();
    assert_eq!(all.len(), 2);
    assert_eq!(all[0].event_id, "evt_0001");
    assert_eq!(all[1].event_id, "evt_0002");
}

#[test]
fn test_tactical_combat_damage_enemy_turns_and_victory() {
    use rpg_engine::llm::EnvironmentAgent;
    use rpg_engine::narrative::entity_store::EntityDndStats;
    use rpg_engine::rules::resolver::RulesResolver;
    use rpg_engine::state::CampaignState;
    use serde_json::json;

    let test_dir = std::env::temp_dir().join(format!("rpg_tactical_test_{}", uuid::Uuid::new_v4()));
    let campaign_id = "test_tactical_camp";
    let campaign_path = test_dir.join("data").join("campaigns").join(campaign_id);
    std::fs::create_dir_all(&campaign_path).unwrap();

    let entity_store = EntityStore::new(&campaign_path).unwrap();
    let mut hero_sheet = WorldEntitySheet::new("player_1", "Hagnar", EntityType::Player, "Guerreiro");
    let mut dnd = EntityDndStats::default();
    dnd.max_hp = 20;
    dnd.current_hp = 20;
    dnd.armor_class = 12;
    dnd.ability_scores.insert("STR".to_string(), 18);
    hero_sheet.dnd_stats = Some(dnd);
    entity_store.save_player_entity(&hero_sheet).unwrap();

    let objectives = vec![SceneObjective {
        id: "obj_1".to_string(),
        description: "Eliminar os batedores goblins na entrada da cova".to_string(),
        completed: false,
        completion_trigger: None,
    }];

    let participants = vec![
        TurnParticipant {
            id: "player_1".to_string(),
            name: "Hagnar".to_string(),
            is_player: true,
            is_hostile: false,
            initiative: 18,
            initiative_roll: 16,
            initiative_modifier: 2,
            current_hp: 20,
            max_hp: 20,
            armor_class: 12,
            is_dead: false,
            actions_budget: ActionsBudget::default(),
            position: Some((10, 10)),
            token_role: Some("Fighter".to_string()),
        },
        TurnParticipant {
            id: "goblin_1".to_string(),
            name: "Batedor Goblin #1".to_string(),
            is_player: false,
            is_hostile: true,
            initiative: 12,
            initiative_roll: 10,
            initiative_modifier: 2,
            current_hp: 4,
            max_hp: 12,
            armor_class: 5, // Garante acerto exceto falha crítica
            is_dead: false,
            actions_budget: ActionsBudget::default(),
            position: Some((30, 10)),
            token_role: Some("Inimigo".to_string()),
        },
    ];

    let mut scene = LivingScene::new(
        campaign_id,
        "Emboscada na Cova",
        "Caverna Subterrânea",
        1,
        objectives,
        participants,
    );

    assert!(scene.in_combat);
    assert!(scene.has_live_hostiles());
    assert_eq!(scene.obstacles.len(), 5);

    // 1. Executa turno dos inimigos via advance_to_next_player_and_run_npcs
    let (logs, turns, victory) =
        EnvironmentAgent::advance_to_next_player_and_run_npcs(&mut scene, &entity_store);
    assert!(!victory);
    assert_eq!(turns.len(), 1);
    assert_eq!(logs.len(), 1);
    // Verifica que o turno voltou para o jogador vivo com orçamento renovado
    assert_eq!(scene.participants[scene.current_participant_index].id, "player_1");
    assert!(!scene.participants[scene.current_participant_index].actions_budget.action_used);

    // 2. Ataca o goblin até matá-lo usando RulesResolver::resolve_with_scene
    let mut dummy_state = CampaignState::with_id_and_room(campaign_id, "ABCD");
    let atk_payload = json!({
        "action": "attack",
        "actor": "Hagnar",
        "actor_id": "player_1",
        "target": "Batedor Goblin #1"
    });
    for _ in 0..10 {
        let _res = RulesResolver::resolve_with_scene(
            &atk_payload,
            &mut dummy_state,
            Some(&mut scene),
            Some(&entity_store),
        );
        if scene.participants[1].is_dead {
            break;
        }
    }

    assert_eq!(scene.participants[1].current_hp, 0);
    assert!(scene.participants[1].is_dead);
    assert!(!scene.has_live_hostiles());

    // 3. Verifica Vitória Tática
    let won = EnvironmentAgent::check_and_apply_combat_victory(&mut scene);
    assert!(won);
    assert!(!scene.in_combat);
    assert!(scene.objectives[0].completed);
    assert_eq!(
        scene.check_completion(),
        Some(SceneResolutionReason::ObjectivesCompleted)
    );
}
