use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::inventory_type::InventoryType;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub id: Uuid,
    pub character_id: Uuid,
    pub item_instance_id: Option<Uuid>,
    pub item_id: Option<Uuid>,
    pub inventory_type: InventoryType,
    pub slot_index: i16,
    pub quantity: i16,
    pub acquired_at: NaiveDateTime,
}
