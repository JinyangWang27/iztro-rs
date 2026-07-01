mod common;

use std::collections::{HashMap, HashSet};

use common::{
    assert_metadata_counts, build_chart_from_horoscope_fixture_case,
    horoscope_runtime_fixture_cases, parse_key, target_solar_date, target_time,
};
use iztro::core::{
    ChartError, EarthlyBranch, HeavenlyStem, HoroscopeChart, HoroscopePalaceProjection,
    HoroscopeRuntime, HoroscopeStackInput, HoroscopeSurroundPalaces, Mutagen, PalaceName, Scope,
    SolarDay, SolarMonth, StarName, TemporalLayer, build_full_horoscope_chart,
};
use serde_json::Value;

#[test]
fn runtime_palace_projections_match_upstream_fixture() {
    for case in horoscope_runtime_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
            .expect("full horoscope stack should build");
        let runtime = HoroscopeRuntime::new(&horoscope).expect("runtime should validate");

        assert_projection(
            case_id,
            "age_palace",
            &runtime.age_palace().expect("age palace projection"),
            &case["runtime"]["age_palace"],
        );

        for expected in case["runtime"]["palace_projections"]
            .as_array()
            .expect("palace projections")
        {
            let scope = parse_runtime_scope(expected["scope"].as_str().expect("scope"));
            let palace = parse_key::<PalaceName>(
                expected["requested_palace_name"]
                    .as_str()
                    .expect("requested palace"),
            );
            let actual = runtime
                .palace(scope, palace)
                .unwrap_or_else(|err| panic!("{case_id}: palace projection failed: {err}"));
            assert_projection(case_id, "palace", &actual, expected);
        }

        for expected in case["runtime"]["surround_palaces"]
            .as_array()
            .expect("surround palaces")
        {
            let scope = parse_runtime_scope(expected["scope"].as_str().expect("scope"));
            let palace = parse_key::<PalaceName>(
                expected["requested_palace_name"]
                    .as_str()
                    .expect("requested palace"),
            );
            let actual = runtime
                .surround_palaces(scope, palace)
                .unwrap_or_else(|err| panic!("{case_id}: surround projection failed: {err}"));
            assert_surround(case_id, actual, expected);
        }
    }
}

#[test]
fn runtime_query_helpers_match_upstream_fixture() {
    for case in horoscope_runtime_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
            .expect("full horoscope stack should build");
        let runtime = HoroscopeRuntime::new(&horoscope).expect("runtime should validate");

        for query in case["runtime"]["star_queries"]
            .as_array()
            .expect("star queries")
        {
            let scope = parse_runtime_scope(query["scope"].as_str().expect("scope"));
            let palace = parse_key::<PalaceName>(query["palace_name"].as_str().expect("palace"));
            let stars = parse_star_list(&query["stars"]);
            let expected = query["expected"].as_bool().expect("expected bool");
            let actual = match query["helper"].as_str().expect("helper") {
                "hasHoroscopeStars" => runtime.has_horoscope_stars(scope, palace, &stars),
                "notHaveHoroscopeStars" => runtime.not_have_horoscope_stars(scope, palace, &stars),
                "hasOneOfHoroscopeStars" => {
                    runtime.has_one_of_horoscope_stars(scope, palace, &stars)
                }
                helper => panic!("{case_id}: unsupported star query helper {helper}"),
            }
            .unwrap_or_else(|err| panic!("{case_id}: star query failed: {err}"));
            assert_eq!(actual, expected, "{case_id}: {query:?}");
        }

        for query in case["runtime"]["mutagen_queries"]
            .as_array()
            .expect("mutagen queries")
        {
            let scope = parse_runtime_scope(query["scope"].as_str().expect("scope"));
            let palace = parse_key::<PalaceName>(query["palace_name"].as_str().expect("palace"));
            let mutagen = parse_key::<Mutagen>(query["mutagen"].as_str().expect("mutagen"));
            let expected = query["expected"].as_bool().expect("expected bool");
            let actual = runtime
                .has_horoscope_mutagen(scope, palace, mutagen)
                .unwrap_or_else(|err| panic!("{case_id}: mutagen query failed: {err}"));
            assert_eq!(actual, expected, "{case_id}: {query:?}");
        }
    }
}

#[test]
fn runtime_decadal_yearly_aliases_match_compatibility_names() {
    for case in horoscope_runtime_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
            .expect("full horoscope stack should build");
        let runtime = HoroscopeRuntime::new(&horoscope).expect("runtime should validate");

        for query in case["runtime"]["star_queries"]
            .as_array()
            .expect("star queries")
        {
            let scope = parse_runtime_scope(query["scope"].as_str().expect("scope"));
            let palace = parse_key::<PalaceName>(query["palace_name"].as_str().expect("palace"));
            let stars = parse_star_list(&query["stars"]);

            assert_eq!(
                runtime.has_horoscope_stars(scope, palace, &stars),
                runtime.has_decadal_yearly_horoscope_stars(scope, palace, &stars),
                "{case_id}: has_* alias diverged"
            );
            assert_eq!(
                runtime.not_have_horoscope_stars(scope, palace, &stars),
                runtime.not_have_decadal_yearly_horoscope_stars(scope, palace, &stars),
                "{case_id}: not_have_* alias diverged"
            );
            assert_eq!(
                runtime.has_one_of_horoscope_stars(scope, palace, &stars),
                runtime.has_one_of_decadal_yearly_horoscope_stars(scope, palace, &stars),
                "{case_id}: has_one_of_* alias diverged"
            );
        }
    }
}

#[test]
fn runtime_boundary_errors_are_clean() {
    let case = horoscope_runtime_fixture_cases()
        .into_iter()
        .next()
        .expect("runtime fixture case");
    let natal = build_chart_from_horoscope_fixture_case(&case);
    let horoscope = build_full_horoscope_chart(natal, stack_input(&case))
        .expect("full horoscope stack should build");

    let missing = HoroscopeChart::with_layers(horoscope.natal().clone(), Vec::new());
    assert_eq!(
        HoroscopeRuntime::new(&missing).err(),
        Some(ChartError::MissingHoroscopeLayer { scope: Scope::Age })
    );

    let mut duplicate_layers = horoscope.layers().to_vec();
    duplicate_layers.push(duplicate_layers[0].clone());
    let duplicate = HoroscopeChart::with_layers(horoscope.natal().clone(), duplicate_layers);
    assert_eq!(
        HoroscopeRuntime::new(&duplicate).err(),
        Some(ChartError::DuplicateHoroscopeLayer {
            scope: Scope::Decadal
        })
    );

    let mut no_layout_layers = horoscope.layers().to_vec();
    let yearly = no_layout_layers
        .iter()
        .position(|layer| layer.scope() == Scope::Yearly)
        .expect("yearly layer");
    let original = &no_layout_layers[yearly];
    no_layout_layers[yearly] = TemporalLayer::try_new(
        original.scope(),
        *original.context(),
        original.placements().to_vec(),
        original.activations().to_vec(),
    )
    .expect("test layer should preserve temporal invariants");
    let no_layout = HoroscopeChart::with_layers(horoscope.natal().clone(), no_layout_layers);
    assert_eq!(
        HoroscopeRuntime::new(&no_layout).err(),
        Some(ChartError::MissingHoroscopePalaceLayout {
            scope: Scope::Yearly
        })
    );
}

#[test]
fn runtime_query_empty_star_lists_are_explicit() {
    let case = horoscope_runtime_fixture_cases()
        .into_iter()
        .next()
        .expect("runtime fixture case");
    let natal = build_chart_from_horoscope_fixture_case(&case);
    let horoscope = build_full_horoscope_chart(natal, stack_input(&case))
        .expect("full horoscope stack should build");
    let runtime = HoroscopeRuntime::new(&horoscope).expect("runtime should validate");

    assert!(
        runtime
            .has_horoscope_stars(Scope::Decadal, PalaceName::Life, &[])
            .expect("empty has query")
    );
    assert!(
        runtime
            .not_have_horoscope_stars(Scope::Decadal, PalaceName::Life, &[])
            .expect("empty not-have query")
    );
    assert!(
        !runtime
            .has_one_of_horoscope_stars(Scope::Decadal, PalaceName::Life, &[])
            .expect("empty one-of query")
    );
}

#[test]
fn runtime_helpers_do_not_change_natal_or_metadata_boundaries() {
    let case = horoscope_runtime_fixture_cases()
        .into_iter()
        .next()
        .expect("runtime fixture case");
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let typed_count = chart.stars().len();
    let decorative_count = chart.decorative_stars().len();
    let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
        .expect("full horoscope stack should build");
    let layer_count = horoscope.layers().len();

    let runtime = HoroscopeRuntime::new(&horoscope).expect("runtime should validate");
    let _ = runtime
        .palace(Scope::Yearly, PalaceName::Life)
        .expect("projection should build");
    let _ = runtime
        .has_horoscope_stars(Scope::Decadal, PalaceName::Life, &[StarName::YunLu])
        .expect("query should run");
    let _ = runtime
        .has_horoscope_mutagen(Scope::Yearly, PalaceName::Life, Mutagen::Lu)
        .expect("mutagen query should run");

    assert_eq!(horoscope.natal().stars().len(), typed_count);
    assert_eq!(horoscope.natal().decorative_stars().len(), decorative_count);
    assert_eq!(horoscope.layers().len(), layer_count);
    assert_metadata_counts();
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

fn parse_runtime_scope(value: &str) -> Scope {
    match value {
        "origin" => Scope::Natal,
        "age" => Scope::Age,
        "decadal" => Scope::Decadal,
        "yearly" => Scope::Yearly,
        "monthly" => Scope::Monthly,
        "daily" => Scope::Daily,
        "hourly" => Scope::Hourly,
        other => panic!("unsupported runtime scope: {other}"),
    }
}

fn parse_star_list(value: &Value) -> Vec<StarName> {
    value
        .as_array()
        .expect("star list")
        .iter()
        .map(|star| parse_key::<StarName>(star.as_str().expect("star name")))
        .collect()
}

fn assert_surround(case_id: &str, actual: HoroscopeSurroundPalaces, expected: &Value) {
    assert_projection(
        case_id,
        "surround target",
        actual.target(),
        &expected["target"],
    );
    assert_projection(
        case_id,
        "surround opposite",
        actual.opposite(),
        &expected["opposite"],
    );
    assert_projection(
        case_id,
        "surround wealth",
        actual.wealth(),
        &expected["wealth"],
    );
    assert_projection(
        case_id,
        "surround career",
        actual.career(),
        &expected["career"],
    );
}

fn assert_projection(
    case_id: &str,
    label: &str,
    actual: &HoroscopePalaceProjection,
    expected: &Value,
) {
    assert_eq!(
        actual.scope(),
        parse_runtime_scope(expected["scope"].as_str().expect("scope")),
        "{case_id}: {label} scope"
    );
    assert_eq!(
        actual.requested_palace_name(),
        parse_key::<PalaceName>(
            expected["requested_palace_name"]
                .as_str()
                .expect("requested palace")
        ),
        "{case_id}: {label} requested palace"
    );
    assert_eq!(
        actual.branch(),
        parse_key::<EarthlyBranch>(expected["branch"].as_str().expect("branch")),
        "{case_id}: {label} branch"
    );
    assert_eq!(
        actual.natal_palace_name(),
        parse_key::<PalaceName>(
            expected["natal_palace_name"]
                .as_str()
                .expect("natal palace")
        ),
        "{case_id}: {label} natal palace"
    );
    assert_eq!(
        actual.temporal_palace_name(),
        expected["temporal_palace_name"]
            .as_str()
            .map(parse_key::<PalaceName>),
        "{case_id}: {label} temporal palace"
    );
    assert_eq!(
        actual.natal_palace_stem(),
        parse_key::<HeavenlyStem>(
            expected["natal_palace_stem"]
                .as_str()
                .expect("natal palace stem")
        ),
        "{case_id}: {label} natal palace stem"
    );
    assert_star_set_eq(
        actual.natal_typed_stars().to_vec(),
        parse_star_list(&expected["natal_typed_stars"]),
        case_id,
        label,
        "natal typed stars",
    );
    assert_star_set_eq(
        actual.natal_decorative_stars().to_vec(),
        parse_star_list(&expected["natal_decorative_stars"]),
        case_id,
        label,
        "natal decorative stars",
    );
    assert_star_set_eq(
        actual.temporal_stars().to_vec(),
        parse_star_list(&expected["temporal_stars"]),
        case_id,
        label,
        "temporal stars",
    );
    assert_star_set_eq(
        actual.temporal_decorative_stars().to_vec(),
        parse_star_list(&expected["temporal_decorative_stars"]),
        case_id,
        label,
        "temporal decorative stars",
    );

    let actual_activations: HashMap<_, _> = actual
        .temporal_mutagen_activations()
        .iter()
        .map(|activation| (activation.target_star(), activation.mutagen()))
        .collect();
    let expected_activations: HashMap<_, _> = expected["temporal_mutagen_activations"]
        .as_array()
        .expect("temporal mutagen activations")
        .iter()
        .map(|activation| {
            (
                parse_key::<StarName>(activation["target_star"].as_str().expect("target star")),
                parse_key::<Mutagen>(activation["mutagen"].as_str().expect("mutagen")),
            )
        })
        .collect();
    assert_eq!(
        actual_activations, expected_activations,
        "{case_id}: {label} temporal mutagen activations"
    );
}

fn assert_star_set_eq(
    actual: Vec<StarName>,
    expected: Vec<StarName>,
    case_id: &str,
    label: &str,
    field: &str,
) {
    assert_eq!(
        actual.into_iter().collect::<HashSet<_>>(),
        expected.into_iter().collect::<HashSet<_>>(),
        "{case_id}: {label} {field}",
    );
}
