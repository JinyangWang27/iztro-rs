//! Classical rule engine contracts for iztro-rs.
//!
//! The active rule engine lives in [`classical`]. It is the Chinese-first,
//! corpus-backed engine that evaluates chart facts into source hits, claims,
//! and diagnostics.

pub mod classical;

pub use classical::{
    Claim, ClaimDomain, ClaimEvaluation, ClaimEvaluationRequest, ClaimId, ClaimPolarity,
    ClaimScope, ClaimSpec, ClaimStrength, ClaimTheme, ClassicalRule, ClassicalRuleId,
    ClassicalSourceHit, ClassicalWork, DiagnosticMode, Evidence, EvidenceKind, RuleDiagnostic,
    RuleOutcome, RuleSchool, RuleStatus, SourceRef, UnsupportedReason, VoidKind, VoidPolicy,
    classical_rules, evaluate_classical, evaluate_classical_claims, pattern_rules, quan_shu_rules,
    rule_by_id,
};
