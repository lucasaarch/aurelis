use crate::models::inventory_type::InventoryTypeModel;
use crate::{
    db::{
        Database,
        schema::{inventory, inventory_items},
    },
    models::{inventory::InventoryModel, inventory_item::InventoryItemModel},
    repositories::{Repository, RepositoryError},
};
use diesel::prelude::*;
use shared::models::inventory::Inventory;
use shared::models::inventory_item::InventoryItem;
use uuid::Uuid;

#[derive(Clone)]
pub struct PgInventoryRepository {
    db: Database,
}

impl Repository for PgInventoryRepository {
    fn db(&self) -> Database {
        self.db.clone()
    }
}

impl PgInventoryRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_by_character_and_type(
        &self,
        character_id: Uuid,
        inv_type: String,
    ) -> Result<Inventory, RepositoryError> {
        self.run_blocking(move |conn| {
            let inv_type = inv_type
                .parse::<InventoryTypeModel>()
                .map_err(|_| RepositoryError::NotFound)?;

            inventory::table
                .filter(inventory::character_id.eq(character_id))
                .filter(inventory::inventory_type.eq(inv_type))
                .first(conn)
                .map(|r: InventoryModel| r.into())
                .map_err(Into::into)
        })
        .await
    }

    pub async fn find_occupied_slots(
        &self,
        inventory_id: Uuid,
    ) -> Result<Vec<i16>, RepositoryError> {
        self.run_blocking(move |conn| {
            inventory_items::table
                .filter(inventory_items::inventory_id.eq(inventory_id))
                .select(inventory_items::slot_index)
                .load(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn find_next_available_slot(
        &self,
        inventory_id: Uuid,
        capacity: i16,
    ) -> Result<Option<i16>, RepositoryError> {
        let occupied = self.find_occupied_slots(inventory_id).await?;
        for slot in 0..capacity {
            if !occupied.contains(&slot) {
                return Ok(Some(slot));
            }
        }
        Ok(None)
    }

    pub async fn find_slot_by_index(
        &self,
        inventory_id: Uuid,
        slot_index: i16,
    ) -> Result<Option<InventoryItem>, RepositoryError> {
        self.run_blocking(move |conn| {
            inventory_items::table
                .filter(inventory_items::inventory_id.eq(inventory_id))
                .filter(inventory_items::slot_index.eq(slot_index))
                .first::<InventoryItemModel>(conn)
                .map(|r| r.into())
                .optional()
                .map_err(Into::into)
        })
        .await
    }

    pub async fn update_slot_index(
        &self,
        item_id: Uuid,
        slot_index: i16,
    ) -> Result<(), RepositoryError> {
        self.run_blocking(move |conn| {
            diesel::update(inventory_items::table.find(item_id))
                .set(inventory_items::slot_index.eq(slot_index))
                .execute(conn)
                .map(|_| ())
                .map_err(Into::into)
        })
        .await
    }

    pub async fn swap_slots(
        &self,
        from_id: Uuid,
        to_id: Uuid,
        from_slot: i16,
        to_slot: i16,
    ) -> Result<(), RepositoryError> {
        self.run_blocking(move |conn| {
            conn.transaction(|conn| {
                diesel::update(inventory_items::table.find(from_id))
                    .set(inventory_items::slot_index.eq(-1))
                    .execute(conn)?;

                diesel::update(inventory_items::table.find(to_id))
                    .set(inventory_items::slot_index.eq(from_slot))
                    .execute(conn)?;

                diesel::update(inventory_items::table.find(from_id))
                    .set(inventory_items::slot_index.eq(to_slot))
                    .execute(conn)
                    .map(|_| ())
            })
            .map_err(Into::into)
        })
        .await
    }

    pub async fn increment_quantity(
        &self,
        slot_id: Uuid,
        amount: i16,
    ) -> Result<(), RepositoryError> {
        self.run_blocking(move |conn| {
            diesel::update(inventory_items::table.find(slot_id))
                .set(inventory_items::quantity.eq(inventory_items::quantity + amount))
                .execute(conn)
                .map(|_| ())
                .map_err(Into::into)
        })
        .await
    }

    pub async fn insert_item_slot(
        &self,
        inventory_id: Uuid,
        item_id: Uuid,
        slot_index: i16,
        quantity: i16,
    ) -> Result<InventoryItem, RepositoryError> {
        self.run_blocking(move |conn: &mut PgConnection| {
            let slot =
                InventoryItemModel::new(inventory_id, None, Some(item_id), slot_index, quantity);

            diesel::insert_into(inventory_items::table)
                .values(&slot)
                .get_result(conn)
                .map(|m: InventoryItemModel| m.into())
                .map_err(Into::into)
        })
        .await
    }

    pub async fn find_slot_by_item_with_space(
        &self,
        inventory_id: Uuid,
        item_id: Uuid,
        max_stack: i16,
    ) -> Result<Option<InventoryItem>, RepositoryError> {
        self.run_blocking(move |conn| {
            inventory_items::table
                .filter(inventory_items::inventory_id.eq(inventory_id))
                .filter(inventory_items::item_id.eq(item_id))
                .filter(inventory_items::quantity.lt(max_stack))
                .first(conn)
                .map(|r: InventoryItemModel| r.into())
                .optional()
                .map_err(Into::into)
        })
        .await
    }
}
