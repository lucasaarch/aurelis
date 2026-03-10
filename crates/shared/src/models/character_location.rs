use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CharacterLocation {
    Aurelis,
    Volcanis,
    Aquavale,
    Sylvandar
}