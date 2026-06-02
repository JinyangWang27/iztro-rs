use std::collections::{HashMap, HashSet};

use iztro_core::{
    BirthContext, Brightness, CalendarDate, Chart, DeterministicMajorStarPlacer, EarthlyBranch,
    FiveElementBureau, Gender, HeavenlyStem, LunarDay, LunarMonth, MajorStarPlacementInput,
    MajorStarPlacer, MethodProfile, NatalChartInput, PALACE_COUNT, Scope, StarCategory, StarName,
    build_minimal_natal_chart, tian_fu_branch, zi_wei_branch,
};
use serde_json::Value;

const MAJOR_STARS_FIXTURE: &str =
    include_str!("../../../fixtures/iztro/major_stars_1990_05_17_chen_female.json");

/// The fourteen major stars (主星) in canonical order.
const ALL_MAJOR_STARS: [StarName; 14] = [
    StarName::ZiWei,
    StarName::TianJi,
    StarName::TaiYang,
    StarName::WuQu,
    StarName::TianTong,
    StarName::LianZhen,
    StarName::TianFu,
    StarName::TaiYin,
    StarName::TanLang,
    StarName::JuMen,
    StarName::TianXiang,
    StarName::TianLiang,
    StarName::QiSha,
    StarName::PoJun,
];

/// Expected fourteen-major-star layout for the iztro fixture
/// (1990-05-17 chen female, lunar 四月廿三, fire6 bureau).
const EXPECTED_PLACEMENT: &[(EarthlyBranch, &[StarName])] = &[
    (EarthlyBranch::Yin, &[StarName::TanLang]),
    (EarthlyBranch::Mao, &[StarName::TianJi, StarName::JuMen]),
    (EarthlyBranch::Chen, &[StarName::ZiWei, StarName::TianXiang]),
    (EarthlyBranch::Si, &[StarName::TianLiang]),
    (EarthlyBranch::Wu, &[StarName::QiSha]),
    (EarthlyBranch::Wei, &[]),
    (EarthlyBranch::Shen, &[StarName::LianZhen]),
    (EarthlyBranch::You, &[]),
    (EarthlyBranch::Xu, &[StarName::PoJun]),
    (EarthlyBranch::Hai, &[StarName::TianTong]),
    (EarthlyBranch::Zi, &[StarName::WuQu, StarName::TianFu]),
    (EarthlyBranch::Chou, &[StarName::TaiYang, StarName::TaiYin]),
];

#[test]
fn zi_wei_branch_matches_iztro_anchors() {
    // The three worked examples from iztro's 起紫微星诀 docstring.
    assert_eq!(
        zi_wei_branch(FiveElementBureau::Wood3, lunar_day(27)),
        EarthlyBranch::Xu
    );
    assert_eq!(
        zi_wei_branch(FiveElementBureau::Fire6, lunar_day(13)),
        EarthlyBranch::Hai
    );
    assert_eq!(
        zi_wei_branch(FiveElementBureau::Earth5, lunar_day(6)),
        EarthlyBranch::Wei
    );
    // The fixture case: fire6 bureau, lunar day 23.
    assert_eq!(
        zi_wei_branch(FiveElementBureau::Fire6, lunar_day(23)),
        EarthlyBranch::Chen
    );
}

#[test]
fn tian_fu_branch_reflects_zi_wei_across_yin_shen_axis() {
    assert_eq!(tian_fu_branch(EarthlyBranch::Chen), EarthlyBranch::Zi);
    assert_eq!(tian_fu_branch(EarthlyBranch::Hai), EarthlyBranch::Si);
    // The Yin and Shen palaces lie on the reflection axis and map to themselves.
    assert_eq!(tian_fu_branch(EarthlyBranch::Yin), EarthlyBranch::Yin);
    assert_eq!(tian_fu_branch(EarthlyBranch::Shen), EarthlyBranch::Shen);
}

#[test]
fn placer_places_each_of_the_fourteen_major_stars_exactly_once() {
    let chart = place_fixture_major_stars();

    let placed: HashSet<StarName> = collect_major_stars(&chart)
        .into_iter()
        .map(|(_, star)| star)
        .collect();

    assert_eq!(placed, HashSet::from(ALL_MAJOR_STARS));
}

#[test]
fn placer_does_not_duplicate_any_major_star() {
    let chart = place_fixture_major_stars();
    let placed = collect_major_stars(&chart);

    let unique: HashSet<StarName> = placed.iter().map(|(_, star)| *star).collect();

    assert_eq!(placed.len(), 14);
    assert_eq!(unique.len(), placed.len());
}

#[test]
fn placer_places_stars_at_expected_branches() {
    let chart = place_fixture_major_stars();

    let mut actual: HashMap<EarthlyBranch, HashSet<StarName>> = HashMap::new();
    for (branch, star) in collect_major_stars(&chart) {
        actual.entry(branch).or_default().insert(star);
    }

    for (branch, stars) in EXPECTED_PLACEMENT {
        let expected: HashSet<StarName> = stars.iter().copied().collect();
        let got = actual.get(branch).cloned().unwrap_or_default();
        assert_eq!(got, expected, "unexpected major stars in {branch:?}");
    }
}

#[test]
fn placer_marks_each_major_star_as_natal_unknown_and_unmutated() {
    let chart = place_fixture_major_stars();

    let mut count = 0;
    for palace in chart.palaces() {
        for star in palace.stars() {
            count += 1;
            assert_eq!(star.category(), StarCategory::Major);
            assert_eq!(star.brightness(), Brightness::Unknown);
            assert_eq!(star.mutagen(), None);
            assert_eq!(star.scope(), Scope::Natal);
        }
    }

    assert_eq!(count, 14);
}

#[test]
fn placer_preserves_palace_count_and_chart_metadata() {
    let chart = build_fixture_chart();
    let birth_context = chart.birth_context().clone();
    let method_profile = chart.method_profile().clone();
    let body_branch = chart.body_palace_branch();
    let bureau = chart.five_element_bureau();

    let placed = DeterministicMajorStarPlacer
        .place_major_stars(
            chart,
            MajorStarPlacementInput::new(lunar_day(23), bureau.unwrap()),
        )
        .expect("deterministic major star placement should not fail");

    assert_eq!(placed.palaces().len(), PALACE_COUNT);
    assert_eq!(placed.birth_context(), &birth_context);
    assert_eq!(placed.method_profile(), &method_profile);
    assert_eq!(placed.body_palace_branch(), body_branch);
    assert_eq!(placed.five_element_bureau(), bureau);
}

#[test]
fn fourteen_major_stars_match_iztro_fixture() {
    let fixture: Value =
        serde_json::from_str(MAJOR_STARS_FIXTURE).expect("fixture should be valid JSON");
    assert_eq!(
        fixture["metadata"]["target_version"].as_str(),
        Some("2.5.8")
    );

    let input = &fixture["input"];
    let day = lunar_day(
        input["lunar_day"]
            .as_u64()
            .expect("fixture should include lunar_day") as u8,
    );
    let chart = build_minimal_natal_chart(NatalChartInput::new(
        BirthContext::new(
            parse_solar_date(input["solar_date"].as_str().expect("solar_date")),
            parse_branch_key(input["birth_time"].as_str().expect("birth_time")),
            parse_gender_key(input["gender"].as_str().expect("gender")),
        ),
        MethodProfile::placeholder("iztro_major_stars_fixture"),
        LunarMonth::new(input["lunar_month"].as_u64().expect("lunar_month") as u8)
            .expect("fixture lunar month should be valid"),
        parse_stem_key(input["birth_year_stem"].as_str().expect("birth_year_stem")),
    ))
    .expect("minimal natal chart should build for fixture input");
    let bureau = chart
        .five_element_bureau()
        .expect("fixture chart should resolve a five-element bureau");

    let placed = DeterministicMajorStarPlacer
        .place_major_stars(chart, MajorStarPlacementInput::new(day, bureau))
        .expect("deterministic major star placement should not fail");

    let mut actual: HashMap<EarthlyBranch, HashSet<&str>> = HashMap::new();
    for palace in placed.palaces() {
        let entry = actual.entry(palace.branch()).or_default();
        for star in palace.stars() {
            if star.category() == StarCategory::Major {
                entry.insert(star_key(star.name()));
            }
        }
    }

    let expected_palaces = fixture["supported_fields"]["major_stars"]
        .as_array()
        .expect("fixture should include supported major-star fields");
    assert_eq!(expected_palaces.len(), placed.palaces().len());

    for expected_palace in expected_palaces {
        let branch = parse_branch_key(expected_palace["branch"].as_str().expect("branch"));
        let expected_stars: HashSet<&str> = expected_palace["stars"]
            .as_array()
            .expect("stars array")
            .iter()
            .map(|star| star.as_str().expect("star key"))
            .collect();
        let got = actual.get(&branch).cloned().unwrap_or_default();

        assert_eq!(got, expected_stars, "major-star mismatch in {branch:?}");
    }
}

fn place_fixture_major_stars() -> Chart {
    DeterministicMajorStarPlacer
        .place_major_stars(
            build_fixture_chart(),
            MajorStarPlacementInput::new(lunar_day(23), FiveElementBureau::Fire6),
        )
        .expect("deterministic major star placement should not fail")
}

fn build_fixture_chart() -> Chart {
    build_minimal_natal_chart(NatalChartInput::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("major_star_profile"),
        LunarMonth::new(4).expect("month 4 should be valid"),
        HeavenlyStem::Geng,
    ))
    .expect("minimal natal chart should build")
}

fn collect_major_stars(chart: &Chart) -> Vec<(EarthlyBranch, StarName)> {
    let mut out = Vec::new();
    for palace in chart.palaces() {
        for star in palace.stars() {
            if star.category() == StarCategory::Major {
                out.push((palace.branch(), star.name()));
            }
        }
    }
    out
}

fn lunar_day(value: u8) -> LunarDay {
    LunarDay::new(value).expect("lunar day should be valid")
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
    }
}

fn parse_solar_date(value: &str) -> CalendarDate {
    let mut parts = value.split('-');
    let year = parts
        .next()
        .and_then(|part| part.parse::<i32>().ok())
        .unwrap_or_else(|| panic!("unsupported solar_date in fixture: {value}"));
    let month = parts
        .next()
        .and_then(|part| part.parse::<u8>().ok())
        .unwrap_or_else(|| panic!("unsupported solar_date in fixture: {value}"));
    let day = parts
        .next()
        .and_then(|part| part.parse::<u8>().ok())
        .unwrap_or_else(|| panic!("unsupported solar_date in fixture: {value}"));

    CalendarDate::solar(year, month, day)
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
