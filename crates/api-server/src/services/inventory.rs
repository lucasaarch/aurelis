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

    pub async fn find_slot_by_item_with_space(
        &self,
        character_id: Uuid,
        inv_type: String,
        item_id: Uuid,
        max_stack: i16,
    ) -> Result<Option<InventoryItem>, AppError> {
        let inv = self
            .inventory_repository
            .find_by_character_and_type(character_id, inv_type)
            .await?;

        self.inventory_repository
            .find_slot_by_item_with_space(inv.id, item_id, max_stack)
            .await
            .map_err(Into::into)
    }

    pub async fn move_item(
        &self,
        character_id: Uuid,
        inv_type: String,
        from_slot: i16,
        to_slot: i16,
    ) -> Result<(), AppError> {
        let inv = self
            .inventory_repository
            .find_by_character_and_type(character_id, inv_type)
            .await?;

        let from_item = self
            .inventory_repository
            .find_slot_by_index(inv.id, from_slot)
            .await?
            .ok_or(AppError::NotFound)?;

        if let Some(to_item) = self
            .inventory_repository
            .find_slot_by_index(inv.id, to_slot)
            .await?
        {
            self.inventory_repository
                .swap_slots(
                    from_item.id,
                    to_item.id,
                    from_item.slot_index,
                    to_item.slot_index,
                )
                .await?;
        } else {
            self.inventory_repository
                .update_slot_index(from_item.id, to_slot)
                .await?;
        }

        Ok(())
    }
}
