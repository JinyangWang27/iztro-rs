use crate::domains::Domain;
use iztro_core::PalaceName;
use serde::{Deserialize, Serialize};

/// Semantic feature attached to a palace.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PalaceFeature {
    palace: PalaceName,
    domain: Domain,
}

impl PalaceFeature {
    /// Creates a palace feature for a domain.
    pub const fn new(palace: PalaceName, domain: Domain) -> Self {
        Self { palace, domain }
    }

    /// Returns the source palace.
    pub const fn palace(&self) -> PalaceName {
        self.palace
    }

    /// Returns the semantic domain.
    pub const fn domain(&self) -> Domain {
        self.domain
    }
}
