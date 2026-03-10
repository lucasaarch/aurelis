use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::app::AppState;
use crate::error::ErrorResponse;
use crate::routes::middlewares::ValidatedBody;
use crate::services::auth::RefreshTokenParams;
use crate::services::auth::{LoginParams, RegisterParams};
use crate::services::jwt::TokenContext;
use shared::utils::validation::validate_strong_password;

#[derive(Deserialize, ToSchema, Validate)]
pub struct RegisterRequest {
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
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh_web))
}

#[utoipa::path(
    post,
    path = "/auth/register",
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
        .auth_service
        .register(RegisterParams {
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
    pub access_token: String,
    pub refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/auth/login",
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
        .auth_service
        .login(LoginParams {
            email: body.email,
            password: body.password,
            context: TokenContext::Web,
        })
        .await?;

    Ok(Json(LoginResponse {
        access_token: result.access_token,
        refresh_token: result.refresh_token,
    }))
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    summary = "Refresh Web token",
    description = "Refreshes a Web access token using a refresh token.",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Refresh successful", body = LoginResponse),
        (status = 401, description = "Invalid or expired refresh token", body = ErrorResponse),
    ),
    tag = "Auth",
)]
pub async fn refresh_web(
    State(state): State<Arc<AppState>>,
    ValidatedBody(body): ValidatedBody<RefreshRequest>,
) -> Result<Json<LoginResponse>, ErrorResponse> {
    let result = state
        .auth_service
        .refresh_token(RefreshTokenParams {
            refresh_token: body.refresh_token,
            context: TokenContext::Web,
        })
        .await?;

    Ok(Json(LoginResponse {
        access_token: result.access_token,
        refresh_token: result.refresh_token,
    }))
}
