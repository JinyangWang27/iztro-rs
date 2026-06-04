use std::collections::{HashMap, HashSet};

use iztro_core::{
    BirthContext, Brightness, CalendarDate, Chart, DeterministicMinorStarPlacer, EARTHLY_BRANCHES,
    EarthlyBranch, Gender, HeavenlyStem, LunarDay, LunarMonth, MethodProfile,
    MinorStarPlacementInput, MinorStarPlacer, Mutagen, NatalChartInput,
    NatalChartWithSupportedStarsInput, Scope, StarCategory, StarKind, StarName,
    birth_year_star_mutagen, build_minimal_natal_chart, build_natal_chart_with_supported_stars,
    minor_star_brightness, minor_star_metadata, minor_star_metadata_table,
    represented_star_metadata_table, star_metadata, try_minor_star_metadata, try_star_metadata,
};
use serde_json::Value;

const MINOR_STARS_1990_FIXTURE: &str =
    include_str!("../../../fixtures/iztro/minor_stars_1990_05_17_chen_female.json");
const MINOR_STARS_1988_FIXTURE: &str =
    include_str!("../../../fixtures/iztro/minor_stars_1988_03_14_zi_male.json");
const MINOR_STARS_1991_FIXTURE: &str =
    include_str!("../../../fixtures/iztro/minor_stars_1991_08_09_hai_female.json");

const ALL_MINOR_STARS: [StarName; 14] = [
    StarName::ZuoFu,
    StarName::YouBi,
    StarName::WenChang,
    StarName::WenQu,
    StarName::TianKui,
    StarName::TianYue,
    StarName::LuCun,
    StarName::TianMa,
    StarName::QingYang,
    StarName::TuoLuo,
    StarName::HuoXing,
    StarName::LingXing,
    StarName::DiKong,
    StarName::DiJie,
];

const REPRESENTED_MINOR_MUTAGENS: &[(HeavenlyStem, StarName, Mutagen)] = &[
    (HeavenlyStem::Bing, StarName::WenChang, Mutagen::Ke),
    (HeavenlyStem::Wu, StarName::YouBi, Mutagen::Ke),
    (HeavenlyStem::Ji, StarName::WenQu, Mutagen::Ji),
    (HeavenlyStem::Xin, StarName::WenQu, Mutagen::Ke),
    (HeavenlyStem::Xin, StarName::WenChang, Mutagen::Ji),
    (HeavenlyStem::Ren, StarName::ZuoFu, Mutagen::Ke),
];

#[test]
fn minor_star_metadata_covers_each_supported_minor_star_once() {
    let metadata = minor_star_metadata_table();
    let names: HashSet<StarName> = metadata.iter().map(|entry| entry.name()).collect();
    let keys: HashSet<&str> = metadata.iter().map(|entry| entry.key()).collect();

    assert_eq!(metadata.len(), ALL_MINOR_STARS.len());
    assert_eq!(names, HashSet::from(ALL_MINOR_STARS));
    assert_eq!(keys.len(), metadata.len());
}

#[test]
fn minor_star_metadata_uses_iztro_kind_mapping() {
    let expected = HashMap::from([
        (StarName::ZuoFu, StarKind::Soft),
        (StarName::YouBi, StarKind::Soft),
        (StarName::WenChang, StarKind::Soft),
        (StarName::WenQu, StarKind::Soft),
        (StarName::TianKui, StarKind::Soft),
        (StarName::TianYue, StarKind::Soft),
        (StarName::LuCun, StarKind::LuCun),
        (StarName::TianMa, StarKind::TianMa),
        (StarName::QingYang, StarKind::Tough),
        (StarName::TuoLuo, StarKind::Tough),
        (StarName::HuoXing, StarKind::Tough),
        (StarName::LingXing, StarKind::Tough),
        (StarName::DiKong, StarKind::Tough),
        (StarName::DiJie, StarKind::Tough),
    ]);

    for star in ALL_MINOR_STARS {
        let metadata = minor_star_metadata(star);

        assert_eq!(metadata.name(), star);
        assert_eq!(metadata.key(), star_key(star));
        assert_eq!(metadata.kind(), expected[&star]);
        assert_eq!(metadata.category(), StarCategory::Minor);
        assert_eq!(star_metadata(star), metadata);
        assert!(!metadata.chinese_name().is_empty());
    }
}

#[test]
fn try_minor_star_metadata_is_some_for_minor_and_none_for_major() {
    for star in ALL_MINOR_STARS {
        let metadata = try_minor_star_metadata(star).expect("minor star should be represented");
        assert_eq!(metadata, minor_star_metadata(star));
    }

    // A major star is not a represented minor star.
    assert!(try_minor_star_metadata(StarName::ZiWei).is_none());
}

#[test]
fn try_star_metadata_is_some_for_represented_major_and_minor_stars() {
    // Represented minor stars resolve through the unified accessor.
    for star in ALL_MINOR_STARS {
        let metadata = try_star_metadata(star).expect("represented minor star");
        assert_eq!(metadata, star_metadata(star));
    }

    // A represented major star also resolves.
    let zi_wei = try_star_metadata(StarName::ZiWei).expect("represented major star");
    assert_eq!(zi_wei.name(), StarName::ZiWei);
    assert_eq!(zi_wei, star_metadata(StarName::ZiWei));
}

#[test]
fn represented_star_metadata_table_includes_major_and_minor_stars() {
    let represented = represented_star_metadata_table();
    let names: HashSet<StarName> = represented.iter().map(|entry| entry.name()).collect();

    assert_eq!(represented.len(), 34);
    for star in ALL_MINOR_STARS {
        assert!(names.contains(&star), "missing metadata for {star:?}");
    }
}

#[test]
fn minor_star_brightness_uses_unknown_when_upstream_has_no_table() {
    assert_eq!(
        minor_star_brightness(StarName::WenChang, EarthlyBranch::Wu),
        Brightness::Trapped
    );
    assert_eq!(
        minor_star_brightness(StarName::HuoXing, EarthlyBranch::Yin),
        Brightness::Temple
    );
    assert_eq!(
        minor_star_brightness(StarName::LuCun, EarthlyBranch::Si),
        Brightness::Unknown
    );
    assert_eq!(
        minor_star_brightness(StarName::DiKong, EarthlyBranch::Zi),
        Brightness::Unknown
    );
}

#[test]
fn minor_star_brightness_matches_iztro_2_5_8_and_never_weak() {
    use Brightness::{Advantage, Favourable, Flat, Prosperous, Temple, Trapped, Unknown};

    // iztro 2.5.8 `STARS_INFO` minor-star brightness, in palace order from 寅
    // (Yin). `Unknown` stands for iztro's empty-string entry. Upstream provides
    // tables only for these six minor stars, and none of them uses 不 (Weak).
    let expected_from_yin: &[(StarName, [Brightness; 12])] = &[
        (
            StarName::WenChang,
            [
                Trapped, Favourable, Advantage, Temple, Trapped, Favourable, Advantage, Temple,
                Trapped, Favourable, Advantage, Temple,
            ],
        ),
        (
            StarName::WenQu,
            [
                Flat, Prosperous, Advantage, Temple, Trapped, Prosperous, Advantage, Temple,
                Trapped, Prosperous, Advantage, Temple,
            ],
        ),
        (
            StarName::HuoXing,
            [
                Temple, Favourable, Trapped, Advantage, Temple, Favourable, Trapped, Advantage,
                Temple, Favourable, Trapped, Advantage,
            ],
        ),
        (
            StarName::LingXing,
            [
                Temple, Favourable, Trapped, Advantage, Temple, Favourable, Trapped, Advantage,
                Temple, Favourable, Trapped, Advantage,
            ],
        ),
        (
            StarName::QingYang,
            [
                Unknown, Trapped, Temple, Unknown, Trapped, Temple, Unknown, Trapped, Temple,
                Unknown, Trapped, Temple,
            ],
        ),
        (
            StarName::TuoLuo,
            [
                Trapped, Unknown, Temple, Trapped, Unknown, Temple, Trapped, Unknown, Temple,
                Trapped, Unknown, Temple,
            ],
        ),
    ];

    for (star, expected) in expected_from_yin {
        for (offset, brightness) in expected.iter().enumerate() {
            let branch = EarthlyBranch::Yin.offset(offset as isize);
            assert_eq!(
                minor_star_brightness(*star, branch),
                *brightness,
                "{star:?} brightness at {branch:?} should match iztro 2.5.8"
            );
        }
    }

    // The eight remaining supported minors have no upstream brightness table.
    for star in [
        StarName::ZuoFu,
        StarName::YouBi,
        StarName::TianKui,
        StarName::TianYue,
        StarName::LuCun,
        StarName::TianMa,
        StarName::DiKong,
        StarName::DiJie,
    ] {
        for branch in EARTHLY_BRANCHES {
            assert_eq!(
                minor_star_brightness(star, branch),
                Brightness::Unknown,
                "{star:?} has no iztro table and should be Unknown at {branch:?}"
            );
        }
    }

    // No minor star ever resolves to 不 (Weak); upstream reserves it for majors.
    for star in ALL_MINOR_STARS {
        for branch in EARTHLY_BRANCHES {
            assert_ne!(
                minor_star_brightness(star, branch),
                Brightness::Weak,
                "{star:?} unexpectedly Weak at {branch:?}"
            );
        }
    }
}

#[test]
fn birth_year_star_mutagen_covers_major_and_represented_minor_targets() {
    assert_eq!(
        birth_year_star_mutagen(HeavenlyStem::Geng, StarName::TaiYang),
        Some(Mutagen::Lu)
    );
    assert_eq!(
        birth_year_star_mutagen(HeavenlyStem::Xin, StarName::WenQu),
        Some(Mutagen::Ke)
    );
    assert_eq!(
        birth_year_star_mutagen(HeavenlyStem::Xin, StarName::WenChang),
        Some(Mutagen::Ji)
    );
    assert_eq!(
        birth_year_star_mutagen(HeavenlyStem::Jia, StarName::ZuoFu),
        None
    );
}

#[test]
fn represented_minor_birth_year_mutagens_match_iztro_table() {
    let expected: HashMap<(HeavenlyStem, StarName), Mutagen> = REPRESENTED_MINOR_MUTAGENS
        .iter()
        .map(|&(stem, star, mutagen)| ((stem, star), mutagen))
        .collect();

    for stem in [
        HeavenlyStem::Jia,
        HeavenlyStem::Yi,
        HeavenlyStem::Bing,
        HeavenlyStem::Ding,
        HeavenlyStem::Wu,
        HeavenlyStem::Ji,
        HeavenlyStem::Geng,
        HeavenlyStem::Xin,
        HeavenlyStem::Ren,
        HeavenlyStem::Gui,
    ] {
        for star in ALL_MINOR_STARS {
            assert_eq!(
                birth_year_star_mutagen(stem, star),
                expected.get(&(stem, star)).copied(),
                "unexpected minor mutagen for {stem:?} {star:?}"
            );
        }
    }
}

#[test]
fn placer_places_each_supported_minor_star_exactly_once() {
    let chart = place_minor_stars(
        1990,
        5,
        17,
        EarthlyBranch::Chen,
        Gender::Female,
        HeavenlyStem::Geng,
        EarthlyBranch::Wu,
    );
    let placed: Vec<_> = chart
        .stars_by_category(StarCategory::Minor)
        .into_iter()
        .map(|fact| fact.placement().name())
        .collect();
    let unique: HashSet<StarName> = placed.iter().copied().collect();

    assert_eq!(placed.len(), ALL_MINOR_STARS.len());
    assert_eq!(unique, HashSet::from(ALL_MINOR_STARS));
}

#[test]
fn placer_matches_iztro_minor_star_fixtures() {
    for fixture in fixture_values() {
        let chart = supported_chart_from_fixture(&fixture);
        assert_minor_stars_match_fixture(&chart, &fixture);
    }
}

#[test]
fn direct_minor_placer_matches_iztro_branch_formulas() {
    let fixture: Value =
        serde_json::from_str(MINOR_STARS_1988_FIXTURE).expect("fixture should be valid JSON");
    let chart = minimal_chart_from_fixture(&fixture);
    let input = &fixture["input"];
    let placed = DeterministicMinorStarPlacer
        .place_minor_stars(
            chart,
            MinorStarPlacementInput::new(
                LunarMonth::new(input["lunar_month"].as_u64().expect("lunar_month") as u8)
                    .expect("fixture lunar month should be valid"),
                parse_branch_key(input["birth_time"].as_str().expect("birth_time")),
                parse_stem_key(input["birth_year_stem"].as_str().expect("birth_year_stem")),
                parse_branch_key(
                    input["birth_year_branch"]
                        .as_str()
                        .expect("birth_year_branch"),
                ),
            ),
        )
        .expect("minor stars should place deterministically");

    assert_minor_stars_match_fixture(&placed, &fixture);
}

#[test]
fn generic_star_queries_return_minor_and_major_context() {
    let fixture: Value =
        serde_json::from_str(MINOR_STARS_1990_FIXTURE).expect("fixture should be valid JSON");
    let chart = supported_chart_from_fixture(&fixture);

    let lu_cun = chart
        .star(StarName::LuCun)
        .expect("Lu Cun should be placed");
    assert_eq!(lu_cun.palace().branch(), EarthlyBranch::Shen);
    assert_eq!(lu_cun.placement().kind(), StarKind::LuCun);

    assert_eq!(
        chart
            .palace_containing_star(StarName::TianMa)
            .expect("Tian Ma palace should be queryable")
            .branch(),
        EarthlyBranch::Shen
    );
    assert_eq!(
        chart
            .star(StarName::WenChang)
            .expect("Wen Chang should be placed")
            .palace()
            .branch(),
        EarthlyBranch::Wu
    );
    assert_eq!(chart.stars_by_kind(StarKind::LuCun).len(), 1);
    assert_eq!(chart.stars_by_category(StarCategory::Minor).len(), 14);
    assert_eq!(chart.stars_by_category(StarCategory::Major).len(), 14);
}

#[test]
fn generic_star_queries_filter_by_palace_and_branch() {
    let fixture: Value =
        serde_json::from_str(MINOR_STARS_1991_FIXTURE).expect("fixture should be valid JSON");
    let chart = supported_chart_from_fixture(&fixture);
    let xu_palace = chart
        .palaces()
        .iter()
        .find(|palace| palace.branch() == EarthlyBranch::Xu)
        .expect("Xu palace should exist");

    let by_branch: HashSet<StarName> = chart
        .stars_in_branch(EarthlyBranch::Xu)
        .iter()
        .map(|star| star.placement().name())
        .collect();
    let by_palace: HashSet<StarName> = chart
        .stars_in_palace(xu_palace.name())
        .iter()
        .filter(|star| star.placement().category() == StarCategory::Minor)
        .map(|star| star.placement().name())
        .collect();

    assert!(by_branch.contains(&StarName::DiJie));
    assert!(by_branch.contains(&StarName::QingYang));
    assert_eq!(
        by_palace,
        HashSet::from([StarName::DiJie, StarName::QingYang])
    );
}

#[test]
fn chart_with_supported_stars_round_trips_through_json() {
    let fixture: Value =
        serde_json::from_str(MINOR_STARS_1991_FIXTURE).expect("fixture should be valid JSON");
    let chart = supported_chart_from_fixture(&fixture);
    let serialized = serde_json::to_string(&chart).expect("chart should serialize");
    let decoded: Chart = serde_json::from_str(&serialized).expect("chart should deserialize");

    assert_minor_stars_match_fixture(&decoded, &fixture);
    assert_eq!(
        decoded
            .star(StarName::WenQu)
            .expect("Wen Qu should remain queryable")
            .placement()
            .mutagen(),
        Some(Mutagen::Ke)
    );
}

fn place_minor_stars(
    lunar_year: i32,
    lunar_month: u8,
    lunar_day: u8,
    birth_time: EarthlyBranch,
    gender: Gender,
    birth_year_stem: HeavenlyStem,
    birth_year_branch: EarthlyBranch,
) -> Chart {
    DeterministicMinorStarPlacer
        .place_minor_stars(
            build_minimal_natal_chart(NatalChartInput::new(
                BirthContext::new(
                    CalendarDate::lunar(lunar_year, lunar_month, lunar_day),
                    birth_time,
                    gender,
                ),
                MethodProfile::placeholder("minor_star_profile"),
                LunarMonth::new(lunar_month).expect("lunar month should be valid"),
                birth_year_stem,
            ))
            .expect("minimal natal chart should build"),
            MinorStarPlacementInput::new(
                LunarMonth::new(lunar_month).expect("lunar month should be valid"),
                birth_time,
                birth_year_stem,
                birth_year_branch,
            ),
        )
        .expect("minor star placement should not fail")
}

fn supported_chart_from_fixture(fixture: &Value) -> Chart {
    let input = &fixture["input"];

    build_natal_chart_with_supported_stars(NatalChartWithSupportedStarsInput::new(
        BirthContext::new(
            CalendarDate::lunar(
                input["lunar_year"].as_i64().expect("lunar_year") as i32,
                input["lunar_month"].as_u64().expect("lunar_month") as u8,
                input["lunar_day"].as_u64().expect("lunar_day") as u8,
            ),
            parse_branch_key(input["birth_time"].as_str().expect("birth_time")),
            parse_gender_key(input["gender"].as_str().expect("gender")),
        ),
        MethodProfile::placeholder("supported_star_fixture"),
        LunarMonth::new(input["lunar_month"].as_u64().expect("lunar_month") as u8)
            .expect("fixture lunar month should be valid"),
        LunarDay::new(input["lunar_day"].as_u64().expect("lunar_day") as u8)
            .expect("fixture lunar day should be valid"),
        parse_stem_key(input["birth_year_stem"].as_str().expect("birth_year_stem")),
        parse_branch_key(
            input["birth_year_branch"]
                .as_str()
                .expect("birth_year_branch"),
        ),
    ))
    .expect("supported natal chart should build")
}

fn minimal_chart_from_fixture(fixture: &Value) -> Chart {
    let input = &fixture["input"];

    build_minimal_natal_chart(NatalChartInput::new(
        BirthContext::new(
            CalendarDate::lunar(
                input["lunar_year"].as_i64().expect("lunar_year") as i32,
                input["lunar_month"].as_u64().expect("lunar_month") as u8,
                input["lunar_day"].as_u64().expect("lunar_day") as u8,
            ),
            parse_branch_key(input["birth_time"].as_str().expect("birth_time")),
            parse_gender_key(input["gender"].as_str().expect("gender")),
        ),
        MethodProfile::placeholder("minimal_minor_fixture"),
        LunarMonth::new(input["lunar_month"].as_u64().expect("lunar_month") as u8)
            .expect("fixture lunar month should be valid"),
        parse_stem_key(input["birth_year_stem"].as_str().expect("birth_year_stem")),
    ))
    .expect("minimal natal chart should build")
}

fn assert_minor_stars_match_fixture(chart: &Chart, fixture: &Value) {
    assert_eq!(
        fixture["metadata"]["target_version"].as_str(),
        Some("2.5.8")
    );

    let actual = collect_minor_star_facts(chart);
    for expected_palace in fixture["supported_fields"]["minor_stars"]
        .as_array()
        .expect("fixture should include supported minor-star fields")
    {
        let branch = parse_branch_key(expected_palace["branch"].as_str().expect("branch"));
        let expected_stars: HashSet<StarName> = expected_palace["stars"]
            .as_array()
            .expect("stars array")
            .iter()
            .map(|star| parse_star_key(star["name"].as_str().expect("star name")))
            .collect();
        let actual_stars: HashSet<StarName> = actual
            .keys()
            .filter_map(|&(actual_branch, star)| (actual_branch == branch).then_some(star))
            .collect();

        assert_eq!(
            actual_stars, expected_stars,
            "minor-star mismatch in {branch:?}"
        );

        for expected_star in expected_palace["stars"].as_array().expect("stars array") {
            let name = parse_star_key(expected_star["name"].as_str().expect("star name"));
            let got = actual
                .get(&(branch, name))
                .unwrap_or_else(|| panic!("missing {name:?} in {branch:?}"));

            assert_eq!(
                got.kind(),
                parse_kind_key(expected_star["kind"].as_str().expect("kind")),
                "kind mismatch for {name:?} in {branch:?}"
            );
            assert_eq!(
                got.category(),
                StarCategory::Minor,
                "category mismatch for {name:?} in {branch:?}"
            );
            assert_eq!(
                got.brightness(),
                parse_brightness_key(expected_star["brightness"].as_str().expect("brightness")),
                "brightness mismatch for {name:?} in {branch:?}"
            );
            assert_eq!(
                got.mutagen(),
                parse_optional_mutagen_key(&expected_star["mutagen"]),
                "mutagen mismatch for {name:?} in {branch:?}"
            );
            assert_eq!(got.scope(), Scope::Natal);
        }
    }
}

fn collect_minor_star_facts(
    chart: &Chart,
) -> HashMap<(EarthlyBranch, StarName), &iztro_core::StarPlacement> {
    let mut out = HashMap::new();
    for palace in chart.palaces() {
        for star in palace.stars() {
            if star.category() == StarCategory::Minor {
                out.insert((palace.branch(), star.name()), star);
            }
        }
    }
    out
}

fn fixture_values() -> Vec<Value> {
    [
        MINOR_STARS_1990_FIXTURE,
        MINOR_STARS_1988_FIXTURE,
        MINOR_STARS_1991_FIXTURE,
    ]
    .into_iter()
    .map(|fixture| serde_json::from_str(fixture).expect("fixture should be valid JSON"))
    .collect()
}

fn star_key(star: StarName) -> &'static str {
    match star {
        StarName::ZiWei => "zi_wei",
        StarName::TianJi => "tian_ji",
        StarName::TaiYang => "tai_yang",
        StarName::WuQu => "wu_qu",
        StarName::TianTong => "tian_tong",
        StarName::LianZhen => "lian_zhen",
        StarName::TianFu => "tian_fu",
        StarName::TaiYin => "tai_yin",
        StarName::TanLang => "tan_lang",
        StarName::JuMen => "ju_men",
        StarName::TianXiang => "tian_xiang",
        StarName::TianLiang => "tian_liang",
        StarName::QiSha => "qi_sha",
        StarName::PoJun => "po_jun",
        StarName::ZuoFu => "zuo_fu",
        StarName::YouBi => "you_bi",
        StarName::WenChang => "wen_chang",
        StarName::WenQu => "wen_qu",
        StarName::TianKui => "tian_kui",
        StarName::TianYue => "tian_yue",
        StarName::LuCun => "lu_cun",
        StarName::TianMa => "tian_ma",
        StarName::QingYang => "qing_yang",
        StarName::TuoLuo => "tuo_luo",
        StarName::HuoXing => "huo_xing",
        StarName::LingXing => "ling_xing",
        StarName::DiKong => "di_kong",
        StarName::DiJie => "di_jie",
        StarName::HongLuan => "hong_luan",
        StarName::TianXi => "tian_xi",
        StarName::TianYao => "tian_yao",
        StarName::TianXing => "tian_xing",
        StarName::TaiFu => "tai_fu",
        StarName::FengGao => "feng_gao",
    }
}

fn parse_star_key(value: &str) -> StarName {
    match value {
        "zuo_fu" => StarName::ZuoFu,
        "you_bi" => StarName::YouBi,
        "wen_chang" => StarName::WenChang,
        "wen_qu" => StarName::WenQu,
        "tian_kui" => StarName::TianKui,
        "tian_yue" => StarName::TianYue,
        "lu_cun" => StarName::LuCun,
        "tian_ma" => StarName::TianMa,
        "qing_yang" => StarName::QingYang,
        "tuo_luo" => StarName::TuoLuo,
        "huo_xing" => StarName::HuoXing,
        "ling_xing" => StarName::LingXing,
        "di_kong" => StarName::DiKong,
        "di_jie" => StarName::DiJie,
        other => panic!("unsupported star key in fixture: {other}"),
    }
}

fn parse_kind_key(value: &str) -> StarKind {
    match value {
        "soft" => StarKind::Soft,
        "tough" => StarKind::Tough,
        "lucun" => StarKind::LuCun,
        "tianma" => StarKind::TianMa,
        other => panic!("unsupported kind key in fixture: {other}"),
    }
}

fn parse_brightness_key(value: &str) -> Brightness {
    match value {
        "temple" => Brightness::Temple,
        "prosperous" => Brightness::Prosperous,
        "advantage" => Brightness::Advantage,
        "favourable" => Brightness::Favourable,
        "flat" => Brightness::Flat,
        "weak" => Brightness::Weak,
        "trapped" => Brightness::Trapped,
        "unknown" => Brightness::Unknown,
        other => panic!("unsupported brightness key in fixture: {other}"),
    }
}

fn parse_optional_mutagen_key(value: &Value) -> Option<Mutagen> {
    match value.as_str() {
        Some("lu") => Some(Mutagen::Lu),
        Some("quan") => Some(Mutagen::Quan),
        Some("ke") => Some(Mutagen::Ke),
        Some("ji") => Some(Mutagen::Ji),
        None => None,
        Some(other) => panic!("unsupported mutagen key in fixture: {other}"),
    }
}

fn parse_branch_key(value: &str) -> EarthlyBranch {
    match value {
        "zi" => EarthlyBranch::Zi,
        "chou" => EarthlyBranch::Chou,
        "yin" => EarthlyBranch::Yin,
        "mao" => EarthlyBranch::Mao,
        "chen" => EarthlyBranch::Chen,
        "si" => EarthlyBranch::Si,
        "wu" => EarthlyBranch::Wu,
        "wei" => EarthlyBranch::Wei,
        "shen" => EarthlyBranch::Shen,
        "you" => EarthlyBranch::You,
        "xu" => EarthlyBranch::Xu,
        "hai" => EarthlyBranch::Hai,
        other => panic!("unsupported branch key in fixture: {other}"),
    }
}

fn parse_stem_key(value: &str) -> HeavenlyStem {
    match value {
        "jia" => HeavenlyStem::Jia,
        "yi" => HeavenlyStem::Yi,
        "bing" => HeavenlyStem::Bing,
        "ding" => HeavenlyStem::Ding,
        "wu" => HeavenlyStem::Wu,
        "ji" => HeavenlyStem::Ji,
        "geng" => HeavenlyStem::Geng,
        "xin" => HeavenlyStem::Xin,
        "ren" => HeavenlyStem::Ren,
        "gui" => HeavenlyStem::Gui,
        other => panic!("unsupported stem key in fixture: {other}"),
    }
}

fn parse_gender_key(value: &str) -> Gender {
    match value {
        "male" => Gender::Male,
        "female" => Gender::Female,
        other => panic!("unsupported gender key in fixture: {other}"),
    }
}
