use serde_json::Value;
use uuid::Uuid;

use crate::{
    error::AppError, repositories::item_instance::PgItemInstanceRepository,
    services::character::CharacterService,
};

#[derive(Clone)]
pub struct ItemInstanceService {
    item_instance_repository: PgItemInstanceRepository,
    character_service: CharacterService,
}

impl ItemInstanceService {
    pub fn new(
        item_instance_repository: PgItemInstanceRepository,
        character_service: CharacterService,
    ) -> Self {
        Self {
            item_instance_repository,
            character_service,
        }
    }

    pub async fn persist_state(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        item_instance_id: Uuid,
        bonus_gem_slots: i16,
        attributes_json: String,
    ) -> Result<(), AppError> {
        self.character_service
            .verify_ownership(account_id, character_id)
            .await?;

        let item_instance = self
            .item_instance_repository
            .find_by_id(item_instance_id)
            .await?;
        if item_instance.owner_character_id != Some(character_id) {
            return Err(AppError::PermissionDenied(
                "Item instance does not belong to this character".to_string(),
            ));
        }

        let attributes: Value = serde_json::from_str(&attributes_json)
            .map_err(|_| AppError::BadRequest("Invalid item instance attributes".to_string()))?;

        self.item_instance_repository
            .update_state(item_instance_id, bonus_gem_slots, attributes)
            .await?;

        Ok(())
    }

    pub async fn update_refinement(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        item_instance_id: Uuid,
        refinement: i16,
    ) -> Result<(), AppError> {
        self.character_service
            .verify_ownership(account_id, character_id)
            .await?;

        let item_instance = self
            .item_instance_repository
            .find_by_id(item_instance_id)
            .await?;
        if item_instance.owner_character_id != Some(character_id) {
            return Err(AppError::PermissionDenied(
                "Item instance does not belong to this character".to_string(),
            ));
        }

        let equipment = self
            .character_service
            .load_playable_character(account_id, character_id)
            .await?
            .equipment;
        let is_equipped = equipment
            .iter()
            .any(|entry| entry.item_instance_id == item_instance_id);
        if !is_equipped {
            return Err(AppError::BadRequest(
                "Only equipped items can be refined".to_string(),
            ));
        }

        self.item_instance_repository
            .update_refinement(item_instance_id, refinement.clamp(0, 7))
            .await?;

        Ok(())
    }
}
