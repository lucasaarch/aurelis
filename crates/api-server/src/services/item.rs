use crate::error::AppError;
use crate::repositories::item::{CreateItemParams, PgItemRepository};
use crate::services::account::AccountService;
use crate::services::inventory::InventoryService;
use shared::models::inventory_type::InventoryType;
use shared::models::item::Item;
use shared::utils::slug::generate_slug;
use uuid::Uuid;

pub struct CreateItemInput {
    pub name: String,
    pub class: Option<String>,
    pub description: Option<String>,
    pub rarity: String,
    pub equipment_slot: Option<String>,
    pub level_req: Option<i16>,
    pub stats: Option<serde_json::Value>,
    pub inventory_type: String,
    pub max_stack: Option<i16>,
}

#[derive(Clone)]
pub struct ItemService {
    item_repository: PgItemRepository,
    account_service: AccountService,
    inventory_service: InventoryService,
}

impl ItemService {
    pub fn new(
        item_repository: PgItemRepository,
        account_service: AccountService,
        inventory_service: InventoryService,
    ) -> Self {
        Self {
            item_repository,
            account_service,
            inventory_service,
        }
    }

    pub async fn find_by_id(&self, item_id: Uuid) -> Result<Item, AppError> {
        self.item_repository
            .find_by_id(item_id)
            .await
            .map_err(Into::into)
    }

    pub async fn list(
        &self,
        actor_id: Uuid,
        page: i64,
        limit: i64,
    ) -> Result<(Vec<Item>, i64), AppError> {
        let account = match self.account_service.find_by_id(actor_id).await {
            Ok(a) => a,
            Err(_) => {
                return Err(AppError::Unauthorized(
                    "Unable to fetch account data".to_string(),
                ));
            }
        };

        if !account.is_admin {
            return Err(AppError::PermissionDenied(
                "Only admins can access this resource".to_string(),
            ));
        }

        self.item_repository
            .list(page, limit)
            .await
            .map_err(Into::into)
    }

    /// Batch fetch items by ids. Returns an empty vec when `ids` is empty.
    pub async fn list_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Item>, AppError> {
        self.item_repository
            .list_by_ids(ids)
            .await
            .map_err(Into::into)
    }

    pub async fn create(&self, actor_id: Uuid, input: CreateItemInput) -> Result<Item, AppError> {
        let account = match self.account_service.find_by_id(actor_id).await {
            Ok(a) => a,
            Err(_) => {
                return Err(AppError::Unauthorized(
                    "Unable to fetch account data".to_string(),
                ));
            }
        };

        if !account.is_admin {
            return Err(AppError::PermissionDenied(
                "Only admins can access this resource".to_string(),
            ));
        }

        let slug = generate_slug(&input.name);

        self.item_repository
            .create(CreateItemParams {
                slug: slug.clone(),
                name: input.name,
                class: input.class,
                description: input.description,
                rarity: input.rarity,
                equipment_slot: input.equipment_slot,
                level_req: input.level_req,
                stats: input.stats,
                inventory_type: input.inventory_type,
                max_stack: input.max_stack,
            })
            .await
            .map_err(Into::into)
    }

    pub async fn give_item(
        &self,
        actor_id: Uuid,
        character_id: Uuid,
        item_id: Uuid,
        quantity: i16,
    ) -> Result<(), AppError> {
        let account = match self.account_service.find_by_id(actor_id).await {
            Ok(a) => a,
            Err(_) => {
                return Err(AppError::Unauthorized(
                    "Unable to fetch account data".to_string(),
                ));
            }
        };

        if !account.is_admin {
            return Err(AppError::PermissionDenied(
                "Only admins can access this resource".to_string(),
            ));
        }

        let item = self.find_by_id(item_id).await?;

        match item.inventory_type {
            InventoryType::Consumable
            | InventoryType::Material
            | InventoryType::QuestItem
            | InventoryType::Special => {
                let inv_type: String = item.inventory_type.into();
                let max_stack = item.max_stack;
                let mut remaining = quantity;

                while remaining > 0 {
                    if let Some(existing) = self
                        .inventory_service
                        .find_slot_by_item_with_space(
                            character_id,
                            inv_type.clone(),
                            item_id,
                            max_stack,
                        )
                        .await?
                    {
                        let space = max_stack - existing.quantity;
                        let to_add = remaining.min(space);
                        self.inventory_service
                            .increment_quantity(existing.id, to_add)
                            .await?;
                        remaining -= to_add;
                    } else {
                        let slot = self
                            .inventory_service
                            .find_next_available_slot(character_id, inv_type.clone())
                            .await?
                            .ok_or(AppError::Conflict("INVENTORY_FULL".into()))?;

                        let to_add = remaining.min(max_stack);
                        self.inventory_service
                            .insert_item_slot(character_id, inv_type.clone(), item_id, slot, to_add)
                            .await?;
                        remaining -= to_add;
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }
}
