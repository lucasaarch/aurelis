use chrono::NaiveDateTime;
use serde::{Deserialize,Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub max_characters: i16,
    pub shared_storage_enabled: bool,
    pub shared_storage_capacity: i16,
    pub cash: i64,
    pub stored_credits: i64,
    pub is_admin: bool,
    pub god_mode: bool,
    pub email_verified: bool,
    pub banned_at: Option<NaiveDateTime>,
    pub banned_reason: Option<String>,
    pub suspended_until: Option<NaiveDateTime>,
    pub chat_restricted_until: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}