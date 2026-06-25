//! Chinese-first classical rule engine (《紫微斗数全书》 pilot).
//!
//! This module encodes classical Zi Wei Dou Shu rules as data-driven metadata
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
//! - **Claims carry no prose.** Localized text is rendered by `iztro-i18n` from
//!   [`Claim::claim_key`]. This crate never depends on `iztro-i18n`.
//! - **Conservative emission.** A claim is emitted only when its condition matches
//!   on *modeled* chart facts. Rules whose condition is not yet modeled return a
//!   typed [`RuleOutcome::Unsupported`], surfaced as a visible [`RuleDiagnostic`].
//! - **Hybrid, not a DSL.** Rule metadata is data-driven; predicates are Rust. A
//!   fully generic condition DSL is intentionally deferred.
//!
//! # Transitional status
//!
//! This module is a **transitional implementation slice, not a permanent second
//! rule engine**. The placeholder scaffold in [`crate::rules`] (the
//! feature-oriented `Claim`/`RuleEngine`/`Evidence` types) will be migrated into,
//! or retired in favor of, this classical engine in a follow-up PR. The two
//! coexist for now only so existing scaffold tests keep passing.

pub mod claim;
pub mod corpus;
pub mod engine;
pub mod evidence;
pub mod outcome;
pub mod predicates;
pub mod quan_shu;
pub mod rule;
pub mod source;
pub mod theme;
pub mod void;

pub use claim::{Claim, ClaimDomain, ClaimId, ClaimScope, ClaimStrength};
pub use corpus::{quan_shu_rules, rule_by_id};
pub use engine::{
    ClaimEvaluationRequest, DiagnosticMode, evaluate_classical, evaluate_classical_claims,
};
pub use evidence::{Evidence, EvidenceKind};
pub use outcome::{ClaimEvaluation, RuleDiagnostic, RuleOutcome, UnsupportedReason};
pub use rule::{ClassicalRule, ClassicalRuleId, RuleSchool, RuleStatus};
pub use source::{ClassicalWork, SourceRef};
pub use theme::{ClaimPolarity, ClaimTheme};
pub use void::{VoidKind, VoidPolicy};
