use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::character_class_paths)]
pub struct CharacterClassPathModel {
    pub id: Uuid,
    pub character_id: Uuid,
    pub created_at: NaiveDateTime,
}

impl CharacterClassPathModel {
    pub fn new(character_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            character_id,
            created_at: Utc::now().naive_utc(),
        }
    }
}
