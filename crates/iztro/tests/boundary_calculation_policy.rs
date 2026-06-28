//! Public integration coverage for calculation boundary policies.
//!
//! These policies are input/runtime calculation axes. They stay separate from
//! algorithm family (`ChartAlgorithmKind`) and chart plane (`ChartPlane`).

mod common;

use common::{
    CALCULATION_CONFIG_FIXTURE, assert_decorative_stars_match, assert_metadata_counts,
    assert_palaces_match, assert_suiqian_algorithm_boundary, assert_typed_stars_match,
    fixture_value, parse_algorithm, parse_key,
};

use iztro::core::{
    Chart, ChartAlgorithmKind, ChartCalculationConfig, ChartPlane, ClockBirthTime, EarthlyBranch,
    FourPillars, Gender, HeavenlyStem, HoroscopeStackInput, HoroscopeSupportedFieldsSnapshot,
    LeapMonthBoundary, LunarBirthInput, LunarChartRequest, LunarDate, LunarDay, LunarMonth,
    MethodProfile, NatalChartOptions, NominalAgeBoundary, PALACE_COUNT, PALACE_NAMES,
    SolarBirthInput, SolarChartRequest, SolarDate, SolarDay, SolarMonth, StemBranch, UtcOffset,
    YearBoundary, build_full_horoscope_chart, by_lunar, by_lunar_with_options, by_solar,
    by_solar_with_options,
};
use serde_json::Value;

#[test]
fn default_boundary_policies_preserve_existing_by_solar_output() {
    let legacy = by_solar(legacy_solar_request()).expect("legacy by_solar should build");
    let configured = by_solar_with_options(
        SolarBirthInput::new(
            SolarDate::new(1990, 6, 15).expect("valid solar date"),
            clock(8),
            Gender::Female,
        ),
        options(
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Heaven,
            ChartCalculationConfig::default(),
        ),
    )
    .expect("configured by_solar should build");

    assert_eq!(configured, legacy);
}

#[test]
fn default_boundary_policies_preserve_existing_by_lunar_output() {
    let legacy = by_lunar(legacy_lunar_request()).expect("legacy by_lunar should build");
    let configured = by_lunar_with_options(
        LunarBirthInput::new(
            LunarDate::new(
                1990,
                LunarMonth::new(5).expect("valid lunar month"),
                LunarDay::new(17).expect("valid lunar day"),
                false,
            ),
            clock(8),
            Gender::Female,
        ),
        options(
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Heaven,
            ChartCalculationConfig::default(),
        ),
    )
    .expect("configured by_lunar should build");

    assert_eq!(configured, legacy);
}

#[test]
fn year_divide_exact_2000_02_04_matches_upstream() {
    // 2000 立春 is on 2000-02-04; Chinese New Year 2000 is 2000-02-05. With the
    // `lunar-lite` calendar, `YearBoundary::LiChun` (YearDivide::Exact) resolves
    // the 立春 boundary at date granularity, matching upstream iztro@2.5.8: the
    // 立春 day belongs to the new Ganzhi year, so a birth on 2000-02-04 advances
    // to 庚辰 regardless of the clock time. The lunar-new-year-eve boundary keeps
    // the prior year 己卯 (2000-02-04 is before Chinese New Year), so the two
    // charts differ.
    let normal_chart = chart_from_case(&chart_case("year_divide_normal_2000_02_04"));
    let exact = chart_case("year_divide_exact_2000_02_04");
    let exact_chart = chart_from_case(&exact);

    assert_eq!(exact_chart.birth_year().stem(), HeavenlyStem::Geng);
    assert_eq!(exact_chart.birth_year().branch(), EarthlyBranch::Chen);
    assert_ne!(
        exact_chart, normal_chart,
        "on the 立春 day the LiChun boundary advances to the new year while the \
         lunar-new-year-eve boundary keeps the prior year",
    );
    assert_year_boundary_chart_matches_fixture(&exact_chart, &exact);
}

#[test]
fn lichun_boundary_is_date_level_within_the_lichun_day() {
    // `YearBoundary::LiChun` resolves the 立春 boundary at date granularity, so
    // two births on the 立春 day (2024-02-04) with different clock minutes share
    // the same Ganzhi year. The 立春 day belongs to the new Ganzhi year 甲辰.
    let early = by_solar_with_options(
        SolarBirthInput::new(
            SolarDate::new(2024, 2, 4).expect("valid solar date"),
            clock_at(16, 10),
            Gender::Female,
        ),
        options(
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Heaven,
            ChartCalculationConfig::default().with_year_boundary(YearBoundary::LiChun),
        ),
    )
    .expect("early-LiChun-day chart should build");
    let late = by_solar_with_options(
        SolarBirthInput::new(
            SolarDate::new(2024, 2, 4).expect("valid solar date"),
            clock_at(16, 40),
            Gender::Female,
        ),
        options(
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Heaven,
            ChartCalculationConfig::default().with_year_boundary(YearBoundary::LiChun),
        ),
    )
    .expect("late-LiChun-day chart should build");

    let jia_chen =
        StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Chen).expect("valid stem-branch");
    assert_eq!(early.birth_year(), jia_chen);
    assert_eq!(late.birth_year(), jia_chen);
    assert_eq!(
        early.birth_year(),
        late.birth_year(),
        "clock minutes within the 立春 day do not split the Ganzhi year at date granularity",
    );
}

#[test]
fn leap_month_boundary_mid_month_day_15_and_16_are_explicit() {
    let day15_mid = chart_from_case(&chart_case("leap_day_15_mid_month"));
    let day15_previous = chart_from_case(&chart_case("leap_day_15_as_previous"));
    assert_eq!(day15_mid, day15_previous, "day 15 stays in the leap month");

    let day16_mid = chart_from_case(&chart_case("leap_day_16_mid_month"));
    let day16_previous = chart_from_case(&chart_case("leap_day_16_as_previous"));
    assert_ne!(
        day16_mid, day16_previous,
        "day 16 advances only when fixLeap=true / MidMonth",
    );
}

#[test]
fn leap_month_boundary_day_1_and_final_day_are_explicit() {
    let day1_mid = chart_from_case(&chart_case("leap_day_1_mid_month"));
    let day1_previous = chart_from_case(&chart_case("leap_day_1_as_previous"));
    assert_eq!(day1_mid, day1_previous, "day 1 never advances");

    let final_mid = chart_from_case(&chart_case("leap_final_day_mid_month"));
    let final_previous = chart_from_case(&chart_case("leap_final_day_as_previous"));
    assert_ne!(
        final_mid, final_previous,
        "final leap day advances under MidMonth"
    );
}

#[test]
fn regular_month_ignores_leap_month_boundary_policy() {
    let mid = chart_from_case(&chart_case("regular_month_mid_month"));
    let previous = chart_from_case(&chart_case("regular_month_as_previous"));

    assert_eq!(mid, previous);
}

#[test]
fn nominal_age_boundary_does_not_affect_natal_chart_generation() {
    let input = LunarBirthInput::new(
        LunarDate::new(
            1990,
            LunarMonth::new(5).expect("valid lunar month"),
            LunarDay::new(17).expect("valid lunar day"),
            false,
        ),
        clock(8),
        Gender::Female,
    );
    let natural = by_lunar_with_options(
        input,
        options(
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Heaven,
            ChartCalculationConfig::default()
                .with_nominal_age_boundary(NominalAgeBoundary::NaturalYear),
        ),
    )
    .expect("natural-year chart should build");
    let birthday = by_lunar_with_options(
        input,
        options(
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Heaven,
            ChartCalculationConfig::default()
                .with_nominal_age_boundary(NominalAgeBoundary::Birthday),
        ),
    )
    .expect("birthday chart should build");

    assert_eq!(natural, birthday);
}

#[test]
fn calculation_policy_fixture_cases_match_upstream_supported_fields() {
    let fixture = fixture_value(CALCULATION_CONFIG_FIXTURE);
    assert_fixture_metadata(&fixture);

    // The 立春-day LiChun case (`year_divide_exact_2000_02_04`) now matches
    // iztro@2.5.8, so it flows through this strict upstream-parity loop like every
    // other case; `year_divide_exact_2000_02_04_matches_upstream` additionally
    // asserts its relationship to the lunar-new-year-eve chart.
    for fixture_case in fixture["cases"].as_array().expect("fixture cases") {
        match fixture_case["kind"].as_str().expect("case kind") {
            "year_divide" => {
                let chart = chart_from_case(fixture_case);
                assert_year_boundary_chart_matches_fixture(&chart, fixture_case);
            }
            "fix_leap" => {
                let chart = chart_from_case(fixture_case);
                assert_chart_matches_fixture(&chart, fixture_case);
            }
            "age_divide" => assert_nominal_age_case_matches_fixture(fixture_case),
            other => panic!("unsupported calculation config fixture kind: {other}"),
        }
    }
}

#[test]
fn representative_boundary_policy_charts_satisfy_invariants() {
    let cases = [
        (
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Heaven,
            ChartCalculationConfig::default(),
        ),
        (
            ChartAlgorithmKind::Zhongzhou,
            ChartPlane::Heaven,
            ChartCalculationConfig::default(),
        ),
        (
            ChartAlgorithmKind::Zhongzhou,
            ChartPlane::Earth,
            ChartCalculationConfig::default(),
        ),
        (
            ChartAlgorithmKind::Zhongzhou,
            ChartPlane::Human,
            ChartCalculationConfig::default(),
        ),
        (
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Heaven,
            ChartCalculationConfig::default().with_year_boundary(YearBoundary::LiChun),
        ),
        (
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Heaven,
            ChartCalculationConfig::default()
                .with_leap_month_boundary(LeapMonthBoundary::AsPreviousMonth),
        ),
        (
            ChartAlgorithmKind::QuanShu,
            ChartPlane::Heaven,
            ChartCalculationConfig::default().with_leap_month_boundary(LeapMonthBoundary::MidMonth),
        ),
        (
            ChartAlgorithmKind::Zhongzhou,
            ChartPlane::Human,
            ChartCalculationConfig::default()
                .with_year_boundary(YearBoundary::LiChun)
                .with_leap_month_boundary(LeapMonthBoundary::AsPreviousMonth)
                .with_nominal_age_boundary(NominalAgeBoundary::Birthday),
        ),
    ];

    for (algorithm, plane, config) in cases {
        let chart = by_solar_with_options(
            SolarBirthInput::new(
                SolarDate::new(2000, 2, 4).expect("valid solar date"),
                clock(8),
                Gender::Female,
            ),
            options(algorithm, plane, config),
        )
        .expect("matrix chart should build");

        assert_chart_invariants(&chart, algorithm, plane);
    }
}

fn chart_case(case_id: &str) -> Value {
    fixture_value(CALCULATION_CONFIG_FIXTURE)["cases"]
        .as_array()
        .expect("fixture cases")
        .iter()
        .find(|case| case["case"].as_str() == Some(case_id))
        .unwrap_or_else(|| panic!("missing calculation config fixture case {case_id}"))
        .clone()
}

fn chart_from_case(fixture_case: &Value) -> Chart {
    let input = &fixture_case["input"];
    let config = calculation_config(input);
    let algorithm = parse_algorithm(fixture_case["algorithm"].as_str().expect("algorithm"));

    match input["calendar"].as_str().expect("calendar") {
        "solar" => by_solar_with_options(
            SolarBirthInput::new(
                SolarDate::new(
                    input["solar_year"].as_i64().expect("solar year") as i32,
                    input["solar_month"].as_u64().expect("solar month") as u8,
                    input["solar_day"].as_u64().expect("solar day") as u8,
                )
                .expect("valid fixture solar date"),
                clock(input["clock_hour"].as_u64().expect("clock hour") as u8),
                parse_key(input["gender"].as_str().expect("gender")),
            ),
            options(algorithm, ChartPlane::Heaven, config),
        )
        .expect("solar policy fixture chart should build"),
        "lunar" => by_lunar_with_options(
            LunarBirthInput::new(
                LunarDate::new(
                    input["lunar_year"].as_i64().expect("lunar year") as i32,
                    LunarMonth::new(input["lunar_month"].as_u64().expect("lunar month") as u8)
                        .expect("valid fixture lunar month"),
                    LunarDay::new(input["lunar_day"].as_u64().expect("lunar day") as u8)
                        .expect("valid fixture lunar day"),
                    input["is_leap_month"].as_bool().expect("leap flag"),
                ),
                clock(input["clock_hour"].as_u64().expect("clock hour") as u8),
                parse_key(input["gender"].as_str().expect("gender")),
            ),
            options(algorithm, ChartPlane::Heaven, config),
        )
        .expect("lunar policy fixture chart should build"),
        other => panic!("unsupported fixture calendar: {other}"),
    }
}

fn assert_nominal_age_case_matches_fixture(fixture_case: &Value) {
    let chart = chart_from_case(fixture_case);
    let input = &fixture_case["input"];
    let target = &input["target"];
    let horoscope = build_full_horoscope_chart(
        chart,
        HoroscopeStackInput::new(
            target_solar_year(target),
            SolarMonth::new(target_solar_part(target, 1)).expect("valid target month"),
            SolarDay::new(target_solar_part(target, 2)).expect("valid target day"),
            iztro::core::BirthTime::from_iztro_time_index(
                target["time_index"].as_u64().expect("target time index") as u8,
            )
            .expect("valid target time index"),
        )
        .with_nominal_age_boundary(parse_age_divide(
            input["calculation_config"]["age_divide"]
                .as_str()
                .expect("age divide"),
        )),
    )
    .expect("full horoscope chart should build");
    let snapshot = HoroscopeSupportedFieldsSnapshot::from_horoscope_chart(&horoscope)
        .expect("supported fields snapshot should build");
    let actual = serde_json::to_value(snapshot.age()).expect("age snapshot should serialize");
    let expected = &fixture_case["supported_fields"]["age"];

    for key in [
        "index",
        "name",
        "heavenly_stem",
        "earthly_branch",
        "nominal_age",
    ] {
        assert_eq!(
            actual[key],
            expected[key],
            "{}: age field {key}",
            case_label(fixture_case)
        );
    }
    assert_eq!(
        actual["palace_names"]
            .as_array()
            .expect("actual palace names")
            .iter()
            .map(|entry| entry["name"].clone())
            .collect::<Vec<_>>(),
        expected["palace_names"]
            .as_array()
            .expect("expected palace names")
            .iter()
            .map(|entry| entry["name"].clone())
            .collect::<Vec<_>>(),
        "{}: age palace names",
        case_label(fixture_case),
    );
}

fn assert_chart_matches_fixture(chart: &Chart, fixture_case: &Value) {
    let algorithm = parse_algorithm(fixture_case["algorithm"].as_str().expect("algorithm"));
    let supported = &fixture_case["supported_fields"];
    let case_label = case_label(fixture_case);

    assert_metadata_counts();
    if fixture_case["input"]["calendar"].as_str() == Some("solar") {
        assert_converted_lunar_matches(chart, fixture_case, &case_label);
        assert_four_pillars_match(chart, &case_label);
    }
    assert_palaces_match(chart, supported, &case_label);
    assert_typed_stars_match(chart, supported, algorithm, &case_label);
    assert_decorative_stars_match(chart, supported, &case_label);
    assert_suiqian_algorithm_boundary(chart, algorithm, &case_label);
}

fn assert_year_boundary_chart_matches_fixture(chart: &Chart, fixture_case: &Value) {
    let supported = &fixture_case["supported_fields"];
    let case_label = case_label(fixture_case);

    assert_converted_lunar_matches(chart, fixture_case, &case_label);
    assert_four_pillars_match(chart, &case_label);
    assert_palaces_match(chart, supported, &case_label);
}

fn assert_converted_lunar_matches(chart: &Chart, fixture_case: &Value, case_label: &str) {
    let converted = &fixture_case["converted_lunar"];
    let date = chart.birth_context().date();

    assert_eq!(
        date.year(),
        converted["lunar_year"].as_i64().expect("lunar year") as i32,
        "{case_label}: lunar year",
    );
    assert_eq!(
        date.month(),
        converted["lunar_month"].as_u64().expect("lunar month") as u8,
        "{case_label}: lunar month",
    );
    assert_eq!(
        date.day(),
        converted["lunar_day"].as_u64().expect("lunar day") as u8,
        "{case_label}: lunar day",
    );
    assert_eq!(
        chart.birth_year().stem(),
        parse_key(
            converted["birth_year_stem"]
                .as_str()
                .expect("birth year stem")
        ),
        "{case_label}: birth year stem",
    );
    assert_eq!(
        chart.birth_year().branch(),
        parse_key(
            converted["birth_year_branch"]
                .as_str()
                .expect("birth year branch")
        ),
        "{case_label}: birth year branch",
    );
}

fn assert_four_pillars_match(chart: &Chart, case_label: &str) {
    let pillars: &FourPillars = chart
        .four_pillars()
        .unwrap_or_else(|| panic!("{case_label}: solar chart should retain four pillars"));
    assert_eq!(
        pillars.yearly,
        chart.birth_year(),
        "{case_label}: yearly pillar"
    );
}

fn assert_chart_invariants(chart: &Chart, algorithm: ChartAlgorithmKind, plane: ChartPlane) {
    assert_eq!(chart.palaces().len(), PALACE_COUNT);

    for name in PALACE_NAMES {
        assert_eq!(
            chart
                .palaces()
                .iter()
                .filter(|palace| palace.name() == name)
                .count(),
            1,
            "{name:?} should appear once",
        );
    }

    for palace in chart.palaces() {
        assert_eq!(chart.palace_by_name(palace.name()), Some(palace));
        assert_eq!(chart.palace_by_branch(palace.branch()), Some(palace));
        assert_eq!(chart.branch_of_palace(palace.name()), Some(palace.branch()));
        assert_eq!(
            chart.palace_name_at_branch(palace.branch()),
            Some(palace.name()),
        );
    }

    assert!(chart.life_palace().is_some());
    let body_branch = chart
        .body_palace_branch()
        .expect("generated chart should have body palace");
    assert!(chart.palace_by_branch(body_branch).is_some());
    assert!(chart.five_element_bureau().is_some());
    assert_eq!(chart.chart_plane(), plane);
    assert_eq!(chart.algorithm_kind(), algorithm);
    assert_eq!(chart.method_profile().algorithm_kind(), algorithm);

    for star in chart.stars() {
        let palace = star.palace();
        assert_eq!(chart.palace_by_name(palace.name()), Some(palace));
        assert_eq!(chart.palace_by_branch(palace.branch()), Some(palace));
    }

    for star in chart.decorative_stars() {
        let palace = star.palace();
        assert_eq!(chart.palace_by_name(palace.name()), Some(palace));
        assert_eq!(chart.palace_by_branch(palace.branch()), Some(palace));
    }
}

fn calculation_config(input: &Value) -> ChartCalculationConfig {
    let raw = &input["calculation_config"];
    ChartCalculationConfig::default()
        .with_year_boundary(parse_year_divide(
            raw["year_divide"].as_str().expect("year divide"),
        ))
        .with_leap_month_boundary(parse_fix_leap(raw["fix_leap"].as_bool().expect("fix leap")))
        .with_nominal_age_boundary(parse_age_divide(
            raw["age_divide"].as_str().expect("age divide"),
        ))
}

fn parse_year_divide(value: &str) -> YearBoundary {
    match value {
        "normal" => YearBoundary::ChineseNewYearEve,
        "exact" => YearBoundary::LiChun,
        other => panic!("unsupported yearDivide: {other}"),
    }
}

fn parse_fix_leap(value: bool) -> LeapMonthBoundary {
    if value {
        LeapMonthBoundary::MidMonth
    } else {
        LeapMonthBoundary::AsPreviousMonth
    }
}

fn parse_age_divide(value: &str) -> NominalAgeBoundary {
    match value {
        "normal" => NominalAgeBoundary::NaturalYear,
        "birthday" => NominalAgeBoundary::Birthday,
        other => panic!("unsupported ageDivide: {other}"),
    }
}

fn legacy_solar_request() -> SolarChartRequest {
    SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(6).expect("valid solar month"))
        .solar_day(SolarDay::new(15).expect("valid solar day"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .method_profile(method_profile(ChartAlgorithmKind::QuanShu))
        .build()
        .expect("legacy solar request should build")
}

fn legacy_lunar_request() -> LunarChartRequest {
    let birth_year = StemBranch::from_lunar_year(1990);
    LunarChartRequest::builder()
        .lunar_year(1990)
        .lunar_month(LunarMonth::new(5).expect("valid lunar month"))
        .lunar_day(LunarDay::new(17).expect("valid lunar day"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .birth_year_stem(birth_year.stem())
        .birth_year_branch(birth_year.branch())
        .is_leap_month(false)
        .method_profile(method_profile(ChartAlgorithmKind::QuanShu))
        .build()
        .expect("legacy lunar request should build")
}

fn options(
    algorithm: ChartAlgorithmKind,
    plane: ChartPlane,
    config: ChartCalculationConfig,
) -> NatalChartOptions {
    NatalChartOptions::new(method_profile(algorithm), plane, config)
}

fn method_profile(algorithm: ChartAlgorithmKind) -> MethodProfile {
    MethodProfile::new(
        format!("boundary_policy_{algorithm:?}").to_lowercase(),
        algorithm,
        "calculation boundary policy test",
    )
}

fn clock(hour: u8) -> ClockBirthTime {
    clock_at(hour, 0)
}

fn clock_at(hour: u8, minute: u8) -> ClockBirthTime {
    ClockBirthTime::new(
        hour,
        minute,
        UtcOffset::from_hours(8).expect("valid offset"),
    )
    .expect("valid clock time")
}

fn target_solar_year(target: &Value) -> i32 {
    target_solar_parts(target)[0]
}

fn target_solar_part(target: &Value, index: usize) -> u8 {
    target_solar_parts(target)[index]
        .try_into()
        .expect("target solar month/day should fit u8")
}

fn target_solar_parts(target: &Value) -> Vec<i32> {
    target["solar_date"]
        .as_str()
        .expect("target solar date")
        .split('-')
        .map(|part| part.parse().expect("target solar part"))
        .collect()
}

fn assert_fixture_metadata(fixture: &Value) {
    assert_eq!(
        fixture["metadata"]["target_package"].as_str(),
        Some("npm:iztro"),
    );
    assert_eq!(
        fixture["metadata"]["target_version"].as_str(),
        Some("2.5.8")
    );
    assert_eq!(
        fixture["metadata"]["supported_fields_only"].as_bool(),
        Some(true),
    );
}

fn case_label(fixture_case: &Value) -> String {
    fixture_case["case"].as_str().expect("case").to_owned()
}
