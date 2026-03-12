use crate::models::account::AccountModel;
use crate::models::suspension_severity::SuspensionSeverity;
use uuid::Uuid;

use crate::{
    error::AppError,
    repositories::account::{ListAccountFilters, PgAccountRepository},
};
#[derive(Clone)]
pub struct AccountService {
    account_repository: PgAccountRepository,
}

impl AccountService {
    pub fn new(account_repository: PgAccountRepository) -> Self {
        Self { account_repository }
    }

    pub async fn find_by_id(&self, account_id: Uuid) -> Result<AccountModel, AppError> {
        self.account_repository
            .find_by_id(account_id)
            .await
            .map_err(Into::into)
    }

    pub async fn list(
        &self,
        actor_id: Uuid,
        page: i64,
        limit: i64,
        filters: ListAccountFilters,
    ) -> Result<(Vec<AccountModel>, i64), AppError> {
        self.ensure_admin(actor_id).await?;

        self.account_repository
            .list(page, limit, filters)
            .await
            .map_err(Into::into)
    }

    pub async fn ban(
        &self,
        actor_id: Uuid,
        target_id: Uuid,
        reason: String,
    ) -> Result<AccountModel, AppError> {
        self.ensure_admin(actor_id).await?;
        self.account_repository
            .ban(target_id, reason)
            .await
            .map_err(Into::into)
    }

    pub async fn unban(&self, actor_id: Uuid, target_id: Uuid) -> Result<AccountModel, AppError> {
        self.ensure_admin(actor_id).await?;
        self.account_repository
            .unban(target_id)
            .await
            .map_err(Into::into)
    }

    pub async fn suspend(
        &self,
        actor_id: Uuid,
        target_id: Uuid,
        until: chrono::NaiveDateTime,
    ) -> Result<AccountModel, AppError> {
        self.ensure_admin(actor_id).await?;
        self.account_repository
            .suspend(target_id, until)
            .await
            .map_err(Into::into)
    }

    pub async fn unsuspend(
        &self,
        actor_id: Uuid,
        target_id: Uuid,
    ) -> Result<AccountModel, AppError> {
        self.ensure_admin(actor_id).await?;
        self.account_repository
            .unsuspend(target_id)
            .await
            .map_err(Into::into)
    }

    pub async fn apply_punishment(
        &self,
        actor_id: Uuid,
        target_id: Uuid,
        punishment_type: String,
        reason: Option<String>,
        severity: Option<SuspensionSeverity>,
    ) -> Result<AccountModel, AppError> {
        match punishment_type.as_str() {
            "ban" => {
                let reason = reason.unwrap_or_else(|| "No reason provided".to_string());
                self.ban(actor_id, target_id, reason).await
            }
            "unban" => self.unban(actor_id, target_id).await,
            "suspend" => {
                let severity = severity.unwrap_or(SuspensionSeverity::Medium);
                let until: chrono::NaiveDateTime = severity.into();
                self.suspend(actor_id, target_id, until).await
            }
            "unsuspend" => self.unsuspend(actor_id, target_id).await,
            _ => Err(AppError::BadRequest("Invalid punishment type".to_string())),
        }
    }

    async fn ensure_admin(&self, actor_id: Uuid) -> Result<(), AppError> {
        let account = match self.account_repository.find_by_id(actor_id).await {
            Ok(a) => a,
            Err(_) => {
                return Err(AppError::Unauthorized(
                    "Unable to fetch account data".to_string(),
                ));
            }
        };

        if !account.is_admin {
            return Err(AppError::PermissionDenied(
                "Only admins can access this resource".to_string(),
            ));
        }

        Ok(())
    }
}
