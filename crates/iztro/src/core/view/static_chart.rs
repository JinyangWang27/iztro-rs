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

use crate::core::calendar::lunar_month_has_thirtieth;
use crate::core::labels::{chinese_date, zh_cn};
use crate::core::model::bureau::FiveElementBureau;
use crate::core::model::calendar::{CalendarKind, Gender};
use crate::core::model::chart::{
    Chart, DecadalFrame, DecadalPeriod, DecorativeStarFamily, DecorativeStarPlacement,
    HoroscopeChart, MutagenActivation, PALACE_COUNT, Palace, PalaceGridPosition, PalaceName,
    StarPlacement, TemporalLayer, TemporalPalaceName, VISUAL_BRANCH_ORDER, build_age_period,
    build_decadal_frame, palace_grid_position,
};
use crate::core::model::master::{body_master, soul_master};
use crate::core::model::star::mutagen::Scope;
use crate::core::model::star::{Brightness, StarCategory, StarKind, StarName, mutagen::Mutagen};
use crate::core::model::zodiac::{WesternZodiac, western_zodiac};
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

/// A language-neutral lunisolar (农历) date, for presentation-layer formatting.
///
/// This carries the typed lunar date parts so a renderer/i18n layer can format
/// them in any locale. The conventional Chinese string form is still available
/// via [`chinese_date::lunar_date_label`](crate::core::labels::chinese_date::lunar_date_label).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LunarDateView {
    /// Lunar year.
    pub year: i32,
    /// One-based lunar month (`1..=12`).
    pub month: u8,
    /// One-based lunar day (`1..=30`).
    pub day: u8,
    /// Whether the month is a leap month (闰月).
    pub is_leap_month: bool,
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
    /// Chinese label for the five-element bureau (五行局), such as `木三局`.
    #[serde(default)]
    pub five_element_bureau_zh: Option<String>,
    /// Birth solar (阳历) date label, such as `1993-05-27`.
    #[serde(default)]
    pub birth_solar_label: String,
    /// Birth lunar (农历) date label, such as `一九九三年四月初七`.
    #[serde(default)]
    pub birth_lunar_label: String,
    /// Birth double-hour (时辰) label with range, such as `酉时(17:00~19:00)`.
    #[serde(default)]
    pub birth_time_label: String,
    /// Chinese zodiac animal (生肖) label, such as `鸡`.
    #[serde(default)]
    pub zodiac_zh: String,
    /// Western constellation (星座) label, such as `双子座`.
    #[serde(default)]
    pub constellation_zh: String,
    /// Soul master (命主) star label, such as `廉贞`.
    #[serde(default)]
    pub soul_master_zh: Option<String>,
    /// Body master (身主) star label, such as `天同`.
    #[serde(default)]
    pub body_master_zh: Option<String>,
    /// Selected period nominal age (年龄(虚岁)) label, such as `16 岁`.
    ///
    /// Filled by the temporal facade because it depends on navigation state.
    #[serde(default)]
    pub nominal_age_label: Option<String>,
    /// Selected period lunar (运限农历) date label.
    #[serde(default)]
    pub temporal_lunar_label: Option<String>,
    /// Selected period solar (运限阳历) date label.
    #[serde(default)]
    pub temporal_solar_label: Option<String>,
    /// Life Palace branch, if modeled.
    pub life_palace_branch: Option<EarthlyBranch>,
    /// Chinese label for the Life Palace branch, if modeled.
    pub life_palace_branch_zh: Option<String>,
    /// Body Palace branch, if modeled.
    pub body_palace_branch: Option<EarthlyBranch>,
    /// Chinese label for the Body Palace branch, if modeled.
    pub body_palace_branch_zh: Option<String>,
    /// Typed lunar (农历) birth date, for locale-neutral formatting.
    ///
    /// Mirrors [`birth_lunar_label`](Self::birth_lunar_label); `None` when the
    /// chart carries no retained lunar date.
    #[serde(default)]
    pub birth_lunar_date: Option<LunarDateView>,
    /// Upstream `iztro` double-hour `timeIndex` (`0..=12`) for the birth time.
    ///
    /// Mirrors [`birth_time_label`](Self::birth_time_label) as typed data so the
    /// presentation layer can render the 时辰 in any locale.
    #[serde(default)]
    pub birth_time_index: Option<u8>,
    /// Typed Western zodiac sign (星座), for locale-neutral formatting.
    ///
    /// Mirrors [`constellation_zh`](Self::constellation_zh).
    #[serde(default)]
    pub western_zodiac: Option<WesternZodiac>,
    /// Soul master (命主) star, typed. Mirrors [`soul_master_zh`](Self::soul_master_zh).
    #[serde(default)]
    pub soul_master: Option<StarName>,
    /// Body master (身主) star, typed. Mirrors [`body_master_zh`](Self::body_master_zh).
    #[serde(default)]
    pub body_master: Option<StarName>,
    /// Selected period nominal age (虚岁) as a number.
    ///
    /// Mirrors [`nominal_age_label`](Self::nominal_age_label); filled by the
    /// temporal facade because it depends on navigation state.
    #[serde(default)]
    pub nominal_age: Option<u16>,
    /// Selected period lunar (运限农历) date, typed, when a concrete day is known.
    ///
    /// Mirrors [`temporal_lunar_label`](Self::temporal_lunar_label) for 流月/流日/流时
    /// selections. A 流年-only selection resolves to a lunar year only, exposed via
    /// [`temporal_lunar_year`](Self::temporal_lunar_year) instead.
    #[serde(default)]
    pub temporal_lunar_date: Option<LunarDateView>,
    /// Selected period lunar year for a 流年-only selection, when no concrete day
    /// is known. Mirrors the year portion of [`temporal_lunar_label`](Self::temporal_lunar_label).
    #[serde(default)]
    pub temporal_lunar_year: Option<i32>,
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
    /// The pre-decadal (限前) cell shown before the normal 大限 row.
    ///
    /// `#[serde(default)]` keeps snapshots serialized before this field existed
    /// roundtripping: they decode to the disabled [`Default`] cell.
    #[serde(default)]
    pub pre_decadal_cell: StaticPreDecadalCellView,
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

/// The pre-decadal (限前) cell shown before the normal 大限 row.
///
/// Core has no childhood-limit (童限) palace-walk derivation yet, so this cell
/// only truthfully represents the nominal-age span *before* the first 大限. It
/// carries no temporal overlay: selecting it shows the natal base slice.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticPreDecadalCellView {
    /// Whether the span before the first decadal period is available.
    pub enabled: bool,
    /// Whether this cell is the current temporal selection.
    #[serde(default)]
    pub selected: bool,
    /// Chinese label for the cell, such as `限前`.
    pub label_zh: String,
    /// Inclusive nominal-age range before the first 大限, such as `1-4`.
    ///
    /// `None` when the decadal frame is unavailable or the first period already
    /// starts at age one.
    pub age_range_zh: Option<String>,
}

/// One factual or disabled decadal-period display cell.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StaticDecadalCellView {
    /// Whether factual display data is available.
    pub enabled: bool,
    /// Whether this cell is the current temporal selection.
    #[serde(default)]
    pub selected: bool,
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
    /// Whether this cell is the current temporal selection.
    #[serde(default)]
    pub selected: bool,
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
    /// Whether this cell is the current temporal selection.
    #[serde(default)]
    pub selected: bool,
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
    /// Builds the bottom panel for one drill-down selection.
    ///
    /// All enable/selected flags and lunar labels are prepared here so the
    /// renderer reads them verbatim. Rows unlock by scope: 大限 enables 流年, a
    /// 流年 enables 流月, a 流月 enables 流日, a 流日 enables 流时.
    pub(crate) fn from_selection(
        natal: &Chart,
        selection: StaticTemporalNavigationSelection,
    ) -> Self {
        let frame = build_decadal_frame(natal).ok();
        let dec_idx = selection.decadal_index();
        let year_idx = selection.year_index();
        let month_idx = selection.month_index();
        let day_idx = selection.day_index();
        let hour_idx = selection.hour_index();

        let pre_decadal_cell = pre_decadal_cell(
            natal,
            matches!(selection, StaticTemporalNavigationSelection::PreDecadal),
        );
        let decadal_cells = decadal_cells(frame.as_ref(), dec_idx);

        // The selected 大限 period unlocks and supplies the 10 流年 years.
        let selected_period = match (frame.as_ref(), dec_idx) {
            (Some(frame), Some(di)) => frame.periods().get(di),
            _ => None,
        };
        let yearly_age_cells = match selected_period {
            Some(period) => yearly_age_cells(natal, period, year_idx),
            None => disabled_yearly_age_cells(),
        };

        // The selected lunar year/month gate the month/day rows.
        let selected_lunar_year = match (selected_period, year_idx) {
            (Some(period), Some(yi)) => Some(lunar_year_of(natal, period, yi)),
            _ => None,
        };
        let month_cells = labeled_cells(&MONTH_LABELS, selected_lunar_year.is_some(), month_idx);

        let selected_lunar_month = month_idx.map(|m| m + 1);
        let day_rows = day_rows(selected_lunar_year, selected_lunar_month, day_idx);

        // Early Zi (`timeIndex` 0) and late Zi (`timeIndex` 12) share the same
        // renderer cell while retaining their distinct core selection values.
        let selected_hour_cell = hour_idx.map(|index| if index == 12 { 0 } else { index });
        let hour_cells = labeled_cells(&HOUR_LABELS, day_idx.is_some(), selected_hour_cell);

        Self {
            pre_decadal_cell,
            decadal_cells,
            yearly_age_cells,
            month_cells,
            day_rows,
            hour_cells,
        }
    }

    fn from_chart(chart: &Chart) -> Self {
        Self::from_selection(chart, StaticTemporalNavigationSelection::Natal)
    }

    fn from_horoscope_chart(chart: &HoroscopeChart) -> Self {
        Self::from_chart(chart.natal())
    }
}

/// The lunar year of the `year_index`-th 流年 (0-based) within a 大限 period.
///
/// The period's nominal ages run `start_age..=start_age+9`; nominal age 1 is the
/// natal lunar year, so `lunar_year = birth_lunar_year + nominal_age - 1`.
fn lunar_year_of(natal: &Chart, period: &DecadalPeriod, year_index: u8) -> i32 {
    let nominal_age = period.start_age() as i32 + year_index as i32;
    natal.birth_context().date().year() + nominal_age - 1
}

fn pre_decadal_cell(chart: &Chart, selected: bool) -> StaticPreDecadalCellView {
    const LABEL: &str = "限前";
    match build_decadal_frame(chart) {
        Ok(frame) => {
            let first_start = frame.periods().first().map(|period| period.start_age());
            let age_range_zh = match first_start {
                Some(start) if start > 1 => Some(format!("1-{}", start - 1)),
                _ => None,
            };
            StaticPreDecadalCellView {
                enabled: true,
                selected,
                label_zh: LABEL.to_owned(),
                age_range_zh,
            }
        }
        Err(_) => StaticPreDecadalCellView {
            enabled: false,
            selected,
            label_zh: LABEL.to_owned(),
            age_range_zh: None,
        },
    }
}

fn decadal_cells(
    frame: Option<&DecadalFrame>,
    selected_index: Option<usize>,
) -> Vec<StaticDecadalCellView> {
    match frame {
        Some(frame) => frame
            .periods()
            .iter()
            .enumerate()
            .map(|(index, period)| StaticDecadalCellView {
                enabled: true,
                selected: selected_index == Some(index),
                age_range_zh: Some(format!("{}-{}", period.start_age(), period.end_age())),
                limit_label_zh: Some(format!("{}限", zh_cn::stem_branch_zh(period.stem_branch()))),
            })
            .collect(),
        None => (0..PALACE_COUNT)
            .map(|_| StaticDecadalCellView {
                enabled: false,
                selected: false,
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
            selected: false,
            year_label: None,
            stem_branch_age_zh: None,
        })
        .collect()
}

/// The 10 流年/小限 cells for the selected 大限 period (solar-year drill-down),
/// padded with disabled cells to the 12-wide row.
fn yearly_age_cells(
    natal: &Chart,
    period: &DecadalPeriod,
    selected_index: Option<u8>,
) -> Vec<StaticYearlyAgeCellView> {
    let mut cells: Vec<StaticYearlyAgeCellView> = (0u8..10)
        .map(|year_index| {
            let nominal_age = period.start_age() + year_index;
            let lunar_year = lunar_year_of(natal, period, year_index);
            let stem_branch = StemBranch::from_lunar_year(lunar_year);
            StaticYearlyAgeCellView {
                enabled: true,
                selected: selected_index == Some(year_index),
                year_label: Some(lunar_year.to_string()),
                stem_branch_age_zh: Some(format!(
                    "{}{}",
                    zh_cn::stem_branch_zh(stem_branch),
                    nominal_age
                )),
            }
        })
        .collect();
    cells.extend(
        disabled_yearly_age_cells()
            .into_iter()
            .take(PALACE_COUNT - cells.len()),
    );
    cells
}

/// The 流日 grid (existing 3×10 layout, 初一..三十). When enabled, the 三十 cell is
/// disabled for a 29-day lunar month. Disabled (greyed) when no 流月 is selected.
fn day_rows(
    selected_lunar_year: Option<i32>,
    selected_lunar_month: Option<u8>,
    selected_index: Option<u8>,
) -> Vec<Vec<StaticNavigationCellView>> {
    let enabled = selected_lunar_month.is_some();
    let has_thirtieth = match (selected_lunar_year, selected_lunar_month) {
        (Some(year), Some(month)) => lunar_month_has_thirtieth(year, month),
        _ => false,
    };
    DAY_LABELS
        .iter()
        .enumerate()
        .map(|(row, labels)| {
            labels
                .iter()
                .enumerate()
                .map(|(col, label)| {
                    let day_index = (row * labels.len() + col) as u8;
                    // 三十 is day_index 29 (the last cell); disable it for a 29-day month.
                    let cell_enabled = enabled && (day_index != 29 || has_thirtieth);
                    StaticNavigationCellView {
                        label_zh: (*label).to_owned(),
                        enabled: cell_enabled,
                        selected: cell_enabled && selected_index == Some(day_index),
                    }
                })
                .collect()
        })
        .collect()
}

/// A single navigation row, all cells sharing one `enabled` flag, with one
/// `selected` cell by index.
fn labeled_cells(
    labels: &[&str],
    enabled: bool,
    selected_index: Option<u8>,
) -> Vec<StaticNavigationCellView> {
    labels
        .iter()
        .enumerate()
        .map(|(index, label)| StaticNavigationCellView {
            label_zh: (*label).to_owned(),
            enabled,
            selected: enabled && selected_index == Some(index as u8),
        })
        .collect()
}

/// The 三方四正 (san fang si zheng) related palaces for one palace, by branch.
///
/// These are deterministic positional facts, not interpretation. The convention
/// matches the horoscope runtime ([`opposite`](Self::opposite) = `+6`,
/// [`wealth`](Self::wealth) = `+8`, [`career`](Self::career) = `+4`), so a
/// renderer never performs branch arithmetic of its own.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct StaticSurroundPalacesView {
    /// 对宫 — the opposite palace (`+6`).
    pub opposite: EarthlyBranch,
    /// 财帛位 — the wealth trine palace (`+8`).
    pub wealth: EarthlyBranch,
    /// 官禄位 — the career trine palace (`+4`).
    pub career: EarthlyBranch,
}

impl StaticSurroundPalacesView {
    /// Builds the 三方四正 branch set for `branch` using the canonical offsets.
    pub fn for_branch(branch: EarthlyBranch) -> Self {
        Self {
            opposite: branch.offset(6),
            wealth: branch.offset(8),
            career: branch.offset(4),
        }
    }

    /// The three related branches, in `[opposite, wealth, career]` order.
    pub fn branches(&self) -> [EarthlyBranch; 3] {
        [self.opposite, self.wealth, self.career]
    }

    /// Whether `branch` is one of the three 三方四正 related palaces.
    pub fn involves(&self, branch: EarthlyBranch) -> bool {
        self.branches().contains(&branch)
    }
}

/// Prepared decadal (大限) and small-limit (小限) facts for one palace cell.
///
/// These are renderer-neutral display facts derived from the natal decadal frame
/// and age-period walk. The GUI draws them in the palace center without
/// performing any age or branch arithmetic of its own.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticPalaceLimitView {
    /// Inclusive decadal age range (大限) for this palace, such as `6-15`.
    pub decadal_age_range_zh: Option<String>,
    /// Nominal small-limit ages (小限) that land on this palace, ascending.
    #[serde(default)]
    pub small_limit_ages_zh: Vec<String>,
    /// Whether this palace holds the currently selected decadal period.
    #[serde(default)]
    pub is_active_decadal: bool,
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
    /// The 三方四正 related palaces (by branch) for this palace.
    ///
    /// A pure positional fact derived from the palace branch; it carries no
    /// interpretation. Renderers use it to highlight a palace's surrounding
    /// influence set without performing any branch arithmetic themselves.
    pub surround: StaticSurroundPalacesView,
    /// Prepared 大限 / 小限 limit facts shown in the palace center.
    #[serde(default)]
    pub limit: StaticPalaceLimitView,
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
    /// Compact prepared badge label for this period, such as `流年·丁`.
    #[serde(default)]
    pub period_label_zh: Option<String>,
    /// The period's Heavenly Stem, set only on the anchor palace that carries the
    /// period badge (mirrors [`period_label_zh`](Self::period_label_zh)). Lets a
    /// presentation layer build the badge in any locale from typed facts.
    #[serde(default)]
    pub period_stem: Option<HeavenlyStem>,
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

/// A renderer-neutral, hierarchical drill-down selection for the bottom panel.
///
/// A renderer (TUI/GUI) reports *which* bottom-panel cell the user chose as an
/// **index path** (大限 → 流年 → 流月 → 流日 → 流时); core resolves the indices to
/// concrete lunar/solar coordinates and prepares the matching
/// [`StaticChartViewSnapshot`]. Each deeper variant carries its ancestors'
/// indices. The renderer never derives the overlay, the lunar labels, or the
/// month/day validity itself.
///
/// Indices: `year_index` 0..=9 (within the 大限's 10 years); `month_index` 0..=11
/// (lunar month 正月..腊月); `day_index` 0..=29 (lunar day 初一..三十); `hour_index`
/// is upstream iztro `timeIndex` 0..=12 (early 子..亥, late 子).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum StaticTemporalNavigationSelection {
    /// 本命 — the natal slice with no temporal overlay.
    #[default]
    Natal,
    /// 限前 — the span before the first 大限. Carries no overlay; natal base.
    PreDecadal,
    /// 大限 — the selected decadal period; enables the 流年 row.
    Decadal {
        /// Zero-based index into the decadal frame periods.
        decadal_index: usize,
    },
    /// 流年/小限 — a year within the selected 大限; enables the 流月 row.
    Yearly {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Zero-based year within the period (0..=9).
        year_index: u8,
    },
    /// 流月 — a lunar month of the selected 流年; enables the 流日 row.
    Monthly {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Selected year within the period (0..=9).
        year_index: u8,
        /// Zero-based lunar month (0..=11 -> 正月..腊月).
        month_index: u8,
    },
    /// 流日 — a lunar day of the selected 流月; enables the 流时 row.
    Daily {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Selected year within the period (0..=9).
        year_index: u8,
        /// Selected lunar month (0..=11).
        month_index: u8,
        /// Zero-based lunar day (0..=29 -> 初一..三十).
        day_index: u8,
    },
    /// 流时 — a double-hour of the selected 流日.
    Hourly {
        /// Selected decadal period index.
        decadal_index: usize,
        /// Selected year within the period (0..=9).
        year_index: u8,
        /// Selected lunar month (0..=11).
        month_index: u8,
        /// Selected lunar day (0..=29).
        day_index: u8,
        /// Upstream iztro `timeIndex` (0..=12: early 子..亥, late 子).
        hour_index: u8,
    },
}

impl StaticTemporalNavigationSelection {
    /// The selected decadal period index, if the path reaches 大限.
    pub const fn decadal_index(&self) -> Option<usize> {
        match self {
            Self::Natal | Self::PreDecadal => None,
            Self::Decadal { decadal_index }
            | Self::Yearly { decadal_index, .. }
            | Self::Monthly { decadal_index, .. }
            | Self::Daily { decadal_index, .. }
            | Self::Hourly { decadal_index, .. } => Some(*decadal_index),
        }
    }

    /// The selected year index, if the path reaches 流年.
    pub const fn year_index(&self) -> Option<u8> {
        match self {
            Self::Yearly { year_index, .. }
            | Self::Monthly { year_index, .. }
            | Self::Daily { year_index, .. }
            | Self::Hourly { year_index, .. } => Some(*year_index),
            _ => None,
        }
    }

    /// The selected lunar-month index, if the path reaches 流月.
    pub const fn month_index(&self) -> Option<u8> {
        match self {
            Self::Monthly { month_index, .. }
            | Self::Daily { month_index, .. }
            | Self::Hourly { month_index, .. } => Some(*month_index),
            _ => None,
        }
    }

    /// The selected lunar-day index, if the path reaches 流日.
    pub const fn day_index(&self) -> Option<u8> {
        match self {
            Self::Daily { day_index, .. } | Self::Hourly { day_index, .. } => Some(*day_index),
            _ => None,
        }
    }

    /// The selected upstream iztro `timeIndex`, if the path reaches 流时.
    pub const fn hour_index(&self) -> Option<u8> {
        match self {
            Self::Hourly { hour_index, .. } => Some(*hour_index),
            _ => None,
        }
    }
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
        let birth_year_branch = chart.birth_year().branch();
        let time_index = chart
            .birth_context()
            .birth_time_variant()
            .iztro_time_index();
        let (birth_solar_label, birth_lunar_label, constellation_zh) = natal_date_labels(chart);
        let (birth_lunar_date, western_zodiac_sign) = natal_typed_dates(chart);
        Self {
            gender: chart.birth_context().gender(),
            birth_year_stem: chart.birth_year().stem(),
            birth_year_stem_zh: zh_cn::heavenly_stem_zh(chart.birth_year().stem()).to_owned(),
            birth_year_branch,
            birth_year_branch_zh: zh_cn::earthly_branch_zh(birth_year_branch).to_owned(),
            four_pillars: chart
                .four_pillars()
                .copied()
                .map(StaticFourPillarsView::from_four_pillars),
            five_element_bureau: chart.five_element_bureau(),
            five_element_bureau_zh: chart
                .five_element_bureau()
                .map(|bureau| zh_cn::five_element_bureau_zh(bureau).to_owned()),
            birth_solar_label,
            birth_lunar_label,
            birth_time_label: chinese_date::birth_time_label(time_index),
            zodiac_zh: zh_cn::zodiac_animal_zh(birth_year_branch).to_owned(),
            constellation_zh,
            soul_master_zh: life_palace_branch
                .map(|branch| zh_cn::star_name_zh(soul_master(branch)).to_owned()),
            body_master_zh: Some(zh_cn::star_name_zh(body_master(birth_year_branch)).to_owned()),
            // Temporal (navigation-dependent) labels are filled by the facade.
            nominal_age_label: None,
            temporal_lunar_label: None,
            temporal_solar_label: None,
            life_palace_branch,
            life_palace_branch_zh: life_palace_branch
                .map(|branch| zh_cn::earthly_branch_zh(branch).to_owned()),
            body_palace_branch,
            body_palace_branch_zh: body_palace_branch
                .map(|branch| zh_cn::earthly_branch_zh(branch).to_owned()),
            birth_lunar_date,
            birth_time_index: Some(time_index),
            western_zodiac: western_zodiac_sign,
            soul_master: life_palace_branch.map(soul_master),
            body_master: Some(body_master(birth_year_branch)),
            // Temporal (navigation-dependent) typed facts are filled by the facade.
            nominal_age: None,
            temporal_lunar_date: None,
            temporal_lunar_year: None,
        }
    }
}

/// Returns `(birth_lunar_date, western_zodiac)` for a natal chart, mirroring the
/// string labels from [`natal_date_labels`] as typed, language-neutral facts.
fn natal_typed_dates(chart: &Chart) -> (Option<LunarDateView>, Option<WesternZodiac>) {
    if let Some(facts) = chart.natal_date_facts() {
        let solar = facts.solar();
        let lunar = facts.lunar();
        return (
            Some(LunarDateView {
                year: lunar.year(),
                month: lunar.month(),
                day: lunar.day(),
                is_leap_month: lunar.is_leap_month(),
            }),
            western_zodiac(solar.month(), solar.day()),
        );
    }

    let date = chart.birth_context().date();
    match date.kind() {
        CalendarKind::Solar => (None, western_zodiac(date.month(), date.day())),
        CalendarKind::Lunar => (
            Some(LunarDateView {
                year: date.year(),
                month: date.month(),
                day: date.day(),
                is_leap_month: false,
            }),
            None,
        ),
    }
}

/// Returns `(solar_label, lunar_label, constellation_zh)` for a natal chart,
/// preferring the retained natal display dates and falling back to whatever
/// calendar the birth context carries.
fn natal_date_labels(chart: &Chart) -> (String, String, String) {
    if let Some(facts) = chart.natal_date_facts() {
        let solar = facts.solar();
        let lunar = facts.lunar();
        return (
            chinese_date::solar_date_label_padded(solar.year(), solar.month(), solar.day()),
            chinese_date::lunar_date_label(
                lunar.year(),
                lunar.month(),
                lunar.day(),
                lunar.is_leap_month(),
            ),
            chinese_date::constellation_zh(solar.month(), solar.day()).to_owned(),
        );
    }

    let date = chart.birth_context().date();
    match date.kind() {
        CalendarKind::Solar => (
            chinese_date::solar_date_label_padded(date.year(), date.month(), date.day()),
            String::new(),
            chinese_date::constellation_zh(date.month(), date.day()).to_owned(),
        ),
        CalendarKind::Lunar => (
            String::new(),
            chinese_date::lunar_date_label(date.year(), date.month(), date.day(), false),
            String::new(),
        ),
    }
}

/// Highest nominal small-limit age retained for per-palace display.
const SMALL_LIMIT_MAX_AGE: u8 = 120;

/// Precomputed per-palace 大限 / 小限 facts, derived once per chart so each
/// palace cell does not recompute the decadal frame or age-period walk.
struct PalaceLimits {
    decadal: Vec<(EarthlyBranch, u8, u8)>,
    small: Vec<(EarthlyBranch, Vec<u8>)>,
}

impl PalaceLimits {
    fn for_chart(chart: &Chart) -> Self {
        let mut decadal = Vec::new();
        if let Ok(frame) = build_decadal_frame(chart) {
            for period in frame.periods() {
                decadal.push((period.palace_branch(), period.start_age(), period.end_age()));
            }
        }

        let mut small: Vec<(EarthlyBranch, Vec<u8>)> = Vec::new();
        for base in 1u8..=PALACE_COUNT as u8 {
            if let Ok(period) = build_age_period(chart, base) {
                let mut ages = Vec::new();
                let mut age = base;
                while age <= SMALL_LIMIT_MAX_AGE {
                    ages.push(age);
                    age += PALACE_COUNT as u8;
                }
                small.push((period.palace_branch(), ages));
            }
        }

        Self { decadal, small }
    }

    fn view_for(&self, branch: EarthlyBranch) -> StaticPalaceLimitView {
        StaticPalaceLimitView {
            decadal_age_range_zh: self
                .decadal
                .iter()
                .find(|(palace_branch, _, _)| *palace_branch == branch)
                .map(|(_, start, end)| format!("{start}-{end}")),
            small_limit_ages_zh: self
                .small
                .iter()
                .find(|(palace_branch, _)| *palace_branch == branch)
                .map(|(_, ages)| ages.iter().map(u8::to_string).collect())
                .unwrap_or_default(),
            is_active_decadal: false,
        }
    }
}

/// Builds the twelve palace cells in fixed [`VISUAL_BRANCH_ORDER`], attaching any
/// overlays from `overlay_layers`.
fn build_palaces(chart: &Chart, overlay_layers: &[&TemporalLayer]) -> Vec<StaticPalaceView> {
    let limits = PalaceLimits::for_chart(chart);
    VISUAL_BRANCH_ORDER
        .into_iter()
        .filter_map(|branch| {
            chart
                .palaces()
                .iter()
                .find(|palace| palace.branch() == branch)
                .map(|palace| StaticPalaceView::from_palace(chart, palace, overlay_layers, &limits))
        })
        .collect()
}

impl StaticPalaceView {
    fn from_palace(
        chart: &Chart,
        palace: &Palace,
        overlay_layers: &[&TemporalLayer],
        limits: &PalaceLimits,
    ) -> Self {
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
            surround: StaticSurroundPalacesView::for_branch(palace.branch()),
            limit: limits.view_for(palace.branch()),
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

        // A period's compact badge belongs only on its anchor palace — the branch
        // the period relabels as 命宫 (Life). Every other branch carries the
        // overlay's stars/mutagens but no period marker, so `period_label_zh`
        // stays `None` there.
        let is_marker = period_marker_branch(layer) == Some(branch);
        let period_stem = layer.context().stem_branch().stem();
        let period_label_zh = is_marker.then(|| {
            format!(
                "{}·{}",
                zh_cn::scope_zh(layer.scope()),
                zh_cn::heavenly_stem_zh(period_stem)
            )
        });

        Self {
            scope: layer.scope(),
            period_label_zh,
            period_stem: is_marker.then_some(period_stem),
            temporal_palace_name,
            temporal_palace_name_zh: temporal_palace_name
                .map(|name| zh_cn::palace_name_zh(name).to_owned()),
            typed_stars,
            decorative_stars,
            mutagens,
        }
    }
}

/// Returns the branch of the palace that should display a temporal layer's
/// compact period badge: the period's anchor palace, i.e. the branch the
/// period's palace layout relabels as 命宫 ([`PalaceName::Life`]).
///
/// This is the single core decision of which palace is a period marker; the GUI
/// renders a badge only where this yields the overlay's branch. Returns `None`
/// when the layer carries no palace layout, so no badge is shown for it.
fn period_marker_branch(layer: &TemporalLayer) -> Option<EarthlyBranch> {
    layer
        .palace_layout()?
        .names()
        .iter()
        .find(|name| name.palace_name() == PalaceName::Life)
        .map(TemporalPalaceName::branch)
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
    fn surround_uses_canonical_offsets() {
        let snapshot = StaticChartViewSnapshot::from_chart(&sample_chart());
        for palace in &snapshot.palaces {
            // 三方四正 must match the horoscope runtime offsets exactly.
            assert_eq!(palace.surround.opposite, palace.branch.offset(6));
            assert_eq!(palace.surround.wealth, palace.branch.offset(8));
            assert_eq!(palace.surround.career, palace.branch.offset(4));
            // The palace itself is never part of its own 三方四正 set.
            assert!(!palace.surround.involves(palace.branch));
            assert_eq!(
                palace.surround.branches(),
                [
                    palace.surround.opposite,
                    palace.surround.wealth,
                    palace.surround.career
                ]
            );
        }
    }

    #[test]
    fn surround_roundtrips_through_json() {
        let snapshot = StaticChartViewSnapshot::from_chart(&sample_chart());
        let palace = &snapshot.palaces[0];
        let encoded = serde_json::to_string(palace).expect("palace serializes");
        let decoded: StaticPalaceView =
            serde_json::from_str(&encoded).expect("palace deserializes");
        assert_eq!(decoded.surround, palace.surround);
        assert!(encoded.contains("\"surround\""));
        assert!(encoded.contains("\"opposite\""));
        assert!(encoded.contains("\"wealth\""));
        assert!(encoded.contains("\"career\""));
    }

    #[test]
    fn surround_is_a_natal_fact_independent_of_overlays() {
        let chart = sample_horoscope_chart();
        let natal_only = StaticChartViewSnapshot::from_chart(chart.natal());
        let with_overlays = StaticChartViewSnapshot::from_horoscope_chart(&chart);
        for (a, b) in natal_only.palaces.iter().zip(&with_overlays.palaces) {
            assert_eq!(a.branch, b.branch);
            // 三方四正 is a positional natal fact; overlays never change it.
            assert_eq!(a.surround, b.surround);
        }
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
        // Selectors follow the fixed display order.
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
