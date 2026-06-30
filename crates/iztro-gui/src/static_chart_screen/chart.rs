use iced::widget::{button, column, container, row, scrollable, stack, text};
use iced::{Alignment, Element, Length, Padding};
use iztro::core::StaticChartViewSnapshot;
use iztro_i18n::I18n;

use crate::app::{Message, StaticChartApp};

use super::inspector::right_inspector;
use super::lines::san_fang_overlay;
use super::palace::palace_grid;
use super::style::{chart_surface_style, header_bar_style};
use super::theme::{CHART_LAYOUT, SPACING, TYPE};

/// Minimum width of a single palace cell that keeps the original-iztro-style
/// star, 大限/小限, and 流 badge text legible instead of collapsing into dashes.
pub(super) const MIN_PALACE_CELL_WIDTH: f32 = CHART_LAYOUT.palace_cell_width;
/// Minimum height of a single palace cell that keeps its stacked text rows
/// (stars, limits, decorative gods, footer) readable.
pub(super) const MIN_PALACE_CELL_HEIGHT: f32 = CHART_LAYOUT.palace_cell_height;
/// Minimum width of the whole 4x4 chart canvas: four palace columns wide.
pub(super) const MIN_CHART_WIDTH: f32 = MIN_PALACE_CELL_WIDTH * CHART_LAYOUT.grid_columns;
/// Minimum height of the whole 4x4 chart canvas: four palace rows tall.
pub(super) const MIN_CHART_HEIGHT: f32 = MIN_PALACE_CELL_HEIGHT * CHART_LAYOUT.grid_rows;
/// Gutter reserved on the chart canvas's right and bottom edges so the
/// scrollable's floating scrollbars sit over padding rather than over the
/// rightmost palace column / bottom row.
const SCROLLBAR_GUTTER: f32 = CHART_LAYOUT.scrollbar_gutter;

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

    // Inset the fixed canvas by a gutter on the right and bottom so the
    // scrollable's floating scrollbars overlay padding, not palace content.
    let padded = container(grid).padding(Padding {
        top: 0.0,
        right: SCROLLBAR_GUTTER,
        bottom: SCROLLBAR_GUTTER,
        left: 0.0,
    });

    let chart_area = scrollable(padded)
        .direction(scrollable::Direction::Both {
            vertical: scrollable::Scrollbar::new(),
            horizontal: scrollable::Scrollbar::new(),
        })
        .width(Length::Fill)
        .height(Length::Fill);

    // The fixed chart canvas sits on its own warm surface card, so the grid
    // reads as a single scholarly reading surface rather than bare widgets on
    // the app background.
    let chart_card = container(chart_area)
        .style(chart_surface_style)
        .padding(SPACING.lg)
        .width(Length::Fill)
        .height(Length::Fill);

    // The inspector lives beside the chart canvas, never inside it: the canvas
    // keeps its fixed minimum size and scrolls, while the side panel takes a
    // fixed-width slot to its right (or is absent when hidden).
    // The row must fill the available width: a default `Shrink` row would
    // collapse the `Fill` chart canvas to nothing and let the fixed-width panel
    // overlap where the chart should be.
    let mut body = row![chart_card]
        .spacing(SPACING.xl)
        .width(Length::Fill)
        .height(Length::Fill);
    if let Some(inspector) = right_inspector(app, i18n) {
        body = body.push(inspector);
    }

    column![chart_toolbar(i18n), body]
        .spacing(SPACING.xl)
        .padding(SPACING.xl)
        .into()
}

/// The application header above the chart: a title on the left, a return action,
/// and the right-panel toggle on the right, sitting on a card surface so it
/// reads as part of a real application rather than a debug toolbar. 三方四正 is
/// always shown as connecting lines, matching the original iztro chart, so there
/// is no toggle for it.
pub(super) fn chart_toolbar<'a>(i18n: &I18n) -> Element<'a, Message> {
    let bar = row![
        button(text(i18n.text("button-back")).size(TYPE.label))
            .on_press(Message::BackToStartup)
            .style(button::secondary),
        text(i18n.text("startup-title"))
            .size(TYPE.heading)
            .style(super::style::section_title_style),
        iced::widget::horizontal_space(),
        button(text(i18n.text("right-panel-toggle")).size(TYPE.label))
            .on_press(Message::ToggleRightPanel)
            .style(button::secondary),
    ]
    .spacing(SPACING.xl)
    .align_y(Alignment::Center);

    container(bar)
        .style(header_bar_style)
        .padding(Padding {
            top: SPACING.lg,
            right: SPACING.xl,
            bottom: SPACING.lg,
            left: SPACING.xl,
        })
        .width(Length::Fill)
        .into()
}
