//! Typed evaluation outcomes and diagnostics.
//!
//! Claims are only emitted when the underlying facts are explicitly modeled. When
//! a rule is encoded but its condition is not yet backed by a modeled chart fact
//! or a defined policy, the evaluator returns a typed [`RuleOutcome::Unsupported`]
//! carrying an [`UnsupportedReason`]. The engine surfaces these as
//! [`RuleDiagnostic`]s so the unsupported status is **typed and visible** rather
//! than silently swallowed.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::rules::classical::claim::Claim;
use crate::rules::classical::rule::ClassicalRuleId;

/// Why a rule could not be evaluated into a claim.
///
/// A closed, typed set: each reason names a specific modeling gap rather than a
/// free-text message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsupportedReason {
    /// The 禄/天马 "交驰" relation is school-dependent and is not yet modeled as a
    /// deterministic chart fact, so the rule cannot fire.
    LuMaRelationNotModeled,
}

impl fmt::Display for UnsupportedReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LuMaRelationNotModeled => {
                f.write_str("the Lu/Tian Ma (禄马交驰) relation is not modeled")
            }
        }
    }
}

/// The typed result of evaluating one classical rule against a chart.
#[derive(Clone, Debug, PartialEq)]
pub enum RuleOutcome {
    /// Facts were modeled and the condition matched: a claim was produced.
    Emitted(Box<Claim>),
    /// Facts were modeled but the condition did not match: no claim.
    NotApplicable,
    /// The rule is encoded but its condition is not yet supported.
    Unsupported(UnsupportedReason),
}

/// A typed, serializable diagnostic recording that a rule was not evaluated into a
/// claim because its condition is unsupported.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuleDiagnostic {
    /// The rule the diagnostic is about.
    pub rule_id: ClassicalRuleId,
    /// Why the rule is unsupported.
    pub reason: UnsupportedReason,
}

/// The full result of a classical evaluation: emitted claims plus typed
/// diagnostics for rules that were unsupported.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct ClaimEvaluation {
    /// Claims emitted by rules whose conditions matched on modeled facts.
    pub claims: Vec<Claim>,
    /// Diagnostics for rules that could not be evaluated (typed, visible).
    pub diagnostics: Vec<RuleDiagnostic>,
}
