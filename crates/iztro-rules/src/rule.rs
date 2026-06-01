use crate::{claim::SourceMetadata, condition::Condition, effect::Effect};
use iztro_features::Domain;
use serde::{Deserialize, Serialize};

/// Metadata for an auditable rule.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuleMetadata {
    id: String,
    domain: Domain,
    source: SourceMetadata,
}

impl RuleMetadata {
    /// Creates rule metadata.
    pub fn new(id: impl Into<String>, domain: Domain, source: SourceMetadata) -> Self {
        Self {
            id: id.into(),
            domain,
            source,
        }
    }

    /// Returns the rule identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the rule domain.
    pub const fn domain(&self) -> Domain {
        self.domain
    }

    /// Returns rule source metadata.
    pub const fn source(&self) -> &SourceMetadata {
        &self.source
    }
}

/// Placeholder rule shape composed from metadata, condition, and effect.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    metadata: RuleMetadata,
    condition: Condition,
    effect: Effect,
}

impl Rule {
    /// Creates a placeholder rule.
    pub const fn new(metadata: RuleMetadata, condition: Condition, effect: Effect) -> Self {
        Self {
            metadata,
            condition,
            effect,
        }
    }

    /// Returns rule metadata.
    pub const fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    /// Returns the placeholder condition.
    pub const fn condition(&self) -> &Condition {
        &self.condition
    }

    /// Returns the placeholder effect.
    pub const fn effect(&self) -> &Effect {
        &self.effect
    }
}
