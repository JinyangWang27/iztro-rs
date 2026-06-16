//! Shared temporal palace-name layout helpers.
//!
//! These helpers build the common twelve-palace relabeling used by period
//! types whose Life palace is determined by a single Earthly Branch.

use crate::core::error::ChartError;
use crate::core::model::chart::{
    PALACE_COUNT, PalaceName, TemporalPalaceLayout, TemporalPalaceName,
};
use crate::core::model::star::mutagen::Scope;
use lunar_lite::EarthlyBranch;

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
