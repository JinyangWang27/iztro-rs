use iztro_core::{
    BirthContext, CalendarDate, Chart, EarthlyBranch, Gender, HeavenlyStem, LunarDay, LunarMonth,
    MethodProfile, Mutagen, NatalChartWithMajorStarsInput, NatalChartWithSupportedStarsInput,
    Scope, StarName, StemBranch, TemporalContext, TemporalLayer, YearlyMutagenLayerInput,
    birth_year_star_mutagen, build_natal_chart_with_major_stars,
    build_natal_chart_with_supported_stars, build_yearly_mutagen_layer,
};

/// The supported-star builder places 14 major + 14 minor + 38 adjective/helper
/// = 66 natal stars.
const NATAL_STAR_COUNT: usize = 66;

/// The documented 1990-05-17 辰时 female case (ganzhi year 庚午 = Geng-Wu).
fn supported_star_natal_chart() -> Chart {
    build_natal_chart_with_supported_stars(NatalChartWithSupportedStarsInput::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("yearly_mutagen_test_profile"),
        LunarMonth::new(1).expect("month 1 should be valid"),
        LunarDay::new(23).expect("day 23 should be valid"),
        HeavenlyStem::Geng,
        EarthlyBranch::Wu,
    ))
    .expect("supported-star natal chart should build")
}

/// The same case built with only the fourteen major stars, so the minor targets
/// 文曲/文昌 are deliberately absent.
fn major_star_only_natal_chart() -> Chart {
    build_natal_chart_with_major_stars(NatalChartWithMajorStarsInput::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("yearly_mutagen_test_profile"),
        LunarMonth::new(1).expect("month 1 should be valid"),
        LunarDay::new(23).expect("day 23 should be valid"),
        HeavenlyStem::Geng,
        EarthlyBranch::Wu,
    ))
    .expect("major-star natal chart should build")
}

fn geng_stem_branch() -> StemBranch {
    StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu).expect("valid sexagenary pair")
}

/// 辛酉 (Xin-You): the 辛 stem maps two major targets (巨门 Lu, 太阳 Quan) and
/// two minor targets (文曲 Ke, 文昌 Ji).
fn xin_stem_branch() -> StemBranch {
    StemBranch::try_new(HeavenlyStem::Xin, EarthlyBranch::You).expect("valid sexagenary pair")
}

#[test]
fn yearly_layer_has_yearly_scope_context_and_activation_shape() {
    let natal = supported_star_natal_chart();
    let layer = build_yearly_mutagen_layer(
        &natal,
        YearlyMutagenLayerInput::new(geng_stem_branch(), 1990),
    )
    .expect("yearly mutagen layer should build");

    assert_eq!(layer.scope(), Scope::Yearly);
    assert_eq!(
        *layer.context(),
        TemporalContext::Yearly {
            stem_branch: geng_stem_branch(),
            lunar_year: 1990,
        }
    );
    assert!(layer.placements().is_empty());
    assert!(!layer.activations().is_empty());
    assert!(
        layer
            .activations()
            .iter()
            .all(|activation| activation.source_scope() == Scope::Yearly)
    );
}

#[test]
fn building_a_yearly_layer_leaves_the_natal_chart_unchanged() {
    let natal = supported_star_natal_chart();
    let before: Vec<(StarName, Option<Mutagen>, Scope)> = natal
        .stars()
        .iter()
        .map(|fact| {
            (
                fact.placement().name(),
                fact.placement().mutagen(),
                fact.placement().scope(),
            )
        })
        .collect();

    let _layer = build_yearly_mutagen_layer(
        &natal,
        YearlyMutagenLayerInput::new(geng_stem_branch(), 1990),
    )
    .expect("yearly mutagen layer should build");

    assert_eq!(natal.stars().len(), NATAL_STAR_COUNT);
    assert!(
        natal
            .stars()
            .iter()
            .all(|fact| fact.placement().scope() == Scope::Natal)
    );

    let after: Vec<(StarName, Option<Mutagen>, Scope)> = natal
        .stars()
        .iter()
        .map(|fact| {
            (
                fact.placement().name(),
                fact.placement().mutagen(),
                fact.placement().scope(),
            )
        })
        .collect();
    assert_eq!(before, after);
}

#[test]
fn each_activation_targets_the_branch_of_its_natal_star() {
    let natal = supported_star_natal_chart();
    let layer = build_yearly_mutagen_layer(
        &natal,
        YearlyMutagenLayerInput::new(geng_stem_branch(), 1990),
    )
    .expect("yearly mutagen layer should build");

    for activation in layer.activations() {
        let palace = natal
            .palace_containing_star(activation.target_star())
            .expect("activation target must be a star present in the natal chart");

        assert_eq!(activation.target_branch(), palace.branch());
        // The mutagen is the one the shared Heavenly Stem table maps, not a
        // hard-coded literal.
        assert_eq!(
            Some(activation.mutagen()),
            birth_year_star_mutagen(HeavenlyStem::Geng, activation.target_star())
        );
    }
}

#[test]
fn absent_target_stars_are_skipped_rather_than_invented() {
    let major_only = major_star_only_natal_chart();
    let supported = supported_star_natal_chart();

    let major_layer = build_yearly_mutagen_layer(
        &major_only,
        YearlyMutagenLayerInput::new(xin_stem_branch(), 1981),
    )
    .expect("yearly mutagen layer should build for a major-only chart");
    let full_layer = build_yearly_mutagen_layer(
        &supported,
        YearlyMutagenLayerInput::new(xin_stem_branch(), 1981),
    )
    .expect("yearly mutagen layer should build for a full chart");

    // The major-only chart lacks 文曲/文昌, so only the two major targets remain.
    assert_eq!(major_layer.activations().len(), 2);
    assert!(major_layer.activations().iter().all(|activation| matches!(
        activation.target_star(),
        StarName::JuMen | StarName::TaiYang
    )));

    // The full chart has the minor targets too, so all four mappings activate.
    assert_eq!(full_layer.activations().len(), 4);
}

#[test]
fn yearly_layer_holds_no_placements_and_only_references_natal_stars() {
    let natal = supported_star_natal_chart();
    let layer = build_yearly_mutagen_layer(
        &natal,
        YearlyMutagenLayerInput::new(geng_stem_branch(), 1990),
    )
    .expect("yearly mutagen layer should build");

    assert!(layer.placements().is_empty());
    for activation in layer.activations() {
        let palace = natal
            .palace_containing_star(activation.target_star())
            .expect("activation must reference a star present in the natal chart");
        assert_eq!(activation.target_branch(), palace.branch());
    }
}

#[test]
fn yearly_layer_round_trips_through_json() {
    let natal = supported_star_natal_chart();
    let layer = build_yearly_mutagen_layer(
        &natal,
        YearlyMutagenLayerInput::new(geng_stem_branch(), 1990),
    )
    .expect("yearly mutagen layer should build");

    let encoded = serde_json::to_string(&layer).expect("yearly layer should serialize");
    let decoded: TemporalLayer =
        serde_json::from_str(&encoded).expect("yearly layer should deserialize");

    assert_eq!(decoded, layer);
    assert_eq!(decoded.scope(), Scope::Yearly);
    assert!(decoded.placements().is_empty());
}
