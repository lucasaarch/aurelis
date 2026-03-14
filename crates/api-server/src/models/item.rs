use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

use crate::models::inventory_type::InventoryTypeModel;

#[derive(Clone, Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::items)]
pub struct ItemModel {
    pub id: Uuid,
    pub slug: String,
    pub inventory_type: InventoryTypeModel,
    pub created_at: NaiveDateTime,
}

impl ItemModel {
    pub fn new(slug: String, inventory_type: InventoryTypeModel) -> Self {
        use chrono::Utc;
        Self {
            id: Uuid::new_v4(),
            slug,
            inventory_type,
            created_at: Utc::now().naive_utc(),
        }
    }
}
