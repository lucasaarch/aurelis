use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CharacterLocation {
    Aurelis,
    Volcanis,
    Aquavale,
    Sylvandar
}