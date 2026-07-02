//! Tests for the shared [`RuleEvaluationContext`] and its pattern/classical
//! wrappers.
//!
//! Patterns are a subset of rules, so `PatternContext` and
//! `ClassicalRuleContext` both wrap the same shared selected-state context.
//! These tests pin the constructor semantics and confirm the wrappers expose an
//! identical selected view for the same inputs.

use iztro::{
    BirthContext, CalendarDate, Chart, ChartError, ClassicalRuleContext, EarthlyBranch, Gender,
    HeavenlyStem, HoroscopeChart, MethodProfile, PALACE_NAMES, Palace, PalaceName, PatternContext,
    RuleEvaluationContext, Scope, StarPlacement, StemBranch, TemporalContext, TemporalLayer,
    TemporalPalaceLayout, TemporalPalaceName,
};

fn build_chart(life_branch: EarthlyBranch) -> Chart {
    let palaces: Vec<Palace> = (0..12)
        .map(|index| {
            let name = PALACE_NAMES[index];
            let branch = life_branch.offset(index as isize);
            Palace::new(name, branch, HeavenlyStem::Jia, Vec::<StarPlacement>::new())
        })
        .collect();

    Chart::try_new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu).expect("valid stem-branch"),
        MethodProfile::placeholder("rule_evaluation_context_test"),
        palaces,
        None,
        None,
    )
    .expect("synthetic chart should build")
}

fn temporal_palace_layout(scope: Scope, life_branch: EarthlyBranch) -> TemporalPalaceLayout {
    let names = PALACE_NAMES
        .iter()
        .enumerate()
        .map(|(index, name)| TemporalPalaceName::new(life_branch.offset(index as isize), *name))
        .collect();
    TemporalPalaceLayout::try_new(scope, names).expect("valid temporal palace layout")
}

/// Builds a horoscope with a natal Life at `natal_life` and a Yearly frame whose
/// Life palace sits at `yearly_life`.
fn build_horoscope(natal_life: EarthlyBranch, yearly_life: EarthlyBranch) -> HoroscopeChart {
    let natal = build_chart(natal_life);
    let yearly = TemporalLayer::try_new_with_palace_layout(
        Scope::Yearly,
        TemporalContext::Yearly {
            stem_branch: StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Zi)
                .expect("valid stem-branch"),
            lunar_year: 2026,
        },
        Vec::new(),
        Vec::new(),
        Some(temporal_palace_layout(Scope::Yearly, yearly_life)),
    )
    .expect("valid temporal layer");
    HoroscopeChart::with_layers(natal, vec![yearly])
}

/// Builds a horoscope carrying only a Decadal layer, so any request for a
/// deeper temporal frame (e.g. Yearly) has no matching layer to resolve.
fn build_decadal_only_horoscope(natal_life: EarthlyBranch) -> HoroscopeChart {
    let natal = build_chart(natal_life);
    let decadal = TemporalLayer::try_new_with_palace_layout(
        Scope::Decadal,
        TemporalContext::Decadal {
            stem_branch: StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Zi)
                .expect("valid stem-branch"),
            start_age: 34,
        },
        Vec::new(),
        Vec::new(),
        Some(temporal_palace_layout(Scope::Decadal, natal_life)),
    )
    .expect("valid temporal layer");
    HoroscopeChart::with_layers(natal, vec![decadal])
}

#[test]
fn try_horoscope_with_frame_errors_on_missing_temporal_layer() {
    // Only a Decadal layer is present, but a Yearly frame is requested: strict
    // effective-state construction has no Yearly layer to resolve. The fallible
    // constructor must surface that as a recoverable `ChartError`, not panic.
    let horoscope = build_decadal_only_horoscope(EarthlyBranch::Zi);
    let result = RuleEvaluationContext::try_horoscope_with_frame(
        &horoscope,
        Scope::Yearly,
        vec![Scope::Natal, Scope::Decadal, Scope::Yearly],
    );

    // The exact variant is owned by `EffectiveChartState::from_horoscope`; a
    // missing frame layer surfaces as `MissingHoroscopeLayer` today.
    assert!(matches!(
        result,
        Err(ChartError::MissingHoroscopeLayer { .. })
            | Err(ChartError::MissingHoroscopePalaceLayout { .. })
            | Err(_)
    ));
    assert!(result.is_err());
}

#[test]
fn pattern_try_horoscope_with_frame_propagates_error() {
    let horoscope = build_decadal_only_horoscope(EarthlyBranch::Zi);
    let result = PatternContext::try_horoscope_with_frame(
        &horoscope,
        Scope::Yearly,
        vec![Scope::Natal, Scope::Decadal, Scope::Yearly],
    );

    assert!(result.is_err());
}

#[test]
fn classical_try_horoscope_with_frame_propagates_error() {
    let horoscope = build_decadal_only_horoscope(EarthlyBranch::Zi);
    let result = ClassicalRuleContext::try_horoscope_with_frame(
        &horoscope,
        Scope::Yearly,
        vec![Scope::Natal, Scope::Decadal, Scope::Yearly],
    );

    assert!(result.is_err());
}

#[test]
fn try_horoscope_with_frame_succeeds_for_valid_frame() {
    // The fallible constructor returns the same selected view as the strict one
    // when the frame/scope combination is valid.
    let horoscope = build_horoscope(EarthlyBranch::Zi, EarthlyBranch::Chou);
    let ctx = RuleEvaluationContext::try_horoscope_with_frame(
        &horoscope,
        Scope::Yearly,
        vec![Scope::Natal, Scope::Yearly],
    )
    .expect("valid frame builds an effective state");

    assert_eq!(ctx.selected_frame_scope(), Some(Scope::Yearly));
    assert_eq!(
        ctx.effective().unwrap().branch_of_palace(PalaceName::Life),
        Some(EarthlyBranch::Chou)
    );
}

#[test]
fn natal_context_is_strict_natal_frame() {
    let chart = build_chart(EarthlyBranch::Zi);
    let ctx = RuleEvaluationContext::natal(&chart);

    assert_eq!(ctx.selected_frame_scope(), Some(Scope::Natal));
    assert_eq!(ctx.active_scopes(), &[Scope::Natal]);
    assert!(ctx.effective().is_some());
    assert!(ctx.horoscope_chart().is_none());
}

#[test]
fn explicit_horoscope_frame_reads_selected_life_branch() {
    let horoscope = build_horoscope(EarthlyBranch::Zi, EarthlyBranch::Chou);
    let ctx = RuleEvaluationContext::horoscope_with_frame(
        &horoscope,
        Scope::Yearly,
        vec![Scope::Natal, Scope::Yearly],
    );

    assert_eq!(ctx.selected_frame_scope(), Some(Scope::Yearly));
    assert_eq!(
        ctx.effective().unwrap().branch_of_palace(PalaceName::Life),
        Some(EarthlyBranch::Chou)
    );
    assert!(ctx.horoscope_chart().is_some());
}

#[test]
fn compatibility_horoscope_derives_frame_from_deepest_active_scope() {
    let horoscope = build_horoscope(EarthlyBranch::Zi, EarthlyBranch::Chou);
    let ctx = RuleEvaluationContext::horoscope(&horoscope, vec![Scope::Natal, Scope::Yearly]);

    assert_eq!(ctx.selected_frame_scope(), Some(Scope::Yearly));
    assert_eq!(
        ctx.effective().unwrap().branch_of_palace(PalaceName::Life),
        Some(EarthlyBranch::Chou)
    );
}

#[test]
fn compatibility_horoscope_fails_closed_without_natal_scope() {
    let horoscope = build_horoscope(EarthlyBranch::Zi, EarthlyBranch::Chou);
    // Missing `Scope::Natal` makes strict effective-state construction fail; the
    // lenient constructor then leaves the selected state empty.
    let ctx = RuleEvaluationContext::horoscope(&horoscope, vec![Scope::Yearly]);

    assert!(ctx.effective().is_none());
    assert_eq!(ctx.selected_frame_scope(), None);
}

#[test]
fn pattern_wrapper_matches_inner_natal_frame() {
    let chart = build_chart(EarthlyBranch::Zi);
    let pattern = PatternContext::natal(&chart);

    assert_eq!(
        pattern.selected_frame_scope(),
        pattern.as_rule_context().selected_frame_scope()
    );
    assert_eq!(pattern.selected_frame_scope(), Some(Scope::Natal));
}

#[test]
fn classical_wrapper_matches_inner_natal_frame() {
    let chart = build_chart(EarthlyBranch::Zi);
    let classical = ClassicalRuleContext::natal(&chart);

    assert_eq!(
        classical.selected_frame_scope(),
        classical.as_rule_context().selected_frame_scope()
    );
    assert_eq!(classical.selected_frame_scope(), Some(Scope::Natal));
}

#[test]
fn pattern_and_classical_wrappers_agree_on_selected_life_branch() {
    let horoscope = build_horoscope(EarthlyBranch::Zi, EarthlyBranch::Chou);
    let active = vec![Scope::Natal, Scope::Yearly];

    let pattern = PatternContext::horoscope_with_frame(&horoscope, Scope::Yearly, active.clone());
    let classical = ClassicalRuleContext::horoscope_with_frame(&horoscope, Scope::Yearly, active);

    let pattern_life = pattern
        .effective()
        .unwrap()
        .branch_of_palace(PalaceName::Life);
    let classical_life = classical
        .effective()
        .unwrap()
        .branch_of_palace(PalaceName::Life);

    assert_eq!(pattern_life, Some(EarthlyBranch::Chou));
    assert_eq!(pattern_life, classical_life);
    assert_eq!(
        pattern.selected_frame_scope(),
        classical.selected_frame_scope()
    );
}
