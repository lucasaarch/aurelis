use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterConsumableSlot {
    pub character_id: Uuid,
    pub slot: i16,
    pub item_instance_id: Option<Uuid>,
}
