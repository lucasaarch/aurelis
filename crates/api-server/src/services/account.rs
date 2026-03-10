use shared::models::account::Account;
use uuid::Uuid;

use crate::{error::AppError, repositories::account::PgAccountRepository};
#[derive(Clone)]
pub struct AccountService {
    account_repository: PgAccountRepository,
}

impl AccountService {
    pub fn new(account_repository: PgAccountRepository) -> Self {
        Self { account_repository }
    }

    pub async fn find_by_id(&self, account_id: Uuid) -> Result<Account, AppError> {
        self.account_repository
            .find_by_id(account_id)
            .await
            .map_err(Into::into)
    }
}
