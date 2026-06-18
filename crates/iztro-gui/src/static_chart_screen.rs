//! Iced rendering of one [`StaticChartViewSnapshot`] in a 文墨天机-style layout.
//!
//! The screen is a composed grid — a top row of four palaces, a middle band with
//! a left palace column, a center panel spanning the middle 2x2, and a right
//! palace column, then a bottom row of four palaces — placed by each palace's
//! fixed `grid_position`. It also renders a solar birth-input bar and a small
//! generation history. This module only reads snapshot view models; it performs
//! no astrology placement, rule evaluation, or 成格 detection.
//!
//! [`StaticChartViewSnapshot`]: iztro::core::StaticChartViewSnapshot

use std::fmt;

use iced::widget::{Column, button, column, container, pick_list, row, text, text_input};
use iced::{Border, Color, Element, Length, Theme};
use iztro::core::{
    Gender, Scope, StaticChartCenterView, StaticDecadalCellView, StaticDecorativeStarView,
    StaticNavigationCellView, StaticPalaceView, StaticTemporalOverlayView, StaticTemporalPanelView,
    StaticTypedStarView, StaticYearlyAgeCellView,
};

use crate::app::{BirthForm, BirthInput, Message, StaticChartApp};

/// Renders the full static chart screen.
pub fn view(app: &StaticChartApp) -> Element<'_, Message> {
    column![
        input_bar(app.form(), app.error()),
        history_bar(app.history()),
        palace_grid(app),
        category_legend(),
        temporal_navigation_panel(&app.snapshot().temporal_panel),
    ]
    .spacing(8)
    .padding(12)
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

fn history_bar(history: &[BirthInput]) -> Element<'_, Message> {
    let mut bar = row![text("历史:").size(13)]
        .spacing(8)
        .align_y(iced::Alignment::Center);
    for (index, input) in history.iter().enumerate() {
        let label = format!("{}-{}-{}", input.year, input.month, input.day);
        bar = bar.push(
            button(text(label).size(13))
                .on_press(Message::SelectHistory(index))
                .style(button::secondary),
        );
    }
    container(bar).into()
}

// ---------------------------------------------------------------------------
// Palace grid (文墨天机 composed layout)
// ---------------------------------------------------------------------------

fn palace_grid(app: &StaticChartApp) -> Element<'_, Message> {
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
    let center = container(center_panel(app.center()))
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
        Some(palace) => palace_cell(palace, app.selected_branch() == Some(palace.branch)),
        None => container(text("")).width(Length::FillPortion(1)).into(),
    }
}

fn palace_cell(palace: &StaticPalaceView, selected: bool) -> Element<'_, Message> {
    let header = column![
        text(palace.name_zh.as_str()).size(16),
        text(format!("{}{}", palace.stem_zh, palace.branch_zh)).size(12),
    ]
    .spacing(1);

    let mut content = column![header].spacing(4);
    content = push_typed_badges(content, "主星", &palace.major_stars, StarGroupTone::Major);
    content = push_typed_badges(content, "辅星", &palace.minor_stars, StarGroupTone::Minor);
    content = push_typed_badges(
        content,
        "杂曜",
        &palace.adjective_stars,
        StarGroupTone::Adjective,
    );
    content = push_typed_badges(
        content,
        "其他",
        &palace.other_typed_stars,
        StarGroupTone::Other,
    );
    content = push_decorative_badges(content, "神煞", &palace.decorative_stars);
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

    button(content)
        .on_press(Message::SelectPalace(palace.branch))
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .padding(6)
        .style(palace_cell_style(selected))
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

fn category_legend() -> Element<'static, Message> {
    row![
        text("图例").size(12).style(subtle_text_style),
        star_badge("主星".to_owned(), StarGroupTone::Major),
        star_badge("辅星".to_owned(), StarGroupTone::Minor),
        star_badge("杂曜".to_owned(), StarGroupTone::Adjective),
        star_badge("神煞".to_owned(), StarGroupTone::Decorative),
        star_badge("流曜".to_owned(), StarGroupTone::Temporal),
    ]
    .spacing(6)
    .align_y(iced::Alignment::Center)
    .into()
}

// ---------------------------------------------------------------------------
// Star helpers
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum StarGroupTone {
    Major,
    Minor,
    Adjective,
    Other,
    Decorative,
    Temporal,
}

fn push_typed_badges<'a>(
    content: Column<'a, Message>,
    label: &'static str,
    stars: &[StaticTypedStarView],
    tone: StarGroupTone,
) -> Column<'a, Message> {
    if stars.is_empty() {
        content
    } else {
        content.push(star_group(
            label,
            stars.iter().map(|star| star.name_zh.clone()).collect(),
            tone,
        ))
    }
}

fn push_decorative_badges<'a>(
    content: Column<'a, Message>,
    label: &'static str,
    stars: &[StaticDecorativeStarView],
) -> Column<'a, Message> {
    if stars.is_empty() {
        content
    } else {
        content.push(star_group(
            label,
            stars.iter().map(|star| star.name_zh.clone()).collect(),
            StarGroupTone::Decorative,
        ))
    }
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

fn temporal_navigation_panel(panel: &StaticTemporalPanelView) -> Element<'_, Message> {
    let mut rows = column![
        decadal_row("大限", &panel.decadal_cells),
        yearly_age_row("流年/小限", &panel.yearly_age_cells),
        navigation_row("流月", &panel.month_cells),
    ]
    .spacing(4);

    for days in &panel.day_rows {
        rows = rows.push(navigation_row("流日", days));
    }
    rows = rows.push(navigation_row("流时", &panel.hour_cells));

    container(rows)
        .style(temporal_panel_style)
        .padding(8)
        .width(Length::Fill)
        .into()
}

fn decadal_row<'a>(
    label: &'static str,
    cells: &'a [StaticDecadalCellView],
) -> Element<'a, Message> {
    temporal_row(label, cells.iter().map(decadal_cell).collect())
}

fn yearly_age_row<'a>(
    label: &'static str,
    cells: &'a [StaticYearlyAgeCellView],
) -> Element<'a, Message> {
    temporal_row(label, cells.iter().map(yearly_age_cell).collect())
}

fn navigation_row<'a>(
    label: &'static str,
    cells: &'a [StaticNavigationCellView],
) -> Element<'a, Message> {
    temporal_row(label, cells.iter().map(navigation_cell).collect())
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

fn decadal_cell(cell: &StaticDecadalCellView) -> Element<'_, Message> {
    temporal_cell(
        cell.age_range_zh.as_deref(),
        cell.limit_label_zh.as_deref(),
        cell.enabled,
    )
}

fn yearly_age_cell(cell: &StaticYearlyAgeCellView) -> Element<'_, Message> {
    temporal_cell(
        cell.year_label.as_deref(),
        cell.stem_branch_age_zh.as_deref(),
        cell.enabled,
    )
}

fn navigation_cell(cell: &StaticNavigationCellView) -> Element<'_, Message> {
    temporal_cell(Some(cell.label_zh.as_str()), None, cell.enabled)
}

fn temporal_cell<'a>(
    primary: Option<&'a str>,
    secondary: Option<&'a str>,
    enabled: bool,
) -> Element<'a, Message> {
    let primary = text(primary.unwrap_or("—")).size(10);
    let primary = if enabled {
        primary
    } else {
        primary.style(subtle_text_style)
    };
    let mut content = column![primary].spacing(1).align_x(iced::Alignment::Center);
    if let Some(secondary) = secondary {
        content = content.push(text(secondary).size(9));
    }

    container(content)
        .style(move |theme| temporal_cell_style(theme, enabled))
        .padding([3, 2])
        .width(Length::FillPortion(1))
        .into()
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

fn palace_cell_style(selected: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |theme, _status| {
        let palette = theme.extended_palette();
        let (background, text_color, border_color) = if selected {
            (
                palette.primary.weak.color,
                palette.primary.weak.text,
                palette.primary.strong.color,
            )
        } else {
            (
                palette.background.base.color,
                palette.background.base.text,
                palette.background.strong.color,
            )
        };
        button::Style {
            background: Some(background.into()),
            text_color,
            border: Border {
                color: border_color,
                width: if selected { 2.0 } else { 1.0 },
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
        StarGroupTone::Major => (Color::from_rgb8(111, 53, 25), Color::WHITE),
        StarGroupTone::Minor => (Color::from_rgb8(38, 88, 96), Color::WHITE),
        StarGroupTone::Adjective => (Color::from_rgb8(92, 75, 132), Color::WHITE),
        StarGroupTone::Other => (Color::from_rgb8(91, 91, 91), Color::WHITE),
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

    fn sample_typed_star() -> StaticTypedStarView {
        let app = StaticChartApp::new();
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
        let app = StaticChartApp::new();
        let rows = center_four_pillar_rows(app.center());

        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].0, "年柱");
        assert_eq!(rows[1].0, "月柱");
        assert_eq!(rows[2].0, "日柱");
        assert_eq!(rows[3].0, "时柱");
        assert!(rows.iter().all(|(_, value)| !value.is_empty()));
    }

    #[test]
    fn center_four_pillar_rows_are_empty_when_unavailable() {
        let app = StaticChartApp::new();
        let mut center = app.center().clone();
        center.four_pillars = None;

        assert!(center_four_pillar_rows(&center).is_empty());
    }

    #[test]
    fn bureau_label_handles_available_and_missing_values() {
        let app = StaticChartApp::new();
        let mut center = app.center().clone();

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

    #[test]
    fn full_static_chart_view_builds_from_prepared_snapshot() {
        let app = StaticChartApp::new();

        let _: Element<'_, Message> = view(&app);
    }

    #[test]
    fn full_static_chart_view_builds_with_visible_input_error() {
        let mut app = StaticChartApp::new();
        app.update(Message::YearChanged("not-a-year".to_owned()));
        app.update(Message::Generate);
        assert!(app.error().is_some());

        let _: Element<'_, Message> = view(&app);
    }
}
