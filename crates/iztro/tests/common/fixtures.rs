//! Fixture loading helpers: the upstream `supported_fields` reference fixtures
//! and the generic JSON-loading utilities shared by the fixture-backed tests.

use serde_json::Value;

/// Source of truth for the upstream `FunctionalAstrolabe#horoscope` supported
/// fields, shared by every temporal-layer integration test.
pub const HOROSCOPE_FIXTURE: &str = include_str!("../../fixtures/iztro/horoscope.json");
pub const HOROSCOPE_RUNTIME_FIXTURE: &str =
    include_str!("../../fixtures/iztro/horoscope_runtime.json");
pub const HOROSCOPE_FACADE_FIXTURE: &str =
    include_str!("../../fixtures/iztro/horoscope_facade.json");

/// Parses a raw fixture JSON string into a [`Value`].
pub fn fixture_value(raw: &str) -> Value {
    serde_json::from_str(raw).expect("fixture should be valid JSON")
}

/// Parses a raw fixture JSON string and returns the array stored under `key`.
pub fn fixture_cases(raw: &str, key: &str) -> Vec<Value> {
    fixture_value(raw)[key]
        .as_array()
        .unwrap_or_else(|| panic!("fixture `{key}` should be an array"))
        .to_vec()
}

/// Returns every horoscope fixture case.
pub fn horoscope_fixture_cases() -> Vec<Value> {
    fixture_cases(HOROSCOPE_FIXTURE, "cases")
}

/// Returns one horoscope fixture case by id.
pub fn horoscope_fixture_case(case_id: &str) -> Value {
    horoscope_fixture_cases()
        .into_iter()
        .find(|case| case["id"].as_str() == Some(case_id))
        .unwrap_or_else(|| panic!("missing horoscope fixture case {case_id}"))
}

/// Returns every horoscope runtime-helper fixture case.
pub fn horoscope_runtime_fixture_cases() -> Vec<Value> {
    fixture_cases(HOROSCOPE_RUNTIME_FIXTURE, "cases")
}

/// Returns every horoscope facade fixture case.
pub fn horoscope_facade_fixture_cases() -> Vec<Value> {
    fixture_cases(HOROSCOPE_FACADE_FIXTURE, "cases")
}
