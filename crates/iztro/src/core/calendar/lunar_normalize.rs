//! Lunar-date normalization backed by the internal calendar adapter.
//!
//! Upstream `iztro.byLunar(date, _, _, isLeapMonth, _)` delegates to a lunar
//! conversion that only honors `isLeapMonth = true` when the requested lunar
//! year actually has a leap month equal to the requested month. If the requested
//! month is not the year's leap month, the flag is ignored and the date is
//! treated as the ordinary month.
//!
//! [`resolve_lunar_date`] reproduces that rule via [`TymeCalendar::resolve_lunar`]:
//! it never blindly trusts the caller's `is_leap_month`. It resolves the flag
//! against the real calendar and returns the corrected facts.
//! Like the rest of this module, it exposes only the crate's own domain types.

use crate::core::error::ChartError;
use crate::core::placement::natal::life_body::{LunarDay, LunarMonth};

use super::tyme::{LunarDateInput, TymeCalendar};

/// A lunar date whose leap-month flag has been resolved against the real
/// calendar (the leap flag is kept only when the month is actually leap).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ResolvedLunarDate {
    lunar_year: i32,
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    is_leap_month: bool,
    month_days: u8,
}

impl ResolvedLunarDate {
    /// Returns the resolved lunar year.
    pub(crate) const fn lunar_year(&self) -> i32 {
        self.lunar_year
    }

    /// Returns the resolved lunar month number (`1..=12`, leap-insensitive).
    pub(crate) const fn lunar_month(&self) -> LunarMonth {
        self.lunar_month
    }

    /// Returns the resolved lunar day of the month.
    pub(crate) const fn lunar_day(&self) -> LunarDay {
        self.lunar_day
    }

    /// Returns whether the resolved lunar month is actually a leap month.
    pub(crate) const fn is_leap_month(&self) -> bool {
        self.is_leap_month
    }

    /// Returns the number of days in the resolved lunar month.
    pub(crate) const fn month_days(&self) -> u8 {
        self.month_days
    }
}

/// Resolves an explicit lunar date and its leap-month flag against the real
/// calendar, mirroring upstream `lunar2solar` leap handling.
///
/// `is_leap_month` is honored only when `lunar_year` actually has a leap month
/// equal to `lunar_month`; otherwise it is ignored and the ordinary month is
/// used. Returns [`ChartError::UnsupportedCalendarDate`] when the day does not
/// exist in the resolved month.
pub(crate) fn resolve_lunar_date(
    lunar_year: i32,
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    is_leap_month: bool,
) -> Result<ResolvedLunarDate, ChartError> {
    let month = lunar_month.value();
    let day = lunar_day.value();
    let conversion_failed = || ChartError::CalendarConversionFailed {
        year: lunar_year,
        month,
        day,
    };

    let resolved = TymeCalendar.resolve_lunar(LunarDateInput {
        year: lunar_year,
        month,
        day,
        is_leap_month,
    })?;

    Ok(ResolvedLunarDate {
        lunar_year: resolved.year,
        lunar_month: LunarMonth::new(resolved.month).map_err(|_| conversion_failed())?,
        lunar_day: LunarDay::new(resolved.day).map_err(|_| conversion_failed())?,
        is_leap_month: resolved.is_leap_month,
        month_days: resolved.month_day_count,
    })
}

#[cfg(test)]
mod tests;
