use iztro_gui::app::{Message, Screen, StaticChartApp, StepDirection, TemporalCell, TemporalUnit};
use iztro_gui::static_chart_screen;

#[test]
fn startup_screen_builds_without_a_generated_chart() {
    let app = StaticChartApp::new();
    assert_eq!(app.screen(), Screen::Startup);
    assert!(app.snapshot().is_none());

    let _ = static_chart_screen::view(&app);
}

#[test]
fn startup_screen_builds_with_visible_input_error() {
    let mut app = StaticChartApp::new();
    app.update(Message::YearChanged("not-a-year".to_owned()));
    app.update(Message::Generate);
    assert!(app.error().is_some());
    assert_eq!(app.screen(), Screen::Startup);

    let _ = static_chart_screen::view(&app);
}

#[test]
fn chart_screen_builds_after_generating_with_full_interaction_state() {
    let mut app = StaticChartApp::new();
    app.update(Message::Generate);
    assert_eq!(app.screen(), Screen::Chart);

    // Exercise the selection, temporal-click, and temporal-stepper paths.
    let branch = app.palaces()[0].branch;
    app.update(Message::SelectPalace(branch));
    app.update(Message::SelectTemporalCell(TemporalCell::Month(0)));
    app.update(Message::StepTemporal(
        TemporalUnit::Decadal,
        StepDirection::Forward,
    ));

    let _ = static_chart_screen::view(&app);

    // Returning to the natal 命宫 default still renders cleanly.
    app.update(Message::SelectTemporalCell(TemporalCell::Natal));
    let _ = static_chart_screen::view(&app);
}
