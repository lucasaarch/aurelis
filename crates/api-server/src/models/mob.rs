use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use shared::models::mob::Mob;
use uuid::Uuid;

use crate::models::mob_type::MobTypeModel;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::mobs)]
pub struct MobModel {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub mob_type: MobTypeModel,
    pub created_at: NaiveDateTime,
}

impl MobModel {
    pub fn new(
        slug: String,
        name: String,
        description: Option<String>,
        mob_type: MobTypeModel,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            slug,
            name,
            description,
            mob_type,
            created_at: Utc::now().naive_utc(),
        }
    }
}

impl From<MobModel> for Mob {
    fn from(model: MobModel) -> Self {
        Self {
            id: model.id,
            slug: model.slug,
            name: model.name,
            description: model.description,
            mob_type: model.mob_type.into(),
            created_at: model.created_at,
        }
    }
}
