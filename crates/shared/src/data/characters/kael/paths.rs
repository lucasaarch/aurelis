use crate::models::character_data::ClassPathData;

use super::classes::{KAEL_ROYAL_SENTINEL, KAEL_SOVEREIGN_BLADE};

pub static KAEL_ROYAL_BLADE: ClassPathData = ClassPathData {
    steps: &[&KAEL_ROYAL_SENTINEL, &KAEL_SOVEREIGN_BLADE],
};
