use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Query, Request},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::error::ErrorResponse;
use crate::{app::AppState, services::jwt::TokenContext};

pub struct ValidatedBody<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedBody<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| err.into_response())?;

        if let Err(validation_errors) = value.validate() {
            let error = ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                "VALIDATION_FAILED",
                format_validation_errors(&validation_errors),
            );

            return Err(error.into_response());
        }

        Ok(ValidatedBody(value))
    }
}

pub struct ValidatedQuery<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedQuery<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request(req, state)
            .await
            .map_err(|err| err.into_response())?;

        if let Err(validation_errors) = value.validate() {
            let error = ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                "VALIDATION_FAILED",
                format_validation_errors(&validation_errors),
            );

            return Err(error.into_response());
        }

        Ok(ValidatedQuery(value))
    }
}

fn format_validation_errors(errors: &ValidationErrors) -> String {
    errors
        .field_errors()
        .iter()
        .map(|(field, errs)| {
            let messages = errs
                .iter()
                .map(|e| e.message.as_deref().unwrap_or(e.code.as_ref()).to_string())
                .collect::<Vec<_>>()
                .join(", ");

            format!("{field}: {messages}")
        })
        .collect::<Vec<_>>()
        .join("; ")
}

pub struct AuthUser(pub Uuid);

impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));

        let token = match auth_header {
            Some(t) => t,
            None => {
                return Err(ErrorResponse::new(
                    StatusCode::UNAUTHORIZED,
                    "UNAUTHORIZED",
                    "Missing or invalid Authorization header",
                )
                .into_response());
            }
        };

        let claims = state
            .jwt_service
            .verify_with_context(token, TokenContext::Web)
            .map_err(|_| {
                ErrorResponse::new(
                    StatusCode::UNAUTHORIZED,
                    "UNAUTHORIZED",
                    "Invalid or expired token",
                )
                .into_response()
            })?;

        Ok(AuthUser(claims.sub))
    }
}
