//! Local Iced desktop GUI prototype for rendering one `iztro` static chart
//! snapshot. The crate consumes [`StaticChartViewSnapshot`] read models and
//! renders them; it implements no astrology placement, rules, or 成格 logic.
//!
//! [`StaticChartViewSnapshot`]: iztro::core::StaticChartViewSnapshot

pub mod analysis;
pub mod app;
pub mod fonts;
pub mod persistence;
pub mod settings;
pub mod static_chart_screen;
mod system_clock;

use app::{Message, StaticChartApp};
use persistence::ChartStore;
use settings::SettingsStore;

const WINDOW_TITLE: &str = "iztro-rs";

/// Launches the local Iced desktop window rendering the static chart.
pub fn run() -> iced::Result {
    iced::application(WINDOW_TITLE, update, static_chart_screen::view)
        .font(fonts::CJK_FONT_BYTES)
        .default_font(fonts::CJK_FONT)
        .window(iced::window::Settings {
            // Default sized to fit the full fixed chart canvas (MIN_CHART_WIDTH x
            // MIN_CHART_HEIGHT) plus toolbar, padding, and the right inspector at
            // its expanded width — snugly, so the `Fill` chart area does not leave
            // a wide gap between the chart and the panel at the default size.
            size: iced::Size::new(1360.0, 900.0),
            // The window may shrink well below the chart's preferred layout so it
            // fits a 13-inch laptop screen; the chart area is wrapped in a
            // both-directions `scrollable`, so it scrolls rather than squeezing
            // text into dash lines or forcing a tall minimum window.
            min_size: Some(iced::Size::new(760.0, 600.0)),
            ..Default::default()
        })
        .run_with(|| {
            (
                StaticChartApp::with_optional_stores(
                    ChartStore::default_store(),
                    SettingsStore::default_store(),
                ),
                iced::Task::none(),
            )
        })
}

/// Bridges the pure [`StaticChartApp::update`] into the Iced update loop.
fn update(app: &mut StaticChartApp, message: Message) -> iced::Task<Message> {
    update_with_clock(app, message, system_clock::local_solar_moment);
    iced::Task::none()
}

/// Dispatches one Iced message, reading the clock only for a click-time `今`
/// action. The clock function is injectable so boundary behavior stays testable.
fn update_with_clock(
    app: &mut StaticChartApp,
    message: Message,
    clock: impl FnOnce() -> app::LocalSolarMoment,
) {
    match message {
        Message::TodayPressed => app.update(Message::SelectToday(clock())),
        message => app.update(message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_title_uses_only_ascii_window_chrome_text() {
        assert!(WINDOW_TITLE.is_ascii());
    }

    #[test]
    fn gui_manifest_uses_wgpu_without_software_fallback() {
        let manifest = include_str!("../Cargo.toml");

        assert!(
            manifest.contains(
                r#"iced = { version = "0.13", default-features = false, features = ["wgpu", "canvas"] }"#
            ),
            "GUI should use GPU rendering (wgpu) plus the canvas overlay; WSL safety comes from forcing XWayland before Iced starts"
        );
        assert!(
            !manifest.contains("tiny-skia"),
            "no software-rendering fallback should be enabled"
        );
    }

    #[test]
    fn wsl_launch_forces_the_stable_xwayland_path() {
        let source = include_str!("main.rs");

        assert!(source.contains(concat!("var_os(\"WSL_", "DISTRO_NAME\")")));
        assert!(source.contains(concat!("remove_var(\"WAYLAND_", "DISPLAY\")")));
        assert!(source.contains(concat!("remove_var(\"WAYLAND_", "SOCKET\")")));
    }

    #[test]
    fn today_pressed_reads_the_clock_at_update_time() {
        let mut app = StaticChartApp::new();
        app.update(Message::Generate);
        let expected = app::LocalSolarMoment {
            year: 2008,
            month: 2,
            day: 10,
            hour: 23,
            minute: 30,
        };
        let mut reads = 0;

        update_with_clock(&mut app, Message::TodayPressed, || {
            reads += 1;
            expected
        });

        assert_eq!(reads, 1);
        assert!(matches!(
            app.selected_temporal_selection(),
            iztro::core::StaticTemporalNavigationSelection::Hourly { hour_index: 12, .. }
        ));
    }

    #[test]
    fn chart_view_construction_does_not_read_the_system_clock() {
        let chart_source = include_str!("static_chart_screen/chart.rs");
        let temporal_source = include_str!("static_chart_screen/temporal.rs");

        for source in [chart_source, temporal_source] {
            assert!(!source.contains("Local::now"));
            assert!(!source.contains("current_moment"));
        }
        assert!(temporal_source.contains("Message::TodayPressed"));
    }
}
