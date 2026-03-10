use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::models::{character_class::CharacterClass, equipment_slot::EquipmentSlot, item_rarity::ItemRarity};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub class: Option<CharacterClass>,
    pub description: Option<String>,
    pub rarity: ItemRarity,
    pub equipment_slot: Option<EquipmentSlot>,
    pub level_req: i16,
    pub stats: Value,
    pub created_at: NaiveDateTime,
}
