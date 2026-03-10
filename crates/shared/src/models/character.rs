use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{character_class::CharacterClass, character_location::CharacterLocation};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub class: CharacterClass,
    pub level: i16,
    pub experience: i64,
    pub location: CharacterLocation,
    pub equipment_inventory_capacity: i16,
    pub accessory_inventory_capacity: i16,
    pub consumable_inventory_capacity: i16,
    pub material_inventory_capacity: i16,
    pub quest_item_inventory_capacity: i16,
    pub special_inventory_capacity: i16,
    pub credits: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}