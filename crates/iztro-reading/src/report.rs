use crate::section::ReadingSection;
use serde::{Deserialize, Serialize};

/// Deterministic report structure produced by the narrative layer.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadingReport {
    sections: Vec<ReadingSection>,
}

impl ReadingReport {
    /// Creates a reading report from sections.
    pub fn new(sections: Vec<ReadingSection>) -> Self {
        Self { sections }
    }

    /// Returns report sections.
    pub fn sections(&self) -> &[ReadingSection] {
        &self.sections
    }
}
