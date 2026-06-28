//! Decadal (大限) mutagen overlay builder.
//!
//! This is the second temporal algorithm layered on top of the model-only
//! horoscope overlays, mirroring [`build_yearly_mutagen_layer`]. Given an
//! immutable natal [`Chart`] and an explicit decadal stem-branch,
//! [`build_decadal_mutagen_layer`] produces a [`Scope::Decadal`]
//! [`TemporalLayer`] whose
//! [`MutagenActivation`](crate::core::model::chart::horoscope::MutagenActivation)s
//! apply the decadal Heavenly Stem to the represented stars actually present in
//! the natal chart.
//!
//! The layer is an overlay only: it never mutates the natal chart, never
//! duplicates natal stars, places no temporal/flow stars (流曜), and derives no
//! calendar facts. The decadal stem-branch and starting age are supplied by the
//! caller; the decade's 大限命宫, decadal palace layout, and age-to-stem
//! derivation are not computed here. 四化 stay modeled as
//! [`MutagenActivation`](crate::core::model::chart::horoscope::MutagenActivation)
//! facts, not independent stars.
//!
//! [`build_yearly_mutagen_layer`]: crate::core::placement::overlay::yearly::build_yearly_mutagen_layer

use crate::core::error::ChartError;
use crate::core::model::chart::{Chart, TemporalContext, TemporalLayer};
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::overlay::mutagen::stem_mutagen_activations;
use lunar_lite::StemBranch;

/// Explicit decadal facts consumed by [`build_decadal_mutagen_layer`].
///
/// No calendar or age-range fact is derived: the decadal stem-branch and the
/// starting age are stored exactly as provided, mirroring the explicit-input
/// style of [`TemporalContext::Decadal`]. The decadal stem/branch is supplied by
/// the caller rather than derived from age, gender, five-element bureau, or natal
/// palace stems.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DecadalMutagenLayerInput {
    stem_branch: StemBranch,
    start_age: u8,
}

impl DecadalMutagenLayerInput {
    /// Creates input for the decadal mutagen overlay builder.
    pub const fn new(stem_branch: StemBranch, start_age: u8) -> Self {
        Self {
            stem_branch,
            start_age,
        }
    }

    /// Returns the explicit decadal stem-branch.
    pub const fn stem_branch(&self) -> StemBranch {
        self.stem_branch
    }

    /// Returns the explicit age at which the decade begins.
    pub const fn start_age(&self) -> u8 {
        self.start_age
    }
}

/// Builds a decadal [`TemporalLayer`] of mutagen activations for a natal chart.
///
/// The decadal Heavenly Stem comes from `input`'s stem-branch. For every
/// represented star placed in `natal`, the shared Heavenly Stem mutagen table
/// (via `stem_mutagen_activations`) decides whether the decadal stem maps that
/// star to a [`Mutagen`](crate::core::model::star::mutagen::Mutagen); the same 天干四化
/// table drives the birth-year (natal), yearly (流年), and decadal (大限)
/// transformations, so it is reused rather than duplicated. Each mapped, present
/// star yields one [`Scope::Decadal`]
/// [`MutagenActivation`](crate::core::model::chart::horoscope::MutagenActivation)
/// targeting the branch of the palace it occupies.
///
/// Stars absent from `natal` (or not in the table, such as adjective stars)
/// produce no activation: iterating placed stars means an unsupported or missing
/// target is skipped rather than invented. The returned layer carries no star
/// placements and never restates or mutates natal facts; the decadal palace
/// layout and 大限命宫 are not derived.
pub fn build_decadal_mutagen_layer(
    natal: &Chart,
    input: DecadalMutagenLayerInput,
) -> Result<TemporalLayer, ChartError> {
    let activations = stem_mutagen_activations(natal, Scope::Decadal, input.stem_branch().stem());

    TemporalLayer::try_new(
        Scope::Decadal,
        TemporalContext::Decadal {
            stem_branch: input.stem_branch(),
            start_age: input.start_age(),
        },
        Vec::new(),
        activations,
    )
}
