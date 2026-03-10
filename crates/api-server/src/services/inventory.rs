use shared::models::inventory_item::InventoryItem;
use uuid::Uuid;

use crate::{error::AppError, repositories::inventory::PgInventoryRepository};

#[derive(Clone)]
pub struct InventoryService {
    inventory_repository: PgInventoryRepository,
}

impl InventoryService {
    pub fn new(inventory_repository: PgInventoryRepository) -> Self {
        Self {
            inventory_repository,
        }
    }

    pub async fn find_next_available_slot(
        &self,
        character_id: Uuid,
        inv_type: String,
    ) -> Result<Option<i16>, AppError> {
        let inv = self
            .inventory_repository
            .find_by_character_and_type(character_id, inv_type)
            .await?;

        self.inventory_repository
            .find_next_available_slot(inv.id, inv.capacity)
            .await
            .map_err(Into::into)
    }

    pub async fn find_slot_by_item(
        &self,
        character_id: Uuid,
        inv_type: String,
        item_id: Uuid,
    ) -> Result<Option<InventoryItem>, AppError> {
        let inv = self
            .inventory_repository
            .find_by_character_and_type(character_id, inv_type)
            .await?;

        self.inventory_repository
            .find_slot_by_item(inv.id, item_id)
            .await
            .map_err(Into::into)
    }

    pub async fn increment_quantity(&self, slot_id: Uuid, amount: i16) -> Result<(), AppError> {
        self.inventory_repository
            .increment_quantity(slot_id, amount)
            .await
            .map_err(Into::into)
    }

    pub async fn insert_item_slot(
        &self,
        character_id: Uuid,
        inv_type: String,
        item_id: Uuid,
        slot_index: i16,
        quantity: i16,
    ) -> Result<InventoryItem, AppError> {
        let inv = self
            .inventory_repository
            .find_by_character_and_type(character_id, inv_type)
            .await?;

        self.inventory_repository
            .insert_item_slot(inv.id, item_id, slot_index, quantity)
            .await
            .map_err(Into::into)
    }
}
