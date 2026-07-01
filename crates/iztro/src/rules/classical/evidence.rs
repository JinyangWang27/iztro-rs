//! Machine-readable evidence for classical claims.
//!
//! Evidence is structured, never prose. Each [`EvidenceKind`] variant carries the
//! typed chart facts that triggered (or failed to support) a rule, so downstream
//! layers can explain a claim without parsing text.

use serde::{Deserialize, Serialize};

use crate::core::{Brightness, EarthlyBranch, Mutagen, StarName};
use crate::rules::classical::outcome::UnsupportedReason;
use crate::rules::classical::void::VoidKind;
use crate::rules::pattern::model::PatternId;
use crate::rules::pattern::relation::PalaceRelation;

/// A concrete, machine-checkable reason a classical rule matched.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    /// A star sits in a specific palace branch.
    StarInPalace {
        /// The star.
        star: StarName,
        /// The palace branch containing it.
        branch: EarthlyBranch,
    },
    /// A star occupies one of the two palaces clamping (夹) a target palace.
    StarClampsPalace {
        /// The clamping star.
        star: StarName,
        /// The branch the clamping star occupies.
        clamp_branch: EarthlyBranch,
        /// The target palace branch being clamped.
        target_branch: EarthlyBranch,
    },
    /// A star is affected by a modeled 空亡-family star in the same palace.
    StarAffectedByVoid {
        /// The affected star.
        star: StarName,
        /// The kind of void affecting it.
        void_kind: VoidKind,
        /// The shared palace branch.
        branch: EarthlyBranch,
    },
    /// A mutagen (四化) is active on a star in a palace.
    MutagenInPalace {
        /// The star carrying the mutagen.
        star: StarName,
        /// The mutagen.
        mutagen: Mutagen,
        /// The branch containing the star.
        branch: EarthlyBranch,
    },
    /// A star sits in a specific brightness (亮度) state.
    BrightnessCondition {
        /// The star.
        star: StarName,
        /// Its brightness state.
        brightness: Brightness,
        /// The branch containing the star.
        branch: EarthlyBranch,
    },
    /// Two palaces stand in a relation. `from` is the anchor/target palace, `to`
    /// is the related palace, and `relation` describes the relation of `to` to
    /// `from`.
    PalaceRelation {
        /// The anchor/target branch.
        from: EarthlyBranch,
        /// The related branch.
        to: EarthlyBranch,
        /// The relation of `to` to `from`.
        relation: PalaceRelation,
    },
    /// A structural shape matched a known 格局 pattern id.
    ///
    /// This does not imply [`crate::rules::pattern::detect_patterns`] was run; it
    /// records that the rule's own predicate matched the same chart shape.
    PatternShapeMatched {
        /// The corresponding pattern id.
        pattern: PatternId,
    },
    /// The rule's condition is not yet supported by modeled facts.
    UnsupportedCondition {
        /// The typed reason the condition is unsupported.
        reason: UnsupportedReason,
    },
}

/// A single structured evidence item supporting (or qualifying) a claim.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Evidence {
    /// The structured fact.
    pub kind: EvidenceKind,
}

impl Evidence {
    /// Wraps an [`EvidenceKind`] as an [`Evidence`] item.
    pub const fn new(kind: EvidenceKind) -> Self {
        Self { kind }
    }

    /// Returns the structured fact.
    pub const fn kind(&self) -> &EvidenceKind {
        &self.kind
    }
}

impl From<EvidenceKind> for Evidence {
    fn from(kind: EvidenceKind) -> Self {
        Self::new(kind)
    }
}
