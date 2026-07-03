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

/// Selects which earthly branch anchors the Life Palace when building a
/// minimal natal chart.
///
/// The Heaven plane uses [`NatalChartAnchor::CalculatedLifePalace`], deriving
/// the Life Palace from the birth month and hour. The Zhongzhou Earth and Human
/// planes re-anchor the Life Palace to an explicit branch
/// ([`NatalChartAnchor::ExplicitLifePalace`]) taken from the Heaven chart's Body
/// or Fortune (福德宫) palace respectively, without mutating any built chart.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum NatalChartAnchor {
    /// Use the normal Life-palace formula from birth month and hour.
    CalculatedLifePalace,

    /// Use this branch as the Life Palace anchor.
    ExplicitLifePalace(EarthlyBranch),
}

/// Builds a minimal natal chart with calculated Life Palace layout.
///
/// Thin wrapper over [`build_minimal_natal_chart_with_anchor`] using
/// [`NatalChartAnchor::CalculatedLifePalace`]; preserves the original Heaven
/// behaviour exactly.
pub fn build_minimal_natal_chart(input: NatalChartInput) -> Result<Chart, ChartError> {
    build_minimal_natal_chart_with_anchor(input, NatalChartAnchor::CalculatedLifePalace)
}

/// Builds a minimal natal chart, re-anchoring the Life Palace per `anchor`.
///
/// The builder calculates the Life and Body palaces, assigns each palace its
/// Heavenly Stem from the birth year stem via 起五行寅例, remaps palace names
/// relative to the (possibly re-anchored) Life Palace, and derives the
/// five-element bureau from the Life Palace stem-branch pair. The Body Palace
/// branch always reflects the calculated birth-month/hour value regardless of
/// anchor, so re-anchoring the Life Palace preserves the original Body Palace
/// fact. Stars are intentionally left empty; full chart generation is layered
/// on later.
pub fn build_minimal_natal_chart_with_anchor(
    input: NatalChartInput,
    anchor: NatalChartAnchor,
) -> Result<Chart, ChartError> {
    let empty_chart = build_empty_chart(
        input.birth_context().clone(),
        StemBranch::try_new(input.birth_year_stem(), input.birth_year_branch()).map_err(|err| {
            match err {
                lunar_lite::StemBranchError::InvalidStemBranchPair { stem, branch } => {
                    ChartError::InvalidStemBranchPair { stem, branch }
                }
                _ => ChartError::InvalidStemBranchPair {
                    stem: input.birth_year_stem(),
                    branch: input.birth_year_branch(),
                },
            }
        })?,
        input.method_profile().clone(),
    )?;
    let indices = calculate_life_body_palace_indices(LunarBirthContext::new(
        input.lunar_month(),
        input.birth_context().birth_time(),
    ))?;
    let life_branch = match anchor {
        NatalChartAnchor::CalculatedLifePalace => indices.life_palace_branch(),
        NatalChartAnchor::ExplicitLifePalace(branch) => branch,
    };
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
                _ => ChartError::InvalidStemBranchPair {
                    stem: palace_stem_for_branch(year_stem, life_branch),
                    branch: life_branch,
                },
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::model::calendar::{CalendarDate, Gender};
    use crate::core::placement::natal::life_body::LunarMonth;
    use lunar_lite::HeavenlyStem;

    fn fixture_input() -> NatalChartInput {
        let birth_context = BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        );
        NatalChartInput::new(
            birth_context,
            MethodProfile::placeholder("anchor_unit_test"),
            LunarMonth::new(4).expect("valid lunar month"),
            HeavenlyStem::Geng,
            EarthlyBranch::Wu,
        )
    }

    fn life_branch(chart: &Chart) -> EarthlyBranch {
        chart
            .life_palace()
            .expect("chart should have a Life Palace")
            .branch()
    }

    #[test]
    fn calculated_anchor_matches_default_builder() {
        let calculated = build_minimal_natal_chart(fixture_input()).expect("calculated chart");
        let via_anchor = build_minimal_natal_chart_with_anchor(
            fixture_input(),
            NatalChartAnchor::CalculatedLifePalace,
        )
        .expect("anchor chart");

        assert_eq!(calculated, via_anchor);
    }

    #[test]
    fn explicit_anchor_moves_life_palace_but_preserves_body_palace() {
        let calculated = build_minimal_natal_chart(fixture_input()).expect("calculated chart");
        let original_body = calculated
            .body_palace_branch()
            .expect("calculated chart should have a Body Palace branch");

        // Re-anchor the Life Palace to a branch that is not the calculated one.
        let target = if life_branch(&calculated) == EarthlyBranch::Zi {
            EarthlyBranch::Wu
        } else {
            EarthlyBranch::Zi
        };

        let reanchored = build_minimal_natal_chart_with_anchor(
            fixture_input(),
            NatalChartAnchor::ExplicitLifePalace(target),
        )
        .expect("re-anchored chart");

        assert_eq!(life_branch(&reanchored), target);
        // Body Palace fact reflects the original calculation, not the anchor.
        assert_eq!(reanchored.body_palace_branch(), Some(original_body));
        assert_ne!(life_branch(&reanchored), life_branch(&calculated));
    }
}
