use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::character_location::CharacterLocation;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quest {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub city: Option<CharacterLocation>,
    pub level_req: i16,
    pub created_at: NaiveDateTime,
}
