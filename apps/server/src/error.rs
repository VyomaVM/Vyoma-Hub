use axum::{
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OciErrorItem {
    pub code: String,
    pub message: String,
    pub detail: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OciErrorEnvelope {
    pub errors: Vec<OciErrorItem>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NAME_UNKNOWN", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "UNSUPPORTED", msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "DENIED", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "MANIFEST_INVALID", msg.clone()),
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "A database error occurred".to_string(),
                )
            }
            AppError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "An internal server error occurred".to_string(),
                )
            }
        };

        // Standard OCI error envelope conforming to OCI Distribution Spec
        let oci_body = OciErrorEnvelope {
            errors: vec![OciErrorItem {
                code: code.to_string(),
                message: message.clone(),
                detail: None,
            }],
        };

        let mut res = (status, Json(oci_body)).into_response();
        res.headers_mut().insert(
            "Docker-Distribution-API-Version",
            HeaderValue::from_static("registry/2.0"),
        );

        if status == StatusCode::UNAUTHORIZED {
            res.headers_mut().insert(
                header::WWW_AUTHENTICATE,
                HeaderValue::from_static(
                    r#"Bearer realm="http://localhost:8080/v2/token",service="hub.vyoma.dev""#,
                ),
            );
        }

        res
    }
}
