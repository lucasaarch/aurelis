use crate::error::AppError;
use crate::repositories::account::PgAccountRepository;
use crate::repositories::mob::{CreateMobParams, PgMobRepository};
use shared::models::mob::Mob;
use shared::utils::slug::generate_slug;
use uuid::Uuid;

pub struct CreateMobInput {
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

        let slug = generate_slug(&input.name);

        self.repository
            .create(CreateMobParams {
                slug,
                name: input.name,
                description: input.description,
                mob_type: input.mob_type,
            })
            .await
            .map_err(Into::into)
    }
}
