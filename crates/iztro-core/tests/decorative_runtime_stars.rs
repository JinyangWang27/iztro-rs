//! Verifies decorative runtime star-family (长生/博士/岁前/将前十二神) placement
//! against upstream iztro@2.5.8 fixtures, and the type boundary that keeps these
//! untyped facts separate from typed [`StarPlacement`]s.

use std::collections::{HashMap, HashSet};

use iztro_core::{
    BirthContext, CalendarDate, Chart, ChartAlgorithmKind, ChartError, DecorativeStarFamily,
    DecorativeStarPlacement, DecorativeStarPlacementInput, DecorativeStarPlacer,
    DeterministicDecorativeStarPlacer, EarthlyBranch, Gender, HeavenlyStem, KnownStarFamily,
    LunarDay, LunarMonth, MethodProfile, NatalChartWithSupportedStarsInput, Scope, StarName,
    build_empty_chart, build_natal_chart_with_supported_stars, known_star_metadata_table,
    try_known_star_metadata, try_star_metadata,
};
use serde_json::Value;

const DEFAULT_FIXTURES: [&str; 3] = [
    include_str!("../../../fixtures/iztro/runtime_decorative_default_1990_05_17_chen_female.json"),
    include_str!("../../../fixtures/iztro/runtime_decorative_default_1988_03_14_zi_male.json"),
    include_str!("../../../fixtures/iztro/runtime_decorative_default_1991_08_09_hai_female.json"),
];

const ZHONGZHOU_FIXTURES: [&str; 3] = [
    include_str!(
        "../../../fixtures/iztro/runtime_decorative_zhongzhou_1990_05_17_chen_female.json"
    ),
    include_str!("../../../fixtures/iztro/runtime_decorative_zhongzhou_1988_03_14_zi_male.json"),
    include_str!("../../../fixtures/iztro/runtime_decorative_zhongzhou_1991_08_09_hai_female.json"),
];

const DECORATIVE_FAMILIES: [(&str, DecorativeStarFamily); 4] = [
    ("changsheng12", DecorativeStarFamily::Changsheng12),
    ("boshi12", DecorativeStarFamily::Boshi12),
    ("suiqian12", DecorativeStarFamily::Suiqian12),
    ("jiangqian12", DecorativeStarFamily::Jiangqian12),
];

#[test]
fn decorative_families_match_upstream_fixtures() {
    for raw in DEFAULT_FIXTURES.into_iter().chain(ZHONGZHOU_FIXTURES) {
        let fixture = fixture_value(raw);
        let chart = chart_from_fixture(&fixture);
        let actual = collect_decorative(&chart);

        for palace in fixture["supported_fields"]["decorative_stars"]
            .as_array()
            .expect("fixture should list decorative palaces")
        {
            let branch = parse_branch(palace["branch"].as_str().expect("branch"));
            for (field, family) in DECORATIVE_FAMILIES {
                let expected = parse_star(palace[field].as_str().expect("family key"));
                assert_eq!(
                    actual.get(&(branch, family)).copied(),
                    Some(expected),
                    "{family:?} mismatch in {branch:?}"
                );
            }
        }
    }
}

#[test]
fn each_family_places_twelve_unique_entries() {
    for raw in DEFAULT_FIXTURES.into_iter().chain(ZHONGZHOU_FIXTURES) {
        let chart = chart_from_fixture(&fixture_value(raw));

        for (_, family) in DECORATIVE_FAMILIES {
            let names: Vec<StarName> = chart
                .decorative_stars()
                .iter()
                .filter(|fact| fact.placement().family() == family)
                .map(|fact| fact.name())
                .collect();
            assert_eq!(names.len(), 12, "{family:?} should place 12 entries");
            assert_eq!(
                names.iter().copied().collect::<HashSet<_>>().len(),
                12,
                "{family:?} entries should each appear exactly once"
            );
        }
    }
}

#[test]
fn suiqian_swaps_da_hao_for_sui_po_under_zhongzhou_only() {
    // 岁破 is a known 岁前 star with no StarKind: known-but-not-placed by default.
    let sui_po = try_known_star_metadata(StarName::SuiPo).expect("岁破 should be known");
    assert_eq!(sui_po.family(), KnownStarFamily::Suiqian12);
    assert!(sui_po.kind().is_none());

    let default_chart = chart_from_fixture(&fixture_value(DEFAULT_FIXTURES[0]));
    assert!(
        default_chart.decorative_star(StarName::SuiPo).is_none(),
        "default 岁前 places 大耗, not 岁破"
    );
    assert!(
        default_chart
            .decorative_star(StarName::DaHaoSuiqian)
            .is_some()
    );

    let zhongzhou_chart = chart_from_fixture(&fixture_value(ZHONGZHOU_FIXTURES[0]));
    assert!(
        zhongzhou_chart.decorative_star(StarName::SuiPo).is_some(),
        "Zhongzhou 岁前 places 岁破 in place of 大耗"
    );
    assert!(
        zhongzhou_chart
            .decorative_star(StarName::DaHaoSuiqian)
            .is_none()
    );
}

#[test]
fn decorative_entries_are_not_typed_placements() {
    let chart = chart_from_fixture(&fixture_value(ZHONGZHOU_FIXTURES[0]));

    // Decorative entries never appear in the typed star surface.
    for fact in chart.decorative_stars() {
        assert!(
            chart.star(fact.name()).is_none(),
            "{:?} should not be a typed StarPlacement",
            fact.name()
        );
    }

    // The typed star count is unchanged by decorative placement.
    let default_chart = chart_from_fixture(&fixture_value(DEFAULT_FIXTURES[0]));
    assert_eq!(default_chart.stars().len(), 66);
    assert_eq!(chart.stars().len(), 68);
}

#[test]
fn decorative_names_resolve_only_through_known_metadata() {
    let decorative_names: Vec<StarName> = known_star_metadata_table()
        .iter()
        .filter(|metadata| {
            matches!(
                metadata.family(),
                KnownStarFamily::Changsheng12
                    | KnownStarFamily::Boshi12
                    | KnownStarFamily::Suiqian12
                    | KnownStarFamily::Jiangqian12
            )
        })
        .map(|metadata| metadata.name())
        .collect();

    // The four families contribute 12 + 12 + 13 (岁前 + 岁破) + 12 known names.
    assert_eq!(decorative_names.len(), 49);

    for name in decorative_names {
        assert!(
            try_star_metadata(name).is_none(),
            "{name:?} must not resolve as a represented typed star"
        );
        let known = try_known_star_metadata(name).expect("decorative name should be known");
        assert!(known.kind().is_none(), "{name:?} must have no StarKind");
    }
}

#[test]
fn checked_constructor_rejects_typed_and_mismatched_stars() {
    // A typed star (紫微 has a StarKind) is rejected.
    assert!(
        DecorativeStarPlacement::try_new(
            StarName::ZiWei,
            DecorativeStarFamily::Changsheng12,
            Scope::Natal,
        )
        .is_err()
    );
    // A decorative star with the wrong family is rejected.
    assert!(
        DecorativeStarPlacement::try_new(
            StarName::ChangSheng,
            DecorativeStarFamily::Boshi12,
            Scope::Natal,
        )
        .is_err()
    );
    // The correct family is accepted.
    assert!(
        DecorativeStarPlacement::try_new(
            StarName::ChangSheng,
            DecorativeStarFamily::Changsheng12,
            Scope::Natal,
        )
        .is_ok()
    );
}

#[test]
fn decorative_placement_json_deserializes_through_checked_constructor() {
    let raw = r#"{"name":"chang_sheng","family":"changsheng12","scope":"natal"}"#;

    let placement: DecorativeStarPlacement =
        serde_json::from_str(raw).expect("valid decorative placement should deserialize");

    assert_eq!(placement.name(), StarName::ChangSheng);
    assert_eq!(placement.family(), DecorativeStarFamily::Changsheng12);
    assert_eq!(placement.scope(), Scope::Natal);
}

#[test]
fn decorative_placement_json_rejects_typed_star() {
    let raw = r#"{"name":"zi_wei","family":"changsheng12","scope":"natal"}"#;

    let error = serde_json::from_str::<DecorativeStarPlacement>(raw)
        .expect_err("typed stars must not deserialize as decorative placements");

    assert!(
        error
            .to_string()
            .contains("invalid decorative star placement"),
        "{error}"
    );
}

#[test]
fn decorative_placement_json_rejects_wrong_family() {
    let raw = r#"{"name":"chang_sheng","family":"boshi12","scope":"natal"}"#;

    let error = serde_json::from_str::<DecorativeStarPlacement>(raw)
        .expect_err("decorative placements must match their family");

    assert!(
        error
            .to_string()
            .contains("invalid decorative star placement"),
        "{error}"
    );
}

#[test]
fn decorative_placement_json_round_trips_valid_placement() {
    let placement = DecorativeStarPlacement::try_new(
        StarName::ChangSheng,
        DecorativeStarFamily::Changsheng12,
        Scope::Natal,
    )
    .expect("valid decorative placement should build");

    let encoded = serde_json::to_string(&placement).expect("placement should serialize");
    let decoded: DecorativeStarPlacement =
        serde_json::from_str(&encoded).expect("placement should deserialize");

    assert_eq!(decoded, placement);
}

#[test]
fn decorative_placement_requires_five_element_bureau() {
    let chart = build_empty_chart(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("missing_bureau_profile"),
    )
    .expect("empty chart should build");

    let error = DeterministicDecorativeStarPlacer
        .place_decorative_stars(
            chart,
            DecorativeStarPlacementInput::new(HeavenlyStem::Geng, EarthlyBranch::Wu),
        )
        .expect_err("decorative placement should require the five-element bureau");

    assert_eq!(error, ChartError::RequiredFiveElementBureauMissing);
}

fn collect_decorative(chart: &Chart) -> HashMap<(EarthlyBranch, DecorativeStarFamily), StarName> {
    let mut out = HashMap::new();
    for fact in chart.decorative_stars() {
        out.insert((fact.branch(), fact.placement().family()), fact.name());
    }
    out
}

fn chart_from_fixture(fixture: &Value) -> Chart {
    let input = &fixture["input"];
    let algorithm = match fixture["metadata"]["algorithm"].as_str() {
        Some("zhongzhou") => ChartAlgorithmKind::Zhongzhou,
        _ => ChartAlgorithmKind::QuanShu,
    };
    let profile = MethodProfile::new("iztro_2_5_8_runtime", algorithm, "iztro 2.5.8 runtime");

    build_natal_chart_with_supported_stars(NatalChartWithSupportedStarsInput::new(
        BirthContext::new(
            CalendarDate::lunar(
                input["lunar_year"].as_i64().expect("lunar_year") as i32,
                input["lunar_month"].as_u64().expect("lunar_month") as u8,
                input["lunar_day"].as_u64().expect("lunar_day") as u8,
            ),
            parse_branch(input["birth_time"].as_str().expect("birth_time")),
            parse_gender(input["gender"].as_str().expect("gender")),
        ),
        profile,
        LunarMonth::new(input["lunar_month"].as_u64().expect("lunar_month") as u8)
            .expect("lunar month"),
        LunarDay::new(input["lunar_day"].as_u64().expect("lunar_day") as u8).expect("lunar day"),
        parse_stem(input["birth_year_stem"].as_str().expect("birth_year_stem")),
        parse_branch(
            input["birth_year_branch"]
                .as_str()
                .expect("birth_year_branch"),
        ),
    ))
    .expect("runtime chart should build")
}

fn fixture_value(raw: &str) -> Value {
    serde_json::from_str(raw).expect("fixture should be valid JSON")
}

fn parse_star(key: &str) -> StarName {
    serde_json::from_str(&format!("\"{key}\"")).expect("star key should parse")
}

fn parse_branch(key: &str) -> EarthlyBranch {
    serde_json::from_str(&format!("\"{key}\"")).expect("branch key should parse")
}

fn parse_stem(key: &str) -> HeavenlyStem {
    serde_json::from_str(&format!("\"{key}\"")).expect("stem key should parse")
}

fn parse_gender(key: &str) -> Gender {
    serde_json::from_str(&format!("\"{key}\"")).expect("gender key should parse")
}
