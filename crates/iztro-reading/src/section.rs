use iztro_features::Domain;
use serde::{Deserialize, Serialize};

/// A deterministic report section derived from structured claims.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReadingSection {
    domain: Domain,
    title: String,
    body: String,
}

impl ReadingSection {
    /// Creates a report section.
    pub fn new(domain: Domain, title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            domain,
            title: title.into(),
            body: body.into(),
        }
    }

    /// Returns the section domain.
    pub const fn domain(&self) -> Domain {
        self.domain
    }

    /// Returns the section title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the section body.
    pub fn body(&self) -> &str {
        &self.body
    }
}
