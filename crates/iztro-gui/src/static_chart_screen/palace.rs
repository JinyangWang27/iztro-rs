use iced::widget::{button, column, container, mouse_area, row, stack, text};
use iced::{Alignment, Color, Element, Length, Padding};
use iztro::core::{
    DecorativeStarFamily, Mutagen, StarCategory, StarKind, StaticChartCenterView,
    StaticChartViewSnapshot, StaticDecorativeStarView, StaticPalaceView,
    StaticTemporalNavigationSelection, StaticTypedStarView,
};

use crate::app::{LocalSolarMoment, Message, StaticChartApp};

use super::labels::{fact_row, four_pillars_line, gender_symbol, section_title};
use super::style::{
    ADJ_GRAY, BRIGHTNESS_GRAY, DECOR_GOD_OLIVE, DECORATIVE_AREA_HEIGHT, LIMIT_ACTIVE, LIMIT_GRAY,
    LU_CUN_ORANGE, MAJOR_PURPLE, MINOR_MALEFIC, PEACH_MAGENTA, TIAN_MA_BLUE, center_panel_style,
    mutagen_badge_color, mutagen_inline_badge, palace_cell_style, section_title_style,
};
use super::temporal::{period_badge, temporal_controls};

// Palace grid
pub(super) fn palace_grid<'a>(
    app: &'a StaticChartApp,
    snapshot: &'a StaticChartViewSnapshot,
    now: LocalSolarMoment,
) -> Element<'a, Message> {
    let top = row![
        grid_cell(app, 0, 0),
        grid_cell(app, 0, 1),
        grid_cell(app, 0, 2),
        grid_cell(app, 0, 3),
    ]
    .spacing(6)
    .height(Length::FillPortion(1));

    let left = column![grid_cell(app, 1, 0), grid_cell(app, 2, 0)]
        .spacing(6)
        .width(Length::FillPortion(1));
    let right = column![grid_cell(app, 1, 3), grid_cell(app, 2, 3)]
        .spacing(6)
        .width(Length::FillPortion(1));
    let center = container(center_panel(
        &snapshot.center,
        app.selected_temporal_selection(),
        now,
    ))
    .style(center_panel_style)
    .padding(10)
    .width(Length::FillPortion(2))
    .height(Length::Fill);
    let middle = row![left, center, right]
        .spacing(6)
        .height(Length::FillPortion(2));

    let bottom = row![
        grid_cell(app, 3, 0),
        grid_cell(app, 3, 1),
        grid_cell(app, 3, 2),
        grid_cell(app, 3, 3),
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
pub(super) fn grid_cell(app: &StaticChartApp, row: u8, column_index: u8) -> Element<'_, Message> {
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
            palace_cell(palace, highlight)
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

pub(super) fn palace_cell(
    palace: &StaticPalaceView,
    highlight: PalaceHighlight,
) -> Element<'_, Message> {
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
        container(typed_star_column(majors, true)).width(Length::FillPortion(3)),
        container(typed_star_column(minors, false)).width(Length::FillPortion(3)),
        container(typed_star_column(adjectives, false))
            .width(Length::FillPortion(2))
            .align_x(Alignment::End),
    ]
    .spacing(4)
    .align_y(Alignment::Start);

    let mut content = column![star_area].spacing(3);

    // 流年/流月/流日/流时 badges sit above the 大限/小限 middle area.
    let is_source = matches!(highlight, PalaceHighlight::Selected);
    if !palace.overlays.is_empty() {
        let mut badges = row![].spacing(3);
        for overlay in &palace.overlays {
            badges = badges.push(period_badge(overlay, palace.branch, is_source));
        }
        content = content.push(badges);
    }

    content = content.push(limit_middle(palace));

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
    let main_layer = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding {
            bottom: DECORATIVE_AREA_HEIGHT,
            ..Padding::ZERO
        });
    let content: Element<'_, Message> = stack![
        main_layer,
        bottom_decorative_layer(palace, gods_left, gods_right),
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
/// (gray) + inline 科权禄忌 badge. All fields are prepared core values.
fn star_line(star: &StaticTypedStarView, major: bool) -> Element<'static, Message> {
    // Majors are emphasized by larger size + tone color only. The bundled CJK
    // font ships a single (Regular) weight; requesting Bold makes cosmic-text
    // fall back to a non-CJK face and render the names as tofu, so no bold here.
    let color = star_color(star_tone(star));
    let size = if major { 15 } else { 12 };
    let name = text(star.name_zh.clone()).size(size).color(color);
    let mut line = row![name].spacing(1).align_y(Alignment::Center);
    if !star.brightness_zh.is_empty() {
        line = line.push(
            text(star.brightness_zh.clone())
                .size(size - 2)
                .color(BRIGHTNESS_GRAY),
        );
    }
    if let (Some(mutagen), Some(label)) = (star.mutagen, star.mutagen_zh.as_deref()) {
        line = line.push(mutagen_inline_badge(mutagen, label));
    }
    line.into()
}

/// A vertical stack of typed star lines for one palace-cell zone.
fn typed_star_column(stars: Vec<&StaticTypedStarView>, major: bool) -> Element<'static, Message> {
    let mut col = column![].spacing(1);
    for star in stars {
        col = col.push(star_line(star, major));
    }
    col.into()
}

/// A vertical stack of decorative "twelve gods" star names in one tone.
fn decorative_column(
    stars: Vec<&StaticDecorativeStarView>,
    color: Color,
) -> Element<'static, Message> {
    let mut col = column![].spacing(1);
    for star in stars {
        col = col.push(text(star.name_zh.clone()).size(10).color(color));
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
) -> Element<'a, Message> {
    let left = column![
        container(decorative_column(gods_left, DECOR_GOD_OLIVE)).width(Length::Fill),
        text(palace.name_zh.as_str()).size(16).color(MAJOR_PURPLE),
    ]
    .spacing(1)
    .align_x(Alignment::Start);
    let right = column![
        container(decorative_column(gods_right, MINOR_MALEFIC))
            .width(Length::Fill)
            .align_x(Alignment::End),
        text(format!("{}{}", palace.stem_zh, palace.branch_zh))
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
/// top stars and the bottom decorative footer. All values are prepared by core.
fn limit_middle(palace: &StaticPalaceView) -> Element<'static, Message> {
    let decadal_color = if palace.limit.is_active_decadal {
        LIMIT_ACTIVE
    } else {
        LIMIT_GRAY
    };
    let mut col = column![].spacing(0).align_x(Alignment::Center);
    if let Some(range) = palace.limit.decadal_age_range_zh.as_deref() {
        col = col.push(text(format!("大限 {range}")).size(9).color(decadal_color));
    }
    if !palace.limit.small_limit_ages_zh.is_empty() {
        col = col.push(
            text(palace.limit.small_limit_ages_zh.join(" "))
                .size(8)
                .color(LIMIT_GRAY),
        );
    }
    container(col)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into()
}

/// The iztro-style center information block: a `♂/♀基本信息` panel followed by a
/// `运限信息` panel with the compact temporal stepper. Every value is a prepared
/// core field; the GUI computes none of them.
pub(super) fn center_panel(
    center: &StaticChartCenterView,
    selection: StaticTemporalNavigationSelection,
    now: LocalSolarMoment,
) -> Element<'static, Message> {
    let dash = "—";
    let basic_header = text(format!("{}基本信息", gender_symbol(center.gender)))
        .size(14)
        .style(section_title_style);
    let basic = column![
        basic_header,
        fact_row(
            "五行局",
            center.five_element_bureau_zh.as_deref().unwrap_or(dash)
        ),
        fact_row(
            "年龄(虚岁)",
            center.nominal_age_label.as_deref().unwrap_or(dash)
        ),
        fact_row(
            "四柱",
            four_pillars_line(center).unwrap_or_else(|| dash.to_owned())
        ),
        fact_row("阳历", center.birth_solar_label.as_str()),
        fact_row("农历", center.birth_lunar_label.as_str()),
        fact_row("时辰", center.birth_time_label.as_str()),
        fact_row("生肖", center.zodiac_zh.as_str()),
        fact_row("星座", center.constellation_zh.as_str()),
        fact_row("命主", center.soul_master_zh.as_deref().unwrap_or(dash)),
        fact_row("身主", center.body_master_zh.as_deref().unwrap_or(dash)),
        fact_row(
            "命宫",
            center.life_palace_branch_zh.as_deref().unwrap_or(dash)
        ),
        fact_row(
            "身宫",
            center.body_palace_branch_zh.as_deref().unwrap_or(dash)
        ),
    ]
    .spacing(2);

    let run_xian = column![
        section_title("运限信息"),
        fact_row(
            "农历",
            center.temporal_lunar_label.as_deref().unwrap_or(dash)
        ),
        fact_row(
            "阳历",
            center.temporal_solar_label.as_deref().unwrap_or(dash)
        ),
        temporal_controls(selection, now),
    ]
    .spacing(2);

    column![basic, run_xian].spacing(10).into()
}
