use crate::models::inventory_type::InventoryTypeModel;
use crate::{
    db::{
        Database,
        schema::{inventory, inventory_items},
    },
    models::{inventory::InventoryModel, inventory_item::InventoryItemModel, item::ItemModel},
    repositories::{Repository, RepositoryError},
};
use diesel::prelude::*;
use shared::models::inventory::Inventory;
use shared::models::inventory_detailed_item::InventoryDetailedItem;
use shared::models::inventory_item::InventoryItem;
use shared::models::item::Item as DomainItem;
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

    /// Fetch inventory items for a character and inventory type together with item details.
    /// This performs a LEFT JOIN and expects every inventory row to have an associated Item.
    /// If any inventory row lacks its Item, a `RepositoryError::NotFound` is returned.
    pub async fn find_items_with_details_by_character_and_type(
        &self,
        character_id: Uuid,
        inv_type: String,
    ) -> Result<Vec<InventoryDetailedItem>, RepositoryError> {
        // Resolve the inventory for the given character and type
        let inv = self
            .find_by_character_and_type(character_id, inv_type)
            .await?;

        self.run_blocking(move |conn| {
            use crate::db::schema::items;

            // LEFT JOIN inventory_items -> items; select both sets of columns (items nullable)
            let query = inventory_items::table
                .left_join(items::table)
                .filter(inventory_items::inventory_id.eq(inv.id))
                .order(inventory_items::slot_index.asc())
                .select((inventory_items::all_columns, items::all_columns.nullable()));

            let rows: Vec<(InventoryItemModel, Option<ItemModel>)> = query.load(conn)?;

            // Convert rows into InventoryDetailedItem; return NotFound if any item is missing.
            let mut out: Vec<InventoryDetailedItem> = Vec::with_capacity(rows.len());

            for (im, opt_item) in rows.into_iter() {
                let inv_item: InventoryItem = im.into();
                match opt_item {
                    Some(item_model) => {
                        let item: DomainItem = item_model.into();
                        out.push(InventoryDetailedItem::from((inv_item, item)));
                    }
                    None => {
                        // Item missing for a slot — treat as inconsistent data
                        return Err(RepositoryError::NotFound);
                    }
                }
            }

            Ok(out)
        })
        .await
    }

    pub async fn delete_slot(
        &self,
        inventory_id: Uuid,
        slot_index: i16,
    ) -> Result<(), RepositoryError> {
        self.run_blocking(move |conn| {
            diesel::delete(inventory_items::table)
                .filter(inventory_items::inventory_id.eq(inventory_id))
                .filter(inventory_items::slot_index.eq(slot_index))
                .execute(conn)
                .map(|_| ())
                .map_err(Into::into)
        })
        .await
    }
}
