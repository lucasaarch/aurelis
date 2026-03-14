use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Clone, Queryable, Insertable)]
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
