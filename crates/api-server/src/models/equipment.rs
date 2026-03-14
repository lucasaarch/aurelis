use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

use crate::models::equipment_slot::EquipmentSlotModel;

#[derive(Clone, Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::equipment)]
pub struct EquipmentModel {
    pub character_id: Uuid,
    pub slot: EquipmentSlotModel,
    pub item_instance_id: Uuid,
    pub equipped_at: NaiveDateTime,
}

impl EquipmentModel {
    pub fn new(character_id: Uuid, slot: EquipmentSlotModel, item_instance_id: Uuid) -> Self {
        Self {
            character_id,
            slot,
            item_instance_id,
            equipped_at: Utc::now().naive_utc(),
        }
    }
}
