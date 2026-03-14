use crate::config::Config;
use crate::db::Database;
use crate::repositories::account::PgAccountRepository;
use crate::repositories::character::PgCharacterRepository;
use crate::repositories::inventory::PgInventoryRepository;
use crate::repositories::item::PgItemRepository;
use crate::services::account::AccountService;
use crate::services::auth::AuthService;
use crate::services::character::CharacterService;
use crate::services::hash::HashService;
use crate::services::inventory::InventoryService;
use crate::services::item::ItemService;
use crate::services::jwt::JwtService;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub account_service: AccountService,
    pub auth_service: AuthService,
    pub character_service: CharacterService,
    pub jwt_service: JwtService,
    pub item_service: ItemService,
    pub inventory_service: InventoryService,
}

impl AppState {
    pub fn new(db: Database, config: &Config) -> Self {
        let account_repository = PgAccountRepository::new(db.clone());
        let character_repository = PgCharacterRepository::new(db.clone());
        let item_repository = PgItemRepository::new(db.clone());
        let inventory_repository = PgInventoryRepository::new(db.clone());

        let hash_service = HashService::default();
        let jwt_service = JwtService::new(
            &config.jwt.secret_web,
            &config.jwt.secret_game,
            &config.jwt.refresh_secret_web,
            &config.jwt.refresh_secret_game,
            config.jwt.expiration_seconds,
            config.jwt.refresh_expiration_seconds,
        );

        let account_service = AccountService::new(account_repository.clone());
        let auth_service = AuthService::new(
            account_repository.clone(),
            hash_service.clone(),
            jwt_service.clone(),
        );
        let character_service = CharacterService::new(
            character_repository,
            account_repository.clone(),
            item_repository.clone(),
        );
        let inventory_service = InventoryService::new(inventory_repository.clone());
        let item_service = ItemService::new(
            item_repository,
            account_service.clone(),
            character_service.clone(),
            inventory_service.clone(),
        );

        Self {
            config: config.clone(),
            account_service,
            auth_service,
            character_service,
            jwt_service,
            item_service,
            inventory_service,
        }
    }
}
