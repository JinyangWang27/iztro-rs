//! Facade tests for `static_temporal_chart_view`.
//!
//! These verify that all temporal-overlay derivation stays inside core: a GUI
//! passes a [`SolarChartRequest`] plus a renderer-neutral
//! [`StaticTemporalNavigationSelection`] and gets back a prepared
//! [`StaticChartViewSnapshot`]. Selecting a temporal cell changes overlays only,
//! never natal facts.

use iztro::core::{
    BirthTime, ChartAlgorithmKind, ChartError, Gender, MethodProfile, Scope, SolarChartRequest,
    SolarDay, SolarMonth, StaticTemporalNavigationSelection, static_temporal_chart_view,
};

fn sample_request() -> SolarChartRequest {
    SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(5).unwrap())
        .solar_day(SolarDay::new(17).unwrap())
        .birth_time_variant(BirthTime::from_iztro_time_index(4).unwrap())
        .gender(Gender::Female)
        .method_profile(MethodProfile::new(
            "iztro_test",
            ChartAlgorithmKind::QuanShu,
            "static temporal chart view test",
        ))
        .build()
        .unwrap()
}

#[test]
fn natal_selection_yields_natal_only_snapshot() {
    let snapshot =
        static_temporal_chart_view(sample_request(), StaticTemporalNavigationSelection::Natal)
            .expect("natal selection should build");

    assert_eq!(snapshot.active_scopes, vec![Scope::Natal]);
    assert!(
        snapshot.palaces.iter().all(|p| p.overlays.is_empty()),
        "natal snapshot must carry no temporal overlays"
    );
}

#[test]
fn pre_decadal_selection_is_natal_base() {
    let snapshot = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::PreDecadal,
    )
    .expect("pre-decadal selection should build");

    assert_eq!(snapshot.active_scopes, vec![Scope::Natal]);
    assert!(snapshot.palaces.iter().all(|p| p.overlays.is_empty()));
}

#[test]
fn decadal_selection_attaches_a_decadal_overlay() {
    let snapshot = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::Decadal { decadal_index: 1 },
    )
    .expect("decadal selection should build");

    assert!(
        snapshot.active_scopes.contains(&Scope::Decadal),
        "decadal selection must activate the decadal scope"
    );
    assert!(
        snapshot
            .palaces
            .iter()
            .any(|p| p.overlays.iter().any(|o| o.scope == Scope::Decadal)),
        "at least one palace must carry a decadal overlay"
    );
}

#[test]
fn out_of_range_decadal_index_is_an_error() {
    let result = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::Decadal { decadal_index: 999 },
    );
    assert!(result.is_err(), "an impossible decadal index must error");
}

#[test]
fn temporal_selection_changes_overlays_only_not_natal_facts() {
    let natal =
        static_temporal_chart_view(sample_request(), StaticTemporalNavigationSelection::Natal)
            .unwrap();
    let decadal = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::Decadal { decadal_index: 0 },
    )
    .unwrap();

    // Center natal facts are identical regardless of temporal selection.
    assert_eq!(natal.center, decadal.center);

    // Natal palace identity, surround (三方四正) and natal star lists are
    // byte-identical; only overlays may differ.
    assert_eq!(natal.palaces.len(), decadal.palaces.len());
    for (n, d) in natal.palaces.iter().zip(decadal.palaces.iter()) {
        assert_eq!(n.branch, d.branch);
        assert_eq!(n.surround, d.surround);
        assert_eq!(n.major_stars, d.major_stars);
        assert_eq!(n.minor_stars, d.minor_stars);
    }
}

#[test]
fn pre_decadal_default_greys_lower_rows_and_enables_decadal() {
    let panel = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::PreDecadal,
    )
    .unwrap()
    .temporal_panel;

    assert!(
        panel.pre_decadal_cell.selected,
        "限前 is the default selection"
    );
    assert!(
        panel.decadal_cells.iter().any(|c| c.enabled),
        "大限 row is enabled by default"
    );
    assert!(
        panel.yearly_age_cells.iter().all(|c| !c.enabled),
        "流年 row greyed before 大限 selection"
    );
    assert!(panel.month_cells.iter().all(|c| !c.enabled));
    assert!(
        panel
            .day_rows
            .iter()
            .all(|row| row.iter().all(|c| !c.enabled))
    );
    assert!(panel.hour_cells.iter().all(|c| !c.enabled));
}

#[test]
fn decadal_selection_enables_exactly_ten_yearly_cells() {
    let panel = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::Decadal { decadal_index: 2 },
    )
    .unwrap()
    .temporal_panel;

    let enabled: Vec<_> = panel
        .yearly_age_cells
        .iter()
        .filter(|c| c.enabled)
        .collect();
    assert_eq!(enabled.len(), 10, "a 大限 spans exactly 10 流年");
    assert!(
        enabled
            .iter()
            .all(|c| c.year_label.is_some() && c.stem_branch_age_zh.is_some()),
        "each 流年 cell carries year + stem-branch-age labels"
    );
    assert!(panel.decadal_cells[2].selected);
    // 流月 still greyed until a 流年 is selected.
    assert!(panel.month_cells.iter().all(|c| !c.enabled));
}

#[test]
fn each_parent_enables_only_the_next_child_row() {
    let yearly = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::Yearly {
            decadal_index: 2,
            year_index: 0,
        },
    )
    .unwrap()
    .temporal_panel;
    assert!(
        yearly.month_cells.iter().all(|c| c.enabled),
        "流年 enables 流月"
    );
    assert_eq!(yearly.month_cells.len(), 12);
    assert!(yearly.yearly_age_cells[0].selected);
    assert!(
        yearly
            .day_rows
            .iter()
            .all(|row| row.iter().all(|c| !c.enabled)),
        "流日 greyed until a 流月 is selected"
    );

    let monthly = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::Monthly {
            decadal_index: 2,
            year_index: 0,
            month_index: 0,
        },
    )
    .unwrap()
    .temporal_panel;
    assert!(
        monthly
            .day_rows
            .iter()
            .any(|row| row.iter().any(|c| c.enabled)),
        "流月 enables 流日"
    );
    assert!(monthly.hour_cells.iter().all(|c| !c.enabled), "流时 greyed");
    assert!(monthly.month_cells[0].selected);

    let daily = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::Daily {
            decadal_index: 2,
            year_index: 0,
            month_index: 0,
            day_index: 0,
        },
    )
    .unwrap()
    .temporal_panel;
    assert!(
        daily.hour_cells.iter().all(|c| c.enabled),
        "流日 enables 流时"
    );
    assert_eq!(daily.hour_cells.len(), 12);
    assert!(daily.day_rows[0][0].selected);
}

#[test]
fn each_selection_builds_exactly_its_partial_scope_stack() {
    let cases = [
        (
            StaticTemporalNavigationSelection::Decadal { decadal_index: 2 },
            vec![Scope::Natal, Scope::Decadal],
        ),
        (
            StaticTemporalNavigationSelection::Yearly {
                decadal_index: 2,
                year_index: 0,
            },
            vec![Scope::Natal, Scope::Decadal, Scope::Age, Scope::Yearly],
        ),
        (
            StaticTemporalNavigationSelection::Monthly {
                decadal_index: 2,
                year_index: 0,
                month_index: 0,
            },
            vec![
                Scope::Natal,
                Scope::Decadal,
                Scope::Age,
                Scope::Yearly,
                Scope::Monthly,
            ],
        ),
        (
            StaticTemporalNavigationSelection::Daily {
                decadal_index: 2,
                year_index: 0,
                month_index: 0,
                day_index: 0,
            },
            vec![
                Scope::Natal,
                Scope::Decadal,
                Scope::Age,
                Scope::Yearly,
                Scope::Monthly,
                Scope::Daily,
            ],
        ),
        (
            StaticTemporalNavigationSelection::Hourly {
                decadal_index: 2,
                year_index: 0,
                month_index: 0,
                day_index: 0,
                hour_index: 6,
            },
            vec![
                Scope::Natal,
                Scope::Decadal,
                Scope::Age,
                Scope::Yearly,
                Scope::Monthly,
                Scope::Daily,
                Scope::Hourly,
            ],
        ),
    ];

    for (selection, expected_scopes) in cases {
        let snapshot = static_temporal_chart_view(sample_request(), selection)
            .expect("partial temporal stack should build");
        assert_eq!(snapshot.active_scopes, expected_scopes);
        if let StaticTemporalNavigationSelection::Hourly { hour_index, .. } = selection {
            assert!(snapshot.temporal_panel.hour_cells[hour_index as usize].selected);
        }
    }
}

#[test]
fn lunar_day_grid_stays_three_by_ten_and_disables_thirty_for_a_short_month() {
    let panel = (0..12)
        .map(|month_index| {
            static_temporal_chart_view(
                sample_request(),
                StaticTemporalNavigationSelection::Monthly {
                    decadal_index: 2,
                    year_index: 0,
                    month_index,
                },
            )
            .expect("monthly selection should build")
            .temporal_panel
        })
        .find(|panel| !panel.day_rows[2][9].enabled)
        .expect("selected lunar year should contain a 29-day month");

    assert_eq!(panel.day_rows.len(), 3);
    assert!(panel.day_rows.iter().all(|row| row.len() == 10));
    assert_eq!(panel.day_rows[0][0].label_zh, "初一");
    assert_eq!(panel.day_rows[2][9].label_zh, "三十");
    assert!(!panel.day_rows[2][9].enabled);
}

#[test]
fn snapshot_with_selection_flags_serializes_round_trip() {
    let snapshot = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::Yearly {
            decadal_index: 1,
            year_index: 3,
        },
    )
    .unwrap();
    let json = serde_json::to_string(&snapshot).expect("serialize");
    let back: iztro::core::StaticChartViewSnapshot =
        serde_json::from_str(&json).expect("deserialize");
    assert_eq!(snapshot, back);
}

#[test]
fn yearly_selection_rejects_out_of_range_year_index() {
    let result = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::Yearly {
            decadal_index: 2,
            year_index: 10,
        },
    );

    assert_eq!(
        result,
        Err(ChartError::InvalidTemporalSelectionIndex {
            field: "year_index",
            value: 10,
            max: 9,
        })
    );
}

#[test]
fn monthly_selection_rejects_out_of_range_month_index() {
    let result = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::Monthly {
            decadal_index: 2,
            year_index: 0,
            month_index: 12,
        },
    );

    assert_eq!(
        result,
        Err(ChartError::InvalidTemporalSelectionIndex {
            field: "month_index",
            value: 12,
            max: 11,
        })
    );
}

#[test]
fn daily_selection_rejects_out_of_range_day_index() {
    let result = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::Daily {
            decadal_index: 2,
            year_index: 0,
            month_index: 0,
            day_index: 30,
        },
    );

    assert_eq!(
        result,
        Err(ChartError::InvalidTemporalSelectionIndex {
            field: "day_index",
            value: 30,
            max: 29,
        })
    );
}

#[test]
fn hourly_selection_rejects_out_of_range_hour_index() {
    let result = static_temporal_chart_view(
        sample_request(),
        StaticTemporalNavigationSelection::Hourly {
            decadal_index: 2,
            year_index: 0,
            month_index: 0,
            day_index: 0,
            hour_index: 12,
        },
    );

    assert_eq!(
        result,
        Err(ChartError::InvalidTemporalSelectionIndex {
            field: "hour_index",
            value: 12,
            max: 11,
        })
    );
}
