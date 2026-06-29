//! Gregorian-to-Chinese-lunisolar conversion backed by `lunar-lite`.
//!
//! This module maps `lunar-lite` date values onto the crate's own typed lunar
//! facts, exposing only in-range month/day domain types to callers.
//!
//! `lunar-lite` provides the [`FourPillars`] value object, but only its
//! `daily`/`hourly` pillars are taken from `lunar-lite` directly. The `yearly`
//! and `monthly` pillars are recomputed by `iztro-rs` (see [`year_boundary`])
//! so that [`YearBoundary::LiChun`] is **datetime-level**: `lunar-lite`'s
//! [`YearDivide::Exact`] resolves 立春 at date granularity for upstream
//! compatibility, whereas `iztro-rs` compares the exact 立春 instant from
//! [`lunar_lite::li_chun_datetime`].
//!
//! The lunar year/month/day and leap-month flag always use the lunar-new-year
//! boundary (they describe the lunisolar calendar position, not the cyclic
//! year), matching pinned upstream `iztro@2.5.8`. Under
//! [`YearBoundary::ChineseNewYearEve`] the recomputed year/month pillars agree
//! with the converted lunar-year stem-branch; under [`YearBoundary::LiChun`]
//! they follow the exact 立春 instant.

use lunar_lite::{
    FourPillars, LunarError, MonthDivide, SolarDate, StemBranchOptions, YearDivide,
    four_pillars_from_solar_date_with_options, lunar_month_days,
    solar_to_lunar as convert_solar_to_lunar,
};

use super::facts::{
    LunarConversion, LunarDateInfo, ResolvedSolarClock, ResolvedSolarMoment, YearBoundaryInput,
};
use super::year_boundary;
use crate::core::calculation::YearBoundary;
use crate::core::error::ChartError;
use crate::core::model::calendar::{SolarDay, SolarMonth};
use crate::core::placement::natal::life_body::{LunarDay, LunarMonth};

/// The representative clock time `lunar-lite` synthesizes for a 时辰 index when
/// no exact clock time is available: `hour = max(time_index * 2 - 1, 0)`,
/// `minute = 30`, `second = 0`.
///
/// This makes the legacy `BirthTime` / `timeIndex` APIs compare the 立春
/// boundary against the 时辰 midpoint: `EarlyZi`/`timeIndex = 0` → `00:30`,
/// `Chou`/`timeIndex = 1` → `01:30`, ..., `LateZi`/`timeIndex = 12` → `23:30`.
fn synthesized_clock_for_time_index(time_index: u8) -> ResolvedSolarClock {
    let hour = (i32::from(time_index) * 2 - 1).max(0) as u8;
    ResolvedSolarClock {
        hour,
        minute: 30,
        second: 0,
    }
}

/// Converts a Gregorian/solar date to typed Chinese-lunisolar facts.
pub(crate) fn solar_to_lunar(
    year: i32,
    month: SolarMonth,
    day: SolarDay,
    time_index: u8,
) -> Result<LunarConversion, ChartError> {
    solar_to_lunar_with_year_boundary(
        year,
        month,
        day,
        time_index,
        YearBoundary::ChineseNewYearEve,
    )
}

/// Converts a Gregorian/solar date to typed Chinese-lunisolar facts, resolving
/// the birth-year pillar through the supplied 年分界 calculation policy.
///
/// This legacy / time-index entry point carries no exact clock time, so for the
/// datetime-level [`YearBoundary::LiChun`] comparison it synthesizes the 时辰
/// midpoint exactly as `lunar-lite` does (see [`synthesized_clock_for_time_index`]:
/// `hour = max(time_index * 2 - 1, 0)`, `minute = 30`, `second = 0`). Clock-time
/// callers that hold the exact resolved hour/minute should use
/// [`solar_to_lunar_with_resolved_datetime`] instead.
///
/// The lunar year/month/day and leap-month flag always use the lunar-new-year
/// boundary (they describe the lunisolar calendar position, not the cyclic
/// year). Only the birth-year stem/branch and the four-pillar year/month pillars
/// follow `year_boundary`: [`YearBoundary::LiChun`] re-resolves them across the
/// exact 立春 instant, while [`YearBoundary::ChineseNewYearEve`] reproduces the
/// lunar-new-year-bounded result.
pub(crate) fn solar_to_lunar_with_year_boundary(
    year: i32,
    month: SolarMonth,
    day: SolarDay,
    time_index: u8,
    year_boundary: YearBoundary,
) -> Result<LunarConversion, ChartError> {
    let clock = synthesized_clock_for_time_index(time_index);
    convert(year, month, day, time_index, clock, year_boundary)
}

/// Converts a Gregorian/solar date to typed Chinese-lunisolar facts using the
/// exact resolved clock `(hour, minute)` (seconds = `0`) for the datetime-level
/// [`YearBoundary::LiChun`] comparison.
///
/// Clock-time facade APIs preserve the resolved hour/minute through this entry
/// point, so two births on the same 立春 day split at the exact 立春 instant. The
/// `time_index` is still used for chart placement and the hour pillar.
pub(crate) fn solar_to_lunar_with_resolved_datetime(
    year: i32,
    month: SolarMonth,
    day: SolarDay,
    time_index: u8,
    hour: u8,
    minute: u8,
    year_boundary: YearBoundary,
) -> Result<LunarConversion, ChartError> {
    convert(
        year,
        month,
        day,
        time_index,
        ResolvedSolarClock {
            hour,
            minute,
            second: 0,
        },
        year_boundary,
    )
}

/// Core conversion shared by the time-index and resolved-datetime entry points.
///
/// `lunar-lite`'s four-pillars are used **only** for the daily/hourly pillars;
/// the yearly and monthly pillars are recomputed by `iztro-rs` (via
/// [`year_boundary`]) so that [`YearBoundary::LiChun`] is datetime-level
/// (`lunar-lite`'s [`YearDivide::Exact`] is date-level for upstream
/// compatibility). The `(hour, minute, second)` clock drives only the 立春
/// comparison.
fn convert(
    year: i32,
    month: SolarMonth,
    day: SolarDay,
    time_index: u8,
    clock: ResolvedSolarClock,
    year_boundary: YearBoundary,
) -> Result<LunarConversion, ChartError> {
    let conversion_failed = || ChartError::CalendarConversionFailed {
        year,
        month: month.value(),
        day: day.value(),
    };

    let solar = SolarDate {
        year,
        month: month.value(),
        day: day.value(),
    };

    let lunar = convert_solar_to_lunar(solar)
        .map_err(|err| map_solar_conversion_error(err, year, month.value(), day.value()))?;

    // `lunar-lite`'s four-pillars supply only the day/hour pillars here; the
    // year and month pillars are recomputed below for the configured boundary.
    let base = four_pillars_from_solar_date_with_options(
        solar,
        time_index,
        StemBranchOptions {
            year: YearDivide::Normal,
            month: MonthDivide::Normal,
        },
    )
    .map_err(|err| map_solar_conversion_error(err, year, month.value(), day.value()))?;

    let lunar_month = LunarMonth::new(lunar.month).map_err(|_| conversion_failed())?;
    let lunar_day = LunarDay::new(lunar.day).map_err(|_| conversion_failed())?;

    let solar_moment = ResolvedSolarMoment::new(year, month.value(), day.value(), clock);
    let yearly = year_boundary::effective_birth_year(YearBoundaryInput {
        lunar_year: lunar.year,
        solar_moment,
        boundary: year_boundary,
    })
    .map_err(|err| map_solar_conversion_error(err, year, month.value(), day.value()))?;
    let monthly = year_boundary::normal_month_pillar(
        yearly.stem(),
        lunar.month,
        lunar.is_leap_month,
        lunar.day,
    );

    let four_pillars = FourPillars {
        yearly,
        monthly,
        daily: base.daily,
        hourly: base.hourly,
    };

    Ok(LunarConversion {
        lunar_year: lunar.year,
        lunar_month,
        lunar_day,
        is_leap_month: lunar.is_leap_month,
        birth_year_stem: yearly.stem(),
        birth_year_branch: yearly.branch(),
        four_pillars,
    })
}

/// Returns the lunar-new-year-bounded lunar facts for a Gregorian/solar date.
///
/// Shared by the full-horoscope stack builder, which needs the target lunar year
/// to derive the flowing year and nominal age, without deriving four pillars.
pub(crate) fn lunar_facts(
    year: i32,
    month: SolarMonth,
    day: SolarDay,
) -> Result<LunarDateInfo, ChartError> {
    let solar = SolarDate {
        year,
        month: month.value(),
        day: day.value(),
    };
    let lunar = convert_solar_to_lunar(solar)
        .map_err(|err| map_solar_conversion_error(err, year, month.value(), day.value()))?;
    let month_day_count = lunar_month_days(lunar.year, lunar.month, lunar.is_leap_month)
        .map_err(|err| map_solar_conversion_error(err, year, month.value(), day.value()))?;

    Ok(LunarDateInfo {
        year: lunar.year,
        month: lunar.month,
        day: lunar.day,
        is_leap_month: lunar.is_leap_month,
        month_day_count,
    })
}

/// Resolves the effective cyclic birth-year stem-branch for a solar date and
/// exact clock `(hour, minute)` under a 年分界 policy.
///
/// This is the focused year-boundary resolver used by tests: it derives only the
/// year pillar and returns it. It is the fact that differs between
/// [`YearBoundary::ChineseNewYearEve`] and [`YearBoundary::LiChun`], and — for
/// [`YearBoundary::LiChun`] — between births before and after the exact 立春
/// instant on the 立春 day.
#[cfg(test)]
pub(crate) fn resolve_effective_birth_year(
    year: i32,
    month: SolarMonth,
    day: SolarDay,
    hour: u8,
    minute: u8,
    policy: YearBoundary,
) -> Result<lunar_lite::StemBranch, ChartError> {
    Ok(
        solar_to_lunar_with_resolved_datetime(year, month, day, 0, hour, minute, policy)?
            .four_pillars()
            .yearly,
    )
}

fn map_solar_conversion_error(err: LunarError, year: i32, month: u8, day: u8) -> ChartError {
    match err {
        LunarError::InvalidSolarDate { .. } => ChartError::InvalidSolarDate { year, month, day },
        LunarError::YearOutOfRange { .. } | LunarError::SolarTermOutOfRange { .. } => {
            ChartError::UnsupportedCalendarDate { year, month, day }
        }
        LunarError::InvalidLunarDate { .. }
        | LunarError::InvalidTime { .. }
        | LunarError::InvalidTimeIndex { .. } => {
            ChartError::CalendarConversionFailed { year, month, day }
        }
    }
}

#[cfg(test)]
mod tests;
