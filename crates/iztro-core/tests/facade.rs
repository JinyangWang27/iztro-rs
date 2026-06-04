use std::collections::{HashMap, HashSet};

use iztro_core::{
    BirthContext, Brightness, CalendarDate, CalendarKind, Chart, ChartError, EarthlyBranch,
    FiveElementBureau, Gender, HeavenlyStem, LunarChartRequest, LunarDay, LunarMonth,
    MethodProfile, Mutagen, NatalChartWithSupportedStarsInput, PALACE_COUNT, StarCategory,
    StarName, build_natal_chart_with_supported_stars, by_lunar,
};
use serde_json::Value;

const MAJOR_STARS_FIXTURE: &str =
    include_str!("../../../fixtures/iztro/major_stars_1990_05_17_chen_female.json");

#[test]
fn calendar_date_lunar_records_lunar_kind() {
    let date = CalendarDate::lunar(1990, 4, 23);

    assert_eq!(date.kind(), CalendarKind::Lunar);
    assert_eq!(date.year(), 1990);
    assert_eq!(date.month(), 4);
    assert_eq!(date.day(), 23);
}

#[test]
fn by_lunar_builds_major_star_chart() {
    let chart = by_lunar(fixture_request()).expect("by_lunar should build fixture chart");
    let date = chart.birth_context().date();

    assert_eq!(date.kind(), CalendarKind::Lunar);
    assert_eq!(date.year(), 1990);
    assert_eq!(date.month(), 4);
    assert_eq!(date.day(), 23);
    assert_eq!(chart.birth_context().birth_time(), EarthlyBranch::Chen);
    assert_eq!(chart.birth_context().gender(), Gender::Female);
    assert_eq!(chart.palaces().len(), PALACE_COUNT);
    assert_eq!(chart.major_stars().len(), 14);
    assert_eq!(chart.stars_by_category(StarCategory::Minor).len(), 14);
    assert_eq!(chart.stars_by_category(StarCategory::Adjective).len(), 12);
    assert_eq!(chart.stars().len(), 40);
    assert_eq!(chart.five_element_bureau(), Some(FiveElementBureau::Fire6));
}

#[test]
fn by_lunar_matches_existing_typed_builder() {
    let request = fixture_request();
    let facade_chart = by_lunar(request.clone()).expect("by_lunar should build fixture chart");
    let typed_chart =
        build_natal_chart_with_supported_stars(NatalChartWithSupportedStarsInput::new(
            BirthContext::new(
                CalendarDate::lunar(1990, 4, 23),
                EarthlyBranch::Chen,
                Gender::Female,
            ),
            request.method_profile().clone(),
            request.lunar_month(),
            request.lunar_day(),
            request.birth_year_stem(),
            request.birth_year_branch(),
        ))
        .expect("typed builder should build fixture chart");

    assert_eq!(facade_chart, typed_chart);
}

#[test]
fn by_lunar_matches_iztro_major_star_fixture() {
    let fixture: Value =
        serde_json::from_str(MAJOR_STARS_FIXTURE).expect("fixture should be valid JSON");
    let request = request_from_fixture(&fixture);
    let chart = by_lunar(request).expect("by_lunar should build fixture chart");
    let actual = collect_major_star_facts(&chart);

    assert_eq!(
        fixture["metadata"]["target_version"].as_str(),
        Some("2.5.8")
    );

    for expected_palace in fixture["supported_fields"]["major_stars"]
        .as_array()
        .expect("fixture should include supported major-star fields")
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
            "major-star mismatch in {branch:?}"
        );

        for expected_star in expected_palace["stars"].as_array().expect("stars array") {
            let name = parse_star_key(expected_star["name"].as_str().expect("star name"));
            let got = actual
                .get(&(branch, name))
                .unwrap_or_else(|| panic!("missing {name:?} in {branch:?}"));

            assert_eq!(
                got.brightness(),
                parse_brightness_key(expected_star["brightness"].as_str().expect("brightness"))
            );
            assert_eq!(
                got.mutagen(),
                parse_optional_mutagen_key(&expected_star["mutagen"])
            );
        }
    }
}

#[test]
fn by_lunar_preserves_method_profile() {
    let chart = by_lunar(fixture_request()).expect("by_lunar should build fixture chart");

    assert_eq!(chart.method_profile().id(), "iztro_by_lunar_fixture");
}

#[test]
fn lunar_chart_request_builder_builds_and_accessors_return_inputs() {
    let method_profile = MethodProfile::placeholder("accessor_profile");
    let request = LunarChartRequest::builder()
        .lunar_year(1990)
        .lunar_month(LunarMonth::new(4).expect("month 4 should be valid"))
        .lunar_day(LunarDay::new(23).expect("day 23 should be valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .birth_year_stem(HeavenlyStem::Geng)
        .birth_year_branch(EarthlyBranch::Wu)
        .method_profile(method_profile.clone())
        .build()
        .expect("request should build");

    assert_eq!(request.lunar_year(), 1990);
    assert_eq!(request.lunar_month().value(), 4);
    assert_eq!(request.lunar_day().value(), 23);
    assert_eq!(request.birth_time(), EarthlyBranch::Chen);
    assert_eq!(request.gender(), Gender::Female);
    assert_eq!(request.birth_year_stem(), HeavenlyStem::Geng);
    assert_eq!(request.birth_year_branch(), EarthlyBranch::Wu);
    assert_eq!(request.method_profile(), &method_profile);
}

#[test]
fn lunar_chart_request_builder_reports_each_missing_field() {
    // Each builder below omits exactly one required field, so `build` must fail
    // naming that field. A complete builder is exercised by other tests.
    assert_eq!(
        LunarChartRequest::builder()
            .lunar_month(LunarMonth::new(4).expect("month 4 should be valid"))
            .lunar_day(LunarDay::new(23).expect("day 23 should be valid"))
            .birth_time(EarthlyBranch::Chen)
            .gender(Gender::Female)
            .birth_year_stem(HeavenlyStem::Geng)
            .birth_year_branch(EarthlyBranch::Wu)
            .method_profile(MethodProfile::placeholder("missing_field_profile"))
            .build(),
        Err(ChartError::MissingRequiredInput {
            field: "lunar_year"
        })
    );
    assert_eq!(
        LunarChartRequest::builder()
            .lunar_year(1990)
            .lunar_day(LunarDay::new(23).expect("day 23 should be valid"))
            .birth_time(EarthlyBranch::Chen)
            .gender(Gender::Female)
            .birth_year_stem(HeavenlyStem::Geng)
            .birth_year_branch(EarthlyBranch::Wu)
            .method_profile(MethodProfile::placeholder("missing_field_profile"))
            .build(),
        Err(ChartError::MissingRequiredInput {
            field: "lunar_month"
        })
    );
    assert_eq!(
        LunarChartRequest::builder()
            .lunar_year(1990)
            .lunar_month(LunarMonth::new(4).expect("month 4 should be valid"))
            .birth_time(EarthlyBranch::Chen)
            .gender(Gender::Female)
            .birth_year_stem(HeavenlyStem::Geng)
            .birth_year_branch(EarthlyBranch::Wu)
            .method_profile(MethodProfile::placeholder("missing_field_profile"))
            .build(),
        Err(ChartError::MissingRequiredInput { field: "lunar_day" })
    );
    assert_eq!(
        LunarChartRequest::builder()
            .lunar_year(1990)
            .lunar_month(LunarMonth::new(4).expect("month 4 should be valid"))
            .lunar_day(LunarDay::new(23).expect("day 23 should be valid"))
            .gender(Gender::Female)
            .birth_year_stem(HeavenlyStem::Geng)
            .birth_year_branch(EarthlyBranch::Wu)
            .method_profile(MethodProfile::placeholder("missing_field_profile"))
            .build(),
        Err(ChartError::MissingRequiredInput {
            field: "birth_time"
        })
    );
    assert_eq!(
        LunarChartRequest::builder()
            .lunar_year(1990)
            .lunar_month(LunarMonth::new(4).expect("month 4 should be valid"))
            .lunar_day(LunarDay::new(23).expect("day 23 should be valid"))
            .birth_time(EarthlyBranch::Chen)
            .birth_year_stem(HeavenlyStem::Geng)
            .birth_year_branch(EarthlyBranch::Wu)
            .method_profile(MethodProfile::placeholder("missing_field_profile"))
            .build(),
        Err(ChartError::MissingRequiredInput { field: "gender" })
    );
    assert_eq!(
        LunarChartRequest::builder()
            .lunar_year(1990)
            .lunar_month(LunarMonth::new(4).expect("month 4 should be valid"))
            .lunar_day(LunarDay::new(23).expect("day 23 should be valid"))
            .birth_time(EarthlyBranch::Chen)
            .gender(Gender::Female)
            .birth_year_branch(EarthlyBranch::Wu)
            .method_profile(MethodProfile::placeholder("missing_field_profile"))
            .build(),
        Err(ChartError::MissingRequiredInput {
            field: "birth_year_stem"
        })
    );
    assert_eq!(
        LunarChartRequest::builder()
            .lunar_year(1990)
            .lunar_month(LunarMonth::new(4).expect("month 4 should be valid"))
            .lunar_day(LunarDay::new(23).expect("day 23 should be valid"))
            .birth_time(EarthlyBranch::Chen)
            .gender(Gender::Female)
            .birth_year_stem(HeavenlyStem::Geng)
            .method_profile(MethodProfile::placeholder("missing_field_profile"))
            .build(),
        Err(ChartError::MissingRequiredInput {
            field: "birth_year_branch"
        })
    );
    assert_eq!(
        LunarChartRequest::builder()
            .lunar_year(1990)
            .lunar_month(LunarMonth::new(4).expect("month 4 should be valid"))
            .lunar_day(LunarDay::new(23).expect("day 23 should be valid"))
            .birth_time(EarthlyBranch::Chen)
            .gender(Gender::Female)
            .birth_year_stem(HeavenlyStem::Geng)
            .birth_year_branch(EarthlyBranch::Wu)
            .build(),
        Err(ChartError::MissingRequiredInput {
            field: "method_profile"
        })
    );
}

fn fixture_request() -> LunarChartRequest {
    LunarChartRequest::builder()
        .lunar_year(1990)
        .lunar_month(LunarMonth::new(4).expect("month 4 should be valid"))
        .lunar_day(LunarDay::new(23).expect("day 23 should be valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .birth_year_stem(HeavenlyStem::Geng)
        .birth_year_branch(EarthlyBranch::Wu)
        .method_profile(MethodProfile::placeholder("iztro_by_lunar_fixture"))
        .build()
        .expect("fixture request should build")
}

fn request_from_fixture(fixture: &Value) -> LunarChartRequest {
    let input = &fixture["input"];
    let solar_year = input["solar_date"]
        .as_str()
        .expect("fixture should include solar_date")
        .split('-')
        .next()
        .and_then(|year| year.parse::<i32>().ok())
        .expect("fixture solar_date should include year");

    LunarChartRequest::builder()
        .lunar_year(solar_year)
        .lunar_month(
            LunarMonth::new(input["lunar_month"].as_u64().expect("lunar_month") as u8)
                .expect("fixture lunar month should be valid"),
        )
        .lunar_day(
            LunarDay::new(input["lunar_day"].as_u64().expect("lunar_day") as u8)
                .expect("fixture lunar day should be valid"),
        )
        .birth_time(parse_branch_key(
            input["birth_time"].as_str().expect("birth_time"),
        ))
        .gender(parse_gender_key(input["gender"].as_str().expect("gender")))
        .birth_year_stem(parse_stem_key(
            input["birth_year_stem"].as_str().expect("birth_year_stem"),
        ))
        .birth_year_branch(parse_branch_key(
            input["birth_year_branch"]
                .as_str()
                .expect("birth_year_branch"),
        ))
        .method_profile(MethodProfile::placeholder("iztro_by_lunar_fixture"))
        .build()
        .expect("fixture request should build")
}

fn collect_major_star_facts(
    chart: &Chart,
) -> HashMap<(EarthlyBranch, StarName), &iztro_core::StarPlacement> {
    let mut out = HashMap::new();
    for palace in chart.palaces() {
        for star in palace.stars() {
            if star.category() == StarCategory::Major {
                out.insert((palace.branch(), star.name()), star);
            }
        }
    }
    out
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

fn parse_star_key(value: &str) -> StarName {
    match value {
        "zi_wei" => StarName::ZiWei,
        "tian_ji" => StarName::TianJi,
        "tai_yang" => StarName::TaiYang,
        "wu_qu" => StarName::WuQu,
        "tian_tong" => StarName::TianTong,
        "lian_zhen" => StarName::LianZhen,
        "tian_fu" => StarName::TianFu,
        "tai_yin" => StarName::TaiYin,
        "tan_lang" => StarName::TanLang,
        "ju_men" => StarName::JuMen,
        "tian_xiang" => StarName::TianXiang,
        "tian_liang" => StarName::TianLiang,
        "qi_sha" => StarName::QiSha,
        "po_jun" => StarName::PoJun,
        other => panic!("unsupported star key in fixture: {other}"),
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
