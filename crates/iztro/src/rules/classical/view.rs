//! Renderer-neutral classical rule panel view model.
//!
//! [`evaluate_classical`] is the low-level evaluation API: it returns the
//! interpreted [`Claim`]s, the matched [`ClassicalSourceHit`]s, and the typed
//! [`RuleDiagnostic`]s for a chart. This module adds the GUI/renderer-facing
//! grouping on top of it.
//!
//! [`classical_rule_panel_view`] bundles one evaluation together with optional
//! corpus metadata into a single [`ClassicalRulePanelView`] a frontend can render
//! directly. It deliberately **preserves** the architecture's separation between:
//!
//! - interpreted claims ([`ClassicalRulePanelView::claims`]),
//! - matched source/provenance hits ([`ClassicalRulePanelView::source_hits`]),
//! - unsupported-rule diagnostics ([`ClassicalRulePanelView::diagnostics`]),
//! - corpus rule metadata for display/filtering
//!   ([`ClassicalRulePanelView::corpus_rules`]).
//!
//! Claims and source hits are never merged into a single card model: a rule that
//! matches but has no claim metadata still appears through `source_hits`, and the
//! interpreted/provenance split stays reversible.
//!
//! As with the rest of `iztro`, this module emits no localized prose. Chinese
//! source text is canonical; localized rendering of claims/domains/themes is the
//! job of `iztro-i18n` keyed off `claim_key` and the typed enums.

use serde::{Deserialize, Serialize};

use crate::core::Chart;
use crate::rules::classical::context::ClassicalRuleContext;
use crate::rules::classical::corpus::classical_rules;
use crate::rules::classical::engine::{
    ClaimEvaluationRequest, DiagnosticMode, evaluate_classical_in_context,
};
use crate::rules::classical::{
    Claim, ClaimDomain, ClaimPolarity, ClaimScope, ClaimTheme, ClassicalRule, ClassicalRuleId,
    ClassicalSourceHit, ClassicalWork, RuleDiagnostic, RuleSchool, RuleStatus,
};

/// A renderer-neutral grouping of one classical evaluation plus corpus metadata.
///
/// Claims, source hits, diagnostics, and corpus rules are kept as separate
/// vectors so a frontend can render (or filter) each facet independently without
/// having to re-assemble rule/corpus semantics itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassicalRulePanelView {
    /// Aggregate counts for headers/badges.
    pub summary: ClassicalRulePanelSummary,
    /// Interpreted claims emitted by matching rules with claim metadata.
    pub claims: Vec<Claim>,
    /// Source/provenance hits for every matching executable rule, including those
    /// without claim metadata.
    pub source_hits: Vec<ClassicalSourceHit>,
    /// Typed diagnostics for rules whose condition is not yet supported.
    pub diagnostics: Vec<RuleDiagnostic>,
    /// Corpus rule metadata for display/filtering (not evaluation output).
    pub corpus_rules: Vec<ClassicalCorpusRuleView>,
}

/// Aggregate counts describing a [`ClassicalRulePanelView`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClassicalRulePanelSummary {
    /// Number of interpreted claims.
    pub claim_count: usize,
    /// Number of matched source hits.
    pub source_hit_count: usize,
    /// Number of unsupported-rule diagnostics.
    pub diagnostic_count: usize,
    /// Number of corpus rules surfaced for display/filtering.
    pub corpus_rule_count: usize,
}

/// Display/filtering metadata for one corpus rule.
///
/// This is metadata *about* a rule (its source, status, and claim shape), not an
/// evaluation result. A frontend can use it to build corpus tabs (Executable /
/// Normalized / Ambiguous) or to show the full provenance of a rule that did or
/// did not fire.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassicalCorpusRuleView {
    /// Stable rule identifier.
    pub rule_id: ClassicalRuleId,
    /// The classical work the rule is drawn from.
    pub work: ClassicalWork,
    /// Stable identifier for the source unit or pattern metadata entry.
    pub source_id: String,
    /// Optional legacy/pattern provenance discriminator.
    pub source_clause_id: Option<String>,
    /// Canonical classical text, Simplified Chinese.
    pub source_text_zh_hans: String,
    /// Optional normalization note, Simplified Chinese.
    pub normalized_note_zh_hans: Option<String>,
    /// Encoding maturity.
    pub status: RuleStatus,
    /// Interpretive school.
    pub school: RuleSchool,
    /// Whether the rule carries claim metadata.
    pub has_claim: bool,
    /// The i18n key for the produced claim, when the rule has claim metadata.
    pub claim_key: Option<String>,
    /// The claim domain, when the rule has claim metadata.
    pub domain: Option<ClaimDomain>,
    /// The claim themes, when the rule has claim metadata (empty otherwise).
    pub themes: Vec<ClaimTheme>,
    /// The claim polarity, when the rule has claim metadata.
    pub polarity: Option<ClaimPolarity>,
}

impl ClassicalCorpusRuleView {
    /// Builds a corpus view entry from a corpus rule's metadata.
    fn from_rule(rule: &ClassicalRule) -> Self {
        let claim = rule.claim.as_ref();
        Self {
            rule_id: rule.id.clone(),
            work: rule.work,
            source_id: rule.source_id.clone(),
            source_clause_id: rule.source_clause_id.clone(),
            source_text_zh_hans: rule.source_text_zh_hans.clone(),
            normalized_note_zh_hans: rule.normalized_note_zh_hans.clone(),
            status: rule.status,
            school: rule.school,
            has_claim: claim.is_some(),
            claim_key: claim.map(|spec| spec.claim_key.clone()),
            domain: claim.map(|spec| spec.domain),
            themes: claim.map(|spec| spec.themes.clone()).unwrap_or_default(),
            polarity: claim.map(|spec| spec.polarity),
        }
    }
}

/// A request for a [`ClassicalRulePanelView`].
///
/// Wraps a low-level [`ClaimEvaluationRequest`] (which drives claim/source-hit
/// filtering and diagnostic visibility) and adds corpus-panel controls.
#[derive(Clone, Debug, PartialEq)]
pub struct ClassicalRulePanelRequest {
    /// The underlying claim evaluation request.
    pub evaluation: ClaimEvaluationRequest,
    /// Whether to include corpus rule metadata in the panel.
    pub include_corpus: bool,
    /// Restrict corpus rules to these statuses. Empty imposes no constraint.
    pub corpus_statuses: Vec<RuleStatus>,
}

impl Default for ClassicalRulePanelRequest {
    fn default() -> Self {
        Self::user_facing()
    }
}

impl ClassicalRulePanelRequest {
    /// A user-facing panel: corpus included, unsupported diagnostics hidden.
    ///
    /// GUIs should not surface unsupported-rule diagnostics to end users unless a
    /// developer/debug mode explicitly asks for them.
    pub fn user_facing() -> Self {
        Self {
            evaluation: ClaimEvaluationRequest {
                diagnostic_mode: DiagnosticMode::None,
                ..Default::default()
            },
            include_corpus: true,
            corpus_statuses: Vec::new(),
        }
    }

    /// A developer panel: corpus included, all unsupported diagnostics surfaced.
    pub fn developer() -> Self {
        Self {
            evaluation: ClaimEvaluationRequest {
                diagnostic_mode: DiagnosticMode::AllUnsupported,
                ..Default::default()
            },
            include_corpus: true,
            corpus_statuses: Vec::new(),
        }
    }

    /// Restricts corpus rules to the given statuses.
    pub fn with_corpus_statuses(mut self, statuses: impl Into<Vec<RuleStatus>>) -> Self {
        self.corpus_statuses = statuses.into();
        self
    }

    /// Omits corpus rule metadata from the panel.
    pub fn without_corpus(mut self) -> Self {
        self.include_corpus = false;
        self
    }
}

/// Builds a renderer-neutral classical rule panel for `chart`.
///
/// This is the GUI/renderer-facing grouping API. It runs one
/// [`evaluate_classical`] pass and, when requested, attaches filtered corpus rule
/// metadata. The claim/source-hit/diagnostic split from the underlying evaluation
/// is preserved verbatim.
pub fn classical_rule_panel_view(
    chart: &Chart,
    request: &ClassicalRulePanelRequest,
) -> ClassicalRulePanelView {
    classical_rule_panel_view_in_context(&ClassicalRuleContext::natal(chart), request)
}

/// Builds a classical rule panel for an explicit [`ClassicalRuleContext`].
///
/// This is the layer-ready panel API. [`classical_rule_panel_view`] is the
/// natal-only compatibility wrapper over it. As with the engine, current
/// executable rules evaluate against natal facts only, so a horoscope context
/// produces the same panel as a natal context until temporal rules exist.
pub fn classical_rule_panel_view_in_context(
    ctx: &ClassicalRuleContext<'_>,
    request: &ClassicalRulePanelRequest,
) -> ClassicalRulePanelView {
    let evaluation = evaluate_classical_in_context(ctx, &request.evaluation);

    let corpus_rules = if request.include_corpus {
        build_corpus_rules(&request.evaluation, &request.corpus_statuses)
    } else {
        Vec::new()
    };

    let summary = ClassicalRulePanelSummary {
        claim_count: evaluation.claims.len(),
        source_hit_count: evaluation.source_hits.len(),
        diagnostic_count: evaluation.diagnostics.len(),
        corpus_rule_count: corpus_rules.len(),
    };

    ClassicalRulePanelView {
        summary,
        claims: evaluation.claims,
        source_hits: evaluation.source_hits,
        diagnostics: evaluation.diagnostics,
        corpus_rules,
    }
}

/// Builds the filtered, deterministically sorted corpus view.
///
/// Filtering mirrors [`ClaimEvaluationRequest`] semantics (every non-empty filter
/// is an allow-list) but operates on rule *metadata*. Status, work, and rule-id
/// filters always apply. Domain/theme/polarity filters apply to a rule's
/// `[rule.claim]` metadata when present; rules without claim metadata are kept
/// only when those three filters are all empty. The scope filter is applied the
/// same conservative way as [`evaluate_classical`]'s rule-metadata path: current
/// corpus rules are natal metadata, so a non-empty `scopes` filter that does not
/// include [`ClaimScope::Natal`] drops every corpus entry, keeping `corpus_rules`
/// consistent with the engine-filtered `claims`/`source_hits`.
fn build_corpus_rules(
    evaluation: &ClaimEvaluationRequest,
    corpus_statuses: &[RuleStatus],
) -> Vec<ClassicalCorpusRuleView> {
    let mut corpus_rules: Vec<ClassicalCorpusRuleView> = classical_rules()
        .iter()
        .filter(|_| corpus_matches_scope_filters(evaluation))
        .filter(|rule| corpus_statuses.is_empty() || corpus_statuses.contains(&rule.status))
        .filter(|rule| evaluation.works.is_empty() || evaluation.works.contains(&rule.work))
        .filter(|rule| evaluation.rule_ids.is_empty() || evaluation.rule_ids.contains(&rule.id))
        .filter(|rule| corpus_matches_claim_filters(evaluation, rule))
        .map(ClassicalCorpusRuleView::from_rule)
        .collect();

    corpus_rules.sort_by(|a, b| {
        a.work
            .cmp(&b.work)
            .then_with(|| a.source_id.cmp(&b.source_id))
            .then_with(|| a.source_clause_id.cmp(&b.source_clause_id))
            .then_with(|| a.rule_id.cmp(&b.rule_id))
    });

    corpus_rules
}

/// Whether `rule` satisfies the domain/theme/polarity claim filters.
///
/// For a rule with claim metadata, every non-empty filter must match. For a rule
/// without claim metadata, it is kept only when all three filters are empty (it
/// has no domain/theme/polarity to match against).
fn corpus_matches_claim_filters(evaluation: &ClaimEvaluationRequest, rule: &ClassicalRule) -> bool {
    match &rule.claim {
        Some(spec) => {
            let domain_ok =
                evaluation.domains.is_empty() || evaluation.domains.contains(&spec.domain);
            let polarity_ok =
                evaluation.polarities.is_empty() || evaluation.polarities.contains(&spec.polarity);
            let theme_ok = evaluation.themes.is_empty()
                || spec.themes.iter().any(|t| evaluation.themes.contains(t));
            domain_ok && polarity_ok && theme_ok
        }
        None => {
            evaluation.domains.is_empty()
                && evaluation.themes.is_empty()
                && evaluation.polarities.is_empty()
        }
    }
}

/// Whether the corpus passes the scope filter.
///
/// Current corpus rules are natal metadata, mirroring `evaluate_classical`'s
/// rule-metadata scope check: an empty `scopes` filter imposes no constraint, and
/// a non-empty one passes only when it includes [`ClaimScope::Natal`].
fn corpus_matches_scope_filters(evaluation: &ClaimEvaluationRequest) -> bool {
    evaluation.scopes.is_empty() || evaluation.scopes.contains(&ClaimScope::Natal)
}
