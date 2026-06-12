use iztro_core::{
    Chart, ChartAlgorithmKind, ChartError, ChartLayerKind, ChartStackSnapshot,
    DecadalHoroscopeInput, EarthlyBranch, Gender, HoroscopeChart, MethodProfile, Scope,
    SolarChartRequest, SolarDay, SolarMonth, TemporalContext, build_decadal_frame,
    build_decadal_horoscope_chart, by_solar,
};

const PERIOD_INDEX: usize = 1;

fn solar_fixture_chart() -> Chart {
    let request = SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(5).expect("May should be valid"))
        .solar_day(SolarDay::new(17).expect("day 17 should be valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .method_profile(MethodProfile::new(
            "decadal_horoscope_test",
            ChartAlgorithmKind::QuanShu,
            "decadal horoscope test",
        ))
        .build()
        .expect("solar chart request should build");

    by_solar(request).expect("by_solar should build fixture chart")
}

#[test]
fn decadal_horoscope_valid_assembly_composes_one_decadal_layer_without_mutating_natal_stars() {
    let chart = solar_fixture_chart();
    let frame = build_decadal_frame(&chart).expect("decadal frame should build");
    let period = &frame.periods()[PERIOD_INDEX];
    let natal_star_count = chart.stars().len();

    let horoscope = build_decadal_horoscope_chart(
        chart.clone(),
        DecadalHoroscopeInput {
            period_index: PERIOD_INDEX,
        },
    )
    .expect("decadal horoscope should build");

    assert_eq!(horoscope.layers().len(), 1);

    let layer = &horoscope.layers()[0];
    assert_eq!(layer.scope(), Scope::Decadal);
    assert_eq!(
        *layer.context(),
        TemporalContext::Decadal {
            stem_branch: period.stem_branch(),
            start_age: period.start_age(),
        }
    );
    assert!(!layer.placements().is_empty());
    assert!(
        layer
            .placements()
            .iter()
            .all(|placement| placement.scope() == Scope::Decadal)
    );
    assert!(
        layer
            .placements()
            .iter()
            .all(|placement| placement.scope() != Scope::Natal)
    );
    assert!(!layer.activations().is_empty());
    assert!(
        layer
            .activations()
            .iter()
            .all(|activation| activation.source_scope() == Scope::Decadal)
    );

    assert_eq!(chart.stars().len(), natal_star_count);
    assert_eq!(horoscope.natal().stars().len(), natal_star_count);
}

#[test]
fn decadal_horoscope_invalid_index_reports_index_and_period_count() {
    let chart = solar_fixture_chart();
    let frame = build_decadal_frame(&chart).expect("decadal frame should build");
    let index = frame.periods().len();

    let error = build_decadal_horoscope_chart(
        chart,
        DecadalHoroscopeInput {
            period_index: index,
        },
    )
    .expect_err("out-of-range period index should fail");

    assert_eq!(
        error,
        ChartError::InvalidDecadalPeriodIndex {
            index,
            len: frame.periods().len(),
        }
    );
}

#[test]
fn decadal_horoscope_snapshot_has_natal_then_decadal_layers_without_natal_payload_duplication() {
    let chart = solar_fixture_chart();
    let frame = build_decadal_frame(&chart).expect("decadal frame should build");
    let period = &frame.periods()[PERIOD_INDEX];
    let horoscope = build_decadal_horoscope_chart(
        chart,
        DecadalHoroscopeInput {
            period_index: PERIOD_INDEX,
        },
    )
    .expect("decadal horoscope should build");

    let snapshot = ChartStackSnapshot::from_horoscope_chart(&horoscope);

    assert_eq!(snapshot.layers().len(), 2);
    assert_eq!(snapshot.layers()[0].kind(), ChartLayerKind::Natal);

    let decadal = &snapshot.layers()[1];
    assert_eq!(decadal.kind(), ChartLayerKind::Decadal);
    assert_eq!(decadal.z_index(), 1);
    assert_eq!(
        decadal.context(),
        Some(&TemporalContext::Decadal {
            stem_branch: period.stem_branch(),
            start_age: period.start_age(),
        })
    );

    assert!(
        decadal
            .cells()
            .iter()
            .all(|cell| cell.typed_stars().is_empty())
    );
    assert!(
        decadal
            .cells()
            .iter()
            .all(|cell| cell.decorative_stars().is_empty())
    );
    assert!(
        decadal
            .cells()
            .iter()
            .flat_map(|cell| cell.scoped_stars())
            .any(|star| star.scope() == Scope::Decadal)
    );
    assert!(
        decadal
            .cells()
            .iter()
            .flat_map(|cell| cell.mutagen_activations())
            .any(|activation| activation.source_scope() == Scope::Decadal)
    );
}

#[test]
fn decadal_horoscope_round_trips_through_json_with_decadal_layer_invariants() {
    let chart = solar_fixture_chart();
    let horoscope = build_decadal_horoscope_chart(
        chart,
        DecadalHoroscopeInput {
            period_index: PERIOD_INDEX,
        },
    )
    .expect("decadal horoscope should build");

    let encoded = serde_json::to_string(&horoscope).expect("horoscope should serialize");
    let decoded: HoroscopeChart =
        serde_json::from_str(&encoded).expect("horoscope should deserialize");

    assert_eq!(decoded, horoscope);
    assert_eq!(decoded.layers().len(), 1);
    assert_eq!(decoded.layers()[0].scope(), Scope::Decadal);
    assert!(!decoded.layers()[0].placements().is_empty());
    assert!(!decoded.layers()[0].activations().is_empty());
    assert!(
        decoded.layers()[0]
            .placements()
            .iter()
            .all(|placement| placement.scope() == Scope::Decadal)
    );
    assert!(
        decoded.layers()[0]
            .activations()
            .iter()
            .all(|activation| activation.source_scope() == Scope::Decadal)
    );
}
