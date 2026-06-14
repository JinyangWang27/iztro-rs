//! End-to-end test for iztro `timeIndex` 0..=12 rat-hour behavior against
//! pinned upstream `iztro@2.5.8`. The fixture distinguishes early Zi (0), late
//! Zi (12), a normal non-Zi time, and the leap second-half late-Zi guard.

mod common;

use common::{
    assert_decorative_stars_match, assert_metadata_counts, assert_palaces_match,
    assert_suiqian_algorithm_boundary, assert_typed_stars_match, parse_algorithm, parse_key,
};

use iztro::core::{
    BirthTime, CalendarKind, Chart, ChartAlgorithmKind, LunarChartRequest, LunarDay, LunarMonth,
    MethodProfile, by_lunar,
};
use serde_json::Value;

const FIXTURE: &str = include_str!("../../../fixtures/iztro/time_index_rat_hour.json");

#[test]
fn by_lunar_matches_time_index_rat_hour_fixture_cases() {
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
        .expect("fixture should list rat-hour cases");
    assert_eq!(cases.len(), 6);

    for fixture_case in cases {
        let algorithm = parse_algorithm(fixture_case["algorithm"].as_str().expect("algorithm"));
        let chart = chart_from_case(fixture_case, algorithm);
        let case_label = case_label(fixture_case);
        let supported = &fixture_case["supported_fields"];

        assert_metadata_counts();
        assert_resolved_lunar_matches(&chart, fixture_case, &case_label);
        assert_birth_time_variant_matches(&chart, fixture_case, &case_label);
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
            "iztro_2_5_8_time_index_{}",
            fixture_case["case"].as_str().expect("case id")
        ),
        algorithm,
        "iztro 2.5.8 time-index rat-hour by_lunar fixture",
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
        .iztro_time_index(
            input["iztro_time_index"]
                .as_u64()
                .expect("iztro_time_index") as u8,
        )
        .expect("fixture time index should be valid")
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

    by_lunar(request).expect("by_lunar should build rat-hour fixture chart")
}

fn assert_resolved_lunar_matches(chart: &Chart, fixture_case: &Value, case_label: &str) {
    let expected = &fixture_case["resolved_lunar"];
    let date = chart.birth_context().date();

    assert_eq!(
        date.kind(),
        CalendarKind::Lunar,
        "{case_label}: by_lunar should record a lunar birth date"
    );
    assert_eq!(
        date.year(),
        expected["lunar_year"].as_i64().expect("lunar_year") as i32,
        "{case_label}: resolved lunar year mismatch"
    );
    assert_eq!(
        date.month(),
        expected["lunar_month"].as_u64().expect("lunar_month") as u8,
        "{case_label}: resolved lunar month mismatch"
    );
    assert_eq!(
        date.day(),
        expected["lunar_day"].as_u64().expect("lunar_day") as u8,
        "{case_label}: resolved lunar day mismatch"
    );
}

fn assert_birth_time_variant_matches(chart: &Chart, fixture_case: &Value, case_label: &str) {
    let input = &fixture_case["input"];
    let expected = BirthTime::from_iztro_time_index(
        input["iztro_time_index"]
            .as_u64()
            .expect("iztro_time_index") as u8,
    )
    .expect("fixture time index should be valid");

    assert_eq!(
        chart.birth_context().birth_time_variant(),
        expected,
        "{case_label}: birth time variant mismatch"
    );
    assert_eq!(
        chart.birth_context().birth_time(),
        expected.branch(),
        "{case_label}: birth time branch mismatch"
    );
}

fn case_label(fixture_case: &Value) -> String {
    format!(
        "{} [{}]",
        fixture_case["case"].as_str().expect("case id"),
        fixture_case["algorithm"].as_str().expect("algorithm")
    )
}
