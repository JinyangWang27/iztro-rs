//! Facade tests for `static_temporal_chart_view`.
//!
//! These verify that all temporal-overlay derivation stays inside core: a GUI
//! passes a [`SolarChartRequest`] plus a renderer-neutral
//! [`StaticTemporalNavigationSelection`] and gets back a prepared
//! [`StaticChartViewSnapshot`]. Selecting a temporal cell changes overlays only,
//! never natal facts.

use iztro::core::{
    BirthTime, ChartAlgorithmKind, Gender, MethodProfile, Scope, SolarChartRequest, SolarDay,
    SolarMonth, StaticTemporalNavigationSelection, static_temporal_chart_view,
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
        StaticTemporalNavigationSelection::Decadal { index: 1 },
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
        StaticTemporalNavigationSelection::Decadal { index: 999 },
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
        StaticTemporalNavigationSelection::Decadal { index: 0 },
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
