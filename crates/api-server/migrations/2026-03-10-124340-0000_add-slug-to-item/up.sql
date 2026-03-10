ALTER TABLE items
    ADD COLUMN slug VARCHAR(64) NOT NULL UNIQUE;

CREATE INDEX idx_items_slug ON items (slug);