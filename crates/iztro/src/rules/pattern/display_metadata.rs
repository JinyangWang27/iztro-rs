//! Display/runtime metadata for canonical pattern detections.
//!
//! Static values live in the central pattern registry. This module preserves the
//! public display metadata type and lookup wrapper.

use crate::rules::pattern::model::{PatternFamily, PatternId, PatternPolarity};
use crate::rules::pattern::registry::pattern_spec;

/// Runtime/display metadata for one canonical pattern id.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PatternDisplayMetadata {
    /// Canonical pattern id this display metadata describes.
    pub pattern_id: PatternId,
    /// Runtime display name.
    pub name_zh: &'static str,
    /// Runtime display aliases.
    pub aliases_zh: &'static [&'static str],
    /// Coarse family used for filtering and display grouping.
    pub family: PatternFamily,
    /// Pattern valence used for filtering and display grouping.
    pub polarity: PatternPolarity,
    /// Normalized condition note for display/help surfaces.
    pub condition_note_zh_hans: &'static str,
    /// Optional source note for display surfaces.
    pub source_note_zh_hans: Option<&'static str>,
    /// Optional interpretation note for display surfaces.
    pub interpretation_note_zh_hans: Option<&'static str>,
}

/// Returns static display metadata for every canonical pattern id.
pub fn pattern_display_metadata(pattern_id: PatternId) -> &'static PatternDisplayMetadata {
    &pattern_spec(pattern_id).display
}
