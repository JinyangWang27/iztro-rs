use iced::widget::{button, checkbox, column, container, row, text};
use iced::{Alignment, Element, Length};
use iztro::core::{StaticChartViewSnapshot, StaticTemporalNavigationSelection};

use crate::app::{Message, StaticChartApp};

use super::palace::{category_legend, palace_grid};
use super::temporal::temporal_navigation_panel;

/// The generated static chart screen.
pub(super) fn chart_screen<'a>(
    app: &'a StaticChartApp,
    snapshot: &'a StaticChartViewSnapshot,
) -> Element<'a, Message> {
    column![
        chart_toolbar(app),
        palace_grid(app, snapshot),
        category_legend(),
        temporal_navigation_panel(
            &snapshot.temporal_panel,
            app.selected_temporal_selection() == StaticTemporalNavigationSelection::Natal,
        ),
    ]
    .spacing(8)
    .padding(12)
    .into()
}

/// Top bar of the chart screen: a return action plus the 三方四正 highlight toggle.
pub(super) fn chart_toolbar(app: &StaticChartApp) -> Element<'_, Message> {
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
    .align_y(Alignment::Center);
    container(bar).width(Length::Fill).into()
}
