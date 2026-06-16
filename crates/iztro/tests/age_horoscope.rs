mod common;

use std::collections::HashMap;

use common::parse_key;
use iztro::core::{
    AgePeriod, Chart, ChartAlgorithmKind, ChartError, ChartLayerKind, ChartStackSnapshot,
    EarthlyBranch, Gender, HeavenlyStem, HoroscopeChart, MethodProfile, Mutagen, PalaceName, Scope,
    SolarChartRequest, SolarDay, SolarMonth, StarName, StemBranch, TemporalContext,
    build_age_horoscope_layer, build_age_period, by_solar,
};

const NOMINAL_AGE: u8 = 37;

fn fixture_chart() -> Chart {
    let request = SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(5).expect("May should be valid"))
        .solar_day(SolarDay::new(17).expect("day 17 should be valid"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .method_profile(MethodProfile::new(
            "age_horoscope_test",
            ChartAlgorithmKind::QuanShu,
            "age horoscope test",
        ))
        .build()
        .expect("solar chart request should build");

    by_solar(request).expect("by_solar should build fixture chart")
}

fn expected_stem_branch() -> StemBranch {
    StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Chen)
        .expect("fixture stem-branch should be valid")
}

fn expected_palace_names_by_branch() -> HashMap<EarthlyBranch, PalaceName> {
    [
        (EarthlyBranch::Yin, PalaceName::Spouse),
        (EarthlyBranch::Mao, PalaceName::Siblings),
        (EarthlyBranch::Chen, PalaceName::Life),
        (EarthlyBranch::Si, PalaceName::Parents),
        (EarthlyBranch::Wu, PalaceName::Spirit),
        (EarthlyBranch::Wei, PalaceName::Property),
        (EarthlyBranch::Shen, PalaceName::Career),
        (EarthlyBranch::You, PalaceName::Friends),
        (EarthlyBranch::Xu, PalaceName::Migration),
        (EarthlyBranch::Hai, PalaceName::Health),
        (EarthlyBranch::Zi, PalaceName::Wealth),
        (EarthlyBranch::Chou, PalaceName::Children),
    ]
    .into_iter()
    .collect()
}

fn expected_mutagens() -> HashMap<(StarName, EarthlyBranch), Mutagen> {
    [
        ((StarName::TaiYang, EarthlyBranch::Chou), Mutagen::Lu),
        ((StarName::WuQu, EarthlyBranch::Zi), Mutagen::Quan),
        ((StarName::TaiYin, EarthlyBranch::Chou), Mutagen::Ke),
        ((StarName::TianTong, EarthlyBranch::Hai), Mutagen::Ji),
    ]
    .into_iter()
    .collect()
}

#[test]
fn age_scope_is_non_natal_temporal_scope() {
    let context = TemporalContext::Age {
        stem_branch: expected_stem_branch(),
        nominal_age: NOMINAL_AGE,
    };

    assert_eq!(context.scope(), Scope::Age);
    assert_eq!(context.stem_branch(), expected_stem_branch());
    assert_ne!(context.scope(), Scope::Natal);
    assert_eq!(ChartLayerKind::from_scope(Scope::Age), ChartLayerKind::Age);
}

#[test]
fn age_horoscope_layer_matches_canonical_fixture_context() {
    let chart = fixture_chart();
    let period = build_age_period(&chart, NOMINAL_AGE).expect("age period should build");

    assert_age_period_matches_fixture(&period);

    let layer =
        build_age_horoscope_layer(&chart, &period).expect("age horoscope layer should build");

    assert_eq!(layer.scope(), Scope::Age);
    assert_eq!(
        *layer.context(),
        TemporalContext::Age {
            stem_branch: expected_stem_branch(),
            nominal_age: NOMINAL_AGE,
        }
    );
    assert!(layer.placements().is_empty());
}

#[test]
fn age_horoscope_layer_matches_canonical_fixture_palace_layout() {
    let chart = fixture_chart();
    let period = build_age_period(&chart, NOMINAL_AGE).expect("age period should build");
    let layer =
        build_age_horoscope_layer(&chart, &period).expect("age horoscope layer should build");

    let layout = layer
        .palace_layout()
        .expect("age layer should carry palace layout");
    assert_eq!(layout.scope(), Scope::Age);
    assert_eq!(layout.names().len(), 12);

    let expected = expected_palace_names_by_branch();
    for name in layout.names() {
        assert_eq!(
            Some(name.palace_name()),
            expected.get(&name.branch()).copied(),
            "age palace name mismatch at {:?}",
            name.branch()
        );
    }
    assert_eq!(
        layout.name_for_branch(EarthlyBranch::Chen),
        Some(PalaceName::Life)
    );
}

#[test]
fn age_horoscope_layer_matches_canonical_fixture_mutagens() {
    let chart = fixture_chart();
    let period = build_age_period(&chart, NOMINAL_AGE).expect("age period should build");
    let layer =
        build_age_horoscope_layer(&chart, &period).expect("age horoscope layer should build");

    let actual: HashMap<(StarName, EarthlyBranch), Mutagen> = layer
        .activations()
        .iter()
        .map(|activation| {
            assert_eq!(activation.source_scope(), Scope::Age);
            (
                (activation.target_star(), activation.target_branch()),
                activation.mutagen(),
            )
        })
        .collect();

    assert_eq!(actual, expected_mutagens());
}

#[test]
fn age_snapshot_exposes_temporal_palace_names_separately_from_natal() {
    let chart = fixture_chart();
    let period = build_age_period(&chart, NOMINAL_AGE).expect("age period should build");
    let layer =
        build_age_horoscope_layer(&chart, &period).expect("age horoscope layer should build");
    let horoscope = HoroscopeChart::with_layers(chart.clone(), vec![layer]);

    let snapshot = ChartStackSnapshot::from_horoscope_chart(&horoscope);

    assert_eq!(snapshot.layers().len(), 2);
    assert_eq!(snapshot.layers()[0].kind(), ChartLayerKind::Natal);

    let age = &snapshot.layers()[1];
    assert_eq!(age.kind(), ChartLayerKind::Age);
    assert_eq!(
        age.context(),
        Some(&TemporalContext::Age {
            stem_branch: expected_stem_branch(),
            nominal_age: NOMINAL_AGE,
        })
    );
    assert!(age.cells().iter().all(|cell| cell.typed_stars().is_empty()));
    assert!(
        age.cells()
            .iter()
            .all(|cell| cell.decorative_stars().is_empty())
    );
    assert!(
        age.cells()
            .iter()
            .all(|cell| cell.scoped_stars().is_empty())
    );

    let expected = expected_palace_names_by_branch();
    for cell in age.cells() {
        let natal_palace = chart
            .palaces()
            .iter()
            .find(|palace| palace.branch() == cell.branch())
            .expect("natal branch should have a palace");
        assert_eq!(cell.natal_palace_name(), Some(natal_palace.name()));
        assert_eq!(
            cell.temporal_palace_name(),
            expected.get(&cell.branch()).copied()
        );
    }

    let chen = age
        .cells()
        .iter()
        .find(|cell| cell.branch() == EarthlyBranch::Chen)
        .expect("Chen cell should exist");
    assert_ne!(chen.natal_palace_name(), chen.temporal_palace_name());
    assert_eq!(chen.temporal_palace_name(), Some(PalaceName::Life));
}

#[test]
fn age_horoscope_layer_round_trips_through_json() {
    let chart = fixture_chart();
    let period = build_age_period(&chart, NOMINAL_AGE).expect("age period should build");
    let layer =
        build_age_horoscope_layer(&chart, &period).expect("age horoscope layer should build");

    let encoded = serde_json::to_string(&layer).expect("age layer should serialize");
    let decoded: iztro::core::TemporalLayer =
        serde_json::from_str(&encoded).expect("age layer should deserialize");

    assert_eq!(decoded, layer);
    assert_eq!(decoded.scope(), Scope::Age);
    assert!(decoded.placements().is_empty());
    assert!(decoded.palace_layout().is_some());
}

#[test]
fn age_period_rejects_nominal_age_outside_supported_range() {
    let chart = fixture_chart();

    for value in [0, 121] {
        assert_eq!(
            build_age_period(&chart, value).unwrap_err(),
            ChartError::InvalidNominalAge { value }
        );
    }
}

fn assert_age_period_matches_fixture(period: &AgePeriod) {
    assert_eq!(period.nominal_age(), NOMINAL_AGE);
    assert_eq!(period.palace_branch(), EarthlyBranch::Chen);
    assert_eq!(period.stem_branch(), expected_stem_branch());

    let layout = period.palace_layout();
    assert_eq!(layout.scope(), Scope::Age);
    assert_eq!(layout.names().len(), 12);

    for name in layout.names() {
        let expected = expected_palace_names_by_branch();
        assert_eq!(
            Some(name.palace_name()),
            expected.get(&name.branch()).copied()
        );
    }

    assert_eq!(
        parse_key::<HeavenlyStem>("geng"),
        period.stem_branch().stem()
    );
    assert_eq!(
        parse_key::<EarthlyBranch>("chen"),
        period.stem_branch().branch()
    );
}
