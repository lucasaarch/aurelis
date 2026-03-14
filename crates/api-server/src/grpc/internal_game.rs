use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::services::character::CharacterService;

use shared::proto::internal_game::{
    LoadPlayableCharacterRequest, LoadPlayableCharacterResponse,
    internal_game_service_server::InternalGameService,
};

pub struct GrpcInternalGameServiceImpl {
    character_service: CharacterService,
}

impl GrpcInternalGameServiceImpl {
    pub fn new(character_service: CharacterService) -> Self {
        Self { character_service }
    }
}

#[tonic::async_trait]
impl InternalGameService for GrpcInternalGameServiceImpl {
    async fn load_playable_character(
        &self,
        request: Request<LoadPlayableCharacterRequest>,
    ) -> Result<Response<LoadPlayableCharacterResponse>, Status> {
        let req = request.into_inner();
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|_| Status::invalid_argument("invalid account_id"))?;
        let character_id = Uuid::parse_str(&req.character_id)
            .map_err(|_| Status::invalid_argument("invalid character_id"))?;

        let result = self
            .character_service
            .load_playable_character(account_id, character_id)
            .await?;

        Ok(Response::new(LoadPlayableCharacterResponse {
            character_id: result.character_id.to_string(),
            name: result.name,
            base_character_slug: result.base_character_slug,
            current_class_slug: result.current_class_slug,
            level: result.level as i32,
        }))
    }
}
