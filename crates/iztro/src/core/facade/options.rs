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

use crate::core::calculation::{ChartCalculationConfig, ClockBirthTime, resolve_birth_datetime};
use crate::core::error::ChartError;
use crate::core::facade::by_lunar::{LunarChartRequest, by_lunar};
use crate::core::facade::by_solar::{SolarChartRequest, by_solar};
use crate::core::facade::static_temporal_chart_view::time_index_for_hour;
use crate::core::model::calendar::{BirthTime, Gender, SolarDate};
use crate::core::model::chart::Chart;
use crate::core::model::profile::{ChartPlane, MethodProfile};
use crate::core::placement::natal::life_body::{LunarDay, LunarMonth};
use lunar_lite::StemBranch;

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

/// Builds a natal chart from a clock-time solar birth input.
///
/// The clock time is resolved to a local solar date/time and 时辰 through the
/// configured [`ChartCalculationConfig`], then delegated to the legacy
/// [`by_solar`] path. No new chart-generation logic lives here.
pub fn by_solar_with_options(
    input: SolarBirthInput,
    options: NatalChartOptions,
) -> Result<Chart, ChartError> {
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
        .method_profile(options.method_profile)
        .chart_plane(options.chart_plane)
        .build()?;

    by_solar(request)
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
    use crate::core::calculation::SolarTimePolicy;

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
        .method_profile(options.method_profile)
        .chart_plane(options.chart_plane)
        .build()?;

    by_lunar(request)
}
