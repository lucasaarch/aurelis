use std::collections::BTreeSet;

use tracing::info;

use crate::models::inventory_type::InventoryTypeModel;
use crate::repositories::RepositoryError;
use crate::repositories::character::{PgCharacterRepository, SyncedCharacterClassPath};
use crate::repositories::dungeon::PgDungeonRepository;
use crate::repositories::item::PgItemRepository;
use crate::repositories::mob::PgMobRepository;
use crate::repositories::quest::PgQuestRepository;
use shared::data::characters::{all_characters, is_valid_current_class_slug};
use shared::data::cities::{
    all_dungeons, all_items, all_mobs, find_dungeon_by_slug, find_item_by_slug, find_mob_by_slug,
};
use shared::data::quests::{all_quests, find_quest_by_slug};
use shared::models::inventory_type::InventoryType;
use shared::models::quest_data::{QuestActivation, QuestObjective, QuestTrigger};

pub async fn sync_items(repository: &PgItemRepository) -> Result<(), RepositoryError> {
    let items = all_items();

    for item in &items {
        repository
            .upsert_catalog_item(
                item.slug.to_string(),
                map_inventory_type(item.inventory_type),
            )
            .await?;
    }

    validate_catalog_slugs(
        "item",
        expected_item_slugs(),
        repository.list_catalog_slugs().await?,
    )?;

    info!("Synchronized {} items", items.len());

    Ok(())
}

pub async fn sync_dungeons(repository: &PgDungeonRepository) -> Result<(), RepositoryError> {
    let dungeons = all_dungeons();

    for dungeon in &dungeons {
        repository.upsert_catalog_dungeon(dungeon.slug).await?;
    }

    validate_catalog_slugs(
        "dungeon",
        expected_dungeon_slugs(),
        repository.list_catalog_slugs().await?,
    )?;

    info!("Synchronized {} dungeons", dungeons.len());

    Ok(())
}

pub async fn sync_mobs(repository: &PgMobRepository) -> Result<(), RepositoryError> {
    let mobs = all_mobs();

    for mob in &mobs {
        repository.upsert_catalog_mob(mob.slug).await?;
    }

    validate_catalog_slugs(
        "mob",
        expected_mob_slugs(),
        repository.list_catalog_slugs().await?,
    )?;

    info!("Synchronized {} mobs", mobs.len());

    Ok(())
}

pub async fn sync_characters(repository: &PgCharacterRepository) -> Result<(), RepositoryError> {
    let characters = all_characters();

    for character in &characters {
        repository.upsert_catalog_character(character.slug).await?;
    }

    validate_catalog_slugs(
        "character",
        expected_character_slugs(),
        repository.list_catalog_slugs().await?,
    )?;

    info!("Synchronized {} characters", characters.len());

    Ok(())
}

pub async fn sync_quests(repository: &PgQuestRepository) -> Result<(), RepositoryError> {
    let quests = all_quests();

    validate_quest_catalog(&quests)?;

    for quest in &quests {
        repository.upsert_catalog_quest(quest.slug).await?;
    }

    validate_catalog_slugs(
        "quest",
        expected_quest_slugs(),
        repository.list_catalog_slugs().await?,
    )?;

    info!("Synchronized {} quests", quests.len());

    Ok(())
}

pub async fn sync_character_class_paths(
    repository: &PgCharacterRepository,
) -> Result<Vec<SyncedCharacterClassPath>, RepositoryError> {
    let characters = all_characters();
    let synced_paths = repository
        .replace_catalog_character_class_paths(&characters)
        .await?;

    info!("Synchronized {} character class paths", synced_paths.len());

    Ok(synced_paths)
}

pub async fn sync_character_class_path_classes(
    repository: &PgCharacterRepository,
    synced_paths: Vec<SyncedCharacterClassPath>,
) -> Result<(), RepositoryError> {
    let characters = all_characters();
    repository
        .replace_catalog_character_class_path_classes(&characters, synced_paths)
        .await?;

    let total_classes = characters
        .iter()
        .flat_map(|character| character.evolution_lines.iter())
        .flat_map(|path| path.steps.iter())
        .count();

    info!(
        "Synchronized {} character class path classes",
        total_classes
    );

    Ok(())
}

pub async fn validate_player_character_integrity(
    repository: &PgCharacterRepository,
) -> Result<(), RepositoryError> {
    let rows = repository.list_player_character_integrity_rows().await?;

    for row in rows {
        if !is_valid_current_class_slug(&row.base_character_slug, &row.current_class_slug) {
            return Err(RepositoryError::Internal(format!(
                "Invalid current_class_slug '{}' for base character '{}'",
                row.current_class_slug, row.base_character_slug
            )));
        }
    }

    Ok(())
}

fn map_inventory_type(value: InventoryType) -> InventoryTypeModel {
    match value {
        InventoryType::Equipment => InventoryTypeModel::Equipment,
        InventoryType::Accessory => InventoryTypeModel::Accessory,
        InventoryType::Consumable => InventoryTypeModel::Consumable,
        InventoryType::Material => InventoryTypeModel::Material,
        InventoryType::QuestItem => InventoryTypeModel::QuestItem,
        InventoryType::Special => InventoryTypeModel::Special,
    }
}

fn validate_catalog_slugs(
    kind: &str,
    expected: BTreeSet<String>,
    actual: Vec<String>,
) -> Result<(), RepositoryError> {
    let actual = actual.into_iter().collect::<BTreeSet<_>>();

    if expected != actual {
        let missing = expected
            .difference(&actual)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        let unexpected = actual
            .difference(&expected)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");

        return Err(RepositoryError::Internal(format!(
            "Catalog {} sync mismatch. Missing: [{}]. Unexpected: [{}]",
            kind, missing, unexpected
        )));
    }

    Ok(())
}

fn expected_item_slugs() -> BTreeSet<String> {
    all_items()
        .into_iter()
        .map(|item| item.slug.to_string())
        .collect::<BTreeSet<_>>()
}

fn expected_mob_slugs() -> BTreeSet<String> {
    all_mobs()
        .into_iter()
        .map(|mob| mob.slug.to_string())
        .collect::<BTreeSet<_>>()
}

fn expected_dungeon_slugs() -> BTreeSet<String> {
    all_dungeons()
        .into_iter()
        .map(|dungeon| dungeon.slug.to_string())
        .collect::<BTreeSet<_>>()
}

fn expected_character_slugs() -> BTreeSet<String> {
    all_characters()
        .into_iter()
        .map(|character| character.slug.to_string())
        .collect::<BTreeSet<_>>()
}

fn expected_quest_slugs() -> BTreeSet<String> {
    all_quests()
        .into_iter()
        .map(|quest| quest.slug.to_string())
        .collect::<BTreeSet<_>>()
}

fn validate_quest_catalog(
    quests: &[&shared::models::quest_data::QuestData],
) -> Result<(), RepositoryError> {
    for quest in quests {
        match &quest.activation {
            QuestActivation::Manual => {}
            QuestActivation::Automatic(trigger) => match trigger {
                QuestTrigger::FirstTravelToCity { city_slug } => {
                    if city_slug.is_empty() {
                        return Err(RepositoryError::Internal(format!(
                            "Quest '{}' has an automatic city trigger with an empty city slug",
                            quest.slug
                        )));
                    }
                }
                QuestTrigger::CharacterLevelReached { level } => {
                    if *level <= 0 {
                        return Err(RepositoryError::Internal(format!(
                            "Quest '{}' has an invalid automatic level trigger '{}'",
                            quest.slug, level
                        )));
                    }
                }
                QuestTrigger::QuestCompleted { quest_slug } => {
                    if find_quest_by_slug(quest_slug).is_none() {
                        return Err(RepositoryError::Internal(format!(
                            "Quest '{}' references unknown trigger quest '{}'",
                            quest.slug, quest_slug
                        )));
                    }
                }
                QuestTrigger::DungeonCleared { dungeon_slug } => {
                    if find_dungeon_by_slug(dungeon_slug).is_none() {
                        return Err(RepositoryError::Internal(format!(
                            "Quest '{}' references unknown trigger dungeon '{}'",
                            quest.slug, dungeon_slug
                        )));
                    }
                }
            },
        }

        if quest.rewards.guaranteed_items.len() > 3 {
            return Err(RepositoryError::Internal(format!(
                "Quest '{}' has more than 3 guaranteed rewards",
                quest.slug
            )));
        }

        if quest.rewards.selectable_items.len() > 3 {
            return Err(RepositoryError::Internal(format!(
                "Quest '{}' has more than 3 selectable rewards",
                quest.slug
            )));
        }

        for reward in quest.rewards.guaranteed_items {
            if find_item_by_slug(reward.item_slug).is_none() {
                return Err(RepositoryError::Internal(format!(
                    "Quest '{}' references unknown guaranteed reward item '{}'",
                    quest.slug, reward.item_slug
                )));
            }
        }

        for reward in quest.rewards.selectable_items {
            if find_item_by_slug(reward.item_slug).is_none() {
                return Err(RepositoryError::Internal(format!(
                    "Quest '{}' references unknown selectable reward item '{}'",
                    quest.slug, reward.item_slug
                )));
            }
        }

        match &quest.objective {
            QuestObjective::KillMobs { targets } => {
                for target in *targets {
                    if find_mob_by_slug(target.mob_slug).is_none() {
                        return Err(RepositoryError::Internal(format!(
                            "Quest '{}' references unknown mob '{}'",
                            quest.slug, target.mob_slug
                        )));
                    }
                }
            }
            QuestObjective::ClearDungeons { targets } => {
                for target in *targets {
                    if find_dungeon_by_slug(target.dungeon_slug).is_none() {
                        return Err(RepositoryError::Internal(format!(
                            "Quest '{}' references unknown dungeon '{}'",
                            quest.slug, target.dungeon_slug
                        )));
                    }
                }
            }
            QuestObjective::CollectItems { targets } => {
                for target in *targets {
                    if find_item_by_slug(target.item_slug).is_none() {
                        return Err(RepositoryError::Internal(format!(
                            "Quest '{}' references unknown objective item '{}'",
                            quest.slug, target.item_slug
                        )));
                    }
                }
            }
            QuestObjective::TalkToNpc { .. } => {}
        }
    }

    Ok(())
}
