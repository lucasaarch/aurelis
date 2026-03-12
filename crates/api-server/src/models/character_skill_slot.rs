use diesel::prelude::{Insertable, Queryable};
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
