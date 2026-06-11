//! Horoscope (运限) overlay models for temporal chart layers.
//!
//! A [`HoroscopeChart`] wraps an immutable natal [`Chart`] and records zero or
//! more [`TemporalLayer`] overlays. Each layer carries a non-natal [`Scope`], a
//! typed [`TemporalContext`], the star placements scoped to that period, and the
//! mutagen activations that period triggers.
//!
//! These are models only. No temporal placement algorithm, calendar derivation,
//! or year-to-ganzhi conversion is performed here: every fact is supplied
//! explicitly by the caller. Natal placements are never duplicated into a layer;
//! a layer holds only the stars a period adds or re-scopes.
//!
//! The shared mutagen-activation builder that turns a Heavenly Stem into a
//! layer's [`MutagenActivation`]s lives with the overlay placement algorithms in
//! `crate::placement::overlay::mutagen`.

use crate::{
    error::ChartError,
    model::{
        chart::{Chart, StarPlacement},
        star::StarName,
        star::mutagen::{Mutagen, Scope},
    },
};
use lunar_lite::{EarthlyBranch, StemBranch};
use serde::{Deserialize, Deserializer, Serialize};

/// Typed temporal context distinguishing each non-natal horoscope scope.
///
/// Every variant records explicit, caller-supplied facts. No calendar fact is
/// derived: the period stem-branch and any period index are stored exactly as
/// provided.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemporalContext {
    /// Decadal period (大限).
    Decadal {
        /// Stem-branch of the decadal period.
        stem_branch: StemBranch,
        /// Explicit age at which the decade begins.
        start_age: u8,
    },
    /// Yearly period (流年).
    Yearly {
        /// Stem-branch of the flowing year.
        stem_branch: StemBranch,
        /// Explicit lunar year number, supplied rather than derived.
        lunar_year: i32,
    },
    /// Monthly period (流月).
    Monthly {
        /// Stem-branch of the flowing month.
        stem_branch: StemBranch,
        /// Explicit one-based lunar month, supplied rather than derived.
        lunar_month: u8,
    },
    /// Daily period (流日).
    Daily {
        /// Stem-branch of the flowing day.
        stem_branch: StemBranch,
        /// Explicit one-based lunar day, supplied rather than derived.
        lunar_day: u8,
    },
    /// Hourly period (流时).
    Hourly {
        /// Stem-branch of the flowing double-hour.
        stem_branch: StemBranch,
    },
}

impl TemporalContext {
    /// Returns the non-natal scope this context describes.
    pub const fn scope(&self) -> Scope {
        match self {
            Self::Decadal { .. } => Scope::Decadal,
            Self::Yearly { .. } => Scope::Yearly,
            Self::Monthly { .. } => Scope::Monthly,
            Self::Daily { .. } => Scope::Daily,
            Self::Hourly { .. } => Scope::Hourly,
        }
    }

    /// Returns the stem-branch of the period.
    pub const fn stem_branch(&self) -> StemBranch {
        match self {
            Self::Decadal { stem_branch, .. }
            | Self::Yearly { stem_branch, .. }
            | Self::Monthly { stem_branch, .. }
            | Self::Daily { stem_branch, .. }
            | Self::Hourly { stem_branch } => *stem_branch,
        }
    }
}

/// A scoped mutagen activation produced by a temporal period.
///
/// The target palace is referenced by its [`EarthlyBranch`]: palace *names*
/// shift between scopes, so the branch is the stable spatial reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MutagenActivation {
    source_scope: Scope,
    target_star: StarName,
    target_branch: EarthlyBranch,
    mutagen: Mutagen,
}

impl MutagenActivation {
    /// Creates a scoped mutagen activation fact.
    pub const fn new(
        source_scope: Scope,
        target_star: StarName,
        target_branch: EarthlyBranch,
        mutagen: Mutagen,
    ) -> Self {
        Self {
            source_scope,
            target_star,
            target_branch,
            mutagen,
        }
    }

    /// Returns the scope that produced this activation.
    pub const fn source_scope(&self) -> Scope {
        self.source_scope
    }

    /// Returns the star the mutagen lands on.
    pub const fn target_star(&self) -> StarName {
        self.target_star
    }

    /// Returns the branch of the palace the target star occupies.
    pub const fn target_branch(&self) -> EarthlyBranch {
        self.target_branch
    }

    /// Returns the mutagen applied to the target star.
    pub const fn mutagen(&self) -> Mutagen {
        self.mutagen
    }
}

/// A typed star placement positioned by branch within a temporal layer.
///
/// Natal [`StarPlacement`]s take their branch from the containing [`Palace`], but
/// a [`TemporalLayer`] is not palace-structured, so a flow placement records the
/// branch it occupies directly — the same stable spatial reference used by
/// [`MutagenActivation::target_branch`].
///
/// [`Palace`]: crate::model::chart::Palace
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScopedStarPlacement {
    branch: EarthlyBranch,
    placement: StarPlacement,
}

impl ScopedStarPlacement {
    /// Creates a branch-positioned temporal star placement.
    pub const fn new(branch: EarthlyBranch, placement: StarPlacement) -> Self {
        Self { branch, placement }
    }

    /// Returns the branch this placement occupies.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// Returns the underlying typed placement.
    pub const fn placement(&self) -> &StarPlacement {
        &self.placement
    }

    /// Returns the scope of the underlying placement.
    pub const fn scope(&self) -> Scope {
        self.placement.scope()
    }
}

/// A single temporal overlay on top of a natal chart.
///
/// A layer never restates natal facts: it carries only the star placements and
/// mutagen activations scoped to one non-natal period.
///
/// [`Deserialize`] is implemented by hand so decoding routes through
/// [`TemporalLayer::try_new`]; the scope invariants cannot be bypassed through
/// serialized input.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TemporalLayer {
    scope: Scope,
    context: TemporalContext,
    placements: Vec<ScopedStarPlacement>,
    activations: Vec<MutagenActivation>,
}

impl TemporalLayer {
    /// Creates a temporal overlay layer after checking scope invariants.
    ///
    /// Rejects the natal scope (natal facts belong to the [`Chart`]), rejects a
    /// `scope` that disagrees with `context`, rejects any placement whose scope
    /// is not the layer scope, and rejects any activation whose source scope is
    /// not the layer scope, so a layer can never duplicate or restate a natal
    /// fact.
    pub fn try_new(
        scope: Scope,
        context: TemporalContext,
        placements: Vec<ScopedStarPlacement>,
        activations: Vec<MutagenActivation>,
    ) -> Result<Self, ChartError> {
        if scope == Scope::Natal {
            return Err(ChartError::NatalScopeInTemporalLayer);
        }
        if scope != context.scope() {
            return Err(ChartError::TemporalScopeMismatch {
                layer: scope,
                context: context.scope(),
            });
        }
        if let Some(placement) = placements
            .iter()
            .find(|placement| placement.scope() != scope)
        {
            return Err(ChartError::TemporalPlacementScopeMismatch {
                layer: scope,
                placement: placement.scope(),
            });
        }
        if let Some(activation) = activations
            .iter()
            .find(|activation| activation.source_scope() != scope)
        {
            return Err(ChartError::TemporalActivationScopeMismatch {
                layer: scope,
                activation: activation.source_scope(),
            });
        }

        Ok(Self {
            scope,
            context,
            placements,
            activations,
        })
    }

    /// Returns the non-natal scope of this layer.
    pub const fn scope(&self) -> Scope {
        self.scope
    }

    /// Returns the typed temporal context of this layer.
    pub const fn context(&self) -> &TemporalContext {
        &self.context
    }

    /// Returns the branch-positioned star placements scoped to this layer.
    pub fn placements(&self) -> &[ScopedStarPlacement] {
        &self.placements
    }

    /// Returns the mutagen activations scoped to this layer.
    pub fn activations(&self) -> &[MutagenActivation] {
        &self.activations
    }
}

impl<'de> Deserialize<'de> for TemporalLayer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// Mirror of [`TemporalLayer`]'s fields used only to decode raw input
        /// before the scope invariants are re-checked.
        #[derive(Deserialize)]
        struct TemporalLayerData {
            scope: Scope,
            context: TemporalContext,
            placements: Vec<ScopedStarPlacement>,
            activations: Vec<MutagenActivation>,
        }

        let data = TemporalLayerData::deserialize(deserializer)?;

        TemporalLayer::try_new(data.scope, data.context, data.placements, data.activations)
            .map_err(serde::de::Error::custom)
    }
}

/// A natal chart with zero or more temporal overlay layers.
///
/// The wrapped natal [`Chart`] is exposed by shared reference only, keeping
/// natal facts immutable. Temporal facts are additive overlays: pushing a layer
/// never alters the natal chart.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopeChart {
    natal: Chart,
    layers: Vec<TemporalLayer>,
}

impl HoroscopeChart {
    /// Wraps a natal chart with no temporal layers.
    pub const fn new(natal: Chart) -> Self {
        Self {
            natal,
            layers: Vec::new(),
        }
    }

    /// Wraps a natal chart with the provided temporal layers.
    pub const fn with_layers(natal: Chart, layers: Vec<TemporalLayer>) -> Self {
        Self { natal, layers }
    }

    /// Returns the immutable natal chart.
    pub const fn natal(&self) -> &Chart {
        &self.natal
    }

    /// Returns the temporal overlay layers.
    pub fn layers(&self) -> &[TemporalLayer] {
        &self.layers
    }

    /// Appends a temporal overlay layer, leaving natal facts untouched.
    pub fn push_layer(&mut self, layer: TemporalLayer) {
        self.layers.push(layer);
    }

    /// Returns the overlay layers whose scope matches `scope`.
    pub fn layers_in_scope(&self, scope: Scope) -> impl Iterator<Item = &TemporalLayer> {
        self.layers
            .iter()
            .filter(move |layer| layer.scope() == scope)
    }

    /// Consumes the overlay, returning the wrapped natal chart.
    pub fn into_natal(self) -> Chart {
        self.natal
    }
}
