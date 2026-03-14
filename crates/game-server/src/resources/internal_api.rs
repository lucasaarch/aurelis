use bevy::prelude::Resource;
use tokio::runtime::Runtime;
use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;
use tonic::transport::Endpoint;
use tonic::{Request, Status};
use uuid::Uuid;

use shared::proto::internal_game::{
    LoadPlayableCharacterRequest, LoadPlayableCharacterResponse,
    internal_game_service_client::InternalGameServiceClient,
};

#[derive(Debug, Resource)]
pub struct InternalApi {
    grpc_addr: String,
    bearer_token: String,
    runtime: Runtime,
}

#[derive(Debug, Clone)]
pub struct PlayableCharacterSnapshot {
    pub character_id: Uuid,
    pub name: String,
    pub base_character_slug: String,
    pub current_class_slug: String,
    pub level: i16,
}

impl InternalApi {
    pub fn new(grpc_addr: String, bearer_token: String) -> Self {
        Self {
            grpc_addr,
            bearer_token,
            runtime: Runtime::new().expect("failed to create tokio runtime"),
        }
    }

    pub fn load_playable_character(
        &self,
        account_id: Uuid,
        character_id: Uuid,
    ) -> Result<PlayableCharacterSnapshot, String> {
        self.runtime.block_on(async {
            let endpoint =
                Endpoint::from_shared(self.grpc_addr.clone()).map_err(|err| err.to_string())?;
            let channel = endpoint.connect().await.map_err(|err| err.to_string())?;
            let interceptor = InternalServerAuthInterceptor::new(self.bearer_token.clone());
            let mut client = InternalGameServiceClient::with_interceptor(channel, interceptor);

            let response = client
                .load_playable_character(Request::new(LoadPlayableCharacterRequest {
                    account_id: account_id.to_string(),
                    character_id: character_id.to_string(),
                }))
                .await
                .map_err(|err| err.to_string())?
                .into_inner();

            map_snapshot(response)
        })
    }
}

fn map_snapshot(
    response: LoadPlayableCharacterResponse,
) -> Result<PlayableCharacterSnapshot, String> {
    Ok(PlayableCharacterSnapshot {
        character_id: Uuid::parse_str(&response.character_id).map_err(|err| err.to_string())?,
        name: response.name,
        base_character_slug: response.base_character_slug,
        current_class_slug: response.current_class_slug,
        level: response.level as i16,
    })
}

#[derive(Clone)]
struct InternalServerAuthInterceptor {
    bearer_header: MetadataValue<tonic::metadata::Ascii>,
}

impl InternalServerAuthInterceptor {
    fn new(token: String) -> Self {
        let bearer = format!("Bearer {token}")
            .parse()
            .expect("invalid internal bearer token");
        Self {
            bearer_header: bearer,
        }
    }
}

impl Interceptor for InternalServerAuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request
            .metadata_mut()
            .insert("authorization", self.bearer_header.clone());
        Ok(request)
    }
}
