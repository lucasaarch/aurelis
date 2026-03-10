use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde_json::Value;
use shared::models::item::Item;
use uuid::Uuid;

use crate::models::{
    character_class::CharacterClassModel, equipment_slot::EquipmentSlotModel,
    inventory_type::InventoryTypeModel, item_rarity::ItemRarityModel,
};

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::items)]
pub struct ItemModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub rarity: ItemRarityModel,
    pub equipment_slot: Option<EquipmentSlotModel>,
    pub class: Option<CharacterClassModel>,
    pub level_req: Option<i16>,
    pub stats: Option<Value>,
    pub created_at: NaiveDateTime,
    pub slug: String,
    pub inventory_type: InventoryTypeModel,
    pub max_stack: i16,
}

impl ItemModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        description: Option<String>,
        rarity: ItemRarityModel,
        equipment_slot: Option<EquipmentSlotModel>,
        class: Option<CharacterClassModel>,
        level_req: Option<i16>,
        stats: Option<Value>,
        slug: String,
        inventory_type: InventoryTypeModel,
        max_stack: i16,
    ) -> Self {
        use chrono::Utc;
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            rarity,
            equipment_slot,
            class,
            level_req,
            stats,
            created_at: Utc::now().naive_utc(),
            slug,
            inventory_type,
            max_stack,
        }
    }

    pub fn equipable(&self) -> bool {
        self.equipment_slot.is_some()
    }
}

impl From<ItemModel> for Item {
    fn from(model: ItemModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            class: model.class.map(Into::into),
            description: model.description,
            rarity: model.rarity.into(),
            equipment_slot: model.equipment_slot.map(Into::into),
            level_req: model.level_req,
            stats: model.stats,
            created_at: model.created_at,
            slug: model.slug,
            inventory_type: model.inventory_type.into(),
            max_stack: model.max_stack,
        }
    }
}
