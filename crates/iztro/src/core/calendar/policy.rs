//! Zi Wei Dou Shu calendar policy owned by `iztro-rs`.
//!
//! The runtime calendar engine adapter ([`super::tyme`]) supplies raw
//! calendrical facts: the lunar date, the unambiguous day/hour pillars, and the
//! exact 立春 (LiChun) instant. This module applies the chart-generation policy
//! that `iztro-rs` owns and that must stay aligned with `iztro@2.5.8`:
//!
//! - the **year pillar** follows the [`YearBoundary`] policy
//!   ([`YearBoundary::ChineseNewYearEve`] uses the lunar-year stem-branch;
//!   [`YearBoundary::LiChun`] compares the resolved birth instant against the
//!   exact 立春 instant);
//! - the **month pillar** follows the lunar-month 五虎遁 convention
//!   (`MonthDivide::Normal` in the previous `lunar-lite` engine).
//!
//! `YearBoundary::LiChun` is **datetime-level**: a birth before the exact 立春
//! instant on the 立春 day still belongs to the previous Ganzhi year. This is a
//! deliberate correction over the previous `lunar-lite` date-level boundary and
//! over `iztro@2.5.8`; the single affected supported-field fixture case is marked
//! as an intentional divergence. See ADR 0006 and `docs/en/compatibility.md`.
//!
//! Apparent-solar-time correction is a separate input-calculation policy applied
//! before the calendar engine; it lives in [`crate::core::calculation`].

use crate::core::calculation::YearBoundary;
use crate::core::error::ChartError;
use lunar_lite::{EarthlyBranch, FourPillars, HeavenlyStem, StemBranch};

use super::tyme::{LunarDateInfo, ResolvedSolarDateTime, TymeCalendar};

/// Resolves the four pillars (四柱) for a resolved local date/time under a
/// [`YearBoundary`] policy.
///
/// The day and hour pillars are unambiguous and come from the calendar engine.
/// The year and month pillars are derived here so they match the
/// lunar-new-year / 五虎遁 conventions used for `iztro@2.5.8` parity.
pub(super) fn resolve_four_pillars(
    calendar: &TymeCalendar,
    time: ResolvedSolarDateTime,
    lunar: LunarDateInfo,
    year_boundary: YearBoundary,
) -> Result<FourPillars, ChartError> {
    let yearly = resolve_year_pillar(calendar, time, lunar.year, year_boundary)?;
    let monthly = month_pillar_normal(yearly.stem(), lunar.month, lunar.day, lunar.is_leap_month);
    let (daily, hourly) = calendar.day_hour_pillars(time)?;
    Ok(FourPillars {
        yearly,
        monthly,
        daily,
        hourly,
    })
}

/// Resolves the cyclic birth-year stem-branch (年柱) under a [`YearBoundary`].
///
/// [`YearBoundary::ChineseNewYearEve`] uses the lunar-new-year-bounded lunar
/// year. [`YearBoundary::LiChun`] uses the exact 立春 instant: a birth *before*
/// the instant on the 立春 day still belongs to the previous Ganzhi year.
pub(super) fn resolve_year_pillar(
    calendar: &TymeCalendar,
    time: ResolvedSolarDateTime,
    lunar_year: i32,
    year_boundary: YearBoundary,
) -> Result<StemBranch, ChartError> {
    match year_boundary {
        YearBoundary::ChineseNewYearEve => Ok(StemBranch::from_lunar_year(lunar_year)),
        YearBoundary::LiChun => {
            let lichun = calendar.lichun_instant(time.year)?;
            let pillar_year = if is_before_instant(time, lichun) {
                time.year - 1
            } else {
                time.year
            };
            // The sexagenary cycle of a Gregorian year shares the 1984 anchor.
            Ok(StemBranch::from_lunar_year(pillar_year))
        }
    }
}

/// Whether a resolved local instant precedes the given 立春 instant within the
/// same Gregorian year.
fn is_before_instant(time: ResolvedSolarDateTime, lichun: ResolvedSolarDateTime) -> bool {
    (time.month, time.day, time.hour, time.minute, time.second)
        < (
            lichun.month,
            lichun.day,
            lichun.hour,
            lichun.minute,
            lichun.second,
        )
}

/// Computes the month pillar (月柱) by the lunar-month 五虎遁 convention.
///
/// Mirrors the previous `lunar-lite` `MonthDivide::Normal` result: the month
/// branch counts from 寅 (正月), and the month stem follows 五虎遁 from the
/// resolved year pillar's stem. A leap month past its 15th day counts toward the
/// following month.
fn month_pillar_normal(
    year_stem: HeavenlyStem,
    lunar_month: u8,
    lunar_day: u8,
    is_leap_month: bool,
) -> StemBranch {
    let yin_stem = year_stem.index() % 5 * 2 + 2;
    let fix_leap = usize::from(is_leap_month && lunar_day > 15);
    let offset = (lunar_month as usize - 1) + fix_leap;
    let stem = (yin_stem + offset) % 10;
    let branch = (2 + offset) % 12; // lunar month 1 (正月) == 寅 (index 2)
    StemBranch::try_new(
        HeavenlyStem::from_index(stem),
        EarthlyBranch::from_index(branch),
    )
    .expect("computed stem and branch share parity by construction")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn time(year: i32, month: u8, day: u8, hour: u8) -> ResolvedSolarDateTime {
        ResolvedSolarDateTime {
            year,
            month,
            day,
            hour,
            minute: 30,
            second: 0,
        }
    }

    fn sb(stem: HeavenlyStem, branch: EarthlyBranch) -> StemBranch {
        StemBranch::try_new(stem, branch).expect("valid stem-branch")
    }

    #[test]
    fn lichun_before_exact_instant_uses_previous_year() {
        // 2024 立春 is on 2024-02-04 ~16:27. A birth earlier that day belongs to
        // the previous Ganzhi year 癸卯 (GuiMao), not 甲辰 (JiaChen).
        let pillar = resolve_year_pillar(
            &TymeCalendar,
            time(2024, 2, 4, 8),
            2023,
            YearBoundary::LiChun,
        )
        .expect("year pillar resolves");
        assert_eq!(pillar, sb(HeavenlyStem::Gui, EarthlyBranch::Mao));
    }

    #[test]
    fn lichun_after_exact_instant_uses_new_year() {
        // Later the same day, after the 立春 instant, the Ganzhi year is 甲辰.
        let pillar = resolve_year_pillar(
            &TymeCalendar,
            time(2024, 2, 4, 20),
            2023,
            YearBoundary::LiChun,
        )
        .expect("year pillar resolves");
        assert_eq!(pillar, sb(HeavenlyStem::Jia, EarthlyBranch::Chen));
    }

    #[test]
    fn chinese_new_year_eve_uses_lunar_year() {
        // 2024-02-04 is still lunar year 2023 (before Chinese New Year 2024-02-10),
        // so the ChineseNewYearEve boundary yields 癸卯 regardless of 立春.
        let pillar = resolve_year_pillar(
            &TymeCalendar,
            time(2024, 2, 4, 20),
            2023,
            YearBoundary::ChineseNewYearEve,
        )
        .expect("year pillar resolves");
        assert_eq!(pillar, sb(HeavenlyStem::Gui, EarthlyBranch::Mao));
    }

    #[test]
    fn month_pillar_normal_matches_wu_hu_dun() {
        // 2000 庚辰 year, lunar month 7 -> 月柱 甲申 (reference lunar-lite 2000-08-16).
        let pillar = month_pillar_normal(HeavenlyStem::Geng, 7, 17, false);
        assert_eq!(pillar, sb(HeavenlyStem::Jia, EarthlyBranch::Shen));
    }
}
