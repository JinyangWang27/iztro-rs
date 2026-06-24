mod common;

use std::collections::HashMap;

use common::{
    build_chart_from_horoscope_fixture_case, expected_palace_names_by_branch,
    expected_scope_flow_stars, expected_scope_mutagens, horoscope_fixture_case,
    horoscope_fixture_cases, scope_stem_branch, target_solar_date, target_time, target_time_index,
    target_year,
};
use iztro::core::{
    ChartLayerKind, ChartStackSnapshot, EarthlyBranch, FlowStarScope, HoroscopeChart,
    HoroscopeStackInput, Scope, SolarDay, SolarMonth, StarName, TemporalContext, TemporalLayer,
    TemporalPalaceLayout, build_decadal_frame, build_full_horoscope_chart,
};
use serde_json::Value;
use tyme4rs::tyme::solar::SolarDay as TymeSolarDay;

const CANONICAL_CASE_ID: &str = "canonical_female_default_2026";

/// The deterministic temporal stack order, paired with each scope's fixture key
/// and the flow-star scope it places (age places none).
const STACK: [(Scope, ChartLayerKind, &str, Option<FlowStarScope>); 6] = [
    (
        Scope::Decadal,
        ChartLayerKind::Decadal,
        "decadal",
        Some(FlowStarScope::Decadal),
    ),
    (Scope::Age, ChartLayerKind::Age, "age", None),
    (
        Scope::Yearly,
        ChartLayerKind::Yearly,
        "yearly",
        Some(FlowStarScope::Yearly),
    ),
    (
        Scope::Monthly,
        ChartLayerKind::Monthly,
        "monthly",
        Some(FlowStarScope::Monthly),
    ),
    (
        Scope::Daily,
        ChartLayerKind::Daily,
        "daily",
        Some(FlowStarScope::Daily),
    ),
    (
        Scope::Hourly,
        ChartLayerKind::Hourly,
        "hourly",
        Some(FlowStarScope::Hourly),
    ),
];

// --- A. Full stack matches all fixture cases -----------------------------------

#[test]
fn full_stack_matches_all_fixture_cases() {
    for case in horoscope_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let horoscope = build_full_horoscope_chart(chart.clone(), stack_input(&case))
            .expect("full horoscope stack should build");
        assert_target_context(case_id, &horoscope, &case);

        let layers = horoscope.layers();
        assert_eq!(layers.len(), 6, "{case_id}: stack should have six layers");
        assert_eq!(
            layers.iter().map(TemporalLayer::scope).collect::<Vec<_>>(),
            STACK.iter().map(|(scope, ..)| *scope).collect::<Vec<_>>(),
            "{case_id}: temporal layer scopes / order mismatch"
        );

        for (layer, (scope, _, fixture_key, flow_scope)) in layers.iter().zip(STACK) {
            let block = &case["supported_fields"][fixture_key];
            assert_eq!(layer.scope(), scope, "{case_id}: {fixture_key} scope");
            assert_layer_context(case_id, layer, &case, fixture_key);
            assert_layer_palace_layout(case_id, layer, block);
            assert_eq!(
                actual_mutagens(layer),
                expected_scope_mutagens(block, &chart),
                "{case_id}: {fixture_key} mutagens"
            );

            match flow_scope {
                Some(flow_scope) => assert_eq!(
                    actual_flow_stars(layer),
                    expected_scope_flow_stars(block, flow_scope),
                    "{case_id}: {fixture_key} flow stars"
                ),
                None => assert!(
                    layer.placements().is_empty(),
                    "{case_id}: age layer must place no flow stars"
                ),
            }

            // 年解 is a yearly-only flow star.
            let has_nian_jie = layer
                .placements()
                .iter()
                .any(|placement| placement.placement().name() == StarName::NianJieYearly);
            assert_eq!(
                has_nian_jie,
                scope == Scope::Yearly,
                "{case_id}: only the yearly layer carries NianJieYearly"
            );
        }
    }
}

// --- B. Snapshot separation ----------------------------------------------------

#[test]
fn full_stack_snapshot_separates_natal_and_temporal_facts() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let horoscope = build_full_horoscope_chart(chart.clone(), stack_input(&case))
        .expect("full horoscope stack should build");

    let snapshot = ChartStackSnapshot::from_horoscope_chart(&horoscope);

    assert_eq!(snapshot.layers().len(), 7, "natal + six temporal layers");
    assert_eq!(snapshot.layers()[0].kind(), ChartLayerKind::Natal);

    let natal = snapshot
        .layer(ChartLayerKind::Natal)
        .expect("natal snapshot layer should exist");
    assert_eq!(
        natal
            .cells()
            .iter()
            .map(|cell| cell.typed_stars().len())
            .sum::<usize>(),
        chart.stars().len(),
        "natal layer keeps every typed natal star"
    );
    assert!(
        natal
            .cells()
            .iter()
            .any(|cell| !cell.decorative_stars().is_empty()),
        "natal layer keeps decorative natal facts"
    );
    assert!(
        natal.cells().iter().all(|cell| {
            cell.temporal_palace_name().is_none()
                && cell.scoped_stars().is_empty()
                && cell.mutagen_activations().is_empty()
        }),
        "natal layer carries no temporal facts"
    );

    for (scope, kind, fixture_key, flow_scope) in STACK {
        let block = &case["supported_fields"][fixture_key];
        let layer = snapshot
            .layer(kind)
            .unwrap_or_else(|| panic!("{fixture_key} snapshot layer should exist"));

        assert!(
            layer
                .cells()
                .iter()
                .all(|cell| cell.typed_stars().is_empty()),
            "{fixture_key}: temporal layer must not duplicate natal typed stars"
        );
        assert!(
            layer
                .cells()
                .iter()
                .all(|cell| cell.decorative_stars().is_empty()),
            "{fixture_key}: temporal layer carries no decorative stars"
        );

        // Temporal palace names are exposed separately from natal palace names.
        let expected_palaces = expected_palace_names_by_branch(block);
        for cell in layer.cells() {
            let natal_palace = chart
                .palaces()
                .iter()
                .find(|palace| palace.branch() == cell.branch())
                .expect("natal branch should have a palace");
            assert_eq!(cell.natal_palace_name(), Some(natal_palace.name()));
            assert_eq!(
                cell.temporal_palace_name(),
                expected_palaces.get(&cell.branch()).copied(),
                "{fixture_key}: temporal palace name mismatch at {:?}",
                cell.branch()
            );
        }

        let scoped_count: usize = layer
            .cells()
            .iter()
            .map(|cell| cell.scoped_stars().len())
            .sum();
        let expected_scoped = match flow_scope {
            Some(flow_scope) => expected_scope_flow_stars(block, flow_scope).len(),
            None => 0,
        };
        assert_eq!(
            scoped_count, expected_scoped,
            "{fixture_key}: scoped star count"
        );
        assert!(
            layer
                .cells()
                .iter()
                .flat_map(|cell| cell.scoped_stars())
                .all(|star| star.scope() == scope),
            "{fixture_key}: scoped stars attach only to their own layer"
        );

        let mutagen_count: usize = layer
            .cells()
            .iter()
            .map(|cell| cell.mutagen_activations().len())
            .sum();
        assert_eq!(
            mutagen_count,
            expected_scope_mutagens(block, &chart).len(),
            "{fixture_key}: mutagen count"
        );
        assert!(
            layer
                .cells()
                .iter()
                .flat_map(|cell| cell.mutagen_activations())
                .all(|activation| activation.source_scope() == scope),
            "{fixture_key}: mutagens attach only to their own layer"
        );
    }
}

// --- C. Decadal and age derivation --------------------------------------------

#[test]
fn nominal_age_and_decadal_selection_derive_from_target_date() {
    let mut decadal_indices = Vec::new();

    for case in horoscope_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let horoscope = build_full_horoscope_chart(chart.clone(), stack_input(&case))
            .expect("full horoscope stack should build");

        let age_block = &case["supported_fields"]["age"];
        let expected_nominal_age = age_block["nominal_age"].as_u64().expect("nominal age") as u8;

        // Derived from the target date: target lunar year - birth lunar year + 1.
        let birth_lunar_year = case["input"]["year"].as_i64().expect("birth year") as i32;
        let derived = (target_year(&case) - birth_lunar_year + 1) as u8;
        assert_eq!(
            derived, expected_nominal_age,
            "{case_id}: derived nominal age mismatch"
        );

        let age_layer = layer_with_scope(&horoscope, Scope::Age);
        assert_eq!(
            *age_layer.context(),
            TemporalContext::Age {
                stem_branch: scope_stem_branch(age_block),
                nominal_age: expected_nominal_age,
            }
        );

        // Decadal period selected by nominal age, never a hard-coded index.
        let decadal_block = &case["supported_fields"]["decadal"];
        let expected_index = decadal_block["index"].as_u64().expect("decadal index") as usize;
        let decadal_layer = layer_with_scope(&horoscope, Scope::Decadal);
        let TemporalContext::Decadal {
            stem_branch,
            start_age,
        } = decadal_layer.context()
        else {
            panic!("{case_id}: decadal layer must carry decadal context");
        };
        assert_eq!(
            *stem_branch,
            scope_stem_branch(decadal_block),
            "{case_id}: decadal stem-branch"
        );
        assert!(
            (*start_age..=start_age + 9).contains(&expected_nominal_age),
            "{case_id}: nominal age {expected_nominal_age} not covered by decadal start age {start_age}"
        );
        assert_eq!(
            yin_first_index(stem_branch.branch()),
            expected_index,
            "{case_id}: decadal index"
        );

        // The selected period equals the frame period whose range covers the age.
        let frame = build_decadal_frame(&chart).expect("decadal frame should build");
        let selected = frame
            .periods()
            .iter()
            .find(|period| (period.start_age()..=period.end_age()).contains(&expected_nominal_age))
            .expect("a decadal period should cover the nominal age");
        assert_eq!(selected.stem_branch(), *stem_branch);

        decadal_indices.push(expected_index);
    }

    // No case hard-codes one decadal index: the fixtures exercise different ones.
    decadal_indices.sort_unstable();
    decadal_indices.dedup();
    assert!(
        decadal_indices.len() > 1,
        "fixtures should exercise more than one decadal index"
    );
}

// --- D. Serialization roundtrip ------------------------------------------------

#[test]
fn full_stack_round_trips_through_json() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
        .expect("full horoscope stack should build");

    let encoded = serde_json::to_string(&horoscope).expect("horoscope should serialize");
    let decoded: HoroscopeChart =
        serde_json::from_str(&encoded).expect("horoscope should deserialize");

    assert_eq!(decoded, horoscope);
    assert!(decoded.target_context().is_some());
    assert_eq!(decoded.layers().len(), 6);
    assert_eq!(
        decoded
            .layers()
            .iter()
            .map(TemporalLayer::scope)
            .collect::<Vec<_>>(),
        STACK.iter().map(|(scope, ..)| *scope).collect::<Vec<_>>(),
    );
}

#[test]
fn manual_horoscope_chart_constructors_do_not_set_target_context() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let natal = build_chart_from_horoscope_fixture_case(&case);
    let horoscope = HoroscopeChart::with_layers(natal, Vec::new());

    assert!(horoscope.target_context().is_none());
}

// --- helpers -------------------------------------------------------------------

/// Minimal lunar-date facts derived for fixture cross-checks.
struct TargetLunar {
    year: i32,
    month: u8,
    day: u8,
    is_leap_month: bool,
}

fn target_lunar_date(case: &Value) -> TargetLunar {
    let (year, month, day) = target_solar_date(case);
    let lunar = TymeSolarDay::from_ymd(year as isize, month as usize, day as usize).get_lunar_day();
    let lunar_month = lunar.get_lunar_month();
    TargetLunar {
        year: lunar_month.get_lunar_year().get_year() as i32,
        month: lunar_month.get_month() as u8,
        day: lunar.get_day() as u8,
        is_leap_month: lunar_month.is_leap(),
    }
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

fn assert_target_context(case_id: &str, horoscope: &HoroscopeChart, case: &Value) {
    let context = horoscope
        .target_context()
        .unwrap_or_else(|| panic!("{case_id}: full stack should retain target context"));
    let (solar_year, solar_month, solar_day) = target_solar_date(case);
    let target_lunar = target_lunar_date(case);

    assert_eq!(
        context.solar_date().year(),
        solar_year,
        "{case_id}: target solar year"
    );
    assert_eq!(
        context.solar_date().month(),
        solar_month,
        "{case_id}: target solar month"
    );
    assert_eq!(
        context.solar_date().day(),
        solar_day,
        "{case_id}: target solar day"
    );
    assert_eq!(
        context.lunar_date().year(),
        target_lunar.year,
        "{case_id}: target lunar year"
    );
    assert_eq!(
        context.lunar_date().month(),
        target_lunar.month,
        "{case_id}: target lunar month"
    );
    assert_eq!(
        context.lunar_date().day(),
        target_lunar.day,
        "{case_id}: target lunar day"
    );
    assert_eq!(
        context.lunar_date().is_leap_month(),
        target_lunar.is_leap_month,
        "{case_id}: target leap-month flag"
    );
    assert_eq!(
        context.time_index(),
        target_time_index(case),
        "{case_id}: target time index"
    );
}

fn layer_with_scope(horoscope: &HoroscopeChart, scope: Scope) -> &TemporalLayer {
    horoscope
        .layers()
        .iter()
        .find(|layer| layer.scope() == scope)
        .unwrap_or_else(|| panic!("stack should contain a {scope:?} layer"))
}

fn assert_layer_context(case_id: &str, layer: &TemporalLayer, case: &Value, fixture_key: &str) {
    let block = &case["supported_fields"][fixture_key];
    let stem_branch = scope_stem_branch(block);
    match layer.context() {
        TemporalContext::Decadal {
            stem_branch: sb, ..
        } => {
            assert_eq!(*sb, stem_branch, "{case_id}: decadal stem-branch")
        }
        TemporalContext::Age {
            stem_branch: sb,
            nominal_age,
        } => {
            assert_eq!(*sb, stem_branch, "{case_id}: age stem-branch");
            assert_eq!(
                *nominal_age,
                block["nominal_age"].as_u64().expect("nominal age") as u8,
                "{case_id}: age nominal age"
            );
        }
        TemporalContext::Yearly {
            stem_branch: sb,
            lunar_year,
        } => {
            assert_eq!(*sb, stem_branch, "{case_id}: yearly stem-branch");
            assert_eq!(
                *lunar_year,
                target_year(case),
                "{case_id}: yearly lunar year"
            );
        }
        TemporalContext::Monthly {
            stem_branch: sb,
            lunar_month,
        } => {
            let target_lunar = target_lunar_date(case);
            assert_eq!(
                *lunar_month, target_lunar.month,
                "{case_id}: monthly lunar month"
            );
            assert_eq!(*sb, stem_branch, "{case_id}: monthly stem-branch");
            assert!(
                (1..=12).contains(lunar_month),
                "{case_id}: monthly lunar month"
            );
        }
        TemporalContext::Daily {
            stem_branch: sb,
            lunar_day,
        } => {
            let target_lunar = target_lunar_date(case);
            assert_eq!(*lunar_day, target_lunar.day, "{case_id}: daily lunar day");
            assert_eq!(*sb, stem_branch, "{case_id}: daily stem-branch");
            assert!((1..=30).contains(lunar_day), "{case_id}: daily lunar day");
        }
        TemporalContext::Hourly { stem_branch: sb } => {
            assert_eq!(*sb, stem_branch, "{case_id}: hourly stem-branch")
        }
    }
}

fn assert_layer_palace_layout(case_id: &str, layer: &TemporalLayer, block: &Value) {
    let layout: &TemporalPalaceLayout = layer
        .palace_layout()
        .expect("temporal layer should carry a palace layout");
    assert_eq!(layout.scope(), layer.scope());
    assert_eq!(layout.names().len(), 12);

    let expected = expected_palace_names_by_branch(block);
    for name in layout.names() {
        assert_eq!(
            Some(name.palace_name()),
            expected.get(&name.branch()).copied(),
            "{case_id}: palace name mismatch at {:?}",
            name.branch()
        );
    }
}

fn actual_mutagens(
    layer: &TemporalLayer,
) -> HashMap<(StarName, EarthlyBranch), iztro::core::Mutagen> {
    layer
        .activations()
        .iter()
        .map(|activation| {
            (
                (activation.target_star(), activation.target_branch()),
                activation.mutagen(),
            )
        })
        .collect()
}

fn actual_flow_stars(
    layer: &TemporalLayer,
) -> HashMap<StarName, (EarthlyBranch, iztro::core::StarKind)> {
    layer
        .placements()
        .iter()
        .map(|placement| {
            (
                placement.placement().name(),
                (placement.branch(), placement.placement().kind()),
            )
        })
        .collect()
}

fn yin_first_index(branch: EarthlyBranch) -> usize {
    (0..12)
        .find(|index| EarthlyBranch::Yin.offset(*index as isize) == branch)
        .expect("branch should be reachable from Yin")
}
