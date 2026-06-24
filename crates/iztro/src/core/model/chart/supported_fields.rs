//! Normalized compatibility snapshot for implemented horoscope supported fields.
//!
//! [`HoroscopeSupportedFieldsSnapshot`] is fixture-facing, not renderer-facing:
//! it exports only the full horoscope fact surface already modeled by
//! [`HoroscopeChart`] in the same normalized shape used by the committed
//! `iztro@2.5.8` supported-field fixture. It intentionally excludes upstream
//! runtime query helpers, runtime palace projections, embedded natal astrolabe
//! payloads, and raw localized labels.

use crate::core::model::ganzhi::{EarthlyBranch, HeavenlyStem};
use crate::core::{
    error::ChartError,
    model::{
        chart::{
            DecorativeStarFamily, HoroscopeChart, PalaceName, ScopedDecorativeStarPlacement,
            TemporalLayer, TemporalPalaceLayout,
        },
        star::{
            FlowStarBase, FlowStarScope, StarKind, StarName,
            mutagen::{Mutagen, Scope},
            try_flow_star_parts,
        },
    },
};
use serde::{Deserialize, Serialize};

const YIN_FIRST_BRANCH_ORDER: [EarthlyBranch; 12] = [
    EarthlyBranch::Yin,
    EarthlyBranch::Mao,
    EarthlyBranch::Chen,
    EarthlyBranch::Si,
    EarthlyBranch::Wu,
    EarthlyBranch::Wei,
    EarthlyBranch::Shen,
    EarthlyBranch::You,
    EarthlyBranch::Xu,
    EarthlyBranch::Hai,
    EarthlyBranch::Zi,
    EarthlyBranch::Chou,
];

/// Supported-field export for the implemented full horoscope stack.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopeSupportedFieldsSnapshot {
    decadal: HoroscopeFlowScopeSupportedFields,
    age: HoroscopeAgeSupportedFields,
    yearly: HoroscopeYearlySupportedFields,
    monthly: HoroscopeFlowScopeSupportedFields,
    daily: HoroscopeFlowScopeSupportedFields,
    hourly: HoroscopeFlowScopeSupportedFields,
}

impl HoroscopeSupportedFieldsSnapshot {
    /// Builds a normalized supported-field snapshot from a full horoscope chart.
    pub fn from_horoscope_chart(chart: &HoroscopeChart) -> Result<Self, ChartError> {
        let decadal = required_layer(chart, Scope::Decadal)?;
        let age = required_layer(chart, Scope::Age)?;
        let yearly = required_layer(chart, Scope::Yearly)?;
        let monthly = required_layer(chart, Scope::Monthly)?;
        let daily = required_layer(chart, Scope::Daily)?;
        let hourly = required_layer(chart, Scope::Hourly)?;

        Ok(Self {
            decadal: HoroscopeFlowScopeSupportedFields::from_layer(decadal)?,
            age: HoroscopeAgeSupportedFields::from_layer(age)?,
            yearly: HoroscopeYearlySupportedFields::from_layer(yearly)?,
            monthly: HoroscopeFlowScopeSupportedFields::from_layer(monthly)?,
            daily: HoroscopeFlowScopeSupportedFields::from_layer(daily)?,
            hourly: HoroscopeFlowScopeSupportedFields::from_layer(hourly)?,
        })
    }

    /// Returns the decadal supported-field block.
    pub const fn decadal(&self) -> &HoroscopeFlowScopeSupportedFields {
        &self.decadal
    }

    /// Returns the nominal-age supported-field block.
    pub const fn age(&self) -> &HoroscopeAgeSupportedFields {
        &self.age
    }

    /// Returns the yearly supported-field block.
    pub const fn yearly(&self) -> &HoroscopeYearlySupportedFields {
        &self.yearly
    }

    /// Returns the monthly supported-field block.
    pub const fn monthly(&self) -> &HoroscopeFlowScopeSupportedFields {
        &self.monthly
    }

    /// Returns the daily supported-field block.
    pub const fn daily(&self) -> &HoroscopeFlowScopeSupportedFields {
        &self.daily
    }

    /// Returns the hourly supported-field block.
    pub const fn hourly(&self) -> &HoroscopeFlowScopeSupportedFields {
        &self.hourly
    }
}

/// Supported fields common to every horoscope scope.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopeScopeSupportedFields {
    index: usize,
    name: Scope,
    heavenly_stem: HeavenlyStem,
    earthly_branch: EarthlyBranch,
    palace_names: Vec<HoroscopePalaceNameSupportedField>,
    mutagen: HoroscopeMutagenSupportedFields,
}

impl HoroscopeScopeSupportedFields {
    fn from_layer(layer: &TemporalLayer) -> Result<Self, ChartError> {
        let stem_branch = layer.context().stem_branch();
        let palace_names = palace_names(layer)?;
        let index = palace_names
            .iter()
            .position(|name| name.name == PalaceName::Life)
            .ok_or(ChartError::MissingHoroscopePalaceLayout {
                scope: layer.scope(),
            })?;

        Ok(Self {
            index,
            name: layer.scope(),
            heavenly_stem: stem_branch.stem(),
            earthly_branch: stem_branch.branch(),
            palace_names,
            mutagen: HoroscopeMutagenSupportedFields::from_layer(layer)?,
        })
    }

    /// Returns the Yin-first index of the period Life Palace.
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Returns the normalized scope name.
    pub const fn name(&self) -> Scope {
        self.name
    }

    /// Returns the period Heavenly Stem.
    pub const fn heavenly_stem(&self) -> HeavenlyStem {
        self.heavenly_stem
    }

    /// Returns the period Earthly Branch.
    pub const fn earthly_branch(&self) -> EarthlyBranch {
        self.earthly_branch
    }

    /// Returns temporal palace names in Yin-first order.
    pub fn palace_names(&self) -> &[HoroscopePalaceNameSupportedField] {
        &self.palace_names
    }

    /// Returns four-transform activations keyed by transform.
    pub const fn mutagen(&self) -> &HoroscopeMutagenSupportedFields {
        &self.mutagen
    }
}

/// Supported fields for the nominal-age scope.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopeAgeSupportedFields {
    #[serde(flatten)]
    common: HoroscopeScopeSupportedFields,
    nominal_age: u8,
}

impl HoroscopeAgeSupportedFields {
    fn from_layer(layer: &TemporalLayer) -> Result<Self, ChartError> {
        let nominal_age = match layer.context() {
            crate::core::model::chart::TemporalContext::Age { nominal_age, .. } => *nominal_age,
            _ => {
                return Err(ChartError::TemporalScopeMismatch {
                    layer: Scope::Age,
                    context: layer.context().scope(),
                });
            }
        };

        Ok(Self {
            common: HoroscopeScopeSupportedFields::from_layer(layer)?,
            nominal_age,
        })
    }

    /// Returns fields common to all horoscope scopes.
    pub const fn common(&self) -> &HoroscopeScopeSupportedFields {
        &self.common
    }

    /// Returns the one-based nominal age.
    pub const fn nominal_age(&self) -> u8 {
        self.nominal_age
    }
}

/// Supported fields for scopes with normalized flow-star matrices.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopeFlowScopeSupportedFields {
    #[serde(flatten)]
    common: HoroscopeScopeSupportedFields,
    flow_stars: Vec<HoroscopeFlowStarSupportedField>,
}

impl HoroscopeFlowScopeSupportedFields {
    fn from_layer(layer: &TemporalLayer) -> Result<Self, ChartError> {
        Ok(Self {
            common: HoroscopeScopeSupportedFields::from_layer(layer)?,
            flow_stars: flow_stars(layer)?.matrix,
        })
    }

    /// Returns fields common to all horoscope scopes.
    pub const fn common(&self) -> &HoroscopeScopeSupportedFields {
        &self.common
    }

    /// Returns normalized matrix flow stars sorted by fixture base order.
    pub fn flow_stars(&self) -> &[HoroscopeFlowStarSupportedField] {
        &self.flow_stars
    }
}

/// Supported fields for the yearly scope.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopeYearlySupportedFields {
    #[serde(flatten)]
    common: HoroscopeScopeSupportedFields,
    flow_stars: Vec<HoroscopeFlowStarSupportedField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nian_jie_branch: Option<EarthlyBranch>,
    yearly_dec_stars: HoroscopeYearlyDecorativeSupportedFields,
}

impl HoroscopeYearlySupportedFields {
    fn from_layer(layer: &TemporalLayer) -> Result<Self, ChartError> {
        let flow = flow_stars(layer)?;
        Ok(Self {
            common: HoroscopeScopeSupportedFields::from_layer(layer)?,
            flow_stars: flow.matrix,
            nian_jie_branch: flow.nian_jie_branch,
            yearly_dec_stars: HoroscopeYearlyDecorativeSupportedFields::from_layer(layer),
        })
    }

    /// Returns fields common to all horoscope scopes.
    pub const fn common(&self) -> &HoroscopeScopeSupportedFields {
        &self.common
    }

    /// Returns normalized matrix flow stars sorted by fixture base order.
    pub fn flow_stars(&self) -> &[HoroscopeFlowStarSupportedField] {
        &self.flow_stars
    }

    /// Returns the branch of yearly 年解, if present.
    pub const fn nian_jie_branch(&self) -> Option<EarthlyBranch> {
        self.nian_jie_branch
    }

    /// Returns yearly 岁前/将前 decorative families.
    pub const fn yearly_dec_stars(&self) -> &HoroscopeYearlyDecorativeSupportedFields {
        &self.yearly_dec_stars
    }
}

/// One temporal palace name in Yin-first fixture order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HoroscopePalaceNameSupportedField {
    name: PalaceName,
}

impl HoroscopePalaceNameSupportedField {
    /// Returns the normalized palace name.
    pub const fn name(&self) -> PalaceName {
        self.name
    }
}

/// Four-transform activation targets keyed by normalized transform name.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopeMutagenSupportedFields {
    lu: HoroscopeMutagenTargetSupportedField,
    quan: HoroscopeMutagenTargetSupportedField,
    ke: HoroscopeMutagenTargetSupportedField,
    ji: HoroscopeMutagenTargetSupportedField,
}

impl HoroscopeMutagenSupportedFields {
    fn from_layer(layer: &TemporalLayer) -> Result<Self, ChartError> {
        Ok(Self {
            lu: mutagen_target(layer, Mutagen::Lu)?,
            quan: mutagen_target(layer, Mutagen::Quan)?,
            ke: mutagen_target(layer, Mutagen::Ke)?,
            ji: mutagen_target(layer, Mutagen::Ji)?,
        })
    }

    /// Returns Lu activation target.
    pub const fn lu(&self) -> &HoroscopeMutagenTargetSupportedField {
        &self.lu
    }

    /// Returns Quan activation target.
    pub const fn quan(&self) -> &HoroscopeMutagenTargetSupportedField {
        &self.quan
    }

    /// Returns Ke activation target.
    pub const fn ke(&self) -> &HoroscopeMutagenTargetSupportedField {
        &self.ke
    }

    /// Returns Ji activation target.
    pub const fn ji(&self) -> &HoroscopeMutagenTargetSupportedField {
        &self.ji
    }
}

/// One mutagen target in fixture-normalized form.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HoroscopeMutagenTargetSupportedField {
    transform: Mutagen,
    star: StarName,
}

impl HoroscopeMutagenTargetSupportedField {
    /// Returns the transform.
    pub const fn transform(&self) -> Mutagen {
        self.transform
    }

    /// Returns the activated star.
    pub const fn star(&self) -> StarName {
        self.star
    }
}

/// One normalized matrix flow-star entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HoroscopeFlowStarSupportedField {
    base: FlowStarBase,
    branch: EarthlyBranch,
    #[serde(rename = "type")]
    kind: StarKind,
}

impl HoroscopeFlowStarSupportedField {
    /// Returns the normalized base identity.
    pub const fn base(&self) -> FlowStarBase {
        self.base
    }

    /// Returns the branch this flow star occupies.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// Returns the iztro-compatible star type.
    pub const fn kind(&self) -> StarKind {
        self.kind
    }
}

/// Yearly temporal decorative families normalized for fixture comparison.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HoroscopeYearlyDecorativeSupportedFields {
    suiqian12: Vec<HoroscopeYearlyDecorativeStarSupportedField>,
    jiangqian12: Vec<HoroscopeYearlyDecorativeStarSupportedField>,
}

impl HoroscopeYearlyDecorativeSupportedFields {
    fn from_layer(layer: &TemporalLayer) -> Self {
        Self {
            suiqian12: yearly_decorative_family(layer, DecorativeStarFamily::Suiqian12),
            jiangqian12: yearly_decorative_family(layer, DecorativeStarFamily::Jiangqian12),
        }
    }

    /// Returns yearly 岁前十二神 entries in Yin-first order.
    pub fn suiqian12(&self) -> &[HoroscopeYearlyDecorativeStarSupportedField] {
        &self.suiqian12
    }

    /// Returns yearly 将前十二神 entries in Yin-first order.
    pub fn jiangqian12(&self) -> &[HoroscopeYearlyDecorativeStarSupportedField] {
        &self.jiangqian12
    }
}

/// One yearly temporal decorative star entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HoroscopeYearlyDecorativeStarSupportedField {
    name: StarName,
    branch: EarthlyBranch,
}

impl HoroscopeYearlyDecorativeStarSupportedField {
    /// Returns the decorative star name.
    pub const fn name(&self) -> StarName {
        self.name
    }

    /// Returns the branch this yearly decorative star occupies.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }
}

struct FlowStarFields {
    matrix: Vec<HoroscopeFlowStarSupportedField>,
    nian_jie_branch: Option<EarthlyBranch>,
}

fn required_layer(chart: &HoroscopeChart, scope: Scope) -> Result<&TemporalLayer, ChartError> {
    let mut layers = chart.layers_in_scope(scope);
    let layer = layers
        .next()
        .ok_or(ChartError::MissingHoroscopeLayer { scope })?;
    if layers.next().is_some() {
        return Err(ChartError::DuplicateHoroscopeLayer { scope });
    }
    Ok(layer)
}

fn palace_names(
    layer: &TemporalLayer,
) -> Result<Vec<HoroscopePalaceNameSupportedField>, ChartError> {
    let layout = layer
        .palace_layout()
        .ok_or(ChartError::MissingHoroscopePalaceLayout {
            scope: layer.scope(),
        })?;

    YIN_FIRST_BRANCH_ORDER
        .into_iter()
        .map(|branch| palace_name(layout, branch, layer.scope()))
        .collect()
}

fn palace_name(
    layout: &TemporalPalaceLayout,
    branch: EarthlyBranch,
    scope: Scope,
) -> Result<HoroscopePalaceNameSupportedField, ChartError> {
    Ok(HoroscopePalaceNameSupportedField {
        name: layout
            .name_for_branch(branch)
            .ok_or(ChartError::MissingHoroscopePalaceLayout { scope })?,
    })
}

fn mutagen_target(
    layer: &TemporalLayer,
    mutagen: Mutagen,
) -> Result<HoroscopeMutagenTargetSupportedField, ChartError> {
    let activation = layer
        .activations()
        .iter()
        .find(|activation| activation.mutagen() == mutagen)
        .ok_or(ChartError::MissingHoroscopeMutagenActivation {
            scope: layer.scope(),
            mutagen,
        })?;

    Ok(HoroscopeMutagenTargetSupportedField {
        transform: mutagen,
        star: activation.target_star(),
    })
}

fn flow_stars(layer: &TemporalLayer) -> Result<FlowStarFields, ChartError> {
    let expected_scope =
        flow_scope_for(layer.scope()).ok_or(ChartError::FlowStarsUnavailableForScope {
            scope: layer.scope(),
        })?;
    let mut matrix = Vec::new();
    let mut nian_jie_branch = None;

    for placement in layer.placements() {
        let star = placement.placement().name();
        if layer.scope() == Scope::Yearly && star == StarName::NianJieYearly {
            nian_jie_branch = Some(placement.branch());
            continue;
        }

        let Some((scope, base)) = try_flow_star_parts(star) else {
            return Err(ChartError::InvalidHoroscopeFlowStar {
                scope: layer.scope(),
                star,
            });
        };
        if scope != expected_scope {
            return Err(ChartError::InvalidHoroscopeFlowStar {
                scope: layer.scope(),
                star,
            });
        }
        matrix.push(HoroscopeFlowStarSupportedField {
            base,
            branch: placement.branch(),
            kind: placement.placement().kind(),
        });
    }

    matrix.sort_by_key(|star| {
        (
            flow_base_sort_key(star.base),
            branch_yin_first_index(star.branch),
        )
    });

    Ok(FlowStarFields {
        matrix,
        nian_jie_branch,
    })
}

fn yearly_decorative_family(
    layer: &TemporalLayer,
    family: DecorativeStarFamily,
) -> Vec<HoroscopeYearlyDecorativeStarSupportedField> {
    let mut entries: Vec<_> = layer
        .temporal_decorative_stars()
        .iter()
        .filter(|placement| placement.family() == family)
        .map(yearly_decorative_star)
        .collect();
    entries.sort_by_key(|star| branch_yin_first_index(star.branch));
    entries
}

fn yearly_decorative_star(
    placement: &ScopedDecorativeStarPlacement,
) -> HoroscopeYearlyDecorativeStarSupportedField {
    HoroscopeYearlyDecorativeStarSupportedField {
        name: placement.name(),
        branch: placement.branch(),
    }
}

const fn flow_scope_for(scope: Scope) -> Option<FlowStarScope> {
    match scope {
        Scope::Decadal => Some(FlowStarScope::Decadal),
        Scope::Yearly => Some(FlowStarScope::Yearly),
        Scope::Monthly => Some(FlowStarScope::Monthly),
        Scope::Daily => Some(FlowStarScope::Daily),
        Scope::Hourly => Some(FlowStarScope::Hourly),
        Scope::Natal | Scope::Age => None,
    }
}

const fn flow_base_sort_key(base: FlowStarBase) -> u8 {
    match base {
        FlowStarBase::Chang => 0,
        FlowStarBase::Kui => 1,
        FlowStarBase::Lu => 2,
        FlowStarBase::Luan => 3,
        FlowStarBase::Ma => 4,
        FlowStarBase::Qu => 5,
        FlowStarBase::Tuo => 6,
        FlowStarBase::Xi => 7,
        FlowStarBase::Yang => 8,
        FlowStarBase::Yue => 9,
    }
}

fn branch_yin_first_index(branch: EarthlyBranch) -> usize {
    YIN_FIRST_BRANCH_ORDER
        .iter()
        .position(|candidate| *candidate == branch)
        .expect("all EarthlyBranch variants are in Yin-first order")
}
