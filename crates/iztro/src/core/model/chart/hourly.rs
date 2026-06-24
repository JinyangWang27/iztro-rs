//! Deterministic hourly-period (流时) facts derived from natal and target facts.
//!
//! This module derives one 流时 period using the upstream `iztro@2.5.8`
//! horoscope rule: the flowing double-hour stem-branch is the target solar
//! date/time's normal-boundary hour pillar, while the temporal Life palace index
//! is derived independently by counting on from the 流日 palace index by the
//! target double-hour. It does not assemble the full horoscope stack, attach
//! temporal decorative arrays, or render narrative text.

use crate::core::calendar::solar_to_lunar;
use crate::core::error::ChartError;
use crate::core::model::calendar::{BirthTime, SolarDay, SolarMonth};
use crate::core::model::chart::{
    Chart, PALACE_COUNT, TemporalPalaceLayout,
    temporal_layout::{build_life_branch_palace_layout, daily_palace_index},
};
use crate::core::model::ganzhi::{EarthlyBranch, StemBranch};
use crate::core::model::star::mutagen::Scope;
use serde::{Deserialize, Serialize};

/// One 流时 period with independent hour pillar and temporal Life palace facts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HourlyPeriod {
    index: usize,
    time_index: u8,
    stem_branch: StemBranch,
    palace_branch: EarthlyBranch,
    palace_layout: TemporalPalaceLayout,
}

impl HourlyPeriod {
    fn new(
        index: usize,
        time_index: u8,
        stem_branch: StemBranch,
        palace_branch: EarthlyBranch,
        palace_layout: TemporalPalaceLayout,
    ) -> Self {
        Self {
            index,
            time_index,
            stem_branch,
            palace_branch,
            palace_layout,
        }
    }

    /// Returns the zero-based upstream hourly index in Yin-first order.
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns the upstream `timeIndex` (0..=12) this period describes.
    pub const fn time_index(&self) -> u8 {
        self.time_index
    }

    /// Returns the target hour stem-branch pair used for flow stars and mutagens.
    pub const fn stem_branch(&self) -> StemBranch {
        self.stem_branch
    }

    /// Returns the branch selected as the hourly temporal Life palace.
    pub const fn palace_branch(&self) -> EarthlyBranch {
        self.palace_branch
    }

    /// Returns the temporal palace-name layout for this hourly period.
    pub const fn palace_layout(&self) -> &TemporalPalaceLayout {
        &self.palace_layout
    }
}

/// Builds one 流时 period from natal chart facts and a target solar date/time.
pub fn build_hourly_period(
    natal: &Chart,
    target_solar_year: i32,
    target_solar_month: SolarMonth,
    target_solar_day: SolarDay,
    target_time: BirthTime,
) -> Result<HourlyPeriod, ChartError> {
    let time_index = target_time.iztro_time_index();
    let conversion = solar_to_lunar(
        target_solar_year,
        target_solar_month,
        target_solar_day,
        time_index,
    )?;

    let daily_index = daily_palace_index(
        natal,
        conversion.lunar_year(),
        conversion.lunar_month().value(),
        conversion.lunar_day().value(),
        conversion.is_leap_month(),
    );
    let index =
        (daily_index as isize + time_index as isize).rem_euclid(PALACE_COUNT as isize) as usize;
    let palace_branch = EarthlyBranch::Yin.offset(index as isize);
    let palace_layout = build_life_branch_palace_layout(Scope::Hourly, palace_branch)?;

    Ok(HourlyPeriod::new(
        index,
        time_index,
        conversion.four_pillars().hourly,
        palace_branch,
        palace_layout,
    ))
}
