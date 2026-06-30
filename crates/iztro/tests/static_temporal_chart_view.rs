//! Facade tests for `static_temporal_chart_view`.
//!
//! These verify that all temporal-overlay derivation stays inside core: a GUI
//! passes a [`SolarChartRequest`] plus a renderer-neutral
//! [`StaticTemporalNavigationSelection`] and gets back a prepared
//! [`StaticChartProjection`]. Selecting a temporal cell changes overlays only,
//! never natal facts.

use iztro::core::{
    BirthTime, ChartAlgorithmKind, ChartError, Gender, MethodProfile, Scope, SolarChartRequest,
    SolarDay, SolarMonth, by_solar,
};
use iztro::{
    StaticTemporalNavigationSelection, static_temporal_chart_view,
    static_temporal_chart_view_from_chart, temporal_selection_for_local_moment,
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

/// The spec reference birth data: solar 1993-05-27, 酉 hour (timeIndex 9), male.
fn spec_request() -> SolarChartRequest {
    SolarChartRequest::builder()
        .solar_year(1993)
        .solar_month(SolarMonth::new(5).unwrap())
        .solar_day(SolarDay::new(27).unwrap())
        .birth_time_variant(BirthTime::from_iztro_time_index(9).unwrap())
        .gender(Gender::Male)
        .method_profile(MethodProfile::new(
            "iztro_test",
            ChartAlgorithmKind::QuanShu,
            "static temporal chart view spec test",
        ))
        .build()
        .unwrap()
}

#[test]
fn center_carries_prepared_iztro_style_natal_labels() {
    let center =
        static_temporal_chart_view(spec_request(), StaticTemporalNavigationSelection::Natal)
            .unwrap()
            .center;

    assert_eq!(center.five_element_bureau_zh.as_deref(), Some("木三局"));
    assert_eq!(center.birth_solar_label, "1993-05-27");
    assert_eq!(center.birth_lunar_label, "一九九三年四月初七");
    assert_eq!(center.birth_time_label, "酉时(17:00~19:00)");
    assert_eq!(center.zodiac_zh, "鸡");
    assert_eq!(center.constellation_zh, "双子座");
    assert!(center.soul_master_zh.is_some(), "命主 should be prepared");
    assert!(center.body_master_zh.is_some(), "身主 should be prepared");

    // The additive typed fields mirror the prepared Chinese labels so a
    // presentation layer can render them in any locale.
    assert_eq!(
        center.birth_lunar_date,
        Some(iztro::LunarDateProjection {
            year: 1993,
            month: 4,
            day: 7,
            is_leap_month: false,
        }),
    );
    assert_eq!(center.birth_time_index, Some(9));
    assert_eq!(
        center.western_zodiac,
        Some(iztro::core::WesternZodiac::Gemini)
    );
    assert!(center.soul_master.is_some(), "命主 star should be typed");
    assert!(center.body_master.is_some(), "身主 star should be typed");
}

#[test]
fn nominal_age_and_run_xian_dates_update_with_navigation() {
    let natal =
        static_temporal_chart_view(spec_request(), StaticTemporalNavigationSelection::Natal)
            .unwrap();
    assert!(natal.center.nominal_age_label.is_none());
    assert!(natal.center.temporal_solar_label.is_none());
    assert!(natal.center.nominal_age.is_none());
    assert!(natal.center.temporal_lunar_date.is_none());
    assert!(natal.center.temporal_lunar_year.is_none());

    // Resolve "today" → a Hourly selection, then render it.
    let chart = by_solar(spec_request()).unwrap();
    let selection = temporal_selection_for_local_moment(&chart, 2008, 2, 10, 10, 0).unwrap();
    let snapshot = static_temporal_chart_view(spec_request(), selection).unwrap();

    // 2008 is the nominal 16th year for a 1993 birth (虚岁).
    assert_eq!(snapshot.center.nominal_age_label.as_deref(), Some("16 岁"));
    assert_eq!(snapshot.center.nominal_age, Some(16));
    assert_eq!(
        snapshot.center.temporal_solar_label.as_deref(),
        Some("2008-2-10"),
        "运限阳历 should use iztro-style unpadded month/day"
    );
    assert!(snapshot.center.temporal_lunar_label.is_some());
    // A concrete-day (流时) selection also exposes the typed run-limit lunar date.
    assert!(snapshot.center.temporal_lunar_date.is_some());
}

#[test]
fn today_at_23_preserves_late_zi_time_index_and_selects_the_zi_cell() {
    let chart = by_solar(spec_request()).unwrap();
    let selection = temporal_selection_for_local_moment(&chart, 2008, 2, 10, 23, 30).unwrap();

    assert!(matches!(
        selection,
        StaticTemporalNavigationSelection::Hourly { hour_index: 12, .. }
    ));

    let snapshot = static_temporal_chart_view(spec_request(), selection).unwrap();
    assert_eq!(
        snapshot
            .temporal_panel
            .hour_cells
            .iter()
            .filter(|cell| cell.selected)
            .map(|cell| cell.label_zh.as_str())
            .collect::<Vec<_>>(),
        vec!["子"],
        "late Zi keeps timeIndex 12 while rendering in the Zi branch cell"
    );
    assert!(
        snapshot
            .palaces
            .iter()
            .flat_map(|palace| palace.overlays.iter())
            .any(|overlay| overlay.scope == Scope::Hourly),
        "the late-Zi selection must build the hourly target overlay"
    );
}

#[test]
fn selecting_a_decadal_marks_exactly_one_active_palace() {
    let snapshot = static_temporal_chart_view(
        spec_request(),
        StaticTemporalNavigationSelection::Decadal { decadal_index: 1 },
    )
    .unwrap();

    let active: Vec<_> = snapshot
        .palaces
        .iter()
        .filter(|p| p.limit.is_active_decadal)
        .collect();
    assert_eq!(active.len(), 1, "exactly one palace is the active 大限");
    // The natal snapshot marks none.
    let natal =
        static_temporal_chart_view(spec_request(), StaticTemporalNavigationSelection::Natal)
            .unwrap();
    assert!(natal.palaces.iter().all(|p| !p.limit.is_active_decadal));
}

#[test]
fn every_palace_carries_a_decadal_age_range() {
    let snapshot =
        static_temporal_chart_view(spec_request(), StaticTemporalNavigationSelection::Natal)
            .unwrap();
    assert!(
        snapshot
            .palaces
            .iter()
            .all(|p| p.limit.decadal_age_range_zh.is_some()),
        "each palace has a prepared 大限 age range"
    );
    assert!(
        snapshot
            .palaces
            .iter()
            .all(|p| !p.limit.small_limit_ages_zh.is_empty()),
        "each palace has prepared 小限 ages"
    );
}

/// Counts, across all palaces, the overlays of `scope` whose `period_label_zh`
/// marks the palace as that scope's period anchor.
fn marker_count(snapshot: &iztro::StaticChartProjection, scope: Scope) -> usize {
    snapshot
        .palaces
        .iter()
        .flat_map(|p| p.overlays.iter())
        .filter(|o| o.scope == scope && o.period_label_zh.is_some())
        .count()
}

#[test]
fn temporal_overlay_carries_a_period_label_only_on_the_marker_palace() {
    let snapshot = static_temporal_chart_view(
        spec_request(),
        StaticTemporalNavigationSelection::Yearly {
            decadal_index: 1,
            year_index: 0,
        },
    )
    .unwrap();

    // Exactly one palace carries the 流年 period marker.
    let markers: Vec<_> = snapshot
        .palaces
        .iter()
        .flat_map(|p| p.overlays.iter())
        .filter(|o| o.scope == Scope::Yearly && o.period_label_zh.is_some())
        .collect();
    assert_eq!(
        markers.len(),
        1,
        "the 流年 marker sits on exactly one palace"
    );
    assert!(
        markers[0]
            .period_label_zh
            .as_deref()
            .unwrap()
            .starts_with("流年·"),
        "got {:?}",
        markers[0].period_label_zh
    );

    // The 流年 overlay still attaches to multiple palaces (stars/mutagens),
    // but only one of them is the marker.
    let overlay_palaces = snapshot
        .palaces
        .iter()
        .filter(|p| p.overlays.iter().any(|o| o.scope == Scope::Yearly))
        .count();
    assert!(
        overlay_palaces > 1,
        "the 流年 overlay spans more than one palace, but only one is a marker"
    );
}

#[test]
fn decadal_selection_marks_exactly_one_period_palace() {
    let snapshot = static_temporal_chart_view(
        spec_request(),
        StaticTemporalNavigationSelection::Decadal { decadal_index: 1 },
    )
    .unwrap();
    assert_eq!(marker_count(&snapshot, Scope::Decadal), 1);
}

#[test]
fn yearly_selection_marks_exactly_one_period_palace() {
    let snapshot = static_temporal_chart_view(
        spec_request(),
        StaticTemporalNavigationSelection::Yearly {
            decadal_index: 1,
            year_index: 0,
        },
    )
    .unwrap();
    assert_eq!(marker_count(&snapshot, Scope::Yearly), 1);
}

#[test]
fn hourly_selection_marks_exactly_one_palace_per_active_scope() {
    let snapshot = static_temporal_chart_view(
        spec_request(),
        StaticTemporalNavigationSelection::Hourly {
            decadal_index: 1,
            year_index: 0,
            month_index: 0,
            day_index: 0,
            hour_index: 6,
        },
    )
    .unwrap();

    for scope in [
        Scope::Decadal,
        Scope::Age,
        Scope::Yearly,
        Scope::Monthly,
        Scope::Daily,
        Scope::Hourly,
    ] {
        assert_eq!(
            marker_count(&snapshot, scope),
            1,
            "{scope:?} must mark exactly one palace"
        );
    }

    // Regression: no active temporal scope may stamp a marker on all 12 palaces.
    assert!(
        snapshot
            .palaces
            .iter()
            .flat_map(|p| p.overlays.iter())
            .filter(|o| o.period_label_zh.is_some())
            .count()
            < snapshot.palaces.len(),
        "period markers must never appear on every palace"
    );
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

    // Natal center facts are identical regardless of temporal selection; only
    // the navigation-dependent labels (年龄(虚岁), 运限农历/阳历) may differ.
    let stable = |center: &iztro::StaticChartCenterProjection| {
        let mut center = center.clone();
        center.nominal_age_label = None;
        center.temporal_lunar_label = None;
        center.temporal_solar_label = None;
        center.nominal_age = None;
        center.temporal_lunar_date = None;
        center.temporal_lunar_year = None;
        center
    };
    assert_eq!(stable(&natal.center), stable(&decadal.center));

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
    let back: iztro::StaticChartProjection = serde_json::from_str(&json).expect("deserialize");
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
fn hourly_selection_accepts_late_zi_time_index() {
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

    assert!(result.is_ok(), "timeIndex 12 is late Zi and must be valid");
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
            hour_index: 13,
        },
    );

    assert_eq!(
        result,
        Err(ChartError::InvalidTemporalSelectionIndex {
            field: "hour_index",
            value: 13,
            max: 12,
        })
    );
}

/// The branch of the single palace carrying the 流年 (`Scope::Yearly`) period
/// marker for `snapshot`, if any.
fn yearly_marker_branch(
    snapshot: &iztro::StaticChartProjection,
) -> Option<iztro::core::EarthlyBranch> {
    snapshot
        .palaces
        .iter()
        .find(|p| {
            p.overlays
                .iter()
                .any(|o| o.scope == Scope::Yearly && o.period_label_zh.is_some())
        })
        .map(|p| p.branch)
}

#[test]
fn yearly_selection_marks_exactly_one_active_small_limit() {
    let snapshot = static_temporal_chart_view(
        spec_request(),
        StaticTemporalNavigationSelection::Yearly {
            decadal_index: 1,
            year_index: 0,
        },
    )
    .unwrap();

    // The center exposes the selected nominal age and the 小限 palace branch.
    assert!(snapshot.center.nominal_age.is_some());
    assert_eq!(snapshot.center.small_limit_age, snapshot.center.nominal_age);
    let branch = snapshot
        .center
        .small_limit_branch
        .expect("a concrete year resolves an active 小限 branch");

    // Exactly one palace is the active 小限, and it is the branch the center
    // reports; that palace carries the selected nominal age.
    let active: Vec<_> = snapshot
        .palaces
        .iter()
        .filter(|p| p.limit.is_active_small_limit)
        .collect();
    assert_eq!(active.len(), 1, "exactly one palace holds the active 小限");
    assert_eq!(active[0].branch, branch);
    assert_eq!(
        active[0].limit.active_small_limit_age,
        snapshot.center.nominal_age
    );

    // Every other palace is inactive and carries no active-age stamp.
    for palace in snapshot.palaces.iter().filter(|p| p.branch != branch) {
        assert!(!palace.limit.is_active_small_limit);
        assert!(palace.limit.active_small_limit_age.is_none());
    }
}

#[test]
fn small_limit_is_absent_without_a_selected_year() {
    for selection in [
        StaticTemporalNavigationSelection::Natal,
        StaticTemporalNavigationSelection::PreDecadal,
        StaticTemporalNavigationSelection::Decadal { decadal_index: 1 },
    ] {
        let snapshot = static_temporal_chart_view(spec_request(), selection).unwrap();
        assert!(
            snapshot.center.small_limit_age.is_none(),
            "{selection:?} has no selected year, so no 小限 age"
        );
        assert!(snapshot.center.small_limit_branch.is_none());
        assert!(
            snapshot
                .palaces
                .iter()
                .all(|p| !p.limit.is_active_small_limit
                    && p.limit.active_small_limit_age.is_none()),
            "{selection:?} marks no active 小限 palace"
        );
    }
}

#[test]
fn small_limit_and_yearly_markers_are_not_conflated() {
    // 小限 (age-based) and 流年 (太岁-based) are independent markers. For at least
    // one 流年 within the decadal period the two land on different palaces, and
    // when they do they coexist without overwriting each other.
    let distinct = (0u8..=9)
        .map(|year_index| {
            static_temporal_chart_view(
                spec_request(),
                StaticTemporalNavigationSelection::Yearly {
                    decadal_index: 1,
                    year_index,
                },
            )
            .unwrap()
        })
        .find(|snapshot| {
            let small = snapshot.center.small_limit_branch;
            let yearly = yearly_marker_branch(snapshot);
            matches!((small, yearly), (Some(s), Some(y)) if s != y)
        })
        .expect("some 流年 in the period lands on a different palace than its 小限");

    let small_branch = distinct.center.small_limit_branch.unwrap();
    let yearly_branch = yearly_marker_branch(&distinct).unwrap();
    assert_ne!(
        small_branch, yearly_branch,
        "小限 branch and 流年 marker branch must be distinct here"
    );

    // The 流年 marker palace is not flagged as the active 小限, and the active
    // 小限 palace does not carry the 流年 period marker: both coexist.
    let yearly_palace = distinct
        .palaces
        .iter()
        .find(|p| p.branch == yearly_branch)
        .unwrap();
    assert!(!yearly_palace.limit.is_active_small_limit);

    let small_palace = distinct
        .palaces
        .iter()
        .find(|p| p.branch == small_branch)
        .unwrap();
    assert!(small_palace.limit.is_active_small_limit);
    assert!(
        !small_palace
            .overlays
            .iter()
            .any(|o| o.scope == Scope::Yearly && o.period_label_zh.is_some()),
        "the active 小限 palace must not also be the 流年 period marker"
    );
}

#[test]
fn snapshot_without_small_limit_fields_still_deserializes() {
    // A legacy snapshot JSON predating the typed 小限 fields must still load,
    // thanks to #[serde(default)] on every new field.
    let snapshot = static_temporal_chart_view(
        spec_request(),
        StaticTemporalNavigationSelection::Yearly {
            decadal_index: 1,
            year_index: 0,
        },
    )
    .unwrap();
    let mut value = serde_json::to_value(&snapshot).unwrap();
    // Strip the new center + palace fields to emulate an older serialization.
    let center = value
        .get_mut("center")
        .and_then(|c| c.as_object_mut())
        .unwrap();
    center.remove("small_limit_age");
    center.remove("small_limit_branch");
    for palace in value
        .get_mut("palaces")
        .and_then(|p| p.as_array_mut())
        .unwrap()
    {
        let limit = palace
            .get_mut("limit")
            .and_then(|l| l.as_object_mut())
            .unwrap();
        limit.remove("small_limit_ages");
        limit.remove("is_active_small_limit");
        limit.remove("active_small_limit_age");
    }
    let restored: iztro::StaticChartProjection =
        serde_json::from_value(value).expect("legacy snapshot without 小限 fields deserializes");
    assert!(restored.center.small_limit_age.is_none());
    assert!(
        restored
            .palaces
            .iter()
            .all(|p| !p.limit.is_active_small_limit && p.limit.small_limit_ages.is_empty())
    );
}

// ---- from_chart helper equivalence ----------------------------------------

/// Building the snapshot from a pre-built natal `Chart` must be byte-for-byte
/// identical to the request-driven entry point, so a GUI can build the chart
/// once and derive both the snapshot and the rule panel from it.
#[test]
fn from_chart_matches_request_path_for_natal_selection() {
    let selection = StaticTemporalNavigationSelection::Natal;
    let via_request = static_temporal_chart_view(spec_request(), selection).unwrap();
    let via_chart =
        static_temporal_chart_view_from_chart(by_solar(spec_request()).unwrap(), selection)
            .unwrap();
    assert_eq!(via_request, via_chart);
}

#[test]
fn from_chart_matches_request_path_for_pre_decadal_selection() {
    let selection = StaticTemporalNavigationSelection::PreDecadal;
    let via_request = static_temporal_chart_view(spec_request(), selection).unwrap();
    let via_chart =
        static_temporal_chart_view_from_chart(by_solar(spec_request()).unwrap(), selection)
            .unwrap();
    assert_eq!(via_request, via_chart);
}

#[test]
fn from_chart_matches_request_path_for_decadal_selection() {
    let selection = StaticTemporalNavigationSelection::Decadal { decadal_index: 0 };
    let via_request = static_temporal_chart_view(sample_request(), selection).unwrap();
    let via_chart =
        static_temporal_chart_view_from_chart(by_solar(sample_request()).unwrap(), selection)
            .unwrap();
    assert_eq!(via_request, via_chart);
}
