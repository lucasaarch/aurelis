use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use shared::models::character_class::CharacterClass;
use utoipa::ToSchema;
use validator::Validate;

use crate::app::AppState;
use crate::error::ErrorResponse;
use crate::routes::middlewares::{AuthUser, ValidatedBody};
use crate::services::character::CreateCharacterInput;
use crate::utils::datetime::format_naive_datetime;

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateCharacterRequest {
    #[validate(length(min = 3, max = 24, message = "must be between 3 and 24 characters"))]
    pub name: String,
    pub class: String,
}

#[derive(Serialize, ToSchema)]
pub struct CreateCharacterResponse {
    pub id: String,
    pub name: String,
    pub class: CharacterClass,
}

#[derive(Serialize, ToSchema)]
pub struct CharacterResponse {
    pub id: String,
    pub name: String,
    pub class: CharacterClass,
    pub level: i16,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, ToSchema)]
pub struct ListCharactersResponse {
    pub characters: Vec<CharacterResponse>,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/characters", post(create_character))
        .route("/characters", get(list_characters))
}

#[utoipa::path(
    post,
    path = "/characters",
    summary = "Create a character",
    description = "Creates a new character for the authenticated account.",
    request_body = CreateCharacterRequest,
    responses(
        (status = 201, description = "Character created", body = CreateCharacterResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 409, description = "Name already taken", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Character"
)]
pub async fn create_character(
    State(state): State<Arc<AppState>>,
    AuthUser(account_id): AuthUser,
    ValidatedBody(body): ValidatedBody<CreateCharacterRequest>,
) -> Result<(StatusCode, Json<CreateCharacterResponse>), ErrorResponse> {
    let character = state
        .character_service
        .create(
            account_id,
            CreateCharacterInput {
                name: body.name,
                class: body.class,
            },
        )
        .await
        .map_err(ErrorResponse::from)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateCharacterResponse {
            id: character.id.to_string(),
            name: character.name,
            class: character.class,
        }),
    ))
}

#[utoipa::path(
    get,
    path = "/characters",
    summary = "List characters",
    description = "Returns all characters for the authenticated account",
    responses(
        (status = 200, description = "Characters list", body = ListCharactersResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
    ),
    security(("bearer_auth" = [])),
    tag = "Character"
)]
pub async fn list_characters(
    State(state): State<Arc<AppState>>,
    AuthUser(account_id): AuthUser,
) -> Result<(StatusCode, Json<ListCharactersResponse>), ErrorResponse> {
    let items = state
        .character_service
        .list_all(account_id)
        .await
        .map_err(ErrorResponse::from)?;

    let response = ListCharactersResponse {
        characters: items
            .into_iter()
            .map(|c| CharacterResponse {
                id: c.id.to_string(),
                name: c.name,
                class: c.class,
                level: c.level,
                created_at: format_naive_datetime(&c.created_at),
                updated_at: format_naive_datetime(&c.updated_at),
            })
            .collect(),
    };

    Ok((StatusCode::OK, Json(response)))
}
