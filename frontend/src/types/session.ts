import type { ContextualScenarioOption } from './game';

export interface SceneObjectiveData {
  id: string;
  description: string;
  completed: boolean;
  completion_trigger?: string;
}

export interface ActionsBudgetData {
  action_used: boolean;
  bonus_action_used: boolean;
  reaction_used: boolean;
  movement_remaining_ft: number;
}

export interface TurnParticipantData {
  id: string;
  name: string;
  is_player: boolean;
  is_hostile?: boolean;
  initiative: number;
  initiative_roll?: number;
  initiative_modifier?: number;
  current_hp?: number;
  max_hp?: number;
  armor_class?: number;
  is_dead?: boolean;
  actions_budget: ActionsBudgetData;
  position?: [number, number];
  token_role?: string;
}

export type TurnParticipant = TurnParticipantData;

export interface SceneObstacle {
  id: string;
  name: string;
  obstacle_type: 'cover' | 'hazard' | 'difficult_terrain' | 'chokepoint' | string;
  x: number;
  y: number;
  radius_ft: number;
  mechanical_effect: string;
}

export type SceneObstacleData = SceneObstacle;

export interface SceneInteractionTurnData {
  turn_number: number;
  actor_id: string;
  actor_name: string;
  via_type: string;
  player_input: string;
  mechanical_intent?: Record<string, unknown>;
  rules_result?: Record<string, unknown>;
  environment_reaction: string;
}

export interface SceneLootItemData {
  id: string;
  name: string;
  item_type: string;
  description: string;
  rarity: string;
  mechanical_bonus?: string;
  location_hint: string;
  is_hidden?: boolean;
  is_claimed: boolean;
  claimed_by?: string;
}

export interface LivingSceneData {
  scene_id: string;
  campaign_id: string;
  title: string;
  location: string;
  chapter: number;
  is_active: boolean;
  in_combat?: boolean;
  objectives: SceneObjectiveData[];
  participants: TurnParticipantData[];
  obstacles?: SceneObstacle[];
  current_participant_index: number;
  round_number: number;
  turn_history: SceneInteractionTurnData[];
  resolution?: string;
  summary?: string;
  opening_narration?: string;
  active_npcs?: string[];
  scenario_options?: ContextualScenarioOption[];
  consumed_scenario_options?: string[];
  scene_loot?: SceneLootItemData[];
  world_prologue?: string;
}

export interface ChronicleEntryData {
  event_id: string;
  timestamp: string;
  chapter: number;
  scene_title: string;
  entry_type: 'LiteraryChapter' | 'LivingSceneSummary' | 'AutonomousEvent' | 'MechanicalAction' | string;
  narrative_prose: string;
  rules_resolutions: Array<Record<string, unknown>>;
  involved_entities: string[];
}

export interface PlayerSlot {
  player_number: number;
  player_id: string;
  player_name: string;
  class_name: string;
  player_code: string;
  is_host: boolean;
}

export interface CampaignSession {
  campaign_id: string;
  room_code: string;
  premise: string;
  literary_style: string;
  host_player_id: string;
  player_ids: string[];
  player_slots?: PlayerSlot[];
  player_number?: number;
  player_code?: string;
  current_act_number: number;
  current_act_title: string;
  is_host: boolean;
  player_id: string;
  player_name: string;
  class_name: string;
  is_offline?: boolean;
  party_size?: number;
  is_imported_backup?: boolean;
  is_single_terminal?: boolean;
  world_prologue?: string;
}
