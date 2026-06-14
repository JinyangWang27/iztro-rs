//! Empty and minimal natal chart builders.
//!
//! [`build_empty_chart`] produces the canonical empty twelve-palace scaffold;
//! [`build_minimal_natal_chart`] computes the Life/Body palaces, palace stems,
//! and five-element bureau on top of it. Star placement is layered on later by
//! [`crate::core::placement::natal::supported`].

use crate::core::error::ChartError;
use crate::core::model::bureau::five_element_bureau_from_life_palace;
use crate::core::model::calendar::BirthContext;
use crate::core::model::chart::{Chart, PALACE_COUNT, PALACE_NAMES, Palace, PalaceName};
use crate::core::model::profile::MethodProfile;
use crate::core::placement::natal::input::NatalChartInput;
use crate::core::placement::natal::life_body::{
    LunarBirthContext, calculate_life_body_palace_indices,
};
use crate::core::placement::natal::palace_stems::palace_stem_for_branch;
use lunar_lite::{EARTHLY_BRANCHES, EarthlyBranch, HEAVENLY_STEMS, StemBranch};

/// Builds a deterministic empty twelve-palace chart from typed chart metadata.
///
/// Palace names and earthly branches follow the canonical core order. Heavenly
/// stems are assigned cyclically as placeholder infrastructure only; this is
/// not the final Zi Wei Dou Shu palace stem-placement algorithm.
pub fn build_empty_chart(
    birth_context: BirthContext,
    birth_year: StemBranch,
    method_profile: MethodProfile,
) -> Result<Chart, ChartError> {
    let palaces = PALACE_NAMES
        .iter()
        .copied()
        .enumerate()
        .map(|(index, name)| {
            Palace::new(
                name,
                EARTHLY_BRANCHES[index],
                HEAVENLY_STEMS[index % HEAVENLY_STEMS.len()],
                Vec::new(),
            )
        })
        .collect();

    Chart::try_new(
        birth_context,
        birth_year,
        method_profile,
        palaces,
        None,
        None,
    )
}

/// Builds a minimal natal chart with calculated Life Palace layout.
///
/// The builder calculates the Life and Body palaces, assigns each palace its
/// Heavenly Stem from the birth year stem via 起五行寅例, remaps palace names
/// relative to the Life Palace, and derives the five-element bureau from the
/// Life Palace stem-branch pair. Stars are intentionally left empty;
/// leap-month handling and full chart generation are deferred.
pub fn build_minimal_natal_chart(input: NatalChartInput) -> Result<Chart, ChartError> {
    let empty_chart = build_empty_chart(
        input.birth_context().clone(),
        StemBranch::try_new(input.birth_year_stem(), input.birth_year_branch()).map_err(|err| {
            match err {
                lunar_lite::StemBranchError::InvalidStemBranchPair { stem, branch } => {
                    ChartError::InvalidStemBranchPair { stem, branch }
                }
            }
        })?,
        input.method_profile().clone(),
    )?;
    let indices = calculate_life_body_palace_indices(LunarBirthContext::new(
        input.lunar_month(),
        input.birth_context().birth_time(),
    ))?;
    let life_branch = indices.life_palace_branch();
    let year_stem = input.birth_year_stem();

    let palaces = empty_chart
        .palaces()
        .iter()
        .map(|palace| {
            let branch = palace.branch();
            Palace::new(
                palace_name_relative_to_life_branch(branch, life_branch),
                branch,
                palace_stem_for_branch(year_stem, branch),
                palace.stars().to_vec(),
            )
        })
        .collect();

    let life_pair =
        StemBranch::try_new(palace_stem_for_branch(year_stem, life_branch), life_branch).map_err(
            |err| match err {
                lunar_lite::StemBranchError::InvalidStemBranchPair { stem, branch } => {
                    ChartError::InvalidStemBranchPair { stem, branch }
                }
            },
        )?;
    let five_element_bureau = five_element_bureau_from_life_palace(life_pair);

    Chart::try_new(
        input.birth_context().clone(),
        empty_chart.birth_year(),
        input.method_profile().clone(),
        palaces,
        Some(indices.body_palace_branch()),
        Some(five_element_bureau),
    )
}

fn palace_name_relative_to_life_branch(
    branch: EarthlyBranch,
    life_branch: EarthlyBranch,
) -> PalaceName {
    let offset = (life_branch.index() + PALACE_COUNT - branch.index()) % PALACE_COUNT;

    PalaceName::from_index(offset)
}
