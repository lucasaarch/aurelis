use crate::error::AppError;
use crate::repositories::item::{CreateItemParams, PgItemRepository};
use crate::services::account::AccountService;
use crate::services::inventory::InventoryService;
use crate::utils::slug::generate_slug;
use shared::models::inventory_type::InventoryType;
use shared::models::item::Item;
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

    pub async fn create(&self, actor_id: Uuid, input: CreateItemInput) -> Result<Item, AppError> {
        let account = match self.account_service.find_by_id(actor_id).await {
            Ok(a) => a,
            Err(_) => return Err(AppError::Unauthorized),
        };

        if !account.is_admin {
            return Err(AppError::Unauthorized);
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
            Err(_) => return Err(AppError::Unauthorized),
        };

        if !account.is_admin {
            return Err(AppError::Unauthorized);
        }

        let item = self.find_by_id(item_id).await?;

        match item.inventory_type {
            InventoryType::Consumable
            | InventoryType::Material
            | InventoryType::QuestItem
            | InventoryType::Special => {
                let inv_type: String = item.inventory_type.into();

                if let Some(existing) = self
                    .inventory_service
                    .find_slot_by_item(character_id, inv_type.clone(), item_id)
                    .await?
                {
                    self.inventory_service
                        .increment_quantity(existing.id, quantity)
                        .await?;

                    return Ok(());
                }

                let slot = self
                    .inventory_service
                    .find_next_available_slot(character_id, inv_type.clone())
                    .await?
                    .ok_or(AppError::Conflict("INVENTORY_FULL".into()))?;

                self.inventory_service
                    .insert_item_slot(character_id, inv_type.clone(), item_id, slot, quantity)
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
