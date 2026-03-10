use diesel::PgConnection;
use thiserror::Error;

use crate::db::Database;

pub mod account;
pub mod character;
pub mod mob;
pub mod item;
pub mod inventory;
pub mod mob_drop_rate;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Resource not found")]
    NotFound,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<diesel::result::Error> for RepositoryError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => RepositoryError::NotFound,
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                info,
            ) => {
                let field = info
                    .constraint_name()
                    .and_then(|c| c.split('_').nth(1))
                    .unwrap_or("field");
                RepositoryError::Conflict(format!("{field} already in use"))
            }
            diesel::result::Error::DatabaseError(_, info) => {
                RepositoryError::Database(info.message().to_string())
            }
            _ => RepositoryError::Internal(err.to_string()),
        }
    }
}

pub trait Repository {
    fn db(&self) -> Database;

    async fn run_blocking<F, T>(&self, f: F) -> Result<T, RepositoryError>
    where
        F: FnOnce(&mut PgConnection) -> Result<T, RepositoryError> + Send + 'static,
        T: Send + 'static,
    {
        let db = self.db();
        tokio::task::spawn_blocking(move || {
            let mut conn = db.get();
            f(&mut conn)
        })
        .await
        .map_err(|e| RepositoryError::Internal(format!("Task join error: {}", e)))?
    }
}
