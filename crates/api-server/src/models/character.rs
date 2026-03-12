use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
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
            created_at: now,
            updated_at: now,
        }
    }
}
