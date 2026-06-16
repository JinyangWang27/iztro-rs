//! Renderer-neutral read models for stacked natal and horoscope chart facts.
//!
//! A [`ChartStackSnapshot`] is an owned, serializable view of existing chart
//! facts arranged into the fixed visual twelve-palace grid. It does not derive
//! temporal facts, mutate natal data, or prescribe a renderer.

use crate::core::model::{
    bureau::FiveElementBureau,
    calendar::BirthContext,
    chart::{
        Chart, DecorativeStarPlacement, HoroscopeChart, MutagenActivation, PalaceName,
        ScopedStarPlacement, StarPlacement, TemporalContext, TemporalLayer,
    },
    profile::MethodProfile,
    star::{
        Brightness, StarCategory, StarKind, StarName,
        mutagen::{Mutagen, Scope},
    },
};
use lunar_lite::{EarthlyBranch, HeavenlyStem, StemBranch};
use serde::{Deserialize, Serialize};

/// Branch order for the renderer-ready 4x4 visual palace grid.
///
/// The center four grid cells are intentionally absent because Zi Wei Dou Shu
/// charts place twelve palaces around the perimeter.
pub const VISUAL_BRANCH_ORDER: [EarthlyBranch; 12] = [
    EarthlyBranch::Si,
    EarthlyBranch::Wu,
    EarthlyBranch::Wei,
    EarthlyBranch::Shen,
    EarthlyBranch::Chen,
    EarthlyBranch::You,
    EarthlyBranch::Mao,
    EarthlyBranch::Xu,
    EarthlyBranch::Yin,
    EarthlyBranch::Chou,
    EarthlyBranch::Zi,
    EarthlyBranch::Hai,
];

/// Returns the fixed 4x4 perimeter-grid position for an Earthly Branch.
pub const fn palace_grid_position(branch: EarthlyBranch) -> PalaceGridPosition {
    match branch {
        EarthlyBranch::Si => PalaceGridPosition::new(0, 0),
        EarthlyBranch::Wu => PalaceGridPosition::new(0, 1),
        EarthlyBranch::Wei => PalaceGridPosition::new(0, 2),
        EarthlyBranch::Shen => PalaceGridPosition::new(0, 3),
        EarthlyBranch::Chen => PalaceGridPosition::new(1, 0),
        EarthlyBranch::You => PalaceGridPosition::new(1, 3),
        EarthlyBranch::Mao => PalaceGridPosition::new(2, 0),
        EarthlyBranch::Xu => PalaceGridPosition::new(2, 3),
        EarthlyBranch::Yin => PalaceGridPosition::new(3, 0),
        EarthlyBranch::Chou => PalaceGridPosition::new(3, 1),
        EarthlyBranch::Zi => PalaceGridPosition::new(3, 2),
        EarthlyBranch::Hai => PalaceGridPosition::new(3, 3),
    }
}

/// Owned, renderer-neutral snapshot of natal facts and temporal overlays.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChartStackSnapshot {
    birth_context: BirthContext,
    birth_year: StemBranch,
    method_profile: MethodProfile,
    life_palace_branch: Option<EarthlyBranch>,
    body_palace_branch: Option<EarthlyBranch>,
    five_element_bureau: Option<FiveElementBureau>,
    layers: Vec<ChartLayerSnapshot>,
}

impl ChartStackSnapshot {
    /// Creates a one-layer stack snapshot from a natal chart.
    pub fn from_natal_chart(chart: &Chart) -> Self {
        Self {
            birth_context: chart.birth_context().clone(),
            birth_year: chart.birth_year(),
            method_profile: chart.method_profile().clone(),
            life_palace_branch: chart.life_palace().map(|palace| palace.branch()),
            body_palace_branch: chart.body_palace_branch(),
            five_element_bureau: chart.five_element_bureau(),
            layers: vec![ChartLayerSnapshot::from_natal_chart(chart)],
        }
    }

    /// Creates a stack snapshot from a horoscope chart and its temporal layers.
    pub fn from_horoscope_chart(chart: &HoroscopeChart) -> Self {
        let natal = chart.natal();
        let mut layers = Vec::with_capacity(chart.layers().len() + 1);
        layers.push(ChartLayerSnapshot::from_natal_chart(natal));
        layers.extend(chart.layers().iter().enumerate().map(|(index, layer)| {
            ChartLayerSnapshot::from_temporal_layer(natal, layer, index + 1)
        }));

        Self {
            birth_context: natal.birth_context().clone(),
            birth_year: natal.birth_year(),
            method_profile: natal.method_profile().clone(),
            life_palace_branch: natal.life_palace().map(|palace| palace.branch()),
            body_palace_branch: natal.body_palace_branch(),
            five_element_bureau: natal.five_element_bureau(),
            layers,
        }
    }

    /// Returns the birth context copied from the natal chart.
    pub const fn birth_context(&self) -> &BirthContext {
        &self.birth_context
    }

    /// Returns the birth-year stem-branch copied from the natal chart.
    pub const fn birth_year(&self) -> StemBranch {
        self.birth_year
    }

    /// Returns the method profile copied from the natal chart.
    pub const fn method_profile(&self) -> &MethodProfile {
        &self.method_profile
    }

    /// Returns the Life Palace branch copied from the natal chart, if present.
    pub const fn life_palace_branch(&self) -> Option<EarthlyBranch> {
        self.life_palace_branch
    }

    /// Returns the Body Palace branch copied from the natal chart, if present.
    pub const fn body_palace_branch(&self) -> Option<EarthlyBranch> {
        self.body_palace_branch
    }

    /// Returns the five-element bureau copied from the natal chart, if present.
    pub const fn five_element_bureau(&self) -> Option<FiveElementBureau> {
        self.five_element_bureau
    }

    /// Returns the ordered stack layers.
    pub fn layers(&self) -> &[ChartLayerSnapshot] {
        &self.layers
    }

    /// Returns the first layer with the requested kind.
    pub fn layer(&self, kind: ChartLayerKind) -> Option<&ChartLayerSnapshot> {
        self.layers.iter().find(|layer| layer.kind == kind)
    }
}

/// One snapshot layer in a chart stack.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChartLayerSnapshot {
    kind: ChartLayerKind,
    z_index: usize,
    context: Option<TemporalContext>,
    cells: Vec<PalaceLayerCellSnapshot>,
}

impl ChartLayerSnapshot {
    fn from_natal_chart(chart: &Chart) -> Self {
        let cells = VISUAL_BRANCH_ORDER
            .into_iter()
            .map(|branch| PalaceLayerCellSnapshot::from_natal_chart(chart, branch))
            .collect();

        Self {
            kind: ChartLayerKind::Natal,
            z_index: 0,
            context: None,
            cells,
        }
    }

    fn from_temporal_layer(chart: &Chart, layer: &TemporalLayer, z_index: usize) -> Self {
        let cells = VISUAL_BRANCH_ORDER
            .into_iter()
            .map(|branch| PalaceLayerCellSnapshot::from_temporal_layer(chart, layer, branch))
            .collect();

        Self {
            kind: ChartLayerKind::from_scope(layer.scope()),
            z_index,
            context: Some(*layer.context()),
            cells,
        }
    }

    /// Returns this layer's stack kind.
    pub const fn kind(&self) -> ChartLayerKind {
        self.kind
    }

    /// Returns this layer's z-index in the stack.
    pub const fn z_index(&self) -> usize {
        self.z_index
    }

    /// Returns this layer's temporal context, if it is non-natal.
    pub const fn context(&self) -> Option<&TemporalContext> {
        self.context.as_ref()
    }

    /// Returns cells ordered by [`VISUAL_BRANCH_ORDER`].
    pub fn cells(&self) -> &[PalaceLayerCellSnapshot] {
        &self.cells
    }
}

/// Layer kind used by renderer-neutral stack snapshots.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartLayerKind {
    /// Natal chart layer.
    Natal,
    /// Nominal-age temporal overlay.
    Age,
    /// Decadal temporal overlay.
    Decadal,
    /// Yearly temporal overlay.
    Yearly,
    /// Monthly temporal overlay.
    Monthly,
    /// Daily temporal overlay.
    Daily,
    /// Hourly temporal overlay.
    Hourly,
}

impl ChartLayerKind {
    /// Maps a fact scope to the corresponding snapshot layer kind.
    pub const fn from_scope(scope: Scope) -> Self {
        match scope {
            Scope::Natal => Self::Natal,
            Scope::Age => Self::Age,
            Scope::Decadal => Self::Decadal,
            Scope::Yearly => Self::Yearly,
            Scope::Monthly => Self::Monthly,
            Scope::Daily => Self::Daily,
            Scope::Hourly => Self::Hourly,
        }
    }
}

/// Position of a palace cell in the fixed 4x4 perimeter grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PalaceGridPosition {
    row: u8,
    column: u8,
}

impl PalaceGridPosition {
    /// Creates a grid position.
    pub const fn new(row: u8, column: u8) -> Self {
        Self { row, column }
    }

    /// Returns the zero-based row.
    pub const fn row(&self) -> u8 {
        self.row
    }

    /// Returns the zero-based column.
    pub const fn column(&self) -> u8 {
        self.column
    }
}

/// Renderer-neutral facts for one branch cell in one stack layer.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PalaceLayerCellSnapshot {
    branch: EarthlyBranch,
    grid_position: PalaceGridPosition,
    natal_palace_name: Option<PalaceName>,
    natal_palace_stem: Option<HeavenlyStem>,
    temporal_palace_name: Option<PalaceName>,
    roles: Vec<PalaceRoleSnapshot>,
    typed_stars: Vec<TypedStarSnapshot>,
    decorative_stars: Vec<DecorativeStarSnapshot>,
    scoped_stars: Vec<ScopedStarSnapshot>,
    mutagen_activations: Vec<MutagenActivationSnapshot>,
}

impl PalaceLayerCellSnapshot {
    fn from_natal_chart(chart: &Chart, branch: EarthlyBranch) -> Self {
        let palace = chart
            .palaces()
            .iter()
            .find(|palace| palace.branch() == branch);
        let mut roles = Vec::new();
        if let Some(palace) = palace {
            roles.push(PalaceRoleSnapshot::new(PalaceRoleKind::NatalPalace(
                palace.name(),
            )));
        }
        if chart.is_body_palace_branch(branch) {
            roles.push(PalaceRoleSnapshot::new(PalaceRoleKind::NatalBodyPalace));
        }

        Self {
            branch,
            grid_position: palace_grid_position(branch),
            natal_palace_name: palace.map(|palace| palace.name()),
            natal_palace_stem: palace.map(|palace| palace.stem()),
            temporal_palace_name: None,
            roles,
            typed_stars: palace
                .map(|palace| {
                    palace
                        .stars()
                        .iter()
                        .map(TypedStarSnapshot::from_star_placement)
                        .collect()
                })
                .unwrap_or_default(),
            decorative_stars: palace
                .map(|palace| {
                    palace
                        .decorative_stars()
                        .iter()
                        .map(DecorativeStarSnapshot::from_decorative_star_placement)
                        .collect()
                })
                .unwrap_or_default(),
            scoped_stars: Vec::new(),
            mutagen_activations: Vec::new(),
        }
    }

    fn from_temporal_layer(chart: &Chart, layer: &TemporalLayer, branch: EarthlyBranch) -> Self {
        let palace = chart
            .palaces()
            .iter()
            .find(|palace| palace.branch() == branch);

        Self {
            branch,
            grid_position: palace_grid_position(branch),
            natal_palace_name: palace.map(|palace| palace.name()),
            natal_palace_stem: palace.map(|palace| palace.stem()),
            temporal_palace_name: layer
                .palace_layout()
                .and_then(|layout| layout.name_for_branch(branch)),
            roles: Vec::new(),
            typed_stars: Vec::new(),
            decorative_stars: Vec::new(),
            scoped_stars: layer
                .placements()
                .iter()
                .filter(|placement| placement.branch() == branch)
                .map(ScopedStarSnapshot::from_scoped_star_placement)
                .collect(),
            mutagen_activations: layer
                .activations()
                .iter()
                .filter(|activation| activation.target_branch() == branch)
                .map(MutagenActivationSnapshot::from_mutagen_activation)
                .collect(),
        }
    }

    /// Returns the branch represented by this cell.
    pub const fn branch(&self) -> EarthlyBranch {
        self.branch
    }

    /// Returns this cell's fixed visual grid position.
    pub const fn grid_position(&self) -> PalaceGridPosition {
        self.grid_position
    }

    /// Returns the natal palace name joined by branch, if present.
    pub const fn natal_palace_name(&self) -> Option<PalaceName> {
        self.natal_palace_name
    }

    /// Returns the natal palace stem joined by branch, if present.
    pub const fn natal_palace_stem(&self) -> Option<HeavenlyStem> {
        self.natal_palace_stem
    }

    /// Returns the temporal palace name this layer assigns to the branch, if any.
    ///
    /// This is an additive overlay fact, kept separate from
    /// [`natal_palace_name`](Self::natal_palace_name): it is `None` on the natal
    /// layer and on temporal layers that carry no palace-name layout.
    pub const fn temporal_palace_name(&self) -> Option<PalaceName> {
        self.temporal_palace_name
    }

    /// Returns role markers for this cell.
    pub fn roles(&self) -> &[PalaceRoleSnapshot] {
        &self.roles
    }

    /// Returns natal typed stars in this cell.
    pub fn typed_stars(&self) -> &[TypedStarSnapshot] {
        &self.typed_stars
    }

    /// Returns natal decorative stars in this cell.
    pub fn decorative_stars(&self) -> &[DecorativeStarSnapshot] {
        &self.decorative_stars
    }

    /// Returns temporal scoped stars grouped into this cell.
    pub fn scoped_stars(&self) -> &[ScopedStarSnapshot] {
        &self.scoped_stars
    }

    /// Returns temporal mutagen activations grouped into this cell.
    pub fn mutagen_activations(&self) -> &[MutagenActivationSnapshot] {
        &self.mutagen_activations
    }
}

/// Role marker attached to a palace-layer cell.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PalaceRoleSnapshot {
    kind: PalaceRoleKind,
}

impl PalaceRoleSnapshot {
    /// Creates a role marker.
    pub const fn new(kind: PalaceRoleKind) -> Self {
        Self { kind }
    }

    /// Returns the role kind.
    pub const fn kind(&self) -> PalaceRoleKind {
        self.kind
    }
}

/// Role kinds attached to snapshot cells.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PalaceRoleKind {
    /// The cell contains the given natal palace.
    NatalPalace(PalaceName),
    /// The cell is the natal Body Palace branch.
    NatalBodyPalace,
}

/// Snapshot of a typed natal star placement.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TypedStarSnapshot {
    name: StarName,
    kind: StarKind,
    category: StarCategory,
    brightness: Brightness,
    mutagen: Option<Mutagen>,
    scope: Scope,
}

impl TypedStarSnapshot {
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

/// Snapshot of an untyped decorative natal star placement.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DecorativeStarSnapshot {
    name: StarName,
    family: crate::core::model::chart::DecorativeStarFamily,
    scope: Scope,
}

impl DecorativeStarSnapshot {
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
    pub const fn family(&self) -> crate::core::model::chart::DecorativeStarFamily {
        self.family
    }

    /// Returns the placement scope.
    pub const fn scope(&self) -> Scope {
        self.scope
    }
}

/// Snapshot of a branch-scoped temporal star placement.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScopedStarSnapshot {
    name: StarName,
    kind: StarKind,
    category: StarCategory,
    brightness: Brightness,
    mutagen: Option<Mutagen>,
    scope: Scope,
}

impl ScopedStarSnapshot {
    fn from_scoped_star_placement(placement: &ScopedStarPlacement) -> Self {
        Self {
            name: placement.placement().name(),
            kind: placement.placement().kind(),
            category: placement.placement().category(),
            brightness: placement.placement().brightness(),
            mutagen: placement.placement().mutagen(),
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

    /// Returns the mutagen attached to the scoped placement, if present.
    pub const fn mutagen(&self) -> Option<Mutagen> {
        self.mutagen
    }

    /// Returns the placement scope.
    pub const fn scope(&self) -> Scope {
        self.scope
    }
}

/// Snapshot of a temporal mutagen activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MutagenActivationSnapshot {
    source_scope: Scope,
    target_star: StarName,
    target_branch: EarthlyBranch,
    mutagen: Mutagen,
}

impl MutagenActivationSnapshot {
    fn from_mutagen_activation(activation: &MutagenActivation) -> Self {
        Self {
            source_scope: activation.source_scope(),
            target_star: activation.target_star(),
            target_branch: activation.target_branch(),
            mutagen: activation.mutagen(),
        }
    }

    /// Returns the temporal scope that produced this activation.
    pub const fn source_scope(&self) -> Scope {
        self.source_scope
    }

    /// Returns the star targeted by the activation.
    pub const fn target_star(&self) -> StarName {
        self.target_star
    }

    /// Returns the target branch.
    pub const fn target_branch(&self) -> EarthlyBranch {
        self.target_branch
    }

    /// Returns the mutagen applied to the target star.
    pub const fn mutagen(&self) -> Mutagen {
        self.mutagen
    }
}
