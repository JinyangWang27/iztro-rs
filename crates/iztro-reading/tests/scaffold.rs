use iztro_features::Domain;
use iztro_reading::{PlaceholderRenderer, ReportRenderer};
use iztro_rules::{Claim, ClaimPolarity, Evidence, SourceMetadata};

#[test]
fn placeholder_renderer_turns_claims_into_report_sections() {
    let claims = vec![Claim::new(
        Domain::Career,
        vec!["responsibility".to_owned()],
        ClaimPolarity::MixedPositive,
        0.5,
        vec![Evidence::new("dummy.rule", "placeholder feature matched")],
        Vec::new(),
        SourceMetadata::new("dummy", "scaffold"),
    )];

    let report = PlaceholderRenderer
        .render(&claims)
        .expect("placeholder rendering should work");

    assert_eq!(report.sections().len(), 1);
    assert_eq!(report.sections()[0].domain(), Domain::Career);
    assert!(report.sections()[0].body().contains("responsibility"));
}
