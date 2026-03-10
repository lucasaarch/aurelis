use crate::error::AppError;
use crate::repositories::account::PgAccountRepository;
use crate::repositories::mob_drop_rate::{CreateMobDropRateParams, PgMobDropRateRepository};
use bigdecimal::BigDecimal;
use shared::models::mob_drop_rate::MobDropRate;
use uuid::Uuid;

#[derive(Clone)]
pub struct MobDropRateService {
    repository: PgMobDropRateRepository,
    account_repository: PgAccountRepository,
}

impl MobDropRateService {
    pub fn new(
        repository: PgMobDropRateRepository,
        account_repository: PgAccountRepository,
    ) -> Self {
        Self {
            repository,
            account_repository,
        }
    }

    pub async fn create(
        &self,
        actor_id: Uuid,
        mob_id: Uuid,
        item_id: Uuid,
        drop_chance: BigDecimal,
    ) -> Result<MobDropRate, AppError> {
        let account = match self.account_repository.find_by_id(actor_id).await {
            Ok(a) => a,
            Err(_) => return Err(AppError::NotFound("Account not found".to_string())),
        };

        if !account.is_admin {
            return Err(AppError::PermissionDenied(
                "Only admins can create mob drop rates".to_string(),
            ));
        }

        self.repository
            .create(CreateMobDropRateParams {
                mob_id,
                item_id,
                drop_chance,
            })
            .await
            .map_err(Into::into)
    }
}
