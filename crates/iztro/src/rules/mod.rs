//! Structured rule and claim contracts for iztro-rs.
//!
//! Two layers currently coexist here:
//!
//! - the original **placeholder scaffold** ([`claim`], [`condition`], [`effect`],
//!   [`engine`], [`loader`], [`rule`]) — a feature-oriented stub; and
//! - the **classical rule engine** ([`classical`]) — the Chinese-first,
//!   data-driven engine that turns chart facts into evidence-backed claims.
//!
//! The scaffold is transitional: it will be migrated into, or retired in favor of,
//! [`classical`] in a follow-up PR. They coexist for now so existing scaffold
//! tests keep passing.

pub mod claim;
pub mod classical;
pub mod condition;
pub mod effect;
pub mod engine;
pub mod loader;
pub mod rule;

pub use claim::{Claim, ClaimPolarity, Evidence, SourceMetadata};
pub use condition::Condition;
pub use effect::Effect;
pub use engine::{RuleEngine, RuleEvaluationError, RuleEvaluator};
pub use loader::{EmptyRuleSetProvider, RuleLoadError, RuleSetProvider};
pub use rule::{Rule, RuleMetadata};
