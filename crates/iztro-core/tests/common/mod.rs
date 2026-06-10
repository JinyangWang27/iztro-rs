//! Shared helpers for the supported-field E2E tests (by_solar and leap-month
//! by_lunar). These assert the normalized supported fields of a built `Chart`
//! against a fixture's `supported_fields` block. Kept local to test code.

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use iztro_core::{
    Brightness, Chart, ChartAlgorithmKind, DecorativeStarFamily, EarthlyBranch, FiveElementBureau,
    HeavenlyStem, Mutagen, PalaceName, StarKind, StarName, known_star_metadata_table,
    represented_star_metadata_table,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub const DECORATIVE_FAMILIES: [(&str, DecorativeStarFamily); 4] = [
    ("changsheng12", DecorativeStarFamily::Changsheng12),
    ("boshi12", DecorativeStarFamily::Boshi12),
    ("suiqian12", DecorativeStarFamily::Suiqian12),
    ("jiangqian12", DecorativeStarFamily::Jiangqian12),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypedStarFact {
    pub kind: StarKind,
    pub brightness: Brightness,
    pub mutagen: Option<Mutagen>,
}

/// Parses a normalized snake_case fixture key into a serde enum value.
pub fn parse_key<T>(key: &str) -> T
where
    T: DeserializeOwned,
{
    serde_json::from_str(&format!("\"{key}\"")).expect("fixture key should parse")
}

/// Maps the fixture `algorithm` label to a `ChartAlgorithmKind`.
pub fn parse_algorithm(value: &str) -> ChartAlgorithmKind {
    match value {
        "default" => ChartAlgorithmKind::QuanShu,
        "zhongzhou" => ChartAlgorithmKind::Zhongzhou,
        other => panic!("unsupported fixture algorithm: {other}"),
    }
}

pub fn parse_optional_mutagen(value: &Value) -> Option<Mutagen> {
    value.as_str().map(parse_key)
}

/// Asserts the star-metadata inventory boundaries hold for every case.
pub fn assert_metadata_counts() {
    assert_eq!(known_star_metadata_table().len(), 170);
    assert_eq!(represented_star_metadata_table().len(), 70);
}

pub fn collect_typed_stars(chart: &Chart) -> HashMap<(EarthlyBranch, StarName), TypedStarFact> {
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

pub fn collect_decorative_stars(
    chart: &Chart,
) -> HashMap<(EarthlyBranch, DecorativeStarFamily), StarName> {
    let mut out = HashMap::new();
    for fact in chart.decorative_stars() {
        out.insert((fact.branch(), fact.placement().family()), fact.name());
    }
    out
}

/// Asserts life/body palace branches, five-element bureau, and per-palace
/// branch/stem/name facts against a `supported_fields` object.
pub fn assert_palaces_match(chart: &Chart, supported: &Value, case_label: &str) {
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

/// Asserts typed natal stars (name/kind/brightness/mutagen) and the typed-star
/// count against a `supported_fields` object.
pub fn assert_typed_stars_match(
    chart: &Chart,
    supported: &Value,
    algorithm: ChartAlgorithmKind,
    case_label: &str,
) {
    let actual = collect_typed_stars(chart);
    let expected_palaces = supported["typed_natal_stars"]
        .as_array()
        .expect("typed natal stars");
    let expected_total = match algorithm {
        ChartAlgorithmKind::QuanShu => 66,
        ChartAlgorithmKind::Zhongzhou => 68,
        ChartAlgorithmKind::Placeholder => panic!("unsupported e2e fixture algorithm"),
    };

    assert_eq!(
        supported["typed_natal_star_count"].as_u64(),
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

/// Asserts the four decorative runtime families per palace and the decorative
/// count against a `supported_fields` object.
pub fn assert_decorative_stars_match(chart: &Chart, supported: &Value, case_label: &str) {
    let actual = collect_decorative_stars(chart);

    assert_eq!(
        supported["decorative_runtime_star_count"].as_u64(),
        Some(48),
        "{case_label}: fixture decorative count mismatch"
    );
    assert_eq!(
        chart.decorative_stars().len(),
        48,
        "{case_label}: decorative count mismatch"
    );

    for expected_palace in supported["decorative_stars"]
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

/// Asserts the 岁前 algorithm boundary (大耗 vs 岁破) for the chosen algorithm.
pub fn assert_suiqian_algorithm_boundary(
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
