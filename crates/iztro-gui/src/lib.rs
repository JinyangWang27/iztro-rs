//! Local Iced desktop GUI prototype for rendering one `iztro` static chart
//! snapshot. The crate consumes [`StaticChartViewSnapshot`] read models and
//! renders them; it implements no astrology placement, rules, or 成格 logic.
//!
//! [`StaticChartViewSnapshot`]: iztro::core::StaticChartViewSnapshot

pub mod app;
pub mod fonts;
pub mod persistence;
pub mod static_chart_screen;

use app::{Message, StaticChartApp};
use persistence::ChartStore;

const WINDOW_TITLE: &str = "iztro Static Chart";

/// Launches the local Iced desktop window rendering the static chart.
pub fn run() -> iced::Result {
    iced::application(WINDOW_TITLE, update, static_chart_screen::view)
        .font(fonts::CJK_FONT_BYTES)
        .default_font(fonts::CJK_FONT)
        .window(iced::window::Settings {
            size: iced::Size::new(980.0, 840.0),
            min_size: Some(iced::Size::new(760.0, 680.0)),
            ..Default::default()
        })
        .run_with(|| {
            (
                StaticChartApp::with_optional_store(ChartStore::default_store()),
                iced::Task::none(),
            )
        })
}

/// Bridges the pure [`StaticChartApp::update`] into the Iced update loop.
fn update(app: &mut StaticChartApp, message: Message) -> iced::Task<Message> {
    app.update(message);
    iced::Task::none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_title_uses_only_ascii_window_chrome_text() {
        assert!(WINDOW_TITLE.is_ascii());
    }

    #[test]
    fn gui_manifest_uses_tiny_skia_without_wgpu_defaults() {
        let manifest = include_str!("../Cargo.toml");

        assert!(
            manifest.contains(
                r#"iced = { version = "0.13", default-features = false, features = ["tiny-skia"] }"#
            ),
            "GUI must avoid Iced's wgpu renderer because wgpu 0.19 can panic when WSLg \
             recreates a Wayland surface during resize"
        );
    }
}
