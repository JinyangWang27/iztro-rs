//! Canonical birth-case registry helpers.
//!
//! `fixtures/iztro/CASES.json` records the recurring birth cases once, by stable
//! id. The compat/runtime fixtures still carry their own `input` blocks; these
//! helpers load the registry, look a case up by id, and compare a fixture
//! `input` block against the registered case so the duplication cannot drift.

use serde_json::Value;

/// Source of truth for the canonical reusable birth cases.
pub const CASES_FIXTURE: &str = include_str!("../../fixtures/iztro/CASES.json");

/// Parses `CASES.json` into a [`Value`].
pub fn cases_registry() -> Value {
    serde_json::from_str(CASES_FIXTURE).expect("CASES.json should be valid JSON")
}

/// Returns every registered birth case.
pub fn birth_cases() -> Vec<Value> {
    cases_registry()["cases"]
        .as_array()
        .expect("CASES.json should have a `cases` array")
        .to_vec()
}

/// Returns the set of declared birth-case ids.
pub fn birth_case_ids() -> Vec<String> {
    birth_cases()
        .iter()
        .map(|case| {
            case["id"]
                .as_str()
                .expect("case should have a string `id`")
                .to_owned()
        })
        .collect()
}

/// Looks a registered birth case up by id.
pub fn birth_case(id: &str) -> Option<Value> {
    birth_cases()
        .into_iter()
        .find(|case| case["id"].as_str() == Some(id))
}

/// The calendar system a fixture `input` block encodes, inferred from its keys.
///
/// Compat/runtime fixtures use `lunar_*` keys for lunar inputs and `solar_date`
/// for solar inputs. Returns `None` when neither shape is recognized.
pub fn input_calendar(input: &Value) -> Option<&'static str> {
    if input.get("lunar_year").is_some() || input.get("lunar_date").is_some() {
        Some("lunar")
    } else if input.get("solar_date").is_some() {
        Some("solar")
    } else {
        None
    }
}

/// Whether a fixture `input` block can be unambiguously checked against `case`.
///
/// A case is only comparable to an `input` block written in the same calendar
/// system. The solar `input` blocks that share a lunar case's id (e.g. the
/// `major_stars`/`minimal_natal` solar readings of `1990_05_17_chen_female`)
/// encode a different birth instant and are intentionally not drift-checked.
pub fn input_is_checkable_against(input: &Value, case: &Value) -> bool {
    input_calendar(input) == case["calendar"].as_str()
}

/// Asserts that a checkable lunar fixture `input` block matches `case`.
///
/// Caller must have confirmed [`input_is_checkable_against`]; this compares the
/// stable identity fields and panics with `file` context on any mismatch.
pub fn assert_input_matches_case(file: &str, input: &Value, case: &Value) {
    let id = case["id"].as_str().expect("case id");

    assert_eq!(
        case["calendar"].as_str(),
        Some("lunar"),
        "{file}: only lunar cases are drift-checked; case `{id}` is not lunar"
    );

    let check_i64 = |field: &str, input_key: &str| {
        assert_eq!(
            input[input_key].as_i64(),
            case[field].as_i64(),
            "{file}: `{input_key}` does not match registered case `{id}` field `{field}`"
        );
    };
    let check_str = |field: &str, input_key: &str| {
        assert_eq!(
            input[input_key].as_str(),
            case[field].as_str(),
            "{file}: `{input_key}` does not match registered case `{id}` field `{field}`"
        );
    };
    let check_bool = |field: &str, input_key: &str| {
        assert_eq!(
            input[input_key].as_bool(),
            case[field].as_bool(),
            "{file}: `{input_key}` does not match registered case `{id}` field `{field}`"
        );
    };

    check_i64("year", "lunar_year");
    check_i64("month", "lunar_month");
    check_i64("day", "lunar_day");
    check_str("birth_time", "birth_time");
    check_i64("time_index", "iztro_time_index");
    check_str("gender", "gender");
    check_bool("is_leap_month", "is_leap_month");
    check_bool("fix_leap", "fix_leap");
    check_str("language", "language");

    // Algorithm-independent identity metadata: only compared when the fixture
    // input carries it (the solar minimal_natal block omits the branch).
    if input.get("birth_year_stem").is_some() {
        check_str("birth_year_stem", "birth_year_stem");
    }
    if input.get("birth_year_branch").is_some() {
        check_str("birth_year_branch", "birth_year_branch");
    }
}
