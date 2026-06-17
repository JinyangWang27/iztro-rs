//! Local Iced desktop GUI prototype for rendering one `iztro` static chart
//! snapshot. The crate consumes [`StaticChartViewSnapshot`] read models and
//! renders them; it implements no astrology placement, rules, or 成格 logic.
//!
//! [`StaticChartViewSnapshot`]: iztro::core::StaticChartViewSnapshot

pub mod app;
pub mod static_chart_screen;

use app::{Message, StaticChartApp};

/// Launches the local Iced desktop window rendering the sample static chart.
pub fn run() -> iced::Result {
    iced::application("iztro · static chart", update, static_chart_screen::view)
        .run_with(|| (StaticChartApp::new(), iced::Task::none()))
}

/// Bridges the pure [`StaticChartApp::update`] into the Iced update loop.
fn update(app: &mut StaticChartApp, message: Message) -> iced::Task<Message> {
    app.update(message);
    iced::Task::none()
}
