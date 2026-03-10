use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemInstance {
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
