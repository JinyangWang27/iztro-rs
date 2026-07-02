//! Reusable low-level chart predicates for pattern detection.
//!
//! Predicates **discover facts** about a chart (clamps, 三方四正 membership,
//! brightness, support sets, breaker stars). They never decide pattern-specific
//! meaning: a helper may find 空劫 in a 三方四正, but the named pattern decides
//! whether that means [`PatternStatus::Weakened`], [`PatternStatus::Broken`], or
//! no effect.
//!
//! These are thin, shared building blocks. Anything pattern-specific stays in the
//! named detector under [`crate::rules::pattern::patterns`].
//!
//! [`PatternStatus::Weakened`]: crate::rules::pattern::model::PatternStatus::Weakened
//! [`PatternStatus::Broken`]: crate::rules::pattern::model::PatternStatus::Broken

pub(crate) mod breakers;
pub(crate) mod brightness;
pub(crate) mod clamp;
pub(crate) mod sanfang;
pub(crate) mod support;

use crate::core::EarthlyBranch;

/// Sorts branches by their canonical index and removes duplicates in place.
///
/// Shared by detectors that assemble `involved_palaces` from several sources and
/// need a stable, duplicate-free branch list.
pub(crate) fn sort_dedup_branches(branches: &mut Vec<EarthlyBranch>) {
    branches.sort_by_key(|branch| branch.index());
    branches.dedup();
}
