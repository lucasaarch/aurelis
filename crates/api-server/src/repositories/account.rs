use crate::{
    db::{Database, schema::accounts},
    models::account::AccountModel,
    repositories::{Repository, RepositoryError},
};
use diesel::prelude::*;
use shared::models::account::Account;

#[derive(Debug, Clone)]
pub struct CreateAccountParams {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Clone)]
pub struct PgAccountRepository {
    db: Database,
}

impl Repository for PgAccountRepository {
    fn db(&self) -> Database {
        self.db.clone()
    }
}

impl PgAccountRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Account, RepositoryError> {
        let email = email.to_string();

        self.run_blocking(move |conn| {
            accounts::table
                .filter(accounts::email.eq(email))
                .first::<AccountModel>(conn)
                .map(|ac| ac.into())
                .map_err(Into::into)
        })
        .await
    }

    pub async fn find_by_email_with_hash(
        &self,
        email: &str,
    ) -> Result<(Account, String), RepositoryError> {
        let email = email.to_string();

        self.run_blocking(move |conn| {
            accounts::table
                .filter(accounts::email.eq(email))
                .first::<AccountModel>(conn)
                .map(|ac| {
                    let hash = ac.password_hash.clone();
                    (ac.into(), hash)
                })
                .map_err(Into::into)
        })
        .await
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Account, RepositoryError> {
        let username = username.to_string();

        self.run_blocking(move |conn| {
            accounts::table
                .filter(accounts::username.eq(username))
                .first::<AccountModel>(conn)
                .map(|ac| ac.into())
                .map_err(Into::into)
        })
        .await
    }

    pub async fn create(&self, params: CreateAccountParams) -> Result<Account, RepositoryError> {
        let account = AccountModel::new(params.username, params.email, params.password_hash);

        self.run_blocking(move |conn| {
            diesel::insert_into(accounts::table)
                .values(&account)
                .get_result(conn)
                .map(|ac: AccountModel| ac.into())
                .map_err(Into::into)
        })
        .await
    }
}
