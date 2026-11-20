use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use axum::Router;

use crate::api::models::*;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "RPG D&D 5e Engine API",
        version = "0.2.0",
        description = "Documentação OpenAPI da API REST do Servidor RPG Engine D&D 5e Multiplayer.\n\nContratos para Campanhas, Cenas Vivas (Grid Tático), Ações de Turno (Três Vias), Autenticação SaaS e Painel Administrativo.",
        contact(
            name = "Equipe RPG Engine",
            url = "https://github.com/rpg-engine"
        ),
        license(
            name = "MIT / Apache-2.0"
        )
    ),
    components(
        schemas(
            HealthResponse,
            ErrorResponse,
            // Campanhas
            AnalyzePremiseRequest,
            AnalyzePremiseResponse,
            CreateCampaignRequest,
            CreateCampaignResponse,
            JoinCampaignRequest,
            JoinCampaignResponse,
            AddLocalPlayerRequest,
            SyncPartyRequest,
            // Cenas Vivas
            StartSceneRequest,
            SceneMoveRequest,
            EndTurnRequest,
            ClaimItemRequest,
            ResolveSceneRequest,
            // Ações de Turno
            TurnActionRequest,
            TurnActionResponse,
            // Chat
            SendChatMessageRequest,
            // SaaS Auth & Admin
            RegisterRequest,
            LoginRequest,
            AuthResponse,
            UserPublicSchema,
            AdminMetricsResponse,
            AdminPremiseRequest,
        )
    ),
    paths(
        health,
        analyze_premise,
        create_campaign,
        join_campaign,
        add_local_player,
        sync_party,
        get_campaign_state,
        get_campaign_chronicle,
        get_campaign_entities,
        start_scene,
        claim_scene_item,
        resolve_scene,
        move_scene_actor,
        end_scene_turn,
        process_turn_action,
        get_campaign_slots,
        send_party_chat,
        get_campaign_audit,
        list_saved_campaigns,
        export_campaign,
        import_campaign,
        list_saved_premises,
        auth_register,
        auth_login,
        auth_me,
        auth_logout,
        admin_login,
        admin_metrics,
        admin_users,
        admin_save_premise,
        admin_preview_premise,
        admin_delete_premise,
    ),
    tags(
        (name = "Health & Sistema", description = "Monitoramento e status operacional da engine"),
        (name = "Campanhas Multiplayer", description = "Criação, lobby, slots e gerenciamento de campanhas"),
        (name = "Cenas Vivas & Combate Tático", description = "Resolução de turnos, movimentação no grid e objetivos de cena"),
        (name = "Ações de Turno (Três Vias)", description = "Execução de ações mecânicas D&D 5e, botões de ação e intenção livre narrada"),
        (name = "Chat da Party", description = "Mensageria e comandos de chat em tempo real"),
        (name = "SaaS Autenticação", description = "Gestão de contas, registro e login com Argon2id"),
        (name = "Painel Administrativo", description = "Métricas da plataforma, usuários e catálogo de premissas com IA"),
    )
)]
pub struct ApiDoc;

// =========================================================================
// Definições de Endpoints OpenAPI
// =========================================================================

#[utoipa::path(
    get,
    path = "/api/health",
    tag = "Health & Sistema",
    responses(
        (status = 200, description = "Status do servidor de RPG", body = HealthResponse)
    )
)]
pub async fn health() {}

#[utoipa::path(
    post,
    path = "/api/campaign/analyze-premise",
    tag = "Campanhas Multiplayer",
    request_body = AnalyzePremiseRequest,
    responses(
        (status = 200, description = "Análise e requisitos extraídos da premissa", body = AnalyzePremiseResponse),
        (status = 400, description = "Erro na análise", body = ErrorResponse)
    )
)]
pub async fn analyze_premise() {}

#[utoipa::path(
    post,
    path = "/api/campaign/create",
    tag = "Campanhas Multiplayer",
    request_body = CreateCampaignRequest,
    responses(
        (status = 200, description = "Campanha criada com sucesso", body = CreateCampaignResponse),
        (status = 400, description = "Erro ao criar campanha", body = ErrorResponse)
    )
)]
pub async fn create_campaign() {}

#[utoipa::path(
    post,
    path = "/api/campaign/join",
    tag = "Campanhas Multiplayer",
    request_body = JoinCampaignRequest,
    responses(
        (status = 200, description = "Entrada na campanha autorizada", body = JoinCampaignResponse),
        (status = 404, description = "Campanha ou sala não encontrada", body = ErrorResponse)
    )
)]
pub async fn join_campaign() {}

#[utoipa::path(
    post,
    path = "/api/campaign/add-local-player",
    tag = "Campanhas Multiplayer",
    request_body = AddLocalPlayerRequest,
    responses(
        (status = 200, description = "Jogador local adicionado à campanha (Modo Um Terminal)", body = JoinCampaignResponse),
        (status = 400, description = "Erro ao adicionar jogador", body = ErrorResponse)
    )
)]
pub async fn add_local_player() {}

#[utoipa::path(
    post,
    path = "/api/campaign/{id}/sync-party",
    tag = "Campanhas Multiplayer",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    request_body = SyncPartyRequest,
    responses(
        (status = 200, description = "Party sincronizada com sucesso", body = serde_json::Value),
        (status = 404, description = "Campanha não encontrada", body = ErrorResponse)
    )
)]
pub async fn sync_party() {}

#[utoipa::path(
    get,
    path = "/api/campaign/{id}/state",
    tag = "Campanhas Multiplayer",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    responses(
        (status = 200, description = "Estado completo da campanha e cena ativa", body = serde_json::Value),
        (status = 404, description = "Campanha não encontrada", body = ErrorResponse)
    )
)]
pub async fn get_campaign_state() {}

#[utoipa::path(
    get,
    path = "/api/campaign/{id}/chronicle",
    tag = "Campanhas Multiplayer",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    responses(
        (status = 200, description = "Livro de Crônica narrativa e histórico", body = serde_json::Value),
        (status = 404, description = "Campanha não encontrada", body = ErrorResponse)
    )
)]
pub async fn get_campaign_chronicle() {}

#[utoipa::path(
    get,
    path = "/api/campaign/{id}/entities",
    tag = "Campanhas Multiplayer",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    responses(
        (status = 200, description = "Entidades, fichas e NPCs da campanha", body = serde_json::Value),
        (status = 404, description = "Campanha não encontrada", body = ErrorResponse)
    )
)]
pub async fn get_campaign_entities() {}

#[utoipa::path(
    post,
    path = "/api/campaign/{id}/scene/start",
    tag = "Cenas Vivas & Combate Tático",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    request_body = StartSceneRequest,
    responses(
        (status = 200, description = "Nova Cena Viva iniciada com grid e participantes", body = serde_json::Value),
        (status = 404, description = "Campanha não encontrada", body = ErrorResponse)
    )
)]
pub async fn start_scene() {}

#[utoipa::path(
    post,
    path = "/api/campaign/{id}/scene/claim_item",
    tag = "Cenas Vivas & Combate Tático",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    request_body = ClaimItemRequest,
    responses(
        (status = 200, description = "Item de saque ou relíquia resgatada com sucesso", body = serde_json::Value),
        (status = 400, description = "Item indisponível ou já saqueado", body = ErrorResponse)
    )
)]
pub async fn claim_scene_item() {}

#[utoipa::path(
    post,
    path = "/api/campaign/{id}/scene/resolve",
    tag = "Cenas Vivas & Combate Tático",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    request_body = ResolveSceneRequest,
    responses(
        (status = 200, description = "Cena concluída e arquivada na Crônica", body = serde_json::Value),
        (status = 404, description = "Campanha não encontrada", body = ErrorResponse)
    )
)]
pub async fn resolve_scene() {}

#[utoipa::path(
    post,
    path = "/api/campaign/{id}/scene/move",
    tag = "Cenas Vivas & Combate Tático",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    request_body = SceneMoveRequest,
    responses(
        (status = 200, description = "Posição do ator atualizada no grid de combate", body = serde_json::Value),
        (status = 400, description = "Movimento inválido ou sem orçamento de deslocamento", body = ErrorResponse)
    )
)]
pub async fn move_scene_actor() {}

#[utoipa::path(
    post,
    path = "/api/campaign/{id}/scene/end-turn",
    tag = "Cenas Vivas & Combate Tático",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    request_body = EndTurnRequest,
    responses(
        (status = 200, description = "Turno finalizado; avança para o próximo participante na ordem de iniciativa", body = serde_json::Value),
        (status = 400, description = "Não é a vez deste ator", body = ErrorResponse)
    )
)]
pub async fn end_scene_turn() {}

#[utoipa::path(
    post,
    path = "/api/campaign/{id}/action/turn",
    tag = "Ações de Turno (Três Vias)",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    request_body = TurnActionRequest,
    responses(
        (status = 200, description = "Ação de turno processada determinística e narrativamente", body = TurnActionResponse),
        (status = 400, description = "Erro no processamento da ação", body = ErrorResponse)
    )
)]
pub async fn process_turn_action() {}

#[utoipa::path(
    get,
    path = "/api/campaign/{id}/slots",
    tag = "Campanhas Multiplayer",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    responses(
        (status = 200, description = "Lista de slots de jogadores presenciais/remotos", body = serde_json::Value),
        (status = 404, description = "Campanha não encontrada", body = ErrorResponse)
    )
)]
pub async fn get_campaign_slots() {}

#[utoipa::path(
    post,
    path = "/api/campaign/{id}/chat",
    tag = "Chat da Party",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    request_body = SendChatMessageRequest,
    responses(
        (status = 200, description = "Mensagem transmitida para o chat da sala", body = serde_json::Value),
        (status = 404, description = "Campanha não encontrada", body = ErrorResponse)
    )
)]
pub async fn send_party_chat() {}

#[utoipa::path(
    get,
    path = "/api/campaign/{id}/audit",
    tag = "Campanhas Multiplayer",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    responses(
        (status = 200, description = "Auditoria completa de rolagens, turnos e decisões do Mestre IA", body = serde_json::Value),
        (status = 404, description = "Campanha não encontrada", body = ErrorResponse)
    )
)]
pub async fn get_campaign_audit() {}

#[utoipa::path(
    get,
    path = "/api/campaigns/saved",
    tag = "Campanhas Multiplayer",
    responses(
        (status = 200, description = "Lista de campanhas salvas em disco", body = serde_json::Value)
    )
)]
pub async fn list_saved_campaigns() {}

#[utoipa::path(
    get,
    path = "/api/campaign/{id}/export",
    tag = "Campanhas Multiplayer",
    params(
        ("id" = String, Path, description = "ID único da campanha")
    ),
    responses(
        (status = 200, description = "Pacote JSON completo para backup e exportação da campanha", body = serde_json::Value),
        (status = 404, description = "Campanha não encontrada", body = ErrorResponse)
    )
)]
pub async fn export_campaign() {}

#[utoipa::path(
    post,
    path = "/api/campaign/import",
    tag = "Campanhas Multiplayer",
    responses(
        (status = 200, description = "Campanha importada e pronta para jogo", body = serde_json::Value),
        (status = 400, description = "Formato de arquivo ou JSON inválido", body = ErrorResponse)
    )
)]
pub async fn import_campaign() {}

#[utoipa::path(
    get,
    path = "/api/premises/saved",
    tag = "Painel Administrativo",
    responses(
        (status = 200, description = "Catálogo de premissas e campanhas prontas", body = serde_json::Value)
    )
)]
pub async fn list_saved_premises() {}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "SaaS Autenticação",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Usuário registrado com sucesso", body = AuthResponse),
        (status = 400, description = "Email já em uso ou dados inválidos", body = ErrorResponse)
    )
)]
pub async fn auth_register() {}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "SaaS Autenticação",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login efetuado com sucesso", body = AuthResponse),
        (status = 401, description = "Credenciais inválidas", body = ErrorResponse)
    )
)]
pub async fn auth_login() {}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    tag = "SaaS Autenticação",
    responses(
        (status = 200, description = "Dados do usuário da sessão atual", body = UserPublicSchema),
        (status = 401, description = "Não autenticado ou sessão expirada", body = ErrorResponse)
    )
)]
pub async fn auth_me() {}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    tag = "SaaS Autenticação",
    responses(
        (status = 200, description = "Logout efetuado com sucesso", body = serde_json::Value)
    )
)]
pub async fn auth_logout() {}

#[utoipa::path(
    post,
    path = "/api/admin/auth/login",
    tag = "Painel Administrativo",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Acesso de administrador confirmado", body = AuthResponse),
        (status = 401, description = "Credenciais de administrador inválidas", body = ErrorResponse)
    )
)]
pub async fn admin_login() {}

#[utoipa::path(
    get,
    path = "/api/admin/metrics",
    tag = "Painel Administrativo",
    responses(
        (status = 200, description = "Métricas do SaaS", body = AdminMetricsResponse),
        (status = 401, description = "Acesso não autorizado", body = ErrorResponse)
    )
)]
pub async fn admin_metrics() {}

#[utoipa::path(
    get,
    path = "/api/admin/users",
    tag = "Painel Administrativo",
    responses(
        (status = 200, description = "Lista de usuários cadastrados", body = Vec<UserPublicSchema>),
        (status = 401, description = "Acesso não autorizado", body = ErrorResponse)
    )
)]
pub async fn admin_users() {}

#[utoipa::path(
    post,
    path = "/api/admin/premises",
    tag = "Painel Administrativo",
    request_body = AdminPremiseRequest,
    responses(
        (status = 200, description = "Premissa salva e refinada com IA", body = serde_json::Value),
        (status = 401, description = "Acesso não autorizado", body = ErrorResponse)
    )
)]
pub async fn admin_save_premise() {}

#[utoipa::path(
    post,
    path = "/api/admin/premises/preview",
    tag = "Painel Administrativo",
    request_body = AdminPremiseRequest,
    responses(
        (status = 200, description = "Prévia de refinamento e análise de monstros", body = serde_json::Value),
        (status = 401, description = "Acesso não autorizado", body = ErrorResponse)
    )
)]
pub async fn admin_preview_premise() {}

#[utoipa::path(
    delete,
    path = "/api/admin/premises/{id}",
    tag = "Painel Administrativo",
    params(
        ("id" = String, Path, description = "ID único da premissa a ser excluída")
    ),
    responses(
        (status = 200, description = "Premissa removida", body = serde_json::Value),
        (status = 401, description = "Acesso não autorizado", body = ErrorResponse)
    )
)]
pub async fn admin_delete_premise() {}

/// Constrói e retorna a instância configurada do SwaggerUi para montagem no router do Axum
pub fn swagger_router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
        .into()
}
