//! Gregorian-to-Chinese-lunisolar conversion backed by `lunar-lite`.
//!
//! This module maps `lunar-lite` date values onto the crate's own typed lunar
//! facts, exposing only in-range month/day domain types to callers. The
//! birth-year sexagenary pair is derived through lunar-lite's four-pillar API
//! with the normal lunar-year boundary; its stem and branch (lunar-lite's own
//! canonical [`HeavenlyStem`]/[`EarthlyBranch`]) flow straight through.
//!
//! The result is verified against pinned upstream `iztro@2.5.8`: `lunar-lite`
//! returns the lunar-new-year-bounded year, month, leap-month flag, and day.
//! iztro derives the chart year pillar with its default `yearDivide: 'normal'`
//! (lunar-new-year boundary), so the four-pillar yearly result agrees with the
//! converted lunar-year stem-branch even across the 立春/正月初一 window.

use lunar_lite::{
    EarthlyBranch, HeavenlyStem, LunarError, MonthDivide, SolarDate, StemBranchOptions, YearDivide,
    four_pillars_from_solar_date_with_options, solar_to_lunar as convert_solar_to_lunar,
};

use crate::error::ChartError;
use crate::model::calendar::{SolarDay, SolarMonth};
use crate::placement::natal::life_body::{LunarDay, LunarMonth};

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
        0,
        StemBranchOptions {
            year: YearDivide::Normal,
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
    })
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
