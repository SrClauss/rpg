use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{delete, get, post},
    Router,
};
use serde_json::{json, Value};

use crate::api::state::{extract_bearer_token, ServerState};
use crate::auth::UserPublic;
use crate::llm::skeleton_agent::SkeletonAgent;

pub fn admin_router() -> Router<ServerState> {
    Router::new()
        .route("/api/admin/metrics", get(admin_get_metrics))
        .route("/api/admin/users", get(admin_list_users))
        .route("/api/admin/premises", post(admin_save_premise))
        .route("/api/admin/premises/preview", post(admin_preview_premise))
        .route("/api/admin/premises/{id}", delete(admin_delete_premise))
}

pub async fn require_admin(
    headers: &HeaderMap,
    state: &ServerState,
) -> Result<UserPublic, (StatusCode, Json<Value>)> {
    let token = match extract_bearer_token(headers) {
        Some(t) => t,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "status": "error", "message": "Autenticação requerida." })),
            ));
        }
    };

    match state.user_store.get_user_by_token(&token).await {
        Some(user) => {
            if user.role == "admin" {
                Ok(user)
            } else {
                Err((
                    StatusCode::FORBIDDEN,
                    Json(json!({ "status": "error", "message": "Acesso restrito ao Painel Administrativo SaaS." })),
                ))
            }
        }
        None => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "status": "error", "message": "Sessão inválida." })),
        )),
    }
}

pub async fn admin_get_metrics(
    headers: HeaderMap,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    if let Err(res) = require_admin(&headers, &state).await {
        return res;
    }

    let users = state.user_store.list_users().await;
    let total_users = users.len();
    let total_hosts = users.iter().filter(|u| u.role == "host" || u.role == "admin").count();
    let total_players = users.iter().filter(|u| u.role == "player").count();

    let rooms = state.rooms.read().await;
    let active_rooms = rooms.len();
    drop(rooms);

    let mut saved_campaigns_count = 0;
    if let Ok(entries) = std::fs::read_dir("data/campaigns") {
        saved_campaigns_count = entries.flatten().count();
    }

    let mut saved_premises_count = 0;
    if let Ok(entries) = std::fs::read_dir("data/premises") {
        saved_premises_count = entries.flatten().count();
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "metrics": {
                "total_users": total_users,
                "total_hosts": total_hosts,
                "total_players": total_players,
                "active_rooms": active_rooms,
                "saved_campaigns": saved_campaigns_count,
                "saved_premises": saved_premises_count,
                "saas_tier": "Alpha Multi-Tenant"
            }
        })),
    )
}

pub async fn admin_list_users(
    headers: HeaderMap,
    State(state): State<ServerState>,
) -> (StatusCode, Json<Value>) {
    if let Err(res) = require_admin(&headers, &state).await {
        return res;
    }

    let users = state.user_store.list_users().await;
    (StatusCode::OK, Json(json!({ "status": "ok", "users": users })))
}

pub async fn admin_save_premise(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    if let Err(res) = require_admin(&headers, &state).await {
        return res;
    }

    let dir = "data/premises";
    if let Err(e) = std::fs::create_dir_all(dir) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "status": "error", "message": e.to_string() })),
        );
    }

    let id = payload["id"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let title = payload["title"]
        .as_str()
        .unwrap_or("Premissa Sem Título")
        .to_string();

    let raw_premise = payload["premise"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let literary_style = payload["literary_style"]
        .as_str()
        .unwrap_or("Fantasia Sombria")
        .to_string();

    // Processo Revisor Editorial de IA (Duas Vias Gemini + DeepSeek)
    let should_refine = payload["auto_refine_ai"].as_bool().unwrap_or(true);
    let final_premise = if should_refine && !raw_premise.trim().is_empty() {
        println!(">>> [ADMIN_AI] Refinando premissa '{}' via processo editorial de IA...", title);
        match state.orchestrator.refine_premise(&title, &raw_premise, &literary_style).await {
            Ok(refined) => {
                println!(">>> [ADMIN_AI] Premissa refinada com sucesso!");
                refined
            }
            Err(e) => {
                println!(">>> [ADMIN_AI] Fallback de refinamento: {}. Usando premissa original.", e);
                raw_premise.clone()
            }
        }
    } else {
        raw_premise.clone()
    };

    let mut full_payload = payload.clone();
    full_payload["id"] = json!(id);
    full_payload["premise"] = json!(final_premise);
    full_payload["raw_premise_submitted"] = json!(raw_premise);
    full_payload["literary_style"] = json!(literary_style);
    full_payload["ai_refined"] = json!(should_refine);

    // Se o payload já traz entidades da prévia, usa-as diretamente; caso contrário, roda a análise
    let analysis_opt = if full_payload.get("custom_monsters").is_none() || full_payload["custom_monsters"].as_array().map_or(true, |a| a.is_empty()) {
        let combined_for_analysis = format!("{}\n\nREGISTROS E ENTIDADES HOMEBREW ORIGINAIS:\n{}", final_premise, raw_premise);
        let analysis = match state.skeleton_agent.analyze_premise_requirements(&combined_for_analysis, &literary_style, 10).await {
            Ok(v) => v,
            Err(_) => SkeletonAgent::deterministic_premise_analysis(&combined_for_analysis, 10),
        };
        if let Some(monsters) = analysis.get("custom_monsters") {
            full_payload["custom_monsters"] = monsters.clone();
        }
        if let Some(overrides) = analysis.get("subclass_overrides") {
            full_payload["subclass_overrides"] = overrides.clone();
        }
        if let Some(spells) = analysis.get("custom_spells") {
            full_payload["custom_spells"] = spells.clone();
        }
        Some(analysis)
    } else {
        None
    };

    if full_payload.get("created_at").is_none() {
        full_payload["created_at"] = json!(chrono::Utc::now().to_rfc3339());
    }

    // Normalização e extração de Tags no Admin
    let mut raw_tags: Vec<String> = full_payload
        .get("tags")
        .and_then(|t| t.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();

    if raw_tags.is_empty() {
        if let Some(ref analysis) = analysis_opt {
            if let Some(analysis_tags) = analysis.get("tags").and_then(|t| t.as_array()) {
                for at in analysis_tags {
                    if let Some(s) = at.as_str() {
                        let trimmed = s.trim().to_string();
                        if !trimmed.is_empty() && !raw_tags.contains(&trimmed) {
                            raw_tags.push(trimmed);
                        }
                    }
                }
            }
        }
    }

    let mut tags = Vec::new();
    for t in raw_tags {
        for word in t.split_whitespace() {
            let clean = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
            if !clean.is_empty() && !tags.contains(&clean) {
                tags.push(clean);
            }
        }
    }

    if tags.is_empty() {
        let title_lower = title.to_lowercase();
        let style_lower = literary_style.to_lowercase();
        let premise_lower = raw_premise.to_lowercase();
        let full_text = format!("{} {} {}", title_lower, style_lower, premise_lower);
        tags = SkeletonAgent::extract_deterministic_tags(&full_text);
    }
    let mut unique_tags = Vec::new();
    for tag in tags {
        if !unique_tags.contains(&tag) {
            unique_tags.push(tag);
        }
    }
    full_payload["tags"] = json!(unique_tags);

    let file_path = format!("{}/{}.json", dir, id);
    match serde_json::to_string_pretty(&full_payload) {
        Ok(json_str) => {
            if let Err(e) = std::fs::write(&file_path, json_str) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "status": "error", "message": e.to_string() })),
                );
            }
            (
                StatusCode::OK,
                Json(json!({
                    "status": "success",
                    "id": id,
                    "title": title,
                    "premise": final_premise,
                    "ai_refined": should_refine,
                    "message": "Premissa revisada por IA e gravada com sucesso no catálogo oficial."
                })),
            )
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "status": "error", "message": e.to_string() })),
        ),
    }
}

pub async fn admin_preview_premise(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    if let Err(res) = require_admin(&headers, &state).await {
        return res;
    }

    let title = payload["title"]
        .as_str()
        .unwrap_or("Premissa Sem Título")
        .to_string();

    let raw_premise = payload["premise"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let literary_style = payload["literary_style"]
        .as_str()
        .unwrap_or("Fantasia Sombria")
        .to_string();

    let slider = payload["dramatic_focus_slider"]
        .as_u64()
        .unwrap_or(10) as u32;

    let should_refine = payload["auto_refine_ai"].as_bool().unwrap_or(true);

    let refined_premise = if should_refine && !raw_premise.trim().is_empty() {
        println!(">>> [ADMIN_AI] Gerando prévia de refinamento para '{}'...", title);
        match state.orchestrator.refine_premise(&title, &raw_premise, &literary_style).await {
            Ok(refined) => refined,
            Err(e) => {
                println!(">>> [ADMIN_AI] Fallback de prévia: {}. Mantendo original.", e);
                raw_premise.clone()
            }
        }
    } else {
        raw_premise.clone()
    };

    let combined_for_analysis = format!("{}\n\nREGISTROS E ENTIDADES HOMEBREW ORIGINAIS:\n{}", refined_premise, raw_premise);
    let analysis = match state.skeleton_agent.analyze_premise_requirements(&combined_for_analysis, &literary_style, slider).await {
        Ok(v) => v,
        Err(_) => SkeletonAgent::deterministic_premise_analysis(&combined_for_analysis, slider),
    };

    (
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "title": title,
            "raw_premise": raw_premise,
            "refined_premise": refined_premise,
            "literary_style": literary_style,
            "dramatic_focus_slider": slider,
            "ai_refined": should_refine,
            "analysis": analysis
        })),
    )
}

pub async fn admin_delete_premise(
    headers: HeaderMap,
    State(state): State<ServerState>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    if let Err(res) = require_admin(&headers, &state).await {
        return res;
    }

    let dir = "data/premises";
    let file_path = format!("{}/{}.json", dir, id);
    if std::path::Path::new(&file_path).exists() {
        if let Err(e) = std::fs::remove_file(&file_path) {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": e.to_string() })),
            );
        }
        (
            StatusCode::OK,
            Json(json!({ "status": "success", "message": "História pré-construída excluída do catálogo oficial." })),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "status": "error", "message": "História não encontrada." })),
        )
    }
}
