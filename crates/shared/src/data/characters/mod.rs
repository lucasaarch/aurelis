pub mod kael;

use crate::models::{character_data::CharacterData, skill_data::SkillData};

pub use kael::KAEL;

pub fn all_characters() -> Vec<&'static CharacterData> {
    vec![&KAEL]
}

pub fn find_character_by_slug(slug: &str) -> Option<&'static CharacterData> {
    all_characters()
        .into_iter()
        .find(|character| character.slug == slug)
}

pub fn all_skills_for_character(character_slug: &str) -> Vec<&'static SkillData> {
    match character_slug {
        "kael" => kael::skills::all_skills(),
        _ => vec![],
    }
}

pub fn is_valid_current_class_slug(character_slug: &str, current_class_slug: &str) -> bool {
    let Some(character) = find_character_by_slug(character_slug) else {
        return false;
    };

    if character.slug == current_class_slug {
        return true;
    }

    character
        .evolution_lines
        .iter()
        .flat_map(|path| path.steps.iter())
        .any(|class| class.slug == current_class_slug)
}
