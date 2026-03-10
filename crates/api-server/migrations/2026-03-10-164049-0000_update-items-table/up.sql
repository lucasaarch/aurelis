-- Your SQL goes here
ALTER TABLE items
    ADD COLUMN max_stack SMALLINT NOT NULL DEFAULT 1;
