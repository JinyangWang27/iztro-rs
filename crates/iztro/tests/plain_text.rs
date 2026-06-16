use iztro::core::{
    Brightness, ChartAlgorithmKind, ChartStackSnapshot, EarthlyBranch, Gender, HeavenlyStem,
    HoroscopeChart, MethodProfile, Mutagen, MutagenActivation, Scope, ScopedStarPlacement,
    SolarChartRequest, SolarDay, SolarMonth, StarKind, StarName, StarPlacement, StemBranch,
    TemporalContext, TemporalLayer, VISUAL_BRANCH_ORDER, by_solar,
};
use iztro::render::{PlainTextChartRenderer, PlainTextRenderOptions, render_chart_stack_text};

fn solar_fixture_snapshot() -> ChartStackSnapshot {
    let request = SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(5).expect("May should be valid"))
        .solar_day(SolarDay::new(17).expect("day 17 should be valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .method_profile(MethodProfile::new(
            "plain_text_renderer_test",
            ChartAlgorithmKind::QuanShu,
            "plain text renderer test",
        ))
        .build()
        .expect("solar chart request should build");

    by_solar(request)
        .expect("by_solar should build fixture chart")
        .stack_snapshot()
}

fn yearly_fixture_snapshot() -> ChartStackSnapshot {
    let request = SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(5).expect("May should be valid"))
        .solar_day(SolarDay::new(17).expect("day 17 should be valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .method_profile(MethodProfile::new(
            "plain_text_renderer_test",
            ChartAlgorithmKind::QuanShu,
            "plain text renderer test",
        ))
        .build()
        .expect("solar chart request should build");
    let natal = by_solar(request).expect("by_solar should build fixture chart");
    let context = TemporalContext::Yearly {
        stem_branch: StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu)
            .expect("valid stem-branch pair"),
        lunar_year: 1990,
    };
    let scoped_star = ScopedStarPlacement::new(
        EarthlyBranch::Si,
        StarPlacement::new(
            StarName::NianJieYearly,
            StarKind::Helper,
            Brightness::Unknown,
            None,
            Scope::Yearly,
        ),
    );
    let activation = MutagenActivation::new(
        Scope::Yearly,
        StarName::TaiYang,
        EarthlyBranch::Wu,
        Mutagen::Lu,
    );
    let temporal_layer =
        TemporalLayer::try_new(Scope::Yearly, context, vec![scoped_star], vec![activation])
            .expect("temporal layer should build");
    let horoscope = HoroscopeChart::with_layers(natal, vec![temporal_layer]);

    ChartStackSnapshot::from_horoscope_chart(&horoscope)
}

#[test]
fn renders_header_and_natal_layer_in_visual_branch_order() {
    let snapshot = solar_fixture_snapshot();
    let output = render_chart_stack_text(&snapshot);

    assert!(output.contains("Chart Stack"));
    assert!(output.contains("birth: Lunar 1990-4-23, time Chen, gender Female"));
    assert!(output.contains("method: plain_text_renderer_test / QuanShu"));
    assert!(output.contains("life_palace_branch:"));
    assert!(output.contains("body_palace_branch:"));
    assert!(output.contains("five_element_bureau:"));
    assert!(output.contains("Layer 0: Natal"));
    assert!(output.contains("ZiWei"));

    let mut previous_index = 0;
    for branch in VISUAL_BRANCH_ORDER {
        let marker = format!("[{branch:?}]");
        let index = output[previous_index..]
            .find(&marker)
            .map(|relative_index| previous_index + relative_index)
            .unwrap_or_else(|| panic!("missing marker {marker}"));
        assert!(
            index >= previous_index,
            "marker {marker} should render after the previous branch marker"
        );
        previous_index = index + marker.len();
    }
}

#[test]
fn renders_same_snapshot_deterministically() {
    let snapshot = solar_fixture_snapshot();

    assert_eq!(
        render_chart_stack_text(&snapshot),
        render_chart_stack_text(&snapshot)
    );
}

#[test]
fn can_hide_decorative_stars() {
    let snapshot = solar_fixture_snapshot();
    let options = PlainTextRenderOptions {
        show_decorative_stars: false,
        ..PlainTextRenderOptions::default()
    };
    let renderer = PlainTextChartRenderer::new(options);

    let output = renderer.render(&snapshot);

    assert!(!output.contains("decorative:"));
}

#[test]
fn truncates_typed_stars_when_limit_is_lower_than_cell_contents() {
    let snapshot = solar_fixture_snapshot();
    let options = PlainTextRenderOptions {
        max_typed_stars_per_cell: 1,
        ..PlainTextRenderOptions::default()
    };
    let renderer = PlainTextChartRenderer::new(options);

    let output = renderer.render(&snapshot);

    assert!(output.contains("... (+"));
}

#[test]
fn can_render_only_natal_layer_from_temporal_stack() {
    let snapshot = yearly_fixture_snapshot();
    let options = PlainTextRenderOptions {
        show_temporal_layers: false,
        ..PlainTextRenderOptions::default()
    };
    let renderer = PlainTextChartRenderer::new(options);

    let output = renderer.render(&snapshot);

    assert!(output.contains("Layer 0: Natal"));
    assert!(!output.contains("Layer 1: Yearly"));
    assert!(!output.contains("context: Yearly"));
}

#[test]
fn renders_temporal_scoped_stars_and_mutagen_activations() {
    let snapshot = yearly_fixture_snapshot();

    let output = render_chart_stack_text(&snapshot);

    assert!(output.contains("Layer 1: Yearly"));
    assert!(output.contains("context: Yearly"));
    assert!(output.contains("scoped: NianJieYearly"));
    assert!(output.contains("mutagens: Yearly TaiYang Lu"));
}

#[test]
fn can_render_after_snapshot_serde_round_trip() {
    let snapshot = solar_fixture_snapshot();
    let encoded = serde_json::to_string(&snapshot).expect("snapshot should serialize");
    let decoded: ChartStackSnapshot =
        serde_json::from_str(&encoded).expect("snapshot should deserialize");

    assert_eq!(
        render_chart_stack_text(&snapshot),
        render_chart_stack_text(&decoded)
    );
}
