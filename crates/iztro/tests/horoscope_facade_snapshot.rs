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
    HoroscopeSupportedFieldsSnapshot, NatalFacadePalaceRole, NatalFacadeSnapshot, PalaceName,
    Scope, SolarDay, SolarMonth, build_full_horoscope_chart,
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
fn facade_snapshot_accessors_expose_serialized_facts() {
    let case = horoscope_facade_fixture_cases()
        .into_iter()
        .next()
        .expect("facade fixture case");
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let horoscope =
        build_full_horoscope_chart(chart, stack_input(&case)).expect("full horoscope stack");
    let snapshot = HoroscopeFacadeSnapshot::from_horoscope_chart(&horoscope)
        .expect("facade snapshot should build");
    let natal = horoscope.natal();

    let facade_json = serde_json::to_value(&snapshot).expect("facade should serialize");
    let supported_json =
        serde_json::to_value(snapshot.supported_fields()).expect("supported should serialize");
    for scope in SUPPORTED_SCOPES {
        assert_eq!(facade_json[scope], supported_json[scope]);
    }

    let context = snapshot.context();
    assert!(context.solar_date().is_some());
    assert_eq!(
        context.lunar_date(),
        horoscope.target_context().unwrap().lunar_date()
    );
    assert_eq!(context.time_index(), Some(target_time_index(&case)));

    let astrolabe = snapshot.astrolabe();
    assert_eq!(astrolabe.gender(), natal.birth_context().gender());
    assert_eq!(astrolabe.birth_year_stem(), natal.birth_year().stem());
    assert_eq!(astrolabe.birth_year_branch(), natal.birth_year().branch());
    assert_eq!(astrolabe.five_element_bureau(), natal.five_element_bureau());
    assert_eq!(
        astrolabe.life_palace_branch(),
        natal.life_palace().map(|palace| palace.branch())
    );
    assert_eq!(astrolabe.body_palace_branch(), natal.body_palace_branch());

    let body_palace = astrolabe
        .palaces()
        .iter()
        .find(|palace| palace.branch() == astrolabe.body_palace_branch().unwrap())
        .expect("body palace snapshot");
    assert_eq!(body_palace.stem(), natal.body_palace().unwrap().stem());
    assert!(
        body_palace
            .roles()
            .contains(&NatalFacadePalaceRole::NatalBodyPalace)
    );

    let populated_palace = astrolabe
        .palaces()
        .iter()
        .find(|palace| !palace.typed_stars().is_empty() && !palace.decorative_stars().is_empty())
        .expect("palace with natal stars");
    let natal_palace = natal
        .palaces()
        .iter()
        .find(|palace| palace.branch() == populated_palace.branch())
        .expect("matching natal palace");
    assert_eq!(populated_palace.name(), natal_palace.name());
    assert_eq!(populated_palace.stem(), natal_palace.stem());

    let typed_star = &populated_palace.typed_stars()[0];
    assert_eq!(typed_star.scope(), Scope::Natal);
    assert_eq!(typed_star.category(), typed_star.kind().category());
    assert!(
        natal
            .stars()
            .iter()
            .any(|fact| fact.placement().name() == typed_star.name()
                && fact.placement().brightness() == typed_star.brightness()
                && fact.placement().mutagen() == typed_star.mutagen())
    );

    let decorative_star = &populated_palace.decorative_stars()[0];
    assert_eq!(decorative_star.scope(), Scope::Natal);
    assert!(
        natal
            .decorative_stars()
            .iter()
            .any(|fact| fact.name() == decorative_star.name()
                && fact.placement().family() == decorative_star.family())
    );

    let yearly_projection = snapshot
        .palace_projections()
        .iter()
        .find(|projection| projection.scope() == Scope::Yearly)
        .expect("yearly projection");
    assert_eq!(yearly_projection.requested_palace_name(), PalaceName::Life);
    assert_eq!(
        yearly_projection.natal_palace_stem(),
        natal
            .palaces()
            .iter()
            .find(|palace| palace.branch() == yearly_projection.branch())
            .unwrap()
            .stem()
    );
    assert!(!yearly_projection.natal_typed_stars().is_empty());
    assert!(!yearly_projection.natal_decorative_stars().is_empty());
    assert!(yearly_projection.temporal_palace_name().is_some());
    assert!(!yearly_projection.temporal_stars().is_empty());
    let _ = yearly_projection.temporal_decorative_stars();

    let surround = snapshot
        .surround_palaces()
        .iter()
        .find(|surround| surround.scope() == Scope::Yearly)
        .expect("yearly surround");
    assert_eq!(surround.requested_palace_name(), PalaceName::Life);
    let projections = [
        surround.target(),
        surround.opposite(),
        surround.wealth(),
        surround.career(),
    ];
    assert!(
        projections
            .iter()
            .any(|projection| { !projection.temporal_mutagen_activations().is_empty() })
    );
    let activation = projections
        .iter()
        .flat_map(|projection| projection.temporal_mutagen_activations())
        .next()
        .expect("temporal mutagen activation in surround");
    assert!(natal.star(activation.target_star()).is_some());
    assert!(matches!(
        activation.mutagen(),
        iztro::core::Mutagen::Lu
            | iztro::core::Mutagen::Quan
            | iztro::core::Mutagen::Ke
            | iztro::core::Mutagen::Ji
    ));
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

#[test]
fn facade_astrolabe_exposes_zh_labels_alongside_machine_fields() {
    use iztro::core::labels::zh_cn;

    for case in horoscope_facade_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let snapshot = build_facade_snapshot(&case);
        let astrolabe = snapshot.astrolabe();

        assert_eq!(
            astrolabe.birth_year_stem_zh(),
            zh_cn::heavenly_stem_zh(astrolabe.birth_year_stem()),
            "{case_id}: birth-year stem zh label"
        );
        assert_eq!(
            astrolabe.birth_year_branch_zh(),
            zh_cn::earthly_branch_zh(astrolabe.birth_year_branch()),
            "{case_id}: birth-year branch zh label"
        );
        assert_eq!(
            astrolabe.life_palace_branch_zh(),
            astrolabe.life_palace_branch().map(zh_cn::earthly_branch_zh),
            "{case_id}: life palace branch zh label"
        );
        assert_eq!(
            astrolabe.body_palace_branch_zh(),
            astrolabe.body_palace_branch().map(zh_cn::earthly_branch_zh),
            "{case_id}: body palace branch zh label"
        );

        for palace in astrolabe.palaces() {
            assert_eq!(
                palace.branch_zh(),
                zh_cn::earthly_branch_zh(palace.branch()),
                "{case_id}: palace branch_zh"
            );
            assert_eq!(
                palace.name_zh(),
                zh_cn::palace_name_zh(palace.name()),
                "{case_id}: palace name_zh"
            );
            assert_eq!(
                palace.stem_zh(),
                zh_cn::heavenly_stem_zh(palace.stem()),
                "{case_id}: palace stem_zh"
            );
            assert!(!palace.branch_zh().is_empty());
            assert!(!palace.name_zh().is_empty());
            assert!(!palace.stem_zh().is_empty());

            for star in palace.typed_stars() {
                assert_eq!(
                    star.name_zh(),
                    zh_cn::star_name_zh(star.name()),
                    "{case_id}: typed star name_zh"
                );
                assert_eq!(
                    star.kind_zh(),
                    zh_cn::star_kind_zh(star.kind()),
                    "{case_id}: typed star kind_zh"
                );
                assert_eq!(
                    star.brightness_zh(),
                    zh_cn::brightness_zh(star.brightness()),
                    "{case_id}: typed star brightness_zh"
                );
                assert!(!star.name_zh().is_empty());
                assert!(!star.kind_zh().is_empty());
                assert_eq!(
                    star.mutagen_zh(),
                    star.mutagen().map(zh_cn::mutagen_zh),
                    "{case_id}: typed star mutagen_zh present iff mutagen present"
                );
            }

            for star in palace.decorative_stars() {
                assert_eq!(
                    star.name_zh(),
                    zh_cn::star_name_zh(star.name()),
                    "{case_id}: decorative star name_zh"
                );
                assert_eq!(
                    star.family_zh(),
                    zh_cn::decorative_star_family_zh(star.family()),
                    "{case_id}: decorative star family_zh"
                );
                assert!(!star.name_zh().is_empty());
                assert!(!star.family_zh().is_empty());
            }
        }
    }
}

#[test]
fn facade_astrolabe_serializes_zh_labels_additively() {
    let case = horoscope_facade_fixture_cases()
        .into_iter()
        .next()
        .expect("facade fixture case");
    let snapshot = build_facade_snapshot(&case);
    let json = serde_json::to_value(&snapshot).expect("facade should serialize");
    let astrolabe = &json["astrolabe"];

    // Machine-readable identities remain unchanged.
    assert!(astrolabe["birth_year_stem"].is_string());
    assert!(astrolabe["birth_year_branch"].is_string());
    // Additive Chinese labels sit beside them.
    assert!(astrolabe["birth_year_stem_zh"].is_string());
    assert!(astrolabe["birth_year_branch_zh"].is_string());

    let palace = astrolabe["palaces"]
        .as_array()
        .expect("palaces array")
        .iter()
        .find(|palace| {
            !palace["typed_stars"].as_array().unwrap().is_empty()
                && !palace["decorative_stars"].as_array().unwrap().is_empty()
        })
        .expect("a populated palace");

    for key in ["branch", "branch_zh", "name", "name_zh", "stem", "stem_zh"] {
        assert!(palace[key].is_string(), "palace.{key} should serialize");
    }

    let typed = &palace["typed_stars"][0];
    for key in [
        "name",
        "name_zh",
        "kind",
        "kind_zh",
        "brightness",
        "brightness_zh",
    ] {
        assert!(typed[key].is_string(), "typed_star.{key} should serialize");
    }

    let decorative = &palace["decorative_stars"][0];
    for key in ["name", "name_zh", "family", "family_zh"] {
        assert!(
            decorative[key].is_string(),
            "decorative_star.{key} should serialize"
        );
    }
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

/// Recursively removes additive `*_zh` localized-label keys so the upstream
/// fixture comparison keeps validating only the canonical machine-readable
/// fields. The fixture intentionally records no Chinese labels.
fn strip_zh_keys(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.retain(|key, _| !key.ends_with("_zh"));
            for child in map.values_mut() {
                strip_zh_keys(child);
            }
        }
        Value::Array(items) => {
            for item in items {
                strip_zh_keys(item);
            }
        }
        _ => {}
    }
}

fn normalize_astrolabe(value: &Value) -> Value {
    let mut value = value.clone();
    strip_zh_keys(&mut value);
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
