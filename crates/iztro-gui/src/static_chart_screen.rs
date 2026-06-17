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
use iced::{Border, Element, Length, Theme};
use iztro::core::{
    Gender, StaticChartCenterView, StaticDecorativeStarView, StaticPalaceView, StaticTypedStarView,
};

use crate::app::{BirthForm, BirthInput, Message, StaticChartApp};

/// Renders the full static chart screen.
pub fn view(app: &StaticChartApp) -> Element<'_, Message> {
    column![
        input_bar(app.form(), app.error()),
        history_bar(app.history()),
        palace_grid(app),
        detail_panel(app.selected_palace()),
    ]
    .spacing(10)
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
                .width(70)
        ),
        labeled(
            "月",
            text_input("5", &form.month)
                .on_input(Message::MonthChanged)
                .width(50)
        ),
        labeled(
            "日",
            text_input("17", &form.day)
                .on_input(Message::DayChanged)
                .width(50)
        ),
        labeled(
            "时",
            pick_list(TIME_CHOICES, Some(TimeChoice(form.time_index)), |choice| {
                Message::TimeSelected(choice.0)
            })
            .width(110),
        ),
        labeled(
            "性别",
            pick_list(GENDER_CHOICES, Some(GenderChoice(form.gender)), |choice| {
                Message::GenderSelected(choice.0)
            })
            .width(70),
        ),
        button(text("生成命盘")).on_press(Message::Generate),
    ]
    .spacing(10)
    .align_y(iced::Alignment::End);

    let mut bar = column![fields].spacing(6);
    if let Some(message) = error {
        bar = bar.push(
            text(format!("⚠ {message}"))
                .style(error_text_style)
                .size(14),
        );
    }
    container(bar).into()
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
    let header = text(format!(
        "{} {}{}",
        palace.name_zh, palace.stem_zh, palace.branch_zh
    ))
    .size(15);

    let mut content = column![header].spacing(2);
    content = push_typed(content, "主", &palace.major_stars);
    content = push_typed(content, "辅", &palace.minor_stars);
    content = push_typed(content, "杂", &palace.adjective_stars);
    content = push_decorative(content, "神", &palace.decorative_stars);

    button(content)
        .on_press(Message::SelectPalace(palace.branch))
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .padding(6)
        .style(palace_cell_style(selected))
        .into()
}

fn center_panel(center: &StaticChartCenterView) -> Element<'_, Message> {
    let mut content = column![
        text("命盘").size(20),
        text(format!(
            "生年 {}{}",
            center.birth_year_stem_zh, center.birth_year_branch_zh
        )),
        text(format!("性别 {}", gender_zh(center.gender))),
    ]
    .spacing(6);

    if let Some(bureau) = center.five_element_bureau {
        content = content.push(text(format!("五行局 {bureau:?}")));
    }
    if let Some(life_zh) = center.life_palace_branch_zh.as_deref() {
        content = content.push(text(format!("命宫 {life_zh}")));
    }
    if let Some(body_zh) = center.body_palace_branch_zh.as_deref() {
        content = content.push(text(format!("身宫 {body_zh}")));
    }

    content.into()
}

fn detail_panel(selected: Option<&StaticPalaceView>) -> Element<'_, Message> {
    let content = match selected {
        None => column![text("点击任一宫位查看详情").size(14)],
        Some(palace) => column![
            text(format!(
                "{} · {}{}",
                palace.name_zh, palace.stem_zh, palace.branch_zh
            ))
            .size(18),
            text(format!("主星: {}", join_typed(&palace.major_stars))),
            text(format!("辅星: {}", join_typed(&palace.minor_stars))),
            text(format!("杂曜: {}", join_typed(&palace.adjective_stars))),
            text(format!(
                "神煞: {}",
                join_decorative(&palace.decorative_stars)
            )),
        ]
        .spacing(3),
    };
    container(content)
        .style(detail_panel_style)
        .width(Length::Fill)
        .padding(10)
        .into()
}

// ---------------------------------------------------------------------------
// Star helpers
// ---------------------------------------------------------------------------

fn push_typed<'a>(
    content: Column<'a, Message>,
    label: &str,
    stars: &[StaticTypedStarView],
) -> Column<'a, Message> {
    if stars.is_empty() {
        content
    } else {
        content.push(text(format!("{label}: {}", join_typed(stars))).size(12))
    }
}

fn push_decorative<'a>(
    content: Column<'a, Message>,
    label: &str,
    stars: &[StaticDecorativeStarView],
) -> Column<'a, Message> {
    if stars.is_empty() {
        content
    } else {
        content.push(text(format!("{label}: {}", join_decorative(stars))).size(12))
    }
}

fn join_typed(stars: &[StaticTypedStarView]) -> String {
    stars
        .iter()
        .map(|star| star.name_zh.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

fn join_decorative(stars: &[StaticDecorativeStarView]) -> String {
    stars
        .iter()
        .map(|star| star.name_zh.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

fn gender_zh(gender: Gender) -> &'static str {
    match gender {
        Gender::Female => "女",
        Gender::Male => "男",
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

fn detail_panel_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..container::Style::default()
    }
}

fn error_text_style(theme: &Theme) -> text::Style {
    text::Style {
        color: Some(theme.extended_palette().danger.base.color),
    }
}
