use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemInstanceGem {
    pub id: Uuid,
    pub item_instance_id: Uuid,
    pub slot_index: i16,
    pub gem_instance_id: Uuid,
    pub socketed_at: NaiveDateTime,
}
