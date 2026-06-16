mod common;

use std::collections::HashMap;

use common::{parse_algorithm, parse_key};
use iztro::core::{
    Chart, ChartLayerKind, ChartStackSnapshot, EarthlyBranch, FlowStarBase, FlowStarScope, Gender,
    HeavenlyStem, HoroscopeChart, LunarChartRequest, LunarDay, LunarMonth, MethodProfile, Mutagen,
    PalaceName, Scope, StarKind, StarName, StemBranch, TemporalContext, YearlyPeriod,
    build_yearly_horoscope_layer, build_yearly_period, by_lunar, flow_star_name,
};
use serde_json::Value;

const HOROSCOPE_FIXTURE: &str = include_str!("../fixtures/iztro/horoscope.json");
const CANONICAL_CASE_ID: &str = "canonical_female_default_2026";

#[test]
fn yearly_period_and_layer_match_all_fixture_cases() {
    for case in horoscope_fixture_cases() {
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let yearly = yearly_fixture(&case);
        let period = build_fixture_yearly_period(&case, yearly);
        let layer = build_yearly_horoscope_layer(&chart, &period)
            .expect("yearly horoscope layer should build");

        assert_yearly_period_matches_fixture(&period, yearly);
        assert_yearly_palace_layout_matches_fixture(
            layer
                .palace_layout()
                .expect("yearly layer should carry palace layout"),
            yearly,
        );
        assert_eq!(
            actual_yearly_mutagens(&layer),
            expected_yearly_mutagens(yearly, &chart)
        );
        assert_eq!(
            actual_yearly_flow_stars(&layer),
            expected_yearly_flow_stars(yearly)
        );
    }
}

#[test]
fn yearly_period_matches_canonical_fixture() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let yearly = yearly_fixture(&case);

    let period = build_fixture_yearly_period(&case, yearly);

    assert_eq!(period.lunar_year(), target_year(&case));
    assert_yearly_period_matches_fixture(&period, yearly);
}

#[test]
fn yearly_horoscope_layer_matches_canonical_fixture_context() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let yearly = yearly_fixture(&case);
    let period = build_fixture_yearly_period(&case, yearly);

    let layer =
        build_yearly_horoscope_layer(&chart, &period).expect("yearly horoscope layer should build");

    assert_eq!(layer.scope(), Scope::Yearly);
    assert_eq!(
        *layer.context(),
        TemporalContext::Yearly {
            stem_branch: yearly_stem_branch(yearly),
            lunar_year: target_year(&case),
        }
    );
}

#[test]
fn yearly_horoscope_layer_matches_canonical_fixture_palace_layout() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let yearly = yearly_fixture(&case);
    let period = build_fixture_yearly_period(&case, yearly);

    let layer =
        build_yearly_horoscope_layer(&chart, &period).expect("yearly horoscope layer should build");

    assert_yearly_palace_layout_matches_fixture(
        layer
            .palace_layout()
            .expect("yearly layer should carry palace layout"),
        yearly,
    );
}

#[test]
fn yearly_horoscope_layer_matches_canonical_fixture_mutagens() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let yearly = yearly_fixture(&case);
    let period = build_fixture_yearly_period(&case, yearly);

    let layer =
        build_yearly_horoscope_layer(&chart, &period).expect("yearly horoscope layer should build");

    assert_eq!(
        actual_yearly_mutagens(&layer),
        expected_yearly_mutagens(yearly, &chart)
    );
}

#[test]
fn yearly_horoscope_layer_matches_canonical_fixture_flow_stars() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let yearly = yearly_fixture(&case);
    let period = build_fixture_yearly_period(&case, yearly);

    let layer =
        build_yearly_horoscope_layer(&chart, &period).expect("yearly horoscope layer should build");

    assert_eq!(
        actual_yearly_flow_stars(&layer),
        expected_yearly_flow_stars(yearly)
    );
    assert!(
        layer
            .placements()
            .iter()
            .any(|placement| placement.placement().name() == StarName::NianJieYearly),
        "yearly layer should include yearly-only NianJieYearly"
    );
}

#[test]
fn yearly_snapshot_keeps_natal_and_yearly_facts_separate() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let yearly = yearly_fixture(&case);
    let period = build_fixture_yearly_period(&case, yearly);
    let layer =
        build_yearly_horoscope_layer(&chart, &period).expect("yearly horoscope layer should build");
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

    let yearly_layer = snapshot
        .layer(ChartLayerKind::Yearly)
        .expect("yearly snapshot layer should exist");
    assert_eq!(
        yearly_layer.context(),
        Some(&TemporalContext::Yearly {
            stem_branch: yearly_stem_branch(yearly),
            lunar_year: target_year(&case),
        })
    );
    assert!(
        yearly_layer
            .cells()
            .iter()
            .all(|cell| cell.typed_stars().is_empty())
    );
    assert!(
        yearly_layer
            .cells()
            .iter()
            .all(|cell| cell.decorative_stars().is_empty())
    );

    let expected_palaces = expected_yearly_palace_names_by_branch(yearly);
    for cell in yearly_layer.cells() {
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

    let yearly_scoped_count: usize = yearly_layer
        .cells()
        .iter()
        .map(|cell| cell.scoped_stars().len())
        .sum();
    assert_eq!(
        yearly_scoped_count,
        expected_yearly_flow_stars(yearly).len()
    );

    let yearly_mutagen_count: usize = yearly_layer
        .cells()
        .iter()
        .map(|cell| cell.mutagen_activations().len())
        .sum();
    assert_eq!(
        yearly_mutagen_count,
        expected_yearly_mutagens(yearly, &chart).len()
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
        "yearly horoscope fixtures should build through by_lunar"
    );
    let lunar_year = input["year"].as_i64().expect("fixture lunar year") as i32;
    let birth_year = StemBranch::from_lunar_year(lunar_year);
    let method_profile = MethodProfile::new(
        case["id"].as_str().expect("case id"),
        parse_algorithm(input["algorithm"].as_str().expect("algorithm")),
        "yearly horoscope fixture test",
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

    by_lunar(request).expect("by_lunar should build yearly horoscope fixture chart")
}

fn yearly_fixture(case: &Value) -> &Value {
    &case["supported_fields"]["yearly"]
}

fn build_fixture_yearly_period(case: &Value, yearly: &Value) -> YearlyPeriod {
    let period = build_yearly_period(target_year(case)).expect("yearly period should build");
    assert_eq!(
        period.stem_branch(),
        yearly_stem_branch(yearly),
        "fixture target year should derive fixture stem-branch"
    );
    period
}

fn assert_yearly_period_matches_fixture(period: &YearlyPeriod, yearly: &Value) {
    assert_eq!(period.index(), yearly_index(yearly));
    assert_eq!(period.stem_branch(), yearly_stem_branch(yearly));
    assert_eq!(period.palace_branch(), yearly_stem_branch(yearly).branch());
    assert_yearly_palace_layout_matches_fixture(period.palace_layout(), yearly);
}

fn assert_yearly_palace_layout_matches_fixture(
    layout: &iztro::core::TemporalPalaceLayout,
    yearly: &Value,
) {
    assert_eq!(layout.scope(), Scope::Yearly);
    assert_eq!(layout.names().len(), 12);

    let expected = expected_yearly_palace_names_by_branch(yearly);
    for name in layout.names() {
        assert_eq!(
            Some(name.palace_name()),
            expected.get(&name.branch()).copied(),
            "yearly palace name mismatch at {:?}",
            name.branch()
        );
    }
    assert_eq!(
        layout.name_for_branch(yearly_stem_branch(yearly).branch()),
        Some(PalaceName::Life)
    );
}

fn expected_yearly_palace_names_by_branch(yearly: &Value) -> HashMap<EarthlyBranch, PalaceName> {
    yearly["palace_names"]
        .as_array()
        .expect("yearly palace names")
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

fn expected_yearly_mutagens(
    yearly: &Value,
    chart: &Chart,
) -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    yearly["mutagen"]
        .as_object()
        .expect("yearly mutagen map")
        .iter()
        .filter_map(|(transform, entry)| {
            let star = parse_key::<StarName>(entry["star"].as_str().expect("mutagen star"));
            let branch = chart.star(star).map(|fact| fact.palace().branch())?;
            Some(((star, branch), parse_key::<Mutagen>(transform)))
        })
        .collect()
}

fn actual_yearly_mutagens(
    layer: &iztro::core::TemporalLayer,
) -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    layer
        .activations()
        .iter()
        .map(|activation| {
            assert_eq!(activation.source_scope(), Scope::Yearly);
            (
                (activation.target_star(), activation.target_branch()),
                activation.mutagen(),
            )
        })
        .collect()
}

fn expected_yearly_flow_stars(yearly: &Value) -> HashMap<StarName, (EarthlyBranch, StarKind)> {
    let mut expected: HashMap<StarName, (EarthlyBranch, StarKind)> = yearly["flow_stars"]
        .as_array()
        .expect("yearly flow stars")
        .iter()
        .map(|entry| {
            let base = parse_flow_base(entry["base"].as_str().expect("flow star base"));
            (
                flow_star_name(FlowStarScope::Yearly, base),
                (
                    parse_key::<EarthlyBranch>(entry["branch"].as_str().expect("branch")),
                    kind_from_type(entry["type"].as_str().expect("type")),
                ),
            )
        })
        .collect();
    expected.insert(
        StarName::NianJieYearly,
        (
            parse_key::<EarthlyBranch>(
                yearly["nian_jie_branch"]
                    .as_str()
                    .expect("yearly NianJie branch"),
            ),
            StarKind::Helper,
        ),
    );
    expected
}

fn actual_yearly_flow_stars(
    layer: &iztro::core::TemporalLayer,
) -> HashMap<StarName, (EarthlyBranch, StarKind)> {
    layer
        .placements()
        .iter()
        .map(|placement| {
            assert_eq!(placement.scope(), Scope::Yearly);
            (
                placement.placement().name(),
                (placement.branch(), placement.placement().kind()),
            )
        })
        .collect()
}

fn yearly_stem_branch(yearly: &Value) -> StemBranch {
    StemBranch::try_new(
        parse_key::<HeavenlyStem>(yearly["heavenly_stem"].as_str().expect("yearly stem")),
        parse_key::<EarthlyBranch>(yearly["earthly_branch"].as_str().expect("yearly branch")),
    )
    .expect("fixture yearly stem-branch should be valid")
}

fn target_year(case: &Value) -> i32 {
    case["input"]["target"]["year"]
        .as_i64()
        .expect("fixture target year") as i32
}

fn yearly_index(yearly: &Value) -> usize {
    yearly["index"].as_u64().expect("yearly index") as usize
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
