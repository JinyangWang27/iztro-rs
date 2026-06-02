use iztro_core::{
    BirthContext, CalendarDate, EarthlyBranch, Gender, LunarMonth, MajorStarPlacementInput,
    MajorStarPlacer, MethodProfile, NatalChartInput, NoopMajorStarPlacer, PALACE_COUNT,
    build_minimal_natal_chart,
};

#[test]
fn noop_major_star_placer_preserves_palace_count() {
    let chart = build_fixture_chart();
    let placer = NoopMajorStarPlacer;

    let placed = placer
        .place_major_stars(chart, MajorStarPlacementInput)
        .expect("no-op major star placement should not fail");

    assert_eq!(placed.palaces().len(), PALACE_COUNT);
}

#[test]
fn noop_major_star_placer_does_not_add_stars() {
    let chart = build_fixture_chart();
    let placer = NoopMajorStarPlacer;

    let placed = placer
        .place_major_stars(chart, MajorStarPlacementInput)
        .expect("no-op major star placement should not fail");

    assert!(
        placed
            .palaces()
            .iter()
            .all(|palace| palace.stars().is_empty())
    );
}

#[test]
fn noop_major_star_placer_preserves_chart_metadata() {
    let birth_context = BirthContext::new(
        CalendarDate::solar(1990, 5, 17),
        EarthlyBranch::Chen,
        Gender::Female,
    );
    let method_profile = MethodProfile::placeholder("noop_major_star_profile");
    let chart = build_minimal_natal_chart(NatalChartInput::new(
        birth_context.clone(),
        method_profile.clone(),
        LunarMonth::new(5).expect("month 5 should be valid"),
    ))
    .expect("minimal natal chart should build");
    let placer = NoopMajorStarPlacer;

    let placed = placer
        .place_major_stars(chart, MajorStarPlacementInput)
        .expect("no-op major star placement should not fail");

    assert_eq!(placed.birth_context(), &birth_context);
    assert_eq!(placed.method_profile(), &method_profile);
}

fn build_fixture_chart() -> iztro_core::Chart {
    build_minimal_natal_chart(NatalChartInput::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("noop_major_star_profile"),
        LunarMonth::new(5).expect("month 5 should be valid"),
    ))
    .expect("minimal natal chart should build")
}
