use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::models::{character_class::CharacterClass, equipment_slot::EquipmentSlot, inventory_type::InventoryType, item_rarity::ItemRarity};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub class: Option<CharacterClass>,
    pub description: Option<String>,
    pub rarity: ItemRarity,
    pub equipment_slot: Option<EquipmentSlot>,
    pub level_req: Option<i16>,
    pub stats: Option<Value>,
    pub created_at: NaiveDateTime,
    pub inventory_type: InventoryType
}
