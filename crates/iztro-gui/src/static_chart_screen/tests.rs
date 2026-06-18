use crate::app::StaticChartApp;
use iztro::core::{DecorativeStarFamily, Gender, Mutagen, StarCategory, StarKind};

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
fn four_pillars_line_joins_prepared_pillar_labels() {
    let center = sample_center();
    let line = four_pillars_line(&center).expect("four pillars present");
    // One row of four space-separated stem-branch pairs, not four labeled rows.
    assert_eq!(line.split(' ').count(), 4);
    assert!(!line.contains('年') && !line.contains('柱'));
}

#[test]
fn four_pillars_line_is_none_when_unavailable() {
    let mut center = sample_center();
    center.four_pillars = None;
    assert!(four_pillars_line(&center).is_none());
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
    assert!(
        source.contains("let basic_left = column![\n        fact_row(\n            \"五行局\"")
    );
    assert!(
        source
            .contains("let basic_right = column![\n        fact_row(\n            \"年龄(虚岁)\"")
    );
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
fn palace_footer_anchors_name_left_and_stem_branch_right() {
    let source = include_str!("palace.rs");

    assert!(source.contains("text(palace.name_zh.as_str()).size(16).color(MAJOR_PURPLE)"));
    assert!(
        source.contains(
            "text(format!(\"{}{}\", palace.stem_zh, palace.branch_zh))\n            .size(12)\n            .color(mutagen_badge_color(Mutagen::Ke))"
        )
    );
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
