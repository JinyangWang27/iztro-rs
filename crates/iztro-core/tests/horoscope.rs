use iztro_core::{
    BirthContext, Brightness, CalendarDate, Chart, ChartError, EarthlyBranch, Gender, HeavenlyStem,
    HoroscopeChart, LunarDay, LunarMonth, MethodProfile, Mutagen, MutagenActivation,
    NatalChartWithSupportedStarsInput, Scope, ScopedStarPlacement, StarKind, StarName,
    StarPlacement, StemBranch, TemporalContext, TemporalLayer,
    build_natal_chart_with_supported_stars,
};

/// `by_lunar`/the supported-star builder place 14 major + 14 minor + 38
/// adjective/helper = 66 natal stars.
const NATAL_STAR_COUNT: usize = 66;

fn supported_star_natal_chart() -> Chart {
    build_natal_chart_with_supported_stars(NatalChartWithSupportedStarsInput::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chou,
            Gender::Female,
        ),
        MethodProfile::placeholder("horoscope_test_profile"),
        LunarMonth::new(1).expect("month 1 should be valid"),
        LunarDay::new(23).expect("day 23 should be valid"),
        HeavenlyStem::Geng,
        EarthlyBranch::Wu,
    ))
    .expect("supported-star natal chart should build")
}

fn yearly_context() -> TemporalContext {
    TemporalContext::Yearly {
        stem_branch: StemBranch::new(HeavenlyStem::Geng, EarthlyBranch::Wu),
        lunar_year: 1990,
    }
}

fn sample_yearly_layer() -> TemporalLayer {
    let placements = vec![ScopedStarPlacement::new(
        EarthlyBranch::Si,
        StarPlacement::new(
            StarName::NianJieYearly,
            StarKind::Helper,
            Brightness::Unknown,
            None,
            Scope::Yearly,
        ),
    )];
    let activations = vec![MutagenActivation::new(
        Scope::Yearly,
        StarName::TaiYang,
        EarthlyBranch::Wu,
        Mutagen::Lu,
    )];

    TemporalLayer::try_new(Scope::Yearly, yearly_context(), placements, activations)
        .expect("yearly layer should build")
}

#[test]
fn horoscope_chart_preserves_the_54_natal_stars() {
    let mut horoscope = HoroscopeChart::new(supported_star_natal_chart());
    horoscope.push_layer(sample_yearly_layer());

    assert_eq!(horoscope.natal().stars().len(), NATAL_STAR_COUNT);
    assert!(
        horoscope
            .natal()
            .stars()
            .iter()
            .all(|fact| fact.placement().scope() == Scope::Natal)
    );
}

#[test]
fn temporal_layer_placements_carry_non_natal_scope() {
    let layer = sample_yearly_layer();

    assert!(!layer.placements().is_empty());
    assert!(
        layer
            .placements()
            .iter()
            .all(|placement| placement.scope() != Scope::Natal)
    );
    assert!(
        layer
            .placements()
            .iter()
            .all(|placement| placement.scope() == layer.scope())
    );
}

#[test]
fn temporal_context_reports_its_scope() {
    assert_eq!(yearly_context().scope(), Scope::Yearly);
    assert_eq!(sample_yearly_layer().scope(), Scope::Yearly);
}

#[test]
fn horoscope_chart_groups_layers_by_scope() {
    let mut horoscope = HoroscopeChart::new(supported_star_natal_chart());
    horoscope.push_layer(sample_yearly_layer());

    assert_eq!(horoscope.layers().len(), 1);
    assert_eq!(horoscope.layers_in_scope(Scope::Yearly).count(), 1);
    assert_eq!(horoscope.layers_in_scope(Scope::Decadal).count(), 0);
}

#[test]
fn horoscope_chart_round_trips_through_json() {
    let mut horoscope = HoroscopeChart::new(supported_star_natal_chart());
    horoscope.push_layer(sample_yearly_layer());

    let encoded = serde_json::to_string(&horoscope).expect("horoscope chart should serialize");
    let decoded: HoroscopeChart =
        serde_json::from_str(&encoded).expect("horoscope chart should deserialize");

    assert_eq!(decoded, horoscope);
    assert_eq!(decoded.natal().stars().len(), NATAL_STAR_COUNT);
    assert_eq!(decoded.layers().len(), 1);
}

#[test]
fn temporal_layer_rejects_natal_scope() {
    let result = TemporalLayer::try_new(Scope::Natal, yearly_context(), Vec::new(), Vec::new());

    assert_eq!(result.unwrap_err(), ChartError::NatalScopeInTemporalLayer);
}

#[test]
fn temporal_layer_rejects_scope_context_mismatch() {
    let context = TemporalContext::Decadal {
        stem_branch: StemBranch::new(HeavenlyStem::Geng, EarthlyBranch::Wu),
        start_age: 6,
    };

    let result = TemporalLayer::try_new(Scope::Yearly, context, Vec::new(), Vec::new());

    assert_eq!(
        result.unwrap_err(),
        ChartError::TemporalScopeMismatch {
            layer: Scope::Yearly,
            context: Scope::Decadal,
        }
    );
}

#[test]
fn temporal_layer_rejects_natal_scoped_placement() {
    let placement = ScopedStarPlacement::new(
        EarthlyBranch::Si,
        StarPlacement::new(
            StarName::NianJieYearly,
            StarKind::Helper,
            Brightness::Unknown,
            None,
            Scope::Natal,
        ),
    );

    let result =
        TemporalLayer::try_new(Scope::Yearly, yearly_context(), vec![placement], Vec::new());

    assert_eq!(
        result.unwrap_err(),
        ChartError::TemporalPlacementScopeMismatch {
            layer: Scope::Yearly,
            placement: Scope::Natal,
        }
    );
}

#[test]
fn temporal_layer_rejects_mismatched_activation_scope() {
    let activation = MutagenActivation::new(
        Scope::Monthly,
        StarName::TaiYang,
        EarthlyBranch::Wu,
        Mutagen::Lu,
    );

    let result = TemporalLayer::try_new(
        Scope::Yearly,
        yearly_context(),
        Vec::new(),
        vec![activation],
    );

    assert_eq!(
        result.unwrap_err(),
        ChartError::TemporalActivationScopeMismatch {
            layer: Scope::Yearly,
            activation: Scope::Monthly,
        }
    );
}

/// Serializes a valid yearly layer, applies `mutate`, then asserts the tampered
/// JSON fails to deserialize because [`TemporalLayer::try_new`] rejects it.
fn assert_tampered_layer_json_is_rejected(
    mutate: impl FnOnce(&mut serde_json::Value),
    expected_fragment: &str,
) {
    let mut value =
        serde_json::to_value(sample_yearly_layer()).expect("yearly layer should serialize");
    mutate(&mut value);

    let error = serde_json::from_value::<TemporalLayer>(value)
        .expect_err("tampered temporal layer JSON should be rejected");

    assert!(
        error.to_string().contains(expected_fragment),
        "error `{error}` should mention `{expected_fragment}`"
    );
}

#[test]
fn temporal_layer_json_cannot_bypass_natal_scope_rejection() {
    assert_tampered_layer_json_is_rejected(
        |value| value["scope"] = serde_json::json!("natal"),
        "natal scope",
    );
}

#[test]
fn temporal_layer_json_cannot_bypass_scope_context_mismatch() {
    assert_tampered_layer_json_is_rejected(
        |value| value["scope"] = serde_json::json!("monthly"),
        "does not match context scope",
    );
}

#[test]
fn temporal_layer_json_cannot_bypass_placement_scope_mismatch() {
    assert_tampered_layer_json_is_rejected(
        |value| value["placements"][0]["placement"]["scope"] = serde_json::json!("natal"),
        "does not match layer scope",
    );
}

#[test]
fn temporal_layer_json_cannot_bypass_activation_scope_mismatch() {
    assert_tampered_layer_json_is_rejected(
        |value| value["activations"][0]["source_scope"] = serde_json::json!("monthly"),
        "does not match layer scope",
    );
}
