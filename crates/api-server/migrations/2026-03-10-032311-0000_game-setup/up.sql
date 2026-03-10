CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    accounts (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        username VARCHAR(32) NOT NULL UNIQUE,
        email VARCHAR(255) NOT NULL UNIQUE,
        password_hash VARCHAR(255) NOT NULL,
        max_characters SMALLINT NOT NULL DEFAULT 3,
        shared_storage_enabled BOOLEAN NOT NULL DEFAULT FALSE,
        shared_storage_capacity SMALLINT NOT NULL DEFAULT 20,
        cash BIGINT NOT NULL DEFAULT 0,
        stored_credits BIGINT NOT NULL DEFAULT 0,
        email_verified BOOLEAN NOT NULL DEFAULT FALSE,
        email_verify_token VARCHAR(255),
        email_verify_token_expires TIMESTAMPTZ,
        password_reset_token VARCHAR(255),
        password_reset_expires TIMESTAMPTZ,
        banned_at TIMESTAMPTZ,
        banned_reason TEXT,
        suspended_until TIMESTAMPTZ,
        chat_restricted_until TIMESTAMPTZ,
        last_login_at TIMESTAMPTZ,
        last_login_ip VARCHAR(45),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        deleted_at TIMESTAMPTZ
    );

CREATE TABLE
    refresh_tokens (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        account_id UUID NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
        token_hash VARCHAR(255) NOT NULL UNIQUE,
        expires_at TIMESTAMPTZ NOT NULL,
        revoked BOOLEAN NOT NULL DEFAULT FALSE,
        revoked_at TIMESTAMPTZ,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TYPE character_class AS ENUM ('kael', 'rin', 'sirena');

CREATE TYPE character_location AS ENUM ('aurelis', 'volcanis', 'aquavale', 'sylvandar');

CREATE TABLE
    characters (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        account_id UUID NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
        name VARCHAR(24) NOT NULL UNIQUE,
        class character_class NOT NULL,
        level SMALLINT NOT NULL DEFAULT 1,
        experience BIGINT NOT NULL DEFAULT 0,
        location character_location NOT NULL DEFAULT 'aurelis',
        credits BIGINT NOT NULL DEFAULT 0,
        equipment_inventory_capacity SMALLINT NOT NULL DEFAULT 56,
        accessory_inventory_capacity SMALLINT NOT NULL DEFAULT 56,
        consumable_inventory_capacity SMALLINT NOT NULL DEFAULT 56,
        material_inventory_capacity SMALLINT NOT NULL DEFAULT 56,
        quest_item_inventory_capacity SMALLINT NOT NULL DEFAULT 56,
        special_inventory_capacity SMALLINT NOT NULL DEFAULT 56,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TYPE item_rarity AS ENUM ('common', 'uncommon', 'rare', 'epic');

CREATE TYPE equipment_slot AS ENUM ('weapon', 'head', 'chest', 'legs', 'accessory');

CREATE TYPE inventory_type AS ENUM (
    'equipment',
    'accessory',
    'consumable',
    'material',
    'quest_item',
    'special'
);

CREATE TABLE
    items (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        name VARCHAR(64) NOT NULL,
        description TEXT,
        rarity item_rarity NOT NULL,
        equipment_slot equipment_slot NULL,
        class character_class NULL,
        level_req SMALLINT NOT NULL DEFAULT 1,
        stats JSONB NOT NULL DEFAULT '{}',
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    inventory (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        character_id UUID NOT NULL REFERENCES characters (id) ON DELETE CASCADE,
        item_id UUID NOT NULL REFERENCES items (id),
        inventory_type inventory_type NOT NULL,
        slot_index SMALLINT NOT NULL CHECK (slot_index >= 0),
        quantity SMALLINT NOT NULL DEFAULT 1,
        acquired_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        UNIQUE (character_id, inventory_type, slot_index)
    );

CREATE TABLE
    equipment (
        character_id UUID NOT NULL REFERENCES characters (id) ON DELETE CASCADE,
        slot equipment_slot NOT NULL,
        inventory_id UUID NOT NULL REFERENCES inventory (id),
        equipped_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        PRIMARY KEY (character_id, slot)
    );

CREATE TYPE quest_status AS ENUM ('available', 'in_progress', 'completed');

CREATE TABLE
    quests (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        name VARCHAR(64) NOT NULL,
        description TEXT,
        city character_location,
        level_req SMALLINT NOT NULL DEFAULT 1,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    character_quests (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        character_id UUID NOT NULL REFERENCES characters (id) ON DELETE CASCADE,
        quest_id UUID NOT NULL REFERENCES quests (id),
        status quest_status NOT NULL DEFAULT 'available',
        started_at TIMESTAMPTZ,
        completed_at TIMESTAMPTZ,
        UNIQUE (character_id, quest_id)
    );

CREATE TABLE
    dungeon_history (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        character_id UUID NOT NULL REFERENCES characters (id) ON DELETE CASCADE,
        dungeon_id VARCHAR(32) NOT NULL,
        hard_mode BOOLEAN NOT NULL DEFAULT FALSE,
        completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        xp_gained INT NOT NULL DEFAULT 0,
        duration_secs INT
    );

CREATE TABLE
    evolution_lines (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        character_class character_class NOT NULL,
        name VARCHAR(64) NOT NULL,
        description TEXT,
        order_index SMALLINT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    evolution_steps (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        line_id UUID NOT NULL REFERENCES evolution_lines (id) ON DELETE CASCADE,
        name VARCHAR(64) NOT NULL,
        description TEXT,
        step_index SMALLINT NOT NULL,
        level_req SMALLINT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        UNIQUE (line_id, step_index)
    );

CREATE TABLE
    character_evolution (
        character_id UUID NOT NULL REFERENCES characters (id) ON DELETE CASCADE,
        line_id UUID REFERENCES evolution_lines (id),
        current_step SMALLINT NOT NULL DEFAULT 0,
        last_evolved_at TIMESTAMPTZ,
        PRIMARY KEY (character_id)
    );

CREATE TABLE
    skills (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        name VARCHAR(64) NOT NULL,
        description TEXT,
        character_class character_class NOT NULL,
        line_id UUID REFERENCES evolution_lines (id),
        level_req SMALLINT NOT NULL DEFAULT 1,
        max_level SMALLINT NOT NULL DEFAULT 1,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    character_skills (
        character_id UUID NOT NULL REFERENCES characters (id) ON DELETE CASCADE,
        skill_id UUID NOT NULL REFERENCES skills (id),
        current_level SMALLINT NOT NULL DEFAULT 1,
        unlocked_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        PRIMARY KEY (character_id, skill_id)
    );

CREATE TABLE
    character_skill_slots (
        character_id UUID NOT NULL REFERENCES characters (id) ON DELETE CASCADE,
        slot SMALLINT NOT NULL CHECK (slot BETWEEN 1 AND 8),
        skill_id UUID NOT NULL REFERENCES skills (id),
        PRIMARY KEY (character_id, slot)
    );

CREATE TABLE
    character_consumable_slots (
        character_id UUID NOT NULL REFERENCES characters (id) ON DELETE CASCADE,
        slot SMALLINT NOT NULL CHECK (slot BETWEEN 1 AND 6),
        inventory_id UUID REFERENCES inventory (id),
        PRIMARY KEY (character_id, slot)
    );

CREATE TYPE currency_origin AS ENUM (
    'purchase',
    'trade',
    'bonus',
    'dungeon',
    'quest',
    'npc'
);

CREATE TABLE
    currency_transactions (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        account_id UUID REFERENCES accounts (id) ON DELETE SET NULL,
        character_id UUID REFERENCES characters (id) ON DELETE SET NULL,
        currency VARCHAR(16) NOT NULL,
        amount BIGINT NOT NULL,
        balance_after BIGINT NOT NULL,
        origin currency_origin NOT NULL,
        reference_id UUID,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE INDEX idx_characters_account_id ON characters (account_id);

CREATE INDEX idx_inventory_character_id ON inventory (character_id);

CREATE INDEX idx_refresh_tokens_account ON refresh_tokens (account_id);

CREATE INDEX idx_character_quests_char ON character_quests (character_id);

CREATE INDEX idx_dungeon_history_char ON dungeon_history (character_id);

CREATE INDEX idx_evolution_lines_class ON evolution_lines (character_class);

CREATE INDEX idx_evolution_steps_line ON evolution_steps (line_id);

CREATE INDEX idx_character_skills_char ON character_skills (character_id);

CREATE INDEX idx_skills_class ON skills (character_class);

CREATE INDEX idx_character_skill_slots_char ON character_skill_slots (character_id);

CREATE INDEX idx_currency_transactions_acc ON currency_transactions (account_id);

CREATE INDEX idx_currency_transactions_char ON currency_transactions (character_id);