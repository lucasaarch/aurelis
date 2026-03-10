use shared::models::character::Character;
use uuid::Uuid;

use crate::error::AppError;
use crate::repositories::character::{CreateCharacterParams, PgCharacterRepository};

pub struct CreateCharacterInput {
    pub name: String,
    pub class: String,
}

#[derive(Clone)]
pub struct CharacterService {
    repository: PgCharacterRepository,
}

impl CharacterService {
    pub fn new(repository: PgCharacterRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, account_id: Uuid, input: CreateCharacterInput) -> Result<Character, AppError> {
        self.repository
            .create(account_id, CreateCharacterParams {
                name: input.name,
                class: input.class,
            })
            .await
            .map_err(Into::into)
    }
}
