//! Locale behavior tests for the GUI: the same generated chart must render with
//! no Chinese user-facing labels under English and with the expected Chinese
//! labels under Simplified Chinese.
//!
//! Iced `Element`s don't expose their rendered text, so these tests assert at the
//! label-resolution boundary the renderer uses (the typed snapshot fields fed
//! through `iztro-i18n`) and additionally build the full view under both locales
//! to prove the render path never panics or yields a missing-key placeholder.

use iztro::core::{PalaceName, Scope};
use iztro::{StaticChartCenterProjection, StaticPalaceProjection};
use iztro_gui::app::{FormError, Message, StaticChartApp, TemporalCell};
use iztro_i18n::{I18n, Locale};

/// Every Fluent key the GUI renderer resolves directly via `i18n.text(..)`.
/// A missing key would resolve to a `!key!` placeholder, which this list guards.
const GUI_UI_KEYS: &[&str] = &[
    "app-title",
    "startup-title",
    "startup-subtitle",
    "ui-language",
    "ui-english",
    "ui-simplified-chinese",
    "field-name",
    "field-year",
    "field-month",
    "field-day",
    "field-time",
    "field-gender",
    "chart-name-placeholder",
    "button-generate",
    "button-update",
    "button-cancel",
    "button-back",
    "button-edit",
    "button-delete",
    "chart-saved-charts",
    "saved-empty",
    "input-error",
    "name-required",
    "error-year",
    "error-month",
    "error-day",
    "error-invalid-calendar-date",
    "error-invalid-birth-time",
    "error-invalid-temporal-selection",
    "error-chart-generation-failed",
    "persistence-unavailable",
    "center-basic-info",
    "center-temporal-info",
    "center-five-element-bureau",
    "center-four-pillars",
    "center-lunar",
    "center-solar",
    "center-zodiac",
    "center-birth-time",
    "center-constellation",
    "center-soul-master",
    "center-body-master",
    "center-life-palace",
    "center-body-palace",
    "center-nominal-age",
    "temporal-today",
];

/// Every `FormError` variant, so the error-localization test is exhaustive.
const ALL_FORM_ERRORS: &[FormError] = &[
    FormError::NameRequired,
    FormError::YearInvalid,
    FormError::MonthInvalid,
    FormError::DayInvalid,
    FormError::InvalidCalendarDate,
    FormError::InvalidBirthTime,
    FormError::InvalidTemporalSelection,
    FormError::ChartGenerationFailed,
    FormError::PersistenceUnavailable,
];

/// True when `text` contains any CJK Unified Ideograph.
fn has_cjk(text: &str) -> bool {
    text.chars().any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c))
}

/// Builds an app with the canonical generated chart.
fn chart_app() -> StaticChartApp {
    let mut app = StaticChartApp::new();
    app.generate();
    app
}

/// Every user-facing label the renderer derives for a palace, via `iztro-i18n`.
fn palace_labels(palace: &StaticPalaceProjection, i18n: &I18n) -> Vec<String> {
    let mut labels = vec![
        i18n.palace_name(palace.natal_identity.palace_name),
        i18n.palace_name(palace.active_frame.palace_name),
        i18n.stem(palace.natal_identity.stem),
        i18n.branch(palace.branch),
    ];
    for star in palace
        .major_stars
        .iter()
        .chain(&palace.minor_stars)
        .chain(&palace.adjective_stars)
    {
        labels.push(i18n.star_name(star.name));
        labels.push(i18n.brightness(star.brightness));
        if let Some(mutagen) = star.mutagen {
            labels.push(i18n.mutagen(mutagen));
        }
    }
    for star in &palace.decorative_stars {
        labels.push(i18n.star_name(star.name));
    }
    for overlay in &palace.overlays {
        if let Some(stem) = overlay.period_stem {
            labels.push(format!(
                "{}·{}",
                i18n.temporal_label(overlay.scope),
                i18n.stem(stem)
            ));
        }
    }
    labels
}

/// Every user-facing value label the renderer derives for the center panel.
fn center_labels(center: &StaticChartCenterProjection, i18n: &I18n) -> Vec<String> {
    let mut labels = Vec::new();
    if let Some(bureau) = center.five_element_bureau {
        labels.push(i18n.bureau(bureau));
    }
    if let Some(date) = &center.birth_lunar_date {
        labels.push(i18n.lunar_date(date));
    }
    labels.push(i18n.zodiac_animal(center.birth_year_branch));
    if let Some(star) = center.soul_master {
        labels.push(i18n.master(star));
    }
    if let Some(star) = center.body_master {
        labels.push(i18n.master(star));
    }
    if let Some(index) = center.birth_time_index {
        labels.push(i18n.double_hour(index));
    }
    if let Some(sign) = center.western_zodiac {
        labels.push(i18n.constellation(sign));
    }
    if let Some(pillars) = &center.four_pillars {
        for sb in [
            pillars.yearly,
            pillars.monthly,
            pillars.daily,
            pillars.hourly,
        ] {
            labels.push(format!(
                "{}{}",
                i18n.stem(sb.stem()),
                i18n.branch(sb.branch())
            ));
        }
    }
    labels
}

#[test]
fn english_chart_labels_contain_no_chinese() {
    let app = chart_app();
    let i18n = I18n::new(Locale::EnUs);
    let center = app.center().expect("generated center");

    for label in center_labels(center, &i18n) {
        assert!(!has_cjk(&label), "English center label has CJK: {label:?}");
        assert!(!label.starts_with('!'), "missing English key: {label:?}");
    }
    for palace in app.palaces() {
        for label in palace_labels(palace, &i18n) {
            assert!(!has_cjk(&label), "English palace label has CJK: {label:?}");
            assert!(!label.starts_with('!'), "missing English key: {label:?}");
        }
    }
}

#[test]
fn simplified_chinese_chart_labels_use_expected_chinese() {
    let app = chart_app();
    let i18n = I18n::new(Locale::ZhHans);

    // Palace names preserve the conventional Chinese terms.
    assert_eq!(i18n.palace_name(PalaceName::Life), "命宫");

    // Star and center labels are Chinese for a chart that has stars.
    let starred = app
        .palaces()
        .iter()
        .find(|p| !p.major_stars.is_empty())
        .expect("a palace with a major star");
    let star = &starred.major_stars[0];
    assert!(
        has_cjk(&i18n.star_name(star.name)),
        "zh star name should be Chinese"
    );

    let center = app.center().expect("generated center");
    assert!(center_labels(center, &i18n).iter().any(|l| has_cjk(l)));
}

#[test]
fn minor_limit_renders_localized_from_typed_fields() {
    // Drive the app to a 流年 selection so an active 小限 (Minor Limit) is
    // exposed, then assert it localizes through the same i18n helpers the
    // renderer uses — English "Minor Limit", Simplified Chinese "小限".
    let mut app = chart_app();
    app.update(Message::SelectTemporalCell(TemporalCell::Decadal(0)));
    app.update(Message::SelectTemporalCell(TemporalCell::YearlyAge(0)));
    let center = app.center().expect("generated center");
    let age = center
        .small_limit_age
        .expect("a selected year exposes a 小限 age");
    let branch = center
        .small_limit_branch
        .expect("a selected year exposes a 小限 branch");

    // The center 小限 row carries only the landing branch (the age is the
    // nominal age shown above), and the active palace middle band renders the
    // localized label plus the selected age — both through the same helpers.
    let en = I18n::new(Locale::EnUs);
    assert_eq!(en.temporal_label(Scope::Age), "Minor Limit");
    let en_center = en.branch(branch);
    assert!(
        !has_cjk(&en_center),
        "English 小限 row has CJK: {en_center}"
    );
    let en_band = format!("{} {age}", en.temporal_label(Scope::Age));
    assert!(!has_cjk(&en_band), "English 小限 band has CJK: {en_band}");

    let zh = I18n::new(Locale::ZhHans);
    assert_eq!(zh.temporal_label(Scope::Age), "小限");
    let zh_center = zh.branch(branch);
    assert!(
        has_cjk(&zh_center),
        "zh 小限 row should be Chinese: {zh_center}"
    );
    let zh_band = format!("{} {age}", zh.temporal_label(Scope::Age));
    assert!(
        has_cjk(&zh_band),
        "zh 小限 band should be Chinese: {zh_band}"
    );

    // The active 小限 palace exposes the typed selected age, so the middle band
    // renders 小限 without depending on the Chinese-string fallback.
    let active = app
        .palaces()
        .iter()
        .find(|p| p.limit.is_active_small_limit)
        .expect("exactly one active 小限 palace");
    assert_eq!(active.limit.active_small_limit_age, Some(age));
    assert!(!active.limit.small_limit_ages.is_empty());
}

#[test]
fn the_full_view_builds_under_both_locales() {
    // Building the widget tree exercises every render path and would surface a
    // missing key (as a `!key!` placeholder) without panicking.
    let mut app = chart_app();
    for locale in Locale::ALL {
        app.update(Message::SetLocale(locale));
        let _ = iztro_gui::static_chart_screen::view(&app);
    }
    // Also the startup screen.
    let mut startup = StaticChartApp::new();
    for locale in Locale::ALL {
        startup.update(Message::SetLocale(locale));
        let _ = iztro_gui::static_chart_screen::view(&startup);
    }
}

#[test]
fn every_gui_ui_key_resolves_in_every_locale() {
    // Unlike the full-view build, this directly inspects resolved text, so a
    // missing key (`!key!` placeholder) fails instead of silently rendering.
    for locale in Locale::ALL {
        let i18n = I18n::new(locale);
        for key in GUI_UI_KEYS {
            let value = i18n.text(key);
            assert!(
                !value.starts_with('!'),
                "missing key {key} for locale {locale}: {value}"
            );
        }
    }
}

#[test]
fn form_errors_localize_without_leaking_raw_core_strings() {
    for &error in ALL_FORM_ERRORS {
        // The key resolves (no placeholder) in both locales.
        for locale in Locale::ALL {
            let value = I18n::new(locale).text(error.fluent_key());
            assert!(
                !value.starts_with('!'),
                "missing error key {} for {locale}: {value}",
                error.fluent_key()
            );
        }
        // Under Simplified Chinese every error renders Chinese text, proving no
        // raw English `ChartError` string leaks through the boundary.
        let zh = I18n::new(Locale::ZhHans).text(error.fluent_key());
        assert!(
            has_cjk(&zh),
            "zh error for {error:?} is not localized: {zh}"
        );
    }
}
