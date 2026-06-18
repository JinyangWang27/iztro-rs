use iztro_gui::app::{Message, StaticChartApp};
use iztro_gui::static_chart_screen;

#[test]
fn full_static_chart_view_builds_from_prepared_snapshot() {
    let app = StaticChartApp::new();

    let _ = static_chart_screen::view(&app);
}

#[test]
fn full_static_chart_view_builds_with_visible_input_error() {
    let mut app = StaticChartApp::new();
    app.update(Message::YearChanged("not-a-year".to_owned()));
    app.update(Message::Generate);
    assert!(app.error().is_some());

    let _ = static_chart_screen::view(&app);
}
