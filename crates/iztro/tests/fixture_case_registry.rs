//! Keeps `fixtures/iztro/CASES.json` and the `birth_cases` ids in MANIFEST.json
//! in sync with each other and with the fixtures' duplicated `input` blocks.
//!
//! CASES.json is the canonical, reusable definition of the recurring birth
//! cases. The compat/runtime fixtures still carry their own self-contained
//! `input` blocks; these tests guard against three kinds of drift:
//!
//! - a registry that is malformed or missing required case fields;
//! - a manifest `birth_cases` id that has no registered case;
//! - a fixture `input` block whose fields diverge from the registered case.

mod common;

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use common::{
    assert_input_matches_case, birth_case, birth_case_ids, birth_cases, input_is_checkable_against,
};

const MANIFEST: &str = include_str!("../fixtures/iztro/MANIFEST.json");

const REQUIRED_CASE_FIELDS: &[&str] = &[
    "id",
    "calendar",
    "year",
    "month",
    "day",
    "birth_time",
    "time_index",
    "gender",
    "is_leap_month",
    "fix_leap",
];

fn manifest() -> Value {
    serde_json::from_str(MANIFEST).expect("MANIFEST.json should be valid JSON")
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/iztro")
}

fn manifest_entries(manifest: &Value) -> &Vec<Value> {
    manifest["fixtures"]
        .as_array()
        .expect("manifest should have a `fixtures` array")
}

/// Reads a fixture file's `input` block, if any.
fn fixture_input(file: &str) -> Option<Value> {
    let raw = fs::read_to_string(fixtures_dir().join(file))
        .unwrap_or_else(|err| panic!("fixture `{file}` should be readable: {err}"));
    let value: Value =
        serde_json::from_str(&raw).unwrap_or_else(|err| panic!("`{file}` should be JSON: {err}"));
    match value.get("input") {
        Some(input) if input.is_object() => Some(input.clone()),
        _ => None,
    }
}

#[test]
fn cases_json_is_valid() {
    let cases = birth_cases();
    assert!(
        !cases.is_empty(),
        "CASES.json should declare at least one case"
    );

    for case in &cases {
        for field in REQUIRED_CASE_FIELDS {
            assert!(
                !case[*field].is_null(),
                "case {case:?} is missing required field `{field}`"
            );
        }
        let id = case["id"].as_str().expect("case `id` should be a string");
        assert!(!id.is_empty(), "case `id` must be non-empty");
        assert!(
            case["calendar"].as_str().is_some(),
            "{id}: `calendar` must be a string"
        );
        assert!(
            case["year"].as_i64().is_some(),
            "{id}: `year` must be an integer"
        );
        assert!(
            case["month"].as_i64().is_some(),
            "{id}: `month` must be an integer"
        );
        assert!(
            case["day"].as_i64().is_some(),
            "{id}: `day` must be an integer"
        );
        assert!(
            case["time_index"].as_i64().is_some(),
            "{id}: `time_index` must be an integer"
        );
        assert!(
            case["gender"].as_str().is_some(),
            "{id}: `gender` must be a string"
        );
        assert!(
            case["is_leap_month"].as_bool().is_some(),
            "{id}: `is_leap_month` must be a bool"
        );
        assert!(
            case["fix_leap"].as_bool().is_some(),
            "{id}: `fix_leap` must be a bool"
        );
    }
}

#[test]
fn case_ids_are_unique() {
    let mut seen = BTreeSet::new();
    for id in birth_case_ids() {
        assert!(
            seen.insert(id.clone()),
            "duplicate birth-case id `{id}` in CASES.json"
        );
    }
}

#[test]
fn every_manifest_birth_case_refers_to_a_registered_case() {
    let registered: BTreeSet<String> = birth_case_ids().into_iter().collect();
    let manifest = manifest();

    // Top-level birth_cases pointer.
    for id in manifest["birth_cases"]
        .as_array()
        .expect("manifest should have a top-level `birth_cases` array")
    {
        let id = id.as_str().expect("birth case id should be a string");
        assert!(
            registered.contains(id),
            "manifest top-level birth case `{id}` is not registered in CASES.json"
        );
    }

    // Per-fixture birth_cases references.
    for entry in manifest_entries(&manifest) {
        let file = entry["file"].as_str().expect("entry should have a `file`");
        let ids = entry["birth_cases"]
            .as_array()
            .unwrap_or_else(|| panic!("{file}: entry should have a `birth_cases` array"));
        for id in ids {
            let id = id.as_str().expect("birth case id should be a string");
            assert!(
                registered.contains(id),
                "{file}: birth case `{id}` is not registered in CASES.json"
            );
        }
    }
}

#[test]
fn recurring_fixture_inputs_match_registered_cases() {
    let manifest = manifest();
    let mut checked = 0usize;

    for entry in manifest_entries(&manifest) {
        let file = entry["file"].as_str().expect("entry should have a `file`");
        let ids = entry["birth_cases"]
            .as_array()
            .unwrap_or_else(|| panic!("{file}: entry should have a `birth_cases` array"));

        // Broad e2e/horoscope/flow fixtures declare no single birth case; skip
        // them, and skip anything that maps to more than one case (ambiguous).
        if ids.len() != 1 {
            continue;
        }

        let case_id = ids[0].as_str().expect("birth case id should be a string");
        let case = birth_case(case_id)
            .unwrap_or_else(|| panic!("{file}: birth case `{case_id}` is not registered"));

        let Some(input) = fixture_input(file) else {
            continue;
        };

        // Solar readings that share a lunar case id encode a different birth
        // instant; only compare same-calendar inputs.
        if !input_is_checkable_against(&input, &case) {
            continue;
        }

        assert_input_matches_case(file, &input, &case);
        checked += 1;
    }

    assert!(
        checked > 0,
        "expected at least one recurring fixture input to be drift-checked against CASES.json"
    );
}
