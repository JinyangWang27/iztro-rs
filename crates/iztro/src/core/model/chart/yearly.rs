//! Deterministic yearly-period (流年) facts derived from a target lunar year.
//!
//! This module derives one yearly period from the target lunar year. It does not
//! assemble full horoscope output, derive monthly/daily/hourly facts, attach
//! yearly decorative arrays, or render narrative text.

use crate::core::error::ChartError;
use crate::core::model::chart::{
    TemporalPalaceLayout,
    temporal_layout::{build_life_branch_palace_layout, yin_first_branch_index},
};
use crate::core::model::ganzhi::{EarthlyBranch, StemBranch};
use crate::core::model::star::mutagen::Scope;
use serde::{Deserialize, Serialize};

/// One 流年 period aligned to the flowing year's branch.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct YearlyPeriod {
    index: usize,
    lunar_year: i32,
    palace_branch: EarthlyBranch,
    stem_branch: StemBranch,
    palace_layout: TemporalPalaceLayout,
}

impl YearlyPeriod {
    fn new(
        index: usize,
        lunar_year: i32,
        palace_branch: EarthlyBranch,
        stem_branch: StemBranch,
        palace_layout: TemporalPalaceLayout,
    ) -> Self {
        Self {
            index,
            lunar_year,
            palace_branch,
            stem_branch,
            palace_layout,
        }
    }

    /// Returns the zero-based upstream yearly index in Yin-first order.
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns the target lunar year this period describes.
    pub const fn lunar_year(&self) -> i32 {
        self.lunar_year
    }

    /// Returns the branch selected as the yearly Life palace.
    pub const fn palace_branch(&self) -> EarthlyBranch {
        self.palace_branch
    }

    /// Returns the target year's stem-branch pair.
    pub const fn stem_branch(&self) -> StemBranch {
        self.stem_branch
    }

    /// Returns the temporal palace-name layout for this yearly period.
    pub const fn palace_layout(&self) -> &TemporalPalaceLayout {
        &self.palace_layout
    }
}

/// Builds one 流年 period from a target lunar year.
pub fn build_yearly_period(lunar_year: i32) -> Result<YearlyPeriod, ChartError> {
    let stem_branch = StemBranch::from_lunar_year(lunar_year);
    let palace_branch = stem_branch.branch();
    let index = yin_first_branch_index(palace_branch);
    let palace_layout = build_life_branch_palace_layout(Scope::Yearly, palace_branch)?;

    Ok(YearlyPeriod::new(
        index,
        lunar_year,
        palace_branch,
        stem_branch,
        palace_layout,
    ))
}
