use iztro_core::{
    BirthContext, CalendarDate, Chart, EarthlyBranch, FiveElementBureau, Gender, HeavenlyStem,
    LunarMonth, MethodProfile, NatalChartInput, PALACE_COUNT, PALACE_NAMES, PalaceName, StemBranch,
    build_empty_chart, build_minimal_natal_chart, five_element_bureau_from_life_palace,
    palace_stem_for_branch,
};

const FIXTURE_YEAR_STEM: HeavenlyStem = HeavenlyStem::Geng;

#[test]
fn minimal_natal_chart_assigns_life_palace_to_calculated_branch() {
    let chart = build_fixture_chart();

    let life_palace = chart
        .palaces()
        .iter()
        .find(|palace| palace.branch() == EarthlyBranch::Chou)
        .expect("calculated life palace branch should exist");

    assert_eq!(life_palace.name(), PalaceName::Life);
}

#[test]
fn minimal_natal_chart_names_proceed_from_life_palace_branch() {
    let chart = build_fixture_chart();

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
        FIXTURE_YEAR_STEM,
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
fn empty_chart_has_no_five_element_bureau() {
    let chart = build_empty_chart(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chou,
            Gender::Female,
        ),
        MethodProfile::placeholder("empty_chart_profile"),
    )
    .expect("empty chart should build");

    assert_eq!(chart.five_element_bureau(), None);
    assert_eq!(chart.body_palace_branch(), None);
}

#[test]
fn minimal_natal_chart_has_five_element_bureau() {
    let chart = build_fixture_chart();

    assert_eq!(chart.five_element_bureau(), Some(FiveElementBureau::Fire6));
}

#[test]
fn minimal_natal_chart_uses_real_palace_stems_from_year_stem() {
    let chart = build_fixture_chart();

    let yin_palace = chart
        .palaces()
        .iter()
        .find(|palace| palace.branch() == EarthlyBranch::Yin)
        .expect("chart should contain the Yin palace");

    assert_eq!(
        yin_palace.stem(),
        palace_stem_for_branch(FIXTURE_YEAR_STEM, EarthlyBranch::Yin)
    );
    // 起五行寅例 for a Geng year places Wu at Yin, not the placeholder stem.
    assert_eq!(yin_palace.stem(), HeavenlyStem::Wu);
}

#[test]
fn minimal_natal_chart_life_palace_pair_drives_bureau() {
    let chart = build_fixture_chart();

    let life_palace = chart
        .life_palace()
        .expect("chart should expose a Life Palace");
    assert_eq!(life_palace.branch(), EarthlyBranch::Chou);
    assert_eq!(life_palace.stem(), HeavenlyStem::Ji);

    let expected = five_element_bureau_from_life_palace(StemBranch::new(
        life_palace.stem(),
        life_palace.branch(),
    ))
    .expect("life palace pair should be valid");
    assert_eq!(chart.five_element_bureau(), Some(expected));
}

#[test]
fn minimal_natal_chart_round_trips_through_json() {
    let chart = build_fixture_chart();

    let encoded = serde_json::to_string(&chart).expect("minimal natal chart should serialize");
    let decoded: Chart =
        serde_json::from_str(&encoded).expect("minimal natal chart should deserialize");

    assert_eq!(decoded, chart);
    assert_eq!(decoded.five_element_bureau(), chart.five_element_bureau());
    assert_eq!(decoded.body_palace_branch(), chart.body_palace_branch());
}

fn build_fixture_chart() -> Chart {
    build_minimal_natal_chart(NatalChartInput::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chou,
            Gender::Female,
        ),
        MethodProfile::placeholder("minimal_natal_profile"),
        LunarMonth::new(1).expect("month 1 should be valid"),
        FIXTURE_YEAR_STEM,
    ))
    .expect("minimal natal chart should build")
}
