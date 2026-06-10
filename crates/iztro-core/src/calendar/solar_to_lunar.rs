//! Gregorian-to-Chinese-lunisolar conversion backed by ICU4X `icu_calendar`.
//!
//! This is the **only** module that depends on `icu_calendar`. It maps an
//! ICU4X Chinese-calendar date onto the crate's own typed lunar facts and
//! birth-year ganzhi, so callers never see ICU4X types.
//!
//! The mapping is verified against pinned upstream `iztro@2.5.8`: ICU4X reports a
//! cyclic year whose `related_iso` is the lunar year and whose 1-based cyclic
//! index gives the year ganzhi, plus a month number, a leap-month flag, and a
//! day of month. iztro derives the chart year pillar with its default
//! `yearDivide: 'normal'` (lunar-new-year boundary), which matches ICU4X's
//! cyclic year, so the two agree even across the 立春/正月初一 window.

use icu_calendar::Date;
use icu_calendar::cal::ChineseTraditional;

use crate::error::ChartError;
use crate::model::calendar::{SolarDay, SolarMonth};
use crate::model::ganzhi::{EarthlyBranch, HeavenlyStem};
use crate::placement::natal::life_body::{LunarDay, LunarMonth};

/// Typed lunar facts produced from a Gregorian/solar date.
///
/// All fields are the crate's own domain types; no ICU4X types are exposed.
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
/// - [`ChartError::CalendarConversionFailed`] when ICU4X does not yield the
///   cyclic year or in-range lunar month/day the supported slice needs.
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

    let iso = Date::try_new_iso(year, month.value(), day.value()).map_err(|_| {
        ChartError::InvalidSolarDate {
            year,
            month: month.value(),
            day: day.value(),
        }
    })?;
    let chinese = iso.to_calendar(ChineseTraditional::new());

    let cyclic = chinese.year().cyclic().ok_or_else(conversion_failed)?;
    let cyclic_index = cyclic.year.checked_sub(1).ok_or_else(conversion_failed)? as usize;
    // `from_index` wraps with modulo, mapping the 1-based cyclic index onto the
    // ten Heavenly Stems and twelve Earthly Branches (cyclic year 1 = 甲子).
    let birth_year_stem = HeavenlyStem::from_index(cyclic_index);
    let birth_year_branch = EarthlyBranch::from_index(cyclic_index);

    let month_info = chinese.month();
    let lunar_month =
        LunarMonth::new(month_info.month_number()).map_err(|_| conversion_failed())?;
    let is_leap_month = month_info.is_leap();
    let lunar_day = LunarDay::new(chinese.day_of_month().0).map_err(|_| conversion_failed())?;

    Ok(LunarConversion {
        lunar_year: cyclic.related_iso,
        lunar_month,
        lunar_day,
        is_leap_month,
        birth_year_stem,
        birth_year_branch,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Golden conversions captured from pinned upstream `iztro@2.5.8`:
    //   node --input-type=module -e "import { astro } from 'iztro';
    //     const a = astro.bySolar('YYYY-M-D', 4, '女', true, 'zh-CN');
    //     console.log(a.rawDates.lunarDate, a.rawDates.chineseDate.yearly);"
    // These exercise the ICU4X mapping independently of the chart E2E fixtures:
    // before / on / after Chinese New Year, an ordinary mid-year date, a date
    // that converts into a leap lunar month (both halves), and a date after a
    // leap month. 1985-02-15 sits in the 立春/正月初一 window and still resolves
    // to the prior lunar year, proving ICU4X uses the lunar-new-year boundary
    // like iztro's default `yearDivide: 'normal'`.
    struct Case {
        year: i32,
        month: u8,
        day: u8,
        lunar_year: i32,
        lunar_month: u8,
        lunar_day: u8,
        is_leap: bool,
        stem: HeavenlyStem,
        branch: EarthlyBranch,
    }

    const CASES: &[Case] = &[
        // before Chinese New Year (in the 立春/CNY window) -> prior lunar year
        Case {
            year: 1985,
            month: 2,
            day: 15,
            lunar_year: 1984,
            lunar_month: 12,
            lunar_day: 26,
            is_leap: false,
            stem: HeavenlyStem::Jia,
            branch: EarthlyBranch::Zi,
        },
        // Chinese New Year day
        Case {
            year: 1986,
            month: 2,
            day: 9,
            lunar_year: 1986,
            lunar_month: 1,
            lunar_day: 1,
            is_leap: false,
            stem: HeavenlyStem::Bing,
            branch: EarthlyBranch::Yin,
        },
        // after Chinese New Year
        Case {
            year: 1986,
            month: 2,
            day: 25,
            lunar_year: 1986,
            lunar_month: 1,
            lunar_day: 17,
            is_leap: false,
            stem: HeavenlyStem::Bing,
            branch: EarthlyBranch::Yin,
        },
        // ordinary mid-year date
        Case {
            year: 1990,
            month: 5,
            day: 17,
            lunar_year: 1990,
            lunar_month: 4,
            lunar_day: 23,
            is_leap: false,
            stem: HeavenlyStem::Geng,
            branch: EarthlyBranch::Wu,
        },
        // before Chinese New Year of the next solar year -> prior lunar year
        Case {
            year: 2020,
            month: 1,
            day: 1,
            lunar_year: 2019,
            lunar_month: 12,
            lunar_day: 7,
            is_leap: false,
            stem: HeavenlyStem::Ji,
            branch: EarthlyBranch::Hai,
        },
        // converts into a leap lunar month, first half (day <= 15)
        Case {
            year: 2020,
            month: 6,
            day: 1,
            lunar_year: 2020,
            lunar_month: 4,
            lunar_day: 10,
            is_leap: true,
            stem: HeavenlyStem::Geng,
            branch: EarthlyBranch::Zi,
        },
        // converts into a leap lunar month, second half (day > 15)
        Case {
            year: 2020,
            month: 6,
            day: 18,
            lunar_year: 2020,
            lunar_month: 4,
            lunar_day: 27,
            is_leap: true,
            stem: HeavenlyStem::Geng,
            branch: EarthlyBranch::Zi,
        },
        // date after a leap month
        Case {
            year: 2020,
            month: 6,
            day: 25,
            lunar_year: 2020,
            lunar_month: 5,
            lunar_day: 5,
            is_leap: false,
            stem: HeavenlyStem::Geng,
            branch: EarthlyBranch::Zi,
        },
    ];

    #[test]
    fn matches_upstream_conversions() {
        for case in CASES {
            let conversion = solar_to_lunar(
                case.year,
                SolarMonth::new(case.month).expect("valid solar month"),
                SolarDay::new(case.day).expect("valid solar day"),
            )
            .unwrap_or_else(|err| {
                panic!(
                    "{}-{}-{} should convert: {err}",
                    case.year, case.month, case.day
                )
            });

            let label = format!("{}-{}-{}", case.year, case.month, case.day);
            assert_eq!(
                conversion.lunar_year(),
                case.lunar_year,
                "{label}: lunar year"
            );
            assert_eq!(
                conversion.lunar_month().value(),
                case.lunar_month,
                "{label}: lunar month"
            );
            assert_eq!(
                conversion.lunar_day().value(),
                case.lunar_day,
                "{label}: lunar day"
            );
            assert_eq!(
                conversion.is_leap_month(),
                case.is_leap,
                "{label}: leap flag"
            );
            assert_eq!(
                conversion.birth_year_stem(),
                case.stem,
                "{label}: year stem"
            );
            assert_eq!(
                conversion.birth_year_branch(),
                case.branch,
                "{label}: year branch"
            );
        }
    }

    #[test]
    fn cyclic_year_matches_lunar_year_ganzhi() {
        // The cyclic-derived ganzhi must equal the classical (year - 4) formula
        // anchored at 1984 = JiaZi, independently of the cyclic index path.
        for case in CASES {
            let conversion = solar_to_lunar(
                case.year,
                SolarMonth::new(case.month).expect("valid solar month"),
                SolarDay::new(case.day).expect("valid solar day"),
            )
            .expect("conversion should succeed");

            let offset = conversion.lunar_year() - 1984;
            let stem = HeavenlyStem::from_index((offset.rem_euclid(10)) as usize);
            let branch = EarthlyBranch::from_index((offset.rem_euclid(12)) as usize);
            assert_eq!(conversion.birth_year_stem(), stem, "{} stem", case.year);
            assert_eq!(
                conversion.birth_year_branch(),
                branch,
                "{} branch",
                case.year
            );
        }
    }

    #[test]
    fn rejects_impossible_solar_date() {
        let err = solar_to_lunar(
            2021,
            SolarMonth::new(2).expect("valid solar month"),
            SolarDay::new(30).expect("valid solar day"),
        )
        .expect_err("30 February is not a real date");
        assert!(matches!(err, ChartError::InvalidSolarDate { .. }));
    }
}
