use shared::{
    data::characters::find_skill_by_slug,
    models::skill_data::{SkillKind, SkillOwner},
};

use crate::runtime::{character::RuntimeCharacter, skill_effects::build_timed_skill_modifier};

pub fn use_skill(runtime_character: &mut RuntimeCharacter, skill_slug: &str) -> Result<(), String> {
    if !runtime_character
        .available_skill_slugs
        .iter()
        .any(|available| available == skill_slug)
    {
        return Err(format!("skill '{}' is not available", skill_slug));
    }

    let skill = find_skill_by_slug(&runtime_character.base_character_slug, skill_slug)
        .ok_or_else(|| format!("unknown skill '{}'", skill_slug))?;

    if runtime_character.is_skill_on_cooldown(skill_slug) {
        return Err(format!("skill '{}' is on cooldown", skill_slug));
    }

    match skill.owner {
        SkillOwner::BaseCharacter { .. } => {}
        SkillOwner::Class { class_slug } => {
            if class_slug != runtime_character.current_class_slug {
                return Err(format!(
                    "skill '{}' does not belong to current class '{}'",
                    skill_slug, runtime_character.current_class_slug
                ));
            }
        }
    }

    runtime_character.spend_mp(skill.mp_cost())?;

    match skill.kind {
        SkillKind::Advantage => {
            let modifier =
                build_timed_skill_modifier(&runtime_character.base_character_slug, skill_slug)?;
            runtime_character.add_timed_modifier(modifier);
            runtime_character
                .set_skill_cooldown(skill_slug, (skill.cooldown_secs * 1000.0).round() as u64);
            Ok(())
        }
        _ => Err(format!("skill '{}' is not usable yet", skill_slug)),
    }
}
