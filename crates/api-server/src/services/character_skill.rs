use shared::models::skill_data::CharacterSkillUnlockTier;
use uuid::Uuid;

use crate::{error::AppError, services::character::CharacterService};

#[derive(Clone)]
pub struct CharacterSkillService {
    character_service: CharacterService,
}

impl CharacterSkillService {
    pub fn new(character_service: CharacterService) -> Self {
        Self { character_service }
    }

    pub async fn unlock_tier(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        tier: CharacterSkillUnlockTier,
    ) -> Result<(), AppError> {
        self.character_service
            .unlock_skill_tier(account_id, character_id, tier)
            .await
    }
}
