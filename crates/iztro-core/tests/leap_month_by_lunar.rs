//! End-to-end test for explicit leap-month `by_lunar` behavior against pinned
//! upstream `iztro@2.5.8`. Builds each fixture case through `by_lunar` with
//! `is_leap_month` / `fix_leap` set, then compares the currently supported chart
//! fields. The leap fourth-month day>15 pair (fix_leap true vs false) exercises
//! the effective-month divergence. Shared assertions live in `common`.

mod common;

use common::{
    assert_decorative_stars_match, assert_metadata_counts, assert_palaces_match,
    assert_suiqian_algorithm_boundary, assert_typed_stars_match, parse_algorithm, parse_key,
};

use iztro_core::{
    Chart, ChartAlgorithmKind, LunarChartRequest, LunarDay, LunarMonth, MethodProfile, by_lunar,
    resolve_lunar_date,
};
use serde_json::Value;

const FIXTURE: &str = include_str!("../../../fixtures/iztro/leap_month_by_lunar.json");

#[test]
fn by_lunar_matches_leap_month_fixture_cases() {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("fixture should be valid JSON");

    assert_eq!(
        fixture["metadata"]["target_package"].as_str(),
        Some("npm:iztro")
    );
    assert_eq!(
        fixture["metadata"]["target_version"].as_str(),
        Some("2.5.8")
    );
    assert_eq!(
        fixture["metadata"]["supported_fields_only"].as_bool(),
        Some(true)
    );

    let cases = fixture["cases"]
        .as_array()
        .expect("fixture should list leap-month cases");
    assert_eq!(cases.len(), 9);

    for fixture_case in cases {
        let algorithm = parse_algorithm(fixture_case["algorithm"].as_str().expect("algorithm"));
        let chart = chart_from_case(fixture_case, algorithm);
        let case_label = case_label(fixture_case);
        let supported = &fixture_case["supported_fields"];

        assert_metadata_counts();
        assert_resolved_lunar_matches(fixture_case, &case_label);
        assert_palaces_match(&chart, supported, &case_label);
        assert_typed_stars_match(&chart, supported, algorithm, &case_label);
        assert_decorative_stars_match(&chart, supported, &case_label);
        assert_suiqian_algorithm_boundary(&chart, algorithm, &case_label);
    }
}

fn chart_from_case(fixture_case: &Value, algorithm: ChartAlgorithmKind) -> Chart {
    let input = &fixture_case["input"];
    let method_profile = MethodProfile::new(
        format!(
            "iztro_2_5_8_leap_month_{}",
            fixture_case["case"].as_str().expect("case id")
        ),
        algorithm,
        "iztro 2.5.8 leap-month by_lunar fixture",
    );

    let request = LunarChartRequest::builder()
        .lunar_year(input["lunar_year"].as_i64().expect("lunar_year") as i32)
        .lunar_month(
            LunarMonth::new(input["lunar_month"].as_u64().expect("lunar_month") as u8)
                .expect("fixture lunar month should be valid"),
        )
        .lunar_day(
            LunarDay::new(input["lunar_day"].as_u64().expect("lunar_day") as u8)
                .expect("fixture lunar day should be valid"),
        )
        .birth_time(parse_key(input["birth_time"].as_str().expect("birth_time")))
        .gender(parse_key(input["gender"].as_str().expect("gender")))
        .birth_year_stem(parse_key(
            input["birth_year_stem"].as_str().expect("birth_year_stem"),
        ))
        .birth_year_branch(parse_key(
            input["birth_year_branch"]
                .as_str()
                .expect("birth_year_branch"),
        ))
        .is_leap_month(input["is_leap_month"].as_bool().expect("is_leap_month"))
        .fix_leap(input["fix_leap"].as_bool().expect("fix_leap"))
        .method_profile(method_profile)
        .build()
        .expect("fixture request should build");

    by_lunar(request).expect("by_lunar should build leap-month fixture chart")
}

/// Asserts that Rust resolves the explicit lunar date (including the leap flag)
/// to the same facts upstream did, captured in the fixture's `resolved_lunar`.
/// This is what catches invalid leap-flag handling: a request like
/// `2020-3-20, is_leap_month=true` must resolve to a non-leap third month,
/// because 2020's leap month is the fourth, not the third.
fn assert_resolved_lunar_matches(fixture_case: &Value, case_label: &str) {
    let input = &fixture_case["input"];
    let expected = &fixture_case["resolved_lunar"];

    let resolved = resolve_lunar_date(
        input["lunar_year"].as_i64().expect("lunar_year") as i32,
        LunarMonth::new(input["lunar_month"].as_u64().expect("lunar_month") as u8)
            .expect("fixture lunar month should be valid"),
        LunarDay::new(input["lunar_day"].as_u64().expect("lunar_day") as u8)
            .expect("fixture lunar day should be valid"),
        input["is_leap_month"].as_bool().expect("is_leap_month"),
    )
    .expect("fixture lunar date should resolve");

    assert_eq!(
        resolved.lunar_year(),
        expected["lunar_year"].as_i64().expect("lunar_year") as i32,
        "{case_label}: resolved lunar year mismatch"
    );
    assert_eq!(
        resolved.lunar_month().value(),
        expected["lunar_month"].as_u64().expect("lunar_month") as u8,
        "{case_label}: resolved lunar month mismatch"
    );
    assert_eq!(
        resolved.lunar_day().value(),
        expected["lunar_day"].as_u64().expect("lunar_day") as u8,
        "{case_label}: resolved lunar day mismatch"
    );
    assert_eq!(
        resolved.is_leap_month(),
        expected["is_leap_month"].as_bool().expect("is_leap_month"),
        "{case_label}: resolved leap flag mismatch"
    );
}

fn case_label(fixture_case: &Value) -> String {
    format!(
        "{} [{}]",
        fixture_case["case"].as_str().expect("case id"),
        fixture_case["algorithm"].as_str().expect("algorithm")
    )
}
