use lunar_lite::{FourPillars, HeavenlyStem, LunarError, li_chun_datetime};

use crate::core::calculation::YearBoundary;
use crate::core::placement::natal::life_body::{LunarDay, LunarMonth};

/// Resolved clock fields used by the internal calendar adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ResolvedSolarClock {
    pub(super) hour: u8,
    pub(super) minute: u8,
    pub(super) second: u8,
}

/// Resolved Gregorian/solar date and clock fields used for boundary policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ResolvedSolarMoment {
    pub(super) year: i32,
    pub(super) month: u8,
    pub(super) day: u8,
    pub(super) hour: u8,
    pub(super) minute: u8,
    pub(super) second: u8,
}

impl ResolvedSolarMoment {
    pub(super) const fn new(year: i32, month: u8, day: u8, clock: ResolvedSolarClock) -> Self {
        Self {
            year,
            month,
            day,
            hour: clock.hour,
            minute: clock.minute,
            second: clock.second,
        }
    }

    /// Returns whether this birth moment is strictly before exact 立春.
    pub(super) fn is_before_li_chun(self) -> Result<bool, LunarError> {
        let li_chun = li_chun_datetime(self.year)?;
        // `li_chun_datetime(year).date.year == year`, so lexicographic tuple
        // comparison resolves the exact second-precision boundary instant.
        let birth = (
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
        );
        let boundary = (
            li_chun.date.year,
            li_chun.date.month,
            li_chun.date.day,
            li_chun.hour,
            li_chun.minute,
            li_chun.second,
        );
        Ok(birth < boundary)
    }
}

/// Inputs needed to resolve the effective cyclic birth year.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct YearBoundaryInput {
    pub(super) lunar_year: i32,
    pub(super) solar_moment: ResolvedSolarMoment,
    pub(super) boundary: YearBoundary,
}

/// Typed lunar facts produced from a Gregorian/solar date.
///
/// Calendar-backend date/error types stay internal; the birth-year stem/branch
/// and four pillars use `lunar-lite`'s canonical GanZhi value objects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LunarConversion {
    pub(super) lunar_year: i32,
    pub(super) lunar_month: LunarMonth,
    pub(super) lunar_day: LunarDay,
    pub(super) is_leap_month: bool,
    pub(super) birth_year_stem: HeavenlyStem,
    pub(super) birth_year_branch: lunar_lite::EarthlyBranch,
    pub(super) four_pillars: FourPillars,
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
    pub(crate) const fn birth_year_branch(&self) -> lunar_lite::EarthlyBranch {
        self.birth_year_branch
    }

    /// Returns the full four pillars.
    pub(crate) const fn four_pillars(&self) -> FourPillars {
        self.four_pillars
    }
}

/// Lunar-new-year-bounded lunar facts for a Gregorian/solar date, without
/// deriving four pillars.
///
/// Shared by the full-horoscope stack builder, which needs the target lunar year
/// to derive the flowing year and nominal age.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LunarDateInfo {
    /// Lunar-new-year-bounded lunar year.
    pub year: i32,
    /// Lunar month number (`1..=12`, leap-insensitive).
    pub month: u8,
    /// Lunar day of the month.
    pub day: u8,
    /// Whether the lunar month is a leap month.
    pub is_leap_month: bool,
    /// Number of days in the lunar month (`29` or `30`).
    pub month_day_count: u8,
}
