//! Shared helpers for the supported-field E2E tests (by_solar and leap-month
//! by_lunar). These assert the normalized supported fields of a built `Chart`
//! against a fixture's `supported_fields` block. Kept local to test code.

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use iztro::core::{
    BirthTime, Brightness, Chart, ChartAlgorithmKind, DecorativeStarFamily, EarthlyBranch,
    FiveElementBureau, FlowStarBase, FlowStarScope, Gender, HeavenlyStem, LunarChartRequest,
    LunarDay, LunarMonth, MethodProfile, Mutagen, PalaceName, StarKind, StarName, StemBranch,
    by_lunar, flow_star_name, known_star_metadata_table, represented_star_metadata_table,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// Source of truth for the upstream `FunctionalAstrolabe#horoscope` supported
/// fields, shared by every temporal-layer integration test.
pub const HOROSCOPE_FIXTURE: &str = include_str!("../../fixtures/iztro/horoscope.json");

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

// --- Shared horoscope (運限) fixture helpers ------------------------------------
//
// The temporal-layer tests all parse the same `horoscope.json` cases the same
// way: build the natal chart through `by_lunar`, read a per-scope supported
// block, and assert palace names / mutagens / flow stars against it. These
// helpers centralize that parsing so each test asserts behavior, not JSON shape.

/// Returns every horoscope fixture case.
pub fn horoscope_fixture_cases() -> Vec<Value> {
    let fixture: Value =
        serde_json::from_str(HOROSCOPE_FIXTURE).expect("horoscope fixture should parse");

    fixture["cases"]
        .as_array()
        .expect("fixture cases should be an array")
        .to_vec()
}

/// Returns one horoscope fixture case by id.
pub fn horoscope_fixture_case(case_id: &str) -> Value {
    horoscope_fixture_cases()
        .into_iter()
        .find(|case| case["id"].as_str() == Some(case_id))
        .unwrap_or_else(|| panic!("missing horoscope fixture case {case_id}"))
}

/// Builds the natal chart for a horoscope fixture case through `by_lunar`.
pub fn build_chart_from_horoscope_fixture_case(case: &Value) -> Chart {
    let input = &case["input"];
    assert_eq!(
        input["calendar"].as_str(),
        Some("lunar"),
        "horoscope fixtures should build through by_lunar"
    );
    let lunar_year = input["year"].as_i64().expect("fixture lunar year") as i32;
    let birth_year = StemBranch::from_lunar_year(lunar_year);
    let method_profile = MethodProfile::new(
        case["id"].as_str().expect("case id"),
        parse_algorithm(input["algorithm"].as_str().expect("algorithm")),
        "horoscope fixture test",
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

    by_lunar(request).expect("by_lunar should build horoscope fixture chart")
}

/// Returns the `(year, month, day)` of a case's target solar date.
pub fn target_solar_date(case: &Value) -> (i32, u8, u8) {
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

/// Returns a case's target `timeIndex`.
pub fn target_time_index(case: &Value) -> u8 {
    case["input"]["target"]["time_index"]
        .as_u64()
        .expect("target time index") as u8
}

/// Returns a case's target birth time.
pub fn target_time(case: &Value) -> BirthTime {
    BirthTime::from_iztro_time_index(target_time_index(case))
        .expect("target time index should be valid")
}

/// Returns a case's declared target lunar year.
pub fn target_year(case: &Value) -> i32 {
    case["input"]["target"]["year"]
        .as_i64()
        .expect("fixture target year") as i32
}

/// Returns the stem-branch declared on a per-scope supported block.
pub fn scope_stem_branch(scope: &Value) -> StemBranch {
    StemBranch::try_new(
        parse_key::<HeavenlyStem>(scope["heavenly_stem"].as_str().expect("scope stem")),
        parse_key::<EarthlyBranch>(scope["earthly_branch"].as_str().expect("scope branch")),
    )
    .expect("fixture scope stem-branch should be valid")
}

/// Maps a per-scope `palace_names` array (Yin-first) to branch-keyed names.
pub fn expected_palace_names_by_branch(scope: &Value) -> HashMap<EarthlyBranch, PalaceName> {
    scope["palace_names"]
        .as_array()
        .expect("scope palace names")
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

/// Resolves a per-scope `mutagen` block to `(star, natal branch) -> transform`.
pub fn expected_scope_mutagens(
    scope: &Value,
    chart: &Chart,
) -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    scope["mutagen"]
        .as_object()
        .expect("scope mutagen map")
        .iter()
        .filter_map(|(transform, entry)| {
            let star = parse_key::<StarName>(entry["star"].as_str().expect("mutagen star"));
            let branch = chart.star(star).map(|fact| fact.palace().branch())?;
            Some(((star, branch), parse_key::<Mutagen>(transform)))
        })
        .collect()
}

/// Resolves a per-scope `flow_stars` array to `star -> (branch, kind)`, adding
/// the yearly-only 年解 (`NianJieYearly`) when `nian_jie_branch` is present.
pub fn expected_scope_flow_stars(
    scope: &Value,
    flow_scope: FlowStarScope,
) -> HashMap<StarName, (EarthlyBranch, StarKind)> {
    let mut expected: HashMap<StarName, (EarthlyBranch, StarKind)> = scope["flow_stars"]
        .as_array()
        .expect("scope flow stars")
        .iter()
        .map(|entry| {
            let base = parse_flow_base(entry["base"].as_str().expect("flow star base"));
            (
                flow_star_name(flow_scope, base),
                (
                    parse_key::<EarthlyBranch>(entry["branch"].as_str().expect("branch")),
                    flow_star_kind(entry["type"].as_str().expect("type")),
                ),
            )
        })
        .collect();

    if let Some(branch) = scope["nian_jie_branch"].as_str() {
        expected.insert(
            StarName::NianJieYearly,
            (parse_key::<EarthlyBranch>(branch), StarKind::Helper),
        );
    }

    expected
}

/// Maps a fixture flow-star `base` label to its `FlowStarBase`.
pub fn parse_flow_base(value: &str) -> FlowStarBase {
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

/// Maps a fixture flow-star `type` label to its `StarKind`.
pub fn flow_star_kind(value: &str) -> StarKind {
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

/// Maps a fixture `gender` label to its `Gender`.
pub fn parse_gender(value: &str) -> Gender {
    match value {
        "female" => Gender::Female,
        "male" => Gender::Male,
        other => panic!("unsupported fixture gender: {other}"),
    }
}
