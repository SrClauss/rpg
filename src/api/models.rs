use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

// =========================================================================
// Health & Sistema
// =========================================================================

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct HealthResponse {
    #[schema(example = "online")]
    pub status: String,
    #[schema(example = "rpg-engine-rust")]
    pub engine: String,
    #[schema(example = "D&D 5e Deterministic")]
    pub rules: String,
    #[schema(example = "WebSockets Active")]
    pub multiplayer: String,
    #[schema(example = "0.2.0")]
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ApiResponse<T> {
    #[schema(example = "ok")]
    pub status: String,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = "error")]
    pub status: String,
    #[schema(example = "Descrição do erro ocorrido")]
    pub message: String,
}

// =========================================================================
// Campanhas
// =========================================================================

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AnalyzePremiseRequest {
    #[schema(example = "Um culto sombrio despertou uma besta ancestral sob as catacumbas da cidade.")]
    pub premise: String,
    #[schema(example = "Fantasia Sombria")]
    pub literary_style: String,
    #[schema(example = "balanced")]
    pub ai_creative_freedom: String,
    #[schema(example = 10)]
    pub dramatic_focus_slider: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AnalyzePremiseResponse {
    #[schema(example = "ok")]
    pub status: String,
    pub analysis: Value,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CreateCampaignRequest {
    #[schema(example = "Nas profundezas do pântano de cinzas, o lich sussurra...")]
    pub premise: String,
    #[schema(example = "Fantasia Sombria")]
    pub literary_style: String,
    #[schema(example = "Theron de Valen")]
    pub host_player_name: String,
    #[schema(example = "Paladin")]
    pub host_class: String,
    #[schema(default)]
    pub custom_monsters: Vec<Value>,
    #[schema(example = "balanced")]
    pub ai_creative_freedom: String,
    #[schema(example = "d20")]
    pub story_focus: String,
    #[schema(example = 12)]
    pub dramatic_focus_slider: Option<u32>,
    pub party: Option<Vec<Value>>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CreateCampaignResponse {
    #[schema(example = "ok")]
    pub status: String,
    #[schema(example = "b7a2d480-1234-4567-89ab-cdef01234567")]
    pub campaign_id: String,
    #[schema(example = "XK9P")]
    pub room_code: String,
    #[schema(example = "player_host_a1b2c3")]
    pub host_player_id: String,
    #[schema(example = "H4TK")]
    pub host_player_code: String,
    pub manifest: Value,
    pub initial_scene: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct JoinCampaignRequest {
    pub campaign_id: Option<String>,
    #[schema(example = "XK9P")]
    pub room_code: String,
    #[schema(example = "Aria das Sombras")]
    pub player_name: String,
    #[schema(example = "Rogue")]
    pub class_name: String,
    #[schema(example = "R2D2")]
    pub player_code: Option<String>,
    #[schema(example = false)]
    pub is_single_terminal: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct JoinCampaignResponse {
    #[schema(example = "ok")]
    pub status: String,
    #[schema(example = "b7a2d480-1234-4567-89ab-cdef01234567")]
    pub campaign_id: String,
    #[schema(example = "player_2_9f8e7d")]
    pub player_id: String,
    #[schema(example = "XK9P")]
    pub room_code: String,
    #[schema(example = "R2D2")]
    pub player_code: String,
    #[schema(example = 2)]
    pub player_number: usize,
    pub current_scene: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AddLocalPlayerRequest {
    #[schema(example = "b7a2d480-1234-4567-89ab-cdef01234567")]
    pub campaign_id: String,
    #[schema(example = "Kaelen")]
    pub player_name: String,
    #[schema(example = "Wizard")]
    pub class_name: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SyncPartyRequest {
    pub party: Vec<Value>,
}

// =========================================================================
// Cenas Vivas & Combate Tático
// =========================================================================

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct StartSceneRequest {
    #[schema(example = "A Emboscada na Ponte Caída")]
    pub title: Option<String>,
    #[schema(example = "Desfiladeiro dos Lamentos")]
    pub location: Option<String>,
    #[schema(example = json!(["Sobreviver à emboscada dos goblins", "Proteger a carruagem mercante"]))]
    pub objectives: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SceneMoveRequest {
    #[schema(example = "player_host_a1b2c3")]
    pub actor_id: String,
    #[schema(example = 15)]
    pub x: i32,
    #[schema(example = 20)]
    pub y: i32,
    pub player_id: Option<String>,
    pub player_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct EndTurnRequest {
    pub actor_id: Option<String>,
    pub player_id: Option<String>,
    pub player_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ClaimItemRequest {
    #[schema(example = "loot_espada_longa_1")]
    pub item_id: String,
    #[schema(example = "Theron de Valen")]
    pub player_name: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ResolveSceneRequest {
    #[schema(example = "manual")]
    pub reason: Option<String>,
    pub summary: Option<String>,
}

// =========================================================================
// Ações de Turno (Três Vias)
// =========================================================================

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct TurnActionRequest {
    #[schema(example = "via1")]
    pub via_type: String,
    #[schema(example = "atk_espada_longa")]
    pub action_id: Option<String>,
    #[schema(example = "Avanço com meu escudo erguido e desfiro um golpe rápido na criatura.")]
    pub player_text: Option<String>,
    #[schema(example = "Theron")]
    pub character_name: String,
    #[schema(example = "Paladin")]
    pub class_name: String,
    pub actor_id: Option<String>,
    pub campaign_id: Option<String>,
    pub player_id: Option<String>,
    pub player_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct TurnActionResponse {
    #[schema(example = "processed")]
    pub status: String,
    #[schema(example = true)]
    pub requires_mechanic: bool,
    #[schema(example = "Ataque corpo-a-corpo")]
    pub detected_intent: String,
    pub roll_result: Option<Value>,
    #[schema(example = "Sua lâmina resvala na carapaça da criatura, arrancando faíscas.")]
    pub narrative_feedback: String,
}

// =========================================================================
// Chat & Mensagens
// =========================================================================

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SendChatMessageRequest {
    #[schema(example = "player_host_a1b2c3")]
    pub sender_id: String,
    #[schema(example = "Theron")]
    pub sender_name: String,
    #[schema(example = "Cuidado com os flancos!")]
    pub text: String,
    #[schema(example = false)]
    pub is_action: bool,
}

// =========================================================================
// Autenticação SaaS (Argon2id)
// =========================================================================

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "mestre@exemplo.com")]
    pub email: String,
    #[schema(example = "Mestre Supremo")]
    pub name: String,
    #[schema(example = "SenhaSegura123!")]
    pub password: String,
    #[schema(example = "host")]
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "mestre@exemplo.com")]
    pub email: String,
    #[schema(example = "SenhaSegura123!")]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AuthResponse {
    #[schema(example = "success")]
    pub status: String,
    #[schema(example = "sess_a8b9c0d1e2f3...")]
    pub token: String,
    pub user: UserPublicSchema,
    #[schema(example = "Operação realizada com sucesso.")]
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct UserPublicSchema {
    #[schema(example = "usr_1a2b3c")]
    pub id: String,
    #[schema(example = "mestre@exemplo.com")]
    pub email: String,
    #[schema(example = "Mestre Supremo")]
    pub name: String,
    #[schema(example = "host")]
    pub role: String,
    #[schema(example = "host_pro")]
    pub plan: String,
    #[schema(example = "2026-11-20T18:00:00Z")]
    pub created_at: String,
}

// =========================================================================
// Painel Administrativo SaaS
// =========================================================================

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AdminMetricsResponse {
    #[schema(example = "ok")]
    pub status: String,
    #[schema(example = 42)]
    pub total_users: usize,
    #[schema(example = 5)]
    pub total_admins: usize,
    #[schema(example = 18)]
    pub total_hosts: usize,
    #[schema(example = 19)]
    pub total_players: usize,
    #[schema(example = 12)]
    pub host_pro_subscribers: usize,
    #[schema(example = 10)]
    pub active_campaigns_in_memory: usize,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct AdminPremiseRequest {
    pub id: Option<String>,
    #[schema(example = "A Maldição de Sangue e Cinzas")]
    pub title: String,
    #[schema(example = "Uma névoa carmesim desce sobre o ducado nas noites sem lua...")]
    pub premise: String,
    #[schema(example = "Fantasia Sombria")]
    pub literary_style: Option<String>,
    #[schema(example = true)]
    pub auto_refine_ai: Option<bool>,
    pub custom_monsters: Option<Vec<Value>>,
    #[schema(example = json!(["vampiros", "horror", "investigacao"]))]
    pub tags: Option<Vec<String>>,
}
