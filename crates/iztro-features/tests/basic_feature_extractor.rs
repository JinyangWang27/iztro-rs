use std::collections::HashMap;

use iztro_core::{
    BirthContext, Brightness, CalendarDate, Chart, EarthlyBranch, Gender, HeavenlyStem, LunarDay,
    LunarMonth, MethodProfile, Mutagen, NatalChartWithMajorStarsInput, PalaceName, Scope,
    StarCategory, StarKind, StarName, build_natal_chart_with_major_stars,
};
use iztro_features::{
    BasicFeatureExtractor, ChartFeatures, Domain, FeatureExtractor, PalaceRelation,
    PalaceRelationKind, StarFeature, domain_for_palace,
};

/// Builds the iztro fixture chart (1990-05-17 chen female, lunar 四月廿三, 火六局,
/// 庚 year) with the fourteen major stars placed.
fn fixture_chart() -> Chart {
    build_natal_chart_with_major_stars(NatalChartWithMajorStarsInput::new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("iztro_major_stars_fixture"),
        LunarMonth::new(4).expect("month 4 should be valid"),
        LunarDay::new(23).expect("day 23 should be valid"),
        HeavenlyStem::Geng,
    ))
    .expect("natal chart with major stars should build for fixture input")
}

fn extract_fixture_features() -> ChartFeatures {
    BasicFeatureExtractor
        .extract(&fixture_chart())
        .expect("basic feature extraction should succeed")
}

fn find_star_feature(features: &ChartFeatures, star: StarName) -> &StarFeature {
    features
        .star_features()
        .iter()
        .find(|feature| feature.star() == star)
        .unwrap_or_else(|| panic!("expected a star feature for {star:?}"))
}

#[test]
fn basic_feature_extractor_preserves_source_profile() {
    let chart = fixture_chart();
    let features = BasicFeatureExtractor
        .extract(&chart)
        .expect("basic feature extraction should succeed");

    assert_eq!(features.source_profile_id(), chart.method_profile().id());
    assert_eq!(features.source_profile_id(), "iztro_major_stars_fixture");
}

#[test]
fn basic_feature_extractor_extracts_supported_palace_domains() {
    let features = extract_fixture_features();

    let by_palace: HashMap<PalaceName, Domain> = features
        .palace_features()
        .iter()
        .map(|feature| (feature.palace(), feature.domain()))
        .collect();

    assert_eq!(features.palace_features().len(), 5);
    assert_eq!(by_palace.get(&PalaceName::Life), Some(&Domain::Identity));
    assert_eq!(by_palace.get(&PalaceName::Career), Some(&Domain::Career));
    assert_eq!(by_palace.get(&PalaceName::Wealth), Some(&Domain::Wealth));
    assert_eq!(
        by_palace.get(&PalaceName::Spouse),
        Some(&Domain::Relationship)
    );
    assert_eq!(by_palace.get(&PalaceName::Health), Some(&Domain::Health));

    // Palaces without a supported domain produce no palace feature.
    assert!(!by_palace.contains_key(&PalaceName::Siblings));
    assert!(!by_palace.contains_key(&PalaceName::Parents));

    // Every palace feature carries the canonical mapping.
    for feature in features.palace_features() {
        assert_eq!(domain_for_palace(feature.palace()), Some(feature.domain()));
    }
}

#[test]
fn basic_feature_extractor_extracts_major_star_features() {
    let features = extract_fixture_features();

    // (star, palace, domain, brightness, mutagen) for the major stars sitting in
    // supported-domain palaces of the fixture chart.
    let expected: &[(StarName, PalaceName, Domain, Brightness, Option<Mutagen>)] = &[
        (
            StarName::TaiYang,
            PalaceName::Life,
            Domain::Identity,
            Brightness::Weak,
            Some(Mutagen::Lu),
        ),
        (
            StarName::TaiYin,
            PalaceName::Life,
            Domain::Identity,
            Brightness::Temple,
            Some(Mutagen::Ke),
        ),
        (
            StarName::TianLiang,
            PalaceName::Career,
            Domain::Career,
            Brightness::Trapped,
            None,
        ),
        (
            StarName::TianTong,
            PalaceName::Spouse,
            Domain::Relationship,
            Brightness::Temple,
            Some(Mutagen::Ji),
        ),
        (
            StarName::LianZhen,
            PalaceName::Health,
            Domain::Health,
            Brightness::Temple,
            None,
        ),
    ];

    assert_eq!(features.star_features().len(), expected.len());

    for &(star, palace, domain, brightness, mutagen) in expected {
        let feature = find_star_feature(&features, star);
        assert_eq!(feature.palace(), palace, "{star:?} palace");
        assert_eq!(feature.domain(), domain, "{star:?} domain");
        assert_eq!(feature.kind(), StarKind::Major, "{star:?} kind");
        assert_eq!(feature.category(), StarCategory::Major, "{star:?} category");
        assert_eq!(feature.brightness(), brightness, "{star:?} brightness");
        assert_eq!(feature.mutagen(), mutagen, "{star:?} mutagen");
        assert_eq!(feature.scope(), Scope::Natal, "{star:?} scope");
    }

    // No star feature is emitted for an unsupported-domain palace.
    for feature in features.star_features() {
        assert_eq!(
            domain_for_palace(feature.palace()),
            Some(feature.domain()),
            "{:?} sits in an unsupported-domain palace",
            feature.star()
        );
    }
}

#[test]
fn basic_feature_extractor_extracts_natal_mutagen_flows() {
    let features = extract_fixture_features();

    let flows: HashMap<StarName, (PalaceName, Mutagen, Scope)> = features
        .mutagen_flows()
        .iter()
        .map(|flow| {
            (
                flow.star(),
                (flow.source_palace(), flow.mutagen(), flow.scope()),
            )
        })
        .collect();

    assert_eq!(features.mutagen_flows().len(), 4);
    assert_eq!(
        flows.get(&StarName::TaiYang),
        Some(&(PalaceName::Life, Mutagen::Lu, Scope::Natal))
    );
    // WuQu sits in Siblings (no supported domain) yet still emits a flow.
    assert_eq!(
        flows.get(&StarName::WuQu),
        Some(&(PalaceName::Siblings, Mutagen::Quan, Scope::Natal))
    );
    assert_eq!(
        flows.get(&StarName::TaiYin),
        Some(&(PalaceName::Life, Mutagen::Ke, Scope::Natal))
    );
    assert_eq!(
        flows.get(&StarName::TianTong),
        Some(&(PalaceName::Spouse, Mutagen::Ji, Scope::Natal))
    );

    for flow in features.mutagen_flows() {
        assert_eq!(flow.scope(), Scope::Natal);
    }
}

#[test]
fn basic_feature_extractor_includes_palace_relations() {
    let features = extract_fixture_features();

    // Twelve palaces, five edge-level relations each.
    assert_eq!(features.relations().len(), 60);

    let life_edges: Vec<&PalaceRelation> = features
        .relations()
        .iter()
        .filter(|relation| relation.source() == PalaceName::Life)
        .collect();

    let opposite: Vec<PalaceName> = life_edges
        .iter()
        .filter(|edge| edge.kind() == PalaceRelationKind::Opposite)
        .map(|edge| edge.target())
        .collect();
    let triad: Vec<PalaceName> = life_edges
        .iter()
        .filter(|edge| edge.kind() == PalaceRelationKind::Triad)
        .map(|edge| edge.target())
        .collect();
    let adjacent: Vec<PalaceName> = life_edges
        .iter()
        .filter(|edge| edge.kind() == PalaceRelationKind::Adjacent)
        .map(|edge| edge.target())
        .collect();

    assert_eq!(opposite, vec![PalaceName::Migration]);
    assert_eq!(triad, vec![PalaceName::Wealth, PalaceName::Career]);
    assert_eq!(adjacent, vec![PalaceName::Parents, PalaceName::Siblings]);
}

#[test]
fn chart_features_round_trips_through_json() {
    let features = extract_fixture_features();

    let serialized = serde_json::to_string(&features).expect("features should serialize");
    let round_tripped: ChartFeatures =
        serde_json::from_str(&serialized).expect("features should deserialize");

    assert_eq!(round_tripped, features);
}
