use iztro_core::{Mutagen, PalaceName, Scope, StarName};
use serde::{Deserialize, Serialize};

/// Placeholder feature describing a mutagen flow from a star in a palace.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MutagenFlow {
    source_palace: PalaceName,
    star: StarName,
    mutagen: Mutagen,
    scope: Scope,
}

impl MutagenFlow {
    /// Creates a mutagen-flow feature.
    pub const fn new(
        source_palace: PalaceName,
        star: StarName,
        mutagen: Mutagen,
        scope: Scope,
    ) -> Self {
        Self {
            source_palace,
            star,
            mutagen,
            scope,
        }
    }

    /// Returns the source palace.
    pub const fn source_palace(&self) -> PalaceName {
        self.source_palace
    }

    /// Returns the source star.
    pub const fn star(&self) -> StarName {
        self.star
    }

    /// Returns the mutagen.
    pub const fn mutagen(&self) -> Mutagen {
        self.mutagen
    }

    /// Returns the flow scope.
    pub const fn scope(&self) -> Scope {
        self.scope
    }
}
