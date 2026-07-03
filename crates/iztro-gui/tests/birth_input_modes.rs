//! Behavioral tests for the two GUI birth-time input modes.
//!
//! These exercise the public app API only. The clock-time assertions prove the
//! GUI routes birth-time resolution through core calculation policy
//! (`by_solar_with_options_report`) rather than deriving the 时辰 itself: the
//! same clock time resolves to a different 时辰 with and without apparent-solar-
//! time correction, and the GUI never computes that difference.

use iztro::core::Gender;
use iztro_gui::app::{
    BirthInput, ChartCache, GenerateOutcome, GuiSolarTimePolicy, InputMode, Message,
    SolarClockBirthInput, StaticChartApp, UtcOffsetChoice, utc_offset_choices,
};

/// Drives the startup form into clock mode with the given fields and generates.
fn generate_clock(apparent_solar_time: bool, longitude: &str) -> StaticChartApp {
    let mut app = StaticChartApp::new();
    app.update(Message::InputModeSelected(InputMode::Clock));
    app.update(Message::YearChanged("2000".to_string()));
    app.update(Message::MonthChanged("1".to_string()));
    app.update(Message::DayChanged("1".to_string()));
    app.update(Message::ClockHourChanged("1".to_string()));
    app.update(Message::ClockMinuteChanged("5".to_string()));
    app.update(Message::UtcOffsetSelected(UtcOffsetChoice::from_minutes(
        8 * 60,
    )));
    app.update(Message::ApparentSolarTimeToggled(apparent_solar_time));
    app.update(Message::LongitudeChanged(longitude.to_string()));
    app.update(Message::GenderSelected(Gender::Male));
    assert_eq!(app.generate(), GenerateOutcome::Built);
    app
}

#[test]
fn clock_mode_with_apparent_solar_time_resolves_time_branch_in_core() {
    // 01:05 at UTC+08:00 with longitude 105E corrects by -60 minutes to 00:05,
    // moving the 时辰 from 丑 (index 1) to 子 (index 0). The GUI passes the UTC
    // offset and longitude to core and reads back the resolved `timeIndex`; it
    // never computes the correction itself.
    let app = generate_clock(true, "105.0");
    let center = app.center().expect("generated chart center");
    assert_eq!(
        center.birth_time_index,
        Some(0),
        "core-resolved 时辰 must be 子 (timeIndex 0)"
    );
    assert_eq!(app.palaces().len(), 12);
}

#[test]
fn clock_mode_without_correction_uses_the_clock_time_directly() {
    // The same 01:05 clock time with correction disabled keeps 时辰 丑 (index 1),
    // derived by core straight from the clock time.
    let app = generate_clock(false, "105.0");
    let center = app.center().expect("generated chart center");
    assert_eq!(center.birth_time_index, Some(1));
}

#[test]
fn known_time_branch_mode_uses_the_selected_time_index() {
    // The default form is in known-time-branch mode; selecting a 时辰 generates a
    // chart whose center reports exactly that `timeIndex`, matching prior GUI
    // behavior.
    let mut app = StaticChartApp::new();
    assert_eq!(app.form().mode, InputMode::KnownTimeBranch);
    app.update(Message::TimeSelected(4));
    assert_eq!(app.generate(), GenerateOutcome::Built);
    let center = app.center().expect("generated chart center");
    assert_eq!(center.birth_time_index, Some(4));
    assert_eq!(app.palaces().len(), 12);
}

/// A clock-time input at 2000-01-01 01:05, UTC+08:00, with `policy`.
fn clock_input(policy: GuiSolarTimePolicy) -> BirthInput {
    BirthInput::SolarClock(SolarClockBirthInput {
        year: 2000,
        month: 1,
        day: 1,
        clock_hour: 1,
        clock_minute: 5,
        utc_offset_minutes: 8 * 60,
        solar_time_policy: policy,
        gender: Gender::Male,
    })
}

#[test]
fn clock_inputs_differing_in_correction_or_longitude_are_distinct_cache_keys() {
    let plain = clock_input(GuiSolarTimePolicy::ClockTime);
    let corrected_105 = clock_input(GuiSolarTimePolicy::ApparentSolarTime {
        longitude_micro_degrees: 105_000_000,
    });
    let corrected_120 = clock_input(GuiSolarTimePolicy::ApparentSolarTime {
        longitude_micro_degrees: 120_000_000,
    });

    let mut cache = ChartCache::default();
    cache.get_or_build(&plain).expect("build plain");
    cache.get_or_build(&corrected_105).expect("build 105");
    cache.get_or_build(&corrected_120).expect("build 120");

    // Toggling correction and changing only the longitude both produce distinct
    // keys, so a stale snapshot can never be served for a different policy.
    assert_eq!(cache.len(), 3);

    // The same input and default selection is a cache hit.
    let (_, hit) = cache.get_or_build(&corrected_105).expect("re-read 105");
    assert!(hit, "identical input + selection should hit the cache");
}

#[test]
fn utc_offset_choices_are_sorted_by_minutes_and_cover_common_offsets() {
    let choices = utc_offset_choices();
    let minutes: Vec<i32> = choices.iter().map(|choice| choice.minutes()).collect();

    let mut sorted = minutes.clone();
    sorted.sort_unstable();
    assert_eq!(
        minutes, sorted,
        "choices must be sorted ascending by minutes"
    );

    let mut deduped = minutes.clone();
    deduped.dedup();
    assert_eq!(minutes, deduped, "choices must be de-duplicated");

    // Values are stored as whole minutes east of UTC.
    assert!(minutes.contains(&(8 * 60)), "includes UTC+08:00 (480)");
    assert!(minutes.contains(&(4 * 60)), "includes UTC+04:00 (240)");
    assert!(minutes.contains(&330), "includes UTC+05:30 (330)");
    assert!(minutes.contains(&(-12 * 60)), "includes UTC-12:00");
    assert!(minutes.contains(&(14 * 60)), "includes UTC+14:00");

    // Labels render as fixed-offset UTC strings, not IANA time zones.
    let label_for = |m: i32| {
        choices
            .iter()
            .find(|choice| choice.minutes() == m)
            .expect("offset present")
            .label()
    };
    assert_eq!(label_for(8 * 60), "UTC+08:00");
    assert_eq!(label_for(-4 * 60), "UTC-04:00");
    assert_eq!(label_for(330), "UTC+05:30");
}
