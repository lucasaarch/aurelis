use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde_json::Value;
use shared::models::item::Item;
use uuid::Uuid;

use crate::models::{
    character_class::CharacterClassModel, equipment_slot::EquipmentSlotModel,
    item_rarity::ItemRarityModel,
};

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::items)]
pub struct ItemModel {
    pub id: Uuid,
    pub name: String,
    pub class: Option<CharacterClassModel>,
    pub description: Option<String>,
    pub rarity: ItemRarityModel,
    pub equipment_slot: Option<EquipmentSlotModel>,
    pub level_req: i16,
    pub stats: Value,
    pub created_at: NaiveDateTime,
}

impl ItemModel {
    pub fn new(
        name: String,
        class: Option<CharacterClassModel>,
        description: Option<String>,
        rarity: ItemRarityModel,
        equipment_slot: Option<EquipmentSlotModel>,
        level_req: i16,
        stats: Value,
    ) -> Self {
        use chrono::Utc;
        Self {
            id: Uuid::new_v4(),
            name,
            class,
            description,
            rarity,
            equipment_slot,
            level_req,
            stats,
            created_at: Utc::now().naive_utc(),
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
        }
    }
}
