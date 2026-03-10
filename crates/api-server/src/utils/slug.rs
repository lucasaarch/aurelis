
pub fn generate_slug(name: &str) -> String {
    let mut slug = name.to_lowercase();
    // replace non-alnum and not underscore with '_'
    slug = regex::Regex::new(r"[^a-z0-9_]+")
        .unwrap()
        .replace_all(&slug, "_")
        .to_string();
    // collapse multiple underscores
    slug = regex::Regex::new(r"_+")
        .unwrap()
        .replace_all(&slug, "_")
        .to_string();
    // trim leading/trailing underscores
    slug = regex::Regex::new(r"(^_+|_+$)")
        .unwrap()
        .replace_all(&slug, "")
        .to_string();
    if slug.is_empty() {
        "invalid_slug".into()
    } else if slug.len() > 64 {
        slug.truncate(64);
        slug
    } else {
        slug
    }
}