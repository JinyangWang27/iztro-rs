//! Tests for the high-level natal star-placement strategy layer.
//!
//! These verify that delegating through `NatalStarPlacementStrategy` preserves
//! the existing behavior (the default strategy reproduces the legacy builder
//! exactly) and that a custom strategy is actually used when injected.

use iztro::core::{
    BirthContext, CalendarDate, Chart, ChartAlgorithmKind, ChartError,
    DeterministicNatalStarPlacementStrategy, EarthlyBranch, Gender, HeavenlyStem, LunarDay,
    LunarMonth, MethodProfile, NatalChartWithSupportedStarsInput, NatalStarPlacementStrategy,
    build_natal_chart_with_supported_stars, build_natal_chart_with_supported_stars_using,
};

fn sample_input() -> NatalChartWithSupportedStarsInput {
    NatalChartWithSupportedStarsInput::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("natal_placement_strategy_test"),
        LunarMonth::new(1).expect("month 1 should be valid"),
        LunarDay::new(23).expect("day 23 should be valid"),
        HeavenlyStem::Geng,
        EarthlyBranch::Wu,
    )
}

fn zhongzhou_input() -> NatalChartWithSupportedStarsInput {
    NatalChartWithSupportedStarsInput::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::new(
            "zhongzhou_strategy_test",
            ChartAlgorithmKind::Zhongzhou,
            "Zhongzhou strategy-preservation test",
        ),
        LunarMonth::new(1).expect("month 1 should be valid"),
        LunarDay::new(23).expect("day 23 should be valid"),
        HeavenlyStem::Geng,
        EarthlyBranch::Wu,
    )
}

#[test]
fn default_strategy_matches_legacy_supported_builder() {
    let legacy = build_natal_chart_with_supported_stars(sample_input())
        .expect("legacy supported-star builder should succeed");
    let via_strategy = build_natal_chart_with_supported_stars_using(
        sample_input(),
        &DeterministicNatalStarPlacementStrategy::default(),
    )
    .expect("strategy-injecting builder should succeed");

    assert_eq!(legacy, via_strategy);
}

#[test]
fn default_strategy_matches_legacy_supported_builder_for_zhongzhou() {
    // The Zhongzhou algorithm exercises adjective-star branching that differs
    // from the default/placeholder path, so it is the key behavior-preservation
    // case for the strategy refactor.
    let input = zhongzhou_input();
    let legacy = build_natal_chart_with_supported_stars(input.clone())
        .expect("legacy supported-star builder should succeed for Zhongzhou");
    let via_strategy = build_natal_chart_with_supported_stars_using(
        input,
        &DeterministicNatalStarPlacementStrategy::default(),
    )
    .expect("strategy-injecting builder should succeed for Zhongzhou");

    assert_eq!(legacy, via_strategy);
}

/// A custom strategy that ignores the input entirely and never touches the
/// chart. It proves the injected strategy — not the deterministic default — is
/// the code path that runs.
struct NoopStrategy;

impl NatalStarPlacementStrategy for NoopStrategy {
    fn place_supported_stars(
        &self,
        chart: Chart,
        _input: &NatalChartWithSupportedStarsInput,
    ) -> Result<Chart, ChartError> {
        Ok(chart)
    }
}

#[test]
fn custom_strategy_is_used_when_injected() {
    let deterministic = build_natal_chart_with_supported_stars(sample_input())
        .expect("deterministic builder should succeed");
    let noop = build_natal_chart_with_supported_stars_using(sample_input(), &NoopStrategy)
        .expect("noop strategy should succeed");

    // The deterministic chart places stars; the noop strategy leaves the
    // minimal chart untouched, so the two must differ.
    assert_ne!(deterministic, noop);

    let any_stars_placed = deterministic
        .palaces()
        .iter()
        .any(|palace| !palace.stars().is_empty());
    assert!(
        any_stars_placed,
        "deterministic strategy should place at least one star"
    );

    let noop_has_no_stars = noop
        .palaces()
        .iter()
        .all(|palace| palace.stars().is_empty());
    assert!(
        noop_has_no_stars,
        "noop strategy should leave the minimal chart without placed stars"
    );
}

#[test]
fn strategy_is_object_safe() {
    let strategy: &dyn NatalStarPlacementStrategy =
        &DeterministicNatalStarPlacementStrategy::default();
    let via_dyn = build_natal_chart_with_supported_stars_using(sample_input(), strategy)
        .expect("dyn strategy should succeed");
    let legacy = build_natal_chart_with_supported_stars(sample_input())
        .expect("legacy builder should succeed");

    assert_eq!(legacy, via_dyn);
}
