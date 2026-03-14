use std::sync::Arc;

use crate::config::Config;
use crate::db::Database;
use crate::repositories::character::PgCharacterRepository;
use crate::repositories::dungeon::PgDungeonRepository;
use crate::repositories::item::PgItemRepository;
use crate::repositories::mob::PgMobRepository;
use crate::repositories::quest::PgQuestRepository;

pub use openapi::ApiDoc;
pub use state::AppState;

mod bootstrap;
mod grpc_server;
mod http;
mod openapi;
mod state;

const HTTP_ADDR: &str = "0.0.0.0:8080";
const GRPC_ADDR: &str = "0.0.0.0:50051";

pub async fn run(config: Config) {
    let db = Database::new(&config.database.url);
    let item_repository = PgItemRepository::new(db.clone());
    let dungeon_repository = PgDungeonRepository::new(db.clone());
    let mob_repository = PgMobRepository::new(db.clone());
    let quest_repository = PgQuestRepository::new(db.clone());
    let character_repository = PgCharacterRepository::new(db.clone());
    bootstrap::sync_items(&item_repository)
        .await
        .expect("Failed to synchronize item catalog");
    bootstrap::sync_dungeons(&dungeon_repository)
        .await
        .expect("Failed to synchronize dungeon catalog");
    bootstrap::sync_mobs(&mob_repository)
        .await
        .expect("Failed to synchronize mob catalog");
    bootstrap::sync_characters(&character_repository)
        .await
        .expect("Failed to synchronize character catalog");
    bootstrap::sync_quests(&quest_repository)
        .await
        .expect("Failed to synchronize quest catalog");
    let synced_paths = bootstrap::sync_character_class_paths(&character_repository)
        .await
        .expect("Failed to synchronize character class paths");
    bootstrap::sync_character_class_path_classes(&character_repository, synced_paths)
        .await
        .expect("Failed to synchronize character class path classes");
    bootstrap::validate_player_character_integrity(&character_repository)
        .await
        .expect("Failed to validate player character integrity");
    let state = Arc::new(AppState::new(db, &config));

    let http_state = state.clone();
    let http_config = config.clone();
    let http = tokio::spawn(async move {
        http::serve_http(http_state, http_config, HTTP_ADDR).await;
    });

    let grpc_state = state.clone();
    let grpc = tokio::spawn(async move {
        grpc_server::serve_grpc(grpc_state, GRPC_ADDR).await;
    });

    let _ = tokio::join!(http, grpc);
}
