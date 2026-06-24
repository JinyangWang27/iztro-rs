mod common;

use std::collections::HashMap;

use common::{parse_algorithm, parse_key, target_lunar_date};
use iztro::core::{
    BirthTime, Chart, ChartLayerKind, ChartStackSnapshot, DailyPeriod, EarthlyBranch, FlowStarBase,
    FlowStarScope, Gender, HeavenlyStem, HoroscopeChart, LunarChartRequest, LunarDay, LunarMonth,
    MethodProfile, Mutagen, PalaceName, Scope, SolarDay, SolarMonth, StarKind, StarName,
    StemBranch, TemporalContext, build_daily_horoscope_layer, build_daily_period, by_lunar,
    flow_star_name,
};
use serde_json::Value;

const HOROSCOPE_FIXTURE: &str = include_str!("../fixtures/iztro/horoscope.json");
const CANONICAL_CASE_ID: &str = "canonical_female_default_2026";

#[test]
fn daily_period_and_layer_match_all_fixture_cases() {
    for case in horoscope_fixture_cases() {
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let daily = daily_fixture(&case);
        let period = build_fixture_daily_period(&chart, &case, daily);
        let layer = build_daily_horoscope_layer(&chart, &period)
            .expect("daily horoscope layer should build");

        assert_daily_period_matches_fixture(&period, daily, &case);
        assert_daily_palace_layout_matches_fixture(
            layer
                .palace_layout()
                .expect("daily layer should carry palace layout"),
            daily,
        );
        assert_eq!(
            actual_daily_mutagens(&layer),
            expected_daily_mutagens(daily, &chart)
        );
        assert_eq!(
            actual_daily_flow_stars(&layer),
            expected_daily_flow_stars(daily)
        );
    }
}

#[test]
fn daily_horoscope_layer_matches_canonical_fixture_context() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let daily = daily_fixture(&case);
    let period = build_fixture_daily_period(&chart, &case, daily);

    assert_ne!(
        period.palace_branch(),
        period.stem_branch().branch(),
        "canonical fixture proves daily stem-branch branch is independent from daily Life branch"
    );

    let layer =
        build_daily_horoscope_layer(&chart, &period).expect("daily horoscope layer should build");

    assert_eq!(layer.scope(), Scope::Daily);
    assert_eq!(
        *layer.context(),
        TemporalContext::Daily {
            stem_branch: daily_stem_branch(daily),
            lunar_day: target_lunar_day(&case),
        }
    );
}

#[test]
fn daily_horoscope_layer_matches_canonical_fixture_palace_layout() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let daily = daily_fixture(&case);
    let period = build_fixture_daily_period(&chart, &case, daily);

    let layer =
        build_daily_horoscope_layer(&chart, &period).expect("daily horoscope layer should build");

    assert_daily_palace_layout_matches_fixture(
        layer
            .palace_layout()
            .expect("daily layer should carry palace layout"),
        daily,
    );
}

#[test]
fn daily_horoscope_layer_matches_canonical_fixture_mutagens() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let daily = daily_fixture(&case);
    let period = build_fixture_daily_period(&chart, &case, daily);

    let layer =
        build_daily_horoscope_layer(&chart, &period).expect("daily horoscope layer should build");

    assert_eq!(
        actual_daily_mutagens(&layer),
        expected_daily_mutagens(daily, &chart)
    );
}

#[test]
fn daily_horoscope_layer_matches_canonical_fixture_flow_stars() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let daily = daily_fixture(&case);
    let period = build_fixture_daily_period(&chart, &case, daily);

    let layer =
        build_daily_horoscope_layer(&chart, &period).expect("daily horoscope layer should build");

    assert_eq!(
        actual_daily_flow_stars(&layer),
        expected_daily_flow_stars(daily)
    );
    assert!(
        layer
            .placements()
            .iter()
            .all(|placement| placement.placement().name() != StarName::NianJieYearly),
        "daily layer should not include yearly-only NianJieYearly"
    );
}

#[test]
fn daily_snapshot_keeps_natal_and_daily_facts_separate() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let daily = daily_fixture(&case);
    let period = build_fixture_daily_period(&chart, &case, daily);
    let layer =
        build_daily_horoscope_layer(&chart, &period).expect("daily horoscope layer should build");
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

    let daily_layer = snapshot
        .layer(ChartLayerKind::Daily)
        .expect("daily snapshot layer should exist");
    assert_eq!(
        daily_layer.context(),
        Some(&TemporalContext::Daily {
            stem_branch: daily_stem_branch(daily),
            lunar_day: target_lunar_day(&case),
        })
    );
    assert!(
        daily_layer
            .cells()
            .iter()
            .all(|cell| cell.typed_stars().is_empty())
    );
    assert!(
        daily_layer
            .cells()
            .iter()
            .all(|cell| cell.decorative_stars().is_empty())
    );

    let expected_palaces = expected_daily_palace_names_by_branch(daily);
    for cell in daily_layer.cells() {
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

    let daily_scoped_count: usize = daily_layer
        .cells()
        .iter()
        .map(|cell| cell.scoped_stars().len())
        .sum();
    assert_eq!(daily_scoped_count, expected_daily_flow_stars(daily).len());

    let daily_mutagen_count: usize = daily_layer
        .cells()
        .iter()
        .map(|cell| cell.mutagen_activations().len())
        .sum();
    assert_eq!(
        daily_mutagen_count,
        expected_daily_mutagens(daily, &chart).len()
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
        "daily horoscope fixtures should build through by_lunar"
    );
    let lunar_year = input["year"].as_i64().expect("fixture lunar year") as i32;
    let birth_year = StemBranch::from_lunar_year(lunar_year);
    let method_profile = MethodProfile::new(
        case["id"].as_str().expect("case id"),
        parse_algorithm(input["algorithm"].as_str().expect("algorithm")),
        "daily horoscope fixture test",
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

    by_lunar(request).expect("by_lunar should build daily horoscope fixture chart")
}

fn daily_fixture(case: &Value) -> &Value {
    &case["supported_fields"]["daily"]
}

fn build_fixture_daily_period(chart: &Chart, case: &Value, daily: &Value) -> DailyPeriod {
    let (target_year, target_month, target_day) = target_solar_date(case);
    let period = build_daily_period(
        chart,
        target_year,
        SolarMonth::new(target_month).expect("target solar month should be valid"),
        SolarDay::new(target_day).expect("target solar day should be valid"),
        target_time(case),
    )
    .expect("daily period should build");
    assert_eq!(
        period.stem_branch(),
        daily_stem_branch(daily),
        "fixture target date should derive fixture daily stem-branch"
    );
    period
}

fn assert_daily_period_matches_fixture(period: &DailyPeriod, daily: &Value, case: &Value) {
    assert_eq!(period.index(), daily_index(daily));
    assert_eq!(period.lunar_day(), target_lunar_day(case));
    assert_eq!(period.stem_branch(), daily_stem_branch(daily));
    assert_eq!(
        period.palace_branch(),
        EarthlyBranch::Yin.offset(daily_index(daily) as isize)
    );
    assert_daily_palace_layout_matches_fixture(period.palace_layout(), daily);
}

fn assert_daily_palace_layout_matches_fixture(
    layout: &iztro::core::TemporalPalaceLayout,
    daily: &Value,
) {
    assert_eq!(layout.scope(), Scope::Daily);
    assert_eq!(layout.names().len(), 12);

    let expected = expected_daily_palace_names_by_branch(daily);
    for name in layout.names() {
        assert_eq!(
            Some(name.palace_name()),
            expected.get(&name.branch()).copied(),
            "daily palace name mismatch at {:?}",
            name.branch()
        );
    }
    assert_eq!(
        layout.name_for_branch(EarthlyBranch::Yin.offset(daily_index(daily) as isize)),
        Some(PalaceName::Life)
    );
}

fn expected_daily_palace_names_by_branch(daily: &Value) -> HashMap<EarthlyBranch, PalaceName> {
    daily["palace_names"]
        .as_array()
        .expect("daily palace names")
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

fn expected_daily_mutagens(
    daily: &Value,
    chart: &Chart,
) -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    daily["mutagen"]
        .as_object()
        .expect("daily mutagen map")
        .iter()
        .filter_map(|(transform, entry)| {
            let star = parse_key::<StarName>(entry["star"].as_str().expect("mutagen star"));
            let branch = chart.star(star).map(|fact| fact.palace().branch())?;
            Some(((star, branch), parse_key::<Mutagen>(transform)))
        })
        .collect()
}

fn actual_daily_mutagens(
    layer: &iztro::core::TemporalLayer,
) -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    layer
        .activations()
        .iter()
        .map(|activation| {
            assert_eq!(activation.source_scope(), Scope::Daily);
            (
                (activation.target_star(), activation.target_branch()),
                activation.mutagen(),
            )
        })
        .collect()
}

fn expected_daily_flow_stars(daily: &Value) -> HashMap<StarName, (EarthlyBranch, StarKind)> {
    daily["flow_stars"]
        .as_array()
        .expect("daily flow stars")
        .iter()
        .map(|entry| {
            let base = parse_flow_base(entry["base"].as_str().expect("flow star base"));
            (
                flow_star_name(FlowStarScope::Daily, base),
                (
                    parse_key::<EarthlyBranch>(entry["branch"].as_str().expect("branch")),
                    kind_from_type(entry["type"].as_str().expect("type")),
                ),
            )
        })
        .collect()
}

fn actual_daily_flow_stars(
    layer: &iztro::core::TemporalLayer,
) -> HashMap<StarName, (EarthlyBranch, StarKind)> {
    layer
        .placements()
        .iter()
        .map(|placement| {
            assert_eq!(placement.scope(), Scope::Daily);
            (
                placement.placement().name(),
                (placement.branch(), placement.placement().kind()),
            )
        })
        .collect()
}

fn daily_stem_branch(daily: &Value) -> StemBranch {
    StemBranch::try_new(
        parse_key::<HeavenlyStem>(daily["heavenly_stem"].as_str().expect("daily stem")),
        parse_key::<EarthlyBranch>(daily["earthly_branch"].as_str().expect("daily branch")),
    )
    .expect("fixture daily stem-branch should be valid")
}

fn daily_index(daily: &Value) -> usize {
    daily["index"].as_u64().expect("daily index") as usize
}

fn target_lunar_day(case: &Value) -> u8 {
    target_lunar_date(case).day
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

fn target_time(case: &Value) -> BirthTime {
    BirthTime::from_iztro_time_index(
        case["input"]["target"]["time_index"]
            .as_u64()
            .expect("target time index") as u8,
    )
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
