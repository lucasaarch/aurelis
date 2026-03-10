use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use shared::models::inventory_item::InventoryItem;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::inventory_items)]
pub struct InventoryItemModel {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub item_instance_id: Option<Uuid>,
    pub item_id: Option<Uuid>,
    pub slot_index: i16,
    pub quantity: i16,
    pub acquired_at: NaiveDateTime,
}

impl InventoryItemModel {
    pub fn new(
        inventory_id: Uuid,
        item_instance_id: Option<Uuid>,
        item_id: Option<Uuid>,
        slot_index: i16,
        quantity: i16,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            inventory_id,
            item_instance_id,
            item_id,
            slot_index,
            quantity,
            acquired_at: Utc::now().naive_utc(),
        }
    }
}

impl From<InventoryItemModel> for InventoryItem {
    fn from(model: InventoryItemModel) -> Self {
        Self {
            id: model.id,
            inventory_id: model.inventory_id,
            item_instance_id: model.item_instance_id,
            item_id: model.item_id,
            slot_index: model.slot_index,
            quantity: model.quantity,
            acquired_at: model.acquired_at,
        }
    }
}
