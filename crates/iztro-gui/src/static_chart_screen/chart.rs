use iced::widget::{button, column, container, stack, text};
use iced::{Element, Length};
use iztro::core::StaticChartViewSnapshot;

use crate::app::{Message, StaticChartApp};

use super::lines::san_fang_overlay;
use super::palace::palace_grid;

/// The generated static chart screen: a slim toolbar above the palace grid, with
/// a transparent 三方四正 line overlay stacked over the grid.
pub(super) fn chart_screen<'a>(
    app: &'a StaticChartApp,
    snapshot: &'a StaticChartViewSnapshot,
) -> Element<'a, Message> {
    let grid = stack![palace_grid(app, snapshot), san_fang_overlay(app)]
        .width(Length::Fill)
        .height(Length::Fill);

    column![chart_toolbar(app), grid]
        .spacing(8)
        .padding(12)
        .into()
}

/// Top bar of the chart screen: just a return action. 三方四正 is always shown as
/// connecting lines, matching the original iztro chart, so there is no toggle.
pub(super) fn chart_toolbar(_app: &StaticChartApp) -> Element<'_, Message> {
    container(
        button(text("← 返回").size(14))
            .on_press(Message::BackToStartup)
            .style(button::secondary),
    )
    .width(Length::Fill)
    .into()
}
