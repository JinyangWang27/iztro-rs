//! Iced rendering of one [`StaticChartViewSnapshot`] as a 4x4 palace grid.
//!
//! This module only reads the snapshot view models exposed by [`StaticChartApp`]
//! and lays them out; it performs no astrology placement, rule evaluation, or
//! 成格 detection. The center four grid cells hold the center panel; the twelve
//! perimeter cells are placed by their fixed `grid_position`.
//!
//! [`StaticChartViewSnapshot`]: iztro::core::StaticChartViewSnapshot

use iced::widget::{Column, Row, Space, button, column, container, text};
use iced::{Element, Length};
use iztro::core::{
    EarthlyBranch, StaticChartCenterView, StaticDecorativeStarView, StaticPalaceView,
    StaticTypedStarView,
};

use crate::app::{GRID_SIZE, Message, StaticChartApp};

/// Renders the full static chart screen: the palace grid plus a detail panel
/// for the currently selected palace.
pub fn view(app: &StaticChartApp) -> Element<'_, Message> {
    column![palace_grid(app), detail_panel(app.selected_palace())]
        .spacing(12)
        .padding(12)
        .into()
}

/// Builds the 4x4 grid, placing palaces by `grid_position` and the center panel
/// in the middle.
fn palace_grid(app: &StaticChartApp) -> Element<'_, Message> {
    let mut grid = Column::new().spacing(4);
    for row in 0..GRID_SIZE {
        let mut grid_row = Row::new().spacing(4);
        for column_index in 0..GRID_SIZE {
            let cell: Element<'_, Message> = if let Some(palace) = app.palace_at(row, column_index)
            {
                palace_cell(palace, app.selected_branch())
            } else if (row, column_index) == (1, 1) {
                center_panel(app.center())
            } else {
                // Remaining center cells are visually absorbed by the panel.
                Space::new(Length::Fill, Length::Fill).into()
            };
            grid_row = grid_row.push(
                container(cell)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(2),
            );
        }
        grid = grid.push(grid_row);
    }
    container(grid)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Renders one palace cell as a clickable button.
fn palace_cell(palace: &StaticPalaceView, selected: Option<EarthlyBranch>) -> Element<'_, Message> {
    let marker = if selected == Some(palace.branch) {
        "▶ "
    } else {
        ""
    };
    let header = text(format!(
        "{marker}{} {}{}",
        palace.name_zh, palace.stem_zh, palace.branch_zh
    ))
    .size(16);

    let mut content = column![header].spacing(2);
    content = push_typed_stars(content, "主", &palace.major_stars);
    content = push_typed_stars(content, "辅", &palace.minor_stars);
    content = push_typed_stars(content, "杂", &palace.adjective_stars);
    content = push_decorative_stars(content, "神", &palace.decorative_stars);

    button(content)
        .on_press(Message::SelectPalace(palace.branch))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Renders the center panel facts in the middle of the grid.
fn center_panel(center: &StaticChartCenterView) -> Element<'_, Message> {
    let mut content = column![
        text("命盘").size(18),
        text(format!(
            "{}{}",
            center.birth_year_stem_zh, center.birth_year_branch_zh
        )),
    ]
    .spacing(4);

    if let Some(bureau_zh) = center.life_palace_branch_zh.as_deref() {
        content = content.push(text(format!("命宫: {bureau_zh}")));
    }
    if let Some(body_zh) = center.body_palace_branch_zh.as_deref() {
        content = content.push(text(format!("身宫: {body_zh}")));
    }
    if let Some(bureau) = center.five_element_bureau {
        content = content.push(text(format!("{bureau:?}")));
    }

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(4)
        .into()
}

/// Renders the detail panel for the selected palace, or a hint when none is
/// selected.
fn detail_panel(selected: Option<&StaticPalaceView>) -> Element<'_, Message> {
    let content = match selected {
        None => column![text("点击任一宫位查看详情")],
        Some(palace) => {
            let mut detail = column![
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
            .spacing(2);
            for role in &palace.roles {
                detail = detail.push(text(format!("{role:?}")));
            }
            detail
        }
    };
    container(content).width(Length::Fill).padding(8).into()
}

fn push_typed_stars<'a>(
    content: Column<'a, Message>,
    label: &str,
    stars: &[StaticTypedStarView],
) -> Column<'a, Message> {
    if stars.is_empty() {
        content
    } else {
        content.push(text(format!("{label}: {}", join_typed(stars))).size(13))
    }
}

fn push_decorative_stars<'a>(
    content: Column<'a, Message>,
    label: &str,
    stars: &[StaticDecorativeStarView],
) -> Column<'a, Message> {
    if stars.is_empty() {
        content
    } else {
        content.push(text(format!("{label}: {}", join_decorative(stars))).size(13))
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
