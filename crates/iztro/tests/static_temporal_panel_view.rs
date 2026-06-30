use std::collections::HashSet;

use iztro::core::{
    Chart, ChartAlgorithmKind, Gender, LunarChartRequest, LunarDay, LunarMonth, MethodProfile,
    PALACE_COUNT, StemBranch, build_empty_chart, by_lunar,
};
use iztro::{StaticChartViewSnapshot, StaticTemporalPanelView};

fn canonical_chart() -> Chart {
    let birth_year = StemBranch::from_lunar_year(1990);
    let request = LunarChartRequest::builder()
        .lunar_year(1990)
        .lunar_month(LunarMonth::new(5).expect("valid lunar month"))
        .lunar_day(LunarDay::new(17).expect("valid lunar day"))
        .iztro_time_index(4)
        .expect("valid time index")
        .gender(Gender::Female)
        .birth_year_stem(birth_year.stem())
        .birth_year_branch(birth_year.branch())
        .is_leap_month(false)
        .fix_leap(true)
        .method_profile(MethodProfile::new(
            "static_temporal_panel_view",
            ChartAlgorithmKind::QuanShu,
            "static temporal panel integration test",
        ))
        .build()
        .expect("lunar request should build");

    by_lunar(request).expect("canonical chart should build")
}

#[test]
fn decadal_cells_follow_public_static_view_order() {
    let cells = StaticChartViewSnapshot::from_chart(&canonical_chart())
        .temporal_panel
        .decadal_cells;

    let actual = cells
        .iter()
        .map(|cell| {
            (
                cell.age_range_zh.as_deref(),
                cell.limit_label_zh.as_deref(),
                cell.enabled,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        actual,
        vec![
            (Some("5-14"), Some("戊寅限"), true),
            (Some("15-24"), Some("己丑限"), true),
            (Some("25-34"), Some("戊子限"), true),
            (Some("35-44"), Some("丁亥限"), true),
            (Some("45-54"), Some("丙戌限"), true),
            (Some("55-64"), Some("乙酉限"), true),
            (Some("65-74"), Some("甲申限"), true),
            (Some("75-84"), Some("癸未限"), true),
            (Some("85-94"), Some("壬午限"), true),
            (Some("95-104"), Some("辛巳限"), true),
            (Some("105-114"), Some("庚辰限"), true),
            (Some("115-124"), Some("己卯限"), true),
        ]
    );
}

#[test]
fn missing_natal_facts_produce_twelve_disabled_decadal_cells() {
    let sample = canonical_chart();
    let empty = build_empty_chart(
        sample.birth_context().clone(),
        sample.birth_year(),
        sample.method_profile().clone(),
    )
    .expect("empty chart scaffold should build");
    let cells = StaticChartViewSnapshot::from_chart(&empty)
        .temporal_panel
        .decadal_cells;

    assert_eq!(cells.len(), PALACE_COUNT);
    assert!(cells.iter().all(|cell| {
        !cell.enabled && cell.age_range_zh.is_none() && cell.limit_label_zh.is_none()
    }));
}

#[test]
fn natal_panel_exposes_static_navigation_and_neutral_yearly_age_cells() {
    let panel = StaticChartViewSnapshot::from_chart(&canonical_chart()).temporal_panel;

    assert_eq!(panel.decadal_cells.len(), PALACE_COUNT);
    assert_eq!(panel.yearly_age_cells.len(), PALACE_COUNT);
    assert_eq!(
        panel
            .month_cells
            .iter()
            .map(|cell| cell.label_zh.as_str())
            .collect::<Vec<_>>(),
        vec![
            "正月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "冬月",
            "腊月",
        ]
    );
    assert_eq!(panel.day_rows.len(), 3);
    assert!(panel.day_rows.iter().all(|row| row.len() == 10));
    assert_eq!(panel.day_rows[0][0].label_zh, "初一");
    assert_eq!(panel.day_rows[2][9].label_zh, "三十");
    assert_eq!(
        panel
            .hour_cells
            .iter()
            .map(|cell| cell.label_zh.as_str())
            .collect::<Vec<_>>(),
        vec![
            "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"
        ]
    );
    // Under the natal base slice the flowing rows are visible but greyed: the
    // drill-down only unlocks them once a parent scope is selected.
    assert!(panel.month_cells.iter().all(|cell| !cell.enabled));
    assert!(panel.day_rows.iter().flatten().all(|cell| !cell.enabled));
    assert!(panel.hour_cells.iter().all(|cell| !cell.enabled));
    assert!(panel.yearly_age_cells.iter().all(|cell| {
        !cell.enabled && cell.year_label.is_none() && cell.stem_branch_age_zh.is_none()
    }));
}

#[test]
fn pre_decadal_cell_labels_the_span_before_the_first_limit() {
    let cell = StaticChartViewSnapshot::from_chart(&canonical_chart())
        .temporal_panel
        .pre_decadal_cell;

    // The first decadal period starts at age 5, so 限前 covers ages 1-4.
    assert!(cell.enabled);
    assert_eq!(cell.label_zh, "限前");
    assert_eq!(cell.age_range_zh.as_deref(), Some("1-4"));
}

#[test]
fn pre_decadal_cell_is_disabled_when_the_frame_is_missing() {
    let sample = canonical_chart();
    let empty = build_empty_chart(
        sample.birth_context().clone(),
        sample.birth_year(),
        sample.method_profile().clone(),
    )
    .expect("empty chart scaffold should build");
    let cell = StaticChartViewSnapshot::from_chart(&empty)
        .temporal_panel
        .pre_decadal_cell;

    assert!(!cell.enabled);
    assert_eq!(cell.label_zh, "限前");
    assert!(cell.age_range_zh.is_none());
}

#[test]
fn temporal_panel_decodes_legacy_json_without_pre_decadal_cell() {
    // A snapshot serialized before `pre_decadal_cell` existed must still decode,
    // defaulting the new field rather than failing the roundtrip.
    let mut value = serde_json::to_value(
        StaticChartViewSnapshot::from_chart(&canonical_chart()).temporal_panel,
    )
    .expect("panel should serialize");
    value
        .as_object_mut()
        .expect("panel is an object")
        .remove("pre_decadal_cell");

    let decoded: StaticTemporalPanelView =
        serde_json::from_value(value).expect("legacy panel should deserialize via serde default");
    assert!(!decoded.pre_decadal_cell.enabled);
    assert_eq!(decoded.pre_decadal_cell.label_zh, "");
}

#[test]
fn temporal_panel_serialization_has_stable_public_shape() {
    let panel = StaticChartViewSnapshot::from_chart(&canonical_chart()).temporal_panel;
    let value = serde_json::to_value(&panel).expect("temporal panel should serialize");
    let object = value
        .as_object()
        .expect("temporal panel should be an object");

    assert_eq!(
        object.keys().map(String::as_str).collect::<HashSet<_>>(),
        HashSet::from([
            "pre_decadal_cell",
            "decadal_cells",
            "yearly_age_cells",
            "month_cells",
            "day_rows",
            "hour_cells",
        ])
    );
    assert_eq!(
        object["decadal_cells"].as_array().unwrap().len(),
        PALACE_COUNT
    );
    assert_eq!(
        object["yearly_age_cells"].as_array().unwrap().len(),
        PALACE_COUNT
    );
    assert_eq!(
        object["month_cells"].as_array().unwrap().len(),
        PALACE_COUNT
    );
    assert_eq!(object["day_rows"].as_array().unwrap().len(), 3);
    assert!(
        object["day_rows"]
            .as_array()
            .unwrap()
            .iter()
            .all(|row| row.as_array().is_some_and(|cells| cells.len() == 10))
    );
    assert_eq!(object["hour_cells"].as_array().unwrap().len(), PALACE_COUNT);

    let decoded: StaticTemporalPanelView =
        serde_json::from_value(value).expect("temporal panel should deserialize");
    assert_eq!(decoded, panel);
}
