use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::item_instances)]
pub struct ItemInstanceModel {
    pub id: Uuid,
    pub item_id: Uuid,
    pub refinement: i16,
    pub bonus_gem_slots: i16,
    pub attributes: Value,
    pub owner_character_id: Option<Uuid>,
    pub owner_account_id: Option<Uuid>,
    pub in_shared_storage: bool,
    pub in_trade: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ItemInstanceModel {
    pub fn new(
        item_id: Uuid,
        refinement: i16,
        bonus_gem_slots: i16,
        attributes: Value,
        owner_character_id: Option<Uuid>,
        owner_account_id: Option<Uuid>,
    ) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: Uuid::new_v4(),
            item_id,
            refinement,
            bonus_gem_slots,
            attributes,
            owner_character_id,
            owner_account_id,
            in_shared_storage: false,
            in_trade: false,
            created_at: now,
            updated_at: now,
        }
    }
}
