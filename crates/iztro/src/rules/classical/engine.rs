//! Classical claim evaluation: corpus + predicates → filtered, sorted claims.
//!
//! [`evaluate_classical`] runs every corpus rule, collects emitted [`Claim`]s and
//! typed [`RuleDiagnostic`]s, applies request filters to the claims, and sorts
//! them deterministically. [`evaluate_classical_claims`] is the headline,
//! claims-only entry point.

use crate::core::Chart;
use crate::rules::classical::claim::{Claim, ClaimDomain, ClaimScope};
use crate::rules::classical::corpus::quan_shu_rules;
use crate::rules::classical::outcome::{ClaimEvaluation, RuleDiagnostic, RuleOutcome};
use crate::rules::classical::quan_shu;
use crate::rules::classical::rule::ClassicalRuleId;
use crate::rules::classical::source::ClassicalWork;
use crate::rules::classical::theme::{ClaimPolarity, ClaimTheme};

/// Filters controlling which claims [`evaluate_classical`] returns.
///
/// Every field is an allow-list: an empty vec imposes no constraint on that
/// dimension. A claim is returned only if it satisfies every non-empty filter.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClaimEvaluationRequest {
    /// Keep only claims in these domains.
    pub domains: Vec<ClaimDomain>,
    /// Keep only claims sharing at least one of these themes.
    pub themes: Vec<ClaimTheme>,
    /// Keep only claims with one of these polarities.
    pub polarities: Vec<ClaimPolarity>,
    /// Keep only claims citing one of these works.
    pub works: Vec<ClassicalWork>,
    /// Keep only claims produced by one of these rules.
    pub rule_ids: Vec<ClassicalRuleId>,
    /// Keep only claims in one of these scopes.
    pub scopes: Vec<ClaimScope>,
}

impl ClaimEvaluationRequest {
    /// Returns whether `claim` satisfies every non-empty filter.
    fn matches(&self, claim: &Claim) -> bool {
        let domain_ok = self.domains.is_empty() || self.domains.contains(&claim.domain);
        let polarity_ok = self.polarities.is_empty() || self.polarities.contains(&claim.polarity);
        let scope_ok = self.scopes.is_empty() || self.scopes.contains(&claim.scope);
        let rule_ok = self.rule_ids.is_empty() || self.rule_ids.contains(&claim.rule_id);
        let theme_ok =
            self.themes.is_empty() || claim.themes.iter().any(|t| self.themes.contains(t));
        let work_ok = self.works.is_empty()
            || claim
                .source_refs
                .iter()
                .any(|source| self.works.contains(&source.work));
        domain_ok && polarity_ok && scope_ok && rule_ok && theme_ok && work_ok
    }
}

/// Evaluates every corpus rule against `chart`, returning filtered claims and the
/// full set of typed diagnostics.
///
/// Diagnostics are returned in full (unfiltered): they describe rules, not claims,
/// and keeping every unsupported reason visible is the point of the diagnostics
/// channel.
pub fn evaluate_classical(chart: &Chart, request: &ClaimEvaluationRequest) -> ClaimEvaluation {
    let mut claims = Vec::new();
    let mut diagnostics = Vec::new();

    for rule in quan_shu_rules() {
        match quan_shu::evaluate(rule, chart) {
            RuleOutcome::Emitted(claim) => claims.push(*claim),
            RuleOutcome::NotApplicable => {}
            RuleOutcome::Unsupported(reason) => diagnostics.push(RuleDiagnostic {
                rule_id: rule.id.clone(),
                reason,
            }),
        }
    }

    claims.retain(|claim| request.matches(claim));
    sort_claims(&mut claims);

    ClaimEvaluation {
        claims,
        diagnostics,
    }
}

/// Headline entry point: evaluates classical rules and returns the claims only.
pub fn evaluate_classical_claims(chart: &Chart, request: &ClaimEvaluationRequest) -> Vec<Claim> {
    evaluate_classical(chart, request).claims
}

/// Sorts claims deterministically by `(scope, domain, rule_id, claim_key)`.
fn sort_claims(claims: &mut [Claim]) {
    claims.sort_by(|a, b| {
        a.scope
            .cmp(&b.scope)
            .then_with(|| a.domain.cmp(&b.domain))
            .then_with(|| a.rule_id.cmp(&b.rule_id))
            .then_with(|| a.claim_key.cmp(&b.claim_key))
    });
}
