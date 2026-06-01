use iztro_core::PalaceName;
use serde::{Deserialize, Serialize};

/// Supported placeholder relation types between palaces.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PalaceRelationKind {
    /// Opposite palace relation.
    Opposite,
    /// Triadic relation.
    Triad,
    /// Adjacent palace relation.
    Adjacent,
}

/// A relation between two palaces.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PalaceRelation {
    source: PalaceName,
    target: PalaceName,
    kind: PalaceRelationKind,
}

impl PalaceRelation {
    /// Creates a palace relation feature.
    pub const fn new(source: PalaceName, target: PalaceName, kind: PalaceRelationKind) -> Self {
        Self {
            source,
            target,
            kind,
        }
    }

    /// Returns the source palace.
    pub const fn source(&self) -> PalaceName {
        self.source
    }

    /// Returns the target palace.
    pub const fn target(&self) -> PalaceName {
        self.target
    }

    /// Returns the relation kind.
    pub const fn kind(&self) -> PalaceRelationKind {
        self.kind
    }
}
