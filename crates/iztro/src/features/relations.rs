//! Feature-extraction relation scaffold over [`PalaceName`].
//!
//! These types describe cyclic relations between palaces keyed by their
//! **palace name** (frame-level identity), not by branch (地支) coordinate.
//! Branch-level rule/evidence relations are a separate vocabulary that lives in
//! [`crate::rules::relation`]; the two must not be conflated. The canonical
//! names here are prefixed `PalaceName*` to make the palace-name basis explicit;
//! the unprefixed `PalaceRelation*` names remain as compatibility aliases.

use crate::core::{PALACE_NAMES, PalaceName};
use serde::{Deserialize, Serialize};

/// Supported placeholder relation types between palaces (by palace name).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PalaceNameRelationKind {
    /// Opposite palace relation.
    Opposite,
    /// Triadic relation.
    Triad,
    /// Adjacent palace relation.
    Adjacent,
}

/// A relation between two palaces, keyed by palace name.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PalaceNameRelation {
    source: PalaceName,
    target: PalaceName,
    kind: PalaceNameRelationKind,
}

impl PalaceNameRelation {
    /// Creates a palace relation feature.
    pub const fn new(source: PalaceName, target: PalaceName, kind: PalaceNameRelationKind) -> Self {
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
    pub const fn kind(&self) -> PalaceNameRelationKind {
        self.kind
    }
}

/// Deterministic cyclic relations for one target palace (by palace name).
///
/// This aggregate uses the canonical twelve-palace order from `iztro-core`.
/// Relation offsets are fixed as:
///
/// - opposite palace: `+6`;
/// - triad palaces: `+4` and `+8`;
/// - adjacent palaces: `-1` and `+1`.
///
/// This is cyclic palace relation infrastructure only. It does not implement
/// full interpretive 三方四正 logic or emit narrative meaning.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PalaceNameRelations {
    target: PalaceName,
    opposite: PalaceName,
    triad: [PalaceName; 2],
    adjacent: [PalaceName; 2],
}

impl PalaceNameRelations {
    /// Creates deterministic cyclic relations for `target`.
    pub fn for_palace(target: PalaceName) -> Self {
        Self {
            target,
            opposite: target.offset(6),
            triad: [target.offset(4), target.offset(8)],
            adjacent: [target.offset(-1), target.offset(1)],
        }
    }

    /// Returns the target palace these relations are generated for.
    pub const fn target(&self) -> PalaceName {
        self.target
    }

    /// Returns the palace opposite the target palace.
    pub const fn opposite(&self) -> PalaceName {
        self.opposite
    }

    /// Returns the two triad palaces related to the target palace.
    pub const fn triad(&self) -> [PalaceName; 2] {
        self.triad
    }

    /// Returns the previous and next adjacent palaces around the target palace.
    pub const fn adjacent(&self) -> [PalaceName; 2] {
        self.adjacent
    }

    /// Returns edge-level relation features for this aggregate.
    pub const fn to_relations(&self) -> [PalaceNameRelation; 5] {
        [
            PalaceNameRelation::new(self.target, self.opposite, PalaceNameRelationKind::Opposite),
            PalaceNameRelation::new(self.target, self.triad[0], PalaceNameRelationKind::Triad),
            PalaceNameRelation::new(self.target, self.triad[1], PalaceNameRelationKind::Triad),
            PalaceNameRelation::new(
                self.target,
                self.adjacent[0],
                PalaceNameRelationKind::Adjacent,
            ),
            PalaceNameRelation::new(
                self.target,
                self.adjacent[1],
                PalaceNameRelationKind::Adjacent,
            ),
        ]
    }
}

/// Generates deterministic relation aggregates for all twelve palaces.
pub fn all_palace_relations() -> [PalaceNameRelations; 12] {
    PALACE_NAMES.map(PalaceNameRelations::for_palace)
}

/// Compatibility alias for the old feature relation kind name.
/// Canonical name: [`PalaceNameRelationKind`].
pub type PalaceRelationKind = PalaceNameRelationKind;

/// Compatibility alias for the old feature relation edge name.
/// Canonical name: [`PalaceNameRelation`].
pub type PalaceRelation = PalaceNameRelation;

/// Compatibility alias for the old feature relation aggregate name.
/// Canonical name: [`PalaceNameRelations`].
pub type PalaceRelations = PalaceNameRelations;
