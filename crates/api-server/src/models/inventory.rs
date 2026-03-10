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
    pub inventory_type: InventoryTypeModel,
    pub capacity: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl InventoryModel {
    pub fn new(
        character_id: Uuid,
        inventory_type: InventoryTypeModel,
        capacity: i16,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            character_id,
            inventory_type,
            capacity,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl From<InventoryModel> for Inventory {
    fn from(model: InventoryModel) -> Self {
        Self {
            id: model.id,
            character_id: model.character_id,
            capacity: model.capacity,
            inventory_type: model.inventory_type.into(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
