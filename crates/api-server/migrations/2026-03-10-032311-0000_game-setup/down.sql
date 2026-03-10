DROP INDEX IF EXISTS idx_currency_transactions_char;

DROP INDEX IF EXISTS idx_currency_transactions_acc;

DROP INDEX IF EXISTS idx_skills_class;

DROP INDEX IF EXISTS idx_character_skill_slots_char;

DROP INDEX IF EXISTS idx_character_skills_char;

DROP INDEX IF EXISTS idx_evolution_steps_line;

DROP INDEX IF EXISTS idx_evolution_lines_class;

DROP INDEX IF EXISTS idx_dungeon_history_char;

DROP INDEX IF EXISTS idx_character_quests_char;

DROP INDEX IF EXISTS idx_refresh_tokens_account;

DROP INDEX IF EXISTS idx_inventory_item_instance;

DROP INDEX IF EXISTS idx_item_instances_account;

DROP INDEX IF EXISTS idx_item_instances_character;

DROP INDEX IF EXISTS idx_inventory_character_id;

DROP INDEX IF EXISTS idx_characters_account_id;

DROP TABLE IF EXISTS currency_transactions;

DROP TABLE IF EXISTS character_consumable_slots;

DROP TABLE IF EXISTS character_skill_slots;

DROP TABLE IF EXISTS character_skills;

DROP TABLE IF EXISTS skills;

DROP TABLE IF EXISTS character_evolution;

DROP TABLE IF EXISTS evolution_steps;

DROP TABLE IF EXISTS evolution_lines;

DROP TABLE IF EXISTS dungeon_history;

DROP TABLE IF EXISTS character_quests;

DROP TABLE IF EXISTS quests;

DROP TABLE IF EXISTS equipment;

DROP TABLE IF EXISTS item_instance_gems;

DROP TABLE IF EXISTS item_instances;

DROP TABLE IF EXISTS inventory;

DROP TABLE IF EXISTS items;

DROP TABLE IF EXISTS characters;

DROP TABLE IF EXISTS refresh_tokens;

DROP TABLE IF EXISTS accounts;

DROP TYPE IF EXISTS currency_origin;

DROP TYPE IF EXISTS quest_status;

DROP TYPE IF EXISTS inventory_type;

DROP TYPE IF EXISTS equipment_slot_type;

DROP TYPE IF EXISTS item_rarity;

DROP TYPE IF EXISTS character_location;

DROP TYPE IF EXISTS character_class;

DROP EXTENSION IF EXISTS "uuid-ossp";