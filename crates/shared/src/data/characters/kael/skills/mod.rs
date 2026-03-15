pub mod base;
pub mod kael_royal_sentinel;
pub mod kael_sovereign_blade;

pub use base::*;
pub use kael_royal_sentinel::*;
pub use kael_sovereign_blade::*;

use crate::models::skill_data::SkillData;

pub fn all_skills() -> Vec<&'static SkillData> {
    vec![
        &KAEL_SLASH,
        &KAEL_GUARDING_STRIKE,
        &SENTINEL_STEEL_PULSE,
        &SENTINEL_SIGNATURE_DECREE,
        &SOVEREIGN_BREAKER,
        &ASCENDANT_KINGSFALL,
    ]
}
