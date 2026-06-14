use iztro::features::{ChartFeatures, Domain};
use iztro::rules::{
    Claim, ClaimPolarity, Evidence, RuleEvaluationError, RuleEvaluator, RuleMetadata,
    SourceMetadata,
};

struct DummyEvaluator;

impl RuleEvaluator for DummyEvaluator {
    fn evaluate(&self, _features: &ChartFeatures) -> Result<Vec<Claim>, RuleEvaluationError> {
        Ok(vec![Claim::new(
            Domain::Career,
            vec!["responsibility".to_owned()],
            ClaimPolarity::MixedPositive,
            0.5,
            vec![Evidence::new("dummy.rule", "placeholder feature matched")],
            Vec::new(),
            SourceMetadata::new("dummy", "scaffold"),
        )])
    }
}

#[test]
fn dummy_rule_evaluator_emits_structured_claim() {
    let features = ChartFeatures::empty("rules_test_profile");
    let claims = DummyEvaluator
        .evaluate(&features)
        .expect("dummy evaluation should work");

    assert_eq!(claims.len(), 1);
    assert_eq!(claims[0].domain(), Domain::Career);
    assert_eq!(claims[0].themes(), ["responsibility"]);
    assert_eq!(claims[0].polarity(), ClaimPolarity::MixedPositive);
    assert_eq!(claims[0].evidence()[0].fact_key(), "dummy.rule");
    assert_eq!(claims[0].source().rule_set(), "dummy");
}

#[test]
fn rule_metadata_keeps_auditable_source_fields() {
    let metadata = RuleMetadata::new(
        "career.placeholder",
        Domain::Career,
        SourceMetadata::new("seed", "scaffold"),
    );

    assert_eq!(metadata.id(), "career.placeholder");
    assert_eq!(metadata.domain(), Domain::Career);
    assert_eq!(metadata.source().source_id(), "scaffold");
}
