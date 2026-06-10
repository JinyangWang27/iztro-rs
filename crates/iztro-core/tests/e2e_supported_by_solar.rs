//! End-to-end test for the supported `by_solar` facade slice against pinned
//! upstream `iztro@2.5.8`. Builds each fixture case through `by_solar`, then
//! compares the ICU-converted lunar facts and the currently supported chart
//! fields. Shared assertions live in `common`.

mod common;

use common::{
    assert_decorative_stars_match, assert_metadata_counts, assert_palaces_match,
    assert_suiqian_algorithm_boundary, assert_typed_stars_match, parse_algorithm, parse_key,
};

use iztro_core::{
    CalendarKind, Chart, ChartAlgorithmKind, EarthlyBranch, Gender, MethodProfile,
    SolarChartRequest, SolarDay, SolarMonth, by_solar,
};
use serde_json::Value;

const FIXTURE: &str = include_str!("../../../fixtures/iztro/e2e_supported_by_solar.json");

#[test]
fn by_solar_matches_supported_e2e_fixture_cases() {
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
        .expect("fixture should list e2e cases");
    assert_eq!(cases.len(), 14);

    for fixture_case in cases {
        let algorithm = parse_algorithm(fixture_case["algorithm"].as_str().expect("algorithm"));
        let chart = chart_from_case(fixture_case, algorithm);
        let case_label = case_label(fixture_case);
        let supported = &fixture_case["supported_fields"];

        assert_metadata_counts();
        assert_converted_lunar_matches(&chart, fixture_case, &case_label);
        assert_birth_facts_match(&chart, fixture_case, &case_label);
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
            "iztro_2_5_8_e2e_supported_by_solar_{}",
            fixture_case["case"].as_str().expect("case id")
        ),
        algorithm,
        "iztro 2.5.8 supported by_solar e2e fixture",
    );

    let request = SolarChartRequest::builder()
        .solar_year(input["solar_year"].as_i64().expect("solar_year") as i32)
        .solar_month(
            SolarMonth::new(input["solar_month"].as_u64().expect("solar_month") as u8)
                .expect("fixture solar month should be valid"),
        )
        .solar_day(
            SolarDay::new(input["solar_day"].as_u64().expect("solar_day") as u8)
                .expect("fixture solar day should be valid"),
        )
        .birth_time(parse_key(input["birth_time"].as_str().expect("birth_time")))
        .gender(parse_key(input["gender"].as_str().expect("gender")))
        .fix_leap(input["fix_leap"].as_bool().expect("fix_leap"))
        .method_profile(method_profile)
        .build()
        .expect("fixture request should build");

    by_solar(request).expect("by_solar should build supported fixture chart")
}

/// Asserts the lunar date the ICU-backed conversion recorded on the chart equals
/// the upstream converted lunar year/month/day. The leap flag and birth-year
/// ganzhi are covered by the adapter unit tests and, end to end, by the palace
/// stem / minor-star assertions that depend on them.
fn assert_converted_lunar_matches(chart: &Chart, fixture_case: &Value, case_label: &str) {
    let converted = &fixture_case["converted_lunar"];
    let date = chart.birth_context().date();

    assert_eq!(
        date.kind(),
        CalendarKind::Lunar,
        "{case_label}: by_solar should record a lunar birth date"
    );
    assert_eq!(
        date.year(),
        converted["lunar_year"].as_i64().expect("lunar_year") as i32,
        "{case_label}: converted lunar year mismatch"
    );
    assert_eq!(
        date.month(),
        converted["lunar_month"].as_u64().expect("lunar_month") as u8,
        "{case_label}: converted lunar month mismatch"
    );
    assert_eq!(
        date.day(),
        converted["lunar_day"].as_u64().expect("lunar_day") as u8,
        "{case_label}: converted lunar day mismatch"
    );
}

fn assert_birth_facts_match(chart: &Chart, fixture_case: &Value, case_label: &str) {
    let supported = &fixture_case["supported_fields"];
    assert_eq!(
        chart.birth_context().birth_time(),
        parse_key::<EarthlyBranch>(supported["birth_time"].as_str().expect("birth_time")),
        "{case_label}: birth time mismatch"
    );
    assert_eq!(
        chart.birth_context().gender(),
        parse_key::<Gender>(supported["gender"].as_str().expect("gender")),
        "{case_label}: gender mismatch"
    );
}

fn case_label(fixture_case: &Value) -> String {
    format!(
        "{} [{}]",
        fixture_case["case"].as_str().expect("case id"),
        fixture_case["algorithm"].as_str().expect("algorithm")
    )
}
