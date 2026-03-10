use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use serde_json::Value;
use shared::models::item_instance::ItemInstance;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::item_instances)]
pub struct ItemInstanceModel {
    pub id: Uuid,
    pub item_id: Uuid,
    pub refinement: i16,
    pub gem_slots: i16,
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
        gem_slots: i16,
        attributes: Value,
        owner_character_id: Option<Uuid>,
        owner_account_id: Option<Uuid>,
    ) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: Uuid::new_v4(),
            item_id,
            refinement,
            gem_slots,
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

impl From<ItemInstanceModel> for ItemInstance {
    fn from(model: ItemInstanceModel) -> Self {
        Self {
            id: model.id,
            item_id: model.item_id,
            refinement: model.refinement,
            gem_slots: model.gem_slots,
            attributes: model.attributes,
            owner_character_id: model.owner_character_id,
            owner_account_id: model.owner_account_id,
            in_shared_storage: model.in_shared_storage,
            in_trade: model.in_trade,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
