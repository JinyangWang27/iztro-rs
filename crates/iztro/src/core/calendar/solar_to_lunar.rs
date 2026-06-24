//! Gregorian-to-Chinese-lunisolar conversion backed by `lunar-lite`.
//!
//! This module maps `lunar-lite` date values onto the crate's own typed lunar
//! facts, exposing only in-range month/day domain types to callers. The
//! four pillars are derived through lunar-lite's four-pillar API with the normal
//! year/month boundaries; the year pillar (lunar-lite's own canonical
//! [`HeavenlyStem`]/[`EarthlyBranch`] pair) flows straight through as the
//! retained birth-year fact.
//!
//! The result is verified against pinned upstream `iztro@2.5.8`: `lunar-lite`
//! returns the lunar-new-year-bounded year, month, leap-month flag, and day.
//! iztro derives the chart year pillar with its default `yearDivide: 'normal'`
//! (lunar-new-year boundary), so the four-pillar yearly result agrees with the
//! converted lunar-year stem-branch even across the 立春/正月初一 window.

use lunar_lite::{
    EarthlyBranch, FourPillars, HeavenlyStem, LunarError, MonthDivide, SolarDate, StemBranchOptions,
    YearDivide, four_pillars_from_solar_date_with_options,
    solar_to_lunar as convert_solar_to_lunar,
};

use crate::core::calculation::YearBoundary;
use crate::core::error::ChartError;
use crate::core::model::calendar::{SolarDay, SolarMonth};
use crate::core::placement::natal::life_body::{LunarDay, LunarMonth};

/// Maps the 年分界 calculation policy to the `lunar-lite` year-pillar boundary.
///
/// [`YearBoundary::ChineseNewYearEve`] uses the lunar-new-year boundary
/// ([`YearDivide::Normal`]), preserving existing behaviour; [`YearBoundary::LiChun`]
/// uses the 立春 boundary ([`YearDivide::Exact`]) at date granularity.
const fn year_divide(boundary: YearBoundary) -> YearDivide {
    match boundary {
        YearBoundary::ChineseNewYearEve => YearDivide::Normal,
        YearBoundary::LiChun => YearDivide::Exact,
    }
}

/// Typed lunar facts produced from a Gregorian/solar date.
///
/// Calendar-backend date/error types stay internal; birth-year stem/branch use
/// lunar-lite's canonical GanZhi primitives.
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

    /// Returns the full four pillars derived with normal year/month boundaries.
    pub(crate) const fn four_pillars(&self) -> FourPillars {
        self.four_pillars
    }
}

/// Converts a Gregorian/solar date to typed Chinese-lunisolar facts.
///
/// Returns:
///
/// - [`ChartError::InvalidSolarDate`] when the year-month-day is not a real
///   Gregorian date (for example 30 February);
/// - [`ChartError::UnsupportedCalendarDate`] when `lunar-lite` reports the date
///   is outside its supported range;
/// - [`ChartError::CalendarConversionFailed`] when conversion does not yield
///   in-range lunar month/day facts the supported slice needs.
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
/// `year_boundary`: [`YearBoundary::LiChun`] re-resolves them across the
/// 立春 boundary, while [`YearBoundary::ChineseNewYearEve`] reproduces the
/// existing lunar-new-year-bounded result.
pub(crate) fn solar_to_lunar_with_year_boundary(
    year: i32,
    month: SolarMonth,
    day: SolarDay,
    time_index: u8,
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
    let pillars = four_pillars_from_solar_date_with_options(
        solar,
        time_index,
        StemBranchOptions {
            year: year_divide(year_boundary),
            month: MonthDivide::Normal,
        },
    )
    .map_err(|err| map_solar_conversion_error(err, year, month.value(), day.value()))?;

    let lunar_month = LunarMonth::new(lunar.month).map_err(|_| conversion_failed())?;
    let lunar_day = LunarDay::new(lunar.day).map_err(|_| conversion_failed())?;

    Ok(LunarConversion {
        lunar_year: lunar.year,
        lunar_month,
        lunar_day,
        is_leap_month: lunar.is_leap_month,
        birth_year_stem: pillars.yearly.stem(),
        birth_year_branch: pillars.yearly.branch(),
        four_pillars: pillars,
    })
}

/// Resolves the effective cyclic birth-year stem-branch for a solar date under a
/// 年分界 policy.
///
/// This is the focused year-boundary resolver: it derives only the year pillar
/// (with the normal month boundary) and returns it as a [`StemBranch`]. It is the
/// fact that differs between [`YearBoundary::ChineseNewYearEve`] and
/// [`YearBoundary::LiChun`] for a date in the 立春/正月初一 window.
#[cfg(test)]
pub(crate) fn resolve_effective_birth_year(
    year: i32,
    month: SolarMonth,
    day: SolarDay,
    policy: YearBoundary,
) -> Result<lunar_lite::StemBranch, ChartError> {
    let conversion = solar_to_lunar_with_year_boundary(year, month, day, 0, policy)?;
    lunar_lite::StemBranch::try_new(conversion.birth_year_stem(), conversion.birth_year_branch())
        .map_err(|err| match err {
            lunar_lite::StemBranchError::InvalidStemBranchPair { stem, branch } => {
                ChartError::InvalidStemBranchPair { stem, branch }
            }
        },
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
