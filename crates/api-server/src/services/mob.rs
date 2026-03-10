use crate::error::AppError;
use crate::repositories::account::PgAccountRepository;
use crate::repositories::mob::{CreateMobParams, PgMobRepository};
use shared::models::mob::Mob;
use uuid::Uuid;

pub struct CreateMobInput {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub mob_type: String,
}

#[derive(Clone)]
pub struct MobService {
    repository: PgMobRepository,
    account_repository: PgAccountRepository,
}

impl MobService {
    pub fn new(repository: PgMobRepository, account_repository: PgAccountRepository) -> Self {
        Self {
            repository,
            account_repository,
        }
    }

    pub async fn create(&self, actor_id: Uuid, input: CreateMobInput) -> Result<Mob, AppError> {
        let account = match self.account_repository.find_by_id(actor_id).await {
            Ok(a) => a,
            Err(_) => return Err(AppError::Unauthorized),
        };

        if !account.is_admin {
            return Err(AppError::Unauthorized);
        }

        self.repository
            .create(CreateMobParams {
                slug: input.slug,
                name: input.name,
                description: input.description,
                mob_type: input.mob_type,
            })
            .await
            .map_err(Into::into)
    }
}
