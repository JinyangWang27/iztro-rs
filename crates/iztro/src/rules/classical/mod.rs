//! Chinese-first classical rule engine.
//!
//! This module encodes Zi Wei Dou Shu rules as data-driven metadata
//! (`rule-corpus/`) paired with hand-coded predicates, producing structured,
//! evidence-backed [`Claim`]s:
//!
//! ```text
//! Chart facts
//!   -> predicates (reuse core/pattern query helpers)
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
pub mod corpus;
pub mod engine;
pub mod evaluator;
pub mod evidence;
pub mod outcome;
pub mod predicates;
pub mod rule;
pub mod source;
pub mod source_hit;
pub mod theme;
pub mod void;

pub use claim::{Claim, ClaimDomain, ClaimId, ClaimScope, ClaimStrength};
pub use corpus::{classical_rules, pattern_rules, quan_shu_rules, rule_by_id};
pub use engine::{
    ClaimEvaluationRequest, DiagnosticMode, evaluate_classical, evaluate_classical_claims,
};
pub use evidence::{Evidence, EvidenceKind};
pub use outcome::{ClaimEvaluation, RuleDiagnostic, RuleOutcome, UnsupportedReason};
pub use rule::{ClaimSpec, ClassicalRule, ClassicalRuleId, RuleSchool, RuleStatus};
pub use source::{ClassicalWork, SourceRef};
pub use source_hit::ClassicalSourceHit;
pub use theme::{ClaimPolarity, ClaimTheme};
pub use void::{VoidKind, VoidPolicy};
