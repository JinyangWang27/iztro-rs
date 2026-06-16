//! Shared temporal-period derivation helpers.
//!
//! These helpers back the period types whose facts are derived with the same
//! upstream `iztro@2.5.8` rules: the common twelve-palace relabeling used when a
//! period's Life palace is fixed by a single Earthly Branch, the 流月 palace
//! index reused by both monthly and daily derivation, and the calendar-error
//! mapping shared by the target-date conversions.

use crate::core::error::ChartError;
use crate::core::model::calendar::CalendarKind;
use crate::core::model::chart::{
    Chart, PALACE_COUNT, PalaceName, TemporalPalaceLayout, TemporalPalaceName,
};
use crate::core::model::star::mutagen::Scope;
use lunar_lite::{EarthlyBranch, LunarError, StemBranch};

pub(super) fn build_life_branch_palace_layout(
    scope: Scope,
    life_branch: EarthlyBranch,
) -> Result<TemporalPalaceLayout, ChartError> {
    let life_index = yin_first_branch_index(life_branch);
    let names = (0..PALACE_COUNT)
        .map(|index| {
            TemporalPalaceName::new(
                EarthlyBranch::Yin.offset(index as isize),
                PalaceName::Life.offset(life_index as isize - index as isize),
            )
        })
        .collect();

    TemporalPalaceLayout::try_new(scope, names)
}

pub(super) fn yin_first_branch_index(branch: EarthlyBranch) -> usize {
    (branch.index() + PALACE_COUNT - EarthlyBranch::Yin.index()) % PALACE_COUNT
}

/// Derives the 流月 temporal Life palace index in Yin-first order.
///
/// Counts the flowing-year branch back to the natal birth month, forward to the
/// natal birth hour to land on the period's first lunar month, then forward by
/// the target lunar month; the natal and target leap-month corrections add a
/// month when the leap day falls in the second half. Shared by monthly period
/// derivation and by daily derivation, which counts on from this index.
pub(super) fn monthly_palace_index(
    natal: &Chart,
    target_lunar_year: i32,
    target_lunar_month: u8,
    target_lunar_day: u8,
    target_is_leap_month: bool,
) -> usize {
    let birth = natal.birth_context();
    let birth_date = birth.date();
    debug_assert_eq!(birth_date.kind(), CalendarKind::Lunar);

    let yearly_index =
        yin_first_branch_index(StemBranch::from_lunar_year(target_lunar_year).branch());
    let birth_month = birth_date.month() as isize;
    let birth_hour = birth.birth_time_variant().branch().index() as isize;
    let target_leap_addition = isize::from(target_is_leap_month && target_lunar_day > 15);
    let target_month = target_lunar_month as isize + target_leap_addition;

    (yearly_index as isize - birth_month + birth_hour + target_month)
        .rem_euclid(PALACE_COUNT as isize) as usize
}

/// Maps a target solar-date conversion failure to the matching [`ChartError`].
///
/// Shared by the monthly and daily period builders, which both convert the
/// caller-supplied target solar date through the lunar backend.
pub(super) fn map_target_solar_error(err: LunarError, year: i32, month: u8, day: u8) -> ChartError {
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
