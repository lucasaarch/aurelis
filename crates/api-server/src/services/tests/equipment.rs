use crate::{
    db::{Database, MIGRATIONS, schema},
    error::AppError,
    models::{
        account::AccountModel, equipment_slot::EquipmentSlotModel,
        inventory_item::InventoryItemModel, inventory_type::InventoryTypeModel, item::ItemModel,
        item_instance::ItemInstanceModel,
    },
    repositories::{
        Repository,
        account::{CreateAccountParams, PgAccountRepository},
        character::{CreateCharacterParams, PgCharacterRepository, PlayableCharacterRow},
        inventory::PgInventoryRepository,
        item::PgItemRepository,
        item_instance::PgItemInstanceRepository,
    },
    services::equipment::{EquipmentService, validate_catalog_equip_requirements},
};
use diesel::prelude::*;
use diesel::{PgConnection, r2d2::ConnectionManager};
use diesel_migrations::MigrationHarness;
use serde_json::json;
use shared::data::cities::find_item_by_slug;
use std::{env, sync::OnceLock};
use uuid::Uuid;

fn playable_character(
    level: i16,
    base_slug: &str,
    current_class_slug: &str,
) -> PlayableCharacterRow {
    PlayableCharacterRow {
        character_id: Uuid::nil(),
        account_id: Uuid::nil(),
        name: "Kaelzinho".to_string(),
        base_character_slug: base_slug.to_string(),
        current_class_slug: current_class_slug.to_string(),
        level,
        experience: 0,
        credits: 0,
        beginner_skill_unlocked: false,
        intermediate_skill_unlocked: false,
    }
}

#[test]
fn validates_equippable_item_requirements_successfully() {
    let item = find_item_by_slug("kael_training_blade").expect("catalog item");
    let character = playable_character(20, "kael", "kael_royal_sentinel");

    let slot = validate_catalog_equip_requirements(item, &character).expect("should equip");

    assert!(matches!(slot, EquipmentSlotModel::Weapon));
}

#[test]
fn rejects_item_when_character_level_is_too_low() {
    let item = find_item_by_slug("kael_training_blade").expect("catalog item");
    let character = playable_character(0, "kael", "kael_royal_sentinel");

    let err = validate_catalog_equip_requirements(item, &character).expect_err("should fail");

    match err {
        AppError::BadRequest(message) => {
            assert!(message.contains("does not meet item requirement"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn rejects_item_when_character_restriction_does_not_match() {
    let item = find_item_by_slug("kael_training_blade").expect("catalog item");
    let character = playable_character(20, "lyra", "lyra_arc_scholar");

    let err = validate_catalog_equip_requirements(item, &character).expect_err("should fail");

    match err {
        AppError::BadRequest(message) => {
            assert!(message.contains("restricted"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn equips_item_instance_in_real_database() {
    let Some(fixture) = TestFixture::new().await else {
        eprintln!("skipping db integration test: API_DATABASE_URL is unavailable");
        return;
    };
    let weapon_instance = fixture
        .insert_item_instance("kael_training_blade", InventoryTypeModel::Equipment)
        .await;
    fixture
        .insert_inventory_instance(weapon_instance.id, 0)
        .await;

    fixture
        .service
        .equip_inventory_item(fixture.player_character.id, "equipment".to_string(), 0)
        .await
        .expect("equip should succeed");

    let equipped = fixture
        .character_repository
        .find_equipped_slot(fixture.player_character.id, EquipmentSlotModel::Weapon)
        .await
        .expect("query should succeed")
        .expect("weapon should be equipped");
    assert_eq!(equipped.item_instance_id, weapon_instance.id);

    let equipment_inventory = fixture.find_inventory(InventoryTypeModel::Equipment).await;
    let source_slot = fixture
        .inventory_repository
        .find_slot_by_index(equipment_inventory.id, 0)
        .await
        .expect("slot query should succeed");
    assert!(source_slot.is_none());
}

#[tokio::test]
async fn swaps_equipped_item_back_to_inventory_in_real_database() {
    let Some(fixture) = TestFixture::new().await else {
        eprintln!("skipping db integration test: API_DATABASE_URL is unavailable");
        return;
    };
    let equipped_weapon = fixture
        .insert_item_instance("kael_training_blade", InventoryTypeModel::Equipment)
        .await;
    let inventory_weapon = fixture
        .insert_item_instance("kael_training_blade", InventoryTypeModel::Equipment)
        .await;

    fixture
        .character_repository
        .equip_item_instance(
            fixture.player_character.id,
            EquipmentSlotModel::Weapon,
            equipped_weapon.id,
        )
        .await
        .expect("seed equip should succeed");
    fixture
        .insert_inventory_instance(inventory_weapon.id, 0)
        .await;

    fixture
        .service
        .equip_inventory_item(fixture.player_character.id, "equipment".to_string(), 0)
        .await
        .expect("equip should swap");

    let equipped = fixture
        .character_repository
        .find_equipped_slot(fixture.player_character.id, EquipmentSlotModel::Weapon)
        .await
        .expect("query should succeed")
        .expect("weapon should remain equipped");
    assert_eq!(equipped.item_instance_id, inventory_weapon.id);

    let equipment_inventory = fixture.find_inventory(InventoryTypeModel::Equipment).await;
    let returned_slot = fixture
        .inventory_repository
        .find_slot_by_index(equipment_inventory.id, 0)
        .await
        .expect("slot query should succeed")
        .expect("old equipped item should return to source slot");
    assert_eq!(returned_slot.item_instance_id, Some(equipped_weapon.id));
}

#[tokio::test]
async fn unequips_item_instance_back_to_inventory_in_real_database() {
    let Some(fixture) = TestFixture::new().await else {
        eprintln!("skipping db integration test: API_DATABASE_URL is unavailable");
        return;
    };
    let equipped_weapon = fixture
        .insert_item_instance("kael_training_blade", InventoryTypeModel::Equipment)
        .await;

    fixture
        .character_repository
        .equip_item_instance(
            fixture.player_character.id,
            EquipmentSlotModel::Weapon,
            equipped_weapon.id,
        )
        .await
        .expect("seed equip should succeed");

    fixture
        .service
        .unequip_item(fixture.player_character.id, "weapon".to_string())
        .await
        .expect("unequip should succeed");

    let equipped = fixture
        .character_repository
        .find_equipped_slot(fixture.player_character.id, EquipmentSlotModel::Weapon)
        .await
        .expect("query should succeed");
    assert!(equipped.is_none());

    let equipment_inventory = fixture.find_inventory(InventoryTypeModel::Equipment).await;
    let returned_slot = fixture
        .inventory_repository
        .find_slot_by_index(equipment_inventory.id, 0)
        .await
        .expect("slot query should succeed")
        .expect("item should return to inventory");
    assert_eq!(returned_slot.item_instance_id, Some(equipped_weapon.id));
}

#[tokio::test]
async fn rejects_unequip_when_inventory_is_full_in_real_database() {
    let Some(fixture) = TestFixture::new().await else {
        eprintln!("skipping db integration test: API_DATABASE_URL is unavailable");
        return;
    };
    let equipped_weapon = fixture
        .insert_item_instance("kael_training_blade", InventoryTypeModel::Equipment)
        .await;

    fixture
        .character_repository
        .equip_item_instance(
            fixture.player_character.id,
            EquipmentSlotModel::Weapon,
            equipped_weapon.id,
        )
        .await
        .expect("seed equip should succeed");

    for slot_index in 0..56 {
        let filler = fixture
            .insert_item_instance("kael_squire_chestplate", InventoryTypeModel::Equipment)
            .await;
        fixture
            .insert_inventory_instance(filler.id, slot_index)
            .await;
    }

    let err = fixture
        .service
        .unequip_item(fixture.player_character.id, "weapon".to_string())
        .await
        .expect_err("unequip should fail when inventory is full");

    match err {
        AppError::BadRequest(message) => {
            assert!(message.contains("No inventory space available"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

struct TestFixture {
    db: Database,
    service: EquipmentService,
    character_repository: PgCharacterRepository,
    inventory_repository: PgInventoryRepository,
    item_repository: PgItemRepository,
    item_instance_repository: PgItemInstanceRepository,
    player_character: crate::models::player_character::PlayerCharacterModel,
    account: AccountModel,
}

impl TestFixture {
    async fn new() -> Option<Self> {
        let db = test_database()?;
        let account_repository = PgAccountRepository::new(db.clone());
        let character_repository = PgCharacterRepository::new(db.clone());
        let inventory_repository = PgInventoryRepository::new(db.clone());
        let item_repository = PgItemRepository::new(db.clone());
        let item_instance_repository = PgItemInstanceRepository::new(db.clone());

        seed_catalog(&character_repository, &item_repository).await;

        let account = account_repository
            .create(CreateAccountParams {
                email: format!("equip-test-{}@local.dev", Uuid::new_v4()),
                password_hash: "hash".to_string(),
            })
            .await
            .expect("account should be created");

        let player_character = character_repository
            .create(
                account.id,
                CreateCharacterParams {
                    name: format!("Kael{}", &Uuid::new_v4().simple().to_string()[..8]),
                    class: "kael".to_string(),
                },
            )
            .await
            .expect("character should be created");

        Some(Self {
            service: EquipmentService::new(
                character_repository.clone(),
                inventory_repository.clone(),
                item_repository.clone(),
                item_instance_repository.clone(),
            ),
            db,
            character_repository,
            inventory_repository,
            item_repository,
            item_instance_repository,
            player_character,
            account,
        })
    }

    async fn insert_item_instance(
        &self,
        item_slug: &str,
        inventory_type: InventoryTypeModel,
    ) -> ItemInstanceModel {
        let item = self.upsert_catalog_item(item_slug, inventory_type).await;
        let item_instance = ItemInstanceModel::new(
            item.id,
            0,
            0,
            json!({}),
            Some(self.player_character.id),
            Some(self.account.id),
        );

        let item_instance_clone = item_instance.clone();
        self.item_instance_repository
            .run_blocking(move |conn| {
                diesel::insert_into(schema::item_instances::table)
                    .values(&item_instance_clone)
                    .execute(conn)?;
                Ok(())
            })
            .await
            .expect("item instance should be inserted");

        item_instance
    }

    async fn insert_inventory_instance(&self, item_instance_id: Uuid, slot_index: i16) {
        let equipment_inventory = self.find_inventory(InventoryTypeModel::Equipment).await;
        let inventory_item = InventoryItemModel::new(
            equipment_inventory.id,
            Some(item_instance_id),
            None,
            slot_index,
            1,
        );

        self.inventory_repository
            .run_blocking(move |conn| {
                diesel::insert_into(schema::inventory_items::table)
                    .values(&inventory_item)
                    .execute(conn)?;
                Ok(())
            })
            .await
            .expect("inventory slot should be inserted");
    }

    async fn find_inventory(
        &self,
        inventory_type: InventoryTypeModel,
    ) -> crate::models::inventory::InventoryModel {
        self.inventory_repository
            .find_by_character_and_type(self.player_character.id, inventory_type.to_string())
            .await
            .expect("inventory should exist")
    }

    async fn upsert_catalog_item(
        &self,
        item_slug: &str,
        inventory_type: InventoryTypeModel,
    ) -> ItemModel {
        self.item_repository
            .upsert_catalog_item(item_slug.to_string(), inventory_type)
            .await
            .expect("catalog item should sync")
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        let mut conn = self.db.get();

        let _ = diesel::delete(
            schema::inventory_items::table.filter(
                schema::inventory_items::inventory_id.eq_any(
                    schema::inventory::table
                        .filter(schema::inventory::character_id.eq(self.player_character.id))
                        .select(schema::inventory::id),
                ),
            ),
        )
        .execute(&mut conn);

        let _ = diesel::delete(
            schema::equipment::table
                .filter(schema::equipment::character_id.eq(self.player_character.id)),
        )
        .execute(&mut conn);

        let _ =
            diesel::delete(schema::item_instances::table.filter(
                schema::item_instances::owner_character_id.eq(Some(self.player_character.id)),
            ))
            .execute(&mut conn);

        let _ = diesel::delete(
            schema::inventory::table
                .filter(schema::inventory::character_id.eq(self.player_character.id)),
        )
        .execute(&mut conn);

        let _ = diesel::delete(
            schema::player_characters::table
                .filter(schema::player_characters::id.eq(self.player_character.id)),
        )
        .execute(&mut conn);

        let _ = diesel::delete(
            schema::accounts::table.filter(schema::accounts::id.eq(self.account.id)),
        )
        .execute(&mut conn);
    }
}

async fn seed_catalog(
    character_repository: &PgCharacterRepository,
    item_repository: &PgItemRepository,
) {
    character_repository
        .upsert_catalog_character("kael")
        .await
        .expect("character catalog should sync");

    for slug in ["kael_training_blade", "kael_squire_chestplate"] {
        let item = find_item_by_slug(slug).expect("catalog item");
        item_repository
            .upsert_catalog_item(slug.to_string(), map_inventory_type(item.inventory_type))
            .await
            .expect("item catalog should sync");
    }
}

fn map_inventory_type(value: shared::models::inventory_type::InventoryType) -> InventoryTypeModel {
    match value {
        shared::models::inventory_type::InventoryType::Equipment => InventoryTypeModel::Equipment,
        shared::models::inventory_type::InventoryType::Accessory => InventoryTypeModel::Accessory,
        shared::models::inventory_type::InventoryType::Consumable => InventoryTypeModel::Consumable,
        shared::models::inventory_type::InventoryType::Material => InventoryTypeModel::Material,
        shared::models::inventory_type::InventoryType::QuestItem => InventoryTypeModel::QuestItem,
        shared::models::inventory_type::InventoryType::Special => InventoryTypeModel::Special,
    }
}

fn test_database() -> Option<Database> {
    static DB_URL: OnceLock<String> = OnceLock::new();
    let url = DB_URL.get_or_init(|| {
        dotenvy::dotenv().ok();
        env::var("API_DATABASE_URL").unwrap_or_default()
    });
    if url.is_empty() {
        return None;
    }

    let manager = ConnectionManager::<PgConnection>::new(url.as_str());
    let pool = r2d2::Pool::builder().build(manager).ok()?;
    let mut conn = pool.get().ok()?;
    conn.run_pending_migrations(MIGRATIONS).ok()?;
    Some(Database(pool))
}
