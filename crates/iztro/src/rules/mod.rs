//! Rule engine contracts for iztro-rs.
//!
//! Pattern rules live in [`pattern`] and recognize structured pattern facts
//! over already-computed chart state. Classical rules live in [`classical`] and
//! evaluate chart facts into source hits, claims, and diagnostics.

pub mod classical;
pub mod pattern;
pub mod query;

pub use classical::{
    Claim, ClaimDomain, ClaimEvaluation, ClaimEvaluationRequest, ClaimId, ClaimPolarity,
    ClaimScope, ClaimSpec, ClaimStrength, ClaimTheme, ClassicalRule, ClassicalRuleId,
    ClassicalSourceHit, ClassicalWork, DiagnosticMode, Evidence, EvidenceKind, RuleDiagnostic,
    RuleOutcome, RuleSchool, RuleStatus, SourceRef, UnsupportedReason, VoidKind, VoidPolicy,
    classical_rules, evaluate_classical, evaluate_classical_claims, pattern_rules, quan_shu_rules,
    rule_by_id,
};
