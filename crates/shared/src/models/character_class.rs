use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CharacterClass {
    Kael,
    Rin,
    Sirena,
}

impl CharacterClass {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "kael" => Some(CharacterClass::Kael),
            "rin" => Some(CharacterClass::Rin),
            "sirena" => Some(CharacterClass::Sirena),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CharacterClass::Kael => "kael".to_string(),
            CharacterClass::Rin => "rin".to_string(),
            CharacterClass::Sirena => "sirena".to_string(),
        }
    }
}
