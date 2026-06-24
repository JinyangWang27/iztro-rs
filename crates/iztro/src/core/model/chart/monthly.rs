//! Deterministic monthly-period (流月) facts derived from natal and target facts.
//!
//! This module derives one 流月 period using the upstream `iztro@2.5.8`
//! horoscope rule: the flowing month stem-branch comes from the target solar
//! date's normal-boundary month pillar, while the temporal Life palace index is
//! derived separately from the yearly index, natal lunar month, natal birth
//! hour, and target lunar month. It does not assemble daily/hourly layers,
//! attach temporal decorative arrays, or render narrative text.

use crate::core::calendar::solar_to_lunar;
use crate::core::error::ChartError;
use crate::core::model::calendar::{BirthTime, SolarDay, SolarMonth};
use crate::core::model::chart::{
    Chart, TemporalPalaceLayout,
    temporal_layout::{build_life_branch_palace_layout, monthly_palace_index},
};
use crate::core::model::ganzhi::{EarthlyBranch, StemBranch};
use crate::core::model::star::mutagen::Scope;
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
    let conversion = solar_to_lunar(
        target_solar_year,
        target_solar_month,
        target_solar_day,
        target_time.iztro_time_index(),
    )?;

    let index = monthly_palace_index(
        natal,
        conversion.lunar_year(),
        conversion.lunar_month().value(),
        conversion.lunar_day().value(),
        conversion.is_leap_month(),
    );
    let palace_branch = EarthlyBranch::Yin.offset(index as isize);
    let palace_layout = build_life_branch_palace_layout(Scope::Monthly, palace_branch)?;

    Ok(MonthlyPeriod::new(
        index,
        conversion.lunar_month().value(),
        conversion.four_pillars().monthly,
        palace_branch,
        palace_layout,
    ))
}
