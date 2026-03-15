ALTER TABLE player_characters
    ADD COLUMN beginner_skill_unlocked BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN intermediate_skill_unlocked BOOLEAN NOT NULL DEFAULT FALSE;
