//! Classical claim evaluation: corpus + predicates → filtered, sorted claims.
//!
//! [`evaluate_classical`] runs every corpus rule, collects emitted [`Claim`]s and
//! typed [`RuleDiagnostic`]s, applies request filters to the claims, and sorts
//! them deterministically. [`evaluate_classical_claims`] is the headline,
//! claims-only entry point.

use crate::core::Chart;
use crate::rules::classical::claim::{Claim, ClaimDomain, ClaimScope};
use crate::rules::classical::context::ClassicalRuleContext;
use crate::rules::classical::corpus::classical_rules;
use crate::rules::classical::evaluator;
use crate::rules::classical::metadata::classical_rule_metadata;
use crate::rules::classical::outcome::{ClaimEvaluation, RuleDiagnostic, RuleOutcome};
use crate::rules::classical::rule::{ClassicalRule, ClassicalRuleId};
use crate::rules::classical::source::ClassicalWork;
use crate::rules::classical::source_hit::ClassicalSourceHit;
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
    fn matches_rule_metadata(&self, rule: &ClassicalRule) -> bool {
        let claim_ok = match &rule.claim {
            Some(spec) => {
                let domain_ok = self.domains.is_empty() || self.domains.contains(&spec.domain);
                let polarity_ok =
                    self.polarities.is_empty() || self.polarities.contains(&spec.polarity);
                let theme_ok =
                    self.themes.is_empty() || spec.themes.iter().any(|t| self.themes.contains(t));
                domain_ok && polarity_ok && theme_ok
            }
            None => self.domains.is_empty() && self.polarities.is_empty() && self.themes.is_empty(),
        };
        let scope_ok = self.scopes.is_empty()
            || classical_rule_metadata(rule.id.clone()).is_some_and(|metadata| {
                metadata
                    .applicable_scopes
                    .iter()
                    .any(|scope| self.scopes.contains(scope))
            });
        let rule_ok = self.rule_ids.is_empty() || self.rule_ids.contains(&rule.id);
        let work_ok = self.works.is_empty() || self.works.contains(&rule.work);
        claim_ok && scope_ok && rule_ok && work_ok
    }

    /// Returns whether an unsupported diagnostic should be included for `rule`.
    fn includes_diagnostic(&self, rule: &ClassicalRule) -> bool {
        match self.diagnostic_mode {
            DiagnosticMode::AllUnsupported => true,
            DiagnosticMode::MatchingRequest => self.matches_rule_metadata(rule),
            DiagnosticMode::None => false,
        }
    }

    /// Returns whether `source_hit` satisfies the source-hit filters.
    fn matches_source_hit(&self, source_hit: &ClassicalSourceHit) -> bool {
        let work_ok = self.works.is_empty() || self.works.contains(&source_hit.work);
        let rule_ok = self.rule_ids.is_empty() || self.rule_ids.contains(&source_hit.rule_id);
        let scope_ok = self.scopes.is_empty() || self.scopes.contains(&source_hit.scope);
        work_ok && rule_ok && scope_ok
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
    evaluate_classical_in_context(&ClassicalRuleContext::natal(chart), request)
}

/// Context-oriented evaluation entry point.
///
/// This is the layer-ready evaluation API: it accepts a [`ClassicalRuleContext`]
/// carrying the chart, optional horoscope, and active scopes. Current executable
/// rules evaluate against the context's natal chart facts only, so for now this
/// produces the same result as [`evaluate_classical`]. Future temporal rules
/// should prefer ctx.effective() and ctx.selected_frame_scope() for
/// selected-state semantics, using ctx.horoscope_chart() / ctx.active_scopes()
/// only for explicitly source/layer-specific logic.
pub fn evaluate_classical_in_context(
    ctx: &ClassicalRuleContext<'_>,
    request: &ClaimEvaluationRequest,
) -> ClaimEvaluation {
    let mut claims = Vec::new();
    let mut source_hits = Vec::new();
    let mut diagnostics = Vec::new();

    for rule in classical_rules() {
        match evaluator::evaluate_in_context(rule, ctx) {
            RuleOutcome::Matched { source_hit, claim } => {
                source_hits.push(*source_hit);
                if let Some(claim) = claim {
                    claims.push(*claim);
                }
            }
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
    source_hits.retain(|source_hit| request.matches_source_hit(source_hit));
    sort_claims(&mut claims);
    sort_source_hits(&mut source_hits);

    ClaimEvaluation {
        claims,
        source_hits,
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

/// Sorts source hits deterministically by `(scope, work, source_id, source_clause_id, rule_id)`,
/// where `source_clause_id` is an optional legacy/pattern provenance discriminator.
fn sort_source_hits(source_hits: &mut [ClassicalSourceHit]) {
    source_hits.sort_by(|a, b| {
        a.scope
            .cmp(&b.scope)
            .then_with(|| a.work.cmp(&b.work))
            .then_with(|| a.source_id.cmp(&b.source_id))
            .then_with(|| a.source_clause_id.cmp(&b.source_clause_id))
            .then_with(|| a.rule_id.cmp(&b.rule_id))
    });
}

#[cfg(test)]
mod tests {
    use super::{ClaimEvaluationRequest, DiagnosticMode};
    use crate::rules::classical::claim::ClaimScope;
    use crate::rules::classical::corpus::rule_by_id;

    const CHANG_QU: &str = "life.chang_qu_clamp_life.literary_reputation";
    const LU_MA: &str = "fortune.lu_ma_jiao_chi.favorable_convergence";

    #[test]
    fn matching_request_scope_filter_uses_rule_metadata_applicable_scopes() {
        let yearly_request = ClaimEvaluationRequest {
            scopes: vec![ClaimScope::Yearly],
            diagnostic_mode: DiagnosticMode::MatchingRequest,
            ..Default::default()
        };

        let overlay_rule = rule_by_id(CHANG_QU).expect("overlay-aware rule");
        assert!(
            yearly_request.matches_rule_metadata(overlay_rule),
            "yearly requests should metadata-match rules that advertise yearly applicability",
        );

        let natal_only_rule = rule_by_id(LU_MA).expect("natal-only unsupported rule");
        assert!(
            !yearly_request.matches_rule_metadata(natal_only_rule),
            "purely yearly requests should not metadata-match natal-only rules",
        );
    }
}
