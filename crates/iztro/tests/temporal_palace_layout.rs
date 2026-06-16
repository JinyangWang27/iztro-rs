//! Decadal temporal palace-name layout: model invariants, snapshot exposure,
//! serialization round-trip, and parity against the upstream horoscope fixture.
//!
//! Only the decadal scope is covered; yearly/monthly/daily/hourly/age derivation
//! is intentionally out of scope for these tests.

use iztro::core::{
    Chart, ChartAlgorithmKind, ChartError, ChartLayerKind, ChartStackSnapshot, DecadalPeriod,
    EarthlyBranch, Gender, HeavenlyStem, HoroscopeChart, LunarChartRequest, LunarDay, LunarMonth,
    MethodProfile, PalaceName, Scope, StemBranch, TemporalContext, TemporalLayer,
    TemporalPalaceLayout, TemporalPalaceName, build_decadal_frame, build_decadal_horoscope_layer,
    by_lunar,
};
use serde::de::DeserializeOwned;
use serde_json::Value;

const HOROSCOPE_FIXTURE: &str = include_str!("../fixtures/iztro/horoscope.json");

/// Parses a normalized snake_case fixture key into a serde enum value.
fn parse_key<T: DeserializeOwned>(key: &str) -> T {
    serde_json::from_str(&format!("\"{key}\"")).expect("fixture key should parse")
}

/// Builds the natal chart for the canonical `canonical_female_default_2026`
/// fixture case: lunar 1990-5-17, 辰 hour (time index 4), female, 庚午 year.
fn canonical_female_chart() -> Chart {
    let request = LunarChartRequest::builder()
        .lunar_year(1990)
        .lunar_month(LunarMonth::new(5).expect("month 5 should be valid"))
        .lunar_day(LunarDay::new(17).expect("day 17 should be valid"))
        .iztro_time_index(4)
        .expect("time index 4 should be valid")
        .gender(Gender::Female)
        .birth_year_stem(HeavenlyStem::Geng)
        .birth_year_branch(EarthlyBranch::Wu)
        .method_profile(MethodProfile::new(
            "temporal_palace_layout_test",
            ChartAlgorithmKind::QuanShu,
            "temporal palace layout test",
        ))
        .build()
        .expect("lunar chart request should build");

    by_lunar(request).expect("by_lunar should build canonical fixture chart")
}

/// Returns the `supported_fields.decadal` block of the canonical fixture case.
fn canonical_decadal_fixture() -> Value {
    let fixture: Value =
        serde_json::from_str(HOROSCOPE_FIXTURE).expect("horoscope fixture should be valid JSON");
    let case = fixture["cases"]
        .as_array()
        .expect("fixture should list cases")
        .iter()
        .find(|case| case["id"] == "canonical_female_default_2026")
        .expect("canonical case should exist");

    case["supported_fields"]["decadal"].clone()
}

/// Selects the decadal period occupying `branch` from the chart's frame.
fn decadal_period_for_branch(chart: &Chart, branch: EarthlyBranch) -> DecadalPeriod {
    build_decadal_frame(chart)
        .expect("decadal frame should build")
        .periods()
        .iter()
        .find(|period| period.palace_branch() == branch)
        .cloned()
        .unwrap_or_else(|| panic!("decadal frame should contain a {branch:?} period"))
}

#[test]
fn decadal_horoscope_layer_attaches_a_twelve_name_palace_layout() {
    let chart = canonical_female_chart();
    let frame = build_decadal_frame(&chart).expect("decadal frame should build");
    let period = &frame.periods()[1];

    let layer = build_decadal_horoscope_layer(&chart, period).expect("decadal layer should build");

    let layout = layer
        .palace_layout()
        .expect("decadal layer should carry a palace-name layout");
    assert_eq!(layout.scope(), Scope::Decadal);
    assert_eq!(layout.names().len(), 12);

    // Branches are the stable spatial key: every branch appears exactly once and
    // every palace name appears exactly once across the layout.
    let mut branches: Vec<EarthlyBranch> = layout
        .names()
        .iter()
        .map(TemporalPalaceName::branch)
        .collect();
    branches.sort_by_key(|branch| format!("{branch:?}"));
    branches.dedup();
    assert_eq!(branches.len(), 12);

    let mut names: Vec<PalaceName> = layout
        .names()
        .iter()
        .map(TemporalPalaceName::palace_name)
        .collect();
    names.sort_by_key(|name| name.index());
    names.dedup();
    assert_eq!(names.len(), 12);

    // The period's own branch is relabeled as the decadal Life palace.
    assert_eq!(
        layout.name_for_branch(period.palace_branch()),
        Some(PalaceName::Life)
    );
}

#[test]
fn decadal_palace_layout_matches_canonical_upstream_fixture() {
    let chart = canonical_female_chart();
    let decadal = canonical_decadal_fixture();

    let fixture_branch: EarthlyBranch = parse_key(
        decadal["earthly_branch"]
            .as_str()
            .expect("decadal earthly_branch"),
    );
    let fixture_stem: HeavenlyStem = parse_key(
        decadal["heavenly_stem"]
            .as_str()
            .expect("decadal heavenly_stem"),
    );

    let period = decadal_period_for_branch(&chart, fixture_branch);
    assert_eq!(
        period.stem_branch(),
        StemBranch::try_new(fixture_stem, fixture_branch).expect("valid sexagenary pair"),
        "selected decadal period should match the fixture stem-branch"
    );

    let layer = build_decadal_horoscope_layer(&chart, &period).expect("decadal layer should build");
    let layout = layer
        .palace_layout()
        .expect("decadal layer should carry a palace-name layout");

    // The fixture stores palace names in Yin-first visual order, so array index
    // `i` maps to branch `Yin.offset(i)`. The Rust model keys names by branch.
    let palace_names = decadal["palace_names"]
        .as_array()
        .expect("decadal palace_names array");
    assert_eq!(palace_names.len(), 12);
    for (index, entry) in palace_names.iter().enumerate() {
        let branch = EarthlyBranch::Yin.offset(index as isize);
        let expected: PalaceName = parse_key(entry["name"].as_str().expect("palace name"));
        assert_eq!(
            layout.name_for_branch(branch),
            Some(expected),
            "decadal palace name mismatch at branch {branch:?} (fixture index {index})"
        );
    }
}

#[test]
fn snapshot_temporal_cells_expose_temporal_palace_names_separately_from_natal() {
    let chart = canonical_female_chart();
    let decadal = canonical_decadal_fixture();
    let fixture_branch: EarthlyBranch = parse_key(
        decadal["earthly_branch"]
            .as_str()
            .expect("decadal earthly_branch"),
    );
    let period = decadal_period_for_branch(&chart, fixture_branch);
    let layer = build_decadal_horoscope_layer(&chart, &period).expect("decadal layer should build");
    let horoscope = HoroscopeChart::with_layers(chart.clone(), vec![layer]);

    let snapshot = ChartStackSnapshot::from_horoscope_chart(&horoscope);

    // Natal layer cells never carry temporal palace names.
    let natal_layer = snapshot
        .layer(ChartLayerKind::Natal)
        .expect("natal snapshot layer should exist");
    for cell in natal_layer.cells() {
        assert_eq!(cell.temporal_palace_name(), None);
    }

    // Decadal layer cells expose the temporal palace name while preserving the
    // natal palace name (and stem) as spatial background facts.
    let decadal_layer = snapshot
        .layer(ChartLayerKind::Decadal)
        .expect("decadal snapshot layer should exist");
    for cell in decadal_layer.cells() {
        let natal_palace = chart
            .palaces()
            .iter()
            .find(|palace| palace.branch() == cell.branch());

        assert_eq!(
            cell.natal_palace_name(),
            natal_palace.map(|palace| palace.name()),
            "natal palace name should be preserved at branch {:?}",
            cell.branch()
        );
        assert_eq!(
            cell.natal_palace_stem(),
            natal_palace.map(|palace| palace.stem())
        );

        assert!(
            cell.temporal_palace_name().is_some(),
            "decadal cell at branch {:?} should expose a temporal palace name",
            cell.branch()
        );
        // Temporal palace names are additive: they do not leak into typed or
        // decorative star facts.
        assert!(cell.typed_stars().is_empty());
        assert!(cell.decorative_stars().is_empty());
    }

    // The Life relabeling lands on the period's branch.
    let life_cell = decadal_layer
        .cells()
        .iter()
        .find(|cell| cell.branch() == period.palace_branch())
        .expect("period branch should have a decadal cell");
    assert_eq!(life_cell.temporal_palace_name(), Some(PalaceName::Life));
}

#[test]
fn temporal_palace_layout_survives_serialization_round_trip() {
    let chart = canonical_female_chart();
    let frame = build_decadal_frame(&chart).expect("decadal frame should build");
    let period = &frame.periods()[3];
    let layer = build_decadal_horoscope_layer(&chart, period).expect("decadal layer should build");

    let encoded = serde_json::to_string(&layer).expect("layer should serialize");
    let decoded: TemporalLayer = serde_json::from_str(&encoded).expect("layer should deserialize");

    assert_eq!(decoded, layer);
    assert_eq!(
        decoded.palace_layout().map(TemporalPalaceLayout::scope),
        Some(Scope::Decadal)
    );
    assert_eq!(
        decoded
            .palace_layout()
            .expect("decoded layer should keep its palace layout")
            .names()
            .len(),
        12
    );
}

#[test]
fn temporal_layer_rejects_palace_layout_scope_mismatch() {
    let context = TemporalContext::Decadal {
        stem_branch: StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu)
            .expect("valid sexagenary pair"),
        start_age: 6,
    };
    // A yearly-scoped layout cannot ride on a decadal layer.
    let layout = TemporalPalaceLayout::try_new(
        Scope::Yearly,
        vec![TemporalPalaceName::new(
            EarthlyBranch::Hai,
            PalaceName::Life,
        )],
    )
    .expect("non-natal layout should build");

    let result = TemporalLayer::try_new_with_palace_layout(
        Scope::Decadal,
        context,
        Vec::new(),
        Vec::new(),
        Some(layout),
    );

    assert_eq!(
        result.unwrap_err(),
        ChartError::TemporalPalaceLayoutScopeMismatch {
            layer: Scope::Decadal,
            layout: Scope::Yearly,
        }
    );
}

#[test]
fn temporal_palace_layout_rejects_natal_scope() {
    let result = TemporalPalaceLayout::try_new(
        Scope::Natal,
        vec![TemporalPalaceName::new(
            EarthlyBranch::Hai,
            PalaceName::Life,
        )],
    );

    assert_eq!(result.unwrap_err(), ChartError::NatalScopeInTemporalLayer);
}
