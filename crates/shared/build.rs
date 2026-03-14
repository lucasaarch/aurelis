use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/");
    println!("cargo:rerun-if-changed=src/data/");

    tonic_prost_build::configure()
        .file_descriptor_set_path(
            PathBuf::from(std::env::var("OUT_DIR")?).join("protos_descriptor.bin"),
        )
        .compile_protos(
            &[
                "proto/auth.proto",
                "proto/character.proto",
                "proto/internal_game.proto",
                "proto/inventory.proto",
            ],
            &["proto"],
        )?;

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let data_root = Path::new(&manifest_dir)
        .join("src")
        .join("data")
        .join("cities");
    let characters_root = Path::new(&manifest_dir)
        .join("src")
        .join("data")
        .join("characters");
    let quests_root = Path::new(&manifest_dir)
        .join("src")
        .join("data")
        .join("quests");

    if !data_root.exists() {
        return Ok(());
    }

    for (kind, matcher) in &[
        ("item", FileMatch::ExactName("items.rs")),
        ("mob", FileMatch::ExactName("mobs.rs")),
        ("dungeon", FileMatch::Suffix("dungeons/mod.rs")),
        ("stage group", FileMatch::ExactName("stage_groups.rs")),
    ] {
        let mut slugs: HashMap<String, Vec<PathBuf>> = HashMap::new();
        visit_files(&data_root, matcher, &mut slugs);
        ensure_unique_slugs(kind, slugs);
    }

    if characters_root.exists() {
        for (kind, matcher) in &[
            ("character", FileMatch::CharacterMod),
            ("class", FileMatch::ExactName("classes.rs")),
        ] {
            let mut slugs: HashMap<String, Vec<PathBuf>> = HashMap::new();
            visit_files(&characters_root, matcher, &mut slugs);
            ensure_unique_slugs(kind, slugs);
        }
    }

    if quests_root.exists() {
        let mut slugs: HashMap<String, Vec<PathBuf>> = HashMap::new();
        visit_files(&quests_root, &FileMatch::QuestFile, &mut slugs);
        ensure_unique_slugs("quest", slugs);
    }

    Ok(())
}

enum FileMatch {
    ExactName(&'static str),
    Suffix(&'static str),
    CharacterMod,
    QuestFile,
}

fn visit_files(root: &Path, matcher: &FileMatch, slugs: &mut HashMap<String, Vec<PathBuf>>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            visit_files(&path, matcher, slugs);
            continue;
        }

        let matches = match matcher {
            FileMatch::ExactName(name) => path.file_name().and_then(|n| n.to_str()) == Some(name),
            FileMatch::Suffix(suffix) => path.to_string_lossy().ends_with(suffix),
            FileMatch::CharacterMod => {
                path.file_name().and_then(|n| n.to_str()) == Some("mod.rs")
                    && path
                        .parent()
                        .and_then(|parent| parent.parent())
                        .and_then(|grandparent| grandparent.file_name())
                        .and_then(|name| name.to_str())
                        == Some("characters")
            }
            FileMatch::QuestFile => {
                path.extension().and_then(|ext| ext.to_str()) == Some("rs")
                    && path
                        .parent()
                        .and_then(|parent| parent.file_name())
                        .and_then(|name| name.to_str())
                        == Some("quests")
            }
        };

        if !matches {
            continue;
        }

        let Ok(contents) = fs::read_to_string(&path) else {
            continue;
        };

        for line in contents.lines() {
            if let Some(slug) = extract_slug(line) {
                slugs.entry(slug).or_default().push(path.clone());
            }
        }
    }
}
fn extract_slug(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with("slug:") {
        return None;
    }

    let first_quote = trimmed.find('"')?;
    let rest = &trimmed[first_quote + 1..];
    let second_quote = rest.find('"')?;
    Some(rest[..second_quote].to_string())
}

fn ensure_unique_slugs(kind: &str, slugs: HashMap<String, Vec<PathBuf>>) {
    let mut duplicates: Vec<(String, Vec<PathBuf>)> = slugs
        .into_iter()
        .filter(|(_, paths)| paths.len() > 1)
        .collect();

    if !duplicates.is_empty() {
        duplicates.sort_by(|a, b| a.0.cmp(&b.0));
        let mut message = format!("Duplicate {kind} slugs detected:\n");
        for (slug, paths) in duplicates {
            message.push_str(&format!("  - {slug}\n"));
            for path in paths {
                message.push_str(&format!("      at {}\n", path.display()));
            }
        }
        panic!("{message}");
    }
}
