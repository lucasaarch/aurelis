CREATE OR REPLACE FUNCTION generate_slug(value TEXT)
RETURNS TEXT AS $$
DECLARE
    slug TEXT;
BEGIN
    slug := lower(value);
    slug := regexp_replace(slug, '[^a-z0-9_]+', '_', 'g');
    slug := regexp_replace(slug, '_+', '_', 'g');
    slug := regexp_replace(slug, '(^_+|_+$)', '', 'g');

    IF slug = '' THEN
        RETURN 'invalid_slug';
    ELSIF length(slug) > 64 THEN
        RETURN substring(slug from 1 for 64);
    END IF;

    RETURN slug;
END;
$$ LANGUAGE plpgsql IMMUTABLE;
