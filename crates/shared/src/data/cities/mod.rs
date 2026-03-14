use crate::models::{dungeon_data::DungeonData, item_data::ItemData, mob_data::MobData};

pub mod aquavale;
pub mod aurelis;
pub mod global;
pub mod sylvandar;
pub mod volcanis;

pub fn all_items() -> Vec<&'static ItemData> {
    let mut items = Vec::new();
    items.extend_from_slice(global::items::ITEMS);
    items.extend_from_slice(aurelis::items::ITEMS);
    items.extend_from_slice(volcanis::items::ITEMS);
    items.extend_from_slice(aquavale::items::ITEMS);
    items.extend_from_slice(sylvandar::items::ITEMS);
    items
}

pub fn all_mobs() -> Vec<&'static MobData> {
    let mut mobs = Vec::new();
    mobs.extend_from_slice(global::mobs::MOBS);
    mobs.extend_from_slice(aurelis::mobs::MOBS);
    mobs.extend_from_slice(volcanis::mobs::MOBS);
    mobs.extend_from_slice(aquavale::mobs::MOBS);
    mobs.extend_from_slice(sylvandar::mobs::MOBS);
    mobs
}

pub fn all_dungeons() -> Vec<&'static DungeonData> {
    let mut dungeons = Vec::new();
    dungeons.extend_from_slice(global::dungeons::DUNGEONS);
    dungeons.extend_from_slice(aurelis::dungeons::DUNGEONS);
    dungeons.extend_from_slice(volcanis::dungeons::DUNGEONS);
    dungeons.extend_from_slice(aquavale::dungeons::DUNGEONS);
    dungeons.extend_from_slice(sylvandar::dungeons::DUNGEONS);
    dungeons
}

pub fn find_item_by_slug(slug: &str) -> Option<&'static ItemData> {
    all_items().into_iter().find(|item| item.slug == slug)
}

pub fn find_mob_by_slug(slug: &str) -> Option<&'static MobData> {
    all_mobs().into_iter().find(|mob| mob.slug == slug)
}

pub fn find_dungeon_by_slug(slug: &str) -> Option<&'static DungeonData> {
    all_dungeons()
        .into_iter()
        .find(|dungeon| dungeon.slug == slug)
}
