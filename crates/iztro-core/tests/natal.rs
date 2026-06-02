use iztro_core::{
    BirthContext, CalendarDate, Chart, EarthlyBranch, Gender, LunarMonth, MethodProfile,
    NatalChartInput, PALACE_COUNT, PALACE_NAMES, PalaceName, build_minimal_natal_chart,
};

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
fn minimal_natal_chart_round_trips_through_json() {
    let chart = build_fixture_chart();

    let encoded = serde_json::to_string(&chart).expect("minimal natal chart should serialize");
    let decoded: Chart =
        serde_json::from_str(&encoded).expect("minimal natal chart should deserialize");

    assert_eq!(decoded, chart);
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
    ))
    .expect("minimal natal chart should build")
}
