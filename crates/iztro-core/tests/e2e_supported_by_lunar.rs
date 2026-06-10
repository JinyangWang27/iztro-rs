use std::collections::{HashMap, HashSet};

use iztro_core::{
    Brightness, Chart, ChartAlgorithmKind, DecorativeStarFamily, EarthlyBranch, FiveElementBureau,
    FlowStarBase, FlowStarScope, HeavenlyStem, LunarChartRequest, LunarDay, LunarMonth,
    MethodProfile, Mutagen, PalaceName, Scope, StarKind, StarName, StemBranch, TemporalContext,
    build_flow_star_layer, by_lunar, flow_star_name, known_star_metadata_table,
    represented_star_metadata_table, try_flow_star_parts,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

const FIXTURE: &str = include_str!("../../../fixtures/iztro/e2e_supported_by_lunar.json");

const DECORATIVE_FAMILIES: [(&str, DecorativeStarFamily); 4] = [
    ("changsheng12", DecorativeStarFamily::Changsheng12),
    ("boshi12", DecorativeStarFamily::Boshi12),
    ("suiqian12", DecorativeStarFamily::Suiqian12),
    ("jiangqian12", DecorativeStarFamily::Jiangqian12),
];

#[test]
fn by_lunar_matches_supported_e2e_fixture_cases() {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("fixture should be valid JSON");

    assert_eq!(
        fixture["metadata"]["target_package"].as_str(),
        Some("npm:iztro")
    );
    assert_eq!(
        fixture["metadata"]["target_version"].as_str(),
        Some("2.5.8")
    );
    assert_eq!(
        fixture["metadata"]["supported_fields_only"].as_bool(),
        Some(true)
    );

    let cases = fixture["cases"]
        .as_array()
        .expect("fixture should list e2e cases");
    assert_eq!(cases.len(), 12);

    for fixture_case in cases {
        let algorithm = parse_algorithm(fixture_case["algorithm"].as_str().expect("algorithm"));
        let chart = chart_from_case(fixture_case, algorithm);
        let case_label = case_label(fixture_case);

        assert_eq!(known_star_metadata_table().len(), 170);
        assert_eq!(represented_star_metadata_table().len(), 70);
        assert_palaces_match(&chart, fixture_case, &case_label);
        assert_typed_stars_match(&chart, fixture_case, algorithm, &case_label);
        assert_decorative_stars_match(&chart, fixture_case, &case_label);
        assert_temporal_flow_stars_match(fixture_case, &case_label);
        assert_suiqian_algorithm_boundary(&chart, algorithm, &case_label);
    }
}

fn chart_from_case(fixture_case: &Value, algorithm: ChartAlgorithmKind) -> Chart {
    let input = &fixture_case["input"];
    let method_profile = MethodProfile::new(
        format!(
            "iztro_2_5_8_e2e_supported_{}",
            fixture_case["case"].as_str().expect("case id")
        ),
        algorithm,
        "iztro 2.5.8 supported by_lunar e2e fixture",
    );

    let builder = LunarChartRequest::builder()
        .lunar_year(input["lunar_year"].as_i64().expect("lunar_year") as i32)
        .lunar_month(
            LunarMonth::new(input["lunar_month"].as_u64().expect("lunar_month") as u8)
                .expect("fixture lunar month should be valid"),
        )
        .lunar_day(
            LunarDay::new(input["lunar_day"].as_u64().expect("lunar_day") as u8)
                .expect("fixture lunar day should be valid"),
        );
    let builder = if let Some(index) = input["iztro_time_index"].as_u64() {
        builder
            .iztro_time_index(index as u8)
            .expect("fixture iztro time index should be valid")
    } else {
        builder.birth_time(parse_key(input["birth_time"].as_str().expect("birth_time")))
    };
    let request = builder
        .gender(parse_key(input["gender"].as_str().expect("gender")))
        .birth_year_stem(parse_key(
            input["birth_year_stem"].as_str().expect("birth_year_stem"),
        ))
        .birth_year_branch(parse_key(
            input["birth_year_branch"]
                .as_str()
                .expect("birth_year_branch"),
        ))
        .method_profile(method_profile)
        .build()
        .expect("fixture request should build");

    by_lunar(request).expect("by_lunar should build supported fixture chart")
}

fn assert_palaces_match(chart: &Chart, fixture_case: &Value, case_label: &str) {
    let supported = &fixture_case["supported_fields"];

    assert_eq!(
        chart
            .life_palace()
            .expect("life palace should be present")
            .branch(),
        parse_key(
            supported["life_palace_branch"]
                .as_str()
                .expect("life palace")
        ),
        "{case_label}: life palace branch mismatch"
    );
    assert_eq!(
        chart.body_palace_branch(),
        Some(parse_key(
            supported["body_palace_branch"]
                .as_str()
                .expect("body palace")
        )),
        "{case_label}: body palace branch mismatch"
    );
    assert_eq!(
        chart.five_element_bureau(),
        Some(parse_key::<FiveElementBureau>(
            supported["five_element_bureau"]
                .as_str()
                .expect("five element bureau")
        )),
        "{case_label}: five-element bureau mismatch"
    );

    for expected in supported["palaces"].as_array().expect("supported palaces") {
        let branch = parse_key::<EarthlyBranch>(expected["branch"].as_str().expect("branch"));
        let palace = chart
            .palaces()
            .iter()
            .find(|palace| palace.branch() == branch)
            .unwrap_or_else(|| panic!("missing palace at {branch:?}"));

        assert_eq!(
            palace.name(),
            parse_key::<PalaceName>(expected["name"].as_str().expect("palace name")),
            "{case_label}: palace name mismatch at {branch:?}"
        );
        assert_eq!(
            palace.stem(),
            parse_key::<HeavenlyStem>(expected["stem"].as_str().expect("palace stem")),
            "{case_label}: palace stem mismatch at {branch:?}"
        );
    }
}

fn assert_typed_stars_match(
    chart: &Chart,
    fixture_case: &Value,
    algorithm: ChartAlgorithmKind,
    case_label: &str,
) {
    let actual = collect_typed_stars(chart);
    let expected_palaces = fixture_case["supported_fields"]["typed_natal_stars"]
        .as_array()
        .expect("typed natal stars");
    let expected_total = match algorithm {
        ChartAlgorithmKind::QuanShu => 66,
        ChartAlgorithmKind::Zhongzhou => 68,
        ChartAlgorithmKind::Placeholder => panic!("unsupported e2e fixture algorithm"),
    };

    assert_eq!(
        fixture_case["supported_fields"]["typed_natal_star_count"].as_u64(),
        Some(expected_total as u64),
        "{case_label}: fixture typed natal star count mismatch"
    );
    assert_eq!(
        chart.stars().len(),
        expected_total,
        "{case_label}: typed natal star count mismatch"
    );

    for expected_palace in expected_palaces {
        let branch =
            parse_key::<EarthlyBranch>(expected_palace["branch"].as_str().expect("branch"));
        let expected_names: HashSet<StarName> = expected_palace["stars"]
            .as_array()
            .expect("stars array")
            .iter()
            .map(|star| parse_key(star["name"].as_str().expect("star name")))
            .collect();
        let actual_names: HashSet<StarName> = actual
            .keys()
            .filter_map(|&(actual_branch, star)| (actual_branch == branch).then_some(star))
            .collect();

        assert_eq!(
            actual_names, expected_names,
            "{case_label}: typed-star mismatch in {branch:?}"
        );

        for expected_star in expected_palace["stars"].as_array().expect("stars array") {
            let name = parse_key::<StarName>(expected_star["name"].as_str().expect("star name"));
            let got = actual
                .get(&(branch, name))
                .unwrap_or_else(|| panic!("missing {name:?} in {branch:?}"));

            assert_eq!(
                got.kind,
                parse_key::<StarKind>(expected_star["kind"].as_str().expect("kind")),
                "{case_label}: kind mismatch for {name:?} in {branch:?}"
            );
            assert_eq!(
                got.brightness,
                parse_key::<Brightness>(expected_star["brightness"].as_str().expect("brightness")),
                "{case_label}: brightness mismatch for {name:?} in {branch:?}"
            );
            assert_eq!(
                got.mutagen,
                parse_optional_mutagen(&expected_star["mutagen"]),
                "{case_label}: mutagen mismatch for {name:?} in {branch:?}"
            );
        }
    }
}

fn assert_decorative_stars_match(chart: &Chart, fixture_case: &Value, case_label: &str) {
    let actual = collect_decorative_stars(chart);

    assert_eq!(
        fixture_case["supported_fields"]["decorative_runtime_star_count"].as_u64(),
        Some(48),
        "{case_label}: fixture decorative count mismatch"
    );
    assert_eq!(
        chart.decorative_stars().len(),
        48,
        "{case_label}: decorative count mismatch"
    );

    for expected_palace in fixture_case["supported_fields"]["decorative_stars"]
        .as_array()
        .expect("decorative stars")
    {
        let branch =
            parse_key::<EarthlyBranch>(expected_palace["branch"].as_str().expect("branch"));
        for (field, family) in DECORATIVE_FAMILIES {
            let expected = parse_key::<StarName>(expected_palace[field].as_str().expect(field));
            assert_eq!(
                actual.get(&(branch, family)).copied(),
                Some(expected),
                "{case_label}: {family:?} mismatch in {branch:?}"
            );
        }
    }
}

fn assert_temporal_flow_stars_match(fixture_case: &Value, case_label: &str) {
    let temporal_cases = fixture_case["supported_fields"]["temporal_flow_stars"]
        .as_array()
        .expect("temporal flow stars");
    assert_eq!(
        temporal_cases.len(),
        5,
        "{case_label}: expected one temporal flow entry per supported scope"
    );

    for temporal_case in temporal_cases {
        let flow_scope = parse_flow_scope(temporal_case["scope"].as_str().expect("scope"));
        let scope = scope_for_flow(flow_scope);
        let stem = parse_key::<HeavenlyStem>(temporal_case["stem"].as_str().expect("stem"));
        let branch = parse_key::<EarthlyBranch>(temporal_case["branch"].as_str().expect("branch"));
        let context = temporal_context_for(flow_scope, StemBranch::new(stem, branch));
        let layer = build_flow_star_layer(context).expect("flow layer should build");
        let actual = collect_temporal_placements(&layer);
        let expected_placements = temporal_case["placements"]
            .as_array()
            .expect("temporal placements");
        let expected_count = if flow_scope == FlowStarScope::Yearly {
            11
        } else {
            10
        };

        assert_eq!(
            layer.scope(),
            scope,
            "{case_label}: temporal layer scope mismatch for {flow_scope:?}"
        );
        assert_eq!(
            expected_placements.len(),
            expected_count,
            "{case_label}: fixture temporal placement count mismatch for {flow_scope:?}"
        );
        assert_eq!(
            actual.len(),
            expected_count,
            "{case_label}: actual temporal placement count mismatch for {flow_scope:?}"
        );

        for expected in expected_placements {
            let name = parse_key::<StarName>(expected["name"].as_str().expect("name"));
            let branch = parse_key::<EarthlyBranch>(expected["branch"].as_str().expect("branch"));
            let expected_scope = parse_key::<Scope>(expected["scope"].as_str().expect("scope"));
            let kind = parse_key::<StarKind>(expected["kind"].as_str().expect("kind"));
            let got = actual
                .get(&name)
                .unwrap_or_else(|| panic!("{case_label}: missing temporal {name:?}"));

            assert_eq!(
                got.branch, branch,
                "{case_label}: branch mismatch for temporal {name:?}"
            );
            assert_eq!(
                got.scope, expected_scope,
                "{case_label}: scope mismatch for temporal {name:?}"
            );
            assert_eq!(
                got.kind, kind,
                "{case_label}: kind mismatch for temporal {name:?}"
            );

            if name == StarName::NianJieYearly {
                assert_eq!(
                    expected["base"],
                    Value::Null,
                    "{case_label}: NianJieYearly must not have FlowStarBase"
                );
                assert_eq!(
                    flow_scope,
                    FlowStarScope::Yearly,
                    "{case_label}: NianJieYearly must be yearly-only"
                );
                assert_eq!(try_flow_star_parts(name), None);
            } else {
                let base = parse_flow_base(expected["base"].as_str().expect("base"));
                assert_eq!(
                    flow_star_name(flow_scope, base),
                    name,
                    "{case_label}: flow star name/base mismatch for {name:?}"
                );
                assert_eq!(try_flow_star_parts(name), Some((flow_scope, base)));
            }
        }

        if flow_scope != FlowStarScope::Yearly {
            assert!(
                !actual.contains_key(&StarName::NianJieYearly),
                "{case_label}: NianJieYearly must be yearly-only"
            );
        }
    }
}

fn assert_suiqian_algorithm_boundary(
    chart: &Chart,
    algorithm: ChartAlgorithmKind,
    case_label: &str,
) {
    match algorithm {
        ChartAlgorithmKind::QuanShu => {
            assert!(
                chart.decorative_star(StarName::SuiPo).is_none(),
                "{case_label}: default must not place SuiPo"
            );
            assert!(
                chart.decorative_star(StarName::DaHaoSuiqian).is_some(),
                "{case_label}: default must place DaHaoSuiqian"
            );
        }
        ChartAlgorithmKind::Zhongzhou => {
            assert!(
                chart.decorative_star(StarName::SuiPo).is_some(),
                "{case_label}: Zhongzhou must place SuiPo"
            );
            assert!(
                chart.decorative_star(StarName::DaHaoSuiqian).is_none(),
                "{case_label}: Zhongzhou must not place DaHaoSuiqian"
            );
        }
        ChartAlgorithmKind::Placeholder => panic!("unsupported e2e fixture algorithm"),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TypedStarFact {
    kind: StarKind,
    brightness: Brightness,
    mutagen: Option<Mutagen>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TemporalStarFact {
    branch: EarthlyBranch,
    scope: Scope,
    kind: StarKind,
}

fn collect_typed_stars(chart: &Chart) -> HashMap<(EarthlyBranch, StarName), TypedStarFact> {
    let mut out = HashMap::new();
    for fact in chart.stars() {
        out.insert(
            (fact.palace().branch(), fact.placement().name()),
            TypedStarFact {
                kind: fact.placement().kind(),
                brightness: fact.placement().brightness(),
                mutagen: fact.placement().mutagen(),
            },
        );
    }
    out
}

fn collect_temporal_placements(
    layer: &iztro_core::TemporalLayer,
) -> HashMap<StarName, TemporalStarFact> {
    layer
        .placements()
        .iter()
        .map(|placement| {
            (
                placement.placement().name(),
                TemporalStarFact {
                    branch: placement.branch(),
                    scope: placement.scope(),
                    kind: placement.placement().kind(),
                },
            )
        })
        .collect()
}

fn collect_decorative_stars(
    chart: &Chart,
) -> HashMap<(EarthlyBranch, DecorativeStarFamily), StarName> {
    let mut out = HashMap::new();
    for fact in chart.decorative_stars() {
        out.insert((fact.branch(), fact.placement().family()), fact.name());
    }
    out
}

fn case_label(fixture_case: &Value) -> String {
    format!(
        "{} [{}]",
        fixture_case["case"].as_str().expect("case id"),
        fixture_case["algorithm"].as_str().expect("algorithm")
    )
}

fn parse_algorithm(value: &str) -> ChartAlgorithmKind {
    match value {
        "default" => ChartAlgorithmKind::QuanShu,
        "zhongzhou" => ChartAlgorithmKind::Zhongzhou,
        other => panic!("unsupported fixture algorithm: {other}"),
    }
}

fn temporal_context_for(scope: FlowStarScope, stem_branch: StemBranch) -> TemporalContext {
    match scope {
        FlowStarScope::Decadal => TemporalContext::Decadal {
            stem_branch,
            start_age: 6,
        },
        FlowStarScope::Yearly => TemporalContext::Yearly {
            stem_branch,
            lunar_year: 2020,
        },
        FlowStarScope::Monthly => TemporalContext::Monthly {
            stem_branch,
            lunar_month: 1,
        },
        FlowStarScope::Daily => TemporalContext::Daily {
            stem_branch,
            lunar_day: 1,
        },
        FlowStarScope::Hourly => TemporalContext::Hourly { stem_branch },
    }
}

fn scope_for_flow(scope: FlowStarScope) -> Scope {
    match scope {
        FlowStarScope::Decadal => Scope::Decadal,
        FlowStarScope::Yearly => Scope::Yearly,
        FlowStarScope::Monthly => Scope::Monthly,
        FlowStarScope::Daily => Scope::Daily,
        FlowStarScope::Hourly => Scope::Hourly,
    }
}

fn parse_flow_scope(value: &str) -> FlowStarScope {
    match value {
        "decadal" => FlowStarScope::Decadal,
        "yearly" => FlowStarScope::Yearly,
        "monthly" => FlowStarScope::Monthly,
        "daily" => FlowStarScope::Daily,
        "hourly" => FlowStarScope::Hourly,
        other => panic!("unsupported flow scope: {other}"),
    }
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

fn parse_key<T>(key: &str) -> T
where
    T: DeserializeOwned,
{
    serde_json::from_str(&format!("\"{key}\"")).expect("fixture key should parse")
}

fn parse_optional_mutagen(value: &Value) -> Option<Mutagen> {
    value.as_str().map(parse_key)
}
