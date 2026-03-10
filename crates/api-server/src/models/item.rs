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
    pub description: Option<String>,
    pub rarity: ItemRarityModel,
    pub equipment_slot: Option<EquipmentSlotModel>,
    pub class: Option<CharacterClassModel>,
    pub level_req: i16,
    pub stats: Value,
    pub created_at: NaiveDateTime,
    pub slug: String,
}

impl ItemModel {
    pub fn new(
        name: String,
        description: Option<String>,
        rarity: ItemRarityModel,
        equipment_slot: Option<EquipmentSlotModel>,
        class: Option<CharacterClassModel>,
        level_req: i16,
        stats: Value,
        slug: String,
    ) -> Self {
        use chrono::Utc;
        Self {
            id: Uuid::new_v4(),
            slug,
            name,
            description,
            rarity,
            equipment_slot,
            class,
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
            slug: model.slug,
        }
    }
}
