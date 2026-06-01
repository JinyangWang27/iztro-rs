use crate::domains::Domain;
use iztro_core::{PalaceName, StarName};
use serde::{Deserialize, Serialize};

/// Semantic feature derived from a star placement.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StarFeature {
    palace: PalaceName,
    star: StarName,
    domain: Domain,
}

impl StarFeature {
    /// Creates a star feature.
    pub const fn new(palace: PalaceName, star: StarName, domain: Domain) -> Self {
        Self {
            palace,
            star,
            domain,
        }
    }

    /// Returns the source palace.
    pub const fn palace(&self) -> PalaceName {
        self.palace
    }

    /// Returns the source star.
    pub const fn star(&self) -> StarName {
        self.star
    }

    /// Returns the semantic domain.
    pub const fn domain(&self) -> Domain {
        self.domain
    }
}
