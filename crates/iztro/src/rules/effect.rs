use crate::rules::claim::ClaimPolarity;
use serde::{Deserialize, Serialize};

/// Placeholder effect shape for future claim emission.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Effect {
    themes: Vec<String>,
    polarity: ClaimPolarity,
    strength: f32,
}

impl Effect {
    /// Creates a placeholder rule effect.
    pub fn new(themes: Vec<String>, polarity: ClaimPolarity, strength: f32) -> Self {
        Self {
            themes,
            polarity,
            strength,
        }
    }

    /// Returns effect themes.
    pub fn themes(&self) -> &[String] {
        &self.themes
    }

    /// Returns effect polarity.
    pub const fn polarity(&self) -> ClaimPolarity {
        self.polarity
    }

    /// Returns effect strength.
    pub const fn strength(&self) -> f32 {
        self.strength
    }
}
