use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CharacterClass {
    Kael,
    Rin,
    Sirena
}