//! Chart builders that translate a fixture `input` block into a built `Chart`
//! through the public facade.

use iztro::core::{
    Chart, LunarChartRequest, LunarDay, LunarMonth, MethodProfile, StemBranch, by_lunar,
};
use serde_json::Value;

use super::normalize::{parse_algorithm, parse_gender};

/// Builds a natal chart from a fixture `input` block through `by_lunar`.
///
/// `case_id` is recorded on the `MethodProfile` for diagnostics. The block must
/// carry the explicit lunar facade inputs (`year`, `month`, `day`,
/// `time_index`, `gender`, `algorithm`, `is_leap_month`, `fix_leap`).
pub fn build_chart_from_lunar_input(case_id: &str, input: &Value) -> Chart {
    assert_eq!(
        input["calendar"].as_str(),
        Some("lunar"),
        "horoscope fixtures should build through by_lunar"
    );
    let lunar_year = input["year"].as_i64().expect("fixture lunar year") as i32;
    let birth_year = StemBranch::from_lunar_year(lunar_year);
    let method_profile = MethodProfile::new(
        case_id,
        parse_algorithm(input["algorithm"].as_str().expect("algorithm")),
        "horoscope fixture test",
    );
    let request = LunarChartRequest::builder()
        .lunar_year(lunar_year)
        .lunar_month(
            LunarMonth::new(input["month"].as_u64().expect("fixture lunar month") as u8)
                .expect("fixture lunar month should be valid"),
        )
        .lunar_day(
            LunarDay::new(input["day"].as_u64().expect("fixture lunar day") as u8)
                .expect("fixture lunar day should be valid"),
        )
        .iztro_time_index(input["time_index"].as_u64().expect("fixture time index") as u8)
        .expect("fixture time index should be valid")
        .gender(parse_gender(
            input["gender"].as_str().expect("fixture gender"),
        ))
        .birth_year_stem(birth_year.stem())
        .birth_year_branch(birth_year.branch())
        .is_leap_month(
            input["is_leap_month"]
                .as_bool()
                .expect("fixture leap-month flag"),
        )
        .fix_leap(input["fix_leap"].as_bool().expect("fixture fix-leap flag"))
        .method_profile(method_profile)
        .build()
        .expect("lunar chart request should build from fixture");

    by_lunar(request).expect("by_lunar should build horoscope fixture chart")
}

/// Builds the natal chart for a horoscope fixture case through `by_lunar`.
pub fn build_chart_from_horoscope_fixture_case(case: &Value) -> Chart {
    build_chart_from_lunar_input(case["id"].as_str().expect("case id"), &case["input"])
}
