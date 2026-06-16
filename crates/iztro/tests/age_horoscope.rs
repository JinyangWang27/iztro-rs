mod common;

use std::collections::HashMap;

use common::{parse_algorithm, parse_key};
use iztro::core::{
    AgePeriod, Chart, ChartError, ChartLayerKind, ChartStackSnapshot, EarthlyBranch, Gender,
    HeavenlyStem, HoroscopeChart, LunarChartRequest, LunarDay, LunarMonth, MethodProfile, Mutagen,
    PalaceName, Scope, StarName, StemBranch, TemporalContext, build_age_horoscope_layer,
    build_age_period, by_lunar,
};
use serde_json::Value;

const HOROSCOPE_FIXTURE: &str = include_str!("../fixtures/iztro/horoscope.json");
const CANONICAL_CASE_ID: &str = "canonical_female_default_2026";

#[test]
fn age_scope_is_non_natal_temporal_scope() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let age = age_fixture(&case);
    let stem_branch = age_stem_branch(age);
    let context = TemporalContext::Age {
        stem_branch,
        nominal_age: nominal_age(age),
    };

    assert_eq!(context.scope(), Scope::Age);
    assert_eq!(context.stem_branch(), stem_branch);
    assert_ne!(context.scope(), Scope::Natal);
    assert_eq!(ChartLayerKind::from_scope(Scope::Age), ChartLayerKind::Age);
}

#[test]
fn age_horoscope_layer_matches_canonical_fixture_context() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let age = age_fixture(&case);
    let period = build_fixture_age_period(&chart, age);

    assert_age_period_matches_fixture(&period, age);

    let layer =
        build_age_horoscope_layer(&chart, &period).expect("age horoscope layer should build");

    assert_eq!(layer.scope(), Scope::Age);
    assert_eq!(
        *layer.context(),
        TemporalContext::Age {
            stem_branch: age_stem_branch(age),
            nominal_age: nominal_age(age),
        }
    );
    assert!(layer.placements().is_empty());
}

#[test]
fn age_horoscope_layer_matches_canonical_fixture_palace_layout() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let age = age_fixture(&case);
    let period = build_fixture_age_period(&chart, age);
    let layer =
        build_age_horoscope_layer(&chart, &period).expect("age horoscope layer should build");

    assert_age_palace_layout_matches_fixture(
        layer
            .palace_layout()
            .expect("age layer should carry palace layout"),
        age,
    );
}

#[test]
fn age_horoscope_layer_matches_canonical_fixture_mutagens() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let age = age_fixture(&case);
    let period = build_fixture_age_period(&chart, age);
    let layer =
        build_age_horoscope_layer(&chart, &period).expect("age horoscope layer should build");

    assert_eq!(
        actual_age_mutagens(&layer),
        expected_age_mutagens(age, &chart)
    );
}

#[test]
fn age_snapshot_exposes_temporal_palace_names_separately_from_natal() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let age_fixture = age_fixture(&case);
    let period = build_fixture_age_period(&chart, age_fixture);
    let layer =
        build_age_horoscope_layer(&chart, &period).expect("age horoscope layer should build");
    let horoscope = HoroscopeChart::with_layers(chart.clone(), vec![layer]);

    let snapshot = ChartStackSnapshot::from_horoscope_chart(&horoscope);

    assert_eq!(snapshot.layers().len(), 2);
    assert_eq!(snapshot.layers()[0].kind(), ChartLayerKind::Natal);

    let age = &snapshot.layers()[1];
    assert_eq!(age.kind(), ChartLayerKind::Age);
    assert_eq!(
        age.context(),
        Some(&TemporalContext::Age {
            stem_branch: age_stem_branch(age_fixture),
            nominal_age: nominal_age(age_fixture),
        })
    );
    assert!(age.cells().iter().all(|cell| cell.typed_stars().is_empty()));
    assert!(
        age.cells()
            .iter()
            .all(|cell| cell.decorative_stars().is_empty())
    );
    assert!(
        age.cells()
            .iter()
            .all(|cell| cell.scoped_stars().is_empty())
    );

    let expected = expected_age_palace_names_by_branch(age_fixture);
    for cell in age.cells() {
        let natal_palace = chart
            .palaces()
            .iter()
            .find(|palace| palace.branch() == cell.branch())
            .expect("natal branch should have a palace");
        assert_eq!(cell.natal_palace_name(), Some(natal_palace.name()));
        assert_eq!(
            cell.temporal_palace_name(),
            expected.get(&cell.branch()).copied()
        );
    }

    let age_life_branch = age_stem_branch(age_fixture).branch();
    let life_cell = age
        .cells()
        .iter()
        .find(|cell| cell.branch() == age_life_branch)
        .expect("age Life branch should have a snapshot cell");
    assert_ne!(
        life_cell.natal_palace_name(),
        life_cell.temporal_palace_name()
    );
    assert_eq!(life_cell.temporal_palace_name(), Some(PalaceName::Life));
}

#[test]
fn age_horoscope_layer_round_trips_through_json() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let age = age_fixture(&case);
    let period = build_fixture_age_period(&chart, age);
    let layer =
        build_age_horoscope_layer(&chart, &period).expect("age horoscope layer should build");

    let encoded = serde_json::to_string(&layer).expect("age layer should serialize");
    let decoded: iztro::core::TemporalLayer =
        serde_json::from_str(&encoded).expect("age layer should deserialize");

    assert_eq!(decoded, layer);
    assert_eq!(decoded.scope(), Scope::Age);
    assert!(decoded.placements().is_empty());
    assert!(decoded.palace_layout().is_some());
}

#[test]
fn age_period_rejects_nominal_age_outside_supported_range() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);

    for value in [0, 121] {
        assert_eq!(
            build_age_period(&chart, value).unwrap_err(),
            ChartError::InvalidNominalAge { value }
        );
    }
}

#[test]
fn age_period_matches_male_fixture_direction() {
    let case = male_horoscope_fixture_case();
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let age = age_fixture(&case);
    let period = build_fixture_age_period(&chart, age);
    let layer =
        build_age_horoscope_layer(&chart, &period).expect("age horoscope layer should build");

    assert_eq!(
        case["input"]["gender"].as_str(),
        Some("male"),
        "fixture case should exercise male age direction"
    );
    assert_age_period_matches_fixture(&period, age);
    assert_eq!(
        actual_age_mutagens(&layer),
        expected_age_mutagens(age, &chart)
    );
}

fn horoscope_fixture_case(case_id: &str) -> Value {
    let fixture: Value =
        serde_json::from_str(HOROSCOPE_FIXTURE).expect("horoscope fixture should parse");

    fixture["cases"]
        .as_array()
        .expect("fixture cases should be an array")
        .iter()
        .find(|case| case["id"].as_str() == Some(case_id))
        .unwrap_or_else(|| panic!("missing horoscope fixture case {case_id}"))
        .clone()
}

fn male_horoscope_fixture_case() -> Value {
    let fixture: Value =
        serde_json::from_str(HOROSCOPE_FIXTURE).expect("horoscope fixture should parse");

    fixture["cases"]
        .as_array()
        .expect("fixture cases should be an array")
        .iter()
        .find(|case| case["input"]["gender"].as_str() == Some("male"))
        .expect("fixture should include a male case")
        .clone()
}

fn build_chart_from_horoscope_fixture_case(case: &Value) -> Chart {
    let input = &case["input"];
    assert_eq!(
        input["calendar"].as_str(),
        Some("lunar"),
        "age horoscope fixtures should build through by_lunar"
    );
    let lunar_year = input["year"].as_i64().expect("fixture lunar year") as i32;
    let birth_year = StemBranch::from_lunar_year(lunar_year);
    let method_profile = MethodProfile::new(
        case["id"].as_str().expect("case id"),
        parse_algorithm(input["algorithm"].as_str().expect("algorithm")),
        "age horoscope fixture test",
    );
    let request = LunarChartRequest::builder()
        .lunar_year(lunar_year)
        .lunar_month(
            LunarMonth::new(input["month"].as_u64().expect("fixture lunar month") as u8)
                .expect("fixture lunar month should be valid"),
        )
        .lunar_day(
            LunarDay::new(input["day"].as_u64().expect("fixture lunar day") as u8)
                .expect("fixture lunar day should be valid"),
        )
        .iztro_time_index(input["time_index"].as_u64().expect("fixture time index") as u8)
        .expect("fixture time index should be valid")
        .gender(parse_gender(
            input["gender"].as_str().expect("fixture gender"),
        ))
        .birth_year_stem(birth_year.stem())
        .birth_year_branch(birth_year.branch())
        .is_leap_month(
            input["is_leap_month"]
                .as_bool()
                .expect("fixture leap-month flag"),
        )
        .fix_leap(input["fix_leap"].as_bool().expect("fixture fix-leap flag"))
        .method_profile(method_profile)
        .build()
        .expect("lunar chart request should build from fixture");

    by_lunar(request).expect("by_lunar should build age horoscope fixture chart")
}

fn age_fixture(case: &Value) -> &Value {
    &case["supported_fields"]["age"]
}

fn expected_age_palace_names_by_branch(age: &Value) -> HashMap<EarthlyBranch, PalaceName> {
    age["palace_names"]
        .as_array()
        .expect("age palace names")
        .iter()
        .enumerate()
        .map(|(index, palace)| {
            (
                EarthlyBranch::Yin.offset(index as isize),
                parse_key::<PalaceName>(palace["name"].as_str().expect("palace name")),
            )
        })
        .collect()
}

fn expected_age_mutagens(
    age: &Value,
    chart: &Chart,
) -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    age["mutagen"]
        .as_object()
        .expect("age mutagen map")
        .iter()
        .filter_map(|(transform, entry)| {
            let star = parse_key::<StarName>(entry["star"].as_str().expect("mutagen star"));
            let branch = chart.star(star).map(|fact| fact.palace().branch())?;
            Some(((star, branch), parse_key::<Mutagen>(transform)))
        })
        .collect()
}

fn actual_age_mutagens(
    layer: &iztro::core::TemporalLayer,
) -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    layer
        .activations()
        .iter()
        .map(|activation| {
            assert_eq!(activation.source_scope(), Scope::Age);
            (
                (activation.target_star(), activation.target_branch()),
                activation.mutagen(),
            )
        })
        .collect()
}

fn build_fixture_age_period(chart: &Chart, age: &Value) -> AgePeriod {
    build_age_period(chart, nominal_age(age)).expect("age period should build")
}

fn assert_age_period_matches_fixture(period: &AgePeriod, age: &Value) {
    assert_eq!(period.index(), age_index(age));
    assert_eq!(period.nominal_age(), nominal_age(age));
    assert_eq!(period.palace_branch(), age_stem_branch(age).branch());
    assert_eq!(period.stem_branch(), age_stem_branch(age));
    assert_age_palace_layout_matches_fixture(period.palace_layout(), age);
}

fn assert_age_palace_layout_matches_fixture(
    layout: &iztro::core::TemporalPalaceLayout,
    age: &Value,
) {
    assert_eq!(layout.scope(), Scope::Age);
    assert_eq!(layout.names().len(), 12);

    let expected = expected_age_palace_names_by_branch(age);
    for name in layout.names() {
        assert_eq!(
            Some(name.palace_name()),
            expected.get(&name.branch()).copied(),
            "age palace name mismatch at {:?}",
            name.branch()
        );
    }
    assert_eq!(
        layout.name_for_branch(age_stem_branch(age).branch()),
        Some(PalaceName::Life)
    );
}

fn age_stem_branch(age: &Value) -> StemBranch {
    StemBranch::try_new(
        parse_key::<HeavenlyStem>(age["heavenly_stem"].as_str().expect("age stem")),
        parse_key::<EarthlyBranch>(age["earthly_branch"].as_str().expect("age branch")),
    )
    .expect("fixture age stem-branch should be valid")
}

fn age_index(age: &Value) -> usize {
    age["index"].as_u64().expect("age index") as usize
}

fn nominal_age(age: &Value) -> u8 {
    age["nominal_age"].as_u64().expect("age nominal age") as u8
}

fn parse_gender(value: &str) -> Gender {
    match value {
        "female" => Gender::Female,
        "male" => Gender::Male,
        other => panic!("unsupported fixture gender: {other}"),
    }
}
