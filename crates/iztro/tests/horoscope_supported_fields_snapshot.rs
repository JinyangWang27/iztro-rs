use std::collections::BTreeSet;

mod common;

use common::{
    assert_metadata_counts, build_chart_from_horoscope_fixture_case, horoscope_fixture_case,
    horoscope_fixture_cases, target_solar_date, target_time,
};
use iztro::core::{
    ChartError, HoroscopeChart, HoroscopeStackInput, HoroscopeSupportedFieldsSnapshot, Scope,
    SolarDay, SolarMonth, StarName, TemporalLayer, build_full_horoscope_chart,
};
use serde_json::{Value, json};

const CANONICAL_CASE_ID: &str = "canonical_female_default_2026";
const SCOPES: [&str; 6] = ["decadal", "age", "yearly", "monthly", "daily", "hourly"];

#[test]
fn supported_fields_snapshot_matches_all_horoscope_fixture_cases() {
    for case in horoscope_fixture_cases() {
        let case_id = case["id"].as_str().expect("case id");
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
            .expect("full horoscope stack should build");
        let snapshot = HoroscopeSupportedFieldsSnapshot::from_horoscope_chart(&horoscope)
            .expect("supported-fields snapshot should build");
        let actual = serde_json::to_value(snapshot).expect("snapshot should serialize");

        assert_eq!(
            actual
                .as_object()
                .expect("snapshot object")
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            SCOPES
                .into_iter()
                .map(str::to_owned)
                .collect::<BTreeSet<_>>(),
            "{case_id}: top-level snapshot keys"
        );

        for scope in SCOPES {
            assert_eq!(
                actual[scope],
                expected_scope_snapshot(&case["supported_fields"][scope], scope),
                "{case_id}: {scope} supported-fields snapshot"
            );
        }
    }
}

#[test]
fn missing_required_scope_returns_error() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let natal = build_chart_from_horoscope_fixture_case(&case);
    let horoscope = HoroscopeChart::with_layers(natal, Vec::new());

    assert_eq!(
        HoroscopeSupportedFieldsSnapshot::from_horoscope_chart(&horoscope),
        Err(ChartError::MissingHoroscopeLayer {
            scope: Scope::Decadal
        })
    );
}

#[test]
fn duplicate_scope_returns_error() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let natal = build_chart_from_horoscope_fixture_case(&case);
    let horoscope = build_full_horoscope_chart(natal, stack_input(&case))
        .expect("full horoscope stack should build");
    let mut layers = horoscope.layers().to_vec();
    layers.push(layers[0].clone());
    let malformed = HoroscopeChart::with_layers(horoscope.natal().clone(), layers);

    assert_eq!(
        HoroscopeSupportedFieldsSnapshot::from_horoscope_chart(&malformed),
        Err(ChartError::DuplicateHoroscopeLayer {
            scope: Scope::Decadal
        })
    );
}

#[test]
fn missing_required_palace_layout_returns_error() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let natal = build_chart_from_horoscope_fixture_case(&case);
    let horoscope = build_full_horoscope_chart(natal, stack_input(&case))
        .expect("full horoscope stack should build");
    let mut layers = horoscope.layers().to_vec();
    let decadal = &layers[0];
    layers[0] = TemporalLayer::try_new(
        decadal.scope(),
        *decadal.context(),
        decadal.placements().to_vec(),
        decadal.activations().to_vec(),
    )
    .expect("malformed test layer should still satisfy temporal invariants");
    let malformed = HoroscopeChart::with_layers(horoscope.natal().clone(), layers);

    assert_eq!(
        HoroscopeSupportedFieldsSnapshot::from_horoscope_chart(&malformed),
        Err(ChartError::MissingHoroscopePalaceLayout {
            scope: Scope::Decadal
        })
    );
}

#[test]
fn supported_fields_snapshot_round_trips_through_json() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
        .expect("full horoscope stack should build");
    let snapshot = HoroscopeSupportedFieldsSnapshot::from_horoscope_chart(&horoscope)
        .expect("supported-fields snapshot should build");

    let encoded = serde_json::to_string(&snapshot).expect("snapshot should serialize");
    let decoded: HoroscopeSupportedFieldsSnapshot =
        serde_json::from_str(&encoded).expect("snapshot should deserialize");

    assert_eq!(decoded, snapshot);
}

#[test]
fn supported_fields_accessors_expose_every_scope_block() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
        .expect("full horoscope stack should build");
    let snapshot = HoroscopeSupportedFieldsSnapshot::from_horoscope_chart(&horoscope)
        .expect("supported-fields snapshot should build");

    assert_eq!(snapshot.decadal().common().name(), Scope::Decadal);
    assert_eq!(snapshot.age().common().name(), Scope::Age);
    assert_eq!(snapshot.yearly().common().name(), Scope::Yearly);
    assert_eq!(snapshot.monthly().common().name(), Scope::Monthly);
    assert_eq!(snapshot.daily().common().name(), Scope::Daily);
    assert_eq!(snapshot.hourly().common().name(), Scope::Hourly);

    let age = &case["supported_fields"]["age"];
    assert_eq!(
        snapshot.age().nominal_age(),
        age["nominal_age"].as_u64().expect("nominal age") as u8
    );
    assert_eq!(
        snapshot.age().common().index(),
        age["index"].as_u64().expect("age index") as usize
    );
    assert_eq!(
        snapshot.age().common().palace_names()[snapshot.age().common().index()].name(),
        iztro::core::PalaceName::Life
    );

    let decadal = snapshot.decadal();
    assert_eq!(decadal.flow_stars().len(), 10);
    assert_eq!(
        decadal.common().mutagen().lu().transform(),
        iztro::core::Mutagen::Lu
    );
    assert_eq!(
        decadal.common().mutagen().quan().transform(),
        iztro::core::Mutagen::Quan
    );
    assert_eq!(
        decadal.common().mutagen().ke().transform(),
        iztro::core::Mutagen::Ke
    );
    assert_eq!(
        decadal.common().mutagen().ji().transform(),
        iztro::core::Mutagen::Ji
    );
    let _ = decadal.common().mutagen().lu().star();

    for scope in [snapshot.monthly(), snapshot.daily(), snapshot.hourly()] {
        assert_eq!(scope.flow_stars().len(), 10);
        assert!(
            scope
                .flow_stars()
                .iter()
                .all(|star| star.kind() == iztro::core::StarKind::Flower
                    || star.kind() == iztro::core::StarKind::LuCun
                    || star.kind() == iztro::core::StarKind::TianMa
                    || star.kind() == iztro::core::StarKind::Soft
                    || star.kind() == iztro::core::StarKind::Tough)
        );
        assert!(
            scope
                .flow_stars()
                .iter()
                .all(|star| star.branch().index() < 12)
        );
        assert!(scope.flow_stars().iter().all(|star| {
            matches!(
                star.base(),
                iztro::core::FlowStarBase::Chang
                    | iztro::core::FlowStarBase::Kui
                    | iztro::core::FlowStarBase::Lu
                    | iztro::core::FlowStarBase::Luan
                    | iztro::core::FlowStarBase::Ma
                    | iztro::core::FlowStarBase::Qu
                    | iztro::core::FlowStarBase::Tuo
                    | iztro::core::FlowStarBase::Xi
                    | iztro::core::FlowStarBase::Yang
                    | iztro::core::FlowStarBase::Yue
            )
        }));
    }

    let yearly = snapshot.yearly();
    assert!(yearly.nian_jie_branch().is_some());
    assert_eq!(yearly.flow_stars().len(), 10);
    assert_eq!(yearly.yearly_dec_stars().suiqian12().len(), 12);
    assert_eq!(yearly.yearly_dec_stars().jiangqian12().len(), 12);
    assert!(
        yearly
            .yearly_dec_stars()
            .suiqian12()
            .iter()
            .all(|star| star.branch().index() < 12 && star.name() != StarName::ZiWei)
    );
    assert!(
        yearly
            .yearly_dec_stars()
            .jiangqian12()
            .iter()
            .all(|star| star.branch().index() < 12 && star.name() != StarName::ZiWei)
    );
}

#[test]
fn supported_fields_snapshot_does_not_change_natal_or_metadata_boundaries() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let typed_count = chart.stars().len();
    let decorative_count = chart.decorative_stars().len();
    let horoscope = build_full_horoscope_chart(chart, stack_input(&case))
        .expect("full horoscope stack should build");

    let _snapshot = HoroscopeSupportedFieldsSnapshot::from_horoscope_chart(&horoscope)
        .expect("supported-fields snapshot should build");

    assert_eq!(horoscope.natal().stars().len(), typed_count);
    assert_eq!(horoscope.natal().decorative_stars().len(), decorative_count);
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

fn expected_scope_snapshot(scope: &Value, scope_name: &str) -> Value {
    let mut out = json!({
        "index": scope["index"],
        "name": scope_name,
        "heavenly_stem": scope["heavenly_stem"],
        "earthly_branch": scope["earthly_branch"],
        "palace_names": scope["palace_names"]
            .as_array()
            .expect("palace names")
            .iter()
            .map(|name| json!({ "name": name["name"] }))
            .collect::<Vec<_>>(),
        "mutagen": expected_mutagen(&scope["mutagen"]),
    });

    if scope_name == "age" {
        out["nominal_age"] = scope["nominal_age"].clone();
    }

    if let Some(flow_stars) = scope["flow_stars"].as_array() {
        out["flow_stars"] = Value::Array(
            flow_stars
                .iter()
                .map(|star| {
                    json!({
                        "base": star["base"],
                        "branch": star["branch"],
                        "type": star["type"],
                    })
                })
                .collect(),
        );
    }

    if scope_name == "yearly" {
        out["nian_jie_branch"] = scope["nian_jie_branch"].clone();
        out["yearly_dec_stars"] = json!({
            "suiqian12": expected_yearly_dec_family(&scope["yearly_dec_stars"]["suiqian12"]),
            "jiangqian12": expected_yearly_dec_family(&scope["yearly_dec_stars"]["jiangqian12"]),
        });
    }

    out
}

fn expected_mutagen(mutagen: &Value) -> Value {
    json!({
        "lu": expected_mutagen_entry(&mutagen["lu"]),
        "quan": expected_mutagen_entry(&mutagen["quan"]),
        "ke": expected_mutagen_entry(&mutagen["ke"]),
        "ji": expected_mutagen_entry(&mutagen["ji"]),
    })
}

fn expected_mutagen_entry(entry: &Value) -> Value {
    json!({
        "transform": entry["transform"],
        "star": entry["star"],
    })
}

fn expected_yearly_dec_family(family: &Value) -> Value {
    Value::Array(
        family
            .as_array()
            .expect("yearly dec family")
            .iter()
            .map(|star| {
                json!({
                    "name": star["name"],
                    "branch": star["branch"],
                })
            })
            .collect(),
    )
}
