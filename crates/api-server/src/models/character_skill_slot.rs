use diesel::prelude::{Insertable, Queryable};
use shared::models::character_skill_slot::CharacterSkillSlot;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::character_skill_slots)]
pub struct CharacterSkillSlotModel {
    pub character_id: Uuid,
    pub slot: i16,
    pub skill_id: Uuid,
}

impl CharacterSkillSlotModel {
    pub fn new(character_id: Uuid, slot: i16, skill_id: Uuid) -> Self {
        Self {
            character_id,
            slot,
            skill_id,
        }
    }
}

impl From<CharacterSkillSlotModel> for CharacterSkillSlot {
    fn from(model: CharacterSkillSlotModel) -> Self {
        Self {
            character_id: model.character_id,
            slot: model.slot,
            skill_id: model.skill_id,
        }
    }
}
