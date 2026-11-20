use rpg_engine::api::openapi::ApiDoc;
use utoipa::OpenApi;

#[test]
fn test_openapi_spec_generation() {
    let doc = ApiDoc::openapi();
    assert_eq!(doc.info.title, "RPG D&D 5e Engine API");
    assert_eq!(doc.info.version, "0.2.0");

    let paths = &doc.paths;
    // Verifica que caminhos essenciais estão presentes
    assert!(paths.paths.contains_key("/api/health"), "Falta /api/health");
    assert!(paths.paths.contains_key("/api/campaign/create"), "Falta /api/campaign/create");
    assert!(paths.paths.contains_key("/api/campaign/{id}/action/turn"), "Falta rota action/turn");
    assert!(paths.paths.contains_key("/api/campaign/{id}/scene/start"), "Falta rota scene/start");
    assert!(paths.paths.contains_key("/api/auth/register"), "Falta /api/auth/register");
    assert!(paths.paths.contains_key("/api/auth/login"), "Falta /api/auth/login");
    assert!(paths.paths.contains_key("/api/admin/metrics"), "Falta /api/admin/metrics");
    assert!(paths.paths.contains_key("/api/admin/premises"), "Falta /api/admin/premises");

    // Verifica que componentes / schemas essenciais estão registrados
    let components = doc.components.expect("ApiDoc deve conter components");
    assert!(components.schemas.contains_key("HealthResponse"));
    assert!(components.schemas.contains_key("CreateCampaignRequest"));
    assert!(components.schemas.contains_key("CreateCampaignResponse"));
    assert!(components.schemas.contains_key("TurnActionRequest"));
    assert!(components.schemas.contains_key("TurnActionResponse"));
    assert!(components.schemas.contains_key("StartSceneRequest"));
    assert!(components.schemas.contains_key("RegisterRequest"));
    assert!(components.schemas.contains_key("LoginRequest"));
    assert!(components.schemas.contains_key("AuthResponse"));
    assert!(components.schemas.contains_key("AdminMetricsResponse"));
}

#[test]
fn test_swagger_router_instantiation() {
    let _router: axum::Router = rpg_engine::api::swagger_router();
}
