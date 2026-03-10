use std::sync::{Arc, LazyLock};

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

use crate::app::AppState;
use crate::error::ErrorResponse;
use crate::routes::middlewares::ValidatedBody;
use crate::services::account::{LoginParams, RegisterParams};

static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9._]*[a-zA-Z0-9])?$").unwrap());

fn validate_strong_password(password: &str) -> Result<(), ValidationError> {
    let checks = [
        (
            password.chars().any(|c| c.is_uppercase()),
            "must contain at least one uppercase letter",
        ),
        (
            password.chars().any(|c| c.is_lowercase()),
            "must contain at least one lowercase letter",
        ),
        (
            password.chars().any(|c| c.is_ascii_digit()),
            "must contain at least one digit",
        ),
        (
            password.chars().any(|c| !c.is_alphanumeric()),
            "must contain at least one special character",
        ),
    ];

    for (valid, msg) in checks {
        if !valid {
            let mut err = ValidationError::new("strong_password");
            err.message = Some(msg.into());
            return Err(err);
        }
    }

    Ok(())
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct RegisterRequest {
    #[validate(
        length(min = 3, max = 20, message = "must be between 3 and 20 characters"),
        regex(path = *USERNAME_REGEX, message = "must start and end with a letter or digit, and only contain letters, digits, dots and underscores")
    )]
    pub username: String,
    #[validate(email(message = "must be a valid email address"))]
    pub email: String,
    #[validate(
        length(min = 8, message = "must be at least 8 characters"),
        custom(function = validate_strong_password)
    )]
    pub password: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/account/register", post(register))
        .route("/account/login", post(login))
}

#[utoipa::path(
    post,
    path = "/account/register",
    summary = "Register a new account",
    description = "Creates a new player account. Returns 409 if the email or username is already in use.",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Account created"),
        (status = 409, description = "Email or username already in use", body = ErrorResponse),
    ),
    tag = "Auth",
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    ValidatedBody(body): ValidatedBody<RegisterRequest>,
) -> Result<StatusCode, ErrorResponse> {
    state
        .account_service
        .register(RegisterParams {
            username: body.username,
            email: body.email,
            password: body.password,
        })
        .await?;

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "must be a valid email address"))]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

#[utoipa::path(
    post,
    path = "/account/login",
    summary = "Login",
    description = "Authenticates an account and returns a JWT token.",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
    ),
    tag = "Auth",
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    ValidatedBody(body): ValidatedBody<LoginRequest>,
) -> Result<Json<LoginResponse>, ErrorResponse> {
    let result = state
        .account_service
        .login(LoginParams {
            email: body.email,
            password: body.password,
        })
        .await?;

    Ok(Json(LoginResponse {
        token: result.token,
    }))
}
