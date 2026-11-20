use axum::{
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::post,
    Router,
};
use serde_json::{json, Value};

use crate::api::models::{LoginRequest, RegisterRequest};
use crate::api::state::{extract_bearer_token, ServerState};

pub fn auth_router() -> Router<ServerState> {
    Router::new()
        .route("/api/auth/register", post(auth_register))
        .route("/api/auth/login", post(auth_login))
        .route("/api/auth/me", axum::routing::get(auth_me))
        .route("/api/auth/logout", post(auth_logout))
        .route("/api/admin/auth/login", post(admin_auth_login))
}

pub async fn auth_register(
    axum::extract::State(state): axum::extract::State<ServerState>,
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

pub async fn auth_login(
    axum::extract::State(state): axum::extract::State<ServerState>,
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

pub async fn admin_auth_login(
    axum::extract::State(state): axum::extract::State<ServerState>,
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

pub async fn auth_me(
    headers: HeaderMap,
    axum::extract::State(state): axum::extract::State<ServerState>,
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

pub async fn auth_logout(
    headers: HeaderMap,
    axum::extract::State(state): axum::extract::State<ServerState>,
) -> (StatusCode, Json<Value>) {
    if let Some(token) = extract_bearer_token(&headers) {
        state.user_store.logout(&token).await;
    }
    (StatusCode::OK, Json(json!({ "status": "ok", "message": "Logout realizado com sucesso." })))
}
