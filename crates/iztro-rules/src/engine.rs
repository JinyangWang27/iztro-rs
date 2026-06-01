use crate::claim::Claim;
use iztro_features::ChartFeatures;
use thiserror::Error;

/// Evaluates extracted features into structured claims.
pub trait RuleEvaluator {
    /// Evaluates features and returns structured claims.
    fn evaluate(&self, features: &ChartFeatures) -> Result<Vec<Claim>, RuleEvaluationError>;
}

/// Minimal rule-engine skeleton that delegates to registered evaluators.
#[derive(Default)]
pub struct RuleEngine {
    evaluators: Vec<Box<dyn RuleEvaluator>>,
}

impl RuleEngine {
    /// Creates an empty rule engine.
    pub const fn new() -> Self {
        Self {
            evaluators: Vec::new(),
        }
    }

    /// Adds an evaluator to this engine.
    pub fn with_evaluator(mut self, evaluator: impl RuleEvaluator + 'static) -> Self {
        self.evaluators.push(Box::new(evaluator));
        self
    }
}

impl RuleEvaluator for RuleEngine {
    fn evaluate(&self, features: &ChartFeatures) -> Result<Vec<Claim>, RuleEvaluationError> {
        let mut claims = Vec::new();
        for evaluator in &self.evaluators {
            claims.extend(evaluator.evaluate(features)?);
        }
        Ok(claims)
    }
}

/// Errors produced by rule evaluation.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum RuleEvaluationError {
    /// Rule evaluation has not been implemented.
    #[error("rule evaluation is not implemented")]
    NotImplemented,
}
