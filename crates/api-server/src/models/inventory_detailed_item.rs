use chrono::NaiveDateTime;
use serde_json::Value;
use uuid::Uuid;

use crate::models::{
    character_class::CharacterClassModel, equipment_slot::EquipmentSlotModel,
    inventory_item::InventoryItemModel, inventory_type::InventoryTypeModel, item::ItemModel,
    item_rarity::ItemRarityModel,
};

#[derive(Clone)]
pub struct InventoryDetailedItem {
    // InventoryItem fields
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub item_instance_id: Option<Uuid>,
    pub item_id: Option<Uuid>,
    pub slot_index: i16,
    pub quantity: i16,
    pub acquired_at: NaiveDateTime,

    // Item fields
    pub slug: String,
    pub name: String,
    pub class: Option<CharacterClassModel>,
    pub description: Option<String>,
    pub rarity: ItemRarityModel,
    pub equipment_slot: Option<EquipmentSlotModel>,
    pub level_req: Option<i16>,
    pub stats: Option<Value>,
    pub created_at: NaiveDateTime,
    pub inventory_type: InventoryTypeModel,
    pub max_stack: i16,
}

impl From<(InventoryItemModel, ItemModel)> for InventoryDetailedItem {
    fn from((inv_item, item): (InventoryItemModel, ItemModel)) -> Self {
        InventoryDetailedItem {
            id: inv_item.id,
            inventory_id: inv_item.inventory_id,
            item_instance_id: inv_item.item_instance_id,
            item_id: inv_item.item_id,
            slot_index: inv_item.slot_index,
            quantity: inv_item.quantity,
            acquired_at: inv_item.acquired_at,

            slug: item.slug,
            name: item.name,
            class: item.class,
            description: item.description,
            rarity: item.rarity,
            equipment_slot: item.equipment_slot,
            level_req: item.level_req,
            stats: item.stats,
            created_at: item.created_at,
            inventory_type: item.inventory_type,
            max_stack: item.max_stack,
        }
    }
}
