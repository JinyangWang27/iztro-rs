use iced::widget::{button, column, container, mouse_area, row, stack, text};
use iced::{Alignment, Color, Element, Length, Padding};
use iztro::core::{
    DecorativeStarFamily, Mutagen, StarCategory, StarKind, StaticChartCenterView,
    StaticChartViewSnapshot, StaticDecorativeStarView, StaticPalaceView, StaticTypedStarView,
};

use crate::app::{Message, StaticChartApp};

use super::labels::{bureau_label, center_four_pillar_rows, fact_row, gender_zh, section_title};
use super::style::{
    ADJ_GRAY, BRIGHTNESS_GRAY, DECOR_GOD_OLIVE, DECORATIVE_AREA_HEIGHT, LU_CUN_ORANGE,
    MAJOR_PURPLE, MINOR_MALEFIC, PEACH_MAGENTA, TIAN_MA_BLUE, center_panel_style,
    mutagen_inline_badge, palace_cell_style, subtle_text_style,
};
use super::temporal::overlay_badges;

// Palace grid
pub(super) fn palace_grid<'a>(
    app: &'a StaticChartApp,
    snapshot: &'a StaticChartViewSnapshot,
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
    let center = container(center_panel(&snapshot.center))
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
    let header = column![
        text(palace.name_zh.as_str()).size(16),
        text(format!("{}{}", palace.stem_zh, palace.branch_zh)).size(12),
    ]
    .spacing(1);

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

    let mut content = column![header, star_area].spacing(4);
    for overlay in &palace.overlays {
        if overlay.temporal_palace_name_zh.is_none()
            && overlay.typed_stars.is_empty()
            && overlay.decorative_stars.is_empty()
            && overlay.mutagens.is_empty()
        {
            continue;
        }
        content = content.push(overlay_badges(overlay));
    }

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
    let has_decorative = !gods_left.is_empty() || !gods_right.is_empty();
    let content: Element<'_, Message> = if has_decorative {
        let main_layer = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding {
                bottom: DECORATIVE_AREA_HEIGHT,
                ..Padding::ZERO
            });
        stack![main_layer, bottom_decorative_layer(gods_left, gods_right),]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    } else {
        content.into()
    };

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
/// content, keeping both prepared family zones visible at the cell bottom.
fn bottom_decorative_layer(
    gods_left: Vec<&StaticDecorativeStarView>,
    gods_right: Vec<&StaticDecorativeStarView>,
) -> Element<'static, Message> {
    let decorative_area = row![
        container(decorative_column(gods_left, DECOR_GOD_OLIVE)).width(Length::FillPortion(1)),
        container(decorative_column(gods_right, MINOR_MALEFIC))
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

pub(super) fn center_panel(center: &StaticChartCenterView) -> Element<'_, Message> {
    let mut four_pillars = column![section_title("四柱")].spacing(3);
    if center.four_pillars.is_some() {
        for (label, value) in center_four_pillar_rows(center) {
            four_pillars = four_pillars.push(fact_row(label, value));
        }
    } else {
        four_pillars = four_pillars.push(text("四柱：未提供").size(13).style(subtle_text_style));
    }

    let facts = column![
        section_title("基本"),
        fact_row("性别", gender_zh(center.gender)),
        fact_row("五行局", bureau_label(center)),
    ]
    .spacing(3);

    let palace_facts = column![
        section_title("宫位"),
        fact_row(
            "命宫",
            center.life_palace_branch_zh.as_deref().unwrap_or("未提供")
        ),
        fact_row(
            "身宫",
            center.body_palace_branch_zh.as_deref().unwrap_or("未提供")
        ),
    ]
    .spacing(3);

    let content = column![text("命盘").size(22), four_pillars, facts, palace_facts].spacing(10);
    content.into()
}

/// Compact color legend matching the new palace-cell tones, so cells need no
/// in-cell category labels.
pub(super) fn category_legend() -> Element<'static, Message> {
    row![
        text("图例").size(12).style(subtle_text_style),
        legend_item("主星/辅星", MAJOR_PURPLE),
        legend_item("六煞", MINOR_MALEFIC),
        legend_item("禄存", LU_CUN_ORANGE),
        legend_item("天马", TIAN_MA_BLUE),
        legend_item("桃花", PEACH_MAGENTA),
        legend_item("杂曜", ADJ_GRAY),
        legend_item("长生/博士", DECOR_GOD_OLIVE),
        legend_item("将前/岁前", MINOR_MALEFIC),
        text("四化").size(12).style(subtle_text_style),
        mutagen_inline_badge(Mutagen::Lu, "禄"),
        mutagen_inline_badge(Mutagen::Quan, "权"),
        mutagen_inline_badge(Mutagen::Ke, "科"),
        mutagen_inline_badge(Mutagen::Ji, "忌"),
    ]
    .spacing(6)
    .align_y(Alignment::Center)
    .into()
}

/// One legend label rendered in its tone color.
fn legend_item(label: &str, color: Color) -> Element<'static, Message> {
    text(label.to_owned()).size(12).color(color).into()
}
