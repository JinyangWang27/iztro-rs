//! Keeps `fixtures/iztro/MANIFEST.json` in sync with the committed fixtures.
//!
//! The manifest is an additive audit of the upstream-reference fixtures (see
//! its own `note`). These tests guard against drift: a fixture added without a
//! manifest entry, a manifest entry pointing at a missing file, or an entry
//! missing its required `file` / `category` / `covers` fields.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde_json::Value;

const MANIFEST: &str = include_str!("../fixtures/iztro/MANIFEST.json");

fn manifest() -> Value {
    serde_json::from_str(MANIFEST).expect("MANIFEST.json should be valid JSON")
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/iztro")
}

/// Returns every committed `*.json` fixture file name except the manifest.
fn committed_fixture_files() -> BTreeSet<String> {
    fs::read_dir(fixtures_dir())
        .expect("fixtures directory should be readable")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".json") && name != "MANIFEST.json" && name != "CASES.json")
        .collect()
}

fn manifest_entries(manifest: &Value) -> &Vec<Value> {
    manifest["fixtures"]
        .as_array()
        .expect("manifest should have a `fixtures` array")
}

#[test]
fn manifest_entries_are_unique() {
    let manifest = manifest();
    let mut seen = BTreeSet::new();

    for entry in manifest_entries(&manifest) {
        let file = entry["file"].as_str().expect("entry should have a `file`");
        assert!(
            seen.insert(file.to_owned()),
            "duplicate MANIFEST.json entry for fixture `{file}`"
        );
    }
}

#[test]
fn manifest_categories_are_declared() {
    let manifest = manifest();

    let declared: BTreeSet<_> = manifest["categories"]
        .as_array()
        .expect("manifest should have categories")
        .iter()
        .map(|value| value.as_str().expect("category should be string"))
        .collect();

    for entry in manifest_entries(&manifest) {
        let file = entry["file"].as_str().expect("entry should have a `file`");
        let category = entry["category"]
            .as_str()
            .unwrap_or_else(|| panic!("{file}: entry should have a `category`"));

        assert!(
            declared.contains(category),
            "{file}: category `{category}` is not declared in MANIFEST.json categories"
        );
    }
}

#[test]
fn every_manifest_entry_points_to_an_existing_fixture() {
    let manifest = manifest();
    for entry in manifest_entries(&manifest) {
        let file = entry["file"].as_str().expect("entry should have a `file`");
        assert!(
            fixtures_dir().join(file).is_file(),
            "manifest entry `{file}` has no fixture file on disk"
        );
    }
}

#[test]
fn every_committed_fixture_is_listed_in_the_manifest() {
    let manifest = manifest();
    let listed: BTreeSet<String> = manifest_entries(&manifest)
        .iter()
        .map(|entry| {
            entry["file"]
                .as_str()
                .expect("entry should have a `file`")
                .to_owned()
        })
        .collect();

    let committed = committed_fixture_files();
    let missing: Vec<&String> = committed.difference(&listed).collect();
    assert!(
        missing.is_empty(),
        "committed fixtures not listed in MANIFEST.json: {missing:?}"
    );
}

#[test]
fn every_manifest_entry_has_required_fields() {
    let manifest = manifest();
    for entry in manifest_entries(&manifest) {
        let file = entry["file"].as_str().expect("entry should have a `file`");
        assert!(!file.is_empty(), "entry `file` must be non-empty");

        let category = entry["category"]
            .as_str()
            .unwrap_or_else(|| panic!("{file}: entry should have a `category`"));
        assert!(!category.is_empty(), "{file}: `category` must be non-empty");

        let covers = entry["covers"]
            .as_array()
            .unwrap_or_else(|| panic!("{file}: entry should have a `covers` array"));
        assert!(!covers.is_empty(), "{file}: `covers` must be non-empty");
        assert!(
            covers.iter().all(|item| item.is_string()),
            "{file}: `covers` entries must be strings"
        );
    }
}
