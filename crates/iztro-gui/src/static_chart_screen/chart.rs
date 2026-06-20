use iced::widget::{button, column, container, scrollable, stack, text};
use iced::{Element, Length};
use iztro::core::StaticChartViewSnapshot;
use iztro_i18n::I18n;

use crate::app::{Message, StaticChartApp};

use super::lines::san_fang_overlay;
use super::palace::palace_grid;

/// Minimum width of a single palace cell that keeps the original-iztro-style
/// star, 大限/小限, and 流 badge text legible instead of collapsing into dashes.
pub(super) const MIN_PALACE_CELL_WIDTH: f32 = 275.0;
/// Minimum height of a single palace cell that keeps its stacked text rows
/// (stars, limits, decorative gods, footer) readable.
pub(super) const MIN_PALACE_CELL_HEIGHT: f32 = 190.0;
/// Minimum width of the whole 4x4 chart canvas: four palace columns wide.
pub(super) const MIN_CHART_WIDTH: f32 = MIN_PALACE_CELL_WIDTH * 4.0;
/// Minimum height of the whole 4x4 chart canvas: four palace rows tall.
pub(super) const MIN_CHART_HEIGHT: f32 = MIN_PALACE_CELL_HEIGHT * 4.0;

/// The generated static chart screen: a slim toolbar above the palace grid, with
/// a transparent 三方四正 line overlay stacked over the grid.
///
/// The grid + overlay stack is pinned to a fixed minimum size so a small window
/// scrolls instead of squeezing palace/center text into unreadable dash lines.
/// Both stack children share that fixed size, so the canvas overlay stays
/// aligned with the palace grid.
pub(super) fn chart_screen<'a>(
    app: &'a StaticChartApp,
    snapshot: &'a StaticChartViewSnapshot,
    i18n: &I18n,
) -> Element<'a, Message> {
    let grid = stack![palace_grid(app, snapshot, i18n), san_fang_overlay(app)]
        .width(Length::Fixed(MIN_CHART_WIDTH))
        .height(Length::Fixed(MIN_CHART_HEIGHT));

    let chart_area = scrollable(grid)
        .direction(scrollable::Direction::Both {
            vertical: scrollable::Scrollbar::new(),
            horizontal: scrollable::Scrollbar::new(),
        })
        .width(Length::Fill)
        .height(Length::Fill);

    column![chart_toolbar(i18n), chart_area]
        .spacing(8)
        .padding(12)
        .into()
}

/// Top bar of the chart screen: just a return action. 三方四正 is always shown as
/// connecting lines, matching the original iztro chart, so there is no toggle.
pub(super) fn chart_toolbar<'a>(i18n: &I18n) -> Element<'a, Message> {
    container(
        button(text(i18n.text("button-back")).size(14))
            .on_press(Message::BackToStartup)
            .style(button::secondary),
    )
    .width(Length::Fill)
    .into()
}
