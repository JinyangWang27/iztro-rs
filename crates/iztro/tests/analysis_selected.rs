//! Integration tests for the selected-view batch analysis facade.
//!
//! These exercise only the public surface
//! ([`detect_static_temporal_analysis_layers_from_chart`]): empty-input
//! short-circuit, exact key/selection validation, input-order preservation, and
//! per-key layer assignment. A real natal chart is built via [`by_solar`] so the
//! selected horoscope overlay stack can be assembled by core.

use iztro::{
    AnalysisLayerKey, AnalysisLayerRequest, Chart, ChartError, EarthlyBranch, Gender,
    MethodProfile, Scope, SolarChartRequest, SolarDay, SolarMonth,
    StaticTemporalNavigationSelection, by_solar, detect_static_temporal_analysis_layers_from_chart,
};

/// A real natal chart with full facts, so the decadal/yearly/monthly overlay
/// stack can be derived by core for the deeper selections under test.
fn natal_chart() -> Chart {
    let request = SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(5).expect("May is valid"))
        .solar_day(SolarDay::new(17).expect("day 17 is valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .method_profile(MethodProfile::placeholder("analysis_selected_test"))
        .build()
        .expect("solar request should build");
    by_solar(request).expect("by_solar should build the fixture chart")
}

#[test]
fn empty_keys_returns_empty_without_building_context() {
    let natal = natal_chart();
    let results = detect_static_temporal_analysis_layers_from_chart(
        natal,
        StaticTemporalNavigationSelection::Monthly {
            decadal_index: 0,
            year_index: 2,
            month_index: 5,
        },
        &[],
        &AnalysisLayerRequest::user_facing(),
    )
    .expect("empty keys must be Ok");
    assert!(results.is_empty());
}

#[test]
fn results_preserve_input_order() {
    let natal = natal_chart();
    let selection = StaticTemporalNavigationSelection::Yearly {
        decadal_index: 0,
        year_index: 2,
    };
    // Deliberately not natal-outward order.
    let keys = vec![
        AnalysisLayerKey::Yearly {
            decadal_index: 0,
            year_index: 2,
        },
        AnalysisLayerKey::Natal,
        AnalysisLayerKey::Decadal { decadal_index: 0 },
    ];
    let results = detect_static_temporal_analysis_layers_from_chart(
        natal,
        selection,
        &keys,
        &AnalysisLayerRequest::user_facing(),
    )
    .expect("visible keys must be Ok");

    let returned: Vec<AnalysisLayerKey> = results.into_iter().map(|result| result.key).collect();
    assert_eq!(returned, keys, "results must follow input order");
}

#[test]
fn monthly_selection_accepts_every_visible_layer() {
    let natal = natal_chart();
    let selection = StaticTemporalNavigationSelection::Monthly {
        decadal_index: 0,
        year_index: 2,
        month_index: 5,
    };
    let keys = vec![
        AnalysisLayerKey::Natal,
        AnalysisLayerKey::Decadal { decadal_index: 0 },
        AnalysisLayerKey::Age {
            decadal_index: 0,
            year_index: 2,
        },
        AnalysisLayerKey::Yearly {
            decadal_index: 0,
            year_index: 2,
        },
        AnalysisLayerKey::Monthly {
            decadal_index: 0,
            year_index: 2,
            month_index: 5,
        },
    ];
    let results = detect_static_temporal_analysis_layers_from_chart(
        natal,
        selection,
        &keys,
        &AnalysisLayerRequest::user_facing(),
    )
    .expect("all visible layers must be accepted");
    assert_eq!(results.len(), keys.len());
}

#[test]
fn yearly_selection_rejects_a_monthly_key() {
    let natal = natal_chart();
    let selection = StaticTemporalNavigationSelection::Yearly {
        decadal_index: 0,
        year_index: 2,
    };
    let keys = vec![AnalysisLayerKey::Monthly {
        decadal_index: 0,
        year_index: 2,
        month_index: 5,
    }];
    let error = detect_static_temporal_analysis_layers_from_chart(
        natal,
        selection,
        &keys,
        &AnalysisLayerRequest::user_facing(),
    )
    .expect_err("a descendant key must be rejected");
    assert!(matches!(
        error,
        ChartError::AnalysisLayerNotVisibleForSelection { scope } if scope == Scope::Monthly
    ));
}

#[test]
fn mismatched_exact_indexes_are_rejected() {
    let request = AnalysisLayerRequest::user_facing();

    // Selected yearly index differs from the key's yearly index.
    let err = detect_static_temporal_analysis_layers_from_chart(
        natal_chart(),
        StaticTemporalNavigationSelection::Monthly {
            decadal_index: 0,
            year_index: 2,
            month_index: 5,
        },
        &[AnalysisLayerKey::Yearly {
            decadal_index: 0,
            year_index: 3,
        }],
        &request,
    )
    .expect_err("mismatched yearly index must be rejected");
    assert!(matches!(
        err,
        ChartError::AnalysisLayerNotVisibleForSelection { .. }
    ));

    // Selected decadal index differs from the key's decadal index.
    let err = detect_static_temporal_analysis_layers_from_chart(
        natal_chart(),
        StaticTemporalNavigationSelection::Decadal { decadal_index: 1 },
        &[AnalysisLayerKey::Decadal { decadal_index: 2 }],
        &request,
    )
    .expect_err("mismatched decadal index must be rejected");
    assert!(matches!(
        err,
        ChartError::AnalysisLayerNotVisibleForSelection { .. }
    ));

    // Selected monthly index differs from the key's monthly index.
    let err = detect_static_temporal_analysis_layers_from_chart(
        natal_chart(),
        StaticTemporalNavigationSelection::Monthly {
            decadal_index: 0,
            year_index: 2,
            month_index: 5,
        },
        &[AnalysisLayerKey::Monthly {
            decadal_index: 0,
            year_index: 2,
            month_index: 6,
        }],
        &request,
    )
    .expect_err("mismatched monthly index must be rejected");
    assert!(matches!(
        err,
        ChartError::AnalysisLayerNotVisibleForSelection { .. }
    ));
}

#[test]
fn monthly_selection_with_yearly_key_returns_a_yearly_layer() {
    let natal = natal_chart();
    let selection = StaticTemporalNavigationSelection::Monthly {
        decadal_index: 0,
        year_index: 2,
        month_index: 5,
    };
    let yearly_key = AnalysisLayerKey::Yearly {
        decadal_index: 0,
        year_index: 2,
    };
    let results = detect_static_temporal_analysis_layers_from_chart(
        natal,
        selection,
        std::slice::from_ref(&yearly_key),
        &AnalysisLayerRequest::user_facing(),
    )
    .expect("a visible ancestor key must be accepted");

    assert_eq!(results.len(), 1);
    assert_eq!(
        results[0].key, yearly_key,
        "the result key must be the requested yearly key, not the selected monthly key"
    );
}

#[test]
fn facade_does_not_auto_return_unrequested_ancestors() {
    let natal = natal_chart();
    let selection = StaticTemporalNavigationSelection::Monthly {
        decadal_index: 0,
        year_index: 2,
        month_index: 5,
    };
    // Request only the deepest layer; ancestors must not be added.
    let monthly_key = AnalysisLayerKey::Monthly {
        decadal_index: 0,
        year_index: 2,
        month_index: 5,
    };
    let results = detect_static_temporal_analysis_layers_from_chart(
        natal,
        selection,
        std::slice::from_ref(&monthly_key),
        &AnalysisLayerRequest::user_facing(),
    )
    .expect("the deepest visible key must be accepted");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].key, monthly_key);
}
