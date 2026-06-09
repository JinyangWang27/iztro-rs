use std::collections::{HashMap, HashSet};

use iztro_core::{
    Brightness, Chart, ChartAlgorithmKind, DecorativeStarFamily, EarthlyBranch, FiveElementBureau,
    HeavenlyStem, LunarChartRequest, LunarDay, LunarMonth, MethodProfile, Mutagen, PalaceName,
    StarKind, StarName, by_lunar, known_star_metadata_table, represented_star_metadata_table,
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

        assert_eq!(known_star_metadata_table().len(), 170);
        assert_eq!(represented_star_metadata_table().len(), 70);
        assert_palaces_match(&chart, fixture_case);
        assert_typed_stars_match(&chart, fixture_case, algorithm);
        assert_decorative_stars_match(&chart, fixture_case);
        assert_suiqian_algorithm_boundary(&chart, algorithm);
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

    let request = LunarChartRequest::builder()
        .lunar_year(input["lunar_year"].as_i64().expect("lunar_year") as i32)
        .lunar_month(
            LunarMonth::new(input["lunar_month"].as_u64().expect("lunar_month") as u8)
                .expect("fixture lunar month should be valid"),
        )
        .lunar_day(
            LunarDay::new(input["lunar_day"].as_u64().expect("lunar_day") as u8)
                .expect("fixture lunar day should be valid"),
        )
        .birth_time(parse_key(input["birth_time"].as_str().expect("birth_time")))
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

fn assert_palaces_match(chart: &Chart, fixture_case: &Value) {
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
        )
    );
    assert_eq!(
        chart.body_palace_branch(),
        Some(parse_key(
            supported["body_palace_branch"]
                .as_str()
                .expect("body palace")
        ))
    );
    assert_eq!(
        chart.five_element_bureau(),
        Some(parse_key::<FiveElementBureau>(
            supported["five_element_bureau"]
                .as_str()
                .expect("five element bureau")
        ))
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
            "palace name mismatch at {branch:?}"
        );
        assert_eq!(
            palace.stem(),
            parse_key::<HeavenlyStem>(expected["stem"].as_str().expect("palace stem")),
            "palace stem mismatch at {branch:?}"
        );
    }
}

fn assert_typed_stars_match(chart: &Chart, fixture_case: &Value, algorithm: ChartAlgorithmKind) {
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
        Some(expected_total as u64)
    );
    assert_eq!(chart.stars().len(), expected_total);

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
            "typed-star mismatch in {branch:?}"
        );

        for expected_star in expected_palace["stars"].as_array().expect("stars array") {
            let name = parse_key::<StarName>(expected_star["name"].as_str().expect("star name"));
            let got = actual
                .get(&(branch, name))
                .unwrap_or_else(|| panic!("missing {name:?} in {branch:?}"));

            assert_eq!(
                got.kind,
                parse_key::<StarKind>(expected_star["kind"].as_str().expect("kind")),
                "kind mismatch for {name:?} in {branch:?}"
            );
            assert_eq!(
                got.brightness,
                parse_key::<Brightness>(expected_star["brightness"].as_str().expect("brightness")),
                "brightness mismatch for {name:?} in {branch:?}"
            );
            assert_eq!(
                got.mutagen,
                parse_optional_mutagen(&expected_star["mutagen"]),
                "mutagen mismatch for {name:?} in {branch:?}"
            );
        }
    }
}

fn assert_decorative_stars_match(chart: &Chart, fixture_case: &Value) {
    let actual = collect_decorative_stars(chart);

    assert_eq!(
        fixture_case["supported_fields"]["decorative_runtime_star_count"].as_u64(),
        Some(48)
    );
    assert_eq!(chart.decorative_stars().len(), 48);

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
                "{family:?} mismatch in {branch:?}"
            );
        }
    }
}

fn assert_suiqian_algorithm_boundary(chart: &Chart, algorithm: ChartAlgorithmKind) {
    match algorithm {
        ChartAlgorithmKind::QuanShu => {
            assert!(chart.decorative_star(StarName::SuiPo).is_none());
            assert!(chart.decorative_star(StarName::DaHaoSuiqian).is_some());
        }
        ChartAlgorithmKind::Zhongzhou => {
            assert!(chart.decorative_star(StarName::SuiPo).is_some());
            assert!(chart.decorative_star(StarName::DaHaoSuiqian).is_none());
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

fn collect_decorative_stars(
    chart: &Chart,
) -> HashMap<(EarthlyBranch, DecorativeStarFamily), StarName> {
    let mut out = HashMap::new();
    for fact in chart.decorative_stars() {
        out.insert((fact.branch(), fact.placement().family()), fact.name());
    }
    out
}

fn parse_algorithm(value: &str) -> ChartAlgorithmKind {
    match value {
        "default" => ChartAlgorithmKind::QuanShu,
        "zhongzhou" => ChartAlgorithmKind::Zhongzhou,
        other => panic!("unsupported fixture algorithm: {other}"),
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
