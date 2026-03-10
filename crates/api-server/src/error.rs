use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

use crate::repositories::RepositoryError;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: u16,
    pub code: String,
    pub message: String,
    pub timestamp: String,
    #[serde(skip)]
    http_status: StatusCode,
}

impl ErrorResponse {
    pub fn new(http_status: StatusCode, code: &str, message: impl Into<String>) -> Self {
        Self {
            status: http_status.as_u16(),
            code: code.to_string(),
            message: message.into(),
            timestamp: Utc::now().to_rfc3339(),
            http_status,
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = self.http_status;
        (status, Json(self)).into_response()
    }
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

impl From<AppError> for ErrorResponse {
    fn from(err: AppError) -> Self {
        match err {
            AppError::NotFound => {
                ErrorResponse::new(StatusCode::NOT_FOUND, "NOT_FOUND", "Not found")
            }
            AppError::Conflict(msg) => ErrorResponse::new(StatusCode::CONFLICT, "CONFLICT", msg),
            AppError::Unauthorized => {
                ErrorResponse::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "Unauthorized")
            }
            AppError::BadRequest(msg) => {
                // Use the provided message as the error code and provide a human-friendly message
                let code = msg.clone();
                let user_message = match msg.as_str() {
                    "MAX_CHARACTERS_REACHED" => {
                        "Maximum number of characters reached for this account".to_string()
                    }
                    other => other.to_string(),
                };

                ErrorResponse::new(StatusCode::BAD_REQUEST, &code, user_message)
            }
            AppError::Internal(_) => ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
                "Internal server error",
            ),
        }
    }
}

impl From<RepositoryError> for AppError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound => AppError::NotFound,
            RepositoryError::Conflict(msg) => AppError::Conflict(msg),
            RepositoryError::Database(msg) | RepositoryError::Internal(msg) => {
                AppError::Internal(anyhow::anyhow!(msg))
            }
        }
    }
}
