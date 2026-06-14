//! Structured rule and claim contracts for iztro-rs.

pub mod claim;
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
