//! Gregorian-to-Chinese-lunisolar conversion backed by the internal calendar
//! adapter.
//!
//! This module maps the adapter's calendar facts onto the crate's own typed
//! lunar facts and owned GanZhi value objects, exposing only in-range month/day
//! domain types to callers. The four pillars are assembled by [`super::policy`]:
//! the day and hour pillars come from the internal calendar adapter, while the
//! year and month pillars follow the lunar-new-year / 五虎遁 conventions for
//! `iztro@2.5.8` parity. The year pillar (the retained birth-year fact) follows
//! the configured [`YearBoundary`].

use crate::core::calculation::YearBoundary;
use crate::core::error::ChartError;
use crate::core::model::calendar::{SolarDate, SolarDay, SolarMonth};
use crate::core::model::ganzhi::{EarthlyBranch, FourPillars, HeavenlyStem};
use crate::core::placement::natal::life_body::{LunarDay, LunarMonth};

use super::policy::resolve_four_pillars;
use super::tyme::{LunarDateInfo, ResolvedSolarDateTime, TymeCalendar};

/// Typed lunar facts produced from a Gregorian/solar date.
///
/// Calendar-backend date/error types stay internal; the birth-year stem/branch
/// and four pillars use the crate's own GanZhi value objects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LunarConversion {
    lunar_year: i32,
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    is_leap_month: bool,
    birth_year_stem: HeavenlyStem,
    birth_year_branch: EarthlyBranch,
    four_pillars: FourPillars,
}

impl LunarConversion {
    /// Returns the converted lunar year (the lunar-new-year-bounded sui year).
    pub(crate) const fn lunar_year(&self) -> i32 {
        self.lunar_year
    }

    /// Returns the converted lunar month number (`1..=12`, leap-insensitive).
    pub(crate) const fn lunar_month(&self) -> LunarMonth {
        self.lunar_month
    }

    /// Returns the converted lunar day of the month.
    pub(crate) const fn lunar_day(&self) -> LunarDay {
        self.lunar_day
    }

    /// Returns whether the converted lunar month is a leap month.
    pub(crate) const fn is_leap_month(&self) -> bool {
        self.is_leap_month
    }

    /// Returns the birth-year Heavenly Stem derived from the cyclic year.
    pub(crate) const fn birth_year_stem(&self) -> HeavenlyStem {
        self.birth_year_stem
    }

    /// Returns the birth-year Earthly Branch derived from the cyclic year.
    pub(crate) const fn birth_year_branch(&self) -> EarthlyBranch {
        self.birth_year_branch
    }

    /// Returns the full four pillars.
    pub(crate) const fn four_pillars(&self) -> FourPillars {
        self.four_pillars
    }
}

/// Synthesizes the wall-clock time for an `iztro` `timeIndex` (`0..=12`).
///
/// Matches the previous engine: `hour = max(time_index * 2 - 1, 0)`, `minute =
/// 30`. The late 子时 (`time_index == 12`, 23:30) rolls the day pillar to the
/// next day in the calendar engine.
const fn synthesized_hour(time_index: u8) -> u8 {
    let raw = time_index as i32 * 2 - 1;
    if raw < 0 { 0 } else { raw as u8 }
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
/// The lunar year/month/day and leap-month flag always use the lunar-new-year
/// boundary (they describe the lunisolar calendar position, not the cyclic
/// year). Only the birth-year stem/branch and the four-pillar year pillar follow
/// `year_boundary`: [`YearBoundary::LiChun`] re-resolves them against the exact
/// 立春 instant, while [`YearBoundary::ChineseNewYearEve`] reproduces the
/// lunar-new-year-bounded result.
pub(crate) fn solar_to_lunar_with_year_boundary(
    year: i32,
    month: SolarMonth,
    day: SolarDay,
    time_index: u8,
    year_boundary: YearBoundary,
) -> Result<LunarConversion, ChartError> {
    let resolved_time = ResolvedSolarDateTime {
        year,
        month: month.value(),
        day: day.value(),
        hour: synthesized_hour(time_index),
        minute: 30,
        second: 0,
    };
    solar_to_lunar_with_resolved_time(year, month, day, resolved_time, year_boundary)
}

/// Converts a Gregorian/solar date to typed Chinese-lunisolar facts using an
/// exact resolved local clock time for four-pillar and LiChun-boundary
/// resolution.
///
/// This is the clock-time API path. Unlike [`solar_to_lunar_with_year_boundary`],
/// it does not synthesize the representative midpoint of a `timeIndex`; the
/// resolved hour/minute/second are preserved when comparing against exact 立春.
pub(crate) fn solar_to_lunar_with_resolved_datetime(
    year: i32,
    month: SolarMonth,
    day: SolarDay,
    hour: u8,
    minute: u8,
    second: u8,
    year_boundary: YearBoundary,
) -> Result<LunarConversion, ChartError> {
    let resolved_time = ResolvedSolarDateTime {
        year,
        month: month.value(),
        day: day.value(),
        hour,
        minute,
        second,
    };
    solar_to_lunar_with_resolved_time(year, month, day, resolved_time, year_boundary)
}

fn solar_to_lunar_with_resolved_time(
    year: i32,
    month: SolarMonth,
    day: SolarDay,
    resolved_time: ResolvedSolarDateTime,
    year_boundary: YearBoundary,
) -> Result<LunarConversion, ChartError> {
    let conversion_failed = || ChartError::CalendarConversionFailed {
        year,
        month: month.value(),
        day: day.value(),
    };

    let calendar = TymeCalendar;
    let date = SolarDate::new(year, month.value(), day.value())?;
    let lunar = calendar.lunar_from_solar(date)?;
    let four_pillars = resolve_four_pillars(&calendar, resolved_time, lunar, year_boundary)?;

    Ok(LunarConversion {
        lunar_year: lunar.year,
        lunar_month: LunarMonth::new(lunar.month).map_err(|_| conversion_failed())?,
        lunar_day: LunarDay::new(lunar.day).map_err(|_| conversion_failed())?,
        is_leap_month: lunar.is_leap_month,
        birth_year_stem: four_pillars.yearly.stem(),
        birth_year_branch: four_pillars.yearly.branch(),
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
    let date = SolarDate::new(year, month.value(), day.value())?;
    TymeCalendar.lunar_from_solar(date)
}

/// Resolves the effective cyclic birth-year stem-branch for a solar date and
/// 时辰 under a 年分界 policy.
///
/// This is the focused year-boundary resolver used by tests: it returns the year
/// pillar that differs between [`YearBoundary::ChineseNewYearEve`] and
/// [`YearBoundary::LiChun`]. Under `LiChun` the resolution is datetime-level, so
/// the `time_index` selects the synthesized wall-clock time compared against the
/// exact 立春 instant.
#[cfg(test)]
pub(crate) fn resolve_effective_birth_year(
    year: i32,
    month: SolarMonth,
    day: SolarDay,
    time_index: u8,
    policy: YearBoundary,
) -> Result<crate::core::model::ganzhi::StemBranch, ChartError> {
    Ok(
        solar_to_lunar_with_year_boundary(year, month, day, time_index, policy)?
            .four_pillars()
            .yearly,
    )
}

#[cfg(test)]
mod tests;
