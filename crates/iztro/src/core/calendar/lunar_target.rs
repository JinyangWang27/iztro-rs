//! Bounded internal resolver from a non-leap lunar date to its solar date.
//!
//! Core overlay builders need a concrete solar date, but the crate exposes no
//! public `lunar_to_solar` API. The static-chart temporal panel drills on
//! authentic lunar coordinates (流月 正月..腊月, 流日 初一..三十), so this module
//! finds the solar date of a requested **non-leap** lunar `(year, month, day)` by
//! scanning candidate solar dates through the existing [`solar_to_lunar`]
//! conversion. It is deliberately narrow: not a general lunar calendar, no
//! leap-month support, no public surface beyond the crate.

use crate::core::calendar::solar_to_lunar;
use crate::core::error::ChartError;
use crate::core::model::calendar::{BirthTime, SolarDay, SolarMonth};

/// A resolved temporal target: the concrete solar date/time for a requested
/// non-leap lunar date, plus the confirming lunar facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ResolvedTemporalTarget {
    /// Gregorian year of the resolved date.
    pub solar_year: i32,
    /// Gregorian month of the resolved date.
    pub solar_month: SolarMonth,
    /// Gregorian day of the resolved date.
    pub solar_day: SolarDay,
    /// Selected double-hour for the target instant.
    pub target_time: BirthTime,
    /// Confirmed lunar year.
    pub lunar_year: i32,
    /// Confirmed lunar month (1..=12, non-leap).
    pub lunar_month: u8,
    /// Confirmed lunar day (1..=30).
    pub lunar_day: u8,
    /// Always `false`: leap-month selection is deferred.
    pub is_leap_month: bool,
}

/// Gregorian month length, accounting for leap Februaries.
const fn gregorian_month_len(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Resolves a non-leap lunar `(lunar_year, lunar_month, lunar_day)` to its solar
/// date by a bounded scan, attaching the given target double-hour.
///
/// A lunar year falls within the two solar years `lunar_year..=lunar_year + 1`,
/// so the scan is bounded to that window. Returns
/// [`ChartError::UnresolvableLunarDate`] when no solar date in the window maps to
/// the requested non-leap lunar date (for example lunar day 30 of a 29-day
/// month, or a date outside the conversion range).
pub(crate) fn resolve_non_leap_lunar(
    lunar_year: i32,
    lunar_month: u8,
    lunar_day: u8,
    target_time: BirthTime,
) -> Result<ResolvedTemporalTarget, ChartError> {
    let not_found = || ChartError::UnresolvableLunarDate {
        lunar_year,
        lunar_month,
        lunar_day,
    };

    if !(1..=12).contains(&lunar_month) || !(1..=30).contains(&lunar_day) {
        return Err(not_found());
    }

    for solar_year in [lunar_year, lunar_year + 1] {
        for raw_month in 1u8..=12 {
            let Ok(month) = SolarMonth::new(raw_month) else {
                continue;
            };
            for raw_day in 1u8..=gregorian_month_len(solar_year, raw_month) {
                let Ok(day) = SolarDay::new(raw_day) else {
                    continue;
                };
                // `time_index` does not affect the lunar date; use 0 for the probe.
                let Ok(conversion) = solar_to_lunar(solar_year, month, day, 0) else {
                    continue;
                };
                if conversion.lunar_year() == lunar_year
                    && conversion.lunar_month().value() == lunar_month
                    && conversion.lunar_day().value() == lunar_day
                    && !conversion.is_leap_month()
                {
                    return Ok(ResolvedTemporalTarget {
                        solar_year,
                        solar_month: month,
                        solar_day: day,
                        target_time,
                        lunar_year,
                        lunar_month,
                        lunar_day,
                        is_leap_month: false,
                    });
                }
            }
        }
    }

    Err(not_found())
}

/// Whether the requested non-leap lunar month has a 30th day (`三十`).
pub(crate) fn lunar_month_has_thirtieth(lunar_year: i32, lunar_month: u8) -> bool {
    resolve_non_leap_lunar(lunar_year, lunar_month, 30, BirthTime::EarlyZi).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Round-trips a known solar date: convert to its lunar date, then resolve
    /// that lunar date back to the same solar date. Avoids hard-coding ephemeris.
    #[test]
    fn resolver_round_trips_a_solar_date_through_its_lunar_date() {
        let solar_month = SolarMonth::new(3).unwrap();
        let solar_day = SolarDay::new(20).unwrap();
        let conversion = solar_to_lunar(2024, solar_month, solar_day, 0).unwrap();
        assert!(!conversion.is_leap_month(), "fixture date must be non-leap");

        let resolved = resolve_non_leap_lunar(
            conversion.lunar_year(),
            conversion.lunar_month().value(),
            conversion.lunar_day().value(),
            BirthTime::EarlyZi,
        )
        .expect("known lunar date must resolve");

        assert_eq!(resolved.solar_year, 2024);
        assert_eq!(resolved.solar_month.value(), 3);
        assert_eq!(resolved.solar_day.value(), 20);
        assert_eq!(resolved.lunar_year, conversion.lunar_year());
        assert_eq!(resolved.lunar_month, conversion.lunar_month().value());
        assert_eq!(resolved.lunar_day, conversion.lunar_day().value());
        assert!(!resolved.is_leap_month);
    }

    #[test]
    fn out_of_range_lunar_coordinates_are_unresolvable() {
        assert!(matches!(
            resolve_non_leap_lunar(2024, 13, 1, BirthTime::EarlyZi),
            Err(ChartError::UnresolvableLunarDate { .. })
        ));
        assert!(matches!(
            resolve_non_leap_lunar(2024, 1, 31, BirthTime::EarlyZi),
            Err(ChartError::UnresolvableLunarDate { .. })
        ));
    }

    #[test]
    fn thirtieth_day_presence_matches_resolution() {
        // Whichever way a given month falls, the helper must agree with a direct
        // resolution attempt for lunar day 30.
        for month in 1u8..=12 {
            let has = lunar_month_has_thirtieth(2024, month);
            let resolved = resolve_non_leap_lunar(2024, month, 30, BirthTime::EarlyZi).is_ok();
            assert_eq!(has, resolved, "month {month}");
        }
    }
}
