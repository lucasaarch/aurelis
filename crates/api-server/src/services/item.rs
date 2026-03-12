use crate::error::AppError;
use crate::models::item::ItemModel;
use crate::repositories::item::{ListItemFilters, PgItemRepository};
use crate::services::account::AccountService;
use crate::services::character::CharacterService;
use crate::services::inventory::InventoryService;
use uuid::Uuid;

#[derive(Clone)]
pub struct ItemService {
    item_repository: PgItemRepository,
    account_service: AccountService,
    character_service: CharacterService,
    inventory_service: InventoryService,
}

impl ItemService {
    pub fn new(
        item_repository: PgItemRepository,
        account_service: AccountService,
        character_service: CharacterService,
        inventory_service: InventoryService,
    ) -> Self {
        Self {
            item_repository,
            account_service,
            character_service,
            inventory_service,
        }
    }

    pub async fn find_by_id(&self, item_id: Uuid) -> Result<ItemModel, AppError> {
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
        filters: ListItemFilters,
    ) -> Result<(Vec<ItemModel>, i64), AppError> {
        self.ensure_admin(actor_id).await?;

        self.item_repository
            .list(page, limit, filters)
            .await
            .map_err(Into::into)
    }

    pub async fn get_by_slug(&self, actor_id: Uuid, slug: String) -> Result<ItemModel, AppError> {
        self.ensure_admin(actor_id).await?;

        self.item_repository
            .find_by_slug(slug)
            .await
            .map_err(Into::into)
    }

    /// Batch fetch items by ids. Returns an empty vec when `ids` is empty.
    pub async fn list_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<ItemModel>, AppError> {
        self.item_repository
            .list_by_ids(ids)
            .await
            .map_err(Into::into)
    }

    pub async fn give_item(
        &self,
        actor_id: Uuid,
        item_slug: String,
        character_username: String,
        quantity: Option<i16>,
    ) -> Result<(), AppError> {
        self.ensure_admin(actor_id).await?;

        let item = self.item_repository.find_by_slug(item_slug).await?;

        let character = self
            .character_service
            .find_by_name(character_username)
            .await?;

        let inv_type = item.inventory_type.to_string();
        let max_stack = item.max_stack.max(1);
        let mut remaining = quantity.unwrap_or(1);

        while remaining > 0 {
            if max_stack > 1 {
                if let Some(slot) = self
                    .inventory_service
                    .find_slot_by_item_with_space(
                        character.id,
                        inv_type.clone(),
                        item.id,
                        max_stack,
                    )
                    .await?
                {
                    let space = max_stack - slot.quantity;
                    let add = remaining.min(space);
                    self.inventory_service
                        .increment_quantity(slot.id, add)
                        .await?;
                    remaining -= add;
                    continue;
                }
            }

            let slot = self
                .inventory_service
                .find_next_available_slot(character.id, inv_type.clone())
                .await?;

            let slot = match slot {
                Some(value) => value,
                None => return Err(AppError::BadRequest("Inventory is full".to_string())),
            };

            let add = remaining.min(max_stack);
            self.inventory_service
                .insert_item_slot(character.id, inv_type.clone(), item.id, slot, add)
                .await?;
            remaining -= add;
        }

        Ok(())
    }

    async fn ensure_admin(&self, actor_id: Uuid) -> Result<(), AppError> {
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

        Ok(())
    }
}
