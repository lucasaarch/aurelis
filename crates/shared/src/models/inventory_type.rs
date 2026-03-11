use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum InventoryType {
    Equipment,
    Accessory,
    Consumable,
    Material,
    QuestItem,
    Special,
}

impl From<InventoryType> for String {
    fn from(inv_type: InventoryType) -> Self {
        match inv_type {
            InventoryType::Equipment => "equipment",
            InventoryType::Accessory => "accessory",
            InventoryType::Consumable => "consumable",
            InventoryType::Material => "material",
            InventoryType::QuestItem => "quest_item",
            InventoryType::Special => "special",
        }
        .to_string()
    }
}