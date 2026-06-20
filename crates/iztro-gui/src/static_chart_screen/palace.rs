use iced::widget::{button, column, container, mouse_area, row, stack, text};
use iced::{Alignment, Color, Element, Length, Padding};
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
    LU_CUN_ORANGE, MAJOR_PURPLE, MINOR_MALEFIC, PALACE_MIDDLE_BAND_HEIGHT, PEACH_MAGENTA,
    PERIOD_BADGE_ROW_HEIGHT, TIAN_MA_BLUE, center_panel_style, mutagen_badge_color,
    mutagen_inline_badge, palace_cell_style, section_title_style,
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
    let star_area = row![
        container(typed_star_column(majors, true, i18n)).width(Length::FillPortion(3)),
        container(typed_star_column(minors, false, i18n)).width(Length::FillPortion(3)),
        container(typed_star_column(adjectives, false, i18n))
            .width(Length::FillPortion(2))
            .align_x(Alignment::End),
    ]
    .spacing(4)
    .align_y(Alignment::Start);

    // Top star content is anchored to the top of the cell.
    let star_layer = container(star_area)
        .width(Length::Fill)
        .height(Length::Fill);

    // 流年/流月/流日/流时 badges sit in the reserved middle band, above the
    // 大限/小限 line. Only overlays core marked as a period anchor
    // (`period_label_zh.is_some()`) get a badge; non-marker palaces carry the
    // overlay's stars but no badge.
    let is_source = matches!(highlight, PalaceHighlight::Selected);
    let mut badges = row![].spacing(3);
    for overlay in &palace.overlays {
        // The badge appears only on the period's anchor palace, where core sets
        // the typed period stem. Built from typed facts so it localizes.
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

    // The middle band is a fixed-height layer centered vertically in the cell
    // (above the anchored bottom footer), so the badge row and 大限/小限 line
    // align across every palace regardless of how many stars sit above them.
    let middle_band = container(
        column![badge_row, limit_middle(palace, i18n)]
            .spacing(2)
            .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fixed(PALACE_MIDDLE_BAND_HEIGHT));
    let middle_layer = container(middle_band)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(Alignment::Center)
        .padding(Padding {
            bottom: DECORATIVE_AREA_HEIGHT,
            ..Padding::ZERO
        });

    // Decorative "twelve gods" go to the bottom, split by prepared family:
    // 长生/博士 bottom-left (olive), 将前/岁前 bottom-right (malefic tone). No
    // group label — color and side carry the family, matching iztro cells.
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
    let content: Element<'_, Message> = stack![
        star_layer,
        middle_layer,
        bottom_decorative_layer(palace, gods_left, gods_right, i18n),
    ]
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

/// Renders decorative stars independently from variable-height main/overlay
/// content, keeping both prepared family zones visible above the anchored
/// palace-name footer labels.
fn bottom_decorative_layer<'a>(
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
    let decorative_area = row![
        container(left)
            .width(Length::FillPortion(1))
            .align_x(Alignment::Start),
        container(right)
            .width(Length::FillPortion(1))
            .align_x(Alignment::End),
    ]
    .spacing(4);

    container(decorative_area)
        .width(Length::Fill)
        .height(Length::Fill)
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
    // 小限 ages from the typed field; the Chinese-string field is only a
    // transitional fallback when the typed ages are absent.
    let small_limit_ages = if !palace.limit.small_limit_ages.is_empty() {
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
    if !small_limit_ages.is_empty() {
        let small_limit_color = if palace.limit.is_active_small_limit {
            LIMIT_ACTIVE
        } else {
            LIMIT_GRAY
        };
        let prefix = i18n.temporal_label(Scope::Age);
        col = col.push(
            text(format!("{prefix} {small_limit_ages}"))
                .size(8)
                .color(small_limit_color),
        );
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

    // 小限 (Minor Limit): the selected nominal age and the palace branch it
    // lands on. Both are typed facts from core; the label is localized. This is
    // an annual age marker (Scope::Age), distinct from the 流年 run-limit above.
    let small_limit = match (center.small_limit_age, center.small_limit_branch) {
        (Some(age), Some(branch)) => {
            format!("{} / {}", i18n.nominal_age(age), i18n.branch(branch))
        }
        _ => dash(),
    };

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
