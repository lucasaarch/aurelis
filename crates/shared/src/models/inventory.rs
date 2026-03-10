use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::inventory_type::InventoryType;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub id: Uuid,
    pub character_id: Uuid,
    pub inventory_type: InventoryType,
    pub capacity: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
