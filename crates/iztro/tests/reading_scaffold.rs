use iztro::reading::{PlaceholderRenderer, ReportRenderer};
use iztro::rules::classical::{
    Claim, ClaimDomain, ClaimId, ClaimPolarity, ClaimScope, ClaimStrength, ClaimTheme,
    ClassicalRuleId,
};

/// Builds a minimal classical claim for renderer tests.
fn sample_claim() -> Claim {
    let rule_id = ClassicalRuleId::new("career.placeholder");
    Claim {
        id: ClaimId::new(&rule_id, ClaimScope::Natal),
        rule_id,
        domain: ClaimDomain::Career,
        themes: vec![ClaimTheme::Responsibility],
        polarity: ClaimPolarity::MixedPositive,
        strength: ClaimStrength::new(0.5),
        scope: ClaimScope::Natal,
        evidence: Vec::new(),
        counter_evidence: Vec::new(),
        source_refs: Vec::new(),
        claim_key: "claim.career.placeholder".to_owned(),
    }
}

#[test]
fn placeholder_renderer_turns_classical_claims_into_report_sections() {
    let claims = vec![sample_claim()];

    let report = PlaceholderRenderer
        .render(&claims)
        .expect("placeholder rendering should work");

    assert_eq!(report.sections().len(), 1);
    assert_eq!(report.sections()[0].domain(), ClaimDomain::Career);
    assert_eq!(report.sections()[0].title(), "claim.career.placeholder");
    assert!(report.sections()[0].body().contains("Responsibility"));
}
