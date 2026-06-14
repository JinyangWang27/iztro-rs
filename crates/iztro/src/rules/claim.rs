use crate::features::Domain;
use serde::{Deserialize, Serialize};

/// Polarity of an interpretive claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimPolarity {
    /// Supportive or favorable signal.
    Positive,
    /// Obstructive or unfavorable signal.
    Negative,
    /// Mixed signal with mostly supportive framing.
    MixedPositive,
    /// Mixed signal with mostly cautionary framing.
    MixedNegative,
    /// Neutral descriptive signal.
    Neutral,
}

/// Auditable source metadata for rules and claims.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SourceMetadata {
    rule_set: String,
    source_id: String,
}

impl SourceMetadata {
    /// Creates source metadata.
    pub fn new(rule_set: impl Into<String>, source_id: impl Into<String>) -> Self {
        Self {
            rule_set: rule_set.into(),
            source_id: source_id.into(),
        }
    }

    /// Returns the rule-set identifier.
    pub fn rule_set(&self) -> &str {
        &self.rule_set
    }

    /// Returns the source identifier.
    pub fn source_id(&self) -> &str {
        &self.source_id
    }
}

/// Chart or feature fact used as evidence for a claim.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Evidence {
    fact_key: String,
    summary: String,
}

impl Evidence {
    /// Creates an evidence item.
    pub fn new(fact_key: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            fact_key: fact_key.into(),
            summary: summary.into(),
        }
    }

    /// Returns the stable fact key.
    pub fn fact_key(&self) -> &str {
        &self.fact_key
    }

    /// Returns the evidence summary.
    pub fn summary(&self) -> &str {
        &self.summary
    }
}

/// Structured rule output consumed by the reading layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    domain: Domain,
    themes: Vec<String>,
    polarity: ClaimPolarity,
    strength: f32,
    evidence: Vec<Evidence>,
    counter_evidence: Vec<Evidence>,
    source: SourceMetadata,
}

impl Claim {
    /// Creates a structured claim.
    pub fn new(
        domain: Domain,
        themes: Vec<String>,
        polarity: ClaimPolarity,
        strength: f32,
        evidence: Vec<Evidence>,
        counter_evidence: Vec<Evidence>,
        source: SourceMetadata,
    ) -> Self {
        Self {
            domain,
            themes,
            polarity,
            strength,
            evidence,
            counter_evidence,
            source,
        }
    }

    /// Returns the claim domain.
    pub const fn domain(&self) -> Domain {
        self.domain
    }

    /// Returns claim themes.
    pub fn themes(&self) -> &[String] {
        &self.themes
    }

    /// Returns the claim polarity.
    pub const fn polarity(&self) -> ClaimPolarity {
        self.polarity
    }

    /// Returns the claim strength.
    pub const fn strength(&self) -> f32 {
        self.strength
    }

    /// Returns supporting evidence.
    pub fn evidence(&self) -> &[Evidence] {
        &self.evidence
    }

    /// Returns counter-evidence.
    pub fn counter_evidence(&self) -> &[Evidence] {
        &self.counter_evidence
    }

    /// Returns source metadata.
    pub const fn source(&self) -> &SourceMetadata {
        &self.source
    }
}
