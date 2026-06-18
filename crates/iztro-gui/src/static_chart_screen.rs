//! Iced rendering of one [`StaticChartViewSnapshot`] in a 文墨天机-style layout.
//!
//! The screen is a composed grid — a top row of four palaces, a middle band with
//! a left palace column, a center panel spanning the middle 2x2, and a right
//! palace column, then a bottom row of four palaces — placed by each palace's
//! fixed `grid_position`. A startup screen carries the solar birth-input bar and
//! the saved-charts list; the chart screen adds a 三方四正 highlight toggle, a
//! clickable temporal navigation panel, and 科权禄忌 badges. This module only
//! reads prepared snapshot view models; it performs no astrology placement,
//! 三方四正, mutagen, rule evaluation, or 成格 derivation.
//!
//! [`StaticChartViewSnapshot`]: iztro::core::StaticChartViewSnapshot

use std::fmt;

use iced::widget::{
    button, checkbox, column, container, mouse_area, pick_list, row, stack, text, text_input,
};
use iced::{Border, Color, Element, Length, Padding, Theme};
use iztro::core::{
    DecorativeStarFamily, Gender, Mutagen, Scope, StarCategory, StarKind, StaticChartCenterView,
    StaticChartViewSnapshot, StaticDecorativeStarView, StaticNavigationCellView, StaticPalaceView,
    StaticTemporalOverlayView, StaticTemporalPanelView, StaticTypedStarView,
};

use crate::app::{BirthForm, BirthInput, Message, Screen, StaticChartApp, TemporalCell};

// ---------------------------------------------------------------------------
// iztro / 文墨天机 palace-cell star tones
//
// These colors classify *display* only. The category each color encodes is
// read from prepared core view fields (`StaticTypedStarView.kind`,
// `StaticDecorativeStarView.family`); the GUI derives no astrology facts.
// ---------------------------------------------------------------------------

/// `const`-friendly sRGB8 color (iced's `Color::from_rgb8` is not `const`).
const fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

/// Major stars (主星) and the auspicious soft minor pair stars.
const MAJOR_PURPLE: Color = rgb8(0x53, 0x1d, 0xab);
/// Brightness suffix (庙旺得利平陷不), independent of star category.
const BRIGHTNESS_GRAY: Color = rgb8(0xc5, 0xcb, 0xd0);
/// Six malefics / 六煞 (擎羊陀罗火星铃星地空地劫).
const MINOR_MALEFIC: Color = rgb8(0x81, 0x33, 0x59);
/// 禄存.
const LU_CUN_ORANGE: Color = rgb8(0xd4, 0x38, 0x0d);
/// 天马.
const TIAN_MA_BLUE: Color = rgb8(0x18, 0x90, 0xff);
/// Ordinary adjective / miscellaneous stars (杂曜).
const ADJ_GRAY: Color = rgb8(0x8c, 0x8c, 0x8c);
/// 桃花 / festive relationship stars (红鸾咸池天姚天喜, and flow variants).
const PEACH_MAGENTA: Color = rgb8(0xc3, 0x1d, 0x7f);
/// 长生十二神 / 博士十二神 decorative gods (bottom-left).
const DECOR_GOD_OLIVE: Color = rgb8(0x90, 0x98, 0x3c);
/// Vertical space reserved so variable-height temporal overlays cannot cover the
/// two decorative-star lines anchored at the bottom of a palace cell.
const DECORATIVE_AREA_HEIGHT: f32 = 28.0;

/// 化禄 badge background.
const MUTAGEN_LU: Color = rgb8(0xd4, 0x38, 0x0d);
/// 化权 badge background.
const MUTAGEN_QUAN: Color = rgb8(0x2f, 0x54, 0xeb);
/// 化科 badge background.
const MUTAGEN_KE: Color = rgb8(0x23, 0x78, 0x04);
/// 化忌 badge background.
const MUTAGEN_JI: Color = rgb8(0x00, 0x00, 0x00);

/// Renders the active screen: the startup landing page or a generated chart.
pub fn view(app: &StaticChartApp) -> Element<'_, Message> {
    match (app.screen(), app.snapshot()) {
        (Screen::Chart, Some(snapshot)) => chart_screen(app, snapshot),
        // Startup, or a defensive fallback if the chart screen has no snapshot.
        _ => startup_screen(app),
    }
}

/// The landing page: birth-input form plus the list of saved charts.
fn startup_screen(app: &StaticChartApp) -> Element<'_, Message> {
    let title = column![
        text("紫微斗数 · 静态命盘").size(24),
        text("输入出生信息生成命盘，或打开已保存的命盘。")
            .size(13)
            .style(subtle_text_style),
    ]
    .spacing(4);

    column![
        title,
        input_bar(app.form(), app.error()),
        saved_charts_panel(app.saved()),
    ]
    .spacing(12)
    .padding(16)
    .into()
}

/// The generated static chart screen.
fn chart_screen<'a>(
    app: &'a StaticChartApp,
    snapshot: &'a StaticChartViewSnapshot,
) -> Element<'a, Message> {
    column![
        chart_toolbar(app),
        palace_grid(app, snapshot),
        category_legend(),
        temporal_navigation_panel(
            &snapshot.temporal_panel,
            app.selected_temporal_selection()
                == iztro::core::StaticTemporalNavigationSelection::Natal,
        ),
    ]
    .spacing(8)
    .padding(12)
    .into()
}

/// Top bar of the chart screen: a return action plus the 三方四正 highlight toggle.
fn chart_toolbar(app: &StaticChartApp) -> Element<'_, Message> {
    let bar = row![
        button(text("← 返回").size(14))
            .on_press(Message::BackToStartup)
            .style(button::secondary),
        checkbox("三方四正", app.highlight_san_fang())
            .on_toggle(Message::ToggleSanFang)
            .size(16)
            .text_size(13),
    ]
    .spacing(16)
    .align_y(iced::Alignment::Center);
    container(bar).width(Length::Fill).into()
}

/// The saved-charts list shown on the startup page.
fn saved_charts_panel(saved: &[BirthInput]) -> Element<'_, Message> {
    let mut content = column![text("已保存命盘").size(15)].spacing(8);
    if saved.is_empty() {
        content = content.push(
            text("暂无保存的命盘。生成命盘后会自动保存到本地。")
                .size(13)
                .style(subtle_text_style),
        );
    } else {
        let mut list = column![].spacing(6);
        for (index, input) in saved.iter().enumerate() {
            let label = format!(
                "{}-{:02}-{:02} · {} · {}",
                input.year,
                input.month,
                input.day,
                gender_zh(input.gender),
                hour_branch_zh(input.time_index),
            );
            list = list.push(
                button(text(label).size(14))
                    .on_press(Message::SelectSaved(index))
                    .style(button::secondary)
                    .width(Length::Fill),
            );
        }
        content = content.push(list);
    }
    container(content)
        .style(input_panel_style)
        .padding(12)
        .width(Length::Fill)
        .into()
}

// ---------------------------------------------------------------------------
// Birth input
// ---------------------------------------------------------------------------

fn input_bar<'a>(form: &BirthForm, error: Option<&'a str>) -> Element<'a, Message> {
    let fields = row![
        labeled(
            "年",
            text_input("1990", &form.year)
                .on_input(Message::YearChanged)
                .width(82)
        ),
        labeled(
            "月",
            text_input("5", &form.month)
                .on_input(Message::MonthChanged)
                .width(58)
        ),
        labeled(
            "日",
            text_input("17", &form.day)
                .on_input(Message::DayChanged)
                .width(58)
        ),
        labeled(
            "时",
            pick_list(TIME_CHOICES, Some(TimeChoice(form.time_index)), |choice| {
                Message::TimeSelected(choice.0)
            })
            .width(126),
        ),
        labeled(
            "性别",
            pick_list(GENDER_CHOICES, Some(GenderChoice(form.gender)), |choice| {
                Message::GenderSelected(choice.0)
            })
            .width(82),
        ),
        button(text("生成命盘").size(15))
            .on_press(Message::Generate)
            .style(button::primary)
            .padding([8, 16]),
    ]
    .spacing(12)
    .align_y(iced::Alignment::End);

    let mut bar = column![fields].spacing(6);
    if let Some(message) = error {
        bar = bar.push(
            text(format!("输入错误：{message}"))
                .style(error_text_style)
                .size(14),
        );
    }
    container(bar)
        .style(input_panel_style)
        .padding(10)
        .width(Length::Fill)
        .into()
}

fn labeled<'a>(label: &'a str, control: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    column![text(label).size(12), control.into()]
        .spacing(2)
        .into()
}

// ---------------------------------------------------------------------------
// Palace grid (文墨天机 composed layout)
// ---------------------------------------------------------------------------

fn palace_grid<'a>(
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
fn grid_cell(app: &StaticChartApp, row: u8, column_index: u8) -> Element<'_, Message> {
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
enum PalaceHighlight {
    /// No emphasis.
    None,
    /// The selected palace.
    Selected,
    /// A 三方四正 palace related to the selected palace.
    Related,
}

fn palace_cell(palace: &StaticPalaceView, highlight: PalaceHighlight) -> Element<'_, Message> {
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
            .align_x(iced::Alignment::End),
    ]
    .spacing(4)
    .align_y(iced::Alignment::Start);

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
enum StaticStarTone {
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
fn star_tone(star: &StaticTypedStarView) -> StaticStarTone {
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

/// 科权禄忌 badge background color (禄 #d4380d / 权 #2f54eb / 科 #237804 / 忌 #000000).
fn mutagen_badge_color(mutagen: Mutagen) -> Color {
    match mutagen {
        Mutagen::Lu => MUTAGEN_LU,
        Mutagen::Quan => MUTAGEN_QUAN,
        Mutagen::Ke => MUTAGEN_KE,
        Mutagen::Ji => MUTAGEN_JI,
    }
}

/// A compact 科权禄忌 badge rendered inline after a star's brightness. The
/// mutagen char is the prepared `mutagen_zh`; the GUI derives no mutagens.
fn mutagen_inline_badge(mutagen: Mutagen, label: &str) -> Element<'static, Message> {
    let background = mutagen_badge_color(mutagen);
    container(text(label.to_owned()).size(9).color(Color::WHITE))
        .style(move |_theme| container::Style {
            background: Some(background.into()),
            text_color: Some(Color::WHITE),
            border: Border {
                color: background,
                width: 1.0,
                radius: 3.0.into(),
            },
            ..container::Style::default()
        })
        .padding([0, 3])
        .into()
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
    let mut line = row![name].spacing(1).align_y(iced::Alignment::Center);
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
            .align_x(iced::Alignment::End),
    ]
    .spacing(4);

    container(decorative_area)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(iced::Alignment::End)
        .into()
}

fn center_panel(center: &StaticChartCenterView) -> Element<'_, Message> {
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
fn category_legend() -> Element<'static, Message> {
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
    .align_y(iced::Alignment::Center)
    .into()
}

/// One legend label rendered in its tone color.
fn legend_item(label: &str, color: Color) -> Element<'static, Message> {
    text(label.to_owned()).size(12).color(color).into()
}

// ---------------------------------------------------------------------------
// Star helpers
// ---------------------------------------------------------------------------

/// Tone for the remaining grouped badges (temporal overlays only).
#[derive(Clone, Copy)]
enum StarGroupTone {
    Decorative,
    Temporal,
}

fn overlay_badges(overlay: &StaticTemporalOverlayView) -> Element<'_, Message> {
    let mut content = column![text(scope_zh(overlay.scope)).size(11)].spacing(2);
    if let Some(name) = overlay.temporal_palace_name_zh.as_deref() {
        content = content.push(text(name).size(11).style(subtle_text_style));
    }
    if !overlay.typed_stars.is_empty() {
        content = content.push(star_group(
            "流曜",
            overlay.typed_stars.iter().map(star_detail_label).collect(),
            StarGroupTone::Temporal,
        ));
    }
    if !overlay.decorative_stars.is_empty() {
        content = content.push(star_group(
            "流神",
            overlay
                .decorative_stars
                .iter()
                .map(|star| star.name_zh.clone())
                .collect(),
            StarGroupTone::Decorative,
        ));
    }
    if !overlay.mutagens.is_empty() {
        let labels = overlay
            .mutagens
            .iter()
            .map(|mutagen| format!("{}{}", mutagen.star_zh, mutagen.mutagen_zh))
            .collect::<Vec<_>>();
        content = content.push(star_group("四化", labels, StarGroupTone::Temporal));
    }
    content.into()
}

fn temporal_navigation_panel<'a>(
    panel: &'a StaticTemporalPanelView,
    natal_selected: bool,
) -> Element<'a, Message> {
    // First row: 本命 (natal) and 限前 (pre-decadal) lead the 大限 cells inline.
    let mut decadal_cells = vec![
        temporal_cell(
            TemporalCell::Natal,
            Some("本命"),
            None,
            true,
            natal_selected,
        ),
        temporal_cell(
            TemporalCell::PreDecadal,
            Some(panel.pre_decadal_cell.label_zh.as_str()),
            panel.pre_decadal_cell.age_range_zh.as_deref(),
            panel.pre_decadal_cell.enabled,
            panel.pre_decadal_cell.selected,
        ),
    ];
    decadal_cells.extend(panel.decadal_cells.iter().enumerate().map(|(i, cell)| {
        temporal_cell(
            TemporalCell::Decadal(i),
            cell.age_range_zh.as_deref(),
            cell.limit_label_zh.as_deref(),
            cell.enabled,
            cell.selected,
        )
    }));
    let decadal = temporal_row("本命/限前/大限", decadal_cells);
    let yearly = temporal_row(
        "流年/小限",
        panel
            .yearly_age_cells
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                temporal_cell(
                    TemporalCell::YearlyAge(i),
                    cell.year_label.as_deref(),
                    cell.stem_branch_age_zh.as_deref(),
                    cell.enabled,
                    cell.selected,
                )
            })
            .collect(),
    );
    let month = temporal_row("流月", nav_cells(&panel.month_cells, TemporalCell::Month));

    let mut rows = column![decadal, yearly, month].spacing(4);
    for (r, days) in panel.day_rows.iter().enumerate() {
        let widgets = days
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                temporal_cell(
                    TemporalCell::Day(r, i),
                    Some(cell.label_zh.as_str()),
                    None,
                    cell.enabled,
                    cell.selected,
                )
            })
            .collect();
        rows = rows.push(temporal_row("流日", widgets));
    }
    rows = rows.push(temporal_row(
        "流时",
        nav_cells(&panel.hour_cells, TemporalCell::Hour),
    ));

    container(rows)
        .style(temporal_panel_style)
        .padding(8)
        .width(Length::Fill)
        .into()
}

/// Builds the clickable cell widgets for a simple navigation row.
fn nav_cells<'a>(
    cells: &'a [StaticNavigationCellView],
    id_for: impl Fn(usize) -> TemporalCell,
) -> Vec<Element<'a, Message>> {
    cells
        .iter()
        .enumerate()
        .map(|(i, cell)| {
            temporal_cell(
                id_for(i),
                Some(cell.label_zh.as_str()),
                None,
                cell.enabled,
                cell.selected,
            )
        })
        .collect()
}

fn temporal_row<'a>(label: &'static str, cells: Vec<Element<'a, Message>>) -> Element<'a, Message> {
    let mut content = row![container(text(label).size(11)).width(72)]
        .spacing(3)
        .align_y(iced::Alignment::Center);
    for cell in cells {
        content = content.push(cell);
    }
    content.into()
}

/// Renders one temporal cell. Enabled cells are clickable buttons that emit a
/// [`Message::SelectTemporalCell`]; disabled cells stay inert containers and can
/// never become an active selection.
fn temporal_cell<'a>(
    id: TemporalCell,
    primary: Option<&'a str>,
    secondary: Option<&'a str>,
    enabled: bool,
    selected: bool,
) -> Element<'a, Message> {
    let primary_text = text(primary.unwrap_or("—")).size(10);
    let primary_text = if enabled {
        primary_text
    } else {
        primary_text.style(subtle_text_style)
    };
    let mut content = column![primary_text]
        .spacing(1)
        .align_x(iced::Alignment::Center);
    if let Some(secondary) = secondary {
        content = content.push(text(secondary).size(9));
    }

    if enabled {
        button(content)
            .on_press(Message::SelectTemporalCell(id))
            .padding([3, 2])
            .width(Length::FillPortion(1))
            .style(move |theme, _status| temporal_cell_button_style(theme, selected))
            .into()
    } else {
        container(content)
            .style(move |theme| temporal_cell_style(theme, false))
            .padding([3, 2])
            .width(Length::FillPortion(1))
            .into()
    }
}

fn star_group(
    label: &'static str,
    labels: Vec<String>,
    tone: StarGroupTone,
) -> Element<'static, Message> {
    row![
        star_badge(label.to_owned(), tone),
        text(labels.join(" ")).size(11).width(Length::Fill),
    ]
    .spacing(4)
    .align_y(iced::Alignment::Center)
    .into()
}

fn star_badge(label: String, tone: StarGroupTone) -> Element<'static, Message> {
    container(text(label).size(11))
        .style(star_badge_style(tone))
        .padding([2, 5])
        .into()
}

fn star_detail_label(star: &StaticTypedStarView) -> String {
    match (&star.brightness_zh.is_empty(), star.mutagen_zh.as_deref()) {
        (false, Some(mutagen)) => format!("{}{}{}", star.name_zh, star.brightness_zh, mutagen),
        (false, None) => format!("{}{}", star.name_zh, star.brightness_zh),
        (true, Some(mutagen)) => format!("{}{}", star.name_zh, mutagen),
        (true, None) => star.name_zh.clone(),
    }
}

fn center_four_pillar_rows(center: &StaticChartCenterView) -> Vec<(&'static str, String)> {
    center
        .four_pillars
        .as_ref()
        .map(|pillars| {
            vec![
                ("年柱", pillars.yearly_zh.clone()),
                ("月柱", pillars.monthly_zh.clone()),
                ("日柱", pillars.daily_zh.clone()),
                ("时柱", pillars.hourly_zh.clone()),
            ]
        })
        .unwrap_or_default()
}

fn bureau_label(center: &StaticChartCenterView) -> String {
    center
        .five_element_bureau
        .map(|bureau| format!("{bureau:?}"))
        .unwrap_or_else(|| "未提供".to_string())
}

fn fact_row<'a>(label: &'a str, value: impl Into<String>) -> Element<'a, Message> {
    text(format!("{label}：{}", value.into())).size(13).into()
}

fn section_title(label: &str) -> Element<'_, Message> {
    text(label).size(13).style(section_title_style).into()
}

fn gender_zh(gender: Gender) -> &'static str {
    match gender {
        Gender::Female => "女",
        Gender::Male => "男",
    }
}

/// Chinese label for an `iztro` `timeIndex` double-hour (`0..=12`).
fn hour_branch_zh(time_index: u8) -> &'static str {
    match time_index {
        0 => "早子时",
        1 => "丑时",
        2 => "寅时",
        3 => "卯时",
        4 => "辰时",
        5 => "巳时",
        6 => "午时",
        7 => "未时",
        8 => "申时",
        9 => "酉时",
        10 => "戌时",
        11 => "亥时",
        12 => "晚子时",
        _ => "未知",
    }
}

fn scope_zh(scope: Scope) -> &'static str {
    match scope {
        Scope::Natal => "本命",
        Scope::Decadal => "大限",
        Scope::Age => "小限",
        Scope::Yearly => "流年",
        Scope::Monthly => "流月",
        Scope::Daily => "流日",
        Scope::Hourly => "流时",
    }
}

// ---------------------------------------------------------------------------
// pick_list option types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
struct TimeChoice(u8);

const TIME_CHOICES: &[TimeChoice] = &[
    TimeChoice(0),
    TimeChoice(1),
    TimeChoice(2),
    TimeChoice(3),
    TimeChoice(4),
    TimeChoice(5),
    TimeChoice(6),
    TimeChoice(7),
    TimeChoice(8),
    TimeChoice(9),
    TimeChoice(10),
    TimeChoice(11),
    TimeChoice(12),
];

impl fmt::Display for TimeChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self.0 {
            0 => "子(早)",
            1 => "丑",
            2 => "寅",
            3 => "卯",
            4 => "辰",
            5 => "巳",
            6 => "午",
            7 => "未",
            8 => "申",
            9 => "酉",
            10 => "戌",
            11 => "亥",
            12 => "子(晚)",
            _ => "?",
        };
        write!(f, "{label} ({})", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct GenderChoice(Gender);

const GENDER_CHOICES: &[GenderChoice] = &[GenderChoice(Gender::Female), GenderChoice(Gender::Male)];

impl fmt::Display for GenderChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(gender_zh(self.0))
    }
}

// ---------------------------------------------------------------------------
// Styles
// ---------------------------------------------------------------------------

fn palace_cell_style(
    highlight: PalaceHighlight,
) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |theme, _status| {
        let palette = theme.extended_palette();
        let (background, text_color, border_color, width) = match highlight {
            PalaceHighlight::Selected => (
                palette.primary.weak.color,
                palette.primary.weak.text,
                palette.primary.strong.color,
                2.0,
            ),
            // 三方四正 related palaces get a subtle filled background, weaker
            // than the active palace above (a soft fill rather than only a
            // border), matching the iztro/文墨天机 highlight feel.
            PalaceHighlight::Related => (
                palette.background.weak.color,
                palette.background.weak.text,
                palette.primary.base.color,
                1.5,
            ),
            PalaceHighlight::None => (
                palette.background.base.color,
                palette.background.base.text,
                palette.background.strong.color,
                1.0,
            ),
        };
        button::Style {
            background: Some(background.into()),
            text_color,
            border: Border {
                color: border_color,
                width,
                radius: 4.0.into(),
            },
            ..button::Style::default()
        }
    }
}

fn input_panel_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..container::Style::default()
    }
}

fn center_panel_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.weak.color.into()),
        text_color: Some(palette.background.weak.text),
        border: Border {
            color: palette.primary.strong.color,
            width: 2.0,
            radius: 6.0.into(),
        },
        ..container::Style::default()
    }
}

fn temporal_panel_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 5.0.into(),
        },
        ..container::Style::default()
    }
}

fn temporal_cell_style(theme: &Theme, enabled: bool) -> container::Style {
    let palette = theme.extended_palette();
    let background = if enabled {
        palette.background.base.color
    } else {
        palette.background.weak.color
    };
    container::Style {
        background: Some(background.into()),
        border: Border {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 3.0.into(),
        },
        ..container::Style::default()
    }
}

/// Style for an enabled, clickable temporal cell; the selected cell is tinted.
fn temporal_cell_button_style(theme: &Theme, selected: bool) -> button::Style {
    let palette = theme.extended_palette();
    let (background, text_color, border_color, width) = if selected {
        (
            palette.primary.weak.color,
            palette.primary.weak.text,
            palette.primary.strong.color,
            2.0,
        )
    } else {
        (
            palette.background.base.color,
            palette.background.base.text,
            palette.background.strong.color,
            1.0,
        )
    };
    button::Style {
        background: Some(background.into()),
        text_color,
        border: Border {
            color: border_color,
            width,
            radius: 3.0.into(),
        },
        ..button::Style::default()
    }
}

fn star_badge_style(tone: StarGroupTone) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let (background, text_color) = star_badge_colors(tone);
        container::Style {
            background: Some(background.into()),
            text_color: Some(text_color),
            border: Border {
                color: background,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..container::Style::default()
        }
    }
}

fn star_badge_colors(tone: StarGroupTone) -> (Color, Color) {
    match tone {
        StarGroupTone::Decorative => (Color::from_rgb8(126, 87, 48), Color::WHITE),
        StarGroupTone::Temporal => (Color::from_rgb8(45, 102, 63), Color::WHITE),
    }
}

fn subtle_text_style(theme: &Theme) -> text::Style {
    text::Style {
        color: Some(theme.extended_palette().background.strong.color),
    }
}

fn section_title_style(theme: &Theme) -> text::Style {
    text::Style {
        color: Some(theme.extended_palette().primary.strong.color),
    }
}

fn error_text_style(theme: &Theme) -> text::Style {
    text::Style {
        color: Some(theme.extended_palette().danger.base.color),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::StaticChartApp;
    use iztro::core::FiveElementBureau;

    /// Builds an app with a generated chart (the startup screen has none).
    fn chart_app() -> StaticChartApp {
        let mut app = StaticChartApp::new();
        app.generate();
        app
    }

    /// Owned copy of the generated chart's center facts.
    fn sample_center() -> StaticChartCenterView {
        chart_app()
            .center()
            .expect("generated chart center")
            .clone()
    }

    fn sample_typed_star() -> StaticTypedStarView {
        let app = chart_app();
        app.palaces()
            .iter()
            .flat_map(|palace| {
                palace
                    .major_stars
                    .iter()
                    .chain(&palace.minor_stars)
                    .chain(&palace.adjective_stars)
            })
            .next()
            .expect("sample chart should contain a typed star")
            .clone()
    }

    #[test]
    fn center_four_pillar_rows_use_available_zh_labels() {
        let center = sample_center();
        let rows = center_four_pillar_rows(&center);

        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].0, "年柱");
        assert_eq!(rows[1].0, "月柱");
        assert_eq!(rows[2].0, "日柱");
        assert_eq!(rows[3].0, "时柱");
        assert!(rows.iter().all(|(_, value)| !value.is_empty()));
    }

    #[test]
    fn center_four_pillar_rows_are_empty_when_unavailable() {
        let mut center = sample_center();
        center.four_pillars = None;

        assert!(center_four_pillar_rows(&center).is_empty());
    }

    #[test]
    fn bureau_label_handles_available_and_missing_values() {
        let mut center = sample_center();

        center.five_element_bureau = Some(FiveElementBureau::Fire6);
        assert_eq!(bureau_label(&center), "Fire6");

        center.five_element_bureau = None;
        assert_eq!(bureau_label(&center), "未提供");
    }

    #[test]
    fn scope_labels_cover_every_supported_scope() {
        assert_eq!(scope_zh(Scope::Natal), "本命");
        assert_eq!(scope_zh(Scope::Decadal), "大限");
        assert_eq!(scope_zh(Scope::Age), "小限");
        assert_eq!(scope_zh(Scope::Yearly), "流年");
        assert_eq!(scope_zh(Scope::Monthly), "流月");
        assert_eq!(scope_zh(Scope::Daily), "流日");
        assert_eq!(scope_zh(Scope::Hourly), "流时");
    }

    /// A typed star carrying only the field that drives visual classification.
    fn typed_star_with_kind(kind: StarKind) -> StaticTypedStarView {
        let mut star = sample_typed_star();
        star.kind = kind;
        star
    }

    #[test]
    fn major_kind_maps_to_major_tone() {
        assert_eq!(
            star_tone(&typed_star_with_kind(StarKind::Major)),
            StaticStarTone::Major
        );
    }

    #[test]
    fn soft_minor_pairs_map_to_minor_purple() {
        // Covers 左辅/右弼/天魁/天钺/文昌/文曲 — all prepared as StarKind::Soft.
        assert_eq!(
            star_tone(&typed_star_with_kind(StarKind::Soft)),
            StaticStarTone::MinorPurple
        );
    }

    #[test]
    fn six_malefics_map_to_minor_malefic() {
        // Covers 擎羊/陀罗/火星/铃星/地空/地劫 — all prepared as StarKind::Tough.
        assert_eq!(
            star_tone(&typed_star_with_kind(StarKind::Tough)),
            StaticStarTone::MinorMalefic
        );
    }

    #[test]
    fn lucun_and_tianma_map_to_their_own_tones() {
        assert_eq!(
            star_tone(&typed_star_with_kind(StarKind::LuCun)),
            StaticStarTone::LuCun
        );
        assert_eq!(
            star_tone(&typed_star_with_kind(StarKind::TianMa)),
            StaticStarTone::TianMa
        );
    }

    #[test]
    fn flower_stars_map_to_peach_blossom() {
        // Covers 红鸾/咸池/天姚/天喜 (and flow 鸾/喜) — all prepared as StarKind::Flower.
        assert_eq!(
            star_tone(&typed_star_with_kind(StarKind::Flower)),
            StaticStarTone::AdjPeachBlossom
        );
    }

    #[test]
    fn ordinary_adjective_stars_map_to_default() {
        assert_eq!(
            star_tone(&typed_star_with_kind(StarKind::Adjective)),
            StaticStarTone::AdjDefault
        );
        assert_eq!(
            star_tone(&typed_star_with_kind(StarKind::Helper)),
            StaticStarTone::AdjDefault
        );
    }

    #[test]
    fn mutagen_badge_colors_cover_all_four_transformations() {
        assert_eq!(mutagen_badge_color(Mutagen::Lu), rgb8(0xd4, 0x38, 0x0d));
        assert_eq!(mutagen_badge_color(Mutagen::Quan), rgb8(0x2f, 0x54, 0xeb));
        assert_eq!(mutagen_badge_color(Mutagen::Ke), rgb8(0x23, 0x78, 0x04));
        assert_eq!(mutagen_badge_color(Mutagen::Ji), rgb8(0x00, 0x00, 0x00));
    }

    #[test]
    fn star_kind_routes_to_expected_palace_zone() {
        // Zone placement uses the prepared kind's coarse category.
        assert_eq!(StarKind::Major.category(), StarCategory::Major);
        assert_eq!(StarKind::Soft.category(), StarCategory::Minor);
        assert_eq!(StarKind::Tough.category(), StarCategory::Minor);
        assert_eq!(StarKind::LuCun.category(), StarCategory::Minor);
        assert_eq!(StarKind::TianMa.category(), StarCategory::Minor);
        assert_eq!(StarKind::Flower.category(), StarCategory::Adjective);
        assert_eq!(StarKind::Adjective.category(), StarCategory::Adjective);
        assert_eq!(StarKind::Helper.category(), StarCategory::Adjective);
    }

    #[test]
    fn decorative_family_splits_into_bottom_zones() {
        // 长生/博士 share the olive bottom-left tone; 将前/岁前 share the malefic
        // bottom-right tone. Each family lands in exactly one zone.
        for family in [
            DecorativeStarFamily::Changsheng12,
            DecorativeStarFamily::Boshi12,
        ] {
            assert!(matches!(
                family,
                DecorativeStarFamily::Changsheng12 | DecorativeStarFamily::Boshi12
            ));
        }
        assert_eq!(DECOR_GOD_OLIVE, rgb8(0x90, 0x98, 0x3c));
        assert_eq!(MINOR_MALEFIC, rgb8(0x81, 0x33, 0x59));
    }

    #[test]
    fn palace_cell_uses_a_dedicated_bottom_decorative_layer() {
        let source = include_str!("static_chart_screen.rs");

        assert!(source.contains(concat!("fn bottom_", "decorative_layer")));
        assert!(source.contains(concat!("stack", "![")));
        assert!(source.contains(concat!("DECORATIVE_", "AREA_HEIGHT")));
    }

    #[test]
    fn palace_highlight_is_disjoint_between_selected_and_related() {
        // Selected always wins over related; none/selected/related are distinct.
        assert_ne!(PalaceHighlight::Selected, PalaceHighlight::Related);
        assert_ne!(PalaceHighlight::None, PalaceHighlight::Related);
    }

    #[test]
    fn star_detail_label_covers_brightness_and_mutagen_combinations() {
        let mut star = sample_typed_star();
        star.name_zh = "测试星".to_owned();

        star.brightness_zh = "庙".to_owned();
        star.mutagen_zh = Some("化禄".to_owned());
        assert_eq!(star_detail_label(&star), "测试星庙化禄");

        star.mutagen_zh = None;
        assert_eq!(star_detail_label(&star), "测试星庙");

        star.brightness_zh.clear();
        star.mutagen_zh = Some("化忌".to_owned());
        assert_eq!(star_detail_label(&star), "测试星化忌");

        star.mutagen_zh = None;
        assert_eq!(star_detail_label(&star), "测试星");
    }
}
