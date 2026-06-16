mod common;

use std::collections::HashMap;

use common::{parse_algorithm, parse_key};
use iztro::core::{
    BirthTime, Chart, ChartLayerKind, ChartStackSnapshot, EarthlyBranch, FlowStarBase,
    FlowStarScope, Gender, HeavenlyStem, HoroscopeChart, LunarChartRequest, LunarDay, LunarMonth,
    MethodProfile, MonthlyPeriod, Mutagen, PalaceName, Scope, SolarDay, SolarMonth, StarKind,
    StarName, StemBranch, TemporalContext, build_monthly_horoscope_layer, build_monthly_period,
    by_lunar, flow_star_name,
};
use lunar_lite::{SolarDate, solar_to_lunar};
use serde_json::Value;

const HOROSCOPE_FIXTURE: &str = include_str!("../fixtures/iztro/horoscope.json");
const CANONICAL_CASE_ID: &str = "canonical_female_default_2026";

#[test]
fn monthly_period_and_layer_match_all_fixture_cases() {
    for case in horoscope_fixture_cases() {
        let chart = build_chart_from_horoscope_fixture_case(&case);
        let monthly = monthly_fixture(&case);
        let period = build_fixture_monthly_period(&chart, &case, monthly);
        let layer = build_monthly_horoscope_layer(&chart, &period)
            .expect("monthly horoscope layer should build");

        assert_monthly_period_matches_fixture(&period, monthly, &case);
        assert_monthly_palace_layout_matches_fixture(
            layer
                .palace_layout()
                .expect("monthly layer should carry palace layout"),
            monthly,
        );
        assert_eq!(
            actual_monthly_mutagens(&layer),
            expected_monthly_mutagens(monthly, &chart)
        );
        assert_eq!(
            actual_monthly_flow_stars(&layer),
            expected_monthly_flow_stars(monthly)
        );
    }
}

#[test]
fn monthly_horoscope_layer_matches_canonical_fixture_context() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let monthly = monthly_fixture(&case);
    let period = build_fixture_monthly_period(&chart, &case, monthly);

    assert_ne!(
        period.palace_branch(),
        period.stem_branch().branch(),
        "canonical fixture proves monthly stem-branch branch is independent from monthly Life branch"
    );

    let layer = build_monthly_horoscope_layer(&chart, &period)
        .expect("monthly horoscope layer should build");

    assert_eq!(layer.scope(), Scope::Monthly);
    assert_eq!(
        *layer.context(),
        TemporalContext::Monthly {
            stem_branch: monthly_stem_branch(monthly),
            lunar_month: target_lunar_month(&case),
        }
    );
}

#[test]
fn monthly_horoscope_layer_matches_canonical_fixture_palace_layout() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let monthly = monthly_fixture(&case);
    let period = build_fixture_monthly_period(&chart, &case, monthly);

    let layer = build_monthly_horoscope_layer(&chart, &period)
        .expect("monthly horoscope layer should build");

    assert_monthly_palace_layout_matches_fixture(
        layer
            .palace_layout()
            .expect("monthly layer should carry palace layout"),
        monthly,
    );
}

#[test]
fn monthly_horoscope_layer_matches_canonical_fixture_mutagens() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let monthly = monthly_fixture(&case);
    let period = build_fixture_monthly_period(&chart, &case, monthly);

    let layer = build_monthly_horoscope_layer(&chart, &period)
        .expect("monthly horoscope layer should build");

    assert_eq!(
        actual_monthly_mutagens(&layer),
        expected_monthly_mutagens(monthly, &chart)
    );
}

#[test]
fn monthly_horoscope_layer_matches_canonical_fixture_flow_stars() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let monthly = monthly_fixture(&case);
    let period = build_fixture_monthly_period(&chart, &case, monthly);

    let layer = build_monthly_horoscope_layer(&chart, &period)
        .expect("monthly horoscope layer should build");

    assert_eq!(
        actual_monthly_flow_stars(&layer),
        expected_monthly_flow_stars(monthly)
    );
    assert!(
        layer
            .placements()
            .iter()
            .all(|placement| placement.placement().name() != StarName::NianJieYearly),
        "monthly layer should not include yearly-only NianJieYearly"
    );
}

#[test]
fn monthly_snapshot_keeps_natal_and_monthly_facts_separate() {
    let case = horoscope_fixture_case(CANONICAL_CASE_ID);
    let chart = build_chart_from_horoscope_fixture_case(&case);
    let monthly = monthly_fixture(&case);
    let period = build_fixture_monthly_period(&chart, &case, monthly);
    let layer = build_monthly_horoscope_layer(&chart, &period)
        .expect("monthly horoscope layer should build");
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

    let monthly_layer = snapshot
        .layer(ChartLayerKind::Monthly)
        .expect("monthly snapshot layer should exist");
    assert_eq!(
        monthly_layer.context(),
        Some(&TemporalContext::Monthly {
            stem_branch: monthly_stem_branch(monthly),
            lunar_month: target_lunar_month(&case),
        })
    );
    assert!(
        monthly_layer
            .cells()
            .iter()
            .all(|cell| cell.typed_stars().is_empty())
    );
    assert!(
        monthly_layer
            .cells()
            .iter()
            .all(|cell| cell.decorative_stars().is_empty())
    );

    let expected_palaces = expected_monthly_palace_names_by_branch(monthly);
    for cell in monthly_layer.cells() {
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

    let monthly_scoped_count: usize = monthly_layer
        .cells()
        .iter()
        .map(|cell| cell.scoped_stars().len())
        .sum();
    assert_eq!(
        monthly_scoped_count,
        expected_monthly_flow_stars(monthly).len()
    );

    let monthly_mutagen_count: usize = monthly_layer
        .cells()
        .iter()
        .map(|cell| cell.mutagen_activations().len())
        .sum();
    assert_eq!(
        monthly_mutagen_count,
        expected_monthly_mutagens(monthly, &chart).len()
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
        "monthly horoscope fixtures should build through by_lunar"
    );
    let lunar_year = input["year"].as_i64().expect("fixture lunar year") as i32;
    let birth_year = StemBranch::from_lunar_year(lunar_year);
    let method_profile = MethodProfile::new(
        case["id"].as_str().expect("case id"),
        parse_algorithm(input["algorithm"].as_str().expect("algorithm")),
        "monthly horoscope fixture test",
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

    by_lunar(request).expect("by_lunar should build monthly horoscope fixture chart")
}

fn monthly_fixture(case: &Value) -> &Value {
    &case["supported_fields"]["monthly"]
}

fn build_fixture_monthly_period(chart: &Chart, case: &Value, monthly: &Value) -> MonthlyPeriod {
    let (target_year, target_month, target_day) = target_solar_date(case);
    let period = build_monthly_period(
        chart,
        target_year,
        SolarMonth::new(target_month).expect("target solar month should be valid"),
        SolarDay::new(target_day).expect("target solar day should be valid"),
        target_time(case),
    )
    .expect("monthly period should build");
    assert_eq!(
        period.stem_branch(),
        monthly_stem_branch(monthly),
        "fixture target date should derive fixture monthly stem-branch"
    );
    period
}

fn assert_monthly_period_matches_fixture(period: &MonthlyPeriod, monthly: &Value, case: &Value) {
    assert_eq!(period.index(), monthly_index(monthly));
    assert_eq!(period.lunar_month(), target_lunar_month(case));
    assert_eq!(period.stem_branch(), monthly_stem_branch(monthly));
    assert_eq!(
        period.palace_branch(),
        EarthlyBranch::Yin.offset(monthly_index(monthly) as isize)
    );
    assert_monthly_palace_layout_matches_fixture(period.palace_layout(), monthly);
}

fn assert_monthly_palace_layout_matches_fixture(
    layout: &iztro::core::TemporalPalaceLayout,
    monthly: &Value,
) {
    assert_eq!(layout.scope(), Scope::Monthly);
    assert_eq!(layout.names().len(), 12);

    let expected = expected_monthly_palace_names_by_branch(monthly);
    for name in layout.names() {
        assert_eq!(
            Some(name.palace_name()),
            expected.get(&name.branch()).copied(),
            "monthly palace name mismatch at {:?}",
            name.branch()
        );
    }
    assert_eq!(
        layout.name_for_branch(EarthlyBranch::Yin.offset(monthly_index(monthly) as isize)),
        Some(PalaceName::Life)
    );
}

fn expected_monthly_palace_names_by_branch(monthly: &Value) -> HashMap<EarthlyBranch, PalaceName> {
    monthly["palace_names"]
        .as_array()
        .expect("monthly palace names")
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

fn expected_monthly_mutagens(
    monthly: &Value,
    chart: &Chart,
) -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    monthly["mutagen"]
        .as_object()
        .expect("monthly mutagen map")
        .iter()
        .filter_map(|(transform, entry)| {
            let star = parse_key::<StarName>(entry["star"].as_str().expect("mutagen star"));
            let branch = chart.star(star).map(|fact| fact.palace().branch())?;
            Some(((star, branch), parse_key::<Mutagen>(transform)))
        })
        .collect()
}

fn actual_monthly_mutagens(
    layer: &iztro::core::TemporalLayer,
) -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    layer
        .activations()
        .iter()
        .map(|activation| {
            assert_eq!(activation.source_scope(), Scope::Monthly);
            (
                (activation.target_star(), activation.target_branch()),
                activation.mutagen(),
            )
        })
        .collect()
}

fn expected_monthly_flow_stars(monthly: &Value) -> HashMap<StarName, (EarthlyBranch, StarKind)> {
    monthly["flow_stars"]
        .as_array()
        .expect("monthly flow stars")
        .iter()
        .map(|entry| {
            let base = parse_flow_base(entry["base"].as_str().expect("flow star base"));
            (
                flow_star_name(FlowStarScope::Monthly, base),
                (
                    parse_key::<EarthlyBranch>(entry["branch"].as_str().expect("branch")),
                    kind_from_type(entry["type"].as_str().expect("type")),
                ),
            )
        })
        .collect()
}

fn actual_monthly_flow_stars(
    layer: &iztro::core::TemporalLayer,
) -> HashMap<StarName, (EarthlyBranch, StarKind)> {
    layer
        .placements()
        .iter()
        .map(|placement| {
            assert_eq!(placement.scope(), Scope::Monthly);
            (
                placement.placement().name(),
                (placement.branch(), placement.placement().kind()),
            )
        })
        .collect()
}

fn monthly_stem_branch(monthly: &Value) -> StemBranch {
    StemBranch::try_new(
        parse_key::<HeavenlyStem>(monthly["heavenly_stem"].as_str().expect("monthly stem")),
        parse_key::<EarthlyBranch>(monthly["earthly_branch"].as_str().expect("monthly branch")),
    )
    .expect("fixture monthly stem-branch should be valid")
}

fn monthly_index(monthly: &Value) -> usize {
    monthly["index"].as_u64().expect("monthly index") as usize
}

fn target_lunar_month(case: &Value) -> u8 {
    let (year, month, day) = target_solar_date(case);
    solar_to_lunar(SolarDate { year, month, day })
        .expect("fixture target solar date should convert")
        .month
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
