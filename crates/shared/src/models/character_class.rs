use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CharacterClass {
    Kael,
    Rin,
    Sirena
}