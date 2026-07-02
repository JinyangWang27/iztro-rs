//! Shared 夹 (clamp) mechanics used by 左右夹命, 昌曲夹命, and 羊陀夹忌.
//!
//! These re-export the canonical clamp helpers so named clamp detectors depend on
//! one shared low-level source. The predicates only report which branches clamp a
//! target and which stars occupy them; the named pattern decides meaning.

pub(crate) use crate::rules::pattern::query::effective_clamp_pair_matches;
pub(crate) use crate::rules::relation::clamp_branches;
