//! Deterministic daily-period (流日) facts derived from natal and target facts.
//!
//! This module derives one 流日 period using the upstream `iztro@2.5.8`
//! horoscope rule: the flowing day stem-branch is the target solar date's
//! normal-boundary day pillar, while the temporal Life palace index is derived
//! independently by counting on from the 流月 palace index by the target lunar
//! day. It does not assemble hourly layers, attach temporal decorative arrays,
//! or render narrative text.

use crate::core::error::ChartError;
use crate::core::model::calendar::{BirthTime, SolarDay, SolarMonth};
use crate::core::model::chart::{
    Chart, PALACE_COUNT, TemporalPalaceLayout,
    temporal_layout::{
        build_life_branch_palace_layout, map_target_solar_error, monthly_palace_index,
    },
};
use crate::core::model::star::mutagen::Scope;
use lunar_lite::{
    EarthlyBranch, MonthDivide, SolarDate, StemBranch, StemBranchOptions, YearDivide,
    four_pillars_from_solar_date_with_options, solar_to_lunar,
};
use serde::{Deserialize, Serialize};

/// One 流日 period with independent day pillar and temporal Life palace facts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DailyPeriod {
    index: usize,
    lunar_day: u8,
    stem_branch: StemBranch,
    palace_branch: EarthlyBranch,
    palace_layout: TemporalPalaceLayout,
}

impl DailyPeriod {
    fn new(
        index: usize,
        lunar_day: u8,
        stem_branch: StemBranch,
        palace_branch: EarthlyBranch,
        palace_layout: TemporalPalaceLayout,
    ) -> Self {
        Self {
            index,
            lunar_day,
            stem_branch,
            palace_branch,
            palace_layout,
        }
    }

    /// Returns the zero-based upstream daily index in Yin-first order.
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns the one-based target lunar day this period describes.
    pub const fn lunar_day(&self) -> u8 {
        self.lunar_day
    }

    /// Returns the target day stem-branch pair used for flow stars and mutagens.
    pub const fn stem_branch(&self) -> StemBranch {
        self.stem_branch
    }

    /// Returns the branch selected as the daily temporal Life palace.
    pub const fn palace_branch(&self) -> EarthlyBranch {
        self.palace_branch
    }

    /// Returns the temporal palace-name layout for this daily period.
    pub const fn palace_layout(&self) -> &TemporalPalaceLayout {
        &self.palace_layout
    }
}

/// Builds one 流日 period from natal chart facts and a target solar date/time.
pub fn build_daily_period(
    natal: &Chart,
    target_solar_year: i32,
    target_solar_month: SolarMonth,
    target_solar_day: SolarDay,
    target_time: BirthTime,
) -> Result<DailyPeriod, ChartError> {
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

    let monthly_index = monthly_palace_index(
        natal,
        target_lunar.year,
        target_lunar.month,
        target_lunar.day,
        target_lunar.is_leap_month,
    );
    let index = (monthly_index as isize + target_lunar.day as isize - 1)
        .rem_euclid(PALACE_COUNT as isize) as usize;
    let palace_branch = EarthlyBranch::Yin.offset(index as isize);
    let palace_layout = build_life_branch_palace_layout(Scope::Daily, palace_branch)?;

    Ok(DailyPeriod::new(
        index,
        target_lunar.day,
        pillars.daily,
        palace_branch,
        palace_layout,
    ))
}
