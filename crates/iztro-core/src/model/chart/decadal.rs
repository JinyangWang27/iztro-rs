//! Deterministic decadal-period frame facts derived from a natal chart.
//!
//! This module derives the conventional 12 ten-year 大限 frame only. It does
//! not assemble horoscope overlays, place temporal stars, attach mutagens, or
//! render narrative text.

use crate::error::ChartError;
use crate::model::calendar::Gender;
use crate::model::chart::{Chart, PalaceName};
use lunar_lite::{EarthlyBranch, HeavenlyStem, StemBranch};
use serde::{Deserialize, Serialize};

/// Direction used to walk decadal periods around the natal palace ring.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecadalDirection {
    /// Periods advance in increasing Earthly Branch order.
    Forward,
    /// Periods advance in decreasing Earthly Branch order.
    Reverse,
}

impl DecadalDirection {
    const fn step(self) -> isize {
        match self {
            Self::Forward => 1,
            Self::Reverse => -1,
        }
    }
}

/// One ten-year decadal period aligned to a natal palace.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DecadalPeriod {
    palace_branch: EarthlyBranch,
    palace_name: PalaceName,
    palace_stem: HeavenlyStem,
    start_age: u8,
    end_age: u8,
    stem_branch: StemBranch,
    direction: DecadalDirection,
}

impl DecadalPeriod {
    fn new(
        palace_branch: EarthlyBranch,
        palace_name: PalaceName,
        palace_stem: HeavenlyStem,
        start_age: u8,
        direction: DecadalDirection,
    ) -> Result<Self, ChartError> {
        let stem_branch =
            StemBranch::try_new(palace_stem, palace_branch).map_err(|err| match err {
                lunar_lite::StemBranchError::InvalidStemBranchPair { stem, branch } => {
                    ChartError::InvalidStemBranchPair { stem, branch }
                }
            })?;

        Ok(Self {
            palace_branch,
            palace_name,
            palace_stem,
            start_age,
            end_age: start_age + 9,
            stem_branch,
            direction,
        })
    }

    /// Returns the natal palace branch this period occupies.
    pub const fn palace_branch(&self) -> EarthlyBranch {
        self.palace_branch
    }

    /// Returns the natal palace name this period occupies.
    pub const fn palace_name(&self) -> PalaceName {
        self.palace_name
    }

    /// Returns the natal palace Heavenly Stem this period occupies.
    pub const fn palace_stem(&self) -> HeavenlyStem {
        self.palace_stem
    }

    /// Returns the inclusive starting age for this period.
    pub const fn start_age(&self) -> u8 {
        self.start_age
    }

    /// Returns the inclusive ending age for this period.
    pub const fn end_age(&self) -> u8 {
        self.end_age
    }

    /// Returns the palace stem-branch pair for this period.
    pub const fn stem_branch(&self) -> StemBranch {
        self.stem_branch
    }

    /// Returns the period traversal direction.
    pub const fn direction(&self) -> DecadalDirection {
        self.direction
    }
}

/// Complete 12-period decadal frame derived from natal chart facts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DecadalFrame {
    direction: DecadalDirection,
    periods: Vec<DecadalPeriod>,
}

impl DecadalFrame {
    fn new(direction: DecadalDirection, periods: Vec<DecadalPeriod>) -> Self {
        Self { direction, periods }
    }

    /// Returns the frame traversal direction.
    pub const fn direction(&self) -> DecadalDirection {
        self.direction
    }

    /// Returns the 12 ordered decadal periods.
    pub fn periods(&self) -> &[DecadalPeriod] {
        &self.periods
    }
}

/// Builds the 12-period decadal frame from natal chart facts.
pub fn build_decadal_frame(chart: &Chart) -> Result<DecadalFrame, ChartError> {
    let life_palace = chart
        .life_palace()
        .ok_or(ChartError::RequiredLifeBodyPalaceMissing)?;
    let start_age = chart
        .five_element_bureau()
        .ok_or(ChartError::RequiredFiveElementBureauMissing)?
        .number();
    let direction = decadal_direction(chart);
    let mut periods = Vec::with_capacity(crate::model::chart::PALACE_COUNT);

    for index in 0..crate::model::chart::PALACE_COUNT {
        let branch = life_palace
            .branch()
            .offset(index as isize * direction.step());
        let palace = chart
            .palaces()
            .iter()
            .find(|palace| palace.branch() == branch)
            .ok_or(ChartError::RequiredLifeBodyPalaceMissing)?;
        periods.push(DecadalPeriod::new(
            branch,
            palace.name(),
            palace.stem(),
            start_age + index as u8 * 10,
            direction,
        )?);
    }

    Ok(DecadalFrame::new(direction, periods))
}

fn decadal_direction(chart: &Chart) -> DecadalDirection {
    let is_yang_birth_year = chart.birth_year().stem().index() % 2 == 0;
    let is_forward = matches!(
        (is_yang_birth_year, chart.birth_context().gender()),
        (true, Gender::Male) | (false, Gender::Female)
    );

    if is_forward {
        DecadalDirection::Forward
    } else {
        DecadalDirection::Reverse
    }
}
