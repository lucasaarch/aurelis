use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Clone, Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::item_instance_gems)]
pub struct ItemInstanceGemModel {
    pub id: Uuid,
    pub item_instance_id: Uuid,
    pub slot_index: i16,
    pub gem_instance_id: Uuid,
    pub socketed_at: NaiveDateTime,
}

impl ItemInstanceGemModel {
    pub fn new(item_instance_id: Uuid, slot_index: i16, gem_instance_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            item_instance_id,
            slot_index,
            gem_instance_id,
            socketed_at: Utc::now().naive_utc(),
        }
    }
}
