use std::collections::{HashMap, HashSet};

use iztro_core::{
    AdjectiveStarPlacementInput, AdjectiveStarPlacer, BirthContext, Brightness, CalendarDate,
    Chart, ChartError, DeterministicAdjectiveStarPlacer, DeterministicMinorStarPlacer,
    EARTHLY_BRANCHES, EarthlyBranch, Gender, HeavenlyStem, LunarChartRequest, LunarDay, LunarMonth,
    MethodProfile, MinorStarPlacementInput, MinorStarPlacer, NatalChartWithMajorStarsInput,
    NatalChartWithSupportedStarsInput, Scope, StarCategory, StarKind, StarName,
    adjective_star_metadata, adjective_star_metadata_table, build_natal_chart_with_major_stars,
    build_natal_chart_with_supported_stars, by_lunar, represented_star_metadata_table,
    star_metadata, try_adjective_star_metadata, try_star_metadata,
};
use serde_json::Value;

const ADJECTIVE_STARS_1990_FIXTURE: &str = include_str!(
    "../../../fixtures/iztro/adjective_stars_full_default_1990_05_17_chen_female.json"
);
const ADJECTIVE_STARS_1988_FIXTURE: &str =
    include_str!("../../../fixtures/iztro/adjective_stars_full_default_1988_03_14_zi_male.json");
const ADJECTIVE_STARS_1991_FIXTURE: &str =
    include_str!("../../../fixtures/iztro/adjective_stars_full_default_1991_08_09_hai_female.json");

/// The full default-algorithm natal adjective/helper-star (杂曜) set (38).
const DEFAULT_ADJECTIVE_STARS: [StarName; 38] = [
    StarName::HongLuan,
    StarName::TianXi,
    StarName::TianYao,
    StarName::TianXing,
    StarName::TaiFu,
    StarName::FengGao,
    StarName::SanTai,
    StarName::BaZuo,
    StarName::LongChi,
    StarName::FengGe,
    StarName::TianKu,
    StarName::TianXu,
    StarName::EnGuang,
    StarName::TianGui,
    StarName::TianWu,
    StarName::TianYueAdj,
    StarName::YinSha,
    StarName::JieShen,
    StarName::HuaGai,
    StarName::GuChen,
    StarName::GuaSu,
    StarName::FeiLian,
    StarName::PoSui,
    StarName::TianDe,
    StarName::YueDe,
    StarName::NianJie,
    StarName::XianChi,
    StarName::TianKong,
    StarName::TianGuan,
    StarName::TianChu,
    StarName::TianFuAdj,
    StarName::TianCai,
    StarName::TianShou,
    StarName::TianShang,
    StarName::TianShi,
    StarName::JieLu,
    StarName::KongWang,
    StarName::XunKong,
];

/// All represented natal adjective/helper stars, including Zhongzhou-only stars.
const ALL_ADJECTIVE_STARS: [StarName; 42] = [
    StarName::HongLuan,
    StarName::TianXi,
    StarName::TianYao,
    StarName::TianXing,
    StarName::TaiFu,
    StarName::FengGao,
    StarName::SanTai,
    StarName::BaZuo,
    StarName::LongChi,
    StarName::FengGe,
    StarName::TianKu,
    StarName::TianXu,
    StarName::EnGuang,
    StarName::TianGui,
    StarName::TianWu,
    StarName::TianYueAdj,
    StarName::YinSha,
    StarName::JieShen,
    StarName::HuaGai,
    StarName::GuChen,
    StarName::GuaSu,
    StarName::FeiLian,
    StarName::PoSui,
    StarName::TianDe,
    StarName::YueDe,
    StarName::NianJie,
    StarName::XianChi,
    StarName::TianKong,
    StarName::TianGuan,
    StarName::TianChu,
    StarName::TianFuAdj,
    StarName::TianCai,
    StarName::TianShou,
    StarName::TianShang,
    StarName::TianShi,
    StarName::JieLu,
    StarName::KongWang,
    StarName::XunKong,
    StarName::LongDeAdj,
    StarName::JieKong,
    StarName::JieShaAdj,
    StarName::DaHaoAdj,
];

#[test]
fn adjective_star_metadata_table_covers_each_selected_star_once() {
    let metadata = adjective_star_metadata_table();
    let names: HashSet<StarName> = metadata.iter().map(|entry| entry.name()).collect();
    let keys: HashSet<&str> = metadata.iter().map(|entry| entry.key()).collect();

    assert_eq!(metadata.len(), ALL_ADJECTIVE_STARS.len());
    assert_eq!(names, HashSet::from(ALL_ADJECTIVE_STARS));
    assert_eq!(keys.len(), metadata.len());
}

#[test]
fn adjective_star_metadata_uses_expected_kind_and_adjective_category() {
    let expected_kind = HashMap::from([
        (StarName::HongLuan, StarKind::Flower),
        (StarName::TianXi, StarKind::Flower),
        (StarName::TianYao, StarKind::Flower),
        (StarName::TianXing, StarKind::Adjective),
        (StarName::TaiFu, StarKind::Adjective),
        (StarName::FengGao, StarKind::Adjective),
        (StarName::SanTai, StarKind::Adjective),
        (StarName::BaZuo, StarKind::Adjective),
        (StarName::LongChi, StarKind::Adjective),
        (StarName::FengGe, StarKind::Adjective),
        (StarName::TianKu, StarKind::Adjective),
        (StarName::TianXu, StarKind::Adjective),
        (StarName::EnGuang, StarKind::Adjective),
        (StarName::TianGui, StarKind::Adjective),
        (StarName::TianWu, StarKind::Adjective),
        (StarName::TianYueAdj, StarKind::Adjective),
        (StarName::YinSha, StarKind::Adjective),
        (StarName::JieShen, StarKind::Helper),
        (StarName::HuaGai, StarKind::Adjective),
        (StarName::GuChen, StarKind::Adjective),
        (StarName::GuaSu, StarKind::Adjective),
        (StarName::FeiLian, StarKind::Adjective),
        (StarName::PoSui, StarKind::Adjective),
        (StarName::TianDe, StarKind::Adjective),
        (StarName::YueDe, StarKind::Adjective),
        (StarName::NianJie, StarKind::Helper),
        (StarName::XianChi, StarKind::Flower),
        (StarName::TianKong, StarKind::Adjective),
        (StarName::TianGuan, StarKind::Adjective),
        (StarName::TianChu, StarKind::Adjective),
        (StarName::TianFuAdj, StarKind::Adjective),
        (StarName::TianCai, StarKind::Adjective),
        (StarName::TianShou, StarKind::Adjective),
        (StarName::TianShang, StarKind::Adjective),
        (StarName::TianShi, StarKind::Adjective),
        (StarName::JieLu, StarKind::Adjective),
        (StarName::KongWang, StarKind::Adjective),
        (StarName::XunKong, StarKind::Adjective),
        (StarName::LongDeAdj, StarKind::Adjective),
        (StarName::JieKong, StarKind::Adjective),
        (StarName::JieShaAdj, StarKind::Adjective),
        (StarName::DaHaoAdj, StarKind::Adjective),
    ]);

    for star in ALL_ADJECTIVE_STARS {
        let metadata = adjective_star_metadata(star);

        assert_eq!(metadata.name(), star);
        assert_eq!(metadata.key(), star_key(star));
        assert_eq!(metadata.kind(), expected_kind[&star]);
        assert_eq!(metadata.category(), StarCategory::Adjective);
        assert_eq!(star_metadata(star), metadata);
        assert!(!metadata.chinese_name().is_empty());
    }
}

#[test]
fn star_kind_flower_and_adjective_map_to_adjective_category() {
    // Protects the generic query helpers: every fine kind in this subset
    // (incl. 解神's Helper kind) must derive the coarse Adjective grouping.
    assert_eq!(StarKind::Flower.category(), StarCategory::Adjective);
    assert_eq!(StarKind::Adjective.category(), StarCategory::Adjective);
    assert_eq!(StarKind::Helper.category(), StarCategory::Adjective);
}

#[test]
fn try_adjective_star_metadata_is_some_for_adjective_and_none_for_others() {
    for star in ALL_ADJECTIVE_STARS {
        let metadata = try_adjective_star_metadata(star).expect("adjective star should resolve");
        assert_eq!(metadata, adjective_star_metadata(star));
    }

    // A major star and a minor star are not represented adjective stars.
    assert!(try_adjective_star_metadata(StarName::ZiWei).is_none());
    assert!(try_adjective_star_metadata(StarName::ZuoFu).is_none());
}

#[test]
fn try_star_metadata_is_total_over_represented_stars() {
    // Guards `star_metadata(..).expect(..)` against an unmapped variant: every
    // currently represented star (incl. the adjective subset) must resolve.
    for entry in represented_star_metadata_table() {
        assert!(
            try_star_metadata(entry.name()).is_some(),
            "try_star_metadata should resolve {:?}",
            entry.name()
        );
    }
}

#[test]
fn selected_adjective_star_names_serialize_to_expected_keys() {
    for star in ALL_ADJECTIVE_STARS {
        let value = serde_json::to_value(star).expect("star name should serialize");
        assert_eq!(value, Value::String(star_key(star).to_owned()));
    }
}

#[test]
fn placer_places_each_selected_adjective_star_exactly_once() {
    let chart = supported_chart_from_fixture(&fixture_value(ADJECTIVE_STARS_1990_FIXTURE));
    let placed: Vec<StarName> = chart
        .stars_by_category(StarCategory::Adjective)
        .into_iter()
        .map(|fact| fact.placement().name())
        .collect();
    let unique: HashSet<StarName> = placed.iter().copied().collect();

    assert_eq!(placed.len(), DEFAULT_ADJECTIVE_STARS.len());
    assert_eq!(unique, HashSet::from(DEFAULT_ADJECTIVE_STARS));
}

#[test]
fn placer_places_each_selected_star_at_expected_branch() {
    // Deterministic placement for the 1990-05-17 chen female case
    // (lunar month 5, birth time 辰, birth year branch 午).
    let chart = supported_chart_from_fixture(&fixture_value(ADJECTIVE_STARS_1990_FIXTURE));
    let expected = HashMap::from([
        (StarName::HongLuan, EarthlyBranch::You),
        (StarName::TianXi, EarthlyBranch::Mao),
        (StarName::TianYao, EarthlyBranch::Si),
        (StarName::TianXing, EarthlyBranch::Chou),
        (StarName::TaiFu, EarthlyBranch::Xu),
        (StarName::FengGao, EarthlyBranch::Wu),
        (StarName::SanTai, EarthlyBranch::Zi),
        (StarName::BaZuo, EarthlyBranch::Yin),
        (StarName::LongChi, EarthlyBranch::Xu),
        (StarName::FengGe, EarthlyBranch::Chen),
        (StarName::TianKu, EarthlyBranch::Zi),
        (StarName::TianXu, EarthlyBranch::Zi),
        (StarName::EnGuang, EarthlyBranch::You),
        (StarName::TianGui, EarthlyBranch::Hai),
        (StarName::TianWu, EarthlyBranch::Si),
        (StarName::TianYueAdj, EarthlyBranch::Wei),
        (StarName::YinSha, EarthlyBranch::Wu),
        (StarName::JieShen, EarthlyBranch::Zi),
        (StarName::HuaGai, EarthlyBranch::Xu),
        (StarName::GuChen, EarthlyBranch::Shen),
        (StarName::GuaSu, EarthlyBranch::Chen),
        (StarName::FeiLian, EarthlyBranch::Yin),
        (StarName::PoSui, EarthlyBranch::Si),
        (StarName::TianDe, EarthlyBranch::Mao),
        (StarName::YueDe, EarthlyBranch::Hai),
        (StarName::NianJie, EarthlyBranch::Chen),
        (StarName::XianChi, EarthlyBranch::Mao),
        (StarName::TianKong, EarthlyBranch::Wei),
        (StarName::TianGuan, EarthlyBranch::Hai),
        (StarName::TianChu, EarthlyBranch::Yin),
        (StarName::TianFuAdj, EarthlyBranch::Wu),
        (StarName::TianCai, EarthlyBranch::Shen),
        (StarName::TianShou, EarthlyBranch::Chen),
        (StarName::TianShang, EarthlyBranch::Wei),
        (StarName::TianShi, EarthlyBranch::You),
        (StarName::JieLu, EarthlyBranch::Wu),
        (StarName::KongWang, EarthlyBranch::Wei),
        (StarName::XunKong, EarthlyBranch::Xu),
    ]);

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

#[test]
fn direct_adjective_placer_matches_iztro_branch_formulas() {
    // Exercise the placer in isolation: place adjective stars directly onto a
    // chart that has major and minor stars, but no adjective stars yet.
    let fixture = fixture_value(ADJECTIVE_STARS_1988_FIXTURE);
    let input = &fixture["input"];
    let minor_chart = minor_chart_from_fixture(&fixture);

    let placed = DeterministicAdjectiveStarPlacer
        .place_adjective_stars(
            minor_chart,
            AdjectiveStarPlacementInput::new(
                LunarMonth::new(input["lunar_month"].as_u64().expect("lunar_month") as u8)
                    .expect("fixture lunar month should be valid"),
                LunarDay::new(input["lunar_day"].as_u64().expect("lunar_day") as u8)
                    .expect("fixture lunar day should be valid"),
                parse_branch_key(input["birth_time"].as_str().expect("birth_time")),
                parse_stem_key(input["birth_year_stem"].as_str().expect("birth_year_stem")),
                parse_branch_key(
                    input["birth_year_branch"]
                        .as_str()
                        .expect("birth_year_branch"),
                ),
            ),
        )
        .expect("adjective stars should place deterministically");

    assert_adjective_stars_match_fixture(&placed, &fixture);
}

#[test]
fn adjective_stars_require_minor_star_anchors() {
    let fixture = fixture_value(ADJECTIVE_STARS_1990_FIXTURE);
    let input = &fixture["input"];
    let major_chart = build_natal_chart_with_major_stars(NatalChartWithMajorStarsInput::new(
        BirthContext::new(
            CalendarDate::lunar(
                input["lunar_year"].as_i64().expect("lunar_year") as i32,
                input["lunar_month"].as_u64().expect("lunar_month") as u8,
                input["lunar_day"].as_u64().expect("lunar_day") as u8,
            ),
            parse_branch_key(input["birth_time"].as_str().expect("birth_time")),
            parse_gender_key(input["gender"].as_str().expect("gender")),
        ),
        MethodProfile::placeholder("adjective_missing_anchor"),
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
    .expect("major-star chart should build");

    let err = DeterministicAdjectiveStarPlacer
        .place_adjective_stars(
            major_chart,
            AdjectiveStarPlacementInput::new(
                LunarMonth::new(input["lunar_month"].as_u64().expect("lunar_month") as u8)
                    .expect("fixture lunar month should be valid"),
                LunarDay::new(input["lunar_day"].as_u64().expect("lunar_day") as u8)
                    .expect("fixture lunar day should be valid"),
                parse_branch_key(input["birth_time"].as_str().expect("birth_time")),
                parse_stem_key(input["birth_year_stem"].as_str().expect("birth_year_stem")),
                parse_branch_key(
                    input["birth_year_branch"]
                        .as_str()
                        .expect("birth_year_branch"),
                ),
            ),
        )
        .expect_err("major-only chart should be missing ZuoFu");

    assert_eq!(
        err,
        ChartError::RequiredStarMissing {
            star: StarName::ZuoFu
        }
    );
}

#[test]
fn san_tai_and_ba_zuo_are_derived_from_placed_minor_star_anchors() {
    let fixture = fixture_value(ADJECTIVE_STARS_1990_FIXTURE);
    let chart = supported_chart_from_fixture(&fixture);
    let day_offset = fixture["input"]["lunar_day"].as_u64().expect("lunar_day") as isize - 1;

    let zuo_fu_branch = chart
        .palace_containing_star(StarName::ZuoFu)
        .expect("ZuoFu should be placed before adjective stars")
        .branch();
    let you_bi_branch = chart
        .palace_containing_star(StarName::YouBi)
        .expect("YouBi should be placed before adjective stars")
        .branch();

    assert_eq!(
        chart
            .palace_containing_star(StarName::SanTai)
            .expect("SanTai should be placed")
            .branch(),
        zuo_fu_branch.offset(day_offset)
    );
    assert_eq!(
        chart
            .palace_containing_star(StarName::BaZuo)
            .expect("BaZuo should be placed")
            .branch(),
        you_bi_branch.offset(-day_offset)
    );
}

#[test]
fn en_guang_and_tian_gui_are_derived_from_placed_chang_qu_anchors() {
    // 恩光/天贵 mirror 三台/八座: they count from the placed 文昌/文曲 by the
    // lunar-day offset minus one (iztro getDailyStarIndex).
    let fixture = fixture_value(ADJECTIVE_STARS_1990_FIXTURE);
    let chart = supported_chart_from_fixture(&fixture);
    let day_offset = fixture["input"]["lunar_day"].as_u64().expect("lunar_day") as isize - 1;

    let wen_chang_branch = chart
        .palace_containing_star(StarName::WenChang)
        .expect("WenChang should be placed before adjective stars")
        .branch();
    let wen_qu_branch = chart
        .palace_containing_star(StarName::WenQu)
        .expect("WenQu should be placed before adjective stars")
        .branch();

    assert_eq!(
        chart
            .palace_containing_star(StarName::EnGuang)
            .expect("EnGuang should be placed")
            .branch(),
        wen_chang_branch.offset(day_offset - 1)
    );
    assert_eq!(
        chart
            .palace_containing_star(StarName::TianGui)
            .expect("TianGui should be placed")
            .branch(),
        wen_qu_branch.offset(day_offset - 1)
    );
}

#[test]
fn life_and_body_anchored_stars_follow_life_and_body_palaces() {
    // 天才/天寿 count forward from the Life/Body palaces by the birth year
    // branch index; 天伤/天使 occupy the 仆役 (Life+5) and 疾厄 (Life+7) palaces
    // under the default algorithm (no 阴阳 swap).
    let fixture = fixture_value(ADJECTIVE_STARS_1990_FIXTURE);
    let chart = supported_chart_from_fixture(&fixture);
    let year_branch = parse_branch_key(
        fixture["input"]["birth_year_branch"]
            .as_str()
            .expect("birth_year_branch"),
    );
    let year_offset = year_branch.index() as isize;
    let life_branch = chart.life_palace().expect("life palace").branch();
    let body_branch = chart.body_palace_branch().expect("body palace branch");

    assert_eq!(
        chart
            .palace_containing_star(StarName::TianCai)
            .expect("TianCai should be placed")
            .branch(),
        life_branch.offset(year_offset),
    );
    assert_eq!(
        chart
            .palace_containing_star(StarName::TianShou)
            .expect("TianShou should be placed")
            .branch(),
        body_branch.offset(year_offset),
    );
    assert_eq!(
        chart
            .palace_containing_star(StarName::TianShang)
            .expect("TianShang should be placed")
            .branch(),
        life_branch.offset(5),
    );
    assert_eq!(
        chart
            .palace_containing_star(StarName::TianShi)
            .expect("TianShi should be placed")
            .branch(),
        life_branch.offset(7),
    );
}

#[test]
fn tian_fu_adj_disambiguates_from_major_tian_fu() {
    // The adjective 天福 and the major 天府 share the "Tian Fu" romanization but
    // are distinct stars with distinct keys and categories.
    let adj = adjective_star_metadata(StarName::TianFuAdj);
    assert_eq!(adj.key(), "tian_fu_adj");
    assert_eq!(adj.chinese_name(), "天福");
    assert_eq!(adj.category(), StarCategory::Adjective);
    assert!(try_adjective_star_metadata(StarName::TianFu).is_none());

    let major = star_metadata(StarName::TianFu);
    assert_eq!(major.key(), "tian_fu");
    assert_eq!(major.chinese_name(), "天府");
    assert_eq!(major.category(), StarCategory::Major);

    assert_ne!(adj.name(), major.name());
    assert_ne!(adj.key(), major.key());

    // Both appear in the built chart, in their own palaces.
    let chart = supported_chart_from_fixture(&fixture_value(ADJECTIVE_STARS_1990_FIXTURE));
    assert!(chart.palace_containing_star(StarName::TianFuAdj).is_some());
    assert!(chart.palace_containing_star(StarName::TianFu).is_some());
}

#[test]
fn placer_matches_iztro_adjective_fixtures() {
    for fixture in fixture_values() {
        let chart = supported_chart_from_fixture(&fixture);
        assert_adjective_stars_match_fixture(&chart, &fixture);
    }
}

#[test]
fn by_lunar_includes_selected_adjective_stars() {
    let request = LunarChartRequest::builder()
        .lunar_year(1990)
        .lunar_month(LunarMonth::new(5).expect("month 5 should be valid"))
        .lunar_day(LunarDay::new(17).expect("day 17 should be valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .birth_year_stem(HeavenlyStem::Geng)
        .birth_year_branch(EarthlyBranch::Wu)
        .method_profile(MethodProfile::placeholder("adjective_by_lunar"))
        .build()
        .expect("request should build");
    let chart = by_lunar(request).expect("by_lunar should build chart");

    let adjective: HashSet<StarName> = chart
        .stars_by_category(StarCategory::Adjective)
        .into_iter()
        .map(|fact| fact.placement().name())
        .collect();

    assert_eq!(adjective, HashSet::from(DEFAULT_ADJECTIVE_STARS));
    assert_eq!(chart.stars().len(), 66);
}

#[test]
fn generic_star_queries_return_adjective_context() {
    let chart = supported_chart_from_fixture(&fixture_value(ADJECTIVE_STARS_1990_FIXTURE));

    let hong_luan = chart
        .star(StarName::HongLuan)
        .expect("Hong Luan should be placed");
    assert_eq!(hong_luan.palace().branch(), EarthlyBranch::You);
    assert_eq!(hong_luan.placement().kind(), StarKind::Flower);
    assert_eq!(hong_luan.placement().category(), StarCategory::Adjective);
    assert_eq!(hong_luan.placement().brightness(), Brightness::Unknown);
    assert_eq!(hong_luan.placement().mutagen(), None);
    assert_eq!(hong_luan.placement().scope(), Scope::Natal);

    assert_eq!(
        chart
            .palace_containing_star(StarName::SanTai)
            .expect("San Tai palace should be queryable")
            .branch(),
        EarthlyBranch::Zi
    );

    // Query helpers expose both multi-star branch collisions and named-palace
    // adjective stars.
    let chou_palace = chart
        .palaces()
        .iter()
        .find(|palace| palace.branch() == EarthlyBranch::Chou)
        .expect("Chou palace should exist");
    let by_branch: HashSet<StarName> = chart
        .stars_in_branch(EarthlyBranch::Zi)
        .iter()
        .map(|star| star.placement().name())
        .collect();
    let by_palace: HashSet<StarName> = chart
        .stars_in_palace(chou_palace.name())
        .iter()
        .filter(|star| star.placement().category() == StarCategory::Adjective)
        .map(|star| star.placement().name())
        .collect();
    assert!(by_branch.contains(&StarName::SanTai));
    assert!(by_branch.contains(&StarName::TianKu));
    assert!(by_branch.contains(&StarName::TianXu));
    assert_eq!(by_palace, HashSet::from([StarName::TianXing]));

    assert_eq!(
        chart.stars_by_category(StarCategory::Adjective).len(),
        DEFAULT_ADJECTIVE_STARS.len()
    );
    assert_eq!(chart.stars_by_kind(StarKind::Flower).len(), 4);
    assert_eq!(chart.stars_by_kind(StarKind::Adjective).len(), 32);
    assert_eq!(chart.stars_by_kind(StarKind::Helper).len(), 2);
}

#[test]
fn chart_with_adjective_stars_round_trips_through_json() {
    let fixture = fixture_value(ADJECTIVE_STARS_1991_FIXTURE);
    let chart = supported_chart_from_fixture(&fixture);
    let serialized = serde_json::to_string(&chart).expect("chart should serialize");
    let decoded: Chart = serde_json::from_str(&serialized).expect("chart should deserialize");

    assert_adjective_stars_match_fixture(&decoded, &fixture);
    assert_eq!(
        decoded
            .star(StarName::LongChi)
            .expect("Long Chi should remain queryable")
            .placement()
            .kind(),
        StarKind::Adjective
    );
}

fn minor_chart_from_fixture(fixture: &Value) -> Chart {
    let input = &fixture["input"];
    let major_chart = build_natal_chart_with_major_stars(NatalChartWithMajorStarsInput::new(
        BirthContext::new(
            CalendarDate::lunar(
                input["lunar_year"].as_i64().expect("lunar_year") as i32,
                input["lunar_month"].as_u64().expect("lunar_month") as u8,
                input["lunar_day"].as_u64().expect("lunar_day") as u8,
            ),
            parse_branch_key(input["birth_time"].as_str().expect("birth_time")),
            parse_gender_key(input["gender"].as_str().expect("gender")),
        ),
        MethodProfile::placeholder("adjective_direct"),
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
    .expect("major-star chart should build");

    DeterministicMinorStarPlacer
        .place_minor_stars(
            major_chart,
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
        .expect("minor-star chart should build")
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
        MethodProfile::placeholder("adjective_star_fixture"),
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

fn assert_adjective_stars_match_fixture(chart: &Chart, fixture: &Value) {
    assert_eq!(
        fixture["metadata"]["target_version"].as_str(),
        Some("2.5.8")
    );

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
            assert_eq!(
                got.category(),
                StarCategory::Adjective,
                "category mismatch for {name:?} in {branch:?}"
            );
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

fn fixture_values() -> Vec<Value> {
    [
        ADJECTIVE_STARS_1990_FIXTURE,
        ADJECTIVE_STARS_1988_FIXTURE,
        ADJECTIVE_STARS_1991_FIXTURE,
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

fn star_key(star: StarName) -> &'static str {
    match star {
        StarName::HongLuan => "hong_luan",
        StarName::TianXi => "tian_xi",
        StarName::TianYao => "tian_yao",
        StarName::TianXing => "tian_xing",
        StarName::TaiFu => "tai_fu",
        StarName::FengGao => "feng_gao",
        StarName::SanTai => "san_tai",
        StarName::BaZuo => "ba_zuo",
        StarName::LongChi => "long_chi",
        StarName::FengGe => "feng_ge",
        StarName::TianKu => "tian_ku",
        StarName::TianXu => "tian_xu",
        StarName::EnGuang => "en_guang",
        StarName::TianGui => "tian_gui",
        StarName::TianWu => "tian_wu",
        StarName::TianYueAdj => "tian_yue_adj",
        StarName::YinSha => "yin_sha",
        StarName::JieShen => "jie_shen",
        StarName::HuaGai => "hua_gai",
        StarName::GuChen => "gu_chen",
        StarName::GuaSu => "gua_su",
        StarName::FeiLian => "fei_lian",
        StarName::PoSui => "po_sui",
        StarName::TianDe => "tian_de",
        StarName::YueDe => "yue_de",
        StarName::NianJie => "nian_jie",
        StarName::XianChi => "xian_chi",
        StarName::TianKong => "tian_kong",
        StarName::TianGuan => "tian_guan",
        StarName::TianChu => "tian_chu",
        StarName::TianFuAdj => "tian_fu_adj",
        StarName::TianCai => "tian_cai",
        StarName::TianShou => "tian_shou",
        StarName::TianShang => "tian_shang",
        StarName::TianShi => "tian_shi",
        StarName::JieLu => "jie_lu",
        StarName::KongWang => "kong_wang",
        StarName::XunKong => "xun_kong",
        StarName::LongDeAdj => "long_de_adj",
        StarName::JieKong => "jie_kong",
        StarName::JieShaAdj => "jie_sha_adj",
        StarName::DaHaoAdj => "da_hao_adj",
        other => panic!("unsupported adjective star: {other:?}"),
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
    EARTHLY_BRANCHES
        .into_iter()
        .find(|branch| branch_key(*branch) == value)
        .unwrap_or_else(|| panic!("unsupported branch key in fixture: {value}"))
}

fn branch_key(branch: EarthlyBranch) -> &'static str {
    match branch {
        EarthlyBranch::Zi => "zi",
        EarthlyBranch::Chou => "chou",
        EarthlyBranch::Yin => "yin",
        EarthlyBranch::Mao => "mao",
        EarthlyBranch::Chen => "chen",
        EarthlyBranch::Si => "si",
        EarthlyBranch::Wu => "wu",
        EarthlyBranch::Wei => "wei",
        EarthlyBranch::Shen => "shen",
        EarthlyBranch::You => "you",
        EarthlyBranch::Xu => "xu",
        EarthlyBranch::Hai => "hai",
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
