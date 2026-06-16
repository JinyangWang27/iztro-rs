mod common;

use std::collections::HashMap;

use common::{parse_algorithm, parse_key};
use iztro::core::{
    BirthTime, Chart, ChartLayerKind, ChartStackSnapshot, EarthlyBranch, FlowStarBase,
    FlowStarScope, Gender, HeavenlyStem, HoroscopeChart, HourlyPeriod, LunarChartRequest, LunarDay,
    LunarMonth, MethodProfile, Mutagen, PalaceName, Scope, SolarDay, SolarMonth, StarKind,
    StarName, StemBranch, TemporalContext, build_hourly_horoscope_layer, build_hourly_period,
    by_lunar, flow_star_name,
};
use serde_json::Value;

const HOROSCOPE_FIXTURE: &str = include_str!("../fixtures/iztro/horoscope.json");
const CANONICAL_CASE_ID: &str = "canonical_female_default_2026";
// In this case the hourly Life branch (亥) differs from the hourly stem-branch
// branch (寅), proving the two facts are modeled independently.
const INDEPENDENT_CASE_ID: &str = "canonical_female_default_2034_decade_boundary";

#[test]
fn hourly_period_and_layer_match_all_fixture_cases() {
    for case in horoscope_fixture_cases() {
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let hourly = hourly_fixture(&case);
        let period = build_fixture_hourly_period(&chart, &case, hourly);
        let layer = build_hourly_horoscope_layer(&chart, &period)
            .expect("hourly horoscope layer should build");

        assert_hourly_period_matches_fixture(&period, hourly, &case);
        assert_hourly_palace_layout_matches_fixture(
            layer
                .palace_layout()
                .expect("hourly layer should carry palace layout"),
            hourly,
        );
        assert_eq!(
            actual_hourly_mutagens(&layer),
            expected_hourly_mutagens(hourly, &chart)
        );
        assert_eq!(
            actual_hourly_flow_stars(&layer),
            expected_hourly_flow_stars(hourly)
        );
    }
}

#[test]
fn hourly_stem_branch_is_independent_from_hourly_life_branch() {
    let case = horoscope_fixture_case(INDEPENDENT_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let hourly = hourly_fixture(&case);
    let period = build_fixture_hourly_period(&chart, &case, hourly);

    assert_ne!(
        period.palace_branch(),
        period.stem_branch().branch(),
        "fixture proves hourly stem-branch branch is independent from hourly Life branch"
    );
    // Stem-branch comes from the hour pillar; the Life branch comes from the
    // derived index — both are validated against the fixture separately.
    assert_eq!(period.stem_branch(), hourly_stem_branch(hourly));
    assert_eq!(
        period.palace_branch(),
        EarthlyBranch::Yin.offset(hourly_index(hourly) as isize)
    );
}

#[test]
fn hourly_horoscope_layer_matches_canonical_fixture_context() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let hourly = hourly_fixture(&case);
    let period = build_fixture_hourly_period(&chart, &case, hourly);

    let layer =
        build_hourly_horoscope_layer(&chart, &period).expect("hourly horoscope layer should build");

    assert_eq!(layer.scope(), Scope::Hourly);
    assert_eq!(
        *layer.context(),
        TemporalContext::Hourly {
            stem_branch: hourly_stem_branch(hourly),
        }
    );
}

#[test]
fn hourly_horoscope_layer_matches_canonical_fixture_palace_layout() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let hourly = hourly_fixture(&case);
    let period = build_fixture_hourly_period(&chart, &case, hourly);

    let layer =
        build_hourly_horoscope_layer(&chart, &period).expect("hourly horoscope layer should build");

    assert_hourly_palace_layout_matches_fixture(
        layer
            .palace_layout()
            .expect("hourly layer should carry palace layout"),
        hourly,
    );
}

#[test]
fn hourly_horoscope_layer_matches_canonical_fixture_mutagens() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let hourly = hourly_fixture(&case);
    let period = build_fixture_hourly_period(&chart, &case, hourly);

    let layer =
        build_hourly_horoscope_layer(&chart, &period).expect("hourly horoscope layer should build");

    assert_eq!(
        actual_hourly_mutagens(&layer),
        expected_hourly_mutagens(hourly, &chart)
    );
}

#[test]
fn hourly_horoscope_layer_matches_canonical_fixture_flow_stars() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let hourly = hourly_fixture(&case);
    let period = build_fixture_hourly_period(&chart, &case, hourly);

    let layer =
        build_hourly_horoscope_layer(&chart, &period).expect("hourly horoscope layer should build");

    assert_eq!(
        actual_hourly_flow_stars(&layer),
        expected_hourly_flow_stars(hourly)
    );
    assert!(
        layer
            .placements()
            .iter()
            .all(|placement| placement.placement().name() != StarName::NianJieYearly),
        "hourly layer should not include yearly-only NianJieYearly"
    );
}

#[test]
fn hourly_snapshot_keeps_natal_and_hourly_facts_separate() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let hourly = hourly_fixture(&case);
    let period = build_fixture_hourly_period(&chart, &case, hourly);
    let layer =
        build_hourly_horoscope_layer(&chart, &period).expect("hourly horoscope layer should build");
    let horoscope = HoroscopeChart::with_layers(chart.clone(), vec![layer]);

    let snapshot = ChartStackSnapshot::from_horoscope_chart(&horoscope);

    assert_eq!(snapshot.layers().len(), 2);
    let natal = snapshot
        .layer(ChartLayerKind::Natal)
        .expect("natal snapshot layer should exist");
    assert!(natal.cells().iter().all(|cell| {
        cell.temporal_palace_name().is_none()
            && cell.scoped_stars().is_empty()
            && cell.mutagen_activations().is_empty()
    }));
    assert_eq!(
        natal
            .cells()
            .iter()
            .map(|cell| cell.typed_stars().len())
            .sum::<usize>(),
        chart.stars().len()
    );

    let hourly_layer = snapshot
        .layer(ChartLayerKind::Hourly)
        .expect("hourly snapshot layer should exist");
    assert_eq!(
        hourly_layer.context(),
        Some(&TemporalContext::Hourly {
            stem_branch: hourly_stem_branch(hourly),
        })
    );
    assert!(
        hourly_layer
            .cells()
            .iter()
            .all(|cell| cell.typed_stars().is_empty())
    );
    assert!(
        hourly_layer
            .cells()
            .iter()
            .all(|cell| cell.decorative_stars().is_empty())
    );

    let expected_palaces = expected_hourly_palace_names_by_branch(hourly);
    for cell in hourly_layer.cells() {
        let natal_palace = chart
            .palaces()
            .iter()
            .find(|palace| palace.branch() == cell.branch())
            .expect("natal branch should have a palace");
        assert_eq!(cell.natal_palace_name(), Some(natal_palace.name()));
        assert_eq!(
            cell.temporal_palace_name(),
            expected_palaces.get(&cell.branch()).copied()
        );
    }

    let hourly_scoped_count: usize = hourly_layer
        .cells()
        .iter()
        .map(|cell| cell.scoped_stars().len())
        .sum();
    assert_eq!(
        hourly_scoped_count,
        expected_hourly_flow_stars(hourly).len()
    );

    let hourly_mutagen_count: usize = hourly_layer
        .cells()
        .iter()
        .map(|cell| cell.mutagen_activations().len())
        .sum();
    assert_eq!(
        hourly_mutagen_count,
        expected_hourly_mutagens(hourly, &chart).len()
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

fn horoscope_fixture_cases() -> Vec<Value> {
    let fixture: Value =
        serde_json::from_str(HOROSCOPE_FIXTURE).expect("horoscope fixture should parse");

    fixture["cases"]
        .as_array()
        .expect("fixture cases should be an array")
        .to_vec()
}

fn build_chart_from_horoscope_fixture_case(case: &Value) -> Chart {
    let input = &case["input"];
    assert_eq!(
        input["calendar"].as_str(),
        Some("lunar"),
        "hourly horoscope fixtures should build through by_lunar"
    );
    let lunar_year = input["year"].as_i64().expect("fixture lunar year") as i32;
    let birth_year = StemBranch::from_lunar_year(lunar_year);
    let method_profile = MethodProfile::new(
        case["id"].as_str().expect("case id"),
        parse_algorithm(input["algorithm"].as_str().expect("algorithm")),
        "hourly horoscope fixture test",
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

    by_lunar(request).expect("by_lunar should build hourly horoscope fixture chart")
}

fn hourly_fixture(case: &Value) -> &Value {
    &case["supported_fields"]["hourly"]
}

fn build_fixture_hourly_period(chart: &Chart, case: &Value, hourly: &Value) -> HourlyPeriod {
    let (target_year, target_month, target_day) = target_solar_date(case);
    let period = build_hourly_period(
        chart,
        target_year,
        SolarMonth::new(target_month).expect("target solar month should be valid"),
        SolarDay::new(target_day).expect("target solar day should be valid"),
        target_time(case),
    )
    .expect("hourly period should build");
    assert_eq!(
        period.stem_branch(),
        hourly_stem_branch(hourly),
        "fixture target date/time should derive fixture hourly stem-branch"
    );
    period
}

fn assert_hourly_period_matches_fixture(period: &HourlyPeriod, hourly: &Value, case: &Value) {
    assert_eq!(period.index(), hourly_index(hourly));
    assert_eq!(period.time_index(), target_time_index(case));
    assert_eq!(period.stem_branch(), hourly_stem_branch(hourly));
    assert_eq!(
        period.palace_branch(),
        EarthlyBranch::Yin.offset(hourly_index(hourly) as isize)
    );
    assert_hourly_palace_layout_matches_fixture(period.palace_layout(), hourly);
}

fn assert_hourly_palace_layout_matches_fixture(
    layout: &iztro::core::TemporalPalaceLayout,
    hourly: &Value,
) {
    assert_eq!(layout.scope(), Scope::Hourly);
    assert_eq!(layout.names().len(), 12);

    let expected = expected_hourly_palace_names_by_branch(hourly);
    for name in layout.names() {
        assert_eq!(
            Some(name.palace_name()),
            expected.get(&name.branch()).copied(),
            "hourly palace name mismatch at {:?}",
            name.branch()
        );
    }
    assert_eq!(
        layout.name_for_branch(EarthlyBranch::Yin.offset(hourly_index(hourly) as isize)),
        Some(PalaceName::Life)
    );
}

fn expected_hourly_palace_names_by_branch(hourly: &Value) -> HashMap<EarthlyBranch, PalaceName> {
    hourly["palace_names"]
        .as_array()
        .expect("hourly palace names")
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

fn expected_hourly_mutagens(
    hourly: &Value,
    chart: &Chart,
) -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    hourly["mutagen"]
        .as_object()
        .expect("hourly mutagen map")
        .iter()
        .filter_map(|(transform, entry)| {
            let star = parse_key::<StarName>(entry["star"].as_str().expect("mutagen star"));
            let branch = chart.star(star).map(|fact| fact.palace().branch())?;
            Some(((star, branch), parse_key::<Mutagen>(transform)))
        })
        .collect()
}

fn actual_hourly_mutagens(
    layer: &iztro::core::TemporalLayer,
) -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    layer
        .activations()
        .iter()
        .map(|activation| {
            assert_eq!(activation.source_scope(), Scope::Hourly);
            (
                (activation.target_star(), activation.target_branch()),
                activation.mutagen(),
            )
        })
        .collect()
}

fn expected_hourly_flow_stars(hourly: &Value) -> HashMap<StarName, (EarthlyBranch, StarKind)> {
    hourly["flow_stars"]
        .as_array()
        .expect("hourly flow stars")
        .iter()
        .map(|entry| {
            let base = parse_flow_base(entry["base"].as_str().expect("flow star base"));
            (
                flow_star_name(FlowStarScope::Hourly, base),
                (
                    parse_key::<EarthlyBranch>(entry["branch"].as_str().expect("branch")),
                    kind_from_type(entry["type"].as_str().expect("type")),
                ),
            )
        })
        .collect()
}

fn actual_hourly_flow_stars(
    layer: &iztro::core::TemporalLayer,
) -> HashMap<StarName, (EarthlyBranch, StarKind)> {
    layer
        .placements()
        .iter()
        .map(|placement| {
            assert_eq!(placement.scope(), Scope::Hourly);
            (
                placement.placement().name(),
                (placement.branch(), placement.placement().kind()),
            )
        })
        .collect()
}

fn hourly_stem_branch(hourly: &Value) -> StemBranch {
    StemBranch::try_new(
        parse_key::<HeavenlyStem>(hourly["heavenly_stem"].as_str().expect("hourly stem")),
        parse_key::<EarthlyBranch>(hourly["earthly_branch"].as_str().expect("hourly branch")),
    )
    .expect("fixture hourly stem-branch should be valid")
}

fn hourly_index(hourly: &Value) -> usize {
    hourly["index"].as_u64().expect("hourly index") as usize
}

fn target_solar_date(case: &Value) -> (i32, u8, u8) {
    let raw = case["input"]["target"]["solar_date"]
        .as_str()
        .expect("target solar date");
    let parts: Vec<_> = raw.split('-').collect();
    assert_eq!(parts.len(), 3);
    (
        parts[0].parse().expect("target solar year"),
        parts[1].parse().expect("target solar month"),
        parts[2].parse().expect("target solar day"),
    )
}

fn target_time_index(case: &Value) -> u8 {
    case["input"]["target"]["time_index"]
        .as_u64()
        .expect("target time index") as u8
}

fn target_time(case: &Value) -> BirthTime {
    BirthTime::from_iztro_time_index(target_time_index(case))
        .expect("target time index should be valid")
}

fn parse_flow_base(value: &str) -> FlowStarBase {
    match value {
        "kui" => FlowStarBase::Kui,
        "yue" => FlowStarBase::Yue,
        "chang" => FlowStarBase::Chang,
        "qu" => FlowStarBase::Qu,
        "lu" => FlowStarBase::Lu,
        "yang" => FlowStarBase::Yang,
        "tuo" => FlowStarBase::Tuo,
        "ma" => FlowStarBase::Ma,
        "luan" => FlowStarBase::Luan,
        "xi" => FlowStarBase::Xi,
        other => panic!("unsupported flow base: {other}"),
    }
}

fn kind_from_type(value: &str) -> StarKind {
    match value {
        "soft" => StarKind::Soft,
        "tough" => StarKind::Tough,
        "lucun" => StarKind::LuCun,
        "tianma" => StarKind::TianMa,
        "flower" => StarKind::Flower,
        "helper" => StarKind::Helper,
        other => panic!("unsupported flow star type: {other}"),
    }
}

fn parse_gender(value: &str) -> Gender {
    match value {
        "female" => Gender::Female,
        "male" => Gender::Male,
        other => panic!("unsupported fixture gender: {other}"),
    }
}
