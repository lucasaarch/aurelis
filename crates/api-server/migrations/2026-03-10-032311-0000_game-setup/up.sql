-- ============================================================
-- EXTENSÕES
-- ============================================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


-- ============================================================
-- ACCOUNTS
-- ============================================================

CREATE TABLE accounts (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username      VARCHAR(32)  NOT NULL UNIQUE,
    email         VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    region        VARCHAR(16)  NOT NULL, -- 'america', 'europa', 'asia'
    is_banned     BOOLEAN      NOT NULL DEFAULT FALSE,
    banned_at     TIMESTAMPTZ,
    banned_reason TEXT,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ============================================================
-- REFRESH TOKENS
-- ============================================================

CREATE TABLE refresh_tokens (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id    UUID         NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    token_hash    VARCHAR(255) NOT NULL UNIQUE,
    expires_at    TIMESTAMPTZ  NOT NULL,
    revoked       BOOLEAN      NOT NULL DEFAULT FALSE,
    revoked_at    TIMESTAMPTZ,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ============================================================
-- CHARACTERS
-- ============================================================

CREATE TYPE character_class AS ENUM (
    'character_1', -- substituir pelo nome real depois
    'character_2',
    'character_3'
);

CREATE TYPE character_location AS ENUM (
    'aurelis',
    'volcanis',
    'aquavale',
    'sylvandar'
);

CREATE TABLE characters (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id      UUID             NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name            VARCHAR(24)      NOT NULL UNIQUE,
    class           character_class  NOT NULL,
    level           SMALLINT         NOT NULL DEFAULT 1,
    experience      BIGINT           NOT NULL DEFAULT 0,
    hp_current      INT              NOT NULL,
    hp_max          INT              NOT NULL,
    mana_current    INT              NOT NULL,
    mana_max        INT              NOT NULL,
    location        character_location NOT NULL DEFAULT 'aurelis',

    is_online       BOOLEAN          NOT NULL DEFAULT FALSE,
    last_seen_at    TIMESTAMPTZ,
    created_at      TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ      NOT NULL DEFAULT NOW()
);


-- ============================================================
-- ITEMS
-- ============================================================

CREATE TYPE item_rarity AS ENUM (
    'common',
    'uncommon',
    'rare',
    'epic'
);

CREATE TYPE item_slot AS ENUM (
    'weapon',
    'head',
    'chest',
    'legs',
    'accessory'
);

CREATE TABLE items (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR(64)  NOT NULL,
    description TEXT,
    rarity      item_rarity  NOT NULL,
    slot        item_slot    NOT NULL,
    level_req   SMALLINT     NOT NULL DEFAULT 1,
    stats       JSONB        NOT NULL DEFAULT '{}', -- atk, def, hp, etc
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ============================================================
-- INVENTORY
-- ============================================================

CREATE TABLE inventory (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    character_id    UUID        NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    item_id         UUID        NOT NULL REFERENCES items(id),
    quantity        SMALLINT    NOT NULL DEFAULT 1,
    acquired_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- ============================================================
-- EQUIPMENT
-- Itens atualmente equipados pelo personagem
-- ============================================================

CREATE TABLE equipment (
    character_id    UUID      NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    slot            item_slot NOT NULL,
    inventory_id    UUID      NOT NULL REFERENCES inventory(id),
    equipped_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (character_id, slot)
);


-- ============================================================
-- QUESTS
-- ============================================================

CREATE TYPE quest_status AS ENUM (
    'available',
    'in_progress',
    'completed'
);

CREATE TABLE quests (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR(64) NOT NULL,
    description TEXT,
    city        character_location,   -- NULL = quest global
    level_req   SMALLINT    NOT NULL DEFAULT 1,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE character_quests (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    character_id    UUID         NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    quest_id        UUID         NOT NULL REFERENCES quests(id),
    status          quest_status NOT NULL DEFAULT 'available',
    started_at      TIMESTAMPTZ,
    completed_at    TIMESTAMPTZ,

    UNIQUE (character_id, quest_id)
);


-- ============================================================
-- DUNGEON HISTORY
-- Registro de dungeons completadas
-- ============================================================

CREATE TABLE dungeon_history (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    character_id    UUID        NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    dungeon_id      VARCHAR(32) NOT NULL, -- ex: 'volcanis_dungeon_1'
    hard_mode       BOOLEAN     NOT NULL DEFAULT FALSE,
    completed_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    xp_gained       INT         NOT NULL DEFAULT 0,
    duration_secs   INT
);


-- ============================================================
-- EVOLUTION LINES
-- Linhas evolutivas disponíveis por classe
-- ============================================================

CREATE TABLE evolution_lines (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    character_class character_class NOT NULL,
    name            VARCHAR(64)     NOT NULL,
    description     TEXT,
    order_index     SMALLINT        NOT NULL, -- ordem de exibição na tela
    created_at      TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);


-- ============================================================
-- EVOLUTION STEPS
-- Passos lineares dentro de cada linha evolutiva
-- ============================================================

CREATE TABLE evolution_steps (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    line_id         UUID         NOT NULL REFERENCES evolution_lines(id) ON DELETE CASCADE,
    name            VARCHAR(64)  NOT NULL,
    description     TEXT,
    step_index      SMALLINT     NOT NULL, -- posição dentro da linha (0, 1, 2...)
    level_req       SMALLINT     NOT NULL, -- level mínimo para evoluir
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    UNIQUE (line_id, step_index)
);


-- ============================================================
-- CHARACTER EVOLUTION
-- Progresso evolutivo do personagem
-- ============================================================

CREATE TABLE character_evolution (
    character_id    UUID        NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    line_id         UUID        REFERENCES evolution_lines(id), -- NULL = ainda na classe base
    current_step    SMALLINT    NOT NULL DEFAULT 0,             -- passo atual dentro da linha
    last_evolved_at TIMESTAMPTZ,                                -- última evolução de step

    PRIMARY KEY (character_id)
);


-- ============================================================
-- ÍNDICES
-- ============================================================

CREATE INDEX idx_characters_account_id   ON characters(account_id);
CREATE INDEX idx_inventory_character_id  ON inventory(character_id);
CREATE INDEX idx_refresh_tokens_account  ON refresh_tokens(account_id);
CREATE INDEX idx_character_quests_char   ON character_quests(character_id);
CREATE INDEX idx_dungeon_history_char    ON dungeon_history(character_id);
CREATE INDEX idx_evolution_lines_class   ON evolution_lines(character_class);
CREATE INDEX idx_evolution_steps_line    ON evolution_steps(line_id);
