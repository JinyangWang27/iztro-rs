//! Public integration tests for calculation diagnostics and generation reports.

use iztro::core::{
    ApparentSolarTimeConfig, BirthInputCalendarKind, BirthTime, ChartAlgorithmKind,
    ChartCalculationConfig, ChartError, ChartPlane, ClockBirthTime, EarthlyBranch,
    EquationOfTimePolicy, Gender, HeavenlyStem, HoroscopeChart, HoroscopeStackInput,
    LeapMonthBoundary, Longitude, LunarBirthInput, LunarDate, LunarDay, LunarMonth, MethodProfile,
    NatalChartOptions, NominalAgeBoundary, Scope, SolarBirthInput, SolarDate, SolarDay, SolarMonth,
    SolarTimePolicyDiagnostic, StemBranch, TemporalContext, UtcOffset, YearBoundary,
    build_full_horoscope_chart, build_full_horoscope_chart_report, by_lunar_with_options,
    by_lunar_with_options_report, by_solar_with_options, by_solar_with_options_report,
};

fn quanshu_profile() -> MethodProfile {
    MethodProfile::new(
        "calculation_diagnostics_test",
        ChartAlgorithmKind::QuanShu,
        "calculation diagnostics test",
    )
}

fn utc_plus_8() -> UtcOffset {
    UtcOffset::from_hours(8).expect("valid offset")
}

fn clock(hour: u8, minute: u8) -> ClockBirthTime {
    ClockBirthTime::new(hour, minute, utc_plus_8()).expect("valid clock time")
}

fn options(config: ChartCalculationConfig) -> NatalChartOptions {
    NatalChartOptions::new(quanshu_profile(), ChartPlane::Heaven, config)
}

fn solar_input(year: i32, month: u8, day: u8, hour: u8, minute: u8) -> SolarBirthInput {
    SolarBirthInput::new(
        SolarDate::new(year, month, day).expect("valid solar date"),
        clock(hour, minute),
        Gender::Female,
    )
}

fn lunar_input() -> LunarBirthInput {
    LunarBirthInput::new(
        LunarDate::new(
            1990,
            LunarMonth::new(5).expect("valid lunar month"),
            LunarDay::new(17).expect("valid lunar day"),
            false,
        ),
        clock(8, 0),
        Gender::Female,
    )
}

fn apparent_config(longitude: f64) -> ChartCalculationConfig {
    ChartCalculationConfig::apparent_solar_time(ApparentSolarTimeConfig::new(
        Longitude::new(longitude).expect("valid longitude"),
        EquationOfTimePolicy::Disabled,
    ))
}

fn assert_float_eq(actual: Option<f64>, expected: f64) {
    let actual = actual.expect("diagnostic field should be present");
    assert!(
        (actual - expected).abs() < f64::EPSILON,
        "expected {expected}, got {actual}",
    );
}

fn nominal_age(chart: &HoroscopeChart) -> u8 {
    chart
        .layers_in_scope(Scope::Age)
        .find_map(|layer| match layer.context() {
            TemporalContext::Age { nominal_age, .. } => Some(*nominal_age),
            _ => None,
        })
        .expect("horoscope should expose nominal-age layer")
}

fn natal_for_horoscope() -> iztro::core::Chart {
    by_solar_with_options(
        solar_input(1985, 2, 15, 8, 0),
        options(ChartCalculationConfig::clock_time()),
    )
    .expect("natal chart should build")
}

fn horoscope_input(boundary: NominalAgeBoundary) -> HoroscopeStackInput {
    HoroscopeStackInput::new(
        2000,
        SolarMonth::new(6).expect("valid solar month"),
        SolarDay::new(1).expect("valid solar day"),
        BirthTime::from_iztro_time_index(2).expect("valid time index"),
    )
    .with_nominal_age_boundary(boundary)
}

#[test]
fn solar_report_matches_plain_chart_api() {
    let input = solar_input(1990, 6, 15, 8, 0);
    let opts = options(ChartCalculationConfig::default());

    let report = by_solar_with_options_report(input, opts.clone()).expect("report should build");
    let plain = by_solar_with_options(input, opts).expect("plain chart should build");

    assert_eq!(report.chart, plain);
}

#[test]
fn lunar_report_matches_plain_chart_api() {
    let input = lunar_input();
    let opts = options(ChartCalculationConfig::default());

    let report = by_lunar_with_options_report(input, opts.clone()).expect("report should build");
    let plain = by_lunar_with_options(input, opts).expect("plain chart should build");

    assert_eq!(report.chart, plain);
}

#[test]
fn apparent_solar_time_report_records_longitude_correction() {
    let report = by_solar_with_options_report(
        solar_input(2000, 1, 1, 1, 5),
        options(apparent_config(105.0)),
    )
    .expect("report should build");

    let birth_time = &report.calculation.birth_time;
    assert_eq!(birth_time.input_calendar, BirthInputCalendarKind::Solar);
    assert_eq!(birth_time.input_date, "2000-01-01");
    assert_eq!(birth_time.input_clock_time, "01:05");
    assert_eq!(birth_time.timezone_offset_minutes, 480);
    assert_eq!(
        birth_time.solar_time_policy,
        SolarTimePolicyDiagnostic::ApparentSolarTime {
            longitude_degrees: 105.0,
            equation_of_time: EquationOfTimePolicy::Disabled,
        },
    );
    assert_eq!(birth_time.longitude_degrees, Some(105.0));
    assert_float_eq(birth_time.longitude_correction_minutes, -60.0);
    assert_eq!(birth_time.equation_of_time_minutes, Some(0.0));
    assert_eq!(birth_time.total_adjustment_minutes, -60.0);
    assert_eq!(
        birth_time.resolved_solar_date.as_deref(),
        Some("2000-01-01")
    );
    assert_eq!(birth_time.resolved_clock_time, "00:05");
    assert_eq!(birth_time.resolved_time_index, 0);
    assert_eq!(birth_time.resolved_time_branch, EarthlyBranch::Zi);
}

#[test]
fn apparent_solar_time_report_records_previous_day_crossing() {
    let report = by_solar_with_options_report(
        solar_input(2000, 1, 1, 0, 30),
        options(apparent_config(105.0)),
    )
    .expect("report should build");

    let birth_time = &report.calculation.birth_time;
    assert_eq!(
        birth_time.resolved_solar_date.as_deref(),
        Some("1999-12-31")
    );
    assert_eq!(birth_time.resolved_clock_time, "23:30");
    assert_eq!(birth_time.resolved_time_index, 12);
    assert_eq!(birth_time.resolved_time_branch, EarthlyBranch::Zi);
}

#[test]
fn year_boundary_report_records_effective_birth_year() {
    // 2000-02-04 is the 立春 day, before Chinese New Year 2000 (02-05). With
    // date-level `YearBoundary::LiChun` the two policies differ here regardless of
    // the clock time: the lunar-new-year-eve boundary keeps the prior year 己卯
    // while the LiChun boundary advances to 庚辰 (the 立春 day belongs to the new
    // Ganzhi year).
    let normal = by_solar_with_options_report(
        solar_input(2000, 2, 4, 21, 0),
        options(
            ChartCalculationConfig::default().with_year_boundary(YearBoundary::ChineseNewYearEve),
        ),
    )
    .expect("normal boundary report should build");
    let exact = by_solar_with_options_report(
        solar_input(2000, 2, 4, 21, 0),
        options(ChartCalculationConfig::default().with_year_boundary(YearBoundary::LiChun)),
    )
    .expect("li chun boundary report should build");

    let ji_mao = StemBranch::try_new(HeavenlyStem::Ji, EarthlyBranch::Mao).expect("valid pair");
    let geng_chen =
        StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Chen).expect("valid pair");

    assert_eq!(
        normal.calculation.year_boundary.effective_birth_year,
        normal.chart.birth_year(),
    );
    assert_eq!(
        exact.calculation.year_boundary.effective_birth_year,
        exact.chart.birth_year(),
    );
    assert_eq!(
        normal.calculation.year_boundary.effective_birth_year,
        ji_mao
    );
    assert_eq!(
        exact.calculation.year_boundary.effective_birth_year,
        geng_chen
    );
    assert_ne!(
        normal.calculation.year_boundary.effective_birth_year,
        exact.calculation.year_boundary.effective_birth_year,
    );
}

#[test]
fn leap_month_report_records_policy_and_fix_leap_mapping() {
    let input = LunarBirthInput::new(
        LunarDate::new(
            2020,
            LunarMonth::new(4).expect("valid lunar month"),
            LunarDay::new(16).expect("valid lunar day"),
            true,
        ),
        clock(8, 0),
        Gender::Female,
    );

    let mid = by_lunar_with_options_report(
        input,
        options(
            ChartCalculationConfig::default().with_leap_month_boundary(LeapMonthBoundary::MidMonth),
        ),
    )
    .expect("mid-month report should build");
    let previous = by_lunar_with_options_report(
        input,
        options(
            ChartCalculationConfig::default()
                .with_leap_month_boundary(LeapMonthBoundary::AsPreviousMonth),
        ),
    )
    .expect("as-previous report should build");

    assert_eq!(
        mid.calculation.leap_month_boundary.policy,
        LeapMonthBoundary::MidMonth
    );
    assert!(mid.calculation.leap_month_boundary.legacy_fix_leap);
    assert!(mid.calculation.leap_month_boundary.input_is_leap_month);

    assert_eq!(
        previous.calculation.leap_month_boundary.policy,
        LeapMonthBoundary::AsPreviousMonth,
    );
    assert!(!previous.calculation.leap_month_boundary.legacy_fix_leap);
    assert!(previous.calculation.leap_month_boundary.input_is_leap_month);
}

#[test]
fn lunar_apparent_solar_time_report_is_rejected() {
    let err = by_lunar_with_options_report(lunar_input(), options(apparent_config(105.0)))
        .expect_err("lunar apparent solar time should be rejected");

    assert_eq!(err, ChartError::ApparentSolarTimeRequiresSolarDate);
}

#[test]
fn horoscope_report_matches_plain_horoscope_api() {
    let input = horoscope_input(NominalAgeBoundary::NaturalYear);
    let report = build_full_horoscope_chart_report(natal_for_horoscope(), input)
        .expect("report should build");
    let plain = build_full_horoscope_chart(natal_for_horoscope(), input)
        .expect("plain horoscope should build");

    assert_eq!(report.horoscope, plain);
}

#[test]
fn horoscope_report_records_nominal_age_boundary_and_age() {
    let natural = build_full_horoscope_chart_report(
        natal_for_horoscope(),
        horoscope_input(NominalAgeBoundary::NaturalYear),
    )
    .expect("natural-year report should build");
    let birthday = build_full_horoscope_chart_report(
        natal_for_horoscope(),
        horoscope_input(NominalAgeBoundary::Birthday),
    )
    .expect("birthday report should build");

    assert_eq!(
        natural.calculation.nominal_age_boundary,
        NominalAgeBoundary::NaturalYear,
    );
    assert_eq!(
        birthday.calculation.nominal_age_boundary,
        NominalAgeBoundary::Birthday,
    );
    assert_eq!(
        natural.calculation.resolved_nominal_age,
        nominal_age(&natural.horoscope),
    );
    assert_eq!(
        birthday.calculation.resolved_nominal_age,
        nominal_age(&birthday.horoscope),
    );
    assert_eq!(
        natural.calculation.resolved_nominal_age,
        birthday.calculation.resolved_nominal_age + 1,
    );
}

#[test]
fn chart_json_remains_unchanged_by_generation_report() {
    let input = solar_input(1990, 6, 15, 8, 0);
    let opts = options(ChartCalculationConfig::default());

    let report = by_solar_with_options_report(input, opts.clone()).expect("report should build");
    let plain = by_solar_with_options(input, opts).expect("plain chart should build");

    assert_eq!(
        serde_json::to_value(&report.chart).expect("report chart should serialize"),
        serde_json::to_value(&plain).expect("plain chart should serialize"),
    );
}
