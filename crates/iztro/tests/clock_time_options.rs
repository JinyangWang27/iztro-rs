use iztro::core::{
    Chart, ChartAlgorithmKind, ChartCalculationConfig, ChartPlane, ClockBirthTime, EarthlyBranch,
    Gender, LeapMonthBoundary, LunarBirthInput, LunarChartRequest, LunarDate, LunarDay, LunarMonth,
    MethodProfile, NatalChartOptions, NominalAgeBoundary, StemBranch, UtcOffset, YearBoundary,
    by_lunar, by_lunar_with_options,
};

fn quanshu_profile() -> MethodProfile {
    MethodProfile::new("quanshu_test", ChartAlgorithmKind::QuanShu, "quanshu test")
}

fn clock(hour: u8) -> ClockBirthTime {
    ClockBirthTime::new(hour, 0, UtcOffset::from_hours(8).expect("valid offset"))
        .expect("valid clock time")
}

fn options(config: ChartCalculationConfig) -> NatalChartOptions {
    NatalChartOptions::new(quanshu_profile(), ChartPlane::Heaven, config)
}

fn leap_input(day: u8, is_leap_month: bool) -> LunarBirthInput {
    // 2020 has a leap fourth month (闰四月).
    let date = LunarDate::new(
        2020,
        LunarMonth::new(4).expect("valid lunar month"),
        LunarDay::new(day).expect("valid lunar day"),
        is_leap_month,
    );
    LunarBirthInput::new(date, clock(8), Gender::Female)
}

fn lunar_chart(input: LunarBirthInput, boundary: LeapMonthBoundary) -> Chart {
    lunar_chart_with(
        input,
        ChartCalculationConfig::clock_time().with_leap_month_boundary(boundary),
    )
}

fn lunar_chart_with(input: LunarBirthInput, config: ChartCalculationConfig) -> Chart {
    by_lunar_with_options(input, options(config)).expect("lunar chart should build")
}

#[test]
fn leap_month_boundary_mid_month_day_15_and_16_are_explicit() {
    // Day 15 (inclusive) stays in the month under both policies, so the
    // charts agree. Day 16 advances only under MidMonth, so they diverge.
    let day15_mid = lunar_chart(leap_input(15, true), LeapMonthBoundary::MidMonth);
    let day15_prev = lunar_chart(leap_input(15, true), LeapMonthBoundary::AsPreviousMonth);
    assert_eq!(day15_mid, day15_prev, "day 15 must match across policies");

    let day16_mid = lunar_chart(leap_input(16, true), LeapMonthBoundary::MidMonth);
    let day16_prev = lunar_chart(leap_input(16, true), LeapMonthBoundary::AsPreviousMonth);
    assert_ne!(day16_mid, day16_prev, "day 16 must diverge across policies");
}

#[test]
fn leap_month_boundary_day_1_and_final_day() {
    // Day 1 never advances; both policies agree.
    let day1_mid = lunar_chart(leap_input(1, true), LeapMonthBoundary::MidMonth);
    let day1_prev = lunar_chart(leap_input(1, true), LeapMonthBoundary::AsPreviousMonth);
    assert_eq!(day1_mid, day1_prev);

    // A late leap-month day (29) is in the second half, so MidMonth advances
    // while AsPreviousMonth does not.
    let final_mid = lunar_chart(leap_input(29, true), LeapMonthBoundary::MidMonth);
    let final_prev = lunar_chart(leap_input(29, true), LeapMonthBoundary::AsPreviousMonth);
    assert_ne!(final_mid, final_prev);
}

#[test]
fn regular_month_ignores_leap_month_boundary_policy() {
    // Without the leap flag, the month is never advanced regardless of the
    // boundary policy.
    let mid = lunar_chart(leap_input(16, false), LeapMonthBoundary::MidMonth);
    let prev = lunar_chart(leap_input(16, false), LeapMonthBoundary::AsPreviousMonth);
    assert_eq!(mid, prev);
}

#[test]
fn default_calculation_config_preserves_existing_output() {
    // The default options path must equal the legacy fix_leap = true / Heaven
    // path for the same birth facts. Clock hour 8 resolves to 辰时 (Chen).
    let default_chart = lunar_chart(leap_input(16, true), LeapMonthBoundary::MidMonth);
    let legacy = by_lunar(
        LunarChartRequest::builder()
            .lunar_year(2020)
            .lunar_month(LunarMonth::new(4).expect("valid lunar month"))
            .lunar_day(LunarDay::new(16).expect("valid lunar day"))
            .birth_time(EarthlyBranch::Chen)
            .gender(Gender::Female)
            .birth_year_stem(StemBranch::from_lunar_year(2020).stem())
            .birth_year_branch(StemBranch::from_lunar_year(2020).branch())
            .is_leap_month(true)
            .fix_leap(true)
            .method_profile(quanshu_profile())
            .build()
            .expect("legacy request should build"),
    )
    .expect("legacy chart should build");
    assert_eq!(default_chart, legacy);
}

#[test]
fn nominal_age_boundary_does_not_affect_natal_chart_generation() {
    // The 虚岁分界 policy is a runtime/horoscope concern; two natal options
    // differing only in nominal_age_boundary must produce identical charts.
    let natural = lunar_chart_with(
        leap_input(10, true),
        ChartCalculationConfig::clock_time()
            .with_nominal_age_boundary(NominalAgeBoundary::NaturalYear),
    );
    let birthday = lunar_chart_with(
        leap_input(10, true),
        ChartCalculationConfig::clock_time()
            .with_nominal_age_boundary(NominalAgeBoundary::Birthday),
    );
    assert_eq!(natural, birthday);
}

#[test]
fn year_boundary_does_not_leak_into_lunar_path() {
    // by_lunar_with_options derives its year from the lunar year directly, so
    // the year-boundary policy is inert there; the charts stay equal.
    let eve = by_lunar_with_options(
        leap_input(10, true),
        options(
            ChartCalculationConfig::clock_time()
                .with_year_boundary(YearBoundary::ChineseNewYearEve),
        ),
    )
    .expect("chart should build");
    let li_chun = by_lunar_with_options(
        leap_input(10, true),
        options(ChartCalculationConfig::clock_time().with_year_boundary(YearBoundary::LiChun)),
    )
    .expect("chart should build");
    assert_eq!(eve, li_chun);
}
