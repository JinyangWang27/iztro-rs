use iztro::core::{
    BirthContext, CalendarDate, Chart, EarthlyBranch, FiveElementBureau, Gender, HeavenlyStem,
    LunarDay, LunarMonth, MethodProfile, NatalChartInput, NatalChartWithMajorStarsInput,
    PALACE_COUNT, PALACE_NAMES, PalaceName, StarCategory, StarName, StemBranch, build_empty_chart,
    build_minimal_natal_chart, build_natal_chart_with_major_stars,
    five_element_bureau_from_life_palace, palace_stem_for_branch,
};

// These are local algorithmic test cases, not the iztro golden fixture; the
// upstream compatibility test lives in `tests/iztro_compatibility.rs`.
const LOCAL_TEST_YEAR_STEM: HeavenlyStem = HeavenlyStem::Geng;
const LOCAL_TEST_YEAR_BRANCH: EarthlyBranch = EarthlyBranch::Wu;
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

#[test]
fn minimal_natal_chart_assigns_life_palace_to_calculated_branch() {
    let chart = build_local_natal_test_chart();

    let life_palace = chart
        .palaces()
        .iter()
        .find(|palace| palace.branch() == EarthlyBranch::Chou)
        .expect("calculated life palace branch should exist");

    assert_eq!(life_palace.name(), PalaceName::Life);
}

#[test]
fn minimal_natal_chart_names_proceed_from_life_palace_branch() {
    let chart = build_local_natal_test_chart();

    for palace in chart.palaces() {
        let offset =
            (EarthlyBranch::Chou.index() + PALACE_COUNT - palace.branch().index()) % PALACE_COUNT;

        assert_eq!(palace.name(), PALACE_NAMES[offset]);
    }
}

#[test]
fn minimal_natal_chart_preserves_metadata_and_empty_stars() {
    let birth_context = BirthContext::new(
        CalendarDate::solar(1990, 5, 17),
        EarthlyBranch::Chou,
        Gender::Female,
    );
    let method_profile = MethodProfile::placeholder("minimal_natal_profile");

    let chart = build_minimal_natal_chart(NatalChartInput::new(
        birth_context.clone(),
        method_profile.clone(),
        LunarMonth::new(1).expect("month 1 should be valid"),
        LOCAL_TEST_YEAR_STEM,
        LOCAL_TEST_YEAR_BRANCH,
    ))
    .expect("minimal natal chart should build");

    assert_eq!(chart.birth_context(), &birth_context);
    assert_eq!(chart.method_profile(), &method_profile);
    assert_eq!(chart.palaces().len(), PALACE_COUNT);
    assert!(
        chart
            .palaces()
            .iter()
            .all(|palace| palace.stars().is_empty())
    );
}

#[test]
fn minimal_natal_chart_still_returns_empty_star_lists() {
    let chart = build_local_natal_test_chart();

    assert!(
        chart
            .palaces()
            .iter()
            .all(|palace| palace.stars().is_empty())
    );
}

#[test]
fn empty_chart_has_no_five_element_bureau() {
    let chart = build_empty_chart(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chou,
            Gender::Female,
        ),
        StemBranch::try_new(LOCAL_TEST_YEAR_STEM, LOCAL_TEST_YEAR_BRANCH)
            .expect("valid sexagenary pair"),
        MethodProfile::placeholder("empty_chart_profile"),
    )
    .expect("empty chart should build");

    assert_eq!(chart.five_element_bureau(), None);
    assert_eq!(chart.body_palace_branch(), None);
}

#[test]
fn minimal_natal_chart_has_five_element_bureau() {
    let chart = build_local_natal_test_chart();

    assert_eq!(chart.five_element_bureau(), Some(FiveElementBureau::Fire6));
}

#[test]
fn minimal_natal_chart_uses_real_palace_stems_from_year_stem() {
    let chart = build_local_natal_test_chart();

    let yin_palace = chart
        .palaces()
        .iter()
        .find(|palace| palace.branch() == EarthlyBranch::Yin)
        .expect("chart should contain the Yin palace");

    assert_eq!(
        yin_palace.stem(),
        palace_stem_for_branch(LOCAL_TEST_YEAR_STEM, EarthlyBranch::Yin)
    );
    // 起五行寅例 for a Geng year places Wu at Yin, not the placeholder stem.
    assert_eq!(yin_palace.stem(), HeavenlyStem::Wu);
}

#[test]
fn minimal_natal_chart_life_palace_pair_drives_bureau() {
    let chart = build_local_natal_test_chart();

    let life_palace = chart
        .life_palace()
        .expect("chart should expose a Life Palace");
    assert_eq!(life_palace.branch(), EarthlyBranch::Chou);
    assert_eq!(life_palace.stem(), HeavenlyStem::Ji);

    let expected = five_element_bureau_from_life_palace(
        StemBranch::try_new(life_palace.stem(), life_palace.branch())
            .expect("life palace pair should be valid"),
    );
    assert_eq!(chart.five_element_bureau(), Some(expected));
}

#[test]
fn minimal_natal_chart_round_trips_through_json() {
    let chart = build_local_natal_test_chart();

    let encoded = serde_json::to_string(&chart).expect("minimal natal chart should serialize");
    let decoded: Chart =
        serde_json::from_str(&encoded).expect("minimal natal chart should deserialize");

    assert_eq!(decoded, chart);
    assert_eq!(decoded.five_element_bureau(), chart.five_element_bureau());
    assert_eq!(decoded.body_palace_branch(), chart.body_palace_branch());
}

#[test]
fn natal_chart_with_major_stars_places_each_major_star_exactly_once() {
    let chart = build_local_major_star_test_chart();
    let placed: Vec<StarName> = chart
        .palaces()
        .iter()
        .flat_map(|palace| palace.stars())
        .filter(|star| star.category() == StarCategory::Major)
        .map(|star| star.name())
        .collect();

    assert_eq!(placed.len(), ALL_MAJOR_STARS.len());

    for star in ALL_MAJOR_STARS {
        assert_eq!(
            placed
                .iter()
                .filter(|&&placed_star| placed_star == star)
                .count(),
            1,
            "{star:?} should be placed exactly once"
        );
    }
}

#[test]
fn natal_chart_with_major_stars_preserves_minimal_chart_facts() {
    let minimal = build_local_natal_test_chart();
    let with_major_stars = build_local_major_star_test_chart();

    assert_eq!(with_major_stars.birth_context(), minimal.birth_context());
    assert_eq!(with_major_stars.method_profile(), minimal.method_profile());
    assert_eq!(
        with_major_stars.body_palace_branch(),
        minimal.body_palace_branch()
    );
    assert_eq!(
        with_major_stars.five_element_bureau(),
        minimal.five_element_bureau()
    );

    for (actual, expected) in with_major_stars
        .palaces()
        .iter()
        .zip(minimal.palaces().iter())
    {
        assert_eq!(actual.name(), expected.name());
        assert_eq!(actual.branch(), expected.branch());
        assert_eq!(actual.stem(), expected.stem());
    }
}

#[test]
fn natal_chart_with_major_stars_round_trips_through_json_with_major_stars() {
    let chart = build_local_major_star_test_chart();

    let encoded =
        serde_json::to_string(&chart).expect("natal chart with major stars should serialize");
    let decoded: Chart =
        serde_json::from_str(&encoded).expect("natal chart with major stars should deserialize");

    assert_eq!(decoded, chart);
    assert_eq!(
        collect_major_star_names(&decoded),
        collect_major_star_names(&chart)
    );
}

fn build_local_natal_test_chart() -> Chart {
    build_minimal_natal_chart(NatalChartInput::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chou,
            Gender::Female,
        ),
        MethodProfile::placeholder("minimal_natal_profile"),
        LunarMonth::new(1).expect("month 1 should be valid"),
        LOCAL_TEST_YEAR_STEM,
        LOCAL_TEST_YEAR_BRANCH,
    ))
    .expect("minimal natal chart should build")
}

fn build_local_major_star_test_chart() -> Chart {
    build_natal_chart_with_major_stars(NatalChartWithMajorStarsInput::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chou,
            Gender::Female,
        ),
        MethodProfile::placeholder("minimal_natal_profile"),
        LunarMonth::new(1).expect("month 1 should be valid"),
        LunarDay::new(23).expect("day 23 should be valid"),
        LOCAL_TEST_YEAR_STEM,
        LOCAL_TEST_YEAR_BRANCH,
    ))
    .expect("natal chart with major stars should build")
}

fn collect_major_star_names(chart: &Chart) -> Vec<StarName> {
    chart
        .palaces()
        .iter()
        .flat_map(|palace| palace.stars())
        .filter(|star| star.category() == StarCategory::Major)
        .map(|star| star.name())
        .collect()
}
