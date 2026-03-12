use crate::{
    db::{Database, schema::accounts},
    models::account::AccountModel,
    repositories::{Repository, RepositoryError},
};
use chrono::{NaiveDateTime, Utc};
use diesel::pg::Pg;
use diesel::prelude::*;

#[derive(Debug, Clone)]
pub struct CreateAccountParams {
    pub email: String,
    pub password_hash: String,
}

#[derive(Clone, Default)]
pub struct ListAccountFilters {
    pub search: Option<String>,
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

    pub async fn find_by_email_with_hash(
        &self,
        email: &str,
    ) -> Result<AccountModel, RepositoryError> {
        let email = email.to_string();

        self.run_blocking(move |conn| {
            accounts::table
                .filter(accounts::email.eq(email))
                .first::<AccountModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<AccountModel, RepositoryError> {
        self.run_blocking(move |conn| {
            accounts::table
                .filter(accounts::id.eq(id))
                .first::<AccountModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn create(
        &self,
        params: CreateAccountParams,
    ) -> Result<AccountModel, RepositoryError> {
        let account = AccountModel::new(params.email, params.password_hash);

        self.run_blocking(move |conn| {
            diesel::insert_into(accounts::table)
                .values(&account)
                .get_result::<AccountModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn list(
        &self,
        page: i64,
        limit: i64,
        filters: ListAccountFilters,
    ) -> Result<(Vec<AccountModel>, i64), RepositoryError> {
        let offset = (page - 1) * limit;
        self.run_blocking(move |conn| {
            use crate::db::schema::accounts::dsl::*;

            let mut count_query = accounts.into_boxed::<Pg>();
            if let Some(ref value) = filters.search {
                let pattern = format!("%{}%", value);
                count_query = count_query.filter(email.ilike(pattern));
            }

            let total = count_query.count().get_result::<i64>(conn)?;

            let mut rows_query = accounts.into_boxed::<Pg>();
            if let Some(ref value) = filters.search {
                let pattern = format!("%{}%", value);
                rows_query = rows_query.filter(email.ilike(pattern));
            }

            let rows = rows_query
                .order((is_admin.desc(), created_at.desc()))
                .limit(limit)
                .offset(offset)
                .load::<AccountModel>(conn)?;

            Ok((rows, total))
        })
        .await
    }

    pub async fn ban(
        &self,
        account_id: uuid::Uuid,
        reason: String,
    ) -> Result<AccountModel, RepositoryError> {
        let now = Utc::now().naive_utc();
        self.run_blocking(move |conn| {
            diesel::update(accounts::table.filter(accounts::id.eq(account_id)))
                .set((
                    accounts::banned_at.eq(now),
                    accounts::banned_reason.eq(reason),
                    accounts::updated_at.eq(now),
                ))
                .get_result::<AccountModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn unban(&self, account_id: uuid::Uuid) -> Result<AccountModel, RepositoryError> {
        let now = Utc::now().naive_utc();
        self.run_blocking(move |conn| {
            diesel::update(accounts::table.filter(accounts::id.eq(account_id)))
                .set((
                    accounts::banned_at.eq(None::<NaiveDateTime>),
                    accounts::banned_reason.eq(None::<String>),
                    accounts::updated_at.eq(now),
                ))
                .get_result::<AccountModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn suspend(
        &self,
        account_id: uuid::Uuid,
        until: NaiveDateTime,
    ) -> Result<AccountModel, RepositoryError> {
        let now = Utc::now().naive_utc();
        self.run_blocking(move |conn| {
            diesel::update(accounts::table.filter(accounts::id.eq(account_id)))
                .set((
                    accounts::suspended_until.eq(until),
                    accounts::updated_at.eq(now),
                ))
                .get_result::<AccountModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn unsuspend(&self, account_id: uuid::Uuid) -> Result<AccountModel, RepositoryError> {
        let now = Utc::now().naive_utc();
        self.run_blocking(move |conn| {
            diesel::update(accounts::table.filter(accounts::id.eq(account_id)))
                .set((
                    accounts::suspended_until.eq(None::<NaiveDateTime>),
                    accounts::updated_at.eq(now),
                ))
                .get_result::<AccountModel>(conn)
                .map_err(Into::into)
        })
        .await
    }
}
