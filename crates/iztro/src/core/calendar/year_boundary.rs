//! Datetime-level 年分界 (year-boundary) pillar resolution.
//!
//! `lunar-lite`'s [`YearDivide::Exact`](lunar_lite::YearDivide) resolves the 立春
//! (LiChun) boundary at **date** granularity for upstream `iztro@2.5.8`
//! compatibility. `iztro-rs` instead resolves [`YearBoundary::LiChun`] at
//! **datetime** granularity using [`lunar_lite::li_chun_datetime`]: a birth
//! before the exact 立春 instant on the 立春 day keeps the previous Ganzhi year.
//! This module owns that policy and the normal 五虎遁 month-pillar derivation
//! from the effective year stem.

use lunar_lite::{EarthlyBranch, HeavenlyStem, LunarError, StemBranch, li_chun_datetime};

use crate::core::calculation::YearBoundary;

/// Resolves the effective cyclic birth-year stem-branch under a 年分界 policy.
///
/// `lunar_year` is the lunar-new-year-bounded lunar year, passed explicitly by
/// the caller (it is never re-derived here).
///
/// - [`YearBoundary::ChineseNewYearEve`] uses the lunar-new-year boundary, i.e.
///   `StemBranch::from_lunar_year(lunar_year)`.
/// - [`YearBoundary::LiChun`] is **datetime-level**: the birth instant
///   `(solar_year, month, day, hour, minute, second)` is compared against
///   [`li_chun_datetime(solar_year)`](lunar_lite::li_chun_datetime). A birth
///   strictly before the 立春 instant keeps the previous Gregorian/cyclic year;
///   at or after it the year advances. This intentionally diverges from upstream
///   `iztro@2.5.8`, which is date-level.
///
/// # Errors
/// Propagates [`LunarError::SolarTermOutOfRange`] from `li_chun_datetime` when
/// `solar_year` is outside the supported `1..=9999` range.
#[allow(clippy::too_many_arguments)]
pub(super) fn effective_birth_year(
    lunar_year: i32,
    solar_year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    boundary: YearBoundary,
) -> Result<StemBranch, LunarError> {
    match boundary {
        YearBoundary::ChineseNewYearEve => Ok(StemBranch::from_lunar_year(lunar_year)),
        YearBoundary::LiChun => {
            let li_chun = li_chun_datetime(solar_year)?;
            // `li_chun_datetime(solar_year).date.year == solar_year`, so a plain
            // lexicographic tuple comparison resolves the exact instant.
            let birth = (solar_year, month, day, hour, minute, second);
            let boundary_instant = (
                li_chun.date.year,
                li_chun.date.month,
                li_chun.date.day,
                li_chun.hour,
                li_chun.minute,
                li_chun.second,
            );
            let effective_year = if birth < boundary_instant {
                solar_year - 1
            } else {
                solar_year
            };
            Ok(StemBranch::from_lunar_year(effective_year))
        }
    }
}

/// Derives the 月柱 (month pillar) with the normal 五虎遁 rule from the effective
/// year stem and the lunar month.
///
/// Reproduces `lunar-lite`'s private `MonthDivide::Normal` formula (lunar month
/// 1 / 正月 maps to 寅), including the leap-month mid-month fix: a leap month past
/// its 15th day counts toward the following month.
pub(super) fn normal_month_pillar(
    year_stem: HeavenlyStem,
    lunar_month: u8,
    is_leap_month: bool,
    lunar_day: u8,
) -> StemBranch {
    let year_stem = year_stem.index();
    let yin_stem = (year_stem % 5 * 2 + 2) % 10;
    // A leap month past its 15th day counts toward the following month.
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
