use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::character_class_path_classes)]
pub struct CharacterClassPathClassModel {
    pub id: Uuid,
    pub slug: String,
    pub character_class_path_id: Uuid,
    pub created_at: NaiveDateTime,
}

impl CharacterClassPathClassModel {
    pub fn new(slug: String, character_class_path_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            slug,
            character_class_path_id,
            created_at: Utc::now().naive_utc(),
        }
    }
}
