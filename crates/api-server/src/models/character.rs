use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use shared::models::character::Character;
use uuid::Uuid;

use crate::models::{
    character_class::CharacterClassModel, character_location::CharacterLocationModel,
};

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::characters)]
pub struct CharacterModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub class: CharacterClassModel,
    pub level: i16,
    pub experience: i64,
    pub location: CharacterLocationModel,
    pub credits: i64,
    pub equipment_inventory_capacity: i16,
    pub accessory_inventory_capacity: i16,
    pub consumable_inventory_capacity: i16,
    pub material_inventory_capacity: i16,
    pub quest_item_inventory_capacity: i16,
    pub special_inventory_capacity: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CharacterModel {
    pub fn new(account_id: Uuid, name: String, class: CharacterClassModel) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: Uuid::new_v4(),
            account_id,
            name,
            class,
            level: 1,
            experience: 0,
            location: CharacterLocationModel::Aurelis,
            credits: 0,
            equipment_inventory_capacity: 56,
            accessory_inventory_capacity: 56,
            consumable_inventory_capacity: 56,
            material_inventory_capacity: 56,
            quest_item_inventory_capacity: 56,
            special_inventory_capacity: 56,
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<CharacterModel> for Character {
    fn from(model: CharacterModel) -> Self {
        Self {
            id: model.id,
            account_id: model.account_id,
            name: model.name,
            class: model.class.into(),
            level: model.level,
            experience: model.experience,
            location: model.location.into(),
            equipment_inventory_capacity: model.equipment_inventory_capacity,
            accessory_inventory_capacity: model.accessory_inventory_capacity,
            consumable_inventory_capacity: model.consumable_inventory_capacity,
            material_inventory_capacity: model.material_inventory_capacity,
            quest_item_inventory_capacity: model.quest_item_inventory_capacity,
            special_inventory_capacity: model.special_inventory_capacity,
            credits: model.credits,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
