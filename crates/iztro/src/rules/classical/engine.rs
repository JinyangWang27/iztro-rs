//! Classical claim evaluation: corpus + predicates → filtered, sorted claims.
//!
//! [`evaluate_classical`] runs every corpus rule, collects emitted [`Claim`]s and
//! typed [`RuleDiagnostic`]s, applies request filters to the claims, and sorts
//! them deterministically. [`evaluate_classical_claims`] is the headline,
//! claims-only entry point.

use crate::core::Chart;
use crate::rules::classical::claim::{Claim, ClaimDomain, ClaimScope};
use crate::rules::classical::corpus::classical_rules;
use crate::rules::classical::outcome::{ClaimEvaluation, RuleDiagnostic, RuleOutcome};
use crate::rules::classical::quan_shu;
use crate::rules::classical::rule::ClassicalRuleId;
use crate::rules::classical::source::ClassicalWork;
use crate::rules::classical::theme::{ClaimPolarity, ClaimTheme};

/// Controls whether unsupported-rule diagnostics are returned with an evaluation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DiagnosticMode {
    /// Return every unsupported diagnostic from corpus evaluation, independent of
    /// claim filters.
    #[default]
    AllUnsupported,
    /// Return only unsupported diagnostics whose rule metadata matches the
    /// request filters as far as metadata can decide.
    MatchingRequest,
    /// Suppress unsupported diagnostics.
    None,
}

/// Filters controlling which claims [`evaluate_classical`] returns.
///
/// Every field is an allow-list: an empty vec imposes no constraint on that
/// dimension. A claim is returned only if it satisfies every non-empty filter.
/// Diagnostics default to [`DiagnosticMode::AllUnsupported`] so unsupported
/// rules remain visible unless callers opt into filtering or suppression.
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
    /// Controls unsupported-rule diagnostics returned by [`evaluate_classical`].
    pub diagnostic_mode: DiagnosticMode,
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

    /// Returns whether `rule` satisfies request filters that can be applied to
    /// rule metadata before a claim exists.
    fn matches_rule_metadata(&self, rule: &crate::rules::classical::rule::ClassicalRule) -> bool {
        let domain_ok = self.domains.is_empty() || self.domains.contains(&rule.domain);
        let polarity_ok = self.polarities.is_empty() || self.polarities.contains(&rule.polarity);
        let scope_ok = self.scopes.is_empty() || self.scopes.contains(&ClaimScope::Natal);
        let rule_ok = self.rule_ids.is_empty() || self.rule_ids.contains(&rule.id);
        let theme_ok =
            self.themes.is_empty() || rule.themes.iter().any(|t| self.themes.contains(t));
        let work_ok = self.works.is_empty() || self.works.contains(&rule.work);
        domain_ok && polarity_ok && scope_ok && rule_ok && theme_ok && work_ok
    }

    /// Returns whether an unsupported diagnostic should be included for `rule`.
    fn includes_diagnostic(&self, rule: &crate::rules::classical::rule::ClassicalRule) -> bool {
        match self.diagnostic_mode {
            DiagnosticMode::AllUnsupported => true,
            DiagnosticMode::MatchingRequest => self.matches_rule_metadata(rule),
            DiagnosticMode::None => false,
        }
    }
}

/// Evaluates every corpus rule against `chart`, returning filtered claims and the
/// requested set of typed diagnostics.
///
/// By default diagnostics are returned in full (unfiltered): they describe
/// unsupported rules, not emitted claims. Callers can set
/// [`DiagnosticMode::MatchingRequest`] or [`DiagnosticMode::None`] for narrower
/// UI/export surfaces.
pub fn evaluate_classical(chart: &Chart, request: &ClaimEvaluationRequest) -> ClaimEvaluation {
    let mut claims = Vec::new();
    let mut diagnostics = Vec::new();

    for rule in classical_rules() {
        match quan_shu::evaluate(rule, chart) {
            RuleOutcome::Emitted(claim) => claims.push(*claim),
            RuleOutcome::NotApplicable => {}
            RuleOutcome::Unsupported(reason) => {
                if request.includes_diagnostic(rule) {
                    diagnostics.push(RuleDiagnostic {
                        rule_id: rule.id.clone(),
                        reason,
                    });
                }
            }
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
