//! Lunar-date normalization backed by ICU4X `icu_calendar`.
//!
//! Upstream `iztro.byLunar(date, _, _, isLeapMonth, _)` delegates to
//! `lunar-lite.lunar2solar(date, isLeapMonth)`, which only honors
//! `isLeapMonth = true` when the requested lunar year actually has a leap month
//! equal to the requested month. If the requested month is not the year's leap
//! month, the flag is ignored and the date is treated as the ordinary month.
//!
//! [`resolve_lunar_date`] reproduces that rule: it never blindly trusts the
//! caller's `is_leap_month`. It resolves the flag against the real calendar with
//! ICU4X (reverse-constructing the Chinese date) and returns the corrected facts.
//! Like the rest of this module, it exposes only the crate's own domain types.

use icu_calendar::Date;
use icu_calendar::cal::ChineseTraditional;
use icu_calendar::types::MonthCode;

use crate::error::ChartError;
use crate::placement::natal::life_body::{LunarDay, LunarMonth};

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
    let unsupported = || ChartError::UnsupportedCalendarDate {
        year: lunar_year,
        month,
        day,
    };
    let conversion_failed = || ChartError::CalendarConversionFailed {
        year: lunar_year,
        month,
        day,
    };

    let normal_code = MonthCode::new_normal(month).ok_or_else(conversion_failed)?;

    // Honor the leap flag only when the requested month is actually leap that
    // year, probed by reverse-constructing the leap month's first day.
    let use_leap = is_leap_month && {
        let leap_code = MonthCode::new_leap(month).ok_or_else(conversion_failed)?;
        Date::try_new_from_codes(None, lunar_year, leap_code, 1, ChineseTraditional::new()).is_ok()
    };

    let month_code = if use_leap {
        MonthCode::new_leap(month).ok_or_else(conversion_failed)?
    } else {
        normal_code
    };

    let date =
        Date::try_new_from_codes(None, lunar_year, month_code, day, ChineseTraditional::new())
            .map_err(|_| unsupported())?;
    let month_days =
        if Date::try_new_from_codes(None, lunar_year, month_code, 30, ChineseTraditional::new())
            .is_ok()
        {
            30
        } else {
            29
        };

    let month_info = date.month();
    Ok(ResolvedLunarDate {
        lunar_year: date
            .year()
            .cyclic()
            .map(|cyclic| cyclic.related_iso)
            .unwrap_or(lunar_year),
        lunar_month: LunarMonth::new(month_info.month_number()).map_err(|_| conversion_failed())?,
        lunar_day: LunarDay::new(date.day_of_month().0).map_err(|_| conversion_failed())?,
        is_leap_month: month_info.is_leap(),
        month_days,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolve(year: i32, month: u8, day: u8, leap: bool) -> ResolvedLunarDate {
        resolve_lunar_date(
            year,
            LunarMonth::new(month).expect("valid lunar month"),
            LunarDay::new(day).expect("valid lunar day"),
            leap,
        )
        .expect("resolution should succeed")
    }

    #[test]
    fn honors_valid_leap_month() {
        // 2020 has a leap fourth month; the flag is kept.
        let resolved = resolve(2020, 4, 27, true);
        assert_eq!(resolved.lunar_year(), 2020);
        assert_eq!(resolved.lunar_month().value(), 4);
        assert_eq!(resolved.lunar_day().value(), 27);
        assert_eq!(resolved.month_days(), 29);
        assert!(resolved.is_leap_month());
    }

    #[test]
    fn ignores_invalid_leap_month() {
        // 2020's leap month is the fourth, not the third; the flag is ignored.
        let resolved = resolve(2020, 3, 20, true);
        assert_eq!(resolved.lunar_month().value(), 3);
        assert_eq!(resolved.lunar_day().value(), 20);
        assert!(!resolved.is_leap_month());

        // Same year, fifth month is not leap either.
        assert!(!resolve(2020, 5, 20, true).is_leap_month());

        // 2021 has no leap month at all.
        assert!(!resolve(2021, 6, 10, true).is_leap_month());
    }

    #[test]
    fn non_leap_request_stays_non_leap() {
        let resolved = resolve(2020, 4, 10, false);
        assert_eq!(resolved.lunar_month().value(), 4);
        assert!(!resolved.is_leap_month());
    }
}
