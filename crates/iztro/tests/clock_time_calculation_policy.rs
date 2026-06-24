//! Chart-level integration tests for the clock-time birth input API and the
//! apparent-solar-time input calculation policy.
//!
//! These exercise the new clock-time entry points against the legacy
//! time-index facade. Apparent solar time is an input calculation extension, so
//! it is tested internally rather than against upstream `iztro` parity fixtures.

use iztro::core::{
    ApparentSolarTimeConfig, Chart, ChartAlgorithmKind, ChartCalculationConfig, ChartError,
    ChartPlane, ClockBirthTime, EarthlyBranch, EquationOfTimePolicy, Gender, Longitude,
    LunarBirthInput, LunarChartRequest, LunarDate, LunarDay, LunarMonth, MethodProfile,
    NatalChartOptions, SolarBirthInput, SolarChartRequest, SolarDate, SolarDay, SolarMonth,
    SolarTimePolicy, StemBranch, UtcOffset, by_lunar, by_lunar_with_options, by_solar,
    by_solar_with_options,
};

fn quanshu_profile() -> MethodProfile {
    MethodProfile::new(
        "clock_time_policy_test",
        ChartAlgorithmKind::QuanShu,
        "clock-time calculation policy test",
    )
}

fn utc_plus_8() -> UtcOffset {
    UtcOffset::from_hours(8).expect("valid offset")
}

fn clock(hour: u8, minute: u8) -> ClockBirthTime {
    ClockBirthTime::new(hour, minute, utc_plus_8()).expect("valid clock time")
}

fn clock_time_options() -> NatalChartOptions {
    NatalChartOptions::new(
        quanshu_profile(),
        ChartPlane::Heaven,
        ChartCalculationConfig::clock_time(),
    )
}

fn apparent_options(longitude: f64) -> NatalChartOptions {
    NatalChartOptions::new(
        quanshu_profile(),
        ChartPlane::Heaven,
        ChartCalculationConfig::apparent_solar_time(ApparentSolarTimeConfig::new(
            Longitude::new(longitude).expect("valid longitude"),
            EquationOfTimePolicy::Disabled,
        )),
    )
}

fn assert_chart_invariants(chart: &Chart) {
    assert_eq!(chart.palaces().len(), 12, "a chart has twelve palaces");
    assert!(
        chart.life_palace().is_some(),
        "a generated chart has a Life Palace",
    );
    assert!(
        chart.body_palace_branch().is_some(),
        "a generated chart has a Body Palace branch",
    );
    assert!(
        chart.five_element_bureau().is_some(),
        "a generated chart has a five-element bureau",
    );
    assert!(
        chart.four_pillars().is_some(),
        "a solar-derived chart retains the natal four pillars",
    );
}

#[test]
fn new_clock_time_api_matches_legacy_time_index_api_for_same_time_branch() {
    // 08:00 at UTC+8 derives timeIndex 4 (辰时), matching a legacy Chen request.
    let profile = quanshu_profile();

    let legacy = by_solar(
        SolarChartRequest::builder()
            .solar_year(1990)
            .solar_month(SolarMonth::new(6).expect("valid month"))
            .solar_day(SolarDay::new(15).expect("valid day"))
            .birth_time(EarthlyBranch::Chen)
            .gender(Gender::Female)
            .method_profile(profile.clone())
            .build()
            .expect("legacy request should build"),
    )
    .expect("legacy by_solar should build");

    let clock_chart = by_solar_with_options(
        SolarBirthInput::new(
            SolarDate::new(1990, 6, 15).expect("valid solar date"),
            clock(8, 0),
            Gender::Female,
        ),
        NatalChartOptions::new(
            profile,
            ChartPlane::Heaven,
            ChartCalculationConfig::clock_time(),
        ),
    )
    .expect("clock-time by_solar should build");

    assert_eq!(clock_chart, legacy);
}

#[test]
fn legacy_by_solar_api_preserves_existing_output() {
    let request = || {
        SolarChartRequest::builder()
            .solar_year(1990)
            .solar_month(SolarMonth::new(6).expect("valid month"))
            .solar_day(SolarDay::new(15).expect("valid day"))
            .birth_time(EarthlyBranch::Chen)
            .gender(Gender::Female)
            .method_profile(quanshu_profile())
            .build()
            .expect("legacy request should build")
    };

    let first = by_solar(request()).expect("legacy by_solar should build");
    let second = by_solar(request()).expect("legacy by_solar should build");

    assert_eq!(first, second, "legacy by_solar stays deterministic");
    assert_eq!(first.chart_plane(), ChartPlane::Heaven);
    assert_eq!(
        first.method_profile().algorithm_kind(),
        ChartAlgorithmKind::QuanShu,
    );
    assert_chart_invariants(&first);
}

#[test]
fn legacy_by_lunar_api_preserves_existing_output() {
    let birth_year = StemBranch::from_lunar_year(1990);
    let request = || {
        LunarChartRequest::builder()
            .lunar_year(1990)
            .lunar_month(LunarMonth::new(5).expect("valid lunar month"))
            .lunar_day(LunarDay::new(17).expect("valid lunar day"))
            .iztro_time_index(4)
            .expect("valid time index")
            .gender(Gender::Female)
            .birth_year_stem(birth_year.stem())
            .birth_year_branch(birth_year.branch())
            .is_leap_month(false)
            .method_profile(quanshu_profile())
            .build()
            .expect("legacy lunar request should build")
    };

    let first = by_lunar(request()).expect("legacy by_lunar should build");
    let second = by_lunar(request()).expect("legacy by_lunar should build");

    assert_eq!(first, second, "legacy by_lunar stays deterministic");
    assert_eq!(first.palaces().len(), 12);
}

#[test]
fn lunar_clock_time_api_matches_legacy_for_same_time_branch() {
    let birth_year = StemBranch::from_lunar_year(1990);
    let profile = quanshu_profile();

    let legacy = by_lunar(
        LunarChartRequest::builder()
            .lunar_year(1990)
            .lunar_month(LunarMonth::new(5).expect("valid lunar month"))
            .lunar_day(LunarDay::new(17).expect("valid lunar day"))
            .iztro_time_index(4)
            .expect("valid time index")
            .gender(Gender::Female)
            .birth_year_stem(birth_year.stem())
            .birth_year_branch(birth_year.branch())
            .is_leap_month(false)
            .method_profile(profile.clone())
            .build()
            .expect("legacy lunar request should build"),
    )
    .expect("legacy by_lunar should build");

    let clock_chart = by_lunar_with_options(
        LunarBirthInput::new(
            LunarDate::new(
                1990,
                LunarMonth::new(5).expect("valid lunar month"),
                LunarDay::new(17).expect("valid lunar day"),
                false,
            ),
            clock(8, 0),
            Gender::Female,
        ),
        NatalChartOptions::new(
            profile,
            ChartPlane::Heaven,
            ChartCalculationConfig::clock_time(),
        ),
    )
    .expect("clock-time by_lunar should build");

    assert_eq!(clock_chart, legacy);
}

#[test]
fn apparent_solar_time_clock_api_can_change_chart_near_time_branch_boundary() {
    let date = SolarDate::new(2000, 1, 1).expect("valid solar date");

    // Clock time 01:05 derives 丑时. With longitude 105E (UTC+8) the apparent
    // solar correction is -60 minutes, moving the time to 00:05 (子时).
    let clock_chart = by_solar_with_options(
        SolarBirthInput::new(date, clock(1, 5), Gender::Female),
        clock_time_options(),
    )
    .expect("clock-time chart should build");

    let apparent_chart = by_solar_with_options(
        SolarBirthInput::new(date, clock(1, 5), Gender::Female),
        apparent_options(105.0),
    )
    .expect("apparent-solar-time chart should build");

    assert_ne!(
        clock_chart, apparent_chart,
        "apparent solar time crossing a 时辰 boundary changes the chart",
    );

    // The apparent-solar-time chart must equal the legacy chart for the
    // resolved 时辰 (子时, timeIndex 0) on the same resolved date.
    let legacy_resolved = by_solar(
        SolarChartRequest::builder()
            .solar_year(2000)
            .solar_month(SolarMonth::new(1).expect("valid month"))
            .solar_day(SolarDay::new(1).expect("valid day"))
            .birth_time(EarthlyBranch::Zi)
            .gender(Gender::Female)
            .method_profile(quanshu_profile())
            .build()
            .expect("legacy request should build"),
    )
    .expect("legacy resolved chart should build");

    assert_eq!(apparent_chart, legacy_resolved);
}

#[test]
fn apparent_solar_time_crossing_previous_day_changes_chart() {
    // 00:30 at UTC+8 with longitude 105E corrects by -60 minutes to the prior
    // day 23:30 (晚子时), differing from the same clock time read literally.
    let date = SolarDate::new(2000, 1, 1).expect("valid solar date");

    let clock_chart = by_solar_with_options(
        SolarBirthInput::new(date, clock(0, 30), Gender::Female),
        clock_time_options(),
    )
    .expect("clock-time chart should build");

    let apparent_chart = by_solar_with_options(
        SolarBirthInput::new(date, clock(0, 30), Gender::Female),
        apparent_options(105.0),
    )
    .expect("apparent-solar-time chart should build");

    assert_ne!(clock_chart, apparent_chart);
}

#[test]
fn apparent_solar_time_is_rejected_for_lunar_input() {
    let result = by_lunar_with_options(
        LunarBirthInput::new(
            LunarDate::new(
                1990,
                LunarMonth::new(5).expect("valid lunar month"),
                LunarDay::new(17).expect("valid lunar day"),
                false,
            ),
            clock(1, 5),
            Gender::Female,
        ),
        NatalChartOptions::new(
            quanshu_profile(),
            ChartPlane::Heaven,
            ChartCalculationConfig::apparent_solar_time(ApparentSolarTimeConfig::new(
                Longitude::new(105.0).expect("valid longitude"),
                EquationOfTimePolicy::Disabled,
            )),
        ),
    );

    assert_eq!(result, Err(ChartError::ApparentSolarTimeRequiresSolarDate));
}

#[test]
fn generated_clock_time_charts_satisfy_existing_invariants() {
    let date = SolarDate::new(1990, 6, 15).expect("valid solar date");

    for config in [
        ChartCalculationConfig::clock_time(),
        ChartCalculationConfig::apparent_solar_time(ApparentSolarTimeConfig::new(
            Longitude::new(116.4).expect("valid longitude"),
            EquationOfTimePolicy::Disabled,
        )),
    ] {
        let chart = by_solar_with_options(
            SolarBirthInput::new(date, clock(8, 30), Gender::Male),
            NatalChartOptions::new(quanshu_profile(), ChartPlane::Heaven, config),
        )
        .expect("clock-time chart should build");

        assert_chart_invariants(&chart);
        assert_eq!(chart.chart_plane(), ChartPlane::Heaven);
        assert_eq!(
            chart.method_profile().algorithm_kind(),
            ChartAlgorithmKind::QuanShu,
        );
    }
}

#[test]
fn default_calculation_config_is_clock_time() {
    assert_eq!(
        ChartCalculationConfig::default().solar_time,
        SolarTimePolicy::ClockTime,
    );
}
