//! Gregorian-to-Chinese-lunisolar conversion backed by `lunar-lite`.
//!
//! This module maps `lunar-lite` date values onto the crate's own typed lunar
//! facts and birth-year ganzhi, so callers never see calendar-backend types.
//! The birth-year sexagenary pair is derived internally via lunar-lite 0.2's
//! [`StemBranch::from_lunar_year`] helper and then mapped onto the crate's own
//! [`HeavenlyStem`]/[`EarthlyBranch`] model types; the upstream stem/branch
//! enums never cross this module's boundary.
//!
//! The mapping is verified against pinned upstream `iztro@2.5.8`: `lunar-lite`
//! returns the lunar-new-year-bounded year, month, leap-month flag, and day.
//! iztro derives the chart year pillar with its default `yearDivide: 'normal'`
//! (lunar-new-year boundary), so deriving ganzhi from the converted lunar year
//! agrees with upstream even across the 立春/正月初一 window.

use lunar_lite::{
    EarthlyBranch as LunarLiteBranch, HeavenlyStem as LunarLiteStem, LunarError, SolarDate,
    StemBranch as LunarLiteStemBranch, solar_to_lunar as convert_solar_to_lunar,
};

use crate::error::ChartError;
use crate::model::calendar::{SolarDay, SolarMonth};
use lunar_lite::{EarthlyBranch, HeavenlyStem};
use crate::placement::natal::life_body::{LunarDay, LunarMonth};

/// Typed lunar facts produced from a Gregorian/solar date.
///
/// All fields are the crate's own domain types; no calendar-backend types are exposed.
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

    let lunar = convert_solar_to_lunar(SolarDate {
        year,
        month: month.value(),
        day: day.value(),
    })
    .map_err(|err| map_solar_conversion_error(err, year, month.value(), day.value()))?;

    let lunar_month = LunarMonth::new(lunar.month).map_err(|_| conversion_failed())?;
    let lunar_day = LunarDay::new(lunar.day).map_err(|_| conversion_failed())?;
    let birth_year_pair = LunarLiteStemBranch::from_lunar_year(lunar.year);
    let birth_year_stem = map_lunar_lite_stem(birth_year_pair.stem());
    let birth_year_branch = map_lunar_lite_branch(birth_year_pair.branch());

    Ok(LunarConversion {
        lunar_year: lunar.year,
        lunar_month,
        lunar_day,
        is_leap_month: lunar.is_leap_month,
        birth_year_stem,
        birth_year_branch,
    })
}

fn map_solar_conversion_error(err: LunarError, year: i32, month: u8, day: u8) -> ChartError {
    match err {
        LunarError::InvalidSolarDate { .. } => ChartError::InvalidSolarDate { year, month, day },
        LunarError::YearOutOfRange { .. } => {
            ChartError::UnsupportedCalendarDate { year, month, day }
        }
        LunarError::InvalidLunarDate { .. } | LunarError::InvalidTime { .. } => {
            ChartError::CalendarConversionFailed { year, month, day }
        }
    }
}

/// Maps a lunar-lite Heavenly Stem onto the crate's own model type.
///
/// Keeps lunar-lite's stem enum from leaking past this module's boundary.
fn map_lunar_lite_stem(stem: LunarLiteStem) -> HeavenlyStem {
    match stem {
        LunarLiteStem::Jia => HeavenlyStem::Jia,
        LunarLiteStem::Yi => HeavenlyStem::Yi,
        LunarLiteStem::Bing => HeavenlyStem::Bing,
        LunarLiteStem::Ding => HeavenlyStem::Ding,
        LunarLiteStem::Wu => HeavenlyStem::Wu,
        LunarLiteStem::Ji => HeavenlyStem::Ji,
        LunarLiteStem::Geng => HeavenlyStem::Geng,
        LunarLiteStem::Xin => HeavenlyStem::Xin,
        LunarLiteStem::Ren => HeavenlyStem::Ren,
        LunarLiteStem::Gui => HeavenlyStem::Gui,
    }
}

/// Maps a lunar-lite Earthly Branch onto the crate's own model type.
///
/// Keeps lunar-lite's branch enum from leaking past this module's boundary.
fn map_lunar_lite_branch(branch: LunarLiteBranch) -> EarthlyBranch {
    match branch {
        LunarLiteBranch::Zi => EarthlyBranch::Zi,
        LunarLiteBranch::Chou => EarthlyBranch::Chou,
        LunarLiteBranch::Yin => EarthlyBranch::Yin,
        LunarLiteBranch::Mao => EarthlyBranch::Mao,
        LunarLiteBranch::Chen => EarthlyBranch::Chen,
        LunarLiteBranch::Si => EarthlyBranch::Si,
        LunarLiteBranch::Wu => EarthlyBranch::Wu,
        LunarLiteBranch::Wei => EarthlyBranch::Wei,
        LunarLiteBranch::Shen => EarthlyBranch::Shen,
        LunarLiteBranch::You => EarthlyBranch::You,
        LunarLiteBranch::Xu => EarthlyBranch::Xu,
        LunarLiteBranch::Hai => EarthlyBranch::Hai,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Golden conversions captured from pinned upstream `iztro@2.5.8`:
    //   node --input-type=module -e "import { astro } from 'iztro';
    //     const a = astro.bySolar('YYYY-M-D', 4, '女', true, 'zh-CN');
    //     console.log(a.rawDates.lunarDate, a.rawDates.chineseDate.yearly);"
    // These exercise the adapter mapping independently of the chart E2E fixtures:
    // before / on / after Chinese New Year, an ordinary mid-year date, a date
    // that converts into a leap lunar month (both halves), and a date after a
    // leap month. 1985-02-15 sits in the 立春/正月初一 window and still resolves
    // to the prior lunar year, proving the adapter uses the lunar-new-year
    // boundary like iztro's default `yearDivide: 'normal'`.
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
    fn lunar_year_pair_maps_to_domain_ganzhi() {
        // Anchor (1984 = JiaZi), a recent dragon year, and a pre-anchor year all
        // map lunar-lite's sexagenary pair onto the crate's own ganzhi types.
        for (year, stem, branch) in [
            (1984, HeavenlyStem::Jia, EarthlyBranch::Zi),
            (2024, HeavenlyStem::Jia, EarthlyBranch::Chen),
            (1983, HeavenlyStem::Gui, EarthlyBranch::Hai),
        ] {
            let pair = LunarLiteStemBranch::from_lunar_year(year);
            assert_eq!(map_lunar_lite_stem(pair.stem()), stem, "{year} stem");
            assert_eq!(
                map_lunar_lite_branch(pair.branch()),
                branch,
                "{year} branch"
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
