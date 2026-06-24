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

use crate::core::model::ganzhi::{EarthlyBranch, FourPillars, HeavenlyStem, StemBranch};
use crate::core::{
    error::ChartError,
    labels::zh_cn,
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
/// overlays, runtime query helpers, decadal ranges, age arrays, render data,
/// rules, and readings.
///
/// The optional `four_pillars` field carries the factual natal four pillars
/// (年柱/月柱/日柱/时柱) retained on [`Chart::four_pillars`]. It is present for
/// `by_solar`-derived charts and absent (`None`) for `by_lunar`-derived charts,
/// which do not derive full pillars today. It is strictly a factual export: full
/// BaZi interpretation (十神, 藏干, 五行 scoring, 喜用神, 成格, readings) remains
/// deferred and is intentionally absent.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NatalFacadeSnapshot {
    gender: Gender,
    birth_year_stem: HeavenlyStem,
    birth_year_stem_zh: String,
    birth_year_branch: EarthlyBranch,
    birth_year_branch_zh: String,
    five_element_bureau: Option<FiveElementBureau>,
    life_palace_branch: Option<EarthlyBranch>,
    life_palace_branch_zh: Option<String>,
    body_palace_branch: Option<EarthlyBranch>,
    body_palace_branch_zh: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    four_pillars: Option<NatalFacadeFourPillarsSnapshot>,
    palaces: Vec<NatalFacadePalaceSnapshot>,
}

impl NatalFacadeSnapshot {
    /// Builds the natal facade snapshot from an already-assembled natal chart.
    pub fn from_chart(chart: &Chart) -> Self {
        let life_palace_branch = chart.life_palace().map(Palace::branch);
        let body_palace_branch = chart.body_palace_branch();
        Self {
            gender: chart.birth_context().gender(),
            birth_year_stem: chart.birth_year().stem(),
            birth_year_stem_zh: zh_cn::heavenly_stem_zh(chart.birth_year().stem()).to_owned(),
            birth_year_branch: chart.birth_year().branch(),
            birth_year_branch_zh: zh_cn::earthly_branch_zh(chart.birth_year().branch()).to_owned(),
            five_element_bureau: chart.five_element_bureau(),
            life_palace_branch,
            life_palace_branch_zh: life_palace_branch
                .map(|branch| zh_cn::earthly_branch_zh(branch).to_owned()),
            body_palace_branch,
            body_palace_branch_zh: body_palace_branch
                .map(|branch| zh_cn::earthly_branch_zh(branch).to_owned()),
            four_pillars: chart
                .four_pillars()
                .map(NatalFacadeFourPillarsSnapshot::from_four_pillars),
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

    /// Returns the Chinese label for the birth-year Heavenly Stem.
    pub fn birth_year_stem_zh(&self) -> &str {
        &self.birth_year_stem_zh
    }

    /// Returns the birth-year Earthly Branch.
    pub const fn birth_year_branch(&self) -> EarthlyBranch {
        self.birth_year_branch
    }

    /// Returns the Chinese label for the birth-year Earthly Branch.
    pub fn birth_year_branch_zh(&self) -> &str {
        &self.birth_year_branch_zh
    }

    /// Returns the five-element bureau, if modeled.
    pub const fn five_element_bureau(&self) -> Option<FiveElementBureau> {
        self.five_element_bureau
    }

    /// Returns the Life Palace branch, if modeled.
    pub const fn life_palace_branch(&self) -> Option<EarthlyBranch> {
        self.life_palace_branch
    }

    /// Returns the Chinese label for the Life Palace branch, if modeled.
    pub fn life_palace_branch_zh(&self) -> Option<&str> {
        self.life_palace_branch_zh.as_deref()
    }

    /// Returns the Body Palace branch, if modeled.
    pub const fn body_palace_branch(&self) -> Option<EarthlyBranch> {
        self.body_palace_branch
    }

    /// Returns the Chinese label for the Body Palace branch, if modeled.
    pub fn body_palace_branch_zh(&self) -> Option<&str> {
        self.body_palace_branch_zh.as_deref()
    }

    /// Returns the factual natal four pillars, if the chart retains them.
    ///
    /// Present for `by_solar`-derived charts; `None` for `by_lunar`-derived
    /// charts, which do not derive full pillars today.
    pub const fn four_pillars(&self) -> Option<&NatalFacadeFourPillarsSnapshot> {
        self.four_pillars.as_ref()
    }

    /// Returns the natal palace snapshots in chart order.
    pub fn palaces(&self) -> &[NatalFacadePalaceSnapshot] {
        &self.palaces
    }
}

/// Factual natal four-pillar (四柱) DTO for the facade astrolabe.
///
/// Reuses [`crate::core::model::ganzhi::FourPillars`] as the underlying fact: each pillar is a
/// machine-readable [`StemBranch`] with an additive conventional Chinese label.
/// This is a factual export only — it carries no 十神, 藏干, 五行 scoring, 喜用神,
/// 成格, readings, or any other BaZi interpretation, all of which remain deferred.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NatalFacadeFourPillarsSnapshot {
    yearly: StemBranch,
    yearly_zh: String,
    monthly: StemBranch,
    monthly_zh: String,
    daily: StemBranch,
    daily_zh: String,
    hourly: StemBranch,
    hourly_zh: String,
}

impl NatalFacadeFourPillarsSnapshot {
    fn from_four_pillars(pillars: &FourPillars) -> Self {
        Self {
            yearly: pillars.yearly,
            yearly_zh: zh_cn::stem_branch_zh(pillars.yearly),
            monthly: pillars.monthly,
            monthly_zh: zh_cn::stem_branch_zh(pillars.monthly),
            daily: pillars.daily,
            daily_zh: zh_cn::stem_branch_zh(pillars.daily),
            hourly: pillars.hourly,
            hourly_zh: zh_cn::stem_branch_zh(pillars.hourly),
        }
    }

    /// Returns the year pillar (年柱).
    pub const fn yearly(&self) -> StemBranch {
        self.yearly
    }

    /// Returns the Chinese label for the year pillar.
    pub fn yearly_zh(&self) -> &str {
        &self.yearly_zh
    }

    /// Returns the month pillar (月柱).
    pub const fn monthly(&self) -> StemBranch {
        self.monthly
    }

    /// Returns the Chinese label for the month pillar.
    pub fn monthly_zh(&self) -> &str {
        &self.monthly_zh
    }

    /// Returns the day pillar (日柱).
    pub const fn daily(&self) -> StemBranch {
        self.daily
    }

    /// Returns the Chinese label for the day pillar.
    pub fn daily_zh(&self) -> &str {
        &self.daily_zh
    }

    /// Returns the hour pillar (时柱).
    pub const fn hourly(&self) -> StemBranch {
        self.hourly
    }

    /// Returns the Chinese label for the hour pillar.
    pub fn hourly_zh(&self) -> &str {
        &self.hourly_zh
    }
}

/// Minimal natal palace snapshot for the facade astrolabe.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NatalFacadePalaceSnapshot {
    branch: EarthlyBranch,
    branch_zh: String,
    name: PalaceName,
    name_zh: String,
    stem: HeavenlyStem,
    stem_zh: String,
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
            branch_zh: zh_cn::earthly_branch_zh(palace.branch()).to_owned(),
            name: palace.name(),
            name_zh: zh_cn::palace_name_zh(palace.name()).to_owned(),
            stem: palace.stem(),
            stem_zh: zh_cn::heavenly_stem_zh(palace.stem()).to_owned(),
            roles,
            typed_stars,
            decorative_stars,
        }
    }

    /// Returns the palace branch.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// Returns the Chinese label for the palace branch.
    pub fn branch_zh(&self) -> &str {
        &self.branch_zh
    }

    /// Returns the natal palace name.
    pub const fn name(&self) -> PalaceName {
        self.name
    }

    /// Returns the Chinese label for the natal palace name.
    pub fn name_zh(&self) -> &str {
        &self.name_zh
    }

    /// Returns the natal palace stem.
    pub const fn stem(&self) -> HeavenlyStem {
        self.stem
    }

    /// Returns the Chinese label for the natal palace stem.
    pub fn stem_zh(&self) -> &str {
        &self.stem_zh
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
    name_zh: String,
    kind: StarKind,
    kind_zh: String,
    category: StarCategory,
    brightness: Brightness,
    brightness_zh: String,
    mutagen: Option<Mutagen>,
    mutagen_zh: Option<String>,
    scope: Scope,
}

impl NatalFacadeTypedStarSnapshot {
    fn from_star_placement(placement: &StarPlacement) -> Self {
        Self {
            name: placement.name(),
            name_zh: zh_cn::star_name_zh(placement.name()).to_owned(),
            kind: placement.kind(),
            kind_zh: zh_cn::star_kind_zh(placement.kind()).to_owned(),
            category: placement.category(),
            brightness: placement.brightness(),
            brightness_zh: zh_cn::brightness_zh(placement.brightness()).to_owned(),
            mutagen: placement.mutagen(),
            mutagen_zh: placement
                .mutagen()
                .map(|mutagen| zh_cn::mutagen_zh(mutagen).to_owned()),
            scope: placement.scope(),
        }
    }

    /// Returns the star name.
    pub const fn name(&self) -> StarName {
        self.name
    }

    /// Returns the Chinese label for the star name.
    pub fn name_zh(&self) -> &str {
        &self.name_zh
    }

    /// Returns the fine star kind.
    pub const fn kind(&self) -> StarKind {
        self.kind
    }

    /// Returns the Chinese label for the fine star kind.
    pub fn kind_zh(&self) -> &str {
        &self.kind_zh
    }

    /// Returns the coarse star category.
    pub const fn category(&self) -> StarCategory {
        self.category
    }

    /// Returns the brightness state.
    pub const fn brightness(&self) -> Brightness {
        self.brightness
    }

    /// Returns the Chinese label for the brightness state.
    pub fn brightness_zh(&self) -> &str {
        &self.brightness_zh
    }

    /// Returns the natal mutagen attached to the placement, if present.
    pub const fn mutagen(&self) -> Option<Mutagen> {
        self.mutagen
    }

    /// Returns the Chinese label for the attached mutagen, if present.
    pub fn mutagen_zh(&self) -> Option<&str> {
        self.mutagen_zh.as_deref()
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
    name_zh: String,
    family: DecorativeStarFamily,
    family_zh: String,
    scope: Scope,
}

impl NatalFacadeDecorativeStarSnapshot {
    fn from_decorative_star_placement(placement: &DecorativeStarPlacement) -> Self {
        Self {
            name: placement.name(),
            name_zh: zh_cn::star_name_zh(placement.name()).to_owned(),
            family: placement.family(),
            family_zh: zh_cn::decorative_star_family_zh(placement.family()).to_owned(),
            scope: placement.scope(),
        }
    }

    /// Returns the decorative star name.
    pub const fn name(&self) -> StarName {
        self.name
    }

    /// Returns the Chinese label for the decorative star name.
    pub fn name_zh(&self) -> &str {
        &self.name_zh
    }

    /// Returns the decorative star family.
    pub const fn family(&self) -> DecorativeStarFamily {
        self.family
    }

    /// Returns the Chinese label for the decorative star family.
    pub fn family_zh(&self) -> &str {
        &self.family_zh
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
