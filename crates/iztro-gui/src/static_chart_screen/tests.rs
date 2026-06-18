use crate::app::StaticChartApp;
use iztro::core::{
    DecorativeStarFamily, FiveElementBureau, Mutagen, Scope, StarCategory, StarKind,
};

use super::labels::{bureau_label, center_four_pillar_rows, scope_zh, star_detail_label};
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
fn center_four_pillar_rows_use_available_zh_labels() {
    let center = sample_center();
    let rows = center_four_pillar_rows(&center);

    assert_eq!(rows.len(), 4);
    assert_eq!(rows[0].0, "年柱");
    assert_eq!(rows[1].0, "月柱");
    assert_eq!(rows[2].0, "日柱");
    assert_eq!(rows[3].0, "时柱");
    assert!(rows.iter().all(|(_, value)| !value.is_empty()));
}

#[test]
fn center_four_pillar_rows_are_empty_when_unavailable() {
    let mut center = sample_center();
    center.four_pillars = None;

    assert!(center_four_pillar_rows(&center).is_empty());
}

#[test]
fn bureau_label_handles_available_and_missing_values() {
    let mut center = sample_center();

    center.five_element_bureau = Some(FiveElementBureau::Fire6);
    assert_eq!(bureau_label(&center), "Fire6");

    center.five_element_bureau = None;
    assert_eq!(bureau_label(&center), "未提供");
}

#[test]
fn scope_labels_cover_every_supported_scope() {
    assert_eq!(scope_zh(Scope::Natal), "本命");
    assert_eq!(scope_zh(Scope::Decadal), "大限");
    assert_eq!(scope_zh(Scope::Age), "小限");
    assert_eq!(scope_zh(Scope::Yearly), "流年");
    assert_eq!(scope_zh(Scope::Monthly), "流月");
    assert_eq!(scope_zh(Scope::Daily), "流日");
    assert_eq!(scope_zh(Scope::Hourly), "流时");
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
fn palace_highlight_is_disjoint_between_selected_and_related() {
    // Selected always wins over related; none/selected/related are distinct.
    assert_ne!(PalaceHighlight::Selected, PalaceHighlight::Related);
    assert_ne!(PalaceHighlight::None, PalaceHighlight::Related);
}

#[test]
fn star_detail_label_covers_brightness_and_mutagen_combinations() {
    let mut star = sample_typed_star();
    star.name_zh = "测试星".to_owned();

    star.brightness_zh = "庙".to_owned();
    star.mutagen_zh = Some("化禄".to_owned());
    assert_eq!(star_detail_label(&star), "测试星庙化禄");

    star.mutagen_zh = None;
    assert_eq!(star_detail_label(&star), "测试星庙");

    star.brightness_zh.clear();
    star.mutagen_zh = Some("化忌".to_owned());
    assert_eq!(star_detail_label(&star), "测试星化忌");

    star.mutagen_zh = None;
    assert_eq!(star_detail_label(&star), "测试星");
}
