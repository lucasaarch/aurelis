use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;
use tonic::transport::Server;
use tracing::info;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_scalar::{Scalar, Servable};

use crate::config::Config;
use crate::db::Database;
use crate::proto::auth::auth_service_server::AuthServiceServer;
use crate::repositories::account::PgAccountRepository;
use crate::repositories::character::PgCharacterRepository;
use crate::repositories::mob::PgMobRepository;
use crate::routes;
use crate::services::account::AccountService;
use crate::services::auth::AuthService;
use crate::services::character::CharacterService;
use crate::services::hash::HashService;
use crate::services::jwt::JwtService;
use crate::services::mob::MobService;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components.as_mut().unwrap().add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::auth::register,
        routes::auth::login,
        routes::character::create_character,
        routes::character::list_characters,
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Character", description = "Character management"),
    ),
    modifiers(&SecurityAddon),
    info(
        title = "Resona API"
    )
)]
struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    pub account_service: AccountService,
    pub auth_service: AuthService,
    pub character_service: CharacterService,
    pub mob_service: MobService,
    pub jwt_service: JwtService,
}

impl AppState {
    fn new(db: Database, config: &Config) -> Self {
        let hash_service = HashService::default();
        let jwt_service = JwtService::new(
            &config.jwt.secret_web,
            &config.jwt.secret_game,
            config.jwt.expiration_seconds,
        );
        let account_repository = PgAccountRepository::new(db.clone());
        let account_service = AccountService::new(account_repository.clone());
        let auth_service = AuthService::new(
            account_repository.clone(),
            hash_service.clone(),
            jwt_service.clone(),
        );
        let character_repository = PgCharacterRepository::new(db.clone());
        let character_service =
            CharacterService::new(character_repository, account_repository.clone());
        let mob_repository = PgMobRepository::new(db.clone());
        let mob_service = MobService::new(mob_repository, account_repository.clone());

        Self {
            account_service,
            auth_service,
            character_service,
            mob_service,
            jwt_service,
        }
    }
}

pub async fn run(config: Config) {
    let db = Database::new(&config.database.url);
    let state = Arc::new(AppState::new(db, &config));

    let http = {
        let state = state.clone();
        let port = config.server.http_port;

        tokio::spawn(async move {
            let app = Router::new()
                .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
                .merge(routes::auth::router())
                .merge(routes::character::router())
                .merge(routes::admin::router())
                .with_state(state);

            let addr = format!("0.0.0.0:{}", port);
            let listener = TcpListener::bind(&addr)
                .await
                .expect("Failed to bind address");

            info!("Server listening on {addr}");
            info!("API docs available at http://{addr}/docs");

            axum::serve(listener, app).await.expect("Server error");
        })
    };

    let grpc = {
        let state = state.clone();
        let port = config.server.grpc_port;

        tokio::spawn(async move {
            let addr = format!("0.0.0.0:{}", port);

            info!("gRPC server listeing on {addr}");

            Server::builder()
                .add_service(AuthServiceServer::new(state.auth_service.clone()))
                .serve(addr.parse().unwrap())
                .await
                .expect("gRPC server error");
        })
    };

    let _ = tokio::join!(http, grpc);
}
