//! Internal calendar adapter over `tyme4rs`.
//!
//! This is the **only** module permitted to depend on `tyme4rs`. Every
//! `tyme4rs` value is converted into an `iztro-rs`-owned type
//! ([`crate::core::model::ganzhi`] value objects, [`SolarDate`], or the
//! `pub(crate)` DTOs below) before crossing this boundary, so no third-party
//! calendar type ever leaks into the public or domain API.
//!
//! Responsibilities kept here are narrowly calendrical:
//!
//! - Gregorian/solar <-> Chinese-lunisolar conversion;
//! - the unambiguous day and hour pillars (continuous day count and 五鼠遁 hour,
//!   including the 晚子时 day roll), read from `tyme4rs`;
//! - the exact 立春 (LiChun) instant.
//!
//! Zi Wei Dou Shu chart policy stays in `iztro-rs`: the year and month pillars
//! (which follow the lunar-new-year / 五虎遁 conventions for `iztro@2.5.8`
//! parity) are derived in [`super::policy`], not here.

use tyme4rs::tyme::lunar::{LunarDay, LunarMonth, LunarYear};
use tyme4rs::tyme::sixtycycle::SixtyCycle;
use tyme4rs::tyme::solar::{SolarDay, SolarTerm, SolarTime};

use crate::core::error::ChartError;
use crate::core::model::calendar::SolarDate;
use lunar_lite::StemBranch;

/// A fully resolved local solar date and wall-clock time handed to the calendar
/// engine. Apparent-solar-time correction is applied by `iztro-rs` policy
/// *before* constructing this value (see [`crate::core::calculation`]).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ResolvedSolarDateTime {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

/// Typed Chinese-lunisolar facts for a solar date (lunar-new-year bounded).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LunarDateInfo {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub is_leap_month: bool,
    pub month_day_count: u8,
}

/// An explicit lunar date request before leap-month resolution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LunarDateInput {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub is_leap_month: bool,
}

/// The `tyme4rs`-backed calendar adapter. Zero-sized; holds no state.
pub(crate) struct TymeCalendar;

impl TymeCalendar {
    /// Converts a Gregorian/solar date to its Chinese-lunisolar facts.
    pub(crate) fn lunar_from_solar(&self, date: SolarDate) -> Result<LunarDateInfo, ChartError> {
        let (year, month, day) = (date.year(), date.month().value(), date.day().value());
        // `from_ymd` panics on invalid input, so validate first.
        SolarDay::validate(year as isize, month as usize, day as usize)
            .map_err(|_| ChartError::InvalidSolarDate { year, month, day })?;
        let lunar = SolarDay::from_ymd(year as isize, month as usize, day as usize).get_lunar_day();
        let lunar_month: LunarMonth = lunar.get_lunar_month();
        Ok(LunarDateInfo {
            year: lunar_month.get_lunar_year().get_year() as i32,
            month: lunar_month.get_month() as u8,
            day: lunar.get_day() as u8,
            is_leap_month: lunar_month.is_leap(),
            month_day_count: lunar_month.get_day_count() as u8,
        })
    }

    /// Resolves an explicit lunar date and its leap-month flag against the real
    /// calendar, mirroring upstream `lunar2solar` leap handling: the leap flag is
    /// honored only when the requested year actually has a leap month equal to
    /// the requested month; otherwise the ordinary month is used.
    pub(crate) fn resolve_lunar(&self, input: LunarDateInput) -> Result<LunarDateInfo, ChartError> {
        let LunarDateInput {
            year,
            month,
            day,
            is_leap_month,
        } = input;
        let unsupported = || ChartError::UnsupportedCalendarDate { year, month, day };

        let leap_month = LunarYear::from_year(year as isize).get_leap_month();
        let effective_leap = is_leap_month && leap_month == month as usize;
        let signed_month = if effective_leap {
            -(month as isize)
        } else {
            month as isize
        };

        LunarMonth::validate(year as isize, signed_month).map_err(|_| unsupported())?;
        let lunar_month = LunarMonth::from_ym(year as isize, signed_month);
        let month_day_count = lunar_month.get_day_count() as u8;
        if !(1..=month_day_count).contains(&day) {
            return Err(unsupported());
        }

        Ok(LunarDateInfo {
            year,
            month: lunar_month.get_month() as u8,
            day,
            is_leap_month: effective_leap,
            month_day_count,
        })
    }

    /// Converts an explicit lunar date to its Gregorian/solar date.
    ///
    /// The leap-month flag is honored only when the requested year actually has
    /// a matching leap month (see [`Self::resolve_lunar`]). Part of the adapter
    /// contract and exercised by the adapter tests; runtime lunar-target
    /// resolution currently scans through [`Self::lunar_from_solar`] instead.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn solar_from_lunar(&self, input: LunarDateInput) -> Result<SolarDate, ChartError> {
        let resolved = self.resolve_lunar(input)?;
        let signed_month = if resolved.is_leap_month {
            -(resolved.month as isize)
        } else {
            resolved.month as isize
        };
        let conversion_failed = || ChartError::CalendarConversionFailed {
            year: input.year,
            month: input.month,
            day: input.day,
        };
        LunarDay::validate(input.year as isize, signed_month, resolved.day as usize)
            .map_err(|_| conversion_failed())?;
        let solar = LunarDay::from_ymd(input.year as isize, signed_month, resolved.day as usize)
            .get_solar_day();
        SolarDate::new(
            solar.get_year() as i32,
            solar.get_month() as u8,
            solar.get_day() as u8,
        )
    }

    /// Returns the unambiguous day pillar (日柱) and hour pillar (时柱) for a
    /// resolved local date/time.
    ///
    /// Both come straight from `tyme4rs`'s sexagenary-hour: the day pillar is the
    /// continuous day count (rolling to the next day for 晚子时, 23:00..24:00),
    /// and the hour pillar follows 五鼠遁 from the (rolled) day stem. These agree
    /// with the previous `lunar-lite` `Normal` results.
    pub(crate) fn day_hour_pillars(
        &self,
        time: ResolvedSolarDateTime,
    ) -> Result<(StemBranch, StemBranch), ChartError> {
        let solar = solar_time(time)?;
        let sch = solar.get_sixty_cycle_hour();
        Ok((
            stem_branch_from_cycle(sch.get_day()),
            stem_branch_from_cycle(sch.get_sixty_cycle()),
        ))
    }

    /// Returns the exact 立春 (LiChun) instant for a Gregorian year.
    ///
    /// `YearBoundary::LiChun` resolution compares the resolved birth time against
    /// this instant (see [`super::policy`]).
    pub(crate) fn lichun_instant(
        &self,
        gregorian_year: i32,
    ) -> Result<ResolvedSolarDateTime, ChartError> {
        let solar = SolarTerm::from_name(gregorian_year as isize, "立春")
            .get_julian_day()
            .get_solar_time();
        Ok(ResolvedSolarDateTime {
            year: solar.get_year() as i32,
            month: solar.get_month() as u8,
            day: solar.get_day() as u8,
            hour: solar.get_hour() as u8,
            minute: solar.get_minute() as u8,
            second: solar.get_second() as u8,
        })
    }
}

/// Builds a validated `tyme4rs` [`SolarTime`] from a resolved local date/time.
fn solar_time(time: ResolvedSolarDateTime) -> Result<SolarTime, ChartError> {
    SolarTime::validate(
        time.year as isize,
        time.month as usize,
        time.day as usize,
        time.hour as usize,
        time.minute as usize,
        time.second as usize,
    )
    .map_err(|_| ChartError::InvalidSolarDate {
        year: time.year,
        month: time.month,
        day: time.day,
    })?;
    Ok(SolarTime::from_ymd_hms(
        time.year as isize,
        time.month as usize,
        time.day as usize,
        time.hour as usize,
        time.minute as usize,
        time.second as usize,
    ))
}

/// Converts a `tyme4rs` [`SixtyCycle`] into an owned [`StemBranch`].
///
/// The sexagenary index orderings agree (`0` == 甲子), so the cycle index maps
/// directly.
fn stem_branch_from_cycle(cycle: SixtyCycle) -> StemBranch {
    StemBranch::from_cycle_index(cycle.get_index())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lunar_lite::{EarthlyBranch, HeavenlyStem};

    fn solar(year: i32, month: u8, day: u8) -> SolarDate {
        SolarDate::new(year, month, day).expect("valid solar date")
    }

    fn sb(stem: HeavenlyStem, branch: EarthlyBranch) -> StemBranch {
        StemBranch::try_new(stem, branch).expect("valid stem-branch")
    }

    #[test]
    fn solar_to_lunar_basic() {
        // 2024-02-10 is lunar 2024-01-01 (Chinese New Year, non-leap).
        let info = TymeCalendar
            .lunar_from_solar(solar(2024, 2, 10))
            .expect("conversion succeeds");
        assert_eq!(info.year, 2024);
        assert_eq!(info.month, 1);
        assert_eq!(info.day, 1);
        assert!(!info.is_leap_month);
    }

    #[test]
    fn lunar_to_solar_basic() {
        let resolved = TymeCalendar
            .solar_from_lunar(LunarDateInput {
                year: 2024,
                month: 1,
                day: 1,
                is_leap_month: false,
            })
            .expect("conversion succeeds");
        assert_eq!(resolved.year(), 2024);
        assert_eq!(resolved.month().value(), 2);
        assert_eq!(resolved.day().value(), 10);
    }

    #[test]
    fn four_pillars_basic() {
        // Reference (lunar-lite@0.2.8 / iztro): 2000-08-16 timeIndex 2 (寅时,
        // synthesized 03:30) -> day 丙午, hour 庚寅.
        let (daily, hourly) = TymeCalendar
            .day_hour_pillars(ResolvedSolarDateTime {
                year: 2000,
                month: 8,
                day: 16,
                hour: 3,
                minute: 30,
                second: 0,
            })
            .expect("pillars resolve");
        assert_eq!(daily, sb(HeavenlyStem::Bing, EarthlyBranch::Wu));
        assert_eq!(hourly, sb(HeavenlyStem::Geng, EarthlyBranch::Yin));
    }

    #[test]
    fn late_zi_rolls_day_pillar() {
        // 晚子时 (23:30) rolls the day pillar forward one day relative to early
        // 子时 (00:30) on the same date.
        let (early_day, _) = TymeCalendar
            .day_hour_pillars(ResolvedSolarDateTime {
                year: 2000,
                month: 8,
                day: 16,
                hour: 0,
                minute: 30,
                second: 0,
            })
            .expect("pillars resolve");
        let (late_day, _) = TymeCalendar
            .day_hour_pillars(ResolvedSolarDateTime {
                year: 2000,
                month: 8,
                day: 16,
                hour: 23,
                minute: 30,
                second: 0,
            })
            .expect("pillars resolve");
        assert_eq!(
            late_day,
            StemBranch::from_cycle_index(early_day.cycle_index() + 1)
        );
    }

    #[test]
    fn lichun_instant_is_datetime_level() {
        // 2024 立春 falls on 2024-02-04 around 16:27 local (CST); assert the
        // adapter exposes an intra-day instant on that date.
        let lichun = TymeCalendar.lichun_instant(2024).expect("lichun resolves");
        assert_eq!(lichun.year, 2024);
        assert_eq!(lichun.month, 2);
        assert_eq!(lichun.day, 4);
        assert!(lichun.hour > 0 || lichun.minute > 0);
    }
}
