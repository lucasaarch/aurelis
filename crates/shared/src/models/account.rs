use chrono::NaiveDateTime;
use serde::{Deserialize,Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub is_banned: bool,
    pub banned_at: Option<NaiveDateTime>,
    pub banned_reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}