use iztro::core::{
    ApparentSolarTimeConfig, ChartCalculationConfig, ChartError, ClockBirthTime,
    EquationOfTimePolicy, LeapMonthBoundary, Longitude, NominalAgeBoundary, SolarTimePolicy,
    UtcOffset, YearBoundary,
};

#[test]
fn longitude_accepts_in_range() {
    assert_eq!(Longitude::new(120.0).expect("valid").degrees(), 120.0);
    assert_eq!(Longitude::new(-180.0).expect("valid").degrees(), -180.0);
    assert_eq!(Longitude::new(180.0).expect("valid").degrees(), 180.0);
}

#[test]
fn longitude_rejects_out_of_range() {
    assert_eq!(
        Longitude::new(180.5),
        Err(ChartError::InvalidLongitude { value: 180.5 }),
    );
    assert_eq!(
        Longitude::new(-181.0),
        Err(ChartError::InvalidLongitude { value: -181.0 }),
    );
}

#[test]
fn utc_offset_accepts_real_world_range() {
    assert_eq!(UtcOffset::from_hours(8).expect("valid").minutes(), 480);
    assert_eq!(UtcOffset::from_hours(-12).expect("valid").minutes(), -720);
    assert_eq!(UtcOffset::from_hours(14).expect("valid").minutes(), 840);
}

#[test]
fn utc_offset_rejects_out_of_range() {
    assert_eq!(
        UtcOffset::from_minutes(841),
        Err(ChartError::InvalidUtcOffset { minutes: 841 }),
    );
    assert_eq!(
        UtcOffset::from_hours(-13),
        Err(ChartError::InvalidUtcOffset { minutes: -780 }),
    );
}

#[test]
fn utc_offset_meridian_is_offset_hours_times_fifteen() {
    assert_eq!(
        UtcOffset::from_hours(8).expect("valid").meridian_degrees(),
        120.0
    );
    assert_eq!(
        UtcOffset::from_hours(0).expect("valid").meridian_degrees(),
        0.0
    );
}

#[test]
fn clock_birth_time_accepts_valid_time() {
    let tz = UtcOffset::from_hours(8).expect("valid offset");
    let clock = ClockBirthTime::new(1, 5, tz).expect("valid clock time");
    assert_eq!(clock.hour(), 1);
    assert_eq!(clock.minute(), 5);
    assert_eq!(clock.minutes_since_midnight(), 65);
}

#[test]
fn clock_birth_time_rejects_invalid_time() {
    let tz = UtcOffset::from_hours(8).expect("valid offset");
    assert_eq!(
        ClockBirthTime::new(24, 0, tz),
        Err(ChartError::InvalidClockTime {
            hour: 24,
            minute: 0,
        }),
    );
    assert_eq!(
        ClockBirthTime::new(0, 60, tz),
        Err(ChartError::InvalidClockTime {
            hour: 0,
            minute: 60,
        }),
    );
}

#[test]
fn calculation_config_defaults_to_clock_time() {
    assert_eq!(
        ChartCalculationConfig::default().solar_time,
        SolarTimePolicy::ClockTime,
    );
}

#[test]
fn calculation_config_boundary_defaults_preserve_existing_behaviour() {
    let config = ChartCalculationConfig::default();
    assert_eq!(config.year_boundary, YearBoundary::ChineseNewYearEve);
    assert_eq!(config.leap_month_boundary, LeapMonthBoundary::MidMonth);
    assert_eq!(config.nominal_age_boundary, NominalAgeBoundary::NaturalYear);
}

#[test]
fn enum_defaults_match_existing_behaviour() {
    assert_eq!(YearBoundary::default(), YearBoundary::ChineseNewYearEve);
    assert_eq!(LeapMonthBoundary::default(), LeapMonthBoundary::MidMonth);
    assert_eq!(
        NominalAgeBoundary::default(),
        NominalAgeBoundary::NaturalYear
    );
}

#[test]
fn constructors_set_boundary_defaults() {
    for config in [
        ChartCalculationConfig::clock_time(),
        ChartCalculationConfig::new(SolarTimePolicy::ClockTime),
        ChartCalculationConfig::apparent_solar_time(ApparentSolarTimeConfig::new(
            Longitude::new(120.0).expect("valid longitude"),
            EquationOfTimePolicy::Disabled,
        )),
    ] {
        assert_eq!(config.year_boundary, YearBoundary::ChineseNewYearEve);
        assert_eq!(config.leap_month_boundary, LeapMonthBoundary::MidMonth);
        assert_eq!(config.nominal_age_boundary, NominalAgeBoundary::NaturalYear);
    }
}

#[test]
fn with_builders_replace_each_boundary() {
    let config = ChartCalculationConfig::clock_time()
        .with_year_boundary(YearBoundary::LiChun)
        .with_leap_month_boundary(LeapMonthBoundary::AsPreviousMonth)
        .with_nominal_age_boundary(NominalAgeBoundary::Birthday);
    assert_eq!(config.year_boundary, YearBoundary::LiChun);
    assert_eq!(
        config.leap_month_boundary,
        LeapMonthBoundary::AsPreviousMonth
    );
    assert_eq!(config.nominal_age_boundary, NominalAgeBoundary::Birthday);
    // Unrelated axis untouched.
    assert_eq!(config.solar_time, SolarTimePolicy::ClockTime);
}
