use iztro_core::{
    BirthContext, CalendarDate, Chart, EarthlyBranch, Gender, HeavenlyStem, MethodProfile,
    PalaceName, StemBranch, build_empty_chart,
};
use iztro_features::{
    ChartFeatures, Domain, FeatureExtractionError, FeatureExtractor, PalaceFeature, PalaceRelation,
    PalaceRelationKind,
};

struct DummyExtractor;

impl FeatureExtractor for DummyExtractor {
    fn extract(&self, chart: &Chart) -> Result<ChartFeatures, FeatureExtractionError> {
        let palace = chart.palaces()[0].name();
        Ok(ChartFeatures::new(
            chart.method_profile().id(),
            vec![Domain::Identity],
            vec![PalaceFeature::new(palace, Domain::Identity)],
            Vec::new(),
            Vec::new(),
            vec![PalaceRelation::new(
                palace,
                PalaceName::Career,
                PalaceRelationKind::Triad,
            )],
        ))
    }
}

#[test]
fn dummy_extractor_can_emit_chart_features() {
    let chart = build_empty_chart(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu).expect("valid sexagenary pair"),
        MethodProfile::placeholder("feature_test_profile"),
    )
    .expect("twelve-palace scaffold chart should be valid");

    let features = DummyExtractor
        .extract(&chart)
        .expect("dummy extraction should work");

    assert_eq!(features.source_profile_id(), "feature_test_profile");
    assert_eq!(features.domains(), &[Domain::Identity]);
    assert_eq!(features.palace_features()[0].palace(), PalaceName::Life);
    assert_eq!(features.relations()[0].kind(), PalaceRelationKind::Triad);
}
