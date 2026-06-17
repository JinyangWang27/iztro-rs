use iztro::features::{ChartFeatures, Domain};
use iztro::rules::{
    Claim, ClaimPolarity, Condition, Effect, EmptyRuleSetProvider, Evidence, Rule, RuleEngine,
    RuleEvaluationError, RuleEvaluator, RuleLoadError, RuleMetadata, RuleSetProvider,
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

#[test]
fn rule_engine_delegates_to_registered_evaluators() {
    let features = ChartFeatures::empty("rules_engine_test_profile");
    let engine = RuleEngine::new().with_evaluator(DummyEvaluator);

    let claims = engine
        .evaluate(&features)
        .expect("rule engine should delegate to evaluator");

    assert_eq!(claims.len(), 1);
    assert_eq!(claims[0].themes(), ["responsibility"]);
}

#[test]
fn empty_rule_set_provider_returns_no_rules() {
    let provider = EmptyRuleSetProvider;

    let rules = provider.load_rules().expect("empty provider should load");

    assert!(rules.is_empty());
}

#[test]
fn rule_shape_preserves_condition_and_effect() {
    let metadata = RuleMetadata::new(
        "career.placeholder",
        Domain::Career,
        SourceMetadata::new("seed", "scaffold"),
    );
    let effect = Effect::new(
        vec!["responsibility".to_owned(), "pressure".to_owned()],
        ClaimPolarity::MixedPositive,
        0.75,
    );
    let rule = Rule::new(metadata, Condition::Always, effect);

    assert_eq!(rule.metadata().id(), "career.placeholder");
    assert_eq!(rule.condition(), &Condition::Always);
    assert_eq!(rule.effect().themes(), ["responsibility", "pressure"]);
    assert_eq!(rule.effect().polarity(), ClaimPolarity::MixedPositive);
    assert_eq!(rule.effect().strength(), 0.75);
}

#[test]
fn scaffold_errors_have_stable_messages() {
    assert_eq!(
        RuleEvaluationError::NotImplemented.to_string(),
        "rule evaluation is not implemented"
    );
    assert_eq!(
        RuleLoadError::NotImplemented.to_string(),
        "rule loading is not implemented"
    );
}
