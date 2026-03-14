use crate::models::quest_data::QuestData;

pub mod character;
pub mod event;
pub mod story;

pub fn all_quests() -> Vec<&'static QuestData> {
    let mut quests = Vec::new();
    quests.extend_from_slice(story::QUESTS);
    quests.extend_from_slice(character::QUESTS);
    quests.extend_from_slice(event::QUESTS);
    quests
}

pub fn find_quest_by_slug(slug: &str) -> Option<&'static QuestData> {
    all_quests().into_iter().find(|quest| quest.slug == slug)
}
