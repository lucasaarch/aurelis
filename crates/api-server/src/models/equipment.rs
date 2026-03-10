use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use shared::models::equipment::Equipment;
use uuid::Uuid;

use crate::models::equipment_slot::EquipmentSlotModel;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::equipment)]
pub struct EquipmentModel {
    pub character_id: Uuid,
    pub slot: EquipmentSlotModel,
    pub inventory_id: Uuid,
    pub equipped_at: NaiveDateTime,
}

impl EquipmentModel {
    pub fn new(character_id: Uuid, slot: EquipmentSlotModel, inventory_id: Uuid) -> Self {
        Self {
            character_id,
            slot,
            inventory_id,
            equipped_at: Utc::now().naive_utc(),
        }
    }
}

impl From<EquipmentModel> for Equipment {
    fn from(model: EquipmentModel) -> Self {
        Self {
            character_id: model.character_id,
            slot: model.slot.into(),
            inventory_id: model.inventory_id,
            equipped_at: model.equipped_at,
        }
    }
}
