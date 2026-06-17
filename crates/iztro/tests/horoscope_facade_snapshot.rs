//! Fixture-driven tests for the upstream-like horoscope facade payload snapshot.
//!
//! These assert that [`HoroscopeFacadeSnapshot`] is assembled purely from the
//! already-modeled facts: the supported-field blocks, the runtime palace
//! projections, and the target lunar-date context. The facade adds no placement
//! logic, so the projections are checked against the same upstream-normalized
//! shape the runtime fixture uses, and the supported-field blocks are checked
//! against [`HoroscopeSupportedFieldsSnapshot`].

use std::collections::BTreeMap;

mod common;

use common::{
    assert_metadata_counts, build_chart_from_horoscope_fixture_case,
    horoscope_facade_fixture_cases, target_solar_date, target_time, target_time_index,
};
use iztro::core::{
    HoroscopeChart, HoroscopeFacadeSnapshot, HoroscopeRuntime, HoroscopeStackInput,
    HoroscopeSupportedFieldsSnapshot, NatalFacadeSnapshot, PalaceName, Scope, SolarDay, SolarMonth,
    build_full_horoscope_chart,
};
use serde_json::{Value, json};

const SUPPORTED_SCOPES: [&str; 6] = ["decadal", "age", "yearly", "monthly", "daily", "hourly"];
const STAR_LIST_FIELDS: [&str; 4] = [
    "natal_typed_stars",
    "natal_decorative_stars",
    "temporal_stars",
    "temporal_decorative_stars",
];

#[test]
fn facade_snapshot_matches_upstream_fixture() {
    for case in horoscope_facade_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let snapshot = build_facade_snapshot(&case);
        let actual = serde_json::to_value(&snapshot).expect("facade snapshot should serialize");
        let expected = &case["facade"];

        assert_eq!(
            actual["context"], expected["context"],
            "{case_id}: facade context"
        );
        assert_rich_context_shape(case_id, &actual["context"], &case);
        assert_eq!(
            normalize_astrolabe(&actual["astrolabe"]),
            normalize_astrolabe(&expected["astrolabe"]),
            "{case_id}: facade astrolabe snapshot"
        );

        assert_eq!(
            normalize_projection(&actual["age_palace"]),
            normalize_projection(&expected["age_palace"]),
            "{case_id}: facade age_palace projection"
        );

        assert_eq!(
            projections_by_scope(&actual["palace_projections"], normalize_projection),
            projections_by_scope(&expected["palace_projections"], normalize_projection),
            "{case_id}: facade palace projections"
        );

        assert_eq!(
            projections_by_scope(&actual["surround_palaces"], normalize_surround),
            projections_by_scope(&expected["surround_palaces"], normalize_surround),
            "{case_id}: facade surround palaces"
        );
    }
}

#[test]
fn facade_embeds_astrolabe_derived_from_natal_chart() {
    for case in horoscope_facade_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
            .expect("full horoscope stack should build");

        let facade = HoroscopeFacadeSnapshot::from_horoscope_chart(&horoscope)
            .expect("facade snapshot should build");
        let natal = NatalFacadeSnapshot::from_chart(horoscope.natal());

        assert_eq!(
            facade.astrolabe(),
            &natal,
            "{case_id}: facade astrolabe must derive only from horoscope.natal()"
        );
    }
}

#[test]
fn facade_astrolabe_keeps_complete_unique_natal_palace_identity() {
    use std::collections::HashSet;

    for case in horoscope_facade_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let snapshot = build_facade_snapshot(&case);
        let astrolabe = snapshot.astrolabe();

        assert_eq!(
            astrolabe.palaces().len(),
            12,
            "{case_id}: astrolabe must expose exactly twelve natal palaces"
        );

        let branches: HashSet<_> = astrolabe
            .palaces()
            .iter()
            .map(|palace| palace.branch())
            .collect();
        assert_eq!(
            branches.len(),
            12,
            "{case_id}: astrolabe palace branches must be unique"
        );

        let names: HashSet<_> = astrolabe
            .palaces()
            .iter()
            .map(|palace| palace.name())
            .collect();
        assert_eq!(
            names.len(),
            12,
            "{case_id}: astrolabe palace names must be unique"
        );
    }
}

#[test]
fn facade_astrolabe_does_not_leak_temporal_facts() {
    for case in horoscope_facade_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let snapshot = build_facade_snapshot(&case);

        for palace in snapshot.astrolabe().palaces() {
            for star in palace.typed_stars() {
                assert_eq!(
                    star.scope(),
                    Scope::Natal,
                    "{case_id}: astrolabe typed stars must stay natal-only"
                );
            }
            for star in palace.decorative_stars() {
                assert_eq!(
                    star.scope(),
                    Scope::Natal,
                    "{case_id}: astrolabe decorative stars must stay natal-only"
                );
            }

            let palace_json = serde_json::to_value(palace).expect("palace should serialize");
            assert!(
                palace_json.get("temporal_stars").is_none(),
                "{case_id}: astrolabe palace must not serialize temporal stars"
            );
            assert!(
                palace_json.get("temporal_decorative_stars").is_none(),
                "{case_id}: astrolabe palace must not serialize temporal decorative stars"
            );
            assert!(
                palace_json.get("temporal_mutagen_activations").is_none(),
                "{case_id}: astrolabe palace must not serialize temporal mutagens"
            );
        }
    }
}

#[test]
fn facade_context_uses_retained_target_context_when_present() {
    for case in horoscope_facade_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
            .expect("full horoscope stack should build");

        assert!(
            horoscope.target_context().is_some(),
            "{case_id}: full stack should retain target context"
        );

        let facade = HoroscopeFacadeSnapshot::from_horoscope_chart(&horoscope)
            .expect("facade snapshot should build");
        let context = serde_json::to_value(facade.context()).expect("context should serialize");

        assert_rich_context_shape(case_id, &context, &case);
        assert_eq!(
            context, case["facade"]["context"],
            "{case_id}: facade context"
        );
    }
}

#[test]
fn facade_context_falls_back_for_manual_horoscope_chart_without_target_context() {
    let case = horoscope_facade_fixture_cases()
        .into_iter()
        .next()
        .expect("facade fixture case");
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let full = build_full_horoscope_chart(chart, stack_input(&case))
        .expect("full horoscope stack should build");
    let manual = HoroscopeChart::with_layers(full.natal().clone(), full.layers().to_vec());

    assert!(manual.target_context().is_none());

    let facade = HoroscopeFacadeSnapshot::from_horoscope_chart(&manual)
        .expect("manual full-layer horoscope should still build facade snapshot");
    let context = serde_json::to_value(facade.context()).expect("context should serialize");
    let expected_lunar = &case["facade"]["context"]["lunar_date"];

    assert!(context["solar_date"].is_null());
    assert_eq!(context["lunar_date"], *expected_lunar);
    assert!(context["time_index"].is_null());
}

#[test]
fn facade_reuses_supported_fields_snapshot() {
    for case in horoscope_facade_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
            .expect("full horoscope stack should build");

        let facade = HoroscopeFacadeSnapshot::from_horoscope_chart(&horoscope)
            .expect("facade snapshot should build");
        let supported = HoroscopeSupportedFieldsSnapshot::from_horoscope_chart(&horoscope)
            .expect("supported-fields snapshot should build");

        let facade_json = serde_json::to_value(&facade).expect("facade should serialize");
        let supported_json = serde_json::to_value(&supported).expect("supported should serialize");

        for scope in SUPPORTED_SCOPES {
            assert_eq!(
                facade_json[scope], supported_json[scope],
                "{case_id}: facade {scope} block must reuse the supported-fields snapshot"
            );
        }
    }
}

#[test]
fn facade_projection_keeps_natal_and_temporal_labels_separate() {
    for case in horoscope_facade_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let snapshot = build_facade_snapshot(&case);

        let origin = snapshot
            .palace_projections()
            .iter()
            .find(|projection| projection.scope() == Scope::Natal)
            .expect("origin projection present");
        assert_eq!(
            origin.requested_palace_name(),
            PalaceName::Life,
            "{case_id}: origin projection requests the Life palace"
        );
        assert!(
            origin.temporal_palace_name().is_none(),
            "{case_id}: origin projection has no temporal palace label"
        );
        assert!(
            origin.temporal_stars().is_empty(),
            "{case_id}: origin projection has no temporal stars"
        );
        assert!(
            origin.temporal_mutagen_activations().is_empty(),
            "{case_id}: origin projection has no temporal mutagen activations"
        );

        let yearly = snapshot
            .palace_projections()
            .iter()
            .find(|projection| projection.scope() == Scope::Yearly)
            .expect("yearly projection present");
        assert!(
            yearly.temporal_palace_name().is_some(),
            "{case_id}: yearly projection carries a temporal palace label distinct from the natal name"
        );
    }
}

#[test]
fn facade_snapshot_coexists_with_runtime_helpers() {
    for case in horoscope_facade_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
            .expect("full horoscope stack should build");

        // Runtime helpers behave identically before and after the facade is built,
        // proving the facade is a read-only export over the same facts.
        let before = HoroscopeRuntime::new(&horoscope)
            .expect("runtime should validate")
            .palace(Scope::Yearly, PalaceName::Career)
            .expect("runtime projection should build");

        let facade = HoroscopeFacadeSnapshot::from_horoscope_chart(&horoscope)
            .expect("facade snapshot should build");

        let after = HoroscopeRuntime::new(&horoscope)
            .expect("runtime should validate")
            .palace(Scope::Yearly, PalaceName::Career)
            .expect("runtime projection should build");

        assert_eq!(
            before.branch(),
            after.branch(),
            "{case_id}: runtime projection unchanged by facade build"
        );
        assert_eq!(
            before.temporal_stars(),
            after.temporal_stars(),
            "{case_id}: runtime temporal stars unchanged by facade build"
        );

        // The facade's Life age projection matches the runtime age helper exactly.
        let runtime_age = HoroscopeRuntime::new(&horoscope)
            .expect("runtime should validate")
            .age_palace()
            .expect("runtime age projection should build");
        assert_eq!(
            facade.age_palace().branch(),
            runtime_age.branch(),
            "{case_id}: facade age_palace branch matches runtime helper"
        );
        assert_eq!(
            facade.age_palace().natal_palace_name(),
            runtime_age.natal_palace_name(),
            "{case_id}: facade age_palace natal palace matches runtime helper"
        );
        assert_eq!(
            facade.age_palace().temporal_palace_name(),
            runtime_age.temporal_palace_name(),
            "{case_id}: facade age_palace temporal palace matches runtime helper"
        );
    }
}

#[test]
fn facade_snapshot_round_trips_through_json() {
    let case = horoscope_facade_fixture_cases()
        .into_iter()
        .next()
        .expect("facade fixture case");
    let snapshot = build_facade_snapshot(&case);

    let encoded = serde_json::to_string(&snapshot).expect("facade snapshot should serialize");
    let decoded: HoroscopeFacadeSnapshot =
        serde_json::from_str(&encoded).expect("facade snapshot should deserialize");

    assert_eq!(decoded, snapshot);
}

#[test]
fn facade_snapshot_does_not_change_natal_or_metadata_boundaries() {
    let case = horoscope_facade_fixture_cases()
        .into_iter()
        .next()
        .expect("facade fixture case");
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let typed_count = chart.stars().len();
    let decorative_count = chart.decorative_stars().len();
    let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
        .expect("full horoscope stack should build");
    let layer_count = horoscope.layers().len();

    let _snapshot = HoroscopeFacadeSnapshot::from_horoscope_chart(&horoscope)
        .expect("facade snapshot should build");

    assert_eq!(
        horoscope.natal().stars().len(),
        typed_count,
        "facade build must not mutate natal typed stars"
    );
    assert_eq!(
        horoscope.natal().decorative_stars().len(),
        decorative_count,
        "facade build must not mutate natal decorative facts"
    );
    assert_eq!(
        horoscope.layers().len(),
        layer_count,
        "facade build must not mutate temporal layers"
    );
    assert_metadata_counts();
}

fn build_facade_snapshot(case: &Value) -> HoroscopeFacadeSnapshot {
    let chart = build_chart_from_horoscope_fixture_case(case);
    let horoscope = build_full_horoscope_chart(chart, stack_input(case))
        .expect("full horoscope stack should build");
    HoroscopeFacadeSnapshot::from_horoscope_chart(&horoscope).expect("facade snapshot should build")
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

fn assert_rich_context_shape(case_id: &str, context: &Value, case: &Value) {
    let (solar_year, solar_month, solar_day) = target_solar_date(case);

    assert_eq!(
        context["solar_date"]["year"].as_i64(),
        Some(solar_year as i64)
    );
    assert_eq!(
        context["solar_date"]["month"].as_u64(),
        Some(solar_month as u64),
        "{case_id}: target solar month"
    );
    assert_eq!(
        context["solar_date"]["day"].as_u64(),
        Some(solar_day as u64),
        "{case_id}: target solar day"
    );
    assert!(context["lunar_date"]["year"].is_i64());
    assert!(context["lunar_date"]["month"].is_u64());
    assert!(context["lunar_date"]["day"].is_u64());
    assert!(context["lunar_date"]["is_leap_month"].is_boolean());
    assert_eq!(
        context["time_index"].as_u64(),
        Some(target_time_index(case) as u64),
        "{case_id}: target time index"
    );
}

/// Indexes projections by their normalized scope so comparison is order-stable.
fn projections_by_scope(value: &Value, normalize: fn(&Value) -> Value) -> BTreeMap<String, Value> {
    value
        .as_array()
        .expect("projection array")
        .iter()
        .map(|projection| {
            let normalized = normalize(projection);
            let scope = normalized["scope"]
                .as_str()
                .expect("scope string")
                .to_owned();
            (scope, normalized)
        })
        .collect()
}

/// Normalizes a projection so upstream `origin` and Rust `natal` scope labels
/// compare equal and order-insensitive star lists compare as sets.
fn normalize_projection(value: &Value) -> Value {
    let mut object = value.as_object().expect("projection object").clone();
    normalize_scope(&mut object);

    for field in STAR_LIST_FIELDS {
        if let Some(stars) = object.get(field).and_then(Value::as_array) {
            let mut names: Vec<String> = stars
                .iter()
                .map(|star| star.as_str().expect("star name").to_owned())
                .collect();
            names.sort();
            object.insert(field.to_owned(), json!(names));
        }
    }

    if let Some(activations) = object
        .get("temporal_mutagen_activations")
        .and_then(Value::as_array)
    {
        let mut sorted = activations.clone();
        sorted.sort_by_key(|activation| {
            (
                activation["target_star"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                activation["mutagen"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            )
        });
        object.insert(
            "temporal_mutagen_activations".to_owned(),
            Value::Array(sorted),
        );
    }

    Value::Object(object)
}

/// Normalizes a surround block by normalizing its scope and each projection.
fn normalize_surround(value: &Value) -> Value {
    let mut object = value.as_object().expect("surround object").clone();
    normalize_scope(&mut object);
    for field in ["target", "opposite", "wealth", "career"] {
        let normalized = normalize_projection(&object[field]);
        object.insert(field.to_owned(), normalized);
    }
    Value::Object(object)
}

fn normalize_astrolabe(value: &Value) -> Value {
    let mut object = value.as_object().expect("astrolabe object").clone();
    let palaces = object["palaces"].as_array().expect("astrolabe palaces");
    let normalized_palaces = palaces
        .iter()
        .map(normalize_astrolabe_palace)
        .collect::<Vec<_>>();
    object.insert("palaces".to_owned(), Value::Array(normalized_palaces));
    Value::Object(object)
}

fn normalize_astrolabe_palace(value: &Value) -> Value {
    let mut object = value.as_object().expect("astrolabe palace").clone();
    if let Some(stars) = object.get("typed_stars").and_then(Value::as_array) {
        let mut sorted = stars.clone();
        sorted.sort_by_key(|star| {
            (
                star["name"].as_str().unwrap_or_default().to_owned(),
                star["kind"].as_str().unwrap_or_default().to_owned(),
                star["brightness"].as_str().unwrap_or_default().to_owned(),
            )
        });
        object.insert("typed_stars".to_owned(), Value::Array(sorted));
    }
    Value::Object(object)
}

fn normalize_scope(object: &mut serde_json::Map<String, Value>) {
    if object.get("scope").and_then(Value::as_str) == Some("natal") {
        object.insert("scope".to_owned(), Value::String("origin".to_owned()));
    }
}
