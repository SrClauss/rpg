use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneObjective {
    pub id: String,
    pub description: String,
    pub completed: bool,
    pub completion_trigger: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum SceneResolutionReason {
    ObjectivesCompleted,
    PartyDeparted,
    PartyDefeated,
    ManualResolution,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionsBudget {
    pub action_used: bool,
    pub bonus_action_used: bool,
    pub reaction_used: bool,
    pub movement_remaining_ft: u32,
}

impl Default for ActionsBudget {
    fn default() -> Self {
        Self {
            action_used: false,
            bonus_action_used: false,
            reaction_used: false,
            movement_remaining_ft: 30,
        }
    }
}

impl ActionsBudget {
    pub fn reset_turn(&mut self) {
        self.action_used = false;
        self.bonus_action_used = false;
        self.reaction_used = false;
        self.movement_remaining_ft = 30;
    }
}

fn default_is_hostile() -> bool {
    false
}

fn default_hp() -> i32 {
    0
}

fn default_ac() -> i32 {
    13
}

fn default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TurnParticipant {
    pub id: String,
    pub name: String,
    pub is_player: bool,
    #[serde(default = "default_is_hostile")]
    pub is_hostile: bool,
    pub initiative: i32,
    #[serde(default)]
    pub initiative_roll: i32,
    #[serde(default)]
    pub initiative_modifier: i32,
    #[serde(default = "default_hp")]
    pub current_hp: i32,
    #[serde(default = "default_hp")]
    pub max_hp: i32,
    #[serde(default = "default_ac")]
    pub armor_class: i32,
    #[serde(default)]
    pub is_dead: bool,
    pub actions_budget: ActionsBudget,
    pub position: Option<(i32, i32)>, // Coordenadas em pés no campo de batalha (ex: X: 0..60, Y: 0..40)
    #[serde(default)]
    pub token_role: Option<String>, // ex: "Fighter", "Ranger", "Inimigo", "Chefe", "Aliado"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneObstacle {
    pub id: String,
    pub name: String,
    pub obstacle_type: String, // "cover" | "hazard" | "difficult_terrain" | "chokepoint"
    pub x: i32, // 0..60 ft
    pub y: i32, // 0..40 ft
    pub radius_ft: i32, // ex: 4 ou 5
    pub mechanical_effect: String, // ex: "+2 CA (Cobertura Parcial)", "Armadilha: 1d6 perfurante"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneInteractionTurn {
    pub turn_number: u32,
    pub actor_id: String,
    pub actor_name: String,
    pub via_type: String, // "via1", "via2", "via3"
    pub player_input: String,
    pub mechanical_intent: Option<serde_json::Value>,
    pub rules_result: Option<serde_json::Value>,
    pub environment_reaction: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneLootItem {
    pub id: String,
    pub name: String,
    pub item_type: String, // "Arma Mágica", "Armadura", "Poção", "Relíquia", "Pergaminho", "Minério"
    pub description: String,
    pub rarity: String, // "Comum", "Incomum", "Raro", "Muito Raro", "Lendário"
    pub mechanical_bonus: Option<String>,
    pub location_hint: String,
    pub is_claimed: bool,
    pub claimed_by: Option<String>,
    #[serde(default = "default_is_hidden")]
    pub is_hidden: bool,
}

fn default_is_hidden() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LivingScene {
    pub scene_id: String,
    pub campaign_id: String,
    pub title: String,
    pub location: String,
    pub chapter: u32,
    pub is_active: bool,
    pub objectives: Vec<SceneObjective>,
    pub participants: Vec<TurnParticipant>,
    pub current_participant_index: usize,
    pub round_number: u32,
    pub turn_history: Vec<SceneInteractionTurn>,
    pub resolution: Option<SceneResolutionReason>,
    pub summary: Option<String>,
    #[serde(default)]
    pub opening_narration: Option<String>,
    #[serde(default)]
    pub active_npcs: Vec<String>,
    #[serde(default)]
    pub scenario_options: Vec<serde_json::Value>,
    #[serde(default)]
    pub scene_loot: Vec<SceneLootItem>,
    #[serde(default)]
    pub consumed_scenario_options: Vec<String>,
    #[serde(default)]
    pub world_prologue: Option<String>,
    #[serde(default = "default_true")]
    pub in_combat: bool,
    #[serde(default)]
    pub obstacles: Vec<SceneObstacle>,
}

impl LivingScene {
    pub fn generate_default_obstacles(location: &str, title: &str) -> Vec<SceneObstacle> {
        let context = format!("{} {}", location, title).to_lowercase();
        let (cover1_name, cover2_name, hazard_name, terrain_name, choke_name) =
            if context.contains("caverna") || context.contains("cova") || context.contains("mina") || context.contains("subterr") {
                (
                    "Estalagmite Maciça Norte",
                    "Rocha Fendida Sul",
                    "Fosso de Estacas Goblins",
                    "Escombros Úmidos e Ossadas",
                    "Garganta Rochosa Estreita",
                )
            } else if context.contains("floresta") || context.contains("bosque") || context.contains("mata") {
                (
                    "Tronco Ancestral Caído",
                    "Pedra Musgosa Elevada",
                    "Armadilha de Laço e Espinhos",
                    "Raízes Retorcidas e Lama",
                    "Passagem entre Barrancos",
                )
            } else {
                (
                    "Pilar de Pedra Norte",
                    "Barricada de Rocha Sul",
                    "Armadilha de Estacas Oculta",
                    "Escombros e Terreno Irregular",
                    "Garganta Tática Lateral",
                )
            };

        vec![
            SceneObstacle {
                id: "obs_cover_1".to_string(),
                name: cover1_name.to_string(),
                obstacle_type: "cover".to_string(),
                x: 25,
                y: 12,
                radius_ft: 4,
                mechanical_effect: "+2 CA (Cobertura Parcial)".to_string(),
            },
            SceneObstacle {
                id: "obs_cover_2".to_string(),
                name: cover2_name.to_string(),
                obstacle_type: "cover".to_string(),
                x: 30,
                y: 28,
                radius_ft: 4,
                mechanical_effect: "+2 CA (Cobertura Parcial)".to_string(),
            },
            SceneObstacle {
                id: "obs_hazard_1".to_string(),
                name: hazard_name.to_string(),
                obstacle_type: "hazard".to_string(),
                x: 35,
                y: 20,
                radius_ft: 4,
                mechanical_effect: "Armadilha: 1d6 perfurante (CD 12 DES)".to_string(),
            },
            SceneObstacle {
                id: "obs_terrain_1".to_string(),
                name: terrain_name.to_string(),
                obstacle_type: "difficult_terrain".to_string(),
                x: 22,
                y: 22,
                radius_ft: 5,
                mechanical_effect: "Terreno Difícil (Custo dobrado de movimento)".to_string(),
            },
            SceneObstacle {
                id: "obs_choke_1".to_string(),
                name: choke_name.to_string(),
                obstacle_type: "chokepoint".to_string(),
                x: 15,
                y: 5,
                radius_ft: 4,
                mechanical_effect: "+5 CA (Cobertura Três-Quartos)".to_string(),
            },
        ]
    }

    pub fn new(
        campaign_id: &str,
        title: &str,
        location: &str,
        chapter: u32,
        objectives: Vec<SceneObjective>,
        mut participants: Vec<TurnParticipant>,
    ) -> Self {
        // Ordena participantes por iniciativa decrescente
        participants.sort_by(|a, b| b.initiative.cmp(&a.initiative));
        let first_player_idx = participants
            .iter()
            .position(|p| p.is_player && !p.is_dead)
            .unwrap_or(0);
        let obstacles = Self::generate_default_obstacles(location, title);

        Self {
            scene_id: uuid::Uuid::new_v4().to_string(),
            campaign_id: campaign_id.to_string(),
            title: title.to_string(),
            location: location.to_string(),
            chapter,
            is_active: true,
            objectives,
            participants,
            current_participant_index: first_player_idx,
            round_number: 1,
            turn_history: Vec::new(),
            resolution: None,
            summary: None,
            opening_narration: None,
            active_npcs: Vec::new(),
            scenario_options: Vec::new(),
            scene_loot: Vec::new(),
            consumed_scenario_options: Vec::new(),
            world_prologue: None,
            in_combat: true,
            obstacles,
        }
    }

    pub fn has_live_hostiles(&self) -> bool {
        self.participants
            .iter()
            .any(|p| !p.is_player && p.is_hostile && !p.is_dead && p.current_hp > 0)
    }

    pub fn current_participant(&self) -> Option<&TurnParticipant> {
        self.participants.get(self.current_participant_index)
    }

    pub fn current_participant_mut(&mut self) -> Option<&mut TurnParticipant> {
        self.participants.get_mut(self.current_participant_index)
    }

    pub fn advance_turn(&mut self) {
        if self.participants.is_empty() {
            return;
        }

        let len = self.participants.len();
        for _ in 0..len {
            self.current_participant_index += 1;
            if self.current_participant_index >= len {
                self.current_participant_index = 0;
                self.round_number += 1;
                for p in &mut self.participants {
                    p.actions_budget.reset_turn();
                }
            }
            if let Some(p) = self.participants.get(self.current_participant_index) {
                if !p.is_dead {
                    break;
                }
            }
        }
    }

    pub fn mark_objective_completed(&mut self, objective_id: &str) {
        for obj in &mut self.objectives {
            if obj.id == objective_id {
                obj.completed = true;
            }
        }
    }

    pub fn check_completion(&self) -> Option<SceneResolutionReason> {
        if !self.is_active {
            return self.resolution.clone();
        }

        // Enquanto houver inimigos hostis vivos em combate, a cena não pode encerrar por objetivos automáticos
        if self.in_combat && self.has_live_hostiles() {
            return None;
        }

        let all_completed = !self.objectives.is_empty() && self.objectives.iter().all(|o| o.completed);
        if all_completed {
            return Some(SceneResolutionReason::ObjectivesCompleted);
        }

        None
    }

    pub fn record_turn(&mut self, turn: SceneInteractionTurn) {
        self.turn_history.push(turn);
    }

    pub fn close_scene(&mut self, reason: SceneResolutionReason, summary: &str) {
        self.is_active = false;
        self.in_combat = false;
        self.resolution = Some(reason);
        self.summary = Some(summary.to_string());
    }
}
