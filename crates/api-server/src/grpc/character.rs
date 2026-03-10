use tonic::{Request, Response, Status};

use crate::services::auth::AuthService;
use crate::services::character::{CharacterService, CreateCharacterInput};
use crate::services::jwt::TokenContext;
use crate::utils::extractors::extract_access_token_from_metadata;
use shared::proto::character::{
    Character, CreateCharacterRequest, CreateCharacterResponse, ListCharactersRequest,
    ListCharactersResponse, character_service_server::CharacterService as GrpcCharacterService,
};
use shared::utils::datetime::format_naive_datetime;

pub struct GrpcCharacterServiceImpl {
    auth_service: AuthService,
    character_service: CharacterService,
}

impl GrpcCharacterServiceImpl {
    pub fn new(auth_service: AuthService, character_service: CharacterService) -> Self {
        Self {
            auth_service,
            character_service,
        }
    }
}

#[tonic::async_trait]
impl GrpcCharacterService for GrpcCharacterServiceImpl {
    async fn list_characters(
        &self,
        request: Request<ListCharactersRequest>,
    ) -> Result<Response<ListCharactersResponse>, Status> {
        let token = extract_access_token_from_metadata(request.metadata())?;
        let account = self
            .auth_service
            .authenticate(&token, TokenContext::Game)
            .await?;

        let characters = self.character_service.list_all(account.id).await?;

        let response = ListCharactersResponse {
            characters: characters
                .into_iter()
                .map(|c| Character {
                    id: c.id.into(),
                    name: c.name,
                    level: c.level.into(),
                    class: c.class.to_string(),
                    created_at: format_naive_datetime(&c.created_at),
                    updated_at: format_naive_datetime(&c.updated_at),
                })
                .collect(),
        };

        Ok(Response::new(response))
    }

    async fn create_character(
        &self,
        request: Request<CreateCharacterRequest>,
    ) -> Result<Response<CreateCharacterResponse>, Status> {
        let token = extract_access_token_from_metadata(request.metadata())?;
        let req = request.into_inner();

        let account = self
            .auth_service
            .authenticate(&token, TokenContext::Game)
            .await?;

        let character = self
            .character_service
            .create(
                account.id,
                CreateCharacterInput {
                    class: req.class,
                    name: req.name,
                },
            )
            .await
            .map(|c| Character {
                id: c.id.into(),
                name: c.name,
                level: c.level.into(),
                class: c.class.to_string(),
                created_at: format_naive_datetime(&c.created_at),
                updated_at: format_naive_datetime(&c.updated_at),
            })?;

        Ok(Response::new(CreateCharacterResponse {
            character: Some(character),
        }))
    }
}
