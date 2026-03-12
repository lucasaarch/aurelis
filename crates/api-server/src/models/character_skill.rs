use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::character_skills)]
pub struct CharacterSkillModel {
    pub character_id: Uuid,
    pub skill_id: Uuid,
    pub unlocked_at: NaiveDateTime,
}

impl CharacterSkillModel {
    pub fn new(character_id: Uuid, skill_id: Uuid) -> Self {
        Self {
            character_id,
            skill_id,
            unlocked_at: Utc::now().naive_utc(),
        }
    }
}
