//! A renderer-neutral static 12-palace chart view model.
//!
//! [`StaticChartViewSnapshot`] is the GUI-facing read model that backs a future static chart.
//! It is one selected chart slice: a center panel,
//! twelve perimeter palaces laid out on the conventional 4x4 grid, scope-selector
//! state (本命/大限/小限/流年/流月/流日/流时), optional temporal overlays, and a
//! reserved (currently always empty) set of highlight annotations.
//!
//! The model is owned, serializable, and deterministic. It reuses the existing
//! grid layout ([`palace_grid_position`]) and the deterministic facade star
//! ordering so a renderer never has to depend on accidental `Vec` order.

use crate::core::labels::zh_cn;
use crate::core::model::bureau::FiveElementBureau;
use crate::core::model::calendar::Gender;
use crate::core::model::chart::{
    Chart, DecorativeStarFamily, DecorativeStarPlacement, HoroscopeChart, MutagenActivation,
    PALACE_COUNT, Palace, PalaceGridPosition, PalaceName, StarPlacement, TemporalContext,
    TemporalLayer, VISUAL_BRANCH_ORDER, build_decadal_frame, palace_grid_position,
};
use crate::core::model::star::mutagen::Scope;
use crate::core::model::star::{Brightness, StarCategory, StarKind, StarName, mutagen::Mutagen};
use lunar_lite::{EarthlyBranch, FourPillars, HeavenlyStem, StemBranch};
use serde::{Deserialize, Serialize};

/// Fixed display order for chart scope selectors.
///
/// This ordering (本命/大限/小限/流年/流月/流日/流时) is independent of the [`Scope`] declaration order.
/// It also fixes the order of [`StaticChartViewSnapshot::selectors`] and [`active_scopes`].
///
/// [`active_scopes`]: StaticChartViewSnapshot::active_scopes
const SELECTOR_ORDER: [Scope; 7] = [
    Scope::Natal,
    Scope::Decadal,
    Scope::Age,
    Scope::Yearly,
    Scope::Monthly,
    Scope::Daily,
    Scope::Hourly,
];

const MONTH_LABELS: [&str; PALACE_COUNT] = [
    "正月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "冬月", "腊月",
];

const DAY_LABELS: [[&str; 10]; 3] = [
    [
        "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十",
    ],
    [
        "十一", "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十",
    ],
    [
        "廿一", "廿二", "廿三", "廿四", "廿五", "廿六", "廿七", "廿八", "廿九", "三十",
    ],
];

const HOUR_LABELS: [&str; PALACE_COUNT] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];

/// A renderer-neutral static 12-palace chart view model for one selected slice.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticChartViewSnapshot {
    /// Center panel facts (gender, birth-year pillar, bureau, life/body palaces).
    pub center: StaticChartCenterView,
    /// The twelve perimeter palaces in fixed [`VISUAL_BRANCH_ORDER`].
    pub palaces: Vec<StaticPalaceView>,
    /// Renderer-neutral bottom temporal navigation panel.
    pub temporal_panel: StaticTemporalPanelView,
    /// Scope-selector state in fixed [`SELECTOR_ORDER`].
    pub selectors: Vec<StaticChartSelectorView>,
    /// The scopes currently visible, in fixed [`SELECTOR_ORDER`].
    pub active_scopes: Vec<Scope>,
    /// Reserved highlight annotations. Always empty until feature/rule layers
    /// populate it; this PR performs no 成格 detection.
    pub highlights: Vec<HighlightView>,
}

/// Center panel facts for a static chart.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticChartCenterView {
    /// Retained gender marker.
    pub gender: Gender,
    /// Birth-year Heavenly Stem.
    pub birth_year_stem: HeavenlyStem,
    /// Chinese label for the birth-year Heavenly Stem.
    pub birth_year_stem_zh: String,
    /// Birth-year Earthly Branch.
    pub birth_year_branch: EarthlyBranch,
    /// Chinese label for the birth-year Earthly Branch.
    pub birth_year_branch_zh: String,
    /// Natal four pillars, if available from the chart facade.
    #[serde(default)]
    pub four_pillars: Option<StaticFourPillarsView>,
    /// Five-element bureau, if modeled.
    pub five_element_bureau: Option<FiveElementBureau>,
    /// Life Palace branch, if modeled.
    pub life_palace_branch: Option<EarthlyBranch>,
    /// Chinese label for the Life Palace branch, if modeled.
    pub life_palace_branch_zh: Option<String>,
    /// Body Palace branch, if modeled.
    pub body_palace_branch: Option<EarthlyBranch>,
    /// Chinese label for the Body Palace branch, if modeled.
    pub body_palace_branch_zh: Option<String>,
}

/// Presentation-friendly natal four-pillar facts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticFourPillarsView {
    /// Year pillar (年柱).
    pub yearly: StemBranch,
    /// Chinese label for the year pillar.
    pub yearly_zh: String,
    /// Month pillar (月柱).
    pub monthly: StemBranch,
    /// Chinese label for the month pillar.
    pub monthly_zh: String,
    /// Day pillar (日柱).
    pub daily: StemBranch,
    /// Chinese label for the day pillar.
    pub daily_zh: String,
    /// Hour pillar (时柱).
    pub hourly: StemBranch,
    /// Chinese label for the hour pillar.
    pub hourly_zh: String,
}

/// Renderer-neutral bottom temporal navigation panel.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticTemporalPanelView {
    /// Twelve factual decadal cells in existing core period order.
    pub decadal_cells: Vec<StaticDecadalCellView>,
    /// Twelve factual or neutral flowing-year / nominal-age cells.
    pub yearly_age_cells: Vec<StaticYearlyAgeCellView>,
    /// Conventional Chinese month navigation labels.
    pub month_cells: Vec<StaticNavigationCellView>,
    /// Three rows of ten conventional Chinese lunar-day labels.
    pub day_rows: Vec<Vec<StaticNavigationCellView>>,
    /// Conventional Earthly Branch double-hour labels.
    pub hour_cells: Vec<StaticNavigationCellView>,
}

/// One factual or disabled decadal-period display cell.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticDecadalCellView {
    /// Whether factual display data is available.
    pub enabled: bool,
    /// Inclusive nominal-age range, such as `16-25`.
    pub age_range_zh: Option<String>,
    /// Chinese stem-branch limit label, such as `戊子限`.
    pub limit_label_zh: Option<String>,
}

/// One factual or disabled flowing-year / nominal-age display cell.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticYearlyAgeCellView {
    /// Whether an exact yearly and age fact pair is available.
    pub enabled: bool,
    /// Display year, such as `2024`.
    pub year_label: Option<String>,
    /// Chinese stem-branch plus nominal age, such as `甲辰17`.
    pub stem_branch_age_zh: Option<String>,
}

/// One static navigation label cell.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticNavigationCellView {
    /// Chinese navigation label.
    pub label_zh: String,
    /// Whether the navigation item is available for display.
    pub enabled: bool,
}

impl StaticFourPillarsView {
    fn from_four_pillars(pillars: FourPillars) -> Self {
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
}

impl StaticTemporalPanelView {
    fn from_chart(chart: &Chart) -> Self {
        Self {
            decadal_cells: decadal_cells(chart),
            yearly_age_cells: disabled_yearly_age_cells(),
            month_cells: navigation_cells(&MONTH_LABELS),
            day_rows: DAY_LABELS
                .iter()
                .map(|labels| navigation_cells(labels))
                .collect::<Vec<_>>(),
            hour_cells: navigation_cells(&HOUR_LABELS),
        }
    }

    fn from_horoscope_chart(chart: &HoroscopeChart) -> Self {
        let mut panel = Self::from_chart(chart.natal());
        panel.yearly_age_cells = yearly_age_cells(chart);
        panel
    }
}

fn decadal_cells(chart: &Chart) -> Vec<StaticDecadalCellView> {
    match build_decadal_frame(chart) {
        Ok(frame) => frame
            .periods()
            .iter()
            .map(|period| StaticDecadalCellView {
                enabled: true,
                age_range_zh: Some(format!("{}-{}", period.start_age(), period.end_age())),
                limit_label_zh: Some(format!("{}限", zh_cn::stem_branch_zh(period.stem_branch()))),
            })
            .collect(),
        Err(_) => (0..PALACE_COUNT)
            .map(|_| StaticDecadalCellView {
                enabled: false,
                age_range_zh: None,
                limit_label_zh: None,
            })
            .collect(),
    }
}

fn disabled_yearly_age_cells() -> Vec<StaticYearlyAgeCellView> {
    (0..PALACE_COUNT)
        .map(|_| StaticYearlyAgeCellView {
            enabled: false,
            year_label: None,
            stem_branch_age_zh: None,
        })
        .collect()
}

fn yearly_age_cells(chart: &HoroscopeChart) -> Vec<StaticYearlyAgeCellView> {
    let yearly: Vec<(StemBranch, i32)> = chart
        .layers()
        .iter()
        .filter_map(|layer| match layer.context() {
            TemporalContext::Yearly {
                stem_branch,
                lunar_year,
            } => Some((*stem_branch, *lunar_year)),
            _ => None,
        })
        .collect();
    let ages: Vec<u8> = chart
        .layers()
        .iter()
        .filter_map(|layer| match layer.context() {
            TemporalContext::Age {
                stem_branch: _,
                nominal_age,
            } => Some(*nominal_age),
            _ => None,
        })
        .collect();

    let mut cells = match (yearly.as_slice(), ages.as_slice()) {
        ([(yearly_stem_branch, lunar_year)], [nominal_age]) => {
            vec![StaticYearlyAgeCellView {
                enabled: true,
                year_label: Some(lunar_year.to_string()),
                stem_branch_age_zh: Some(format!(
                    "{}{}",
                    zh_cn::stem_branch_zh(*yearly_stem_branch),
                    nominal_age
                )),
            }]
        }
        _ => Vec::new(),
    };
    cells.extend(
        disabled_yearly_age_cells()
            .into_iter()
            .take(PALACE_COUNT - cells.len()),
    );
    cells
}

fn navigation_cells(labels: &[&str]) -> Vec<StaticNavigationCellView> {
    labels
        .iter()
        .map(|label| StaticNavigationCellView {
            label_zh: (*label).to_owned(),
            enabled: true,
        })
        .collect()
}

/// One perimeter palace cell of a static chart.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticPalaceView {
    /// Palace branch (the stable spatial reference).
    pub branch: EarthlyBranch,
    /// Chinese label for the palace branch.
    pub branch_zh: String,
    /// Fixed 4x4 perimeter-grid position.
    pub grid_position: PalaceGridPosition,
    /// Natal palace name.
    pub name: PalaceName,
    /// Chinese label for the natal palace name.
    pub name_zh: String,
    /// Palace Heavenly Stem.
    pub stem: HeavenlyStem,
    /// Chinese label for the palace stem.
    pub stem_zh: String,
    /// Role markers (natal palace name, body palace).
    pub roles: Vec<StaticPalaceRole>,
    /// Major stars (主星) in this palace.
    pub major_stars: Vec<StaticTypedStarView>,
    /// Minor stars (辅星) in this palace.
    pub minor_stars: Vec<StaticTypedStarView>,
    /// Adjective / miscellaneous stars (杂曜) in this palace.
    pub adjective_stars: Vec<StaticTypedStarView>,
    /// Typed stars whose [`StarCategory`] falls outside major/minor/adjective.
    ///
    /// Reserved for forward compatibility: [`StarCategory`] is currently
    /// exhaustive over [`Major`](StarCategory::Major), [`Minor`](StarCategory::Minor),
    /// and [`Adjective`](StarCategory::Adjective), so this is always empty today.
    pub other_typed_stars: Vec<StaticTypedStarView>,
    /// Decorative "twelve gods" stars (十二神) in this palace.
    pub decorative_stars: Vec<StaticDecorativeStarView>,
    /// Selected temporal overlays for this palace, kept separate from natal facts.
    pub overlays: Vec<StaticTemporalOverlayView>,
    /// Reserved per-palace highlight annotations. Always empty for now.
    pub highlights: Vec<HighlightView>,
}

/// Role marker attached to a static palace cell.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "palace_name", rename_all = "snake_case")]
pub enum StaticPalaceRole {
    /// The cell contains this natal palace (the Life Palace is `NatalPalace(Life)`).
    NatalPalace(PalaceName),
    /// The cell is the Body Palace branch.
    BodyPalace,
}

/// A typed star for display, with grouping category and finer kind.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticTypedStarView {
    /// Star name.
    pub name: StarName,
    /// Chinese label for the star name.
    pub name_zh: String,
    /// Coarse grouping category (主星/辅星/杂曜).
    pub category: StarCategory,
    /// Chinese label for the coarse category.
    pub category_zh: String,
    /// Fine star kind.
    pub kind: StarKind,
    /// Chinese label for the fine star kind.
    pub kind_zh: String,
    /// Brightness state.
    pub brightness: Brightness,
    /// Chinese label for the brightness state.
    pub brightness_zh: String,
    /// Attached mutagen, if any.
    pub mutagen: Option<Mutagen>,
    /// Chinese label for the attached mutagen, if any.
    pub mutagen_zh: Option<String>,
}

impl StaticTypedStarView {
    fn from_star_placement(placement: &StarPlacement) -> Self {
        Self {
            name: placement.name(),
            name_zh: zh_cn::star_name_zh(placement.name()).to_owned(),
            category: placement.category(),
            category_zh: zh_cn::star_category_zh(placement.category()).to_owned(),
            kind: placement.kind(),
            kind_zh: zh_cn::star_kind_zh(placement.kind()).to_owned(),
            brightness: placement.brightness(),
            brightness_zh: zh_cn::brightness_zh(placement.brightness()).to_owned(),
            mutagen: placement.mutagen(),
            mutagen_zh: placement
                .mutagen()
                .map(|mutagen| zh_cn::mutagen_zh(mutagen).to_owned()),
        }
    }

    /// Deterministic facade ordering key: `(kind, name, brightness, mutagen)`.
    fn order_key(&self) -> (StarKind, StarName, Brightness, Option<Mutagen>) {
        (self.kind, self.name, self.brightness, self.mutagen)
    }
}

/// A decorative "twelve gods" star for display.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticDecorativeStarView {
    /// Decorative star name.
    pub name: StarName,
    /// Chinese label for the decorative star name.
    pub name_zh: String,
    /// Decorative star family.
    pub family: DecorativeStarFamily,
    /// Chinese label for the decorative star family.
    pub family_zh: String,
}

impl StaticDecorativeStarView {
    fn from_decorative_star_placement(placement: &DecorativeStarPlacement) -> Self {
        Self {
            name: placement.name(),
            name_zh: zh_cn::star_name_zh(placement.name()).to_owned(),
            family: placement.family(),
            family_zh: zh_cn::decorative_star_family_zh(placement.family()).to_owned(),
        }
    }

    /// Deterministic facade ordering key: `(family, name)`.
    fn order_key(&self) -> (DecorativeStarFamily, StarName) {
        (self.family, self.name)
    }
}

/// Selector state for one horoscope scope (renderer draws the actual control).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticChartSelectorView {
    /// The scope this selector represents.
    pub scope: Scope,
    /// Chinese label (本命/大限/小限/流年/流月/流日/流时).
    pub label_zh: String,
    /// Whether the scope is available in the underlying chart.
    pub enabled: bool,
    /// Whether the scope is currently selected/visible.
    pub selected: bool,
}

/// A temporal overlay on a palace cell, kept separate from natal facts.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticTemporalOverlayView {
    /// The non-natal scope this overlay belongs to.
    pub scope: Scope,
    /// The temporal palace name this period assigns to the branch, if any.
    pub temporal_palace_name: Option<PalaceName>,
    /// Chinese label for the temporal palace name, if any.
    pub temporal_palace_name_zh: Option<String>,
    /// Typed flow stars this period places on the branch.
    pub typed_stars: Vec<StaticTypedStarView>,
    /// Untyped decorative stars this period adds to the branch.
    pub decorative_stars: Vec<StaticDecorativeStarView>,
    /// Mutagen activations this period lands on stars in the branch.
    pub mutagens: Vec<StaticOverlayMutagenView>,
}

/// A mutagen activation landing on a star within an overlay's palace.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticOverlayMutagenView {
    /// The star the mutagen lands on.
    pub star: StarName,
    /// Chinese label for the target star.
    pub star_zh: String,
    /// The mutagen applied.
    pub mutagen: Mutagen,
    /// Chinese label for the mutagen.
    pub mutagen_zh: String,
}

impl StaticOverlayMutagenView {
    fn from_activation(activation: &MutagenActivation) -> Self {
        Self {
            star: activation.target_star(),
            star_zh: zh_cn::star_name_zh(activation.target_star()).to_owned(),
            mutagen: activation.mutagen(),
            mutagen_zh: zh_cn::mutagen_zh(activation.mutagen()).to_owned(),
        }
    }
}

/// A reserved, renderer-neutral highlight annotation.
///
/// This shape is structurally reserved for a future feature/rule layer. This PR
/// performs no 成格 detection, so generated snapshots always carry an empty
/// highlight list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HighlightView {
    /// Stable identifier for the highlight.
    pub id: String,
    /// Optional Chinese label.
    pub label_zh: Option<String>,
    /// Optional scope the highlight is tied to.
    pub scope: Option<Scope>,
    /// Palaces involved, by branch.
    pub involved_palaces: Vec<EarthlyBranch>,
    /// Stars involved.
    pub involved_stars: Vec<StarName>,
    /// Mutagens involved.
    pub involved_mutagens: Vec<Mutagen>,
}

/// A request describing which temporal overlays a static chart view should show.
///
/// [`Scope::Natal`] is always the base layer of a static chart, so it is always
/// selected/active regardless of this request. `visible_scopes` only controls
/// the requested temporal overlays on top of natal.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticChartViewRequest {
    /// Requested temporal overlays. Natal is always included as the base layer
    /// even if omitted here. Scopes absent from the chart are ignored (their
    /// selector is emitted disabled).
    pub visible_scopes: Vec<Scope>,
}

impl StaticChartViewSnapshot {
    /// Builds a natal-only static chart view from a natal chart.
    ///
    /// Only [`Scope::Natal`] is enabled and selected; every temporal selector is
    /// disabled. No overlays are produced.
    pub fn from_chart(chart: &Chart) -> Self {
        let palaces = build_palaces(chart, &[]);
        let present = [Scope::Natal];
        let selected = [Scope::Natal];
        Self {
            center: StaticChartCenterView::from_chart(chart),
            palaces,
            temporal_panel: StaticTemporalPanelView::from_chart(chart),
            selectors: build_selectors(&present, &selected),
            active_scopes: active_scopes(&selected),
            highlights: Vec::new(),
        }
    }

    /// Builds a static chart view from a horoscope chart, with 本命 plus every
    /// available temporal layer selected.
    pub fn from_horoscope_chart(chart: &HoroscopeChart) -> Self {
        let mut visible_scopes = vec![Scope::Natal];
        visible_scopes.extend(
            present_scopes(chart)
                .into_iter()
                .filter(|s| *s != Scope::Natal),
        );
        Self::from_horoscope_chart_with(chart, &StaticChartViewRequest { visible_scopes })
    }

    /// Builds a static chart view from a horoscope chart, selecting exactly the
    /// scopes in `request` that are present in the chart.
    ///
    /// Natal facts are identical regardless of selection: selecting scopes only
    /// changes which temporal overlays are attached to each palace.
    pub fn from_horoscope_chart_with(
        chart: &HoroscopeChart,
        request: &StaticChartViewRequest,
    ) -> Self {
        let natal = chart.natal();
        let present = present_scopes(chart);
        // Natal is always the base layer of a static chart, so it is always
        // selected/active; `visible_scopes` only controls temporal overlays.
        let selected: Vec<Scope> = SELECTOR_ORDER
            .into_iter()
            .filter(|scope| {
                *scope == Scope::Natal
                    || (present.contains(scope) && request.visible_scopes.contains(scope))
            })
            .collect();

        // Only selected non-natal layers contribute overlays.
        let overlay_layers: Vec<&TemporalLayer> = SELECTOR_ORDER
            .into_iter()
            .filter(|scope| *scope != Scope::Natal && selected.contains(scope))
            .flat_map(|scope| {
                chart
                    .layers()
                    .iter()
                    .filter(move |layer| layer.scope() == scope)
            })
            .collect();

        Self {
            center: StaticChartCenterView::from_chart(natal),
            palaces: build_palaces(natal, &overlay_layers),
            temporal_panel: StaticTemporalPanelView::from_horoscope_chart(chart),
            selectors: build_selectors(&present, &selected),
            active_scopes: active_scopes(&selected),
            highlights: Vec::new(),
        }
    }
}

impl StaticChartCenterView {
    fn from_chart(chart: &Chart) -> Self {
        let life_palace_branch = chart.life_palace().map(Palace::branch);
        let body_palace_branch = chart.body_palace_branch();
        Self {
            gender: chart.birth_context().gender(),
            birth_year_stem: chart.birth_year().stem(),
            birth_year_stem_zh: zh_cn::heavenly_stem_zh(chart.birth_year().stem()).to_owned(),
            birth_year_branch: chart.birth_year().branch(),
            birth_year_branch_zh: zh_cn::earthly_branch_zh(chart.birth_year().branch()).to_owned(),
            four_pillars: chart
                .four_pillars()
                .copied()
                .map(StaticFourPillarsView::from_four_pillars),
            five_element_bureau: chart.five_element_bureau(),
            life_palace_branch,
            life_palace_branch_zh: life_palace_branch
                .map(|branch| zh_cn::earthly_branch_zh(branch).to_owned()),
            body_palace_branch,
            body_palace_branch_zh: body_palace_branch
                .map(|branch| zh_cn::earthly_branch_zh(branch).to_owned()),
        }
    }
}

/// Builds the twelve palace cells in fixed [`VISUAL_BRANCH_ORDER`], attaching any
/// overlays from `overlay_layers`.
fn build_palaces(chart: &Chart, overlay_layers: &[&TemporalLayer]) -> Vec<StaticPalaceView> {
    VISUAL_BRANCH_ORDER
        .into_iter()
        .filter_map(|branch| {
            chart
                .palaces()
                .iter()
                .find(|palace| palace.branch() == branch)
                .map(|palace| StaticPalaceView::from_palace(chart, palace, overlay_layers))
        })
        .collect()
}

impl StaticPalaceView {
    fn from_palace(chart: &Chart, palace: &Palace, overlay_layers: &[&TemporalLayer]) -> Self {
        let mut roles = vec![StaticPalaceRole::NatalPalace(palace.name())];
        if chart.is_body_palace_branch(palace.branch()) {
            roles.push(StaticPalaceRole::BodyPalace);
        }

        let mut major_stars = Vec::new();
        let mut minor_stars = Vec::new();
        let mut adjective_stars = Vec::new();
        let other_typed_stars = Vec::new();
        for placement in palace.stars() {
            let star = StaticTypedStarView::from_star_placement(placement);
            match star.category {
                StarCategory::Major => major_stars.push(star),
                StarCategory::Minor => minor_stars.push(star),
                StarCategory::Adjective => adjective_stars.push(star),
            }
        }
        major_stars.sort_by_key(StaticTypedStarView::order_key);
        minor_stars.sort_by_key(StaticTypedStarView::order_key);
        adjective_stars.sort_by_key(StaticTypedStarView::order_key);

        let mut decorative_stars: Vec<StaticDecorativeStarView> = palace
            .decorative_stars()
            .iter()
            .map(StaticDecorativeStarView::from_decorative_star_placement)
            .collect();
        decorative_stars.sort_by_key(StaticDecorativeStarView::order_key);

        let overlays = overlay_layers
            .iter()
            .map(|layer| StaticTemporalOverlayView::from_layer(layer, palace.branch()))
            .collect();

        Self {
            branch: palace.branch(),
            branch_zh: zh_cn::earthly_branch_zh(palace.branch()).to_owned(),
            grid_position: palace_grid_position(palace.branch()),
            name: palace.name(),
            name_zh: zh_cn::palace_name_zh(palace.name()).to_owned(),
            stem: palace.stem(),
            stem_zh: zh_cn::heavenly_stem_zh(palace.stem()).to_owned(),
            roles,
            major_stars,
            minor_stars,
            adjective_stars,
            other_typed_stars,
            decorative_stars,
            overlays,
            highlights: Vec::new(),
        }
    }
}

impl StaticTemporalOverlayView {
    fn from_layer(layer: &TemporalLayer, branch: EarthlyBranch) -> Self {
        let temporal_palace_name = layer
            .palace_layout()
            .and_then(|layout| layout.name_for_branch(branch));

        let mut typed_stars: Vec<StaticTypedStarView> = layer
            .placements()
            .iter()
            .filter(|placement| placement.branch() == branch)
            .map(|placement| StaticTypedStarView::from_star_placement(placement.placement()))
            .collect();
        typed_stars.sort_by_key(StaticTypedStarView::order_key);

        let mut decorative_stars: Vec<StaticDecorativeStarView> = layer
            .temporal_decorative_stars()
            .iter()
            .filter(|placement| placement.branch() == branch)
            .map(|placement| {
                StaticDecorativeStarView::from_decorative_star_placement(placement.placement())
            })
            .collect();
        decorative_stars.sort_by_key(StaticDecorativeStarView::order_key);

        let mutagens = layer
            .activations()
            .iter()
            .filter(|activation| activation.target_branch() == branch)
            .map(StaticOverlayMutagenView::from_activation)
            .collect();

        Self {
            scope: layer.scope(),
            temporal_palace_name,
            temporal_palace_name_zh: temporal_palace_name
                .map(|name| zh_cn::palace_name_zh(name).to_owned()),
            typed_stars,
            decorative_stars,
            mutagens,
        }
    }
}

/// Returns the scopes present in a horoscope chart, including [`Scope::Natal`],
/// in fixed [`SELECTOR_ORDER`].
fn present_scopes(chart: &HoroscopeChart) -> Vec<Scope> {
    SELECTOR_ORDER
        .into_iter()
        .filter(|scope| {
            *scope == Scope::Natal || chart.layers().iter().any(|layer| layer.scope() == *scope)
        })
        .collect()
}

/// Builds selector state for every scope in fixed [`SELECTOR_ORDER`].
fn build_selectors(present: &[Scope], selected: &[Scope]) -> Vec<StaticChartSelectorView> {
    SELECTOR_ORDER
        .into_iter()
        .map(|scope| StaticChartSelectorView {
            scope,
            label_zh: zh_cn::scope_zh(scope).to_owned(),
            enabled: present.contains(&scope),
            selected: selected.contains(&scope),
        })
        .collect()
}

/// Returns the selected scopes in fixed [`SELECTOR_ORDER`].
fn active_scopes(selected: &[Scope]) -> Vec<Scope> {
    SELECTOR_ORDER
        .into_iter()
        .filter(|scope| selected.contains(scope))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::model::chart::PALACE_COUNT;
    use crate::core::{
        ChartAlgorithmKind, LunarChartRequest, LunarDay, LunarMonth, MethodProfile,
        SolarChartRequest, SolarDay, SolarMonth, StemBranch, by_lunar, by_solar,
    };
    use std::collections::HashSet;

    /// Builds the canonical natal chart (lunar 1990-05-17, Chen hour, female).
    fn sample_chart() -> Chart {
        let birth_year = StemBranch::from_lunar_year(1990);
        let method_profile = MethodProfile::new(
            "1990_05_17_chen_female",
            ChartAlgorithmKind::QuanShu,
            "static chart view unit test",
        );
        let request = LunarChartRequest::builder()
            .lunar_year(1990)
            .lunar_month(LunarMonth::new(5).expect("valid lunar month"))
            .lunar_day(LunarDay::new(17).expect("valid lunar day"))
            .iztro_time_index(4)
            .expect("valid time index")
            .gender(Gender::Female)
            .birth_year_stem(birth_year.stem())
            .birth_year_branch(birth_year.branch())
            .is_leap_month(false)
            .fix_leap(true)
            .method_profile(method_profile)
            .build()
            .expect("lunar chart request should build");
        by_lunar(request).expect("by_lunar should build the canonical chart")
    }

    fn sample_solar_chart() -> Chart {
        let method_profile = MethodProfile::new(
            "1990_05_17_chen_female_solar",
            ChartAlgorithmKind::QuanShu,
            "static chart view solar unit test",
        );
        let request = SolarChartRequest::builder()
            .solar_year(1990)
            .solar_month(SolarMonth::new(5).expect("valid solar month"))
            .solar_day(SolarDay::new(17).expect("valid solar day"))
            .iztro_time_index(4)
            .expect("valid time index")
            .gender(Gender::Female)
            .method_profile(method_profile)
            .build()
            .expect("solar chart request should build");

        by_solar(request).expect("by_solar should build the canonical chart")
    }

    #[test]
    fn snapshot_has_exactly_twelve_palaces() {
        let snapshot = StaticChartViewSnapshot::from_chart(&sample_chart());
        assert_eq!(snapshot.palaces.len(), PALACE_COUNT);
    }

    #[test]
    fn every_branch_appears_exactly_once() {
        let snapshot = StaticChartViewSnapshot::from_chart(&sample_chart());
        let branches: HashSet<EarthlyBranch> = snapshot
            .palaces
            .iter()
            .map(|palace| palace.branch)
            .collect();
        assert_eq!(branches.len(), PALACE_COUNT);
        assert_eq!(branches, VISUAL_BRANCH_ORDER.into_iter().collect());
    }

    #[test]
    fn every_grid_position_appears_exactly_once_and_matches_layout() {
        let snapshot = StaticChartViewSnapshot::from_chart(&sample_chart());
        let positions: HashSet<(u8, u8)> = snapshot
            .palaces
            .iter()
            .map(|palace| (palace.grid_position.row(), palace.grid_position.column()))
            .collect();
        assert_eq!(positions.len(), PALACE_COUNT);
        // Grid position is the canonical 4x4 perimeter mapping.
        for palace in &snapshot.palaces {
            assert_eq!(palace.grid_position, palace_grid_position(palace.branch));
        }
    }

    #[test]
    fn every_palace_has_chinese_labels() {
        let snapshot = StaticChartViewSnapshot::from_chart(&sample_chart());
        for palace in &snapshot.palaces {
            assert!(!palace.branch_zh.is_empty());
            assert!(!palace.name_zh.is_empty());
            assert!(!palace.stem_zh.is_empty());
        }
    }

    #[test]
    fn solar_chart_center_includes_optional_four_pillar_labels() {
        let chart = sample_solar_chart();
        let snapshot = StaticChartViewSnapshot::from_chart(&chart);
        let center = &snapshot.center;
        let pillars = center
            .four_pillars
            .as_ref()
            .expect("by_solar chart should carry four pillars");

        assert_eq!(
            Some(&pillars.yearly),
            chart.four_pillars().map(|p| &p.yearly)
        );
        assert_eq!(pillars.yearly, chart.birth_year());
        assert!(!pillars.yearly_zh.is_empty());
        assert!(!pillars.monthly_zh.is_empty());
        assert!(!pillars.daily_zh.is_empty());
        assert!(!pillars.hourly_zh.is_empty());
    }

    #[test]
    fn every_typed_star_has_chinese_labels_and_mutagen_labels_match() {
        let snapshot = StaticChartViewSnapshot::from_chart(&sample_chart());
        for palace in &snapshot.palaces {
            let typed = palace
                .major_stars
                .iter()
                .chain(&palace.minor_stars)
                .chain(&palace.adjective_stars)
                .chain(&palace.other_typed_stars);
            for star in typed {
                assert!(!star.name_zh.is_empty());
                assert!(!star.kind_zh.is_empty());
                assert!(!star.category_zh.is_empty());
                // Brightness::Unknown maps to "" by design; otherwise non-empty.
                assert_eq!(star.mutagen.is_some(), star.mutagen_zh.is_some());
            }
        }
    }

    #[test]
    fn every_decorative_star_has_chinese_labels() {
        let snapshot = StaticChartViewSnapshot::from_chart(&sample_chart());
        for palace in &snapshot.palaces {
            for star in &palace.decorative_stars {
                assert!(!star.name_zh.is_empty());
                assert!(!star.family_zh.is_empty());
            }
        }
    }

    #[test]
    fn typed_stars_are_grouped_by_category_without_loss() {
        let chart = sample_chart();
        let snapshot = StaticChartViewSnapshot::from_chart(&chart);
        for palace in &snapshot.palaces {
            for star in &palace.major_stars {
                assert_eq!(star.category, StarCategory::Major);
            }
            for star in &palace.minor_stars {
                assert_eq!(star.category, StarCategory::Minor);
            }
            for star in &palace.adjective_stars {
                assert_eq!(star.category, StarCategory::Adjective);
            }
            // No typed star is lost across the grouped arrays.
            let source = chart
                .palaces()
                .iter()
                .find(|p| p.branch() == palace.branch)
                .expect("palace by branch");
            let grouped = palace.major_stars.len()
                + palace.minor_stars.len()
                + palace.adjective_stars.len()
                + palace.other_typed_stars.len();
            assert_eq!(grouped, source.stars().len());
        }
        // StarCategory is exhaustive over major/minor/adjective today.
        assert!(
            snapshot
                .palaces
                .iter()
                .all(|p| p.other_typed_stars.is_empty())
        );
    }

    #[test]
    fn highlights_are_empty() {
        let snapshot = StaticChartViewSnapshot::from_chart(&sample_chart());
        assert!(snapshot.highlights.is_empty());
        assert!(snapshot.palaces.iter().all(|p| p.highlights.is_empty()));
    }

    #[test]
    fn repeated_construction_serializes_identically() {
        let chart = sample_chart();
        let first = serde_json::to_string(&StaticChartViewSnapshot::from_chart(&chart))
            .expect("snapshot should serialize");
        let second = serde_json::to_string(&StaticChartViewSnapshot::from_chart(&chart))
            .expect("snapshot should serialize");
        assert_eq!(first, second);
    }

    #[test]
    fn constructing_the_view_does_not_mutate_natal_facts() {
        let chart = sample_chart();
        let before = chart.clone();
        let _ = StaticChartViewSnapshot::from_chart(&chart);
        assert_eq!(chart, before);
    }

    #[test]
    fn from_chart_enables_and_selects_only_natal() {
        let snapshot = StaticChartViewSnapshot::from_chart(&sample_chart());
        assert_eq!(snapshot.active_scopes, vec![Scope::Natal]);
        for selector in &snapshot.selectors {
            let is_natal = selector.scope == Scope::Natal;
            assert_eq!(selector.enabled, is_natal);
            assert_eq!(selector.selected, is_natal);
        }
        // Selectors follow the fixed 文墨天机 display order.
        let order: Vec<Scope> = snapshot.selectors.iter().map(|s| s.scope).collect();
        assert_eq!(order, SELECTOR_ORDER.to_vec());
        // Labels are correct.
        let natal = &snapshot.selectors[0];
        assert_eq!(natal.label_zh, "本命");
    }

    #[test]
    fn from_chart_produces_no_overlays() {
        let snapshot = StaticChartViewSnapshot::from_chart(&sample_chart());
        assert!(snapshot.palaces.iter().all(|p| p.overlays.is_empty()));
    }

    use crate::core::model::chart::{ScopedStarPlacement, TemporalContext, TemporalLayer};

    /// Branch carrying the synthetic decadal overlay facts in the test horoscope.
    const OVERLAY_BRANCH: EarthlyBranch = EarthlyBranch::Zi;

    /// Builds a horoscope chart with a decadal layer (carrying one flow star and
    /// one mutagen activation on [`OVERLAY_BRANCH`]) and an empty yearly layer.
    fn sample_horoscope_chart() -> HoroscopeChart {
        let mut chart = HoroscopeChart::new(sample_chart());

        let period = StemBranch::from_lunar_year(2020);
        let decadal_star = StarPlacement::new(
            StarName::YunLu,
            StarKind::LuCun,
            Brightness::Unknown,
            None,
            Scope::Decadal,
        );
        let decadal_layer = TemporalLayer::try_new(
            Scope::Decadal,
            TemporalContext::Decadal {
                stem_branch: period,
                start_age: 6,
            },
            vec![ScopedStarPlacement::new(OVERLAY_BRANCH, decadal_star)],
            vec![MutagenActivation::new(
                Scope::Decadal,
                StarName::ZiWei,
                OVERLAY_BRANCH,
                Mutagen::Lu,
            )],
        )
        .expect("decadal layer should build");
        chart.push_layer(decadal_layer);

        let yearly_layer = TemporalLayer::try_new(
            Scope::Yearly,
            TemporalContext::Yearly {
                stem_branch: period,
                lunar_year: 2020,
            },
            Vec::new(),
            Vec::new(),
        )
        .expect("yearly layer should build");
        chart.push_layer(yearly_layer);

        chart
    }

    #[test]
    fn from_horoscope_chart_enables_and_selects_present_scopes() {
        let chart = sample_horoscope_chart();
        let snapshot = StaticChartViewSnapshot::from_horoscope_chart(&chart);

        let present = [Scope::Natal, Scope::Decadal, Scope::Yearly];
        for selector in &snapshot.selectors {
            let is_present = present.contains(&selector.scope);
            assert_eq!(selector.enabled, is_present, "{:?}", selector.scope);
            assert_eq!(selector.selected, is_present, "{:?}", selector.scope);
        }
        // Active scopes follow the fixed display order: 本命 / 大限 / 流年.
        assert_eq!(
            snapshot.active_scopes,
            vec![Scope::Natal, Scope::Decadal, Scope::Yearly]
        );
    }

    #[test]
    fn selector_labels_are_correct() {
        let snapshot = StaticChartViewSnapshot::from_horoscope_chart(&sample_horoscope_chart());
        let labels: Vec<(Scope, &str)> = snapshot
            .selectors
            .iter()
            .map(|s| (s.scope, s.label_zh.as_str()))
            .collect();
        assert_eq!(
            labels,
            vec![
                (Scope::Natal, "本命"),
                (Scope::Decadal, "大限"),
                (Scope::Age, "小限"),
                (Scope::Yearly, "流年"),
                (Scope::Monthly, "流月"),
                (Scope::Daily, "流日"),
                (Scope::Hourly, "流时"),
            ]
        );
    }

    #[test]
    fn absent_scopes_are_disabled_deterministically() {
        let snapshot = StaticChartViewSnapshot::from_horoscope_chart(&sample_horoscope_chart());
        for scope in [Scope::Age, Scope::Monthly, Scope::Daily, Scope::Hourly] {
            let selector = snapshot
                .selectors
                .iter()
                .find(|s| s.scope == scope)
                .expect("selector present");
            assert!(!selector.enabled);
            assert!(!selector.selected);
        }
    }

    #[test]
    fn active_scopes_match_requested_visible_scopes() {
        let chart = sample_horoscope_chart();
        // Request only Natal + Decadal, plus an absent Monthly (must be ignored).
        let request = StaticChartViewRequest {
            visible_scopes: vec![Scope::Natal, Scope::Decadal, Scope::Monthly],
        };
        let snapshot = StaticChartViewSnapshot::from_horoscope_chart_with(&chart, &request);
        assert_eq!(snapshot.active_scopes, vec![Scope::Natal, Scope::Decadal]);

        // Yearly is present but not requested: enabled yet not selected.
        let yearly = snapshot
            .selectors
            .iter()
            .find(|s| s.scope == Scope::Yearly)
            .unwrap();
        assert!(yearly.enabled);
        assert!(!yearly.selected);
    }

    #[test]
    fn natal_is_always_active_for_horoscope_static_views() {
        let chart = sample_horoscope_chart();
        let request = StaticChartViewRequest {
            visible_scopes: vec![Scope::Yearly],
        };

        let snapshot = StaticChartViewSnapshot::from_horoscope_chart_with(&chart, &request);

        assert_eq!(snapshot.active_scopes, vec![Scope::Natal, Scope::Yearly]);

        let natal = snapshot
            .selectors
            .iter()
            .find(|selector| selector.scope == Scope::Natal)
            .expect("natal selector");

        assert!(natal.enabled);
        assert!(natal.selected);
    }

    #[test]
    fn selected_overlays_appear_only_on_the_overlay_branch() {
        let snapshot = StaticChartViewSnapshot::from_horoscope_chart(&sample_horoscope_chart());
        for palace in &snapshot.palaces {
            // One overlay per selected non-natal layer (decadal + yearly).
            assert_eq!(palace.overlays.len(), 2, "{:?}", palace.branch);
            let decadal = palace
                .overlays
                .iter()
                .find(|o| o.scope == Scope::Decadal)
                .expect("decadal overlay");
            if palace.branch == OVERLAY_BRANCH {
                assert_eq!(decadal.typed_stars.len(), 1);
                assert_eq!(decadal.typed_stars[0].name, StarName::YunLu);
                assert_eq!(decadal.mutagens.len(), 1);
                assert_eq!(decadal.mutagens[0].mutagen, Mutagen::Lu);
            } else {
                assert!(decadal.typed_stars.is_empty());
                assert!(decadal.mutagens.is_empty());
            }
        }
    }

    #[test]
    fn selecting_scopes_changes_only_overlays_not_natal_facts() {
        let chart = sample_horoscope_chart();
        let natal_only = StaticChartViewSnapshot::from_chart(chart.natal());
        let with_overlays = StaticChartViewSnapshot::from_horoscope_chart(&chart);

        // Center and per-palace natal facts are identical; only overlays differ.
        assert_eq!(natal_only.center, with_overlays.center);
        for (a, b) in natal_only.palaces.iter().zip(&with_overlays.palaces) {
            assert_eq!(a.branch, b.branch);
            assert_eq!(a.major_stars, b.major_stars);
            assert_eq!(a.minor_stars, b.minor_stars);
            assert_eq!(a.adjective_stars, b.adjective_stars);
            assert_eq!(a.decorative_stars, b.decorative_stars);
            assert_eq!(a.roles, b.roles);
            assert!(a.overlays.is_empty());
            assert!(!b.overlays.is_empty());
        }
    }

    #[test]
    fn horoscope_snapshot_serializes_deterministically_and_keeps_natal_immutable() {
        let chart = sample_horoscope_chart();
        let before = chart.clone();
        let first = serde_json::to_string(&StaticChartViewSnapshot::from_horoscope_chart(&chart))
            .expect("serialize");
        let second = serde_json::to_string(&StaticChartViewSnapshot::from_horoscope_chart(&chart))
            .expect("serialize");
        assert_eq!(first, second);
        assert_eq!(chart, before);
    }
}
