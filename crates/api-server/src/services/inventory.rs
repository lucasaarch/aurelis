use anyhow::anyhow;
use shared::models::inventory_detailed_item::InventoryDetailedItem;
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

    /// Insert an item into a slot and return the detailed enriched representation.
    pub async fn insert_item_slot(
        &self,
        character_id: Uuid,
        inv_type: String,
        item_id: Uuid,
        slot_index: i16,
        quantity: i16,
    ) -> Result<InventoryDetailedItem, AppError> {
        let inv = self
            .inventory_repository
            .find_by_character_and_type(character_id, inv_type.clone())
            .await?;

        // Insert the raw inventory item
        let _ = self
            .inventory_repository
            .insert_item_slot(inv.id, item_id, slot_index, quantity)
            .await?;

        // Fetch the detailed items for this inventory and return the one we just inserted
        let detailed = self
            .inventory_repository
            .find_items_with_details_by_character_and_type(character_id, inv_type)
            .await?;

        // Find the row that matches the inserted slot/item
        let found = detailed
            .into_iter()
            .find(|d| d.slot_index == slot_index && d.item_id == Some(item_id))
            .ok_or(AppError::Internal(anyhow!(
                "Inserted item detail not found"
            )))?;

        Ok(found)
    }

    /// Find a slot by item that still has space for stacking, returning the detailed item if found.
    pub async fn find_slot_by_item_with_space(
        &self,
        character_id: Uuid,
        inv_type: String,
        item_id: Uuid,
        max_stack: i16,
    ) -> Result<Option<InventoryDetailedItem>, AppError> {
        // Fetch detailed inventory items in one query and search for a slot with space
        let detailed = self
            .inventory_repository
            .find_items_with_details_by_character_and_type(character_id, inv_type)
            .await?;

        for d in detailed.into_iter() {
            if d.item_id == Some(item_id) && d.quantity < max_stack {
                return Ok(Some(d));
            }
        }

        Ok(None)
    }

    /// List all items in the inventory (detailed)
    pub async fn list_items(
        &self,
        character_id: Uuid,
        inv_type: String,
    ) -> Result<Vec<InventoryDetailedItem>, AppError> {
        let items = self
            .inventory_repository
            .find_items_with_details_by_character_and_type(character_id, inv_type)
            .await?;

        Ok(items)
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
            .ok_or(AppError::NotFound(
                "The item you are trying to move does not exist".to_string(),
            ))?;

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

    pub async fn delete_slot(
        &self,
        character_id: Uuid,
        inv_type: String,
        slot_index: i16,
    ) -> Result<(), AppError> {
        let inv = self
            .inventory_repository
            .find_by_character_and_type(character_id, inv_type)
            .await?;

        self.inventory_repository
            .delete_slot(inv.id, slot_index)
            .await
            .map_err(Into::into)
    }
}
