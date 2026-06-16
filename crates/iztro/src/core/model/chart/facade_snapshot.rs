//! Serializable horoscope facade payload snapshot.
//!
//! [`HoroscopeFacadeSnapshot`] is an export/facade layer, not a new engine layer.
//! It wraps the already-modeled full horoscope facts into one deterministic,
//! serializable payload that moves toward the upstream `iztro@2.5.8`
//! `FunctionalAstrolabe#horoscope` shape, while staying explicit about the parts
//! that remain deferred.
//!
//! It is assembled only from existing facts:
//!
//! * the normalized supported-field blocks of [`HoroscopeSupportedFieldsSnapshot`]
//!   (decadal/age/yearly/monthly/daily/hourly), reused verbatim;
//! * the target lunar-date context the modeled temporal layers already retain;
//! * the runtime palace projections of [`HoroscopeRuntime`] for the Life palace
//!   across each modeled scope.
//!
//! No placement logic is added here: every projection is produced by
//! [`HoroscopeRuntime`], and every supported-field block by
//! [`HoroscopeSupportedFieldsSnapshot`].
//!
//! Deferred, and intentionally absent from this payload:
//!
//! * the upstream localized `lunarDate` string, the `solarDate` string, and the
//!   target time index — [`HoroscopeChart`] does not retain the target instant,
//!   so these cannot be exposed exactly without recomputation;
//! * the re-embedded full natal astrolabe payload;
//! * the runtime query helpers (`hasHoroscopeStars` and friends), which remain
//!   [`HoroscopeRuntime`] methods rather than precomputed DTO fields;
//! * full BaZi output, bindings, renderers, rules, and narrative.

use crate::core::{
    error::ChartError,
    model::{
        chart::{
            HoroscopeChart, HoroscopePalaceProjection, HoroscopeProjectionMutagenActivation,
            HoroscopeRuntime, HoroscopeSupportedFieldsSnapshot, HoroscopeSurroundPalaces,
            PalaceName, TemporalContext,
        },
        star::{
            StarName,
            mutagen::{Mutagen, Scope},
        },
    },
};
use lunar_lite::{EarthlyBranch, HeavenlyStem};
use serde::{Deserialize, Serialize};

/// Scopes the facade projects the Life palace through, mirroring the upstream
/// split between `agePalace()` (the [`HoroscopeFacadeSnapshot::age_palace`] field)
/// and `palace(name, scope)` (these scopes). `age` is excluded here because it is
/// already exposed through `age_palace`.
const FACADE_PROJECTION_SCOPES: [Scope; 6] = [
    Scope::Natal,
    Scope::Decadal,
    Scope::Yearly,
    Scope::Monthly,
    Scope::Daily,
    Scope::Hourly,
];

/// Upstream-like, serializable snapshot of the modeled full horoscope surface.
///
/// The decadal/age/yearly/monthly/daily/hourly supported-field blocks are flattened
/// to the top level (reused from [`HoroscopeSupportedFieldsSnapshot`]); the facade
/// adds the target lunar-date `context` and the Life-palace runtime projections.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopeFacadeSnapshot {
    #[serde(flatten)]
    supported_fields: HoroscopeSupportedFieldsSnapshot,
    context: HoroscopeFacadeContext,
    age_palace: HoroscopePalaceProjectionSnapshot,
    palace_projections: Vec<HoroscopePalaceProjectionSnapshot>,
    surround_palaces: Vec<HoroscopeSurroundPalacesSnapshot>,
}

impl HoroscopeFacadeSnapshot {
    /// Builds the facade payload from a full horoscope chart.
    ///
    /// Requires the full six-layer horoscope stack: the supported-field blocks,
    /// the runtime projections, and the target lunar-date context all read from
    /// the modeled temporal layers and so propagate the same
    /// [`ChartError`] those builders raise when a required layer, palace layout,
    /// or temporal context is missing.
    pub fn from_horoscope_chart(chart: &HoroscopeChart) -> Result<Self, ChartError> {
        let supported_fields = HoroscopeSupportedFieldsSnapshot::from_horoscope_chart(chart)?;
        let context = HoroscopeFacadeContext::from_horoscope_chart(chart)?;
        let runtime = HoroscopeRuntime::new(chart)?;

        let age_palace = HoroscopePalaceProjectionSnapshot::from_projection(&runtime.age_palace()?);

        let mut palace_projections = Vec::with_capacity(FACADE_PROJECTION_SCOPES.len());
        let mut surround_palaces = Vec::with_capacity(FACADE_PROJECTION_SCOPES.len());
        for scope in FACADE_PROJECTION_SCOPES {
            palace_projections.push(HoroscopePalaceProjectionSnapshot::from_projection(
                &runtime.palace(scope, PalaceName::Life)?,
            ));
            surround_palaces.push(HoroscopeSurroundPalacesSnapshot::from_surround(
                &runtime.surround_palaces(scope, PalaceName::Life)?,
            ));
        }

        Ok(Self {
            supported_fields,
            context,
            age_palace,
            palace_projections,
            surround_palaces,
        })
    }

    /// Returns the reused supported-field blocks for every horoscope scope.
    pub const fn supported_fields(&self) -> &HoroscopeSupportedFieldsSnapshot {
        &self.supported_fields
    }

    /// Returns the target lunar-date context.
    pub const fn context(&self) -> &HoroscopeFacadeContext {
        &self.context
    }

    /// Returns the nominal-age Life palace projection (`agePalace`).
    pub const fn age_palace(&self) -> &HoroscopePalaceProjectionSnapshot {
        &self.age_palace
    }

    /// Returns the Life palace projections across each modeled scope.
    pub fn palace_projections(&self) -> &[HoroscopePalaceProjectionSnapshot] {
        &self.palace_projections
    }

    /// Returns the Life palace 三方四正 projections across each modeled scope.
    pub fn surround_palaces(&self) -> &[HoroscopeSurroundPalacesSnapshot] {
        &self.surround_palaces
    }
}

/// Target date/context fields the modeled horoscope retains.
///
/// Only the target lunar date is carried: the yearly, monthly, and daily
/// temporal contexts retain the target lunar year, month, and day respectively.
/// The upstream localized `lunarDate` string, the `solarDate` string, and the
/// target time index are not retained on [`HoroscopeChart`] and remain deferred.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HoroscopeFacadeContext {
    target_lunar_year: i32,
    target_lunar_month: u8,
    target_lunar_day: u8,
}

impl HoroscopeFacadeContext {
    fn from_horoscope_chart(chart: &HoroscopeChart) -> Result<Self, ChartError> {
        Ok(Self {
            target_lunar_year: lunar_year(chart)?,
            target_lunar_month: lunar_month(chart)?,
            target_lunar_day: lunar_day(chart)?,
        })
    }

    /// Returns the target lunar year carried by the yearly layer.
    pub const fn target_lunar_year(&self) -> i32 {
        self.target_lunar_year
    }

    /// Returns the one-based target lunar month carried by the monthly layer.
    pub const fn target_lunar_month(&self) -> u8 {
        self.target_lunar_month
    }

    /// Returns the one-based target lunar day carried by the daily layer.
    pub const fn target_lunar_day(&self) -> u8 {
        self.target_lunar_day
    }
}

/// Serializable form of a [`HoroscopePalaceProjection`].
///
/// Preserves the natal-versus-temporal split exactly as the runtime projection
/// produces it: the natal palace identity and stars stay separate from the
/// period's temporal palace name, temporal stars, and temporal mutagen
/// activations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopePalaceProjectionSnapshot {
    scope: Scope,
    requested_palace_name: PalaceName,
    branch: EarthlyBranch,
    natal_palace_name: PalaceName,
    temporal_palace_name: Option<PalaceName>,
    natal_palace_stem: HeavenlyStem,
    natal_typed_stars: Vec<StarName>,
    natal_decorative_stars: Vec<StarName>,
    temporal_stars: Vec<StarName>,
    temporal_decorative_stars: Vec<StarName>,
    temporal_mutagen_activations: Vec<HoroscopeProjectionMutagenActivationSnapshot>,
}

impl HoroscopePalaceProjectionSnapshot {
    fn from_projection(projection: &HoroscopePalaceProjection) -> Self {
        Self {
            scope: projection.scope(),
            requested_palace_name: projection.requested_palace_name(),
            branch: projection.branch(),
            natal_palace_name: projection.natal_palace_name(),
            temporal_palace_name: projection.temporal_palace_name(),
            natal_palace_stem: projection.natal_palace_stem(),
            natal_typed_stars: projection.natal_typed_stars().to_vec(),
            natal_decorative_stars: projection.natal_decorative_stars().to_vec(),
            temporal_stars: projection.temporal_stars().to_vec(),
            temporal_decorative_stars: projection.temporal_decorative_stars().to_vec(),
            temporal_mutagen_activations: projection
                .temporal_mutagen_activations()
                .iter()
                .map(HoroscopeProjectionMutagenActivationSnapshot::from_activation)
                .collect(),
        }
    }

    /// Returns the scope used to select the projection branch.
    pub const fn scope(&self) -> Scope {
        self.scope
    }

    /// Returns the palace name requested by the facade.
    pub const fn requested_palace_name(&self) -> PalaceName {
        self.requested_palace_name
    }

    /// Returns the stable branch identity of this projection.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// Returns the natal palace name at this branch.
    pub const fn natal_palace_name(&self) -> PalaceName {
        self.natal_palace_name
    }

    /// Returns the temporal palace name assigned to this branch, if any.
    pub const fn temporal_palace_name(&self) -> Option<PalaceName> {
        self.temporal_palace_name
    }

    /// Returns the natal palace stem at this branch.
    pub const fn natal_palace_stem(&self) -> HeavenlyStem {
        self.natal_palace_stem
    }

    /// Returns natal typed star names at this branch.
    pub fn natal_typed_stars(&self) -> &[StarName] {
        &self.natal_typed_stars
    }

    /// Returns natal decorative star names at this branch.
    pub fn natal_decorative_stars(&self) -> &[StarName] {
        &self.natal_decorative_stars
    }

    /// Returns temporal scoped star names at this branch.
    pub fn temporal_stars(&self) -> &[StarName] {
        &self.temporal_stars
    }

    /// Returns temporal decorative star names at this branch.
    pub fn temporal_decorative_stars(&self) -> &[StarName] {
        &self.temporal_decorative_stars
    }

    /// Returns temporal mutagen activations at this branch.
    pub fn temporal_mutagen_activations(&self) -> &[HoroscopeProjectionMutagenActivationSnapshot] {
        &self.temporal_mutagen_activations
    }
}

/// Serializable form of a [`HoroscopeProjectionMutagenActivation`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HoroscopeProjectionMutagenActivationSnapshot {
    target_star: StarName,
    mutagen: Mutagen,
}

impl HoroscopeProjectionMutagenActivationSnapshot {
    fn from_activation(activation: &HoroscopeProjectionMutagenActivation) -> Self {
        Self {
            target_star: activation.target_star(),
            mutagen: activation.mutagen(),
        }
    }

    /// Returns the activated natal star.
    pub const fn target_star(&self) -> StarName {
        self.target_star
    }

    /// Returns the transform applied to the star.
    pub const fn mutagen(&self) -> Mutagen {
        self.mutagen
    }
}

/// Serializable form of a [`HoroscopeSurroundPalaces`] 三方四正 projection.
///
/// The `scope` and `requested_palace_name` are lifted from the target projection
/// so the surround block mirrors the upstream facade shape.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopeSurroundPalacesSnapshot {
    scope: Scope,
    requested_palace_name: PalaceName,
    target: HoroscopePalaceProjectionSnapshot,
    opposite: HoroscopePalaceProjectionSnapshot,
    wealth: HoroscopePalaceProjectionSnapshot,
    career: HoroscopePalaceProjectionSnapshot,
}

impl HoroscopeSurroundPalacesSnapshot {
    fn from_surround(surround: &HoroscopeSurroundPalaces) -> Self {
        let target = HoroscopePalaceProjectionSnapshot::from_projection(surround.target());
        Self {
            scope: target.scope(),
            requested_palace_name: target.requested_palace_name(),
            target,
            opposite: HoroscopePalaceProjectionSnapshot::from_projection(surround.opposite()),
            wealth: HoroscopePalaceProjectionSnapshot::from_projection(surround.wealth()),
            career: HoroscopePalaceProjectionSnapshot::from_projection(surround.career()),
        }
    }

    /// Returns the scope used to select the projection branches.
    pub const fn scope(&self) -> Scope {
        self.scope
    }

    /// Returns the palace name requested by the facade.
    pub const fn requested_palace_name(&self) -> PalaceName {
        self.requested_palace_name
    }

    /// Returns the target palace projection.
    pub const fn target(&self) -> &HoroscopePalaceProjectionSnapshot {
        &self.target
    }

    /// Returns the opposite palace projection.
    pub const fn opposite(&self) -> &HoroscopePalaceProjectionSnapshot {
        &self.opposite
    }

    /// Returns the 财帛位 projection in the upstream 三方四正 convention.
    pub const fn wealth(&self) -> &HoroscopePalaceProjectionSnapshot {
        &self.wealth
    }

    /// Returns the 官禄位 projection in the upstream 三方四正 convention.
    pub const fn career(&self) -> &HoroscopePalaceProjectionSnapshot {
        &self.career
    }
}

fn single_layer_context(
    chart: &HoroscopeChart,
    scope: Scope,
) -> Result<&TemporalContext, ChartError> {
    let mut layers = chart.layers_in_scope(scope);
    let layer = layers
        .next()
        .ok_or(ChartError::MissingHoroscopeLayer { scope })?;
    if layers.next().is_some() {
        return Err(ChartError::DuplicateHoroscopeLayer { scope });
    }
    Ok(layer.context())
}

fn lunar_year(chart: &HoroscopeChart) -> Result<i32, ChartError> {
    match single_layer_context(chart, Scope::Yearly)? {
        TemporalContext::Yearly { lunar_year, .. } => Ok(*lunar_year),
        context => Err(ChartError::TemporalScopeMismatch {
            layer: Scope::Yearly,
            context: context.scope(),
        }),
    }
}

fn lunar_month(chart: &HoroscopeChart) -> Result<u8, ChartError> {
    match single_layer_context(chart, Scope::Monthly)? {
        TemporalContext::Monthly { lunar_month, .. } => Ok(*lunar_month),
        context => Err(ChartError::TemporalScopeMismatch {
            layer: Scope::Monthly,
            context: context.scope(),
        }),
    }
}

fn lunar_day(chart: &HoroscopeChart) -> Result<u8, ChartError> {
    match single_layer_context(chart, Scope::Daily)? {
        TemporalContext::Daily { lunar_day, .. } => Ok(*lunar_day),
        context => Err(ChartError::TemporalScopeMismatch {
            layer: Scope::Daily,
            context: context.scope(),
        }),
    }
}
