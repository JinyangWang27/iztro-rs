//! Deterministic nominal-age (小限 / Minor Limit) period facts derived from a
//! natal chart.
//!
//! This module derives one 小限 period from natal birth-year branch, gender, and
//! palace stem facts. It does not assemble a full horoscope, place flow stars,
//! derive yearly/monthly/daily/hourly facts, or render narrative text.
//!
//! 小限 ([`Scope::Age`]) is keyed by nominal age (虚岁) and is distinct from 流年
//! ([`Scope::Yearly`]), which is keyed by the selected calendar year's
//! stem-branch (太岁). Callers that need the active 小限 for a selected year
//! should resolve the nominal age first and then call [`build_age_period`];
//! they must not treat 小限 as another yearly stem-branch layer.

use crate::core::error::ChartError;
use crate::core::model::calendar::Gender;
use crate::core::model::chart::{
    Chart, PALACE_COUNT, TemporalPalaceLayout,
    temporal_layout::{build_life_branch_palace_layout, yin_first_branch_index},
};
use crate::core::model::star::mutagen::Scope;
use lunar_lite::{EarthlyBranch, StemBranch};
use serde::{Deserialize, Serialize};

/// One 小限 period aligned to a natal palace branch.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgePeriod {
    index: usize,
    nominal_age: u8,
    palace_branch: EarthlyBranch,
    stem_branch: StemBranch,
    palace_layout: TemporalPalaceLayout,
}

impl AgePeriod {
    fn new(
        index: usize,
        nominal_age: u8,
        palace_branch: EarthlyBranch,
        stem_branch: StemBranch,
        palace_layout: TemporalPalaceLayout,
    ) -> Self {
        Self {
            index,
            nominal_age,
            palace_branch,
            stem_branch,
            palace_layout,
        }
    }

    /// Returns the zero-based upstream age frame index in Yin-first order.
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns the one-based nominal age this period describes.
    pub const fn nominal_age(&self) -> u8 {
        self.nominal_age
    }

    /// Returns the natal palace branch selected by this nominal age.
    pub const fn palace_branch(&self) -> EarthlyBranch {
        self.palace_branch
    }

    /// Returns the selected natal palace stem-branch pair.
    pub const fn stem_branch(&self) -> StemBranch {
        self.stem_branch
    }

    /// Returns the temporal palace-name layout for this nominal age.
    pub const fn palace_layout(&self) -> &TemporalPalaceLayout {
        &self.palace_layout
    }
}

/// Builds one 小限 period from natal chart facts and a one-based nominal age.
pub fn build_age_period(natal: &Chart, nominal_age: u8) -> Result<AgePeriod, ChartError> {
    if !(1..=120).contains(&nominal_age) {
        return Err(ChartError::InvalidNominalAge { value: nominal_age });
    }

    let start_branch = age_start_branch(natal.birth_year().branch());
    let step = age_direction_step(natal.birth_context().gender());
    let offset = ((nominal_age - 1) % PALACE_COUNT as u8) as isize;
    let palace_branch = start_branch.offset(offset * step);
    let index = age_index(palace_branch);
    let palace = natal
        .palaces()
        .iter()
        .find(|palace| palace.branch() == palace_branch)
        .ok_or(ChartError::RequiredLifeBodyPalaceMissing)?;
    let stem_branch =
        StemBranch::try_new(palace.stem(), palace_branch).map_err(|err| match err {
            lunar_lite::StemBranchError::InvalidStemBranchPair { stem, branch } => {
                ChartError::InvalidStemBranchPair { stem, branch }
            }
        })?;
    let palace_layout = build_age_palace_layout(palace_branch)?;

    Ok(AgePeriod::new(
        index,
        nominal_age,
        palace_branch,
        stem_branch,
        palace_layout,
    ))
}

fn age_start_branch(birth_year_branch: EarthlyBranch) -> EarthlyBranch {
    match birth_year_branch {
        EarthlyBranch::Yin | EarthlyBranch::Wu | EarthlyBranch::Xu => EarthlyBranch::Chen,
        EarthlyBranch::Shen | EarthlyBranch::Zi | EarthlyBranch::Chen => EarthlyBranch::Xu,
        EarthlyBranch::Si | EarthlyBranch::You | EarthlyBranch::Chou => EarthlyBranch::Wei,
        EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Wei => EarthlyBranch::Chou,
    }
}

const fn age_direction_step(gender: Gender) -> isize {
    match gender {
        Gender::Male => 1,
        Gender::Female => -1,
    }
}

fn build_age_palace_layout(age_branch: EarthlyBranch) -> Result<TemporalPalaceLayout, ChartError> {
    build_life_branch_palace_layout(Scope::Age, age_branch)
}

fn age_index(branch: EarthlyBranch) -> usize {
    yin_first_branch_index(branch)
}
