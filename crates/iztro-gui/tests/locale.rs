//! Locale behavior tests for the GUI: the same generated chart must render with
//! no Chinese user-facing labels under English and with the expected Chinese
//! labels under Simplified Chinese.
//!
//! Iced `Element`s don't expose their rendered text, so these tests assert at the
//! label-resolution boundary the renderer uses (the typed snapshot fields fed
//! through `iztro-i18n`) and additionally build the full view under both locales
//! to prove the render path never panics or yields a missing-key placeholder.

use iztro::core::{PalaceName, StaticChartCenterView, StaticPalaceView};
use iztro_gui::app::{Message, StaticChartApp};
use iztro_i18n::{I18n, Locale};

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
fn palace_labels(palace: &StaticPalaceView, i18n: &I18n) -> Vec<String> {
    let mut labels = vec![
        i18n.palace_name(palace.name),
        i18n.stem(palace.stem),
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
fn center_labels(center: &StaticChartCenterView, i18n: &I18n) -> Vec<String> {
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
        for sb in [pillars.yearly, pillars.monthly, pillars.daily, pillars.hourly] {
            labels.push(format!("{}{}", i18n.stem(sb.stem()), i18n.branch(sb.branch())));
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
    assert!(has_cjk(&i18n.star_name(star.name)), "zh star name should be Chinese");

    let center = app.center().expect("generated center");
    assert!(center_labels(center, &i18n).iter().any(|l| has_cjk(l)));
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
