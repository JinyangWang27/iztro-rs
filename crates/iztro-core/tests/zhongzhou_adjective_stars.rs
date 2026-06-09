use std::collections::{HashMap, HashSet};

use iztro_core::{
    BirthContext, Brightness, CalendarDate, Chart, ChartAlgorithmKind, EarthlyBranch, FlowStarBase,
    FlowStarScope, Gender, HeavenlyStem, LunarChartRequest, LunarDay, LunarMonth, MethodProfile,
    NatalChartWithSupportedStarsInput, Scope, StarCategory, StarKind, StarName,
    build_natal_chart_with_supported_stars, by_lunar, known_star_metadata_table,
    represented_star_metadata_table, try_adjective_star_metadata, try_flow_star_parts,
    try_star_metadata,
};
use serde_json::Value;

const ZHONGZHOU_STARS: [StarName; 4] = [
    StarName::LongDeAdj,
    StarName::JieKong,
    StarName::JieShaAdj,
    StarName::DaHaoAdj,
];

const FLOW_SCOPES: [FlowStarScope; 5] = [
    FlowStarScope::Decadal,
    FlowStarScope::Yearly,
    FlowStarScope::Monthly,
    FlowStarScope::Daily,
    FlowStarScope::Hourly,
];

const FLOW_BASES: [FlowStarBase; 10] = [
    FlowStarBase::Kui,
    FlowStarBase::Yue,
    FlowStarBase::Chang,
    FlowStarBase::Qu,
    FlowStarBase::Lu,
    FlowStarBase::Yang,
    FlowStarBase::Tuo,
    FlowStarBase::Ma,
    FlowStarBase::Luan,
    FlowStarBase::Xi,
];

const ZHONGZHOU_1990_FIXTURE: &str =
    include_str!("../../../fixtures/iztro/zhongzhou_adjective_stars_1990_05_17_chen_female.json");
const ZHONGZHOU_1988_FIXTURE: &str =
    include_str!("../../../fixtures/iztro/zhongzhou_adjective_stars_1988_03_14_zi_male.json");
const ZHONGZHOU_1991_FIXTURE: &str =
    include_str!("../../../fixtures/iztro/zhongzhou_adjective_stars_1991_08_09_hai_female.json");

#[test]
fn default_profile_keeps_default_adjective_star_output() {
    let chart = by_lunar(
        LunarChartRequest::builder()
            .lunar_year(1990)
            .lunar_month(LunarMonth::new(5).expect("month"))
            .lunar_day(LunarDay::new(17).expect("day"))
            .birth_time(EarthlyBranch::Chen)
            .gender(Gender::Female)
            .birth_year_stem(HeavenlyStem::Geng)
            .birth_year_branch(EarthlyBranch::Wu)
            .method_profile(MethodProfile::placeholder("default_adjective_output"))
            .build()
            .expect("request should build"),
    )
    .expect("default by_lunar should build");

    assert_eq!(chart.stars().len(), 66);
    assert_eq!(chart.stars_by_category(StarCategory::Adjective).len(), 38);
    for star in ZHONGZHOU_STARS {
        assert!(
            chart.palace_containing_star(star).is_none(),
            "{star:?} should not be placed by default"
        );
    }
    assert!(chart.palace_containing_star(StarName::JieLu).is_some());
    assert!(chart.palace_containing_star(StarName::KongWang).is_some());
}

#[test]
fn zhongzhou_profile_places_only_upstream_zhongzhou_adjective_stars() {
    let chart = zhongzhou_chart_from_fixture(&fixture_value(ZHONGZHOU_1990_FIXTURE));

    assert_eq!(chart.stars().len(), 68);
    assert_eq!(chart.stars_by_category(StarCategory::Adjective).len(), 40);
    assert_zhongzhou_star_branches(
        &chart,
        HashMap::from([
            (StarName::LongDeAdj, EarthlyBranch::Chou),
            (StarName::JieKong, EarthlyBranch::Wu),
            (StarName::JieShaAdj, EarthlyBranch::Hai),
            (StarName::DaHaoAdj, EarthlyBranch::Chou),
        ]),
    );
    assert!(chart.palace_containing_star(StarName::JieLu).is_none());
    assert!(chart.palace_containing_star(StarName::KongWang).is_none());
}

#[test]
fn zhongzhou_adjective_stars_match_upstream_fixtures_exactly() {
    for fixture in zhongzhou_fixture_values() {
        let chart = zhongzhou_chart_from_fixture(&fixture);

        assert_eq!(
            fixture["metadata"]["target_version"].as_str(),
            Some("2.5.8")
        );
        assert_eq!(fixture["metadata"]["algorithm"].as_str(), Some("zhongzhou"));
        assert_eq!(
            fixture["metadata"]["adjective_star_count"].as_u64(),
            Some(40)
        );
        assert_adjective_stars_match_fixture(&chart, &fixture);
        assert!(chart.palace_containing_star(StarName::JieLu).is_none());
        assert!(chart.palace_containing_star(StarName::KongWang).is_none());
    }
}

#[test]
fn zhongzhou_metadata_is_represented_but_known_family_stays_zhongzhou() {
    for star in ZHONGZHOU_STARS {
        assert!(try_adjective_star_metadata(star).is_some());
        assert!(try_star_metadata(star).is_some());
    }

    assert_eq!(known_star_metadata_table().len(), 170);
    assert_eq!(represented_star_metadata_table().len(), 70);

    let known_names: HashSet<StarName> = known_star_metadata_table()
        .iter()
        .map(|metadata| metadata.name())
        .collect();
    for represented in represented_star_metadata_table() {
        assert!(
            known_names.contains(&represented.name()),
            "{:?} represented but not known",
            represented.name()
        );
    }
}

#[test]
fn natal_zhongzhou_output_does_not_place_flow_stars() {
    let chart = zhongzhou_chart_from_fixture(&fixture_value(ZHONGZHOU_1991_FIXTURE));

    for fact in chart.stars() {
        assert_eq!(try_flow_star_parts(fact.placement().name()), None);
    }
    for scope in FLOW_SCOPES {
        for base in FLOW_BASES {
            let flow_name = iztro_core::flow_star_name(scope, base);
            assert!(
                chart.palace_containing_star(flow_name).is_none(),
                "{flow_name:?} should not be placed in natal chart"
            );
        }
    }
    assert!(
        chart
            .palace_containing_star(StarName::NianJieYearly)
            .is_none()
    );
}

fn zhongzhou_chart_from_fixture(fixture: &Value) -> Chart {
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
        MethodProfile::new(
            "iztro_2_5_8_zhongzhou",
            ChartAlgorithmKind::Zhongzhou,
            "iztro 2.5.8 Zhongzhou algorithm",
        ),
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
    .expect("Zhongzhou chart should build")
}

fn assert_zhongzhou_star_branches(chart: &Chart, expected: HashMap<StarName, EarthlyBranch>) {
    for (star, branch) in expected {
        assert_eq!(
            chart
                .palace_containing_star(star)
                .unwrap_or_else(|| panic!("{star:?} should be placed"))
                .branch(),
            branch,
            "unexpected branch for {star:?}"
        );
    }
}

fn assert_adjective_stars_match_fixture(chart: &Chart, fixture: &Value) {
    let actual = collect_adjective_star_facts(chart);
    for expected_palace in fixture["supported_fields"]["adjective_stars"]
        .as_array()
        .expect("fixture should include supported adjective-star fields")
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
            "adjective-star mismatch in {branch:?}"
        );

        for expected_star in expected_palace["stars"].as_array().expect("stars array") {
            let name = parse_star_key(expected_star["name"].as_str().expect("star name"));
            let got = actual
                .get(&(branch, name))
                .unwrap_or_else(|| panic!("missing {name:?} in {branch:?}"));

            assert_eq!(
                got.kind(),
                kind_from_upstream_type(expected_star["type"].as_str().expect("type")),
                "kind mismatch for {name:?} in {branch:?}"
            );
            assert_eq!(got.category(), StarCategory::Adjective);
            assert_eq!(got.brightness(), Brightness::Unknown);
            assert_eq!(got.mutagen(), None);
            assert_eq!(got.scope(), Scope::Natal);
        }
    }
}

fn collect_adjective_star_facts(
    chart: &Chart,
) -> HashMap<(EarthlyBranch, StarName), &iztro_core::StarPlacement> {
    let mut out = HashMap::new();
    for palace in chart.palaces() {
        for star in palace.stars() {
            if star.category() == StarCategory::Adjective {
                out.insert((palace.branch(), star.name()), star);
            }
        }
    }
    out
}

fn fixture_value(raw: &str) -> Value {
    serde_json::from_str(raw).expect("fixture should be valid JSON")
}

fn zhongzhou_fixture_values() -> Vec<Value> {
    [
        ZHONGZHOU_1990_FIXTURE,
        ZHONGZHOU_1988_FIXTURE,
        ZHONGZHOU_1991_FIXTURE,
    ]
    .into_iter()
    .map(fixture_value)
    .collect()
}

fn kind_from_upstream_type(value: &str) -> StarKind {
    match value {
        "flower" => StarKind::Flower,
        "adjective" => StarKind::Adjective,
        "helper" => StarKind::Helper,
        other => panic!("unsupported adjective type in fixture: {other}"),
    }
}

fn parse_star_key(value: &str) -> StarName {
    match value {
        "hong_luan" => StarName::HongLuan,
        "tian_xi" => StarName::TianXi,
        "tian_yao" => StarName::TianYao,
        "tian_xing" => StarName::TianXing,
        "tai_fu" => StarName::TaiFu,
        "feng_gao" => StarName::FengGao,
        "san_tai" => StarName::SanTai,
        "ba_zuo" => StarName::BaZuo,
        "long_chi" => StarName::LongChi,
        "feng_ge" => StarName::FengGe,
        "tian_ku" => StarName::TianKu,
        "tian_xu" => StarName::TianXu,
        "en_guang" => StarName::EnGuang,
        "tian_gui" => StarName::TianGui,
        "tian_wu" => StarName::TianWu,
        "tian_yue_adj" => StarName::TianYueAdj,
        "yin_sha" => StarName::YinSha,
        "jie_shen" => StarName::JieShen,
        "hua_gai" => StarName::HuaGai,
        "gu_chen" => StarName::GuChen,
        "gua_su" => StarName::GuaSu,
        "fei_lian" => StarName::FeiLian,
        "po_sui" => StarName::PoSui,
        "tian_de" => StarName::TianDe,
        "yue_de" => StarName::YueDe,
        "nian_jie" => StarName::NianJie,
        "xian_chi" => StarName::XianChi,
        "tian_kong" => StarName::TianKong,
        "tian_guan" => StarName::TianGuan,
        "tian_chu" => StarName::TianChu,
        "tian_fu_adj" => StarName::TianFuAdj,
        "tian_cai" => StarName::TianCai,
        "tian_shou" => StarName::TianShou,
        "tian_shang" => StarName::TianShang,
        "tian_shi" => StarName::TianShi,
        "jie_lu" => StarName::JieLu,
        "kong_wang" => StarName::KongWang,
        "xun_kong" => StarName::XunKong,
        "long_de_adj" => StarName::LongDeAdj,
        "jie_kong" => StarName::JieKong,
        "jie_sha_adj" => StarName::JieShaAdj,
        "da_hao_adj" => StarName::DaHaoAdj,
        other => panic!("unsupported adjective star key in fixture: {other}"),
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
