use crate::config::Config;
use crate::db::Database;
use crate::repositories::account::PgAccountRepository;
use crate::repositories::character::PgCharacterRepository;
use crate::repositories::inventory::PgInventoryRepository;
use crate::repositories::item::PgItemRepository;
use crate::repositories::item_instance::PgItemInstanceRepository;
use crate::services::account::AccountService;
use crate::services::auth::AuthService;
use crate::services::character::CharacterService;
use crate::services::character_skill::CharacterSkillService;
use crate::services::equipment::EquipmentService;
use crate::services::hash::HashService;
use crate::services::inventory::InventoryService;
use crate::services::item::ItemService;
use crate::services::item_instance::ItemInstanceService;
use crate::services::jwt::JwtService;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub account_service: AccountService,
    pub auth_service: AuthService,
    pub character_service: CharacterService,
    pub character_skill_service: CharacterSkillService,
    pub equipment_service: EquipmentService,
    pub jwt_service: JwtService,
    pub item_service: ItemService,
    pub item_instance_service: ItemInstanceService,
    pub inventory_service: InventoryService,
}

impl AppState {
    pub fn new(db: Database, config: &Config) -> Self {
        let account_repository = PgAccountRepository::new(db.clone());
        let character_repository = PgCharacterRepository::new(db.clone());
        let item_repository = PgItemRepository::new(db.clone());
        let item_instance_repository = PgItemInstanceRepository::new(db.clone());
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
            character_repository.clone(),
            account_repository.clone(),
            item_repository.clone(),
        );
        let character_skill_service = CharacterSkillService::new(character_service.clone());
        let equipment_service = EquipmentService::new(
            character_repository.clone(),
            inventory_repository.clone(),
            item_repository.clone(),
            item_instance_repository.clone(),
        );
        let inventory_service = InventoryService::new(inventory_repository.clone());
        let item_service = ItemService::new(
            item_repository,
            account_service.clone(),
            character_service.clone(),
            inventory_service.clone(),
        );
        let item_instance_service =
            ItemInstanceService::new(item_instance_repository, character_service.clone());

        Self {
            config: config.clone(),
            account_service,
            auth_service,
            character_service,
            character_skill_service,
            equipment_service,
            jwt_service,
            item_service,
            item_instance_service,
            inventory_service,
        }
    }
}
