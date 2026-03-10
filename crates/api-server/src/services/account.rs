use crate::repositories::account::PgAccountRepository;
#[derive(Clone)]
pub struct AccountService {
    account_repository: PgAccountRepository,
}

impl AccountService {
    pub fn new(account_repository: PgAccountRepository) -> Self {
        Self { account_repository }
    }
}
