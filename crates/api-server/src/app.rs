use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::config::Config;
use crate::db::Database;
use crate::repositories::account::PgAccountRepository;
use crate::routes;
use crate::services::account::AccountService;
use crate::services::hash::HashService;

#[derive(Clone)]
pub struct AppState {
    pub account_service: AccountService,
}

impl AppState {
    fn new(db: Database) -> Self {
        let hash_service = HashService::default();
        let account_repository = PgAccountRepository::new(db);
        let account_service = AccountService::new(account_repository, hash_service);

        Self { account_service }
    }
}

pub async fn run(config: Config) {
    let db = Database::new(&config.database.url);
    let state = Arc::new(AppState::new(db));

    let app = Router::new()
        .merge(routes::account::router())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.server.port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");

    info!("Server listening on {addr}");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
