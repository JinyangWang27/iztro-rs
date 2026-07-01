//! Effective temporal chart-state tests.
//!
//! The effective state is the selected-view fact model used by analysis: branch
//! coordinates stay stable, palace names come from the selected frame, and star
//! / mutagen facts retain their source scope.

use iztro::{
    BirthContext, Brightness, CalendarDate, Chart, ChartError, EarthlyBranch, Gender, HeavenlyStem,
    HoroscopeChart, MethodProfile, Mutagen, MutagenActivation, PALACE_NAMES, Palace, PalaceName,
    Scope, ScopedStarPlacement, StarKind, StarName, StarPlacement, StemBranch, TemporalContext,
    TemporalLayer, TemporalPalaceLayout, TemporalPalaceName,
};

fn build_chart(life_branch: EarthlyBranch, placements: &[(EarthlyBranch, StarName)]) -> Chart {
    let palaces: Vec<Palace> = (0..12)
        .map(|index| {
            let name = PALACE_NAMES[index];
            let branch = life_branch.offset(index as isize);
            let stars: Vec<StarPlacement> = placements
                .iter()
                .filter(|(placement_branch, _)| *placement_branch == branch)
                .map(|(_, star)| {
                    StarPlacement::new(
                        *star,
                        StarKind::Major,
                        Brightness::Unknown,
                        None,
                        Scope::Natal,
                    )
                })
                .collect();
            Palace::new(name, branch, HeavenlyStem::Jia, stars)
        })
        .collect();

    Chart::try_new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu).expect("valid stem-branch"),
        MethodProfile::placeholder("effective_state_test"),
        palaces,
        None,
        None,
    )
    .expect("synthetic chart should build")
}

fn temporal_context(scope: Scope) -> TemporalContext {
    let stem_branch =
        StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Zi).expect("valid stem-branch");
    match scope {
        Scope::Decadal => TemporalContext::Decadal {
            stem_branch,
            start_age: 34,
        },
        Scope::Yearly => TemporalContext::Yearly {
            stem_branch,
            lunar_year: 2026,
        },
        Scope::Monthly => TemporalContext::Monthly {
            stem_branch,
            lunar_month: 5,
        },
        Scope::Daily => TemporalContext::Daily {
            stem_branch,
            lunar_day: 17,
        },
        Scope::Hourly => TemporalContext::Hourly { stem_branch },
        Scope::Age => TemporalContext::Age {
            stem_branch,
            nominal_age: 37,
        },
        Scope::Natal => panic!("temporal context cannot be natal"),
    }
}

fn temporal_palace_layout(scope: Scope, life_branch: EarthlyBranch) -> TemporalPalaceLayout {
    let names = PALACE_NAMES
        .iter()
        .enumerate()
        .map(|(index, name)| TemporalPalaceName::new(life_branch.offset(index as isize), *name))
        .collect();
    TemporalPalaceLayout::try_new(scope, names).expect("valid temporal palace layout")
}

fn scoped(branch: EarthlyBranch, star: StarName, scope: Scope) -> ScopedStarPlacement {
    ScopedStarPlacement::new(
        branch,
        StarPlacement::new(star, StarKind::Soft, Brightness::Unknown, None, scope),
    )
}

fn temporal_layer(
    scope: Scope,
    life_branch: EarthlyBranch,
    placements: Vec<ScopedStarPlacement>,
    activations: Vec<MutagenActivation>,
    with_layout: bool,
) -> TemporalLayer {
    TemporalLayer::try_new_with_palace_layout(
        scope,
        temporal_context(scope),
        placements,
        activations,
        with_layout.then(|| temporal_palace_layout(scope, life_branch)),
    )
    .expect("valid temporal layer")
}

#[test]
fn constructor_rejects_invalid_effective_scope_inputs() {
    let natal = build_chart(EarthlyBranch::Zi, &[]);
    let yearly = temporal_layer(
        Scope::Yearly,
        EarthlyBranch::Wu,
        Vec::new(),
        Vec::new(),
        true,
    );
    let horoscope = HoroscopeChart::with_layers(natal.clone(), vec![yearly.clone()]);

    assert_eq!(
        iztro::EffectiveChartState::from_horoscope(&horoscope, Scope::Yearly, vec![Scope::Yearly],),
        Err(ChartError::EffectiveChartStateMissingNatalScope)
    );
    assert_eq!(
        iztro::EffectiveChartState::from_horoscope(
            &horoscope,
            Scope::Yearly,
            vec![Scope::Natal, Scope::Yearly, Scope::Yearly],
        ),
        Err(ChartError::DuplicateEffectiveChartStateScope {
            scope: Scope::Yearly,
        })
    );
    assert_eq!(
        iztro::EffectiveChartState::from_horoscope(
            &horoscope,
            Scope::Decadal,
            vec![Scope::Natal, Scope::Decadal],
        ),
        Err(ChartError::MissingHoroscopeLayer {
            scope: Scope::Decadal,
        })
    );

    let duplicate = HoroscopeChart::with_layers(natal.clone(), vec![yearly.clone(), yearly]);
    assert_eq!(
        iztro::EffectiveChartState::from_horoscope(
            &duplicate,
            Scope::Yearly,
            vec![Scope::Natal, Scope::Yearly],
        ),
        Err(ChartError::DuplicateHoroscopeLayer {
            scope: Scope::Yearly,
        })
    );

    let inactive_frame = HoroscopeChart::with_layers(
        natal.clone(),
        vec![temporal_layer(
            Scope::Yearly,
            EarthlyBranch::Wu,
            Vec::new(),
            Vec::new(),
            true,
        )],
    );
    assert_eq!(
        iztro::EffectiveChartState::from_horoscope(
            &inactive_frame,
            Scope::Yearly,
            vec![Scope::Natal],
        ),
        Err(ChartError::ActiveFrameScopeNotVisible {
            scope: Scope::Yearly,
        })
    );

    let missing_layout = HoroscopeChart::with_layers(
        natal,
        vec![temporal_layer(
            Scope::Yearly,
            EarthlyBranch::Wu,
            Vec::new(),
            Vec::new(),
            false,
        )],
    );
    assert_eq!(
        iztro::EffectiveChartState::from_horoscope(
            &missing_layout,
            Scope::Yearly,
            vec![Scope::Natal, Scope::Yearly],
        ),
        Err(ChartError::MissingHoroscopePalaceLayout {
            scope: Scope::Yearly,
        })
    );
}

#[test]
fn effective_state_uses_selected_frame_and_retains_fact_provenance() {
    let natal = build_chart(
        EarthlyBranch::Zi,
        &[(EarthlyBranch::Hai, StarName::TaiYang)],
    );
    let yearly_star = scoped(EarthlyBranch::Hai, StarName::LiuChang, Scope::Yearly);
    let activation = MutagenActivation::new(
        Scope::Yearly,
        StarName::TaiYang,
        EarthlyBranch::Hai,
        Mutagen::Ji,
    );
    let horoscope = HoroscopeChart::with_layers(
        natal,
        vec![temporal_layer(
            Scope::Yearly,
            EarthlyBranch::Hai,
            vec![yearly_star],
            vec![activation],
            true,
        )],
    );

    let state = iztro::EffectiveChartState::from_horoscope(
        &horoscope,
        Scope::Yearly,
        vec![Scope::Natal, Scope::Yearly],
    )
    .expect("valid effective state");

    assert_eq!(
        state.branch_of_palace(PalaceName::Life),
        Some(EarthlyBranch::Hai),
        "Life palace branch comes from the selected yearly frame"
    );

    let stars = state.stars_in_palace(EarthlyBranch::Hai);
    assert!(stars.iter().any(|star| {
        star.source_scope() == Scope::Natal && star.placement().name() == StarName::TaiYang
    }));
    assert!(stars.iter().any(|star| {
        star.source_scope() == Scope::Yearly && star.placement().name() == StarName::LiuChang
    }));

    assert_eq!(
        state
            .stars_in_palace_for_source(Scope::Natal, EarthlyBranch::Hai)
            .len(),
        1,
        "source-specific star queries isolate natal facts"
    );
    assert_eq!(
        state
            .stars_in_palace_for_source(Scope::Yearly, EarthlyBranch::Hai)
            .len(),
        1,
        "source-specific star queries isolate yearly facts"
    );

    let activations = state.mutagen_activations();
    assert_eq!(activations.len(), 1);
    assert_eq!(activations[0].source_scope(), Scope::Yearly);
    assert_eq!(activations[0].activation().target_star(), StarName::TaiYang);
}
