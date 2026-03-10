use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use shared::models::inventory::Inventory;
use uuid::Uuid;

use crate::models::inventory_type::InventoryTypeModel;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::inventory)]
pub struct InventoryModel {
    pub id: Uuid,
    pub character_id: Uuid,
    pub item_instance_id: Option<Uuid>,
    pub item_id: Option<Uuid>,
    pub inventory_type: InventoryTypeModel,
    pub slot_index: i16,
    pub quantity: i16,
    pub acquired_at: NaiveDateTime,
}

impl InventoryModel {
    pub fn new(
        character_id: Uuid,
        item_instance_id: Option<Uuid>,
        item_id: Option<Uuid>,
        inventory_type: InventoryTypeModel,
        slot_index: i16,
        quantity: i16,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            character_id,
            item_instance_id,
            item_id,
            inventory_type,
            slot_index,
            quantity,
            acquired_at: Utc::now().naive_utc(),
        }
    }
}

impl From<InventoryModel> for Inventory {
    fn from(model: InventoryModel) -> Self {
        Self {
            id: model.id,
            character_id: model.character_id,
            item_instance_id: model.item_instance_id,
            item_id: model.item_id,
            inventory_type: model.inventory_type.into(),
            slot_index: model.slot_index,
            quantity: model.quantity,
            acquired_at: model.acquired_at,
        }
    }
}
