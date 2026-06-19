//! Top-level pattern detection entry point.
//!
//! Detection is read-only: it inspects chart facts and emits structured
//! [`PatternDetection`]s. It never mutates chart facts and never produces
//! narrative prose.

use crate::core::Scope;

use super::context::{PatternContext, PatternDetectionRequest};
use super::model::{PatternAnchor, PatternDetection, PatternScope, PatternStatus};
use super::rules;

/// Detects all supported patterns on a chart, honoring the request filters.
///
/// The returned vector is deterministically ordered by, in priority:
/// scope, family, id, anchor, then involved palaces.
pub fn detect_patterns(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Vec<PatternDetection> {
    let mut out = Vec::new();

    rules::zi_fu_chao_yuan::detect(ctx, request, &mut out);
    rules::ji_yue_tong_liang::detect(ctx, request, &mut out);
    rules::yang_tuo_jia_ji::detect(ctx, request, &mut out);

    filter_and_sort(out, request)
}

/// Drops detections excluded by the request and applies deterministic ordering.
fn filter_and_sort(
    mut detections: Vec<PatternDetection>,
    request: &PatternDetectionRequest,
) -> Vec<PatternDetection> {
    detections.retain(|detection| keep(detection, request));
    detections.sort_by(|a, b| {
        scope_key(&a.scope)
            .cmp(&scope_key(&b.scope))
            .then(a.family.cmp(&b.family))
            .then(a.id.cmp(&b.id))
            .then(anchor_key(&a.anchor).cmp(&anchor_key(&b.anchor)))
            .then_with(|| palaces_key(&a.involved_palaces).cmp(&palaces_key(&b.involved_palaces)))
    });
    detections
}

/// Returns whether a detection passes the request status and family filters.
fn keep(detection: &PatternDetection, request: &PatternDetectionRequest) -> bool {
    let status_ok = match detection.status {
        PatternStatus::Fulfilled => true,
        PatternStatus::Partial => request.include_partial,
        PatternStatus::Weakened => request.include_weakened,
        PatternStatus::Broken => request.include_broken,
    };
    let family_ok = request.families.is_empty() || request.families.contains(&detection.family);
    status_ok && family_ok
}

/// Stable ordering rank for a single scope.
fn scope_rank(scope: Scope) -> u8 {
    match scope {
        Scope::Natal => 0,
        Scope::Age => 1,
        Scope::Decadal => 2,
        Scope::Yearly => 3,
        Scope::Monthly => 4,
        Scope::Daily => 5,
        Scope::Hourly => 6,
    }
}

/// Stable ordering key for a pattern scope.
fn scope_key(scope: &PatternScope) -> (u8, Vec<u8>) {
    match scope {
        PatternScope::Natal => (0, Vec::new()),
        PatternScope::Decadal => (1, Vec::new()),
        PatternScope::Age => (2, Vec::new()),
        PatternScope::Yearly => (3, Vec::new()),
        PatternScope::Monthly => (4, Vec::new()),
        PatternScope::Daily => (5, Vec::new()),
        PatternScope::Hourly => (6, Vec::new()),
        PatternScope::Combined(scopes) => (7, scopes.iter().map(|s| scope_rank(*s)).collect()),
    }
}

/// Stable ordering key for a pattern anchor.
fn anchor_key(anchor: &PatternAnchor) -> (u8, usize) {
    match anchor {
        PatternAnchor::Palace(branch) => (0, branch.index()),
        PatternAnchor::Star(star) => (1, *star as usize),
        PatternAnchor::Mutagen(mutagen) => (2, *mutagen as usize),
        PatternAnchor::Chart => (3, 0),
    }
}

/// Stable ordering key for an ordered set of involved palaces.
fn palaces_key(branches: &[crate::core::EarthlyBranch]) -> Vec<usize> {
    branches.iter().map(|branch| branch.index()).collect()
}
