use diesel::prelude::{Insertable, Queryable};
use shared::models::character_consumable_slot::CharacterConsumableSlot;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::character_consumable_slots)]
pub struct CharacterConsumableSlotModel {
    pub character_id: Uuid,
    pub slot: i16,
    pub item_instance_id: Option<Uuid>,
}

impl CharacterConsumableSlotModel {
    pub fn new(character_id: Uuid, slot: i16, item_instance_id: Option<Uuid>) -> Self {
        Self {
            character_id,
            slot,
            item_instance_id,
        }
    }
}

impl From<CharacterConsumableSlotModel> for CharacterConsumableSlot {
    fn from(model: CharacterConsumableSlotModel) -> Self {
        Self {
            character_id: model.character_id,
            slot: model.slot,
            item_instance_id: model.item_instance_id,
        }
    }
}
