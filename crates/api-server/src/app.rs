use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::config::Config;
use crate::db::Database;
use crate::repositories::account::PgAccountRepository;
use crate::repositories::character::PgCharacterRepository;
use crate::routes;
use crate::services::account::AccountService;
use crate::services::character::CharacterService;
use crate::services::hash::HashService;
use crate::services::jwt::JwtService;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::account::register,
        routes::account::login,
        routes::character::create_character,
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Character", description = "Character management"),
    ),
    components(
        schemas()
    ),
    security(
        ("bearer_auth" = [])
    ),
    info(
        title = "Resona API"
    )
)]
struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    pub account_service: AccountService,
    pub character_service: CharacterService,
    pub jwt_service: JwtService,
}

impl AppState {
    fn new(db: Database, config: &Config) -> Self {
        let hash_service = HashService::default();
        let jwt_service = JwtService::new(&config.jwt.secret, config.jwt.expiration_seconds);
        let account_repository = PgAccountRepository::new(db.clone());
        let account_service = AccountService::new(account_repository, hash_service, jwt_service.clone());
        let character_repository = PgCharacterRepository::new(db);
        let character_service = CharacterService::new(character_repository);

        Self { account_service, character_service, jwt_service }
    }
}

pub async fn run(config: Config) {
    let db = Database::new(&config.database.url);
    let state = Arc::new(AppState::new(db, &config));

    let app = Router::new()
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .merge(routes::account::router())
        .merge(routes::character::router())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.server.port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    info!("Server listening on {addr}");
    info!("API docs available at http://{addr}/docs");

    axum::serve(listener, app).await.expect("Server error");
}
