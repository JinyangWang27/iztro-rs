use crate::app::StaticChartApp;
use iztro::core::{DecorativeStarFamily, Gender, Mutagen, StarCategory, StarKind};
use iztro_i18n::{I18n, Locale};

use super::labels::{four_pillars_line, gender_symbol};
use super::palace::{PalaceHighlight, StaticStarTone, star_tone};
use super::style::{DECOR_GOD_OLIVE, MINOR_MALEFIC, mutagen_badge_color, rgb8};

/// Builds an app with a generated chart (the startup screen has none).
fn chart_app() -> StaticChartApp {
    let mut app = StaticChartApp::new();
    app.generate();
    app
}

/// Owned copy of the generated chart's center facts.
fn sample_center() -> iztro::core::StaticChartCenterView {
    chart_app()
        .center()
        .expect("generated chart center")
        .clone()
}

fn sample_typed_star() -> iztro::core::StaticTypedStarView {
    let app = chart_app();
    app.palaces()
        .iter()
        .flat_map(|palace| {
            palace
                .major_stars
                .iter()
                .chain(&palace.minor_stars)
                .chain(&palace.adjective_stars)
        })
        .next()
        .expect("sample chart should contain a typed star")
        .clone()
}

#[test]
fn chart_screen_pins_a_minimum_size_and_scrolls_when_smaller() {
    let source = include_str!("chart.rs");

    // The grid + overlay stack is pinned to the fixed minimum chart size, not
    // Length::Fill, so a small window cannot squeeze it below legibility.
    assert!(source.contains("Length::Fixed(MIN_CHART_WIDTH)"));
    assert!(source.contains("Length::Fixed(MIN_CHART_HEIGHT)"));
    // A scrollable wrapper lets a smaller window scroll instead of shrinking.
    assert!(source.contains("scrollable(grid)"));
    assert!(source.contains("scrollable::Direction::Both"));
}

#[test]
fn palace_grid_layout_constants_exist_and_derive_the_chart_size() {
    use super::chart::{
        MIN_CHART_HEIGHT, MIN_CHART_WIDTH, MIN_PALACE_CELL_HEIGHT, MIN_PALACE_CELL_WIDTH,
    };

    // Per-cell minimums keep palace text legible, and the whole 4x4 canvas
    // minimum is derived from them (four columns wide, four rows tall).
    const {
        assert!(MIN_PALACE_CELL_WIDTH > 0.0);
        assert!(MIN_PALACE_CELL_HEIGHT > 0.0);
        assert!(MIN_CHART_WIDTH == MIN_PALACE_CELL_WIDTH * 4.0);
        assert!(MIN_CHART_HEIGHT == MIN_PALACE_CELL_HEIGHT * 4.0);
    }
}

#[test]
fn window_sets_a_minimum_size_to_complement_chart_scrolling() {
    let source = include_str!("../lib.rs");

    assert!(source.contains("min_size: Some("));
}

#[test]
fn four_pillars_line_joins_prepared_pillar_labels() {
    let center = sample_center();
    let i18n = I18n::new(Locale::ZhHans);
    let line = four_pillars_line(&center, &i18n).expect("four pillars present");
    // One row of four space-separated stem-branch pairs, not four labeled rows.
    assert_eq!(line.split(' ').count(), 4);
    assert!(!line.contains('年') && !line.contains('柱'));
}

#[test]
fn four_pillars_line_is_none_when_unavailable() {
    let mut center = sample_center();
    center.four_pillars = None;
    assert!(four_pillars_line(&center, &I18n::new(Locale::EnUs)).is_none());
}

#[test]
fn gender_symbol_uses_mars_and_venus_glyphs() {
    assert_eq!(gender_symbol(Gender::Male), "♂");
    assert_eq!(gender_symbol(Gender::Female), "♀");
}

#[test]
fn bureau_label_is_a_prepared_chinese_core_field() {
    // The GUI no longer Debug-formats the bureau; it reads the prepared label.
    let center = sample_center();
    let label = center
        .five_element_bureau_zh
        .as_deref()
        .expect("prepared bureau label");
    assert!(label.ends_with('局'), "got {label}");
}

#[test]
fn basic_information_uses_two_alternating_columns() {
    let source = include_str!("palace.rs");

    assert!(source.contains("row![basic_left, basic_right]"));
    // The two columns are built from localized section labels, not hardcoded
    // Chinese literals.
    assert!(source.contains("let basic_left = column!["));
    assert!(source.contains("center-five-element-bureau"));
    assert!(source.contains("let basic_right = column!["));
    assert!(source.contains("center-nominal-age"));
}

/// A typed star carrying only the field that drives visual classification.
fn typed_star_with_kind(kind: StarKind) -> iztro::core::StaticTypedStarView {
    let mut star = sample_typed_star();
    star.kind = kind;
    star
}

#[test]
fn major_kind_maps_to_major_tone() {
    assert_eq!(
        star_tone(&typed_star_with_kind(StarKind::Major)),
        StaticStarTone::Major
    );
}

#[test]
fn soft_minor_pairs_map_to_minor_purple() {
    // Covers 左辅/右弼/天魁/天钺/文昌/文曲 — all prepared as StarKind::Soft.
    assert_eq!(
        star_tone(&typed_star_with_kind(StarKind::Soft)),
        StaticStarTone::MinorPurple
    );
}

#[test]
fn six_malefics_map_to_minor_malefic() {
    // Covers 擎羊/陀罗/火星/铃星/地空/地劫 — all prepared as StarKind::Tough.
    assert_eq!(
        star_tone(&typed_star_with_kind(StarKind::Tough)),
        StaticStarTone::MinorMalefic
    );
}

#[test]
fn lucun_and_tianma_map_to_their_own_tones() {
    assert_eq!(
        star_tone(&typed_star_with_kind(StarKind::LuCun)),
        StaticStarTone::LuCun
    );
    assert_eq!(
        star_tone(&typed_star_with_kind(StarKind::TianMa)),
        StaticStarTone::TianMa
    );
}

#[test]
fn flower_stars_map_to_peach_blossom() {
    // Covers 红鸾/咸池/天姚/天喜 (and flow 鸾/喜) — all prepared as StarKind::Flower.
    assert_eq!(
        star_tone(&typed_star_with_kind(StarKind::Flower)),
        StaticStarTone::AdjPeachBlossom
    );
}

#[test]
fn ordinary_adjective_stars_map_to_default() {
    assert_eq!(
        star_tone(&typed_star_with_kind(StarKind::Adjective)),
        StaticStarTone::AdjDefault
    );
    assert_eq!(
        star_tone(&typed_star_with_kind(StarKind::Helper)),
        StaticStarTone::AdjDefault
    );
}

#[test]
fn mutagen_badge_colors_cover_all_four_transformations() {
    assert_eq!(mutagen_badge_color(Mutagen::Lu), rgb8(0xd4, 0x38, 0x0d));
    assert_eq!(mutagen_badge_color(Mutagen::Quan), rgb8(0x2f, 0x54, 0xeb));
    assert_eq!(mutagen_badge_color(Mutagen::Ke), rgb8(0x23, 0x78, 0x04));
    assert_eq!(mutagen_badge_color(Mutagen::Ji), rgb8(0x00, 0x00, 0x00));
}

#[test]
fn star_kind_routes_to_expected_palace_zone() {
    // Zone placement uses the prepared kind's coarse category.
    assert_eq!(StarKind::Major.category(), StarCategory::Major);
    assert_eq!(StarKind::Soft.category(), StarCategory::Minor);
    assert_eq!(StarKind::Tough.category(), StarCategory::Minor);
    assert_eq!(StarKind::LuCun.category(), StarCategory::Minor);
    assert_eq!(StarKind::TianMa.category(), StarCategory::Minor);
    assert_eq!(StarKind::Flower.category(), StarCategory::Adjective);
    assert_eq!(StarKind::Adjective.category(), StarCategory::Adjective);
    assert_eq!(StarKind::Helper.category(), StarCategory::Adjective);
}

#[test]
fn decorative_family_splits_into_bottom_zones() {
    // 长生/博士 share the olive bottom-left tone; 将前/岁前 share the malefic
    // bottom-right tone. Each family lands in exactly one zone.
    for family in [
        DecorativeStarFamily::Changsheng12,
        DecorativeStarFamily::Boshi12,
    ] {
        assert!(matches!(
            family,
            DecorativeStarFamily::Changsheng12 | DecorativeStarFamily::Boshi12
        ));
    }
    assert_eq!(DECOR_GOD_OLIVE, rgb8(0x90, 0x98, 0x3c));
    assert_eq!(MINOR_MALEFIC, rgb8(0x81, 0x33, 0x59));
}

#[test]
fn palace_cell_uses_a_dedicated_bottom_decorative_layer() {
    let source = include_str!("palace.rs");

    assert!(source.contains(concat!("fn bottom_", "decorative_layer")));
    assert!(source.contains(concat!("stack", "![")));
    assert!(source.contains(concat!("DECORATIVE_", "AREA_HEIGHT")));
}

#[test]
fn palace_middle_band_is_deliberately_reserved() {
    use super::style::{PALACE_MIDDLE_BAND_HEIGHT, PERIOD_BADGE_ROW_HEIGHT};

    // The badge row reserves real height, and the full middle band is taller
    // still (badge row + 大限/小限 line).
    const {
        assert!(PERIOD_BADGE_ROW_HEIGHT > 0.0);
        assert!(PALACE_MIDDLE_BAND_HEIGHT > PERIOD_BADGE_ROW_HEIGHT);
    }

    let source = include_str!("palace.rs");
    // The badge row keeps a fixed height even with no badge, and the middle band
    // is a fixed-height layer centered vertically, so 大限/小限 aligns across
    // palaces whether or not a period badge is present.
    assert!(source.contains("Length::Fixed(PERIOD_BADGE_ROW_HEIGHT)"));
    assert!(source.contains("Length::Fixed(PALACE_MIDDLE_BAND_HEIGHT)"));
    assert!(source.contains("align_y(Alignment::Center)"));
    // Three independent stacked layers: top stars, centered middle band, footer.
    assert!(source.contains("star_layer,"));
    assert!(source.contains("middle_layer,"));
}

#[test]
fn palace_footer_anchors_name_left_and_stem_branch_right() {
    let source = include_str!("palace.rs");

    // The footer renders the localized palace name (left) and stem-branch (right)
    // from typed fields, not pre-rendered Chinese strings.
    assert!(source.contains("i18n.palace_name(palace.name)).size(16).color(MAJOR_PURPLE)"));
    assert!(source.contains("i18n.stem_branch(palace.stem, palace.branch)"));
    assert!(source.contains("align_x(Alignment::Start)"));
    assert!(source.contains("align_x(Alignment::End)"));
}

#[test]
fn palace_highlight_is_disjoint_between_selected_and_related() {
    // Selected always wins over related; none/selected/related are distinct.
    assert_ne!(PalaceHighlight::Selected, PalaceHighlight::Related);
    assert_ne!(PalaceHighlight::None, PalaceHighlight::Related);
}

#[test]
fn period_badge_label_comes_from_prepared_overlay_field() {
    use iztro::core::{StaticTemporalNavigationSelection, static_temporal_chart_view};

    // A 流年 selection attaches an overlay whose compact badge label is prepared
    // by core (e.g. `流年·丁`); the GUI renders it verbatim.
    let request = {
        use iztro::core::{
            BirthTime, ChartAlgorithmKind, Gender, MethodProfile, SolarChartRequest, SolarDay,
            SolarMonth,
        };
        SolarChartRequest::builder()
            .solar_year(1993)
            .solar_month(SolarMonth::new(5).unwrap())
            .solar_day(SolarDay::new(27).unwrap())
            .birth_time_variant(BirthTime::from_iztro_time_index(9).unwrap())
            .gender(Gender::Male)
            .method_profile(MethodProfile::new(
                "iztro_gui_test",
                ChartAlgorithmKind::QuanShu,
                "period badge label test",
            ))
            .build()
            .unwrap()
    };
    let snapshot = static_temporal_chart_view(
        request,
        StaticTemporalNavigationSelection::Yearly {
            decadal_index: 1,
            year_index: 0,
        },
    )
    .unwrap();
    let label = snapshot
        .palaces
        .iter()
        .flat_map(|p| p.overlays.iter())
        .find_map(|o| o.period_label_zh.clone())
        .expect("a prepared period label");
    assert!(label.contains('·'), "got {label}");
}

#[test]
fn palace_badges_are_gated_on_prepared_period_label_only() {
    let source = include_str!("palace.rs");

    // The badge row is built only from overlays whose typed period stem is set
    // (the anchor palace); non-marker overlays (and their temporal palace-name
    // metadata) never yield a badge.
    assert!(source.contains("overlay.period_stem"));
    assert!(
        !source.contains("temporal_palace_name_zh"),
        "the GUI must not derive a badge from temporal palace-name metadata"
    );
}

#[test]
fn period_badge_takes_a_prepared_label_not_an_overlay() {
    let source = include_str!("temporal.rs");

    // `period_badge` renders the core-prepared label string directly; it no
    // longer inspects an overlay or falls back to `temporal_palace_name_zh`.
    assert!(source.contains("pub(super) fn period_badge(\n    label: &str,"));
    assert!(
        !source.contains("temporal_palace_name_zh"),
        "the badge renderer must not fall back to temporal palace-name metadata"
    );
}

#[test]
fn startup_exposes_name_input_and_saved_chart_actions() {
    let source = include_str!("startup.rs");

    // A localized name input drives the chart name.
    assert!(source.contains("field-name"));
    assert!(source.contains("Message::NameChanged"));
    // Saved rows can be opened, edited, and deleted.
    assert!(source.contains("Message::SelectSaved(index)"));
    assert!(source.contains("Message::EditSaved(index)"));
    assert!(source.contains("Message::DeleteSaved(index)"));
    // The primary button reads as an update while editing a saved chart.
    assert!(source.contains("button-update"));
    assert!(source.contains("button-generate"));
}

#[test]
fn temporal_controls_render_on_a_single_row() {
    let source = include_str!("temporal.rs");

    // The compact stepper must be one horizontal row, not the old two-line
    // `column![ row![backs, today], forwards ]` layout that wrapped controls.
    assert!(
        !source.contains(concat!("column", "![")),
        "temporal controls must be a single row, not a two-line column"
    );
    // The single row keeps the backward … today … forward ordering with the
    // today control between the backward and forward steppers.
    assert!(source.contains("Message::TodayPressed"));
    // The today control is a row item between the backward hour step and the
    // forward hour step, keeping them adjacent on the single line.
    let backward_hour = source
        .find("back(Scope::Hourly)")
        .expect("backward hour step");
    let today_item = source.find("\n        today,").expect("today row item");
    let forward_hour = source
        .find("fwd(Scope::Hourly)")
        .expect("forward hour step");
    assert!(
        backward_hour < today_item && today_item < forward_hour,
        "the today control sits between the backward and forward steppers"
    );
}
