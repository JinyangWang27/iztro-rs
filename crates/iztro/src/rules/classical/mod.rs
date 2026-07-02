//! Chinese-first classical rule engine.
//!
//! This module encodes Zi Wei Dou Shu rules as data-driven metadata
//! (`rule-corpus/`) paired with hand-coded predicates, producing structured,
//! evidence-backed [`Claim`]s:
//!
//! ```text
//! Chart facts
//!   -> predicates (reuse shared rules::query helpers)
//!   -> classical rule evaluation
//!   -> structured Claim[]
//!   -> [optional] localized rendering via iztro-i18n (claim_key)
//!   -> JSON export (serde)
//! ```
//!
//! # Design boundaries
//!
//! - **Chinese source text is canonical** for classical terminology
//!   ([`SourceRef::source_text_zh_hans`]); Rust enum/key identity is canonical for
//!   machine logic. Chinese strings are never used as logic keys.
//! - **Rule provenance is explicit.** QuanShu source rules and project
//!   pattern/格局 rules live in separate corpora but are evaluated through the same
//!   classical rule path.
//! - **Claims carry no prose.** Localized text is rendered by `iztro-i18n` from
//!   [`Claim::claim_key`]. This crate never depends on `iztro-i18n`.
//! - **Conservative emission.** A claim is emitted only when its condition matches
//!   on *modeled* chart facts. Rules whose condition is not yet modeled return a
//!   typed [`RuleOutcome::Unsupported`], surfaced as a visible [`RuleDiagnostic`].
//! - **Hybrid, not a DSL.** Rule metadata is data-driven; predicates are Rust. A
//!   fully generic condition DSL is intentionally deferred.

pub mod claim;
pub mod context;
pub mod corpus;
pub mod engine;
pub mod evaluator;
pub mod evidence;
pub mod hit_ref;
pub mod metadata;
pub mod outcome;
pub mod predicates;
pub mod rule;
pub(crate) mod scope_registry;
pub mod source;
pub mod source_hit;
pub mod theme;
pub mod view;
pub mod void;

pub use claim::{Claim, ClaimDomain, ClaimId, ClaimScope, ClaimStrength};
pub use context::ClassicalRuleContext;
pub use corpus::{classical_rules, pattern_rules, quan_shu_rules, rule_by_id};
pub use engine::{
    ClaimEvaluationRequest, DiagnosticMode, evaluate_classical, evaluate_classical_claims,
    evaluate_classical_in_context,
};
pub use evidence::{Evidence, EvidenceKind};
pub use hit_ref::ClassicalRuleHitRef;
pub use metadata::{ClassicalRuleMetadata, classical_rule_metadata};
pub use outcome::{ClaimEvaluation, RuleDiagnostic, RuleOutcome, UnsupportedReason};
pub use rule::{ClaimSpec, ClassicalRule, ClassicalRuleId, RuleSchool, RuleStatus};
pub use source::{ClassicalWork, SourceRef};
pub use source_hit::ClassicalSourceHit;
pub use theme::{ClaimPolarity, ClaimTheme};
pub use view::{
    ClassicalCorpusRuleView, ClassicalRulePanelRequest, ClassicalRulePanelSummary,
    ClassicalRulePanelView, classical_rule_panel_view, classical_rule_panel_view_in_context,
};
pub use void::{VoidKind, VoidPolicy};

#[cfg(test)]
mod tests {
    use super::claim::ClaimScope;
    use super::corpus::classical_rules;
    use super::metadata::classical_rule_metadata;
    use super::rule::ClassicalRuleId;
    use super::scope_registry::OVERLAY_AWARE_RULES;

    #[test]
    fn overlay_aware_rule_registry_matches_corpus_metadata() {
        for (rule_id, scopes) in OVERLAY_AWARE_RULES {
            assert!(
                scopes.iter().any(|scope| *scope != ClaimScope::Natal),
                "overlay-aware rule {rule_id} must include at least one non-natal scope",
            );
            assert!(
                classical_rules()
                    .iter()
                    .any(|rule| rule.id.as_str() == *rule_id),
                "overlay-aware rule {rule_id} is missing from the classical corpus",
            );
            let metadata = classical_rule_metadata(ClassicalRuleId::new(*rule_id))
                .unwrap_or_else(|| panic!("overlay-aware rule {rule_id} has no metadata"));
            assert_eq!(
                metadata.applicable_scopes, *scopes,
                "overlay-aware rule {rule_id} metadata scopes must match the registry",
            );
        }
    }

    #[test]
    fn only_overlay_aware_rules_advertise_non_natal_scopes() {
        for rule in classical_rules() {
            let metadata = classical_rule_metadata(rule.id.clone())
                .unwrap_or_else(|| panic!("rule {} has no metadata", rule.id));
            let has_non_natal = metadata
                .applicable_scopes
                .iter()
                .any(|scope| *scope != ClaimScope::Natal);
            let registered = OVERLAY_AWARE_RULES
                .iter()
                .any(|(rule_id, _)| *rule_id == rule.id.as_str());
            assert_eq!(
                has_non_natal, registered,
                "rule {} has inconsistent overlay-aware metadata registration",
                rule.id,
            );
        }
    }
}
