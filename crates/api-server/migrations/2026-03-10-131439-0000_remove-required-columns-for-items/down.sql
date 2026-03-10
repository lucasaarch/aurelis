-- This file should undo anything in `up.sql`
UPDATE items SET level_req = 1 WHERE level_req IS NULL;
UPDATE items SET stats = '{}' WHERE stats IS NULL;

ALTER TABLE items
    ALTER COLUMN level_req SET NOT NULL,
    ALTER COLUMN level_req SET DEFAULT 1,
    ALTER COLUMN stats SET NOT NULL,
    ALTER COLUMN stats SET DEFAULT '{}';