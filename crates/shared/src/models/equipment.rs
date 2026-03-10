use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::equipment_slot::EquipmentSlot;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Equipment {
    pub character_id: Uuid,
    pub slot: EquipmentSlot,
    pub inventory_id: Uuid,
    pub equipped_at: NaiveDateTime,
}
