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
    rules::zuo_you_jia_ming::detect(ctx, request, &mut out);
    rules::chang_qu_jia_ming::detect(ctx, request, &mut out);
    rules::ri_yue_bing_ming::detect(ctx, request, &mut out);
    rules::ri_yue_fan_bei::detect(ctx, request, &mut out);
    rules::quan_shu_v01::detect(ctx, request, &mut out);

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

/// Returns whether a detection passes the request status, family, and scope
/// filters.
fn keep(detection: &PatternDetection, request: &PatternDetectionRequest) -> bool {
    let status_ok = match detection.status {
        PatternStatus::Fulfilled => true,
        PatternStatus::Partial => request.include_partial,
        PatternStatus::Weakened => request.include_weakened,
        PatternStatus::Broken => request.include_broken,
    };
    let family_ok = request.families.is_empty() || request.families.contains(&detection.family);
    let scope_ok = scope_allowed(&detection.scope, request);
    status_ok && family_ok && scope_ok
}

/// Returns whether a detection's scope is permitted by `request.scopes`.
///
/// An empty `request.scopes` permits nothing. A [`PatternScope::Combined`] is
/// permitted only when it spans at least one scope and every contained scope is
/// requested. An empty `Combined(vec![])` is never permitted: `Iterator::all`
/// over an empty set returns `true`, so the explicit `is_empty` guard is what
/// keeps a degenerate combined scope from matching.
fn scope_allowed(scope: &PatternScope, request: &PatternDetectionRequest) -> bool {
    if request.scopes.is_empty() {
        return false;
    }

    match scope {
        PatternScope::Natal => request.scopes.contains(&Scope::Natal),
        PatternScope::Decadal => request.scopes.contains(&Scope::Decadal),
        PatternScope::Age => request.scopes.contains(&Scope::Age),
        PatternScope::Yearly => request.scopes.contains(&Scope::Yearly),
        PatternScope::Monthly => request.scopes.contains(&Scope::Monthly),
        PatternScope::Daily => request.scopes.contains(&Scope::Daily),
        PatternScope::Hourly => request.scopes.contains(&Scope::Hourly),
        PatternScope::Combined(scopes) => {
            !scopes.is_empty() && scopes.iter().all(|scope| request.scopes.contains(scope))
        }
    }
}

/// Stable ordering rank for a single scope.
///
/// Mirrors the variant order used by [`scope_key`] so the two stay consistent:
/// `Natal, Decadal, Age, Yearly, Monthly, Daily, Hourly`.
fn scope_rank(scope: Scope) -> u8 {
    match scope {
        Scope::Natal => 0,
        Scope::Decadal => 1,
        Scope::Age => 2,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::pattern::model::PatternFamily;

    fn request_with(scopes: Vec<Scope>) -> PatternDetectionRequest {
        PatternDetectionRequest {
            scopes,
            include_partial: false,
            include_weakened: true,
            include_broken: true,
            families: Vec::<PatternFamily>::new(),
        }
    }

    #[test]
    fn empty_combined_scope_is_never_allowed() {
        // An empty Combined must not slip through via `Iterator::all` on an empty
        // set, even when the request asks for every scope.
        let request = request_with(vec![
            Scope::Natal,
            Scope::Decadal,
            Scope::Age,
            Scope::Yearly,
            Scope::Monthly,
            Scope::Daily,
            Scope::Hourly,
        ]);
        assert!(!scope_allowed(
            &PatternScope::Combined(Vec::new()),
            &request
        ));
    }

    #[test]
    fn combined_scope_requires_every_member_requested() {
        let combined = PatternScope::Combined(vec![Scope::Natal, Scope::Yearly]);

        // Both members requested: allowed.
        assert!(scope_allowed(
            &combined,
            &request_with(vec![Scope::Natal, Scope::Yearly]),
        ));

        // Missing one member: not allowed.
        assert!(!scope_allowed(&combined, &request_with(vec![Scope::Natal])));
    }

    #[test]
    fn empty_request_scopes_allow_nothing() {
        assert!(!scope_allowed(
            &PatternScope::Natal,
            &request_with(Vec::new())
        ));
        assert!(!scope_allowed(
            &PatternScope::Combined(vec![Scope::Natal]),
            &request_with(Vec::new()),
        ));
    }
}
