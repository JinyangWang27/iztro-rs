use crate::rule::Rule;
use thiserror::Error;

/// Provides rule definitions to a rule engine.
pub trait RuleSetProvider {
    /// Loads rules from the provider.
    fn load_rules(&self) -> Result<Vec<Rule>, RuleLoadError>;
}

/// Placeholder provider that returns no rules.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmptyRuleSetProvider;

impl RuleSetProvider for EmptyRuleSetProvider {
    fn load_rules(&self) -> Result<Vec<Rule>, RuleLoadError> {
        Ok(Vec::new())
    }
}

/// Errors produced by rule loading.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum RuleLoadError {
    /// Rule loading has not been implemented for this provider.
    #[error("rule loading is not implemented")]
    NotImplemented,
}
