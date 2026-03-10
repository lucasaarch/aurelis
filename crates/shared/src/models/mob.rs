use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::mob_type::MobType;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mob {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub mob_type: MobType,
    pub created_at: NaiveDateTime,
}
