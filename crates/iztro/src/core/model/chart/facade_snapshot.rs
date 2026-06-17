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
//! * the target numeric solar/lunar/time context retained by full stack assembly,
//!   with a lunar-only fallback for manually assembled full-layer charts;
//! * a minimal natal astrolabe snapshot derived from [`HoroscopeChart::natal`];
//! * the runtime palace projections of [`HoroscopeRuntime`] for the Life palace
//!   across each modeled scope.
//!
//! No placement logic is added here: every projection is produced by
//! [`HoroscopeRuntime`], and every supported-field block by
//! [`HoroscopeSupportedFieldsSnapshot`].
//!
//! Deferred, and intentionally absent from this payload:
//!
//! * the upstream localized `lunarDate` and `solarDate` strings;
//! * the complete upstream natal astrolabe payload with helper/query methods,
//!   localized labels, BaZi strings, decadal ranges, or age arrays;
//! * the runtime query helpers (`hasHoroscopeStars` and friends), which remain
//!   [`HoroscopeRuntime`] methods rather than precomputed DTO fields;
//! * full BaZi output, bindings, renderers, rules, and narrative.

use crate::core::{
    error::ChartError,
    model::{
        bureau::FiveElementBureau,
        calendar::Gender,
        chart::{
            Chart, DecorativeStarFamily, DecorativeStarPlacement, HoroscopeChart,
            HoroscopeLunarDate, HoroscopePalaceProjection, HoroscopeProjectionMutagenActivation,
            HoroscopeRuntime, HoroscopeSolarDate, HoroscopeSupportedFieldsSnapshot,
            HoroscopeSurroundPalaces, Palace, PalaceName, StarPlacement, TemporalContext,
        },
        star::{
            Brightness, StarCategory, StarKind, StarName,
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
/// adds the target `context` and the Life-palace runtime projections.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopeFacadeSnapshot {
    #[serde(flatten)]
    supported_fields: HoroscopeSupportedFieldsSnapshot,
    context: HoroscopeFacadeContext,
    astrolabe: NatalFacadeSnapshot,
    age_palace: HoroscopePalaceProjectionSnapshot,
    palace_projections: Vec<HoroscopePalaceProjectionSnapshot>,
    surround_palaces: Vec<HoroscopeSurroundPalacesSnapshot>,
}

impl HoroscopeFacadeSnapshot {
    /// Builds the facade payload from a full horoscope chart.
    ///
    /// Requires the full six-layer horoscope stack: the supported-field blocks,
    /// the runtime projections, and target context all read from modeled facts
    /// and so propagate the same
    /// [`ChartError`] those builders raise when a required layer, palace layout,
    /// or temporal context is missing.
    pub fn from_horoscope_chart(chart: &HoroscopeChart) -> Result<Self, ChartError> {
        let supported_fields = HoroscopeSupportedFieldsSnapshot::from_horoscope_chart(chart)?;
        let context = HoroscopeFacadeContext::from_horoscope_chart(chart)?;
        let astrolabe = NatalFacadeSnapshot::from_chart(chart.natal());
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
            astrolabe,
            age_palace,
            palace_projections,
            surround_palaces,
        })
    }

    /// Returns the reused supported-field blocks for every horoscope scope.
    pub const fn supported_fields(&self) -> &HoroscopeSupportedFieldsSnapshot {
        &self.supported_fields
    }

    /// Returns the target facade context.
    pub const fn context(&self) -> &HoroscopeFacadeContext {
        &self.context
    }

    /// Returns the minimal natal astrolabe snapshot derived from the natal chart.
    pub const fn astrolabe(&self) -> &NatalFacadeSnapshot {
        &self.astrolabe
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

/// Minimal serializable natal astrolabe snapshot embedded in the horoscope facade.
///
/// This is not the full upstream `astrolabe` object. It contains only stable
/// natal facts already modeled by [`Chart`] and deliberately excludes temporal
/// overlays, BaZi strings, runtime query helpers, decadal ranges, age arrays,
/// render data, rules, and readings.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NatalFacadeSnapshot {
    gender: Gender,
    birth_year_stem: HeavenlyStem,
    birth_year_branch: EarthlyBranch,
    five_element_bureau: Option<FiveElementBureau>,
    life_palace_branch: Option<EarthlyBranch>,
    body_palace_branch: Option<EarthlyBranch>,
    palaces: Vec<NatalFacadePalaceSnapshot>,
}

impl NatalFacadeSnapshot {
    /// Builds the natal facade snapshot from an already-assembled natal chart.
    pub fn from_chart(chart: &Chart) -> Self {
        Self {
            gender: chart.birth_context().gender(),
            birth_year_stem: chart.birth_year().stem(),
            birth_year_branch: chart.birth_year().branch(),
            five_element_bureau: chart.five_element_bureau(),
            life_palace_branch: chart.life_palace().map(Palace::branch),
            body_palace_branch: chart.body_palace_branch(),
            palaces: chart
                .palaces()
                .iter()
                .map(|palace| NatalFacadePalaceSnapshot::from_palace(chart, palace))
                .collect(),
        }
    }

    /// Returns the retained gender marker.
    pub const fn gender(&self) -> Gender {
        self.gender
    }

    /// Returns the birth-year Heavenly Stem.
    pub const fn birth_year_stem(&self) -> HeavenlyStem {
        self.birth_year_stem
    }

    /// Returns the birth-year Earthly Branch.
    pub const fn birth_year_branch(&self) -> EarthlyBranch {
        self.birth_year_branch
    }

    /// Returns the five-element bureau, if modeled.
    pub const fn five_element_bureau(&self) -> Option<FiveElementBureau> {
        self.five_element_bureau
    }

    /// Returns the Life Palace branch, if modeled.
    pub const fn life_palace_branch(&self) -> Option<EarthlyBranch> {
        self.life_palace_branch
    }

    /// Returns the Body Palace branch, if modeled.
    pub const fn body_palace_branch(&self) -> Option<EarthlyBranch> {
        self.body_palace_branch
    }

    /// Returns the natal palace snapshots in chart order.
    pub fn palaces(&self) -> &[NatalFacadePalaceSnapshot] {
        &self.palaces
    }
}

/// Minimal natal palace snapshot for the facade astrolabe.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NatalFacadePalaceSnapshot {
    branch: EarthlyBranch,
    name: PalaceName,
    stem: HeavenlyStem,
    roles: Vec<NatalFacadePalaceRole>,
    typed_stars: Vec<NatalFacadeTypedStarSnapshot>,
    decorative_stars: Vec<NatalFacadeDecorativeStarSnapshot>,
}

impl NatalFacadePalaceSnapshot {
    fn from_palace(chart: &Chart, palace: &Palace) -> Self {
        let mut roles = vec![NatalFacadePalaceRole::NatalPalace(palace.name())];
        if chart.is_body_palace_branch(palace.branch()) {
            roles.push(NatalFacadePalaceRole::NatalBodyPalace);
        }

        let mut typed_stars: Vec<NatalFacadeTypedStarSnapshot> = palace
            .stars()
            .iter()
            .map(NatalFacadeTypedStarSnapshot::from_star_placement)
            .collect();
        order_facade_typed_stars(&mut typed_stars);

        let mut decorative_stars: Vec<NatalFacadeDecorativeStarSnapshot> = palace
            .decorative_stars()
            .iter()
            .map(NatalFacadeDecorativeStarSnapshot::from_decorative_star_placement)
            .collect();
        order_facade_decorative_stars(&mut decorative_stars);

        Self {
            branch: palace.branch(),
            name: palace.name(),
            stem: palace.stem(),
            roles,
            typed_stars,
            decorative_stars,
        }
    }

    /// Returns the palace branch.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// Returns the natal palace name.
    pub const fn name(&self) -> PalaceName {
        self.name
    }

    /// Returns the natal palace stem.
    pub const fn stem(&self) -> HeavenlyStem {
        self.stem
    }

    /// Returns role markers for this natal palace.
    pub fn roles(&self) -> &[NatalFacadePalaceRole] {
        &self.roles
    }

    /// Returns typed natal stars in this palace.
    pub fn typed_stars(&self) -> &[NatalFacadeTypedStarSnapshot] {
        &self.typed_stars
    }

    /// Returns decorative natal stars in this palace.
    pub fn decorative_stars(&self) -> &[NatalFacadeDecorativeStarSnapshot] {
        &self.decorative_stars
    }
}

/// Role markers attached to a natal facade palace.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "palace_name", rename_all = "snake_case")]
pub enum NatalFacadePalaceRole {
    /// The cell contains this natal palace.
    NatalPalace(PalaceName),
    /// The cell is the Body Palace branch.
    NatalBodyPalace,
}

/// Typed natal star DTO for the facade astrolabe.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NatalFacadeTypedStarSnapshot {
    name: StarName,
    kind: StarKind,
    category: StarCategory,
    brightness: Brightness,
    mutagen: Option<Mutagen>,
    scope: Scope,
}

impl NatalFacadeTypedStarSnapshot {
    fn from_star_placement(placement: &StarPlacement) -> Self {
        Self {
            name: placement.name(),
            kind: placement.kind(),
            category: placement.category(),
            brightness: placement.brightness(),
            mutagen: placement.mutagen(),
            scope: placement.scope(),
        }
    }

    /// Returns the star name.
    pub const fn name(&self) -> StarName {
        self.name
    }

    /// Returns the fine star kind.
    pub const fn kind(&self) -> StarKind {
        self.kind
    }

    /// Returns the coarse star category.
    pub const fn category(&self) -> StarCategory {
        self.category
    }

    /// Returns the brightness state.
    pub const fn brightness(&self) -> Brightness {
        self.brightness
    }

    /// Returns the natal mutagen attached to the placement, if present.
    pub const fn mutagen(&self) -> Option<Mutagen> {
        self.mutagen
    }

    /// Returns the placement scope.
    pub const fn scope(&self) -> Scope {
        self.scope
    }
}

/// Decorative natal star DTO for the facade astrolabe.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NatalFacadeDecorativeStarSnapshot {
    name: StarName,
    family: DecorativeStarFamily,
    scope: Scope,
}

impl NatalFacadeDecorativeStarSnapshot {
    fn from_decorative_star_placement(placement: &DecorativeStarPlacement) -> Self {
        Self {
            name: placement.name(),
            family: placement.family(),
            scope: placement.scope(),
        }
    }

    /// Returns the decorative star name.
    pub const fn name(&self) -> StarName {
        self.name
    }

    /// Returns the decorative star family.
    pub const fn family(&self) -> DecorativeStarFamily {
        self.family
    }

    /// Returns the placement scope.
    pub const fn scope(&self) -> Scope {
        self.scope
    }
}

/// Deterministic facade ordering policy for exported palace star arrays.
///
/// Core engine placement facts are **order-independent**: a palace is the set of
/// stars placed in it, and the core compatibility tests compare those sets
/// without regard to `Vec` order (Rust and upstream TS `iztro` do not
/// necessarily emit stars in the same order). The facade/export layer, however,
/// must not depend on accidental `Vec` order, so this helper imposes one stable,
/// deterministic ordering for typed natal stars in a serialized palace:
///
/// * sort by `(kind, name, brightness, mutagen)`, using the canonical
///   declaration-order [`Ord`] of each typed enum.
///
/// This is a stable Rust-side canonical order. It is **not** a claim of byte
/// parity with the upstream TS `iztro` palace-star array order; full upstream
/// order parity is deferred unless separately targeted.
fn order_facade_typed_stars(stars: &mut [NatalFacadeTypedStarSnapshot]) {
    stars.sort_by_key(|star| (star.kind, star.name, star.brightness, star.mutagen));
}

/// Deterministic facade ordering policy for exported decorative palace stars.
///
/// Like [`order_facade_typed_stars`], this imposes a stable Rust-side canonical
/// order on the facade/export array without changing which stars are present:
///
/// * sort by `(family, name)`, using the canonical declaration-order [`Ord`] of
///   [`DecorativeStarFamily`] and [`StarName`].
///
/// It is not a claim of upstream TS array-order parity.
fn order_facade_decorative_stars(stars: &mut [NatalFacadeDecorativeStarSnapshot]) {
    stars.sort_by_key(|star| (star.family, star.name));
}

/// Target date/context fields exposed by the facade.
///
/// Full-stack-built charts retain numeric solar date, lunar date, leap-month
/// flag, and upstream `timeIndex`. Manually assembled charts without retained
/// target context still expose the older lunar year/month/day facts derived from
/// temporal layer contexts, with solar date and time index absent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HoroscopeFacadeContext {
    solar_date: Option<HoroscopeSolarDate>,
    lunar_date: HoroscopeLunarDate,
    time_index: Option<u8>,
}

impl HoroscopeFacadeContext {
    fn from_horoscope_chart(chart: &HoroscopeChart) -> Result<Self, ChartError> {
        if let Some(context) = chart.target_context() {
            return Ok(Self {
                solar_date: Some(context.solar_date()),
                lunar_date: context.lunar_date(),
                time_index: Some(context.time_index()),
            });
        }

        Ok(Self {
            solar_date: None,
            lunar_date: HoroscopeLunarDate::new(
                lunar_year(chart)?,
                lunar_month(chart)?,
                lunar_day(chart)?,
                false,
            ),
            time_index: None,
        })
    }

    /// Returns the retained target solar date, if available.
    pub const fn solar_date(&self) -> Option<HoroscopeSolarDate> {
        self.solar_date
    }

    /// Returns the target lunar date.
    pub const fn lunar_date(&self) -> HoroscopeLunarDate {
        self.lunar_date
    }

    /// Returns the retained upstream `iztro` target `timeIndex`, if available.
    pub const fn time_index(&self) -> Option<u8> {
        self.time_index
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
