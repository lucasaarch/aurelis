use shared::models::character::Character;
use uuid::Uuid;

use crate::error::AppError;
use crate::repositories::account::PgAccountRepository;
use crate::repositories::character::{CreateCharacterParams, PgCharacterRepository};

pub struct CreateCharacterInput {
    pub name: String,
    pub class: String,
}

#[derive(Clone)]
pub struct CharacterService {
    character_repository: PgCharacterRepository,
    account_repository: PgAccountRepository,
}

impl CharacterService {
    pub fn new(
        character_repository: PgCharacterRepository,
        account_repository: PgAccountRepository,
    ) -> Self {
        Self {
            character_repository,
            account_repository,
        }
    }

    pub async fn create(
        &self,
        account_id: Uuid,
        input: CreateCharacterInput,
    ) -> Result<Character, AppError> {
        let account = match self.account_repository.find_by_id(account_id).await {
            Ok(a) => a,
            Err(_) => {
                return Err(AppError::Unauthorized(
                    "Unable to fetch account data".to_string(),
                ));
            }
        };

        let max = account.max_characters as i64;
        let current = match self.character_repository.count_by_account(account_id).await {
            Ok(c) => c,
            Err(_) => {
                return Err(AppError::Internal(anyhow::anyhow!(
                    "Unknown Database Error"
                )));
            }
        };

        if current >= max {
            return Err(AppError::BadRequest(
                "Character limit reached for this account".to_string(),
            ));
        }

        self.character_repository
            .create(
                account_id,
                CreateCharacterParams {
                    name: input.name,
                    class: input.class,
                },
            )
            .await
            .map_err(Into::into)
    }

    pub async fn list_all(&self, account_id: Uuid) -> Result<Vec<Character>, AppError> {
        match self
            .character_repository
            .list_all_by_account(account_id)
            .await
        {
            Ok(v) => Ok(v),
            Err(_) => Err(AppError::Internal(anyhow::anyhow!("DB error"))),
        }
    }

    pub async fn verify_ownership(
        &self,
        account_id: Uuid,
        character_id: Uuid,
    ) -> Result<(), AppError> {
        let character = match self.character_repository.find_by_id(character_id).await {
            Ok(c) => c,
            Err(_) => {
                return Err(AppError::Unauthorized(
                    "Unable to fech character data".to_string(),
                ));
            }
        };

        if character.account_id != account_id {
            return Err(AppError::PermissionDenied(
                "Character does not belong to this account".to_string(),
            ));
        }

        Ok(())
    }
}
