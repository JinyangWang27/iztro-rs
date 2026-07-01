//! Projection-level invariants for the active palace frame.
//!
//! First principle: a branch is the stable palace-cell coordinate; a palace name
//! is frame-relative. Selecting a temporal scope re-titles the branch ring with
//! that scope's palace-name frame, while the natal identity stays immutable and
//! the temporal facts stay attached as overlays.

use iztro::core::pattern::query::branch_of_palace_for_scope;
use iztro::core::{EarthlyBranch, PalaceName, Scope};
use iztro::{
    BirthTime, ChartAlgorithmKind, ChartError, DecadalHoroscopeInput, Gender, HoroscopeChart,
    MethodProfile, PatternContext, SolarChartRequest, SolarDay, SolarMonth, StaticChartProjection,
    StaticChartProjectionRequest, StaticTemporalNavigationSelection, build_decadal_horoscope_chart,
    by_solar, static_temporal_chart_view,
};

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
            "static projection active frame test",
        ))
        .build()
        .unwrap()
}

/// The natal Life-palace branch for the spec chart (the immutable reference).
fn natal_life_branch() -> EarthlyBranch {
    by_solar(spec_request())
        .unwrap()
        .branch_of_palace(PalaceName::Life)
        .expect("natal chart has a Life palace")
}

/// A representative drill-down selection that reaches `scope`.
fn selection_for(scope: Scope) -> StaticTemporalNavigationSelection {
    match scope {
        Scope::Decadal => StaticTemporalNavigationSelection::Decadal { decadal_index: 1 },
        Scope::Yearly => StaticTemporalNavigationSelection::Yearly {
            decadal_index: 1,
            year_index: 0,
        },
        Scope::Monthly => StaticTemporalNavigationSelection::Monthly {
            decadal_index: 1,
            year_index: 0,
            month_index: 0,
        },
        Scope::Daily => StaticTemporalNavigationSelection::Daily {
            decadal_index: 1,
            year_index: 0,
            month_index: 0,
            day_index: 0,
        },
        Scope::Hourly => StaticTemporalNavigationSelection::Hourly {
            decadal_index: 1,
            year_index: 0,
            month_index: 0,
            day_index: 0,
            hour_index: 0,
        },
        other => panic!("selection_for does not cover {other:?}"),
    }
}

/// The single active-frame Life-palace branch in a projection (asserts exactly
/// one exists and that its name and flag are consistent).
fn active_life_branch(projection: &StaticChartProjection) -> EarthlyBranch {
    let life: Vec<&_> = projection
        .palaces
        .iter()
        .filter(|p| p.active_frame.is_life_palace)
        .collect();
    assert_eq!(
        life.len(),
        1,
        "exactly one palace is the active-frame Life palace",
    );
    assert_eq!(life[0].active_frame.palace_name, PalaceName::Life);
    life[0].branch
}

#[test]
fn projection_exposes_active_life_branch_helper() {
    let natal = by_solar(spec_request()).unwrap();
    let natal_projection = StaticChartProjection::from_chart(&natal);
    assert_eq!(
        natal_projection.active_life_branch(),
        natal_life_branch(),
        "natal projection helper returns the natal Life palace"
    );

    let pre_decadal = static_temporal_chart_view(
        spec_request(),
        StaticTemporalNavigationSelection::PreDecadal,
    )
    .unwrap();
    assert_eq!(
        pre_decadal.active_life_branch(),
        natal_life_branch(),
        "pre-decadal projection still uses the natal frame"
    );

    for scope in [
        Scope::Decadal,
        Scope::Yearly,
        Scope::Monthly,
        Scope::Daily,
        Scope::Hourly,
    ] {
        let projection = static_temporal_chart_view(spec_request(), selection_for(scope)).unwrap();
        assert_eq!(
            projection.active_life_branch(),
            active_life_branch(&projection),
            "{scope:?} projection helper returns the active-frame Life palace"
        );
    }
}

/// The single natal-identity Life-palace branch in a projection.
fn natal_identity_life_branch(projection: &StaticChartProjection) -> EarthlyBranch {
    let life: Vec<&_> = projection
        .palaces
        .iter()
        .filter(|p| p.natal_identity.palace_name == PalaceName::Life)
        .collect();
    assert_eq!(life.len(), 1, "exactly one palace is the natal Life palace");
    life[0].branch
}

#[test]
fn each_selected_scope_sets_its_active_frame_and_relocates_the_life_palace() {
    let natal_life = natal_life_branch();
    let scopes = [
        Scope::Decadal,
        Scope::Yearly,
        Scope::Monthly,
        Scope::Daily,
        Scope::Hourly,
    ];

    let mut any_relocated = false;
    for scope in scopes {
        let projection = static_temporal_chart_view(spec_request(), selection_for(scope)).unwrap();

        // 1. Every palace carries the selected scope as its active frame.
        for palace in &projection.palaces {
            assert_eq!(
                palace.active_frame.frame_scope, scope,
                "every palace's active frame is {scope:?}",
            );
        }

        // 2 & 3. Exactly one active Life palace, and it sits on the branch the
        // selected layer's TemporalPalaceLayout relabels as 命宫 — surfaced here
        // through that same scope's overlay `temporal_palace_name`.
        let active_life = active_life_branch(&projection);
        let overlay_life: Vec<EarthlyBranch> = projection
            .palaces
            .iter()
            .filter(|p| {
                p.overlays
                    .iter()
                    .any(|o| o.scope == scope && o.temporal_palace_name == Some(PalaceName::Life))
            })
            .map(|p| p.branch)
            .collect();
        assert_eq!(
            overlay_life,
            vec![active_life],
            "active Life branch equals the {scope:?} layer layout's Life branch",
        );

        // 4. Natal identity is immutable: its Life palace stays on the natal
        // branch regardless of the selected scope.
        assert_eq!(
            natal_identity_life_branch(&projection),
            natal_life,
            "natal identity Life branch is immutable under a {scope:?} selection",
        );

        if active_life != natal_life {
            any_relocated = true;
        }
    }

    assert!(
        any_relocated,
        "at least one temporal frame relabels 命宫 onto a different branch than natal",
    );
}

#[test]
fn yearly_selection_uses_the_yearly_frame_not_age() {
    let projection = static_temporal_chart_view(
        spec_request(),
        StaticTemporalNavigationSelection::Yearly {
            decadal_index: 1,
            year_index: 0,
        },
    )
    .unwrap();

    // The active frame is 流年, never 小限, even though 小限 is computed for the year.
    for palace in &projection.palaces {
        assert_eq!(palace.active_frame.frame_scope, Scope::Yearly);
        assert_ne!(palace.active_frame.frame_scope, Scope::Age);
    }

    // 小限 (Age) stays visible as auxiliary data alongside 流年.
    assert!(
        projection.active_scopes.contains(&Scope::Yearly),
        "流年 is visible",
    );
    assert!(
        projection.active_scopes.contains(&Scope::Age),
        "小限 remains visible as auxiliary data for a 流年 view",
    );
}

#[test]
fn projection_active_life_matches_scope_aware_query_not_natal_lookup() {
    let natal = by_solar(spec_request()).unwrap();
    let horoscope =
        build_decadal_horoscope_chart(natal, DecadalHoroscopeInput { period_index: 1 }).unwrap();

    // The scope-aware domain query is the source of truth for a non-natal Life
    // branch; a raw natal lookup means a different (natal) branch.
    let ctx = PatternContext::horoscope(&horoscope, vec![Scope::Natal, Scope::Decadal]);
    let decadal_life = branch_of_palace_for_scope(&ctx, Scope::Decadal, PalaceName::Life)
        .expect("decadal Life branch");
    let natal_life = horoscope
        .natal()
        .branch_of_palace(PalaceName::Life)
        .expect("natal Life branch");
    assert_ne!(
        decadal_life, natal_life,
        "the 大限 Life palace relabels a different branch than natal",
    );

    // The projection's active frame must agree with the scope-aware query, not
    // with the raw natal lookup.
    let projection = StaticChartProjection::from_horoscope_chart_with(
        &horoscope,
        &StaticChartProjectionRequest {
            visible_scopes: vec![Scope::Natal, Scope::Decadal],
            active_frame_scope: Scope::Decadal,
        },
    )
    .unwrap();
    assert_eq!(
        active_life_branch(&projection),
        decadal_life,
        "active Decadal frame Life branch equals branch_of_palace_for_scope(Decadal, Life)",
    );
    assert_eq!(
        natal_identity_life_branch(&projection),
        natal_life,
        "natal identity still reports the natal Life branch",
    );
}

#[test]
fn missing_active_frame_layer_fails_loudly() {
    // A visible non-natal active frame with no matching temporal layer must error
    // rather than fall back to natal names.
    let natal = by_solar(spec_request()).unwrap();
    let horoscope = HoroscopeChart::new(natal);
    let result = StaticChartProjection::from_horoscope_chart_with(
        &horoscope,
        &StaticChartProjectionRequest {
            visible_scopes: vec![Scope::Natal, Scope::Decadal],
            active_frame_scope: Scope::Decadal,
        },
    );
    assert_eq!(
        result,
        Err(ChartError::MissingHoroscopeLayer {
            scope: Scope::Decadal,
        }),
    );
}

#[test]
fn active_frame_scope_must_be_visible() {
    // A non-natal active frame that is not requested as visible is rejected: the
    // projection refuses to title palaces from a frame the request does not also
    // mark visible, rather than silently adding the scope.
    let natal = by_solar(spec_request()).unwrap();
    let horoscope =
        build_decadal_horoscope_chart(natal, DecadalHoroscopeInput { period_index: 1 }).unwrap();
    let result = StaticChartProjection::from_horoscope_chart_with(
        &horoscope,
        &StaticChartProjectionRequest {
            visible_scopes: vec![Scope::Natal],
            active_frame_scope: Scope::Decadal,
        },
    );
    assert_eq!(
        result,
        Err(ChartError::ActiveFrameScopeNotVisible {
            scope: Scope::Decadal,
        }),
    );
}

#[test]
fn duplicate_active_frame_layers_fail_loudly() {
    // Two temporal layers of the same active scope make the active frame
    // ambiguous, so resolution fails loudly instead of picking the first.
    let natal = by_solar(spec_request()).unwrap();
    let decadal =
        build_decadal_horoscope_chart(natal.clone(), DecadalHoroscopeInput { period_index: 1 })
            .unwrap();
    let decadal_layer = decadal
        .layers()
        .iter()
        .find(|layer| layer.scope() == Scope::Decadal)
        .expect("decadal horoscope carries a decadal layer")
        .clone();

    let mut horoscope = HoroscopeChart::new(natal);
    horoscope.push_layer(decadal_layer.clone());
    horoscope.push_layer(decadal_layer);

    let result = StaticChartProjection::from_horoscope_chart_with(
        &horoscope,
        &StaticChartProjectionRequest {
            visible_scopes: vec![Scope::Natal, Scope::Decadal],
            active_frame_scope: Scope::Decadal,
        },
    );
    assert_eq!(
        result,
        Err(ChartError::DuplicateHoroscopeLayer {
            scope: Scope::Decadal,
        }),
    );
}
