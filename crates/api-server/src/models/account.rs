use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use shared::models::account::Account;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::accounts)]
pub struct AccountModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_banned: bool,
    pub banned_at: Option<NaiveDateTime>,
    pub banned_reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl AccountModel {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            is_banned: false,
            banned_at: None,
            banned_reason: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn ban(&mut self, reason: String) {
        self.is_banned = true;
        self.banned_at = Some(Utc::now().naive_utc());
        self.banned_reason = Some(reason);
        self.updated_at = Utc::now().naive_utc();
    }

    pub fn unban(&mut self) {
        self.is_banned = false;
        self.banned_at = None;
        self.banned_reason = None;
        self.updated_at = Utc::now().naive_utc();
    }

    pub fn update_email(&mut self, new_email: String) {
        self.email = new_email;
        self.updated_at = Utc::now().naive_utc();
    }

    pub fn update_password(&mut self, new_password_hash: String) {
        self.password_hash = new_password_hash;
        self.updated_at = Utc::now().naive_utc();
    }
}

impl From<AccountModel> for Account {
    fn from(model: AccountModel) -> Self {
        Account {
            id: model.id,
            username: model.username,
            email: model.email,
            is_banned: model.is_banned,
            banned_at: model.banned_at,
            banned_reason: model.banned_reason,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
