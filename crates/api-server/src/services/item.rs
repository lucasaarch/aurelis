use crate::error::AppError;
use crate::repositories::account::PgAccountRepository;
use crate::repositories::item::{CreateItemParams, PgItemRepository};
use crate::utils::slug::generate_slug;
use shared::models::item::Item;
use uuid::Uuid;

pub struct CreateItemInput {
    pub name: String,
    pub class: Option<String>,
    pub description: Option<String>,
    pub rarity: String,
    pub equipment_slot: Option<String>,
    pub level_req: i16,
    pub stats: serde_json::Value,
}

#[derive(Clone)]
pub struct ItemService {
    repository: PgItemRepository,
    account_repository: PgAccountRepository,
}

impl ItemService {
    pub fn new(repository: PgItemRepository, account_repository: PgAccountRepository) -> Self {
        Self {
            repository,
            account_repository,
        }
    }

    pub async fn create(&self, actor_id: Uuid, input: CreateItemInput) -> Result<Item, AppError> {
        let account = match self.account_repository.find_by_id(actor_id).await {
            Ok(a) => a,
            Err(_) => return Err(AppError::Unauthorized),
        };

        if !account.is_admin {
            return Err(AppError::Unauthorized);
        }

        let slug = generate_slug(&input.name);

        self.repository
            .create(CreateItemParams {
                slug: slug.clone(),
                name: input.name,
                class: input.class,
                description: input.description,
                rarity: input.rarity,
                equipment_slot: input.equipment_slot,
                level_req: input.level_req,
                stats: input.stats,
            })
            .await
            .map_err(Into::into)
    }
}
