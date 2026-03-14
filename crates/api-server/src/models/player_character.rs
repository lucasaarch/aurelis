use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::player_characters)]
pub struct PlayerCharacterModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub character_id: Uuid,
    pub current_class_slug: String,
    pub level: i16,
    pub experience: i64,
    pub credits: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PlayerCharacterModel {
    pub fn new(
        account_id: Uuid,
        name: String,
        character_id: Uuid,
        current_class_slug: String,
    ) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: Uuid::new_v4(),
            account_id,
            name,
            character_id,
            current_class_slug,
            level: 1,
            experience: 0,
            credits: 0,
            created_at: now,
            updated_at: now,
        }
    }
}
