use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::models::{
    inventory_item::InventoryItemModel, inventory_type::InventoryTypeModel, item::ItemModel,
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
    pub inventory_type: InventoryTypeModel,
    pub created_at: NaiveDateTime,
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
            inventory_type: item.inventory_type,
            created_at: item.created_at,
        }
    }
}
