use iztro_core::{
    BirthContext, CalendarDate, Chart, EarthlyBranch, Gender, HeavenlyStem, MethodProfile,
    PALACE_NAMES, Palace, PalaceName,
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
    let chart = Chart::try_new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        MethodProfile::placeholder("feature_test_profile"),
        PALACE_NAMES
            .iter()
            .copied()
            .enumerate()
            .map(|(index, palace)| {
                Palace::new(
                    palace,
                    EarthlyBranch::from_index(index),
                    HeavenlyStem::from_index(index),
                    Vec::new(),
                )
            })
            .collect(),
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
