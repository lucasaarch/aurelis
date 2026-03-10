use chrono::{NaiveDateTime, Utc};
use diesel::prelude::{Insertable, Queryable};
use shared::models::account::Account;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::accounts)]
pub struct AccountModel {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub max_characters: i16,
    pub shared_storage_enabled: bool,
    pub shared_storage_capacity: i16,
    pub cash: i64,
    pub stored_credits: i64,
    pub is_admin: bool,
    pub god_mode: bool,
    pub email_verified: bool,
    pub email_verified_at: Option<NaiveDateTime>,
    pub email_verify_token: Option<String>,
    pub email_verify_token_expires: Option<NaiveDateTime>,
    pub password_reset_token: Option<String>,
    pub password_reset_expires: Option<NaiveDateTime>,
    pub banned_at: Option<NaiveDateTime>,
    pub banned_reason: Option<String>,
    pub suspended_until: Option<NaiveDateTime>,
    pub chat_restricted_until: Option<NaiveDateTime>,
    pub last_login_at: Option<NaiveDateTime>,
    pub last_login_ip: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl AccountModel {
    pub fn new(email: String, password_hash: String) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            max_characters: 3,
            shared_storage_enabled: false,
            shared_storage_capacity: 20,
            cash: 0,
            stored_credits: 0,
            is_admin: false,
            god_mode: false,
            email_verified: false,
            email_verified_at: None,
            email_verify_token: None,
            email_verify_token_expires: None,
            password_reset_token: None,
            password_reset_expires: None,
            banned_at: None,
            banned_reason: None,
            suspended_until: None,
            chat_restricted_until: None,
            last_login_at: None,
            last_login_ip: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    pub fn ban(&mut self, reason: String) {
        self.banned_at = Some(Utc::now().naive_utc());
        self.banned_reason = Some(reason);
        self.updated_at = Utc::now().naive_utc();
    }

    pub fn unban(&mut self) {
        self.banned_at = None;
        self.banned_reason = None;
        self.updated_at = Utc::now().naive_utc();
    }

    pub fn suspend(&mut self, until: NaiveDateTime) {
        self.suspended_until = Some(until);
        self.updated_at = Utc::now().naive_utc();
    }

    pub fn unsuspend(&mut self) {
        self.suspended_until = None;
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

    pub fn verify_email(&mut self) {
        self.email_verified = true;
        self.updated_at = Utc::now().naive_utc();
    }
}

impl From<AccountModel> for Account {
    fn from(model: AccountModel) -> Self {
        Account {
            id: model.id,
            email: model.email,
            max_characters: model.max_characters,
            shared_storage_enabled: model.shared_storage_enabled,
            shared_storage_capacity: model.shared_storage_capacity,
            cash: model.cash,
            stored_credits: model.stored_credits,
            is_admin: model.is_admin,
            god_mode: model.god_mode,
            email_verified: model.email_verified,
            email_verified_at: model.email_verified_at,
            banned_at: model.banned_at,
            banned_reason: model.banned_reason,
            suspended_until: model.suspended_until,
            chat_restricted_until: model.chat_restricted_until,
            created_at: model.created_at,
            updated_at: model.updated_at,
            deleted_at: model.deleted_at,
        }
    }
}
