use crate::domains::Domain;
use iztro_core::{Brightness, Mutagen, PalaceName, Scope, StarCategory, StarKind, StarName};
use serde::{Deserialize, Serialize};

/// Factual feature derived from a star placement.
///
/// This preserves the deterministic placement facts (palace, star, fine star
/// type, brightness, birth-year mutagen, scope) together with the semantic
/// [`Domain`] of the palace the star occupies. It performs no interpretation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StarFeature {
    palace: PalaceName,
    star: StarName,
    kind: StarKind,
    brightness: Brightness,
    mutagen: Option<Mutagen>,
    scope: Scope,
    domain: Domain,
}

impl StarFeature {
    /// Creates a star feature from placement facts and a semantic domain.
    pub const fn new(
        palace: PalaceName,
        star: StarName,
        kind: StarKind,
        brightness: Brightness,
        mutagen: Option<Mutagen>,
        scope: Scope,
        domain: Domain,
    ) -> Self {
        Self {
            palace,
            star,
            kind,
            brightness,
            mutagen,
            scope,
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

    /// Returns the iztro-compatible fine star type.
    pub const fn kind(&self) -> StarKind {
        self.kind
    }

    /// Returns the coarse palace grouping derived from the fine star type.
    pub const fn category(&self) -> StarCategory {
        self.kind.category()
    }

    /// Returns the star brightness.
    pub const fn brightness(&self) -> Brightness {
        self.brightness
    }

    /// Returns the optional birth-year mutagen attached to this placement.
    pub const fn mutagen(&self) -> Option<Mutagen> {
        self.mutagen
    }

    /// Returns the scope of the underlying placement.
    pub const fn scope(&self) -> Scope {
        self.scope
    }

    /// Returns the semantic domain of the palace this star occupies.
    pub const fn domain(&self) -> Domain {
        self.domain
    }
}
