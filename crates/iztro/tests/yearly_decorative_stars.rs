//! Fixture-driven tests for yearly-scope temporal decorative facts
//! (`yearlyDecStar`: 岁前/将前十二神).
//!
//! These assert that the yearly horoscope layer carries `yearlyDecStar` as
//! branch-keyed untyped decorative facts scoped to [`Scope::Yearly`], that the
//! full stack exposes them only on the yearly layer, that snapshots keep them
//! separate from natal decorative facts, and that supporting them does not change
//! the typed-star metadata counts.

mod common;

use std::collections::HashMap;

use common::{
    assert_metadata_counts, build_chart_from_horoscope_fixture_case, expected_yearly_dec_stars,
    horoscope_fixture_case, horoscope_fixture_cases, target_solar_date, target_time,
};
use iztro::core::{
    ChartLayerKind, ChartStackSnapshot, DecorativeStarFamily, EarthlyBranch, HoroscopeChart,
    HoroscopeStackInput, Scope, SolarDay, SolarMonth, StarName, TemporalLayer,
    build_full_horoscope_chart, build_yearly_horoscope_layer, build_yearly_period,
};
use serde_json::Value;

const CANONICAL_CASE_ID: &str = "canonical_female_default_2026";

/// The five temporal scopes that must never carry `yearlyDecStar`.
const NON_YEARLY_LAYERS: [ChartLayerKind; 5] = [
    ChartLayerKind::Decadal,
    ChartLayerKind::Age,
    ChartLayerKind::Monthly,
    ChartLayerKind::Daily,
    ChartLayerKind::Hourly,
];

// --- A. Yearly layer includes yearlyDecStar across all fixture cases -----------

#[test]
fn yearly_layer_includes_yearly_dec_stars_for_all_cases() {
    for case in horoscope_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let yearly = &case["supported_fields"]["yearly"];
        let target_year = case["input"]["target"]["year"]
            .as_i64()
            .expect("target year") as i32;
        let period = build_yearly_period(target_year).expect("yearly period should build");
        let layer =
            build_yearly_horoscope_layer(&chart, &period).expect("yearly layer should build");

        assert_eq!(layer.scope(), Scope::Yearly, "{case_id}: layer scope");

        let expected = expected_yearly_dec_stars(yearly);
        assert_eq!(expected.len(), 24, "{case_id}: expect 12 岁前 + 12 将前");
        assert_eq!(
            actual_layer_dec_stars(&layer),
            expected,
            "{case_id}: yearly dec stars mismatch"
        );

        // Both families must be represented, all scoped Yearly, all branch-keyed.
        let families: std::collections::HashSet<_> = layer
            .temporal_decorative_stars()
            .iter()
            .map(|placement| placement.family())
            .collect();
        assert!(
            families.contains(&DecorativeStarFamily::Suiqian12)
                && families.contains(&DecorativeStarFamily::Jiangqian12),
            "{case_id}: both 岁前 and 将前 families present"
        );

        // The decorative facts are not typed scoped flow stars nor mutagen targets.
        let dec_names: std::collections::HashSet<StarName> = layer
            .temporal_decorative_stars()
            .iter()
            .map(|placement| placement.name())
            .collect();
        assert!(
            layer
                .placements()
                .iter()
                .all(|placement| !dec_names.contains(&placement.placement().name())),
            "{case_id}: yearlyDecStar must not appear among typed scoped stars"
        );
        assert!(
            layer
                .activations()
                .iter()
                .all(|activation| !dec_names.contains(&activation.target_star())),
            "{case_id}: yearlyDecStar must not appear among mutagen activations"
        );
    }
}

// --- B. Full stack includes yearlyDecStar only on the yearly layer -------------

#[test]
fn full_stack_carries_yearly_dec_stars_only_on_yearly_layer() {
    for case in horoscope_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let yearly = &case["supported_fields"]["yearly"];
        let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
            .expect("full horoscope stack should build");

        for layer in horoscope.layers() {
            if layer.scope() == Scope::Yearly {
                assert_eq!(
                    actual_layer_dec_stars(layer),
                    expected_yearly_dec_stars(yearly),
                    "{case_id}: yearly layer dec stars mismatch"
                );
            } else {
                assert!(
                    layer.temporal_decorative_stars().is_empty(),
                    "{case_id}: {:?} layer must carry no temporal decorative stars",
                    layer.scope()
                );
            }
        }
    }
}

// --- C. Snapshot separation ----------------------------------------------------

#[test]
fn snapshot_keeps_temporal_dec_stars_separate_from_natal() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let yearly = &case["supported_fields"]["yearly"];
    let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
        .expect("full horoscope stack should build");

    let snapshot = ChartStackSnapshot::from_horoscope_chart(&horoscope);

    // Natal layer keeps natal decorative stars and exposes no temporal ones.
    let natal = snapshot
        .layer(ChartLayerKind::Natal)
        .expect("natal snapshot layer should exist");
    assert!(
        natal
            .cells()
            .iter()
            .any(|cell| !cell.decorative_stars().is_empty()),
        "natal layer keeps natal decorative facts"
    );
    assert!(
        natal
            .cells()
            .iter()
            .all(|cell| cell.temporal_decorative_stars().is_empty()),
        "natal layer exposes no temporal decorative facts"
    );

    // Yearly layer exposes yearlyDecStar branch-keyed, with no natal facts.
    let yearly_layer = snapshot
        .layer(ChartLayerKind::Yearly)
        .expect("yearly snapshot layer should exist");
    let mut snapshot_dec = HashMap::new();
    for cell in yearly_layer.cells() {
        for star in cell.temporal_decorative_stars() {
            assert_eq!(star.scope(), Scope::Yearly);
            assert!(
                snapshot_dec
                    .insert((cell.branch(), star.family()), star.name())
                    .is_none()
            );
        }
    }
    assert_eq!(snapshot_dec, expected_yearly_dec_stars(yearly));
    assert!(
        yearly_layer
            .cells()
            .iter()
            .all(|cell| cell.typed_stars().is_empty() && cell.decorative_stars().is_empty()),
        "yearly layer carries no natal typed or decorative stars"
    );

    // No non-yearly temporal layer exposes temporal decorative facts.
    for kind in NON_YEARLY_LAYERS {
        let layer = snapshot
            .layer(kind)
            .unwrap_or_else(|| panic!("{kind:?} snapshot layer should exist"));
        assert!(
            layer
                .cells()
                .iter()
                .all(|cell| cell.temporal_decorative_stars().is_empty()),
            "{kind:?}: no temporal decorative facts"
        );
    }

    // Typed flow stars and mutagen activations remain present on the yearly layer.
    assert!(
        yearly_layer
            .cells()
            .iter()
            .any(|cell| !cell.scoped_stars().is_empty()),
        "yearly layer keeps typed scoped flow stars"
    );
    assert!(
        yearly_layer
            .cells()
            .iter()
            .any(|cell| !cell.mutagen_activations().is_empty()),
        "yearly layer keeps mutagen activations"
    );
}

// --- D. Serialization roundtrip ------------------------------------------------

#[test]
fn full_stack_round_trips_with_temporal_dec_stars() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let yearly = &case["supported_fields"]["yearly"];
    let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
        .expect("full horoscope stack should build");

    let encoded = serde_json::to_string(&horoscope).expect("horoscope should serialize");
    let decoded: HoroscopeChart =
        serde_json::from_str(&encoded).expect("horoscope should deserialize");

    assert_eq!(decoded, horoscope);

    let yearly_layer = decoded
        .layers()
        .iter()
        .find(|layer| layer.scope() == Scope::Yearly)
        .expect("decoded stack should keep a yearly layer");
    assert_eq!(
        actual_layer_dec_stars(yearly_layer),
        expected_yearly_dec_stars(yearly),
        "decoded yearly dec stars survive the roundtrip"
    );
}

// --- E. Metadata count invariants ----------------------------------------------

#[test]
fn yearly_dec_stars_do_not_change_metadata_counts() {
    assert_metadata_counts();
}

// --- helpers -------------------------------------------------------------------

fn actual_layer_dec_stars(
    layer: &TemporalLayer,
) -> HashMap<(EarthlyBranch, DecorativeStarFamily), StarName> {
    layer
        .temporal_decorative_stars()
        .iter()
        .map(|placement| {
            assert_eq!(placement.scope(), Scope::Yearly);
            ((placement.branch(), placement.family()), placement.name())
        })
        .collect()
}

fn stack_input(case: &Value) -> HoroscopeStackInput {
    let (year, month, day) = target_solar_date(case);
    HoroscopeStackInput::new(
        year,
        SolarMonth::new(month).expect("target solar month should be valid"),
        SolarDay::new(day).expect("target solar day should be valid"),
        target_time(case),
    )
}
