use iced::widget::{button, column, container, mouse_area, row, text};
use iced::{Alignment, Color, Element, Length};
use iztro::core::{
    DecorativeStarFamily, Mutagen, Scope, StarCategory, StarKind, StaticChartCenterView,
    StaticChartViewSnapshot, StaticDecorativeStarView, StaticPalaceView,
    StaticTemporalNavigationSelection, StaticTypedStarView,
};
use iztro_i18n::I18n;

use crate::app::{Message, StaticChartApp};

use super::labels::{fact_row, four_pillars_line, gender_symbol, section_title};
use super::style::{
    ADJ_GRAY, BRIGHTNESS_GRAY, DECOR_GOD_OLIVE, DECORATIVE_AREA_HEIGHT, LIMIT_ACTIVE, LIMIT_GRAY,
    LU_CUN_ORANGE, MAJOR_PURPLE, MAX_STAR_COLUMNS, MAX_STAR_ROWS, MINOR_MALEFIC,
    PALACE_MIDDLE_BAND_HEIGHT, PEACH_MAGENTA, PERIOD_BADGE_ROW_HEIGHT, TIAN_MA_BLUE,
    center_panel_style, mutagen_badge_color, mutagen_inline_badge, palace_cell_style,
    section_title_style,
};
use super::temporal::{period_badge, temporal_controls};

// Palace grid
pub(super) fn palace_grid<'a>(
    app: &'a StaticChartApp,
    snapshot: &'a StaticChartViewSnapshot,
    i18n: &I18n,
) -> Element<'a, Message> {
    let top = row![
        grid_cell(app, 0, 0, i18n),
        grid_cell(app, 0, 1, i18n),
        grid_cell(app, 0, 2, i18n),
        grid_cell(app, 0, 3, i18n),
    ]
    .spacing(6)
    .height(Length::FillPortion(1));

    let left = column![grid_cell(app, 1, 0, i18n), grid_cell(app, 2, 0, i18n)]
        .spacing(6)
        .width(Length::FillPortion(1));
    let right = column![grid_cell(app, 1, 3, i18n), grid_cell(app, 2, 3, i18n)]
        .spacing(6)
        .width(Length::FillPortion(1));
    let center = container(center_panel(
        &snapshot.center,
        app.selected_temporal_selection(),
        i18n,
    ))
    .style(center_panel_style)
    .padding(10)
    .width(Length::FillPortion(2))
    .height(Length::Fill);
    let middle = row![left, center, right]
        .spacing(6)
        .height(Length::FillPortion(2));

    let bottom = row![
        grid_cell(app, 3, 0, i18n),
        grid_cell(app, 3, 1, i18n),
        grid_cell(app, 3, 2, i18n),
        grid_cell(app, 3, 3, i18n),
    ]
    .spacing(6)
    .height(Length::FillPortion(1));

    column![top, middle, bottom]
        .spacing(6)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Builds one grid cell by grid position. Perimeter cells are palaces; the
/// (rare) absent cell becomes inert filler so layout stays stable.
pub(super) fn grid_cell<'a>(
    app: &'a StaticChartApp,
    row: u8,
    column_index: u8,
    i18n: &I18n,
) -> Element<'a, Message> {
    match app.palace_at(row, column_index) {
        Some(palace) => {
            let highlight = if app.active_branch() == Some(palace.branch) {
                PalaceHighlight::Selected
            } else if app.is_in_san_fang(palace.branch) {
                // 三方四正 membership comes from the prepared `surround` field.
                PalaceHighlight::Related
            } else {
                PalaceHighlight::None
            };
            palace_cell(palace, highlight, i18n)
        }
        None => container(text("")).width(Length::FillPortion(1)).into(),
    }
}

/// How a palace cell is visually emphasized.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PalaceHighlight {
    /// No emphasis.
    None,
    /// The selected palace.
    Selected,
    /// A 三方四正 palace related to the selected palace.
    Related,
}

pub(super) fn palace_cell<'a>(
    palace: &'a StaticPalaceView,
    highlight: PalaceHighlight,
    i18n: &I18n,
) -> Element<'a, Message> {
    // Zone every prepared natal typed star by its coarse `kind.category()`:
    // major top-left, minor top-middle, adjective top-right. Routing by the
    // prepared kind keeps placement correct regardless of which source vec a
    // star arrived in; the GUI does no classification of its own.
    let (mut majors, mut minors, mut adjectives) = (Vec::new(), Vec::new(), Vec::new());
    for star in palace
        .major_stars
        .iter()
        .chain(&palace.minor_stars)
        .chain(&palace.adjective_stars)
        .chain(&palace.other_typed_stars)
    {
        match star.kind.category() {
            StarCategory::Major => majors.push(star),
            StarCategory::Minor => minors.push(star),
            StarCategory::Adjective => adjectives.push(star),
        }
    }
    // Major stars keep a single vertical column (top stars stay prominent);
    // minor and adjective zones wrap into extra columns when their lines exceed
    // `MAX_STAR_ROWS`, so a star-heavy palace grows sideways instead of running
    // down into the protected metadata below.
    let star_groups = row![
        container(typed_star_column(majors, true, i18n)).width(Length::FillPortion(3)),
        container(wrapped_star_group(
            minors,
            false,
            MAX_STAR_ROWS,
            false,
            i18n
        ))
        .width(Length::FillPortion(3)),
        container(wrapped_star_group(
            adjectives,
            false,
            MAX_STAR_ROWS,
            true,
            i18n
        ))
        .width(Length::FillPortion(2))
        .align_x(Alignment::End),
    ]
    .spacing(4)
    .align_y(Alignment::Start);

    // The flexible star area takes all height left above the protected metadata
    // and clips, so however many star lines a palace carries they can never paint
    // over the metadata zone (the CSS `min-height: 0; overflow: hidden` intent).
    let star_area = container(star_groups)
        .width(Length::Fill)
        .height(Length::Fill)
        .clip(true);

    // Decorative "twelve gods" go to the identity footer, split by prepared
    // family: 长生/博士 bottom-left (olive), 将前/岁前 bottom-right (malefic
    // tone). No group label — color and side carry the family, matching iztro.
    let (mut gods_left, mut gods_right) = (Vec::new(), Vec::new());
    for star in &palace.decorative_stars {
        match star.family {
            DecorativeStarFamily::Changsheng12 | DecorativeStarFamily::Boshi12 => {
                gods_left.push(star)
            }
            DecorativeStarFamily::Jiangqian12 | DecorativeStarFamily::Suiqian12 => {
                gods_right.push(star)
            }
        }
    }

    // The metadata zone is a fixed-height column pinned below the star area, so
    // it is always visible and its time-flow / identity rows keep a constant
    // y-position across every palace regardless of star count.
    let metadata = palace_metadata(palace, highlight, gods_left, gods_right, i18n);

    let content: Element<'_, Message> = column![star_area, metadata]
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

    let cell = button(content)
        .on_press(Message::SelectPalace(palace.branch))
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .padding(6)
        .style(palace_cell_style(highlight));

    // Hovering a palace drives the 三方四正 highlight; the exit carries the
    // branch so a stale exit cannot clear a newer hover.
    mouse_area(cell)
        .on_enter(Message::HoverPalace(palace.branch))
        .on_exit(Message::ClearHoveredPalace(palace.branch))
        .into()
}

/// GUI-only visual tone for a typed star, derived from its prepared `kind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StaticStarTone {
    /// Fourteen major stars (主星).
    Major,
    /// Auspicious soft minor pair stars (左辅右弼天魁天钺文昌文曲).
    MinorPurple,
    /// Six malefics / 六煞 (擎羊陀罗火星铃星地空地劫).
    MinorMalefic,
    /// 禄存.
    LuCun,
    /// 天马.
    TianMa,
    /// Ordinary adjective / miscellaneous stars (杂曜).
    AdjDefault,
    /// 桃花 / festive relationship stars (红鸾咸池天姚天喜, flow variants).
    AdjPeachBlossom,
}

/// Classifies a prepared typed star into a display tone by its `kind`. This is
/// pure visual classification of an already-derived core field — no astrology.
pub(super) fn star_tone(star: &StaticTypedStarView) -> StaticStarTone {
    match star.kind {
        StarKind::Major => StaticStarTone::Major,
        StarKind::Soft => StaticStarTone::MinorPurple,
        StarKind::Tough => StaticStarTone::MinorMalefic,
        StarKind::LuCun => StaticStarTone::LuCun,
        StarKind::TianMa => StaticStarTone::TianMa,
        StarKind::Flower => StaticStarTone::AdjPeachBlossom,
        StarKind::Adjective | StarKind::Helper => StaticStarTone::AdjDefault,
    }
}

/// The star-name color for a display tone.
fn star_color(tone: StaticStarTone) -> Color {
    match tone {
        StaticStarTone::Major | StaticStarTone::MinorPurple => MAJOR_PURPLE,
        StaticStarTone::MinorMalefic => MINOR_MALEFIC,
        StaticStarTone::LuCun => LU_CUN_ORANGE,
        StaticStarTone::TianMa => TIAN_MA_BLUE,
        StaticStarTone::AdjDefault => ADJ_GRAY,
        StaticStarTone::AdjPeachBlossom => PEACH_MAGENTA,
    }
}

/// One star line: name (tone color, bold for majors) + inline brightness
/// (gray) + inline mutagen badge. Star name, brightness, and mutagen are
/// localized from the prepared typed fields.
fn star_line(star: &StaticTypedStarView, major: bool, i18n: &I18n) -> Element<'static, Message> {
    // Majors are emphasized by larger size + tone color only. The bundled CJK
    // font ships a single (Regular) weight; requesting Bold makes cosmic-text
    // fall back to a non-CJK face and render the names as tofu, so no bold here.
    let color = star_color(star_tone(star));
    let size = if major { 15 } else { 12 };
    let name = text(i18n.star_name(star.name)).size(size).color(color);
    let mut line = row![name].spacing(1).align_y(Alignment::Center);
    let brightness = i18n.brightness(star.brightness);
    if !brightness.is_empty() {
        line = line.push(text(brightness).size(size - 2).color(BRIGHTNESS_GRAY));
    }
    if let Some(mutagen) = star.mutagen {
        let label = i18n.mutagen(mutagen);
        line = line.push(mutagen_inline_badge(mutagen, &label));
    }
    line.into()
}

/// A vertical stack of typed star lines for one palace-cell zone.
fn typed_star_column(
    stars: Vec<&StaticTypedStarView>,
    major: bool,
    i18n: &I18n,
) -> Element<'static, Message> {
    let mut col = column![].spacing(1);
    for star in stars {
        col = col.push(star_line(star, major, i18n));
    }
    col.into()
}

/// Lays typed stars out in column-major order with at most `max_rows` lines per
/// column, wrapping into a new column when a column fills — the Rust analogue of
/// a CSS `grid-auto-flow: column; grid-template-rows: repeat(max_rows, …)` group.
///
/// `max_rows` is a parameter (not a baked-in constant) so a future
/// `responsive`-based caller can derive it from the real star-area height.
/// Beyond `MAX_STAR_COLUMNS` columns the remaining stars collapse into a compact
/// `+N` indicator, bounding horizontal growth. `align_end` right-aligns each
/// column for the right-hand adjective zone.
fn wrapped_star_group(
    stars: Vec<&StaticTypedStarView>,
    major: bool,
    max_rows: usize,
    align_end: bool,
    i18n: &I18n,
) -> Element<'static, Message> {
    let max_rows = max_rows.max(1);
    let capacity = max_rows.saturating_mul(MAX_STAR_COLUMNS);
    // Reserve the last cell for the `+N` marker when stars exceed the grid.
    let (visible, overflow) = if stars.len() > capacity {
        (&stars[..capacity - 1], stars.len() - (capacity - 1))
    } else {
        (&stars[..], 0)
    };

    let chunks: Vec<&[&StaticTypedStarView]> = visible.chunks(max_rows).collect();
    let last = chunks.len().saturating_sub(1);
    let mut columns = row![].spacing(4).align_y(Alignment::Start);
    for (index, chunk) in chunks.iter().enumerate() {
        let mut col = column![].spacing(1);
        if align_end {
            col = col.align_x(Alignment::End);
        }
        for star in *chunk {
            col = col.push(star_line(star, major, i18n));
        }
        if overflow > 0 && index == last {
            col = col.push(text(format!("+{overflow}")).size(11).color(ADJ_GRAY));
        }
        columns = columns.push(col);
    }
    columns.into()
}

/// A vertical stack of decorative "twelve gods" star names in one tone.
fn decorative_column(
    stars: Vec<&StaticDecorativeStarView>,
    color: Color,
    i18n: &I18n,
) -> Element<'static, Message> {
    let mut col = column![].spacing(1);
    for star in stars {
        col = col.push(text(i18n.star_name(star.name)).size(10).color(color));
    }
    col.into()
}

/// The protected metadata zone pinned below the flexible star area: a fixed
/// total height so it is always visible and its rows align across all palaces.
///
/// It stacks two parts, matching the desired bottom layout:
/// 1. a time-flow band — 流年/流月/… period badges over the 大限/小限 line;
/// 2. an identity footer — decorative gods over the 宫名 / 干支 labels.
///
/// Both parts have fixed heights, so the 大限/小限 line and the 宫名 / 干支 row
/// keep a constant y-position regardless of how many stars sit above them.
fn palace_metadata<'a>(
    palace: &'a StaticPalaceView,
    highlight: PalaceHighlight,
    gods_left: Vec<&'a StaticDecorativeStarView>,
    gods_right: Vec<&'a StaticDecorativeStarView>,
    i18n: &I18n,
) -> Element<'a, Message> {
    // 流年/流月/流日/流时 badges sit above the 大限/小限 line. Only overlays core
    // marked as a period anchor (typed `period_stem` set) get a badge; non-marker
    // palaces carry the overlay's stars but no badge.
    let is_source = matches!(highlight, PalaceHighlight::Selected);
    let mut badges = row![].spacing(3);
    for overlay in &palace.overlays {
        if let Some(stem) = overlay.period_stem {
            let label = format!("{}·{}", i18n.temporal_label(overlay.scope), i18n.stem(stem));
            badges = badges.push(period_badge(&label, palace.branch, is_source));
        }
    }
    // Always reserve the badge-row height (an empty placeholder when there is no
    // badge) so the 大限/小限 line keeps the same y-position whether or not a
    // palace has a period badge.
    let badge_row = container(badges)
        .width(Length::Fill)
        .height(Length::Fixed(PERIOD_BADGE_ROW_HEIGHT))
        .align_x(Alignment::Center);
    let flow = container(
        column![badge_row, limit_middle(palace, i18n)]
            .spacing(2)
            .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fixed(PALACE_MIDDLE_BAND_HEIGHT))
    .align_y(Alignment::End);

    column![flow, palace_identity(palace, gods_left, gods_right, i18n)]
        .width(Length::Fill)
        .into()
}

/// The fixed-height identity footer: decorative "twelve gods" above the localized
/// 宫名 (left) and 干支 (right), bottom-anchored so the name/stem-branch row pins
/// to the cell's bottom edge. Both labels come from typed fields, not
/// pre-rendered Chinese strings.
fn palace_identity<'a>(
    palace: &'a StaticPalaceView,
    gods_left: Vec<&'a StaticDecorativeStarView>,
    gods_right: Vec<&'a StaticDecorativeStarView>,
    i18n: &I18n,
) -> Element<'a, Message> {
    let left = column![
        container(decorative_column(gods_left, DECOR_GOD_OLIVE, i18n)).width(Length::Fill),
        text(i18n.palace_name(palace.name))
            .size(16)
            .color(MAJOR_PURPLE),
    ]
    .spacing(1)
    .align_x(Alignment::Start);
    let right = column![
        container(decorative_column(gods_right, MINOR_MALEFIC, i18n))
            .width(Length::Fill)
            .align_x(Alignment::End),
        text(i18n.stem_branch(palace.stem, palace.branch))
            .size(12)
            .color(mutagen_badge_color(Mutagen::Ke)),
    ]
    .spacing(1)
    .align_x(Alignment::End);
    let identity_row = row![
        container(left)
            .width(Length::FillPortion(1))
            .align_x(Alignment::Start),
        container(right)
            .width(Length::FillPortion(1))
            .align_x(Alignment::End),
    ]
    .spacing(4);

    container(identity_row)
        .width(Length::Fill)
        .height(Length::Fixed(DECORATIVE_AREA_HEIGHT))
        .align_y(Alignment::End)
        .into()
}

/// The 大限 / 小限 limit facts shown in the middle of a palace cell, between the
/// top stars and the bottom decorative footer. All values are prepared by core;
/// only the 大限 / 小限 prefixes are localized.
///
/// 小限 (Minor Limit) is a palace middle-band age marker, not a 流年 period
/// badge: the palace holding the selected nominal age's 小限 is emphasized with
/// the same active color as the active 大限, while the badge mechanism used by
/// 流年 is deliberately not reused here.
fn limit_middle(palace: &StaticPalaceView, i18n: &I18n) -> Element<'static, Message> {
    let decadal_color = if palace.limit.is_active_decadal {
        LIMIT_ACTIVE
    } else {
        LIMIT_GRAY
    };
    let mut col = column![].spacing(0).align_x(Alignment::Center);
    if let Some(range) = palace.limit.decadal_age_range_zh.as_deref() {
        let prefix = i18n.temporal_label(Scope::Decadal);
        col = col.push(
            text(format!("{prefix} {range}"))
                .size(9)
                .color(decadal_color),
        );
    }
    // 小限 (Minor Limit) middle-band marker. The active palace shows the
    // localized label plus only the selected age (e.g. `小限 34` / `Minor
    // Limit 34`); inactive palaces show just their compact age list with no
    // prefix, so an English label never widens all twelve cells. The
    // Chinese-string field is only a transitional fallback for the age list.
    let small_limit = if palace.limit.is_active_small_limit {
        palace.limit.active_small_limit_age.map(|age| {
            let prefix = i18n.temporal_label(Scope::Age);
            (format!("{prefix} {age}"), LIMIT_ACTIVE)
        })
    } else {
        let ages = if !palace.limit.small_limit_ages.is_empty() {
            palace
                .limit
                .small_limit_ages
                .iter()
                .map(u16::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            palace.limit.small_limit_ages_zh.join(" ")
        };
        (!ages.is_empty()).then_some((ages, LIMIT_GRAY))
    };
    if let Some((label, color)) = small_limit {
        col = col.push(text(label).size(8).color(color));
    }
    container(col)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into()
}

/// The iztro-style center information block: a `♂/♀` basic-info panel followed by
/// a period-info panel with the compact temporal stepper. Data values come from
/// prepared typed fields; labels are localized.
pub(super) fn center_panel(
    center: &StaticChartCenterView,
    selection: StaticTemporalNavigationSelection,
    i18n: &I18n,
) -> Element<'static, Message> {
    let dash = || "—".to_owned();
    let basic_header = text(format!(
        "{}{}",
        gender_symbol(center.gender),
        i18n.text("center-basic-info")
    ))
    .size(14)
    .style(section_title_style);

    let bureau = center
        .five_element_bureau
        .map(|b| i18n.bureau(b))
        .unwrap_or_else(dash);
    let four_pillars = four_pillars_line(center, i18n).unwrap_or_else(dash);
    let birth_lunar = center
        .birth_lunar_date
        .as_ref()
        .map(|d| i18n.lunar_date(d))
        .unwrap_or_else(dash);
    let zodiac = i18n.zodiac_animal(center.birth_year_branch);
    let soul_master = center
        .soul_master
        .map(|s| i18n.master(s))
        .unwrap_or_else(dash);
    let life_palace = center
        .life_palace_branch
        .map(|b| i18n.branch(b))
        .unwrap_or_else(dash);

    let basic_left = column![
        fact_row(i18n, &i18n.text("center-five-element-bureau"), bureau),
        fact_row(i18n, &i18n.text("center-four-pillars"), four_pillars),
        fact_row(i18n, &i18n.text("center-lunar"), birth_lunar),
        fact_row(i18n, &i18n.text("center-zodiac"), zodiac),
        fact_row(i18n, &i18n.text("center-soul-master"), soul_master),
        fact_row(i18n, &i18n.text("center-life-palace"), life_palace),
    ]
    .spacing(2)
    .width(Length::FillPortion(1));

    let nominal_age = center
        .nominal_age
        .map(|n| i18n.nominal_age(n))
        .unwrap_or_else(dash);
    let birth_time = center
        .birth_time_index
        .map(|t| i18n.double_hour(t))
        .unwrap_or_else(dash);
    let constellation = center
        .western_zodiac
        .map(|s| i18n.constellation(s))
        .unwrap_or_else(dash);
    let body_master = center
        .body_master
        .map(|s| i18n.master(s))
        .unwrap_or_else(dash);
    let body_palace = center
        .body_palace_branch
        .map(|b| i18n.branch(b))
        .unwrap_or_else(dash);
    let birth_solar = if center.birth_solar_label.is_empty() {
        dash()
    } else {
        center.birth_solar_label.clone()
    };

    let basic_right = column![
        fact_row(i18n, &i18n.text("center-nominal-age"), nominal_age),
        fact_row(i18n, &i18n.text("center-solar"), birth_solar),
        fact_row(i18n, &i18n.text("center-birth-time"), birth_time),
        fact_row(i18n, &i18n.text("center-constellation"), constellation),
        fact_row(i18n, &i18n.text("center-body-master"), body_master),
        fact_row(i18n, &i18n.text("center-body-palace"), body_palace),
    ]
    .spacing(2)
    .width(Length::FillPortion(1));
    let basic = column![
        basic_header,
        row![basic_left, basic_right]
            .spacing(12)
            .width(Length::Fill),
    ]
    .spacing(2);

    // Run-limit (运限) lunar: full typed date when known, else the year-only
    // fallback, else a dash. The solar label is already locale-neutral.
    let temporal_lunar = center
        .temporal_lunar_date
        .as_ref()
        .map(|d| i18n.lunar_date(d))
        .or_else(|| center.temporal_lunar_year.map(|y| i18n.lunar_year(y)))
        .unwrap_or_else(dash);
    let temporal_solar = center.temporal_solar_label.clone().unwrap_or_else(dash);

    // 小限 (Minor Limit): the palace branch the selected nominal age lands on.
    // The age itself is the nominal age already shown above, so this row carries
    // only the branch to stay compact and avoid wrapping the one-line stepper
    // below. It is an annual age marker (Scope::Age), distinct from the 流年
    // run-limit above; both are typed facts from core and the label is localized.
    let small_limit = center
        .small_limit_branch
        .map(|branch| i18n.branch(branch))
        .unwrap_or_else(dash);

    let run_xian = column![
        section_title(&i18n.text("center-temporal-info")),
        fact_row(i18n, &i18n.text("center-lunar"), temporal_lunar),
        fact_row(i18n, &i18n.text("center-solar"), temporal_solar),
        fact_row(i18n, &i18n.temporal_label(Scope::Age), small_limit),
        temporal_controls(selection, i18n),
    ]
    .spacing(2);

    column![basic, run_xian].spacing(10).into()
}
