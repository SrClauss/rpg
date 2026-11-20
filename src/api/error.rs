use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Erros padronizados da API HTTP do rpg-engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiError {
    NotFound,
    BadRequest(String),
    Unauthorized,
    Forbidden(String),
    Conflict(String),
    Internal(String),
}

impl ApiError {
    pub fn not_found() -> Self {
        ApiError::NotFound
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        ApiError::BadRequest(msg.into())
    }

    pub fn unauthorized() -> Self {
        ApiError::Unauthorized
    }

    pub fn forbidden(msg: impl Into<String>) -> Self {
        ApiError::Forbidden(msg.into())
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        ApiError::Conflict(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        ApiError::Internal(msg.into())
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            ApiError::NotFound => "NOT_FOUND",
            ApiError::BadRequest(_) => "BAD_REQUEST",
            ApiError::Unauthorized => "UNAUTHORIZED",
            ApiError::Forbidden(_) => "FORBIDDEN",
            ApiError::Conflict(_) => "CONFLICT",
            ApiError::Internal(_) => "INTERNAL_SERVER_ERROR",
        }
    }

    pub fn message(&self) -> String {
        match self {
            ApiError::NotFound => "Recurso não encontrado".to_string(),
            ApiError::BadRequest(msg) => msg.clone(),
            ApiError::Unauthorized => "Não autorizado".to_string(),
            ApiError::Forbidden(msg) => msg.clone(),
            ApiError::Conflict(msg) => msg.clone(),
            ApiError::Internal(msg) => msg.clone(),
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(json!({
            "status": "error",
            "code": self.error_code(),
            "message": self.message(),
        }));
        (status, body).into_response()
    }
}

impl From<String> for ApiError {
    fn from(err: String) -> Self {
        ApiError::Internal(err)
    }
}

impl From<&str> for ApiError {
    fn from(err: &str) -> Self {
        ApiError::Internal(err.to_string())
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes_and_codes() {
        assert_eq!(ApiError::NotFound.status_code(), StatusCode::NOT_FOUND);
        assert_eq!(ApiError::NotFound.error_code(), "NOT_FOUND");

        let bad_req = ApiError::bad_request("Parâmetro inválido");
        assert_eq!(bad_req.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(bad_req.error_code(), "BAD_REQUEST");
        assert_eq!(bad_req.message(), "Parâmetro inválido");

        assert_eq!(ApiError::Unauthorized.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(ApiError::Unauthorized.error_code(), "UNAUTHORIZED");

        let forbidden = ApiError::forbidden("Acesso negado");
        assert_eq!(forbidden.status_code(), StatusCode::FORBIDDEN);
        assert_eq!(forbidden.error_code(), "FORBIDDEN");

        let conflict = ApiError::conflict("Conflito existente");
        assert_eq!(conflict.status_code(), StatusCode::CONFLICT);
        assert_eq!(conflict.error_code(), "CONFLICT");

        let internal = ApiError::internal("Erro no servidor");
        assert_eq!(internal.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(internal.error_code(), "INTERNAL_SERVER_ERROR");
    }

    #[test]
    fn test_display_and_from() {
        let err: ApiError = "Falha genérica".into();
        assert_eq!(err.message(), "Falha genérica");
        assert_eq!(format!("{}", err), "Falha genérica");
    }

    #[test]
    fn test_into_response() {
        let err = ApiError::bad_request("Dados incorretos");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
