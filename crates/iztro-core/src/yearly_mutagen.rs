//! Yearly (流年) mutagen overlay builder.
//!
//! This is the first temporal algorithm layered on top of the model-only
//! horoscope overlays. Given an immutable natal [`Chart`] and an explicit yearly
//! stem-branch, [`build_yearly_mutagen_layer`] produces a [`Scope::Yearly`]
//! [`TemporalLayer`] whose [`MutagenActivation`]s apply the yearly Heavenly Stem
//! to the represented stars actually present in the natal chart.
//!
//! The layer is an overlay only: it never mutates the natal chart, never
//! duplicates natal stars, places no temporal/flow stars, and derives no
//! calendar facts. The yearly stem-branch and lunar year are supplied by the
//! caller. 四化 stay modeled as [`MutagenActivation`] facts, not independent
//! stars.

use crate::{
    chart::Chart,
    error::ChartError,
    horoscope::{TemporalContext, TemporalLayer, stem_mutagen_activations},
    mutagen::Scope,
    sexagenary::StemBranch,
};

/// Explicit yearly facts consumed by [`build_yearly_mutagen_layer`].
///
/// No calendar fact is derived: the yearly stem-branch and lunar year are stored
/// exactly as provided, mirroring the explicit-input style of the natal chart
/// builders and [`TemporalContext::Yearly`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct YearlyMutagenLayerInput {
    stem_branch: StemBranch,
    lunar_year: i32,
}

impl YearlyMutagenLayerInput {
    /// Creates input for the yearly mutagen overlay builder.
    pub const fn new(stem_branch: StemBranch, lunar_year: i32) -> Self {
        Self {
            stem_branch,
            lunar_year,
        }
    }

    /// Returns the explicit yearly stem-branch.
    pub const fn stem_branch(&self) -> StemBranch {
        self.stem_branch
    }

    /// Returns the explicit lunar year.
    pub const fn lunar_year(&self) -> i32 {
        self.lunar_year
    }
}

/// Builds a yearly [`TemporalLayer`] of mutagen activations for a natal chart.
///
/// The yearly Heavenly Stem comes from `input`'s stem-branch. For every
/// represented star placed in `natal`, the shared Heavenly Stem mutagen table
/// (via [`stem_mutagen_activations`]) decides whether the yearly stem maps that
/// star to a [`Mutagen`](crate::mutagen::Mutagen); the same 天干四化 table drives
/// both the birth-year (natal) and yearly (流年) transformations, so it is
/// reused rather than duplicated. Each mapped, present star yields one
/// [`Scope::Yearly`] [`MutagenActivation`](crate::horoscope::MutagenActivation)
/// targeting the branch of the palace it occupies.
///
/// Stars absent from `natal` (or not in the table, such as adjective stars)
/// produce no activation: iterating placed stars means an unsupported or missing
/// target is skipped rather than invented. The returned layer carries no star
/// placements and never restates or mutates natal facts.
pub fn build_yearly_mutagen_layer(
    natal: &Chart,
    input: YearlyMutagenLayerInput,
) -> Result<TemporalLayer, ChartError> {
    let activations = stem_mutagen_activations(natal, Scope::Yearly, input.stem_branch().stem());

    TemporalLayer::try_new(
        Scope::Yearly,
        TemporalContext::Yearly {
            stem_branch: input.stem_branch(),
            lunar_year: input.lunar_year(),
        },
        Vec::new(),
        activations,
    )
}
