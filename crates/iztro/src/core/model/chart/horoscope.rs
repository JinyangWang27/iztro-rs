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
//! `crate::core::placement::overlay::mutagen`.

use crate::core::{
    error::ChartError,
    model::{
        chart::{
            Chart, DecorativeStarFamily, DecorativeStarPlacement, PALACE_COUNT, PalaceName,
            StarPlacement,
        },
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
    /// Nominal-age period (小限).
    Age {
        /// Stem-branch of the nominal-age palace.
        stem_branch: StemBranch,
        /// One-based nominal age.
        nominal_age: u8,
    },
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
            Self::Age { .. } => Scope::Age,
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
            Self::Age { stem_branch, .. }
            | Self::Decadal { stem_branch, .. }
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
/// [`Palace`]: crate::core::model::chart::Palace
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

/// A branch-keyed untyped decorative star placement scoped to a temporal period.
///
/// Mirrors how upstream emits `yearlyDecStar` (岁前/将前十二神): bare decorative
/// names with no [`StarKind`], keyed by branch and owned by a non-natal period. It
/// reuses [`DecorativeStarPlacement`] for the name/family/scope fact and records
/// the [`EarthlyBranch`] directly, because a [`TemporalLayer`] is not
/// palace-structured — the same stable spatial reference [`ScopedStarPlacement`]
/// uses.
///
/// The [`Scope::Natal`] scope is rejected: natal decorative facts belong to
/// [`Palace::decorative_stars`], never to a temporal overlay. [`Deserialize`]
/// routes through [`ScopedDecorativeStarPlacement::try_new`] so that invariant
/// cannot be bypassed through serialized input.
///
/// [`StarKind`]: crate::core::model::star::StarKind
/// [`Palace`]: crate::core::model::chart::Palace
/// [`Palace::decorative_stars`]: crate::core::model::chart::Palace::decorative_stars
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ScopedDecorativeStarPlacement {
    branch: EarthlyBranch,
    placement: DecorativeStarPlacement,
}

impl ScopedDecorativeStarPlacement {
    /// Creates a branch-keyed temporal decorative placement after checking that it
    /// does not carry the natal scope.
    ///
    /// Returns [`ChartError::NatalScopeInTemporalLayer`] when `placement` carries
    /// [`Scope::Natal`]; temporal decorative facts are always non-natal.
    pub fn try_new(
        branch: EarthlyBranch,
        placement: DecorativeStarPlacement,
    ) -> Result<Self, ChartError> {
        if placement.scope() == Scope::Natal {
            return Err(ChartError::NatalScopeInTemporalLayer);
        }
        Ok(Self { branch, placement })
    }

    /// Returns the branch this decorative placement occupies.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// Returns the underlying untyped decorative placement.
    pub const fn placement(&self) -> &DecorativeStarPlacement {
        &self.placement
    }

    /// Returns the scope of the underlying decorative placement.
    pub const fn scope(&self) -> Scope {
        self.placement.scope()
    }

    /// Returns the decorative star name.
    pub const fn name(&self) -> StarName {
        self.placement.name()
    }

    /// Returns the decorative star family.
    pub const fn family(&self) -> DecorativeStarFamily {
        self.placement.family()
    }
}

impl<'de> Deserialize<'de> for ScopedDecorativeStarPlacement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// Mirror of [`ScopedDecorativeStarPlacement`]'s fields used only to decode
        /// raw input before the non-natal scope invariant is re-checked.
        #[derive(Deserialize)]
        struct ScopedDecorativeStarPlacementData {
            branch: EarthlyBranch,
            placement: DecorativeStarPlacement,
        }

        let data = ScopedDecorativeStarPlacementData::deserialize(deserializer)?;

        ScopedDecorativeStarPlacement::try_new(data.branch, data.placement)
            .map_err(serde::de::Error::custom)
    }
}

/// One temporal palace name assigned to a branch within a horoscope period.
///
/// A temporal period (e.g. a 大限) relabels the twelve palaces around the natal
/// branch ring. The name is keyed by [`EarthlyBranch`] — the stable spatial
/// reference — because palace *names* shift between scopes. This is an additive
/// overlay fact: it never replaces the natal [`PalaceName`] at that branch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TemporalPalaceName {
    branch: EarthlyBranch,
    palace_name: PalaceName,
}

impl TemporalPalaceName {
    /// Creates a branch-keyed temporal palace name.
    pub const fn new(branch: EarthlyBranch, palace_name: PalaceName) -> Self {
        Self {
            branch,
            palace_name,
        }
    }

    /// Returns the branch this temporal palace name occupies.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// Returns the temporal palace name assigned to the branch.
    pub const fn palace_name(&self) -> PalaceName {
        self.palace_name
    }
}

/// The twelve temporal palace names a non-natal period imposes on the branch ring.
///
/// The layout carries one [`Scope`] for all of its names; a single layout never
/// mixes scopes. The [`Scope::Natal`] scope is rejected — natal palace names
/// belong to the [`Chart`], not to a temporal overlay.
///
/// [`Deserialize`] routes through [`TemporalPalaceLayout::try_new`] so the
/// non-natal scope invariant cannot be bypassed through serialized input.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TemporalPalaceLayout {
    scope: Scope,
    names: Vec<TemporalPalaceName>,
}

impl TemporalPalaceLayout {
    /// Creates a temporal palace-name layout after checking its invariants.
    ///
    /// Rejects the natal scope (natal palace names are spatial facts of the
    /// [`Chart`], so a temporal layout always describes a non-natal period),
    /// requires exactly [`PALACE_COUNT`] names, and requires every
    /// [`EarthlyBranch`] and every [`PalaceName`] to appear exactly once — a
    /// well-formed layout relabels each of the twelve branch positions with a
    /// distinct palace name.
    pub fn try_new(scope: Scope, names: Vec<TemporalPalaceName>) -> Result<Self, ChartError> {
        if scope == Scope::Natal {
            return Err(ChartError::NatalScopeInTemporalLayer);
        }
        if names.len() != PALACE_COUNT {
            return Err(ChartError::InvalidTemporalPalaceLayoutCount {
                expected: PALACE_COUNT,
                actual: names.len(),
            });
        }
        for (index, name) in names.iter().enumerate() {
            if names[..index]
                .iter()
                .any(|seen| seen.branch() == name.branch())
            {
                return Err(ChartError::DuplicateTemporalPalaceLayoutBranch {
                    branch: name.branch(),
                });
            }
            if names[..index]
                .iter()
                .any(|seen| seen.palace_name() == name.palace_name())
            {
                return Err(ChartError::DuplicateTemporalPalaceLayoutName {
                    palace_name: name.palace_name(),
                });
            }
        }

        Ok(Self { scope, names })
    }

    /// Returns the non-natal scope this layout describes.
    pub const fn scope(&self) -> Scope {
        self.scope
    }

    /// Returns the branch-keyed temporal palace names.
    pub fn names(&self) -> &[TemporalPalaceName] {
        &self.names
    }

    /// Returns the temporal palace name assigned to `branch`, if present.
    pub fn name_for_branch(&self, branch: EarthlyBranch) -> Option<PalaceName> {
        self.names
            .iter()
            .find(|name| name.branch() == branch)
            .map(TemporalPalaceName::palace_name)
    }
}

impl<'de> Deserialize<'de> for TemporalPalaceLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// Mirror of [`TemporalPalaceLayout`]'s fields used only to decode raw
        /// input before the scope invariant is re-checked.
        #[derive(Deserialize)]
        struct TemporalPalaceLayoutData {
            scope: Scope,
            names: Vec<TemporalPalaceName>,
        }

        let data = TemporalPalaceLayoutData::deserialize(deserializer)?;

        TemporalPalaceLayout::try_new(data.scope, data.names).map_err(serde::de::Error::custom)
    }
}

/// A single temporal overlay on top of a natal chart.
///
/// A layer never restates natal facts: it carries only the star placements,
/// mutagen activations, and temporal palace-name layout scoped to one non-natal
/// period.
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
    palace_layout: Option<TemporalPalaceLayout>,
    /// Branch-keyed untyped decorative facts this period adds (e.g. yearly
    /// `yearlyDecStar`). Skipped when empty so layers without temporal decorative
    /// facts serialize unchanged. Kept separate from natal
    /// [`Palace::decorative_stars`](crate::core::model::chart::Palace::decorative_stars).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    decorative_placements: Vec<ScopedDecorativeStarPlacement>,
}

impl TemporalLayer {
    /// Creates a temporal overlay layer with no palace-name layout.
    ///
    /// This is the common constructor for layers that carry only scoped star
    /// placements and mutagen activations. Use
    /// [`TemporalLayer::try_new_with_palace_layout`] to attach a temporal
    /// palace-name layout. The same scope invariants are checked.
    pub fn try_new(
        scope: Scope,
        context: TemporalContext,
        placements: Vec<ScopedStarPlacement>,
        activations: Vec<MutagenActivation>,
    ) -> Result<Self, ChartError> {
        Self::try_new_with_palace_layout(scope, context, placements, activations, None)
    }

    /// Creates a temporal overlay layer with a palace-name layout but no temporal
    /// decorative placements.
    ///
    /// Convenience constructor for layers that carry scoped star placements,
    /// mutagen activations, and an optional palace-name layout. The temporal
    /// decorative list defaults to empty. Use
    /// [`TemporalLayer::try_new_with_palace_layout_and_decorative_stars`] to attach
    /// temporal decorative facts (e.g. yearly `yearlyDecStar`). The same scope
    /// invariants are checked.
    pub fn try_new_with_palace_layout(
        scope: Scope,
        context: TemporalContext,
        placements: Vec<ScopedStarPlacement>,
        activations: Vec<MutagenActivation>,
        palace_layout: Option<TemporalPalaceLayout>,
    ) -> Result<Self, ChartError> {
        Self::try_new_with_palace_layout_and_decorative_stars(
            scope,
            context,
            placements,
            activations,
            palace_layout,
            Vec::new(),
        )
    }

    /// Creates a temporal overlay layer after checking scope invariants.
    ///
    /// Rejects the natal scope (natal facts belong to the [`Chart`]), rejects a
    /// `scope` that disagrees with `context`, rejects any placement whose scope
    /// is not the layer scope, rejects any activation whose source scope is not
    /// the layer scope, rejects a palace layout whose scope is not the layer
    /// scope, and rejects any temporal decorative placement whose scope is not the
    /// layer scope, so a layer can never duplicate or restate a natal fact.
    pub fn try_new_with_palace_layout_and_decorative_stars(
        scope: Scope,
        context: TemporalContext,
        placements: Vec<ScopedStarPlacement>,
        activations: Vec<MutagenActivation>,
        palace_layout: Option<TemporalPalaceLayout>,
        decorative_placements: Vec<ScopedDecorativeStarPlacement>,
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
        if let Some(layout) = &palace_layout {
            if layout.scope() != scope {
                return Err(ChartError::TemporalPalaceLayoutScopeMismatch {
                    layer: scope,
                    layout: layout.scope(),
                });
            }
        }
        if let Some(decorative) = decorative_placements
            .iter()
            .find(|decorative| decorative.scope() != scope)
        {
            return Err(ChartError::TemporalDecorativeScopeMismatch {
                layer: scope,
                decorative: decorative.scope(),
            });
        }

        Ok(Self {
            scope,
            context,
            placements,
            activations,
            palace_layout,
            decorative_placements,
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

    /// Returns the temporal palace-name layout scoped to this layer, if any.
    pub const fn palace_layout(&self) -> Option<&TemporalPalaceLayout> {
        self.palace_layout.as_ref()
    }

    /// Returns the branch-keyed temporal decorative placements scoped to this layer.
    ///
    /// These are untyped decorative facts a period adds (e.g. yearly
    /// `yearlyDecStar`), kept distinct from natal
    /// [`Palace::decorative_stars`](crate::core::model::chart::Palace::decorative_stars).
    pub fn temporal_decorative_stars(&self) -> &[ScopedDecorativeStarPlacement] {
        &self.decorative_placements
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
            #[serde(default)]
            palace_layout: Option<TemporalPalaceLayout>,
            #[serde(default)]
            decorative_placements: Vec<ScopedDecorativeStarPlacement>,
        }

        let data = TemporalLayerData::deserialize(deserializer)?;

        TemporalLayer::try_new_with_palace_layout_and_decorative_stars(
            data.scope,
            data.context,
            data.placements,
            data.activations,
            data.palace_layout,
            data.decorative_placements,
        )
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
