use crate::models::character_data::{CharacterData, CharacterProgress};

pub struct SkillData {
    pub slug: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub owner: SkillOwner,
    pub kind: SkillKind,
    pub level_req: i16,
    pub mp_cost: i32,
    pub cooldown_secs: f32,
    pub cast_time_secs: f32,
    pub range: f32,
}

pub enum SkillOwner {
    BaseCharacter { character_slug: &'static str },
    Class { class_slug: &'static str },
}

pub enum SkillKind {
    Passive,
    Active,
    SpecialActive,
}

impl SkillData {
    pub fn is_unlocked_for(
        &self,
        character: &CharacterData,
        progress: &CharacterProgress,
        character_level: i16,
    ) -> bool {
        if character_level < self.level_req {
            return false;
        }

        let Some(unlocked_slugs) = character.unlocked_class_slugs(progress) else {
            return false;
        };

        match self.owner {
            SkillOwner::BaseCharacter { character_slug } => {
                unlocked_slugs.contains(&character_slug)
            }
            SkillOwner::Class { class_slug } => unlocked_slugs.contains(&class_slug),
        }
    }
}
