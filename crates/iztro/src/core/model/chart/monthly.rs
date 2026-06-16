//! Deterministic monthly-period (流月) facts derived from natal and target facts.
//!
//! This module derives one 流月 period using the upstream `iztro@2.5.8`
//! horoscope rule: the flowing month stem-branch comes from the target solar
//! date's normal-boundary month pillar, while the temporal Life palace index is
//! derived separately from the yearly index, natal lunar month, natal birth
//! hour, and target lunar month. It does not assemble daily/hourly layers,
//! attach temporal decorative arrays, or render narrative text.

use crate::core::error::ChartError;
use crate::core::model::calendar::{BirthTime, CalendarKind, SolarDay, SolarMonth};
use crate::core::model::chart::{
    Chart, PALACE_COUNT, TemporalPalaceLayout,
    temporal_layout::{build_life_branch_palace_layout, yin_first_branch_index},
};
use crate::core::model::star::mutagen::Scope;
use lunar_lite::{
    EarthlyBranch, LunarError, MonthDivide, SolarDate, StemBranch, StemBranchOptions, YearDivide,
    four_pillars_from_solar_date_with_options, solar_to_lunar,
};
use serde::{Deserialize, Serialize};

/// One 流月 period with independent month pillar and temporal Life palace facts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MonthlyPeriod {
    index: usize,
    lunar_month: u8,
    stem_branch: StemBranch,
    palace_branch: EarthlyBranch,
    palace_layout: TemporalPalaceLayout,
}

impl MonthlyPeriod {
    fn new(
        index: usize,
        lunar_month: u8,
        stem_branch: StemBranch,
        palace_branch: EarthlyBranch,
        palace_layout: TemporalPalaceLayout,
    ) -> Self {
        Self {
            index,
            lunar_month,
            stem_branch,
            palace_branch,
            palace_layout,
        }
    }

    /// Returns the zero-based upstream monthly index in Yin-first order.
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns the one-based target lunar month this period describes.
    pub const fn lunar_month(&self) -> u8 {
        self.lunar_month
    }

    /// Returns the target month stem-branch pair used for flow stars and mutagens.
    pub const fn stem_branch(&self) -> StemBranch {
        self.stem_branch
    }

    /// Returns the branch selected as the monthly temporal Life palace.
    pub const fn palace_branch(&self) -> EarthlyBranch {
        self.palace_branch
    }

    /// Returns the temporal palace-name layout for this monthly period.
    pub const fn palace_layout(&self) -> &TemporalPalaceLayout {
        &self.palace_layout
    }
}

/// Builds one 流月 period from natal chart facts and a target solar date/time.
pub fn build_monthly_period(
    natal: &Chart,
    target_solar_year: i32,
    target_solar_month: SolarMonth,
    target_solar_day: SolarDay,
    target_time: BirthTime,
) -> Result<MonthlyPeriod, ChartError> {
    let solar = SolarDate {
        year: target_solar_year,
        month: target_solar_month.value(),
        day: target_solar_day.value(),
    };
    let target_lunar = solar_to_lunar(solar).map_err(|err| {
        map_target_solar_error(
            err,
            target_solar_year,
            target_solar_month.value(),
            target_solar_day.value(),
        )
    })?;
    let pillars = four_pillars_from_solar_date_with_options(
        solar,
        target_time.iztro_time_index(),
        StemBranchOptions {
            year: YearDivide::Normal,
            month: MonthDivide::Normal,
        },
    )
    .map_err(|err| {
        map_target_solar_error(
            err,
            target_solar_year,
            target_solar_month.value(),
            target_solar_day.value(),
        )
    })?;

    let index = monthly_index(
        natal,
        target_lunar.year,
        target_lunar.month,
        target_lunar.day,
        target_lunar.is_leap_month,
    );
    let palace_branch = EarthlyBranch::Yin.offset(index as isize);
    let palace_layout = build_life_branch_palace_layout(Scope::Monthly, palace_branch)?;

    Ok(MonthlyPeriod::new(
        index,
        target_lunar.month,
        pillars.monthly,
        palace_branch,
        palace_layout,
    ))
}

fn monthly_index(
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

fn map_target_solar_error(err: LunarError, year: i32, month: u8, day: u8) -> ChartError {
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
