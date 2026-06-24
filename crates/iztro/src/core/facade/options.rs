//! Clock-time birth input API with an explicit calculation policy.
//!
//! The legacy [`by_solar`](crate::core::facade::by_solar::by_solar) and
//! [`by_lunar`](crate::core::facade::by_lunar::by_lunar) entry points accept a
//! 时辰 (time branch / `timeIndex`) directly and remain unchanged compatibility
//! APIs. This module adds a newer API where the user always supplies the birth
//! *clock* time together with a [`ChartCalculationConfig`].
//!
//! With the default [`ChartCalculationConfig`] (clock time), the 时辰 is derived
//! from the supplied clock time and the resulting chart matches the legacy
//! time-index API for the same 时辰. With apparent solar time enabled, the clock
//! time is normalised first (time zone + longitude) and may move to an adjacent
//! solar date before the chart is generated.

use crate::core::calculation::{
    BirthInputCalendarKind, BirthTimeResolutionSnapshot, ChartCalculationConfig,
    ChartCalculationDiagnosticSnapshot, ClockBirthTime, LeapMonthBoundary,
    LeapMonthBoundaryDiagnosticSnapshot, ResolvedBirthDateTime, SolarTimePolicy,
    SolarTimePolicyDiagnostic, YearBoundaryDiagnosticSnapshot, resolve_birth_datetime,
};
use crate::core::calendar::solar_to_lunar_with_year_boundary;
use crate::core::error::ChartError;
use crate::core::facade::by_lunar::{LunarChartRequest, by_lunar};
use crate::core::facade::by_solar::{SolarChartRequest, by_solar};
use crate::core::facade::static_temporal_chart_view::time_index_for_hour;
use crate::core::model::calendar::{BirthTime, Gender, SolarDate};
use crate::core::model::chart::Chart;
use crate::core::model::ganzhi::StemBranch;
use crate::core::model::profile::{ChartPlane, MethodProfile};
use crate::core::placement::natal::life_body::{LunarDay, LunarMonth};

/// A validated lunar (lunisolar) calendar date for the clock-time birth input
/// API.
///
/// Unlike [`LunarChartRequest`], the birth-year stem and branch are derived from
/// the lunar year here rather than supplied.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LunarDate {
    year: i32,
    month: LunarMonth,
    day: LunarDay,
    is_leap_month: bool,
}

impl LunarDate {
    /// Creates a lunar date from a year, validated month/day, and leap-month flag.
    pub const fn new(year: i32, month: LunarMonth, day: LunarDay, is_leap_month: bool) -> Self {
        Self {
            year,
            month,
            day,
            is_leap_month,
        }
    }

    /// Returns the lunar year.
    pub const fn year(self) -> i32 {
        self.year
    }

    /// Returns the validated lunar month.
    pub const fn month(self) -> LunarMonth {
        self.month
    }

    /// Returns the validated lunar day.
    pub const fn day(self) -> LunarDay {
        self.day
    }

    /// Returns whether the lunar month is a leap month (闰月).
    pub const fn is_leap_month(self) -> bool {
        self.is_leap_month
    }
}

/// Clock-time solar birth input.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SolarBirthInput {
    /// Civil solar (Gregorian) birth date.
    pub date: SolarDate,
    /// Wall-clock birth time with its civil UTC offset.
    pub birth_time: ClockBirthTime,
    /// Gender marker.
    pub gender: Gender,
}

impl SolarBirthInput {
    /// Creates a clock-time solar birth input.
    pub const fn new(date: SolarDate, birth_time: ClockBirthTime, gender: Gender) -> Self {
        Self {
            date,
            birth_time,
            gender,
        }
    }
}

/// Clock-time lunar birth input.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LunarBirthInput {
    /// Lunar (lunisolar) birth date.
    pub date: LunarDate,
    /// Wall-clock birth time with its civil UTC offset.
    pub birth_time: ClockBirthTime,
    /// Gender marker.
    pub gender: Gender,
}

impl LunarBirthInput {
    /// Creates a clock-time lunar birth input.
    pub const fn new(date: LunarDate, birth_time: ClockBirthTime, gender: Gender) -> Self {
        Self {
            date,
            birth_time,
            gender,
        }
    }
}

/// Natal chart options pairing the three independent generation axes: the
/// method profile (algorithm family), the chart plane, and the input
/// calculation config.
#[derive(Clone, Debug, PartialEq)]
pub struct NatalChartOptions {
    /// Method profile metadata (algorithm family).
    pub method_profile: MethodProfile,
    /// Requested chart plane (天盘 / 地盘 / 人盘).
    pub chart_plane: ChartPlane,
    /// Input calculation policy applied before chart generation.
    pub calculation_config: ChartCalculationConfig,
}

impl NatalChartOptions {
    /// Creates natal chart options from explicit axes.
    pub const fn new(
        method_profile: MethodProfile,
        chart_plane: ChartPlane,
        calculation_config: ChartCalculationConfig,
    ) -> Self {
        Self {
            method_profile,
            chart_plane,
            calculation_config,
        }
    }

    /// Creates natal chart options using the default clock-time calculation
    /// config and the default chart plane (天盘).
    pub fn from_method_profile(method_profile: MethodProfile) -> Self {
        Self {
            method_profile,
            chart_plane: ChartPlane::default(),
            calculation_config: ChartCalculationConfig::default(),
        }
    }
}

/// Maps the 闰月分界 calculation policy to the legacy `fix_leap` flag.
///
/// [`LeapMonthBoundary::MidMonth`] reproduces the legacy `fix_leap = true`
/// behaviour (split at the 15th, second half advances to the next month);
/// [`LeapMonthBoundary::AsPreviousMonth`] reproduces `fix_leap = false` (the whole
/// leap month keeps its numeric month).
const fn fix_leap_for(boundary: LeapMonthBoundary) -> bool {
    match boundary {
        LeapMonthBoundary::MidMonth => true,
        LeapMonthBoundary::AsPreviousMonth => false,
    }
}

/// Natal chart generation report containing the chart and calculation facts
/// resolved before chart generation.
#[derive(Clone, Debug, PartialEq)]
pub struct NatalChartGenerationReport {
    /// Generated natal chart.
    pub chart: Chart,
    /// Calculation-resolution diagnostics for this chart generation.
    pub calculation: ChartCalculationDiagnosticSnapshot,
}

/// Builds a natal chart from a clock-time solar birth input.
///
/// The clock time is resolved to a local solar date/time and 时辰 through the
/// configured [`ChartCalculationConfig`], then delegated to the legacy
/// [`by_solar`] path. No new chart-generation logic lives here.
pub fn by_solar_with_options(
    input: SolarBirthInput,
    options: NatalChartOptions,
) -> Result<Chart, ChartError> {
    Ok(by_solar_with_options_report(input, options)?.chart)
}

/// Builds a natal chart from a clock-time solar birth input and returns a
/// report with the resolved calculation facts.
pub fn by_solar_with_options_report(
    input: SolarBirthInput,
    options: NatalChartOptions,
) -> Result<NatalChartGenerationReport, ChartError> {
    let resolved =
        resolve_birth_datetime(input.date, input.birth_time, &options.calculation_config)?;
    let resolved_date = resolved.resolved_date();

    let request = SolarChartRequest::builder()
        .solar_year(resolved_date.year())
        .solar_month(resolved_date.month())
        .solar_day(resolved_date.day())
        .birth_time_variant(resolved.resolved_birth_time()?)
        .gender(input.gender)
        .year_boundary(options.calculation_config.year_boundary)
        .fix_leap(fix_leap_for(options.calculation_config.leap_month_boundary))
        .method_profile(options.method_profile.clone())
        .chart_plane(options.chart_plane)
        .build()?;

    let chart = by_solar(request)?;
    let calculation = solar_calculation_diagnostic(&options, resolved, chart.birth_year())?;

    Ok(NatalChartGenerationReport { chart, calculation })
}

/// Builds a natal chart from a clock-time lunar birth input.
///
/// The 时辰 is derived directly from the supplied clock time and delegated to the
/// legacy [`by_lunar`] path. Apparent solar time is an input calculation policy
/// defined over a civil solar date; requesting it for a lunar-date input returns
/// [`ChartError::ApparentSolarTimeRequiresSolarDate`].
pub fn by_lunar_with_options(
    input: LunarBirthInput,
    options: NatalChartOptions,
) -> Result<Chart, ChartError> {
    Ok(by_lunar_with_options_report(input, options)?.chart)
}

/// Builds a natal chart from a clock-time lunar birth input and returns a
/// report with the resolved calculation facts.
pub fn by_lunar_with_options_report(
    input: LunarBirthInput,
    options: NatalChartOptions,
) -> Result<NatalChartGenerationReport, ChartError> {
    if matches!(
        options.calculation_config.solar_time,
        SolarTimePolicy::ApparentSolarTime(_)
    ) {
        return Err(ChartError::ApparentSolarTimeRequiresSolarDate);
    }

    let time_index = time_index_for_hour(input.birth_time.hour());
    let birth_time_variant = BirthTime::from_iztro_time_index(time_index)?;
    let birth_year = StemBranch::from_lunar_year(input.date.year());

    let request = LunarChartRequest::builder()
        .lunar_year(input.date.year())
        .lunar_month(input.date.month())
        .lunar_day(input.date.day())
        .birth_time_variant(birth_time_variant)
        .gender(input.gender)
        .birth_year_stem(birth_year.stem())
        .birth_year_branch(birth_year.branch())
        .is_leap_month(input.date.is_leap_month())
        .fix_leap(fix_leap_for(options.calculation_config.leap_month_boundary))
        .method_profile(options.method_profile.clone())
        .chart_plane(options.chart_plane)
        .build()?;

    let chart = by_lunar(request)?;
    let calculation = lunar_calculation_diagnostic(input, &options)?;

    Ok(NatalChartGenerationReport { chart, calculation })
}

/// Resolves calculation facts for a solar clock-time birth input without
/// generating a chart.
pub fn resolve_solar_birth_input(
    input: SolarBirthInput,
    options: NatalChartOptions,
) -> Result<ChartCalculationDiagnosticSnapshot, ChartError> {
    let resolved =
        resolve_birth_datetime(input.date, input.birth_time, &options.calculation_config)?;
    let resolved_date = resolved.resolved_date();
    let conversion = solar_to_lunar_with_year_boundary(
        resolved_date.year(),
        resolved_date.month(),
        resolved_date.day(),
        resolved.resolved_time_index(),
        options.calculation_config.year_boundary,
    )?;
    let effective_birth_year = StemBranch::try_new(
        conversion.birth_year_stem(),
        conversion.birth_year_branch(),
    )
    .map_err(|err| match err {
        crate::core::model::ganzhi::StemBranchError::InvalidStemBranchPair { stem, branch } => {
            ChartError::InvalidStemBranchPair { stem, branch }
        }
    })?;

    solar_calculation_diagnostic(&options, resolved, effective_birth_year)
}

/// Resolves calculation facts for a lunar clock-time birth input without
/// generating a chart.
pub fn resolve_lunar_birth_input(
    input: LunarBirthInput,
    options: NatalChartOptions,
) -> Result<ChartCalculationDiagnosticSnapshot, ChartError> {
    lunar_calculation_diagnostic(input, &options)
}

fn solar_calculation_diagnostic(
    options: &NatalChartOptions,
    resolved: ResolvedBirthDateTime,
    effective_birth_year: StemBranch,
) -> Result<ChartCalculationDiagnosticSnapshot, ChartError> {
    let resolved_date = resolved.resolved_date();
    let conversion = solar_to_lunar_with_year_boundary(
        resolved_date.year(),
        resolved_date.month(),
        resolved_date.day(),
        resolved.resolved_time_index(),
        options.calculation_config.year_boundary,
    )?;

    Ok(ChartCalculationDiagnosticSnapshot {
        birth_time: solar_birth_time_diagnostic(resolved, &options.calculation_config),
        year_boundary: YearBoundaryDiagnosticSnapshot {
            policy: options.calculation_config.year_boundary,
            effective_birth_year,
        },
        leap_month_boundary: LeapMonthBoundaryDiagnosticSnapshot {
            policy: options.calculation_config.leap_month_boundary,
            legacy_fix_leap: fix_leap_for(options.calculation_config.leap_month_boundary),
            input_is_leap_month: conversion.is_leap_month(),
        },
    })
}

fn lunar_calculation_diagnostic(
    input: LunarBirthInput,
    options: &NatalChartOptions,
) -> Result<ChartCalculationDiagnosticSnapshot, ChartError> {
    if matches!(
        options.calculation_config.solar_time,
        SolarTimePolicy::ApparentSolarTime(_)
    ) {
        return Err(ChartError::ApparentSolarTimeRequiresSolarDate);
    }

    let time_index = time_index_for_hour(input.birth_time.hour());
    let birth_time_variant = BirthTime::from_iztro_time_index(time_index)?;
    let birth_year = StemBranch::from_lunar_year(input.date.year());

    Ok(ChartCalculationDiagnosticSnapshot {
        birth_time: BirthTimeResolutionSnapshot {
            input_calendar: BirthInputCalendarKind::Lunar,
            input_date: format_lunar_date(input.date),
            input_clock_time: format_clock_time(input.birth_time.hour(), input.birth_time.minute()),
            timezone_offset_minutes: input.birth_time.timezone().minutes(),
            solar_time_policy: SolarTimePolicyDiagnostic::ClockTime,
            longitude_degrees: None,
            longitude_correction_minutes: None,
            equation_of_time_minutes: None,
            total_adjustment_minutes: 0.0,
            resolved_solar_date: None,
            resolved_clock_time: format_clock_time(
                input.birth_time.hour(),
                input.birth_time.minute(),
            ),
            resolved_time_index: time_index,
            resolved_time_branch: birth_time_variant.branch(),
        },
        year_boundary: YearBoundaryDiagnosticSnapshot {
            policy: options.calculation_config.year_boundary,
            effective_birth_year: birth_year,
        },
        leap_month_boundary: LeapMonthBoundaryDiagnosticSnapshot {
            policy: options.calculation_config.leap_month_boundary,
            legacy_fix_leap: fix_leap_for(options.calculation_config.leap_month_boundary),
            input_is_leap_month: input.date.is_leap_month(),
        },
    })
}

fn solar_birth_time_diagnostic(
    resolved: ResolvedBirthDateTime,
    config: &ChartCalculationConfig,
) -> BirthTimeResolutionSnapshot {
    let solar_time_policy = solar_time_policy_diagnostic(config.solar_time);
    let longitude_degrees = match config.solar_time {
        SolarTimePolicy::ClockTime => None,
        SolarTimePolicy::ApparentSolarTime(apparent) => Some(apparent.longitude.degrees()),
    };

    BirthTimeResolutionSnapshot {
        input_calendar: BirthInputCalendarKind::Solar,
        input_date: format_solar_date(resolved.input_date()),
        input_clock_time: format_clock_time(
            resolved.input_time().hour(),
            resolved.input_time().minute(),
        ),
        timezone_offset_minutes: resolved.input_time().timezone().minutes(),
        solar_time_policy,
        longitude_degrees,
        longitude_correction_minutes: resolved.longitude_correction_minutes(),
        equation_of_time_minutes: resolved.equation_of_time_minutes(),
        total_adjustment_minutes: resolved.total_adjustment_minutes(),
        resolved_solar_date: Some(format_solar_date(resolved.resolved_date())),
        resolved_clock_time: format_clock_time(
            resolved.resolved_hour(),
            resolved.resolved_minute(),
        ),
        resolved_time_index: resolved.resolved_time_index(),
        resolved_time_branch: resolved.resolved_time_branch(),
    }
}

fn solar_time_policy_diagnostic(policy: SolarTimePolicy) -> SolarTimePolicyDiagnostic {
    match policy {
        SolarTimePolicy::ClockTime => SolarTimePolicyDiagnostic::ClockTime,
        SolarTimePolicy::ApparentSolarTime(apparent) => {
            SolarTimePolicyDiagnostic::ApparentSolarTime {
                longitude_degrees: apparent.longitude.degrees(),
                equation_of_time: apparent.equation_of_time,
            }
        }
    }
}

fn format_solar_date(date: SolarDate) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        date.year(),
        date.month().value(),
        date.day().value()
    )
}

fn format_lunar_date(date: LunarDate) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        date.year(),
        date.month().value(),
        date.day().value()
    )
}

fn format_clock_time(hour: u8, minute: u8) -> String {
    format!("{hour:02}:{minute:02}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::calculation::{NominalAgeBoundary, UtcOffset, YearBoundary};
    use crate::core::model::profile::ChartAlgorithmKind;

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
        // path for the same birth facts.
        let default_chart = lunar_chart(leap_input(16, true), LeapMonthBoundary::MidMonth);
        let legacy = by_lunar(
            LunarChartRequest::builder()
                .lunar_year(2020)
                .lunar_month(LunarMonth::new(4).expect("valid lunar month"))
                .lunar_day(LunarDay::new(16).expect("valid lunar day"))
                .birth_time_variant(
                    BirthTime::from_iztro_time_index(time_index_for_hour(8))
                        .expect("valid time index"),
                )
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
}
