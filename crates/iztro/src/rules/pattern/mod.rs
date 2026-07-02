//! Pattern (格局) detection: a read-only analytical layer over chart facts.
//!
//! This module recognizes classical Zi Wei Dou Shu patterns (格局) as
//! **structured, explainable facts** over already-computed chart facts. It is an
//! analytical layer, not a narrative one:
//!
//! - detection is **read-only** — it never mutates [`Chart`], palaces, star
//!   placements, temporal layers, or mutagen activations;
//! - detections are **not** narrative readings — they carry structured evidence
//!   and conditions, never interpretive prose;
//! - temporal facts remain **overlays** — a temporal [`PatternScope`] never folds
//!   temporal placement into natal facts.
//!
//! The output is intended for downstream consumers (e.g. the GUI's
//! `HighlightProjection` or a future static pattern view), which render the structured
//! data without parsing prose.
//!
//! [`Chart`]: crate::core::Chart

pub mod context;
pub mod detector;
pub mod display_metadata;
pub mod metadata;
pub mod model;
pub(crate) mod patterns;
pub(crate) mod predicates;
pub mod query;
pub mod registry;
pub mod relation;

pub use context::{PatternContext, PatternDetectionRequest};
pub use detector::detect_patterns;
pub use display_metadata::{PatternDisplayMetadata, pattern_display_metadata};
pub use metadata::{PatternSourceGroup, PatternSourceMetadata, pattern_source_metadata};
pub use model::{
    PatternAnchor, PatternCondition, PatternDetection, PatternEvidence, PatternFamily, PatternId,
    PatternPolarity, PatternScope, PatternStatus, PatternStrength,
};
pub use registry::{PatternSpec, pattern_spec, pattern_specs, try_pattern_spec};
pub use relation::PalaceRelation;
