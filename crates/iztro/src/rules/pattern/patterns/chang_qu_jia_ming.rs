//! 昌曲夹命 — Wen Chang (文昌) and Wen Qu (文曲) clamping the Life palace.
//!
//! 成格: the two palaces clamping (夹) the Life palace are occupied — one by 文昌
//! and the other by 文曲 (exact identities), in either orientation. Matching is
//! exact: the runtime flow 昌/曲 stars (流昌/流曲, 运昌/运曲, …) are independent
//! [`StarName`] identities and do **not** satisfy this classical pattern. A
//! selected temporal frame may make natal 文昌/文曲 visible in a temporal Life
//! palace's clamps, but a flow 昌/曲 star never stands in for the base star.
//!
//! TODO: if a distinct temporal-flow variant is wanted later, add an explicit
//! detector that queries the exact per-scope flow names (e.g. 流昌/流曲 for
//! [`Scope::Yearly`]) via [`StarFamily::member_in_scope`] — do not reintroduce
//! hidden base↔flow equivalence in the generic clamp helpers.
//!
//! [`StarFamily::member_in_scope`]: crate::core::StarFamily::member_in_scope
//! 减力/破格: no weakening/breaker policy is modeled, so integrity is always
//! fulfilled.

use crate::core::{PalaceName, Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::predicates::clamp::effective_clamp_pair_matches;
use crate::rules::pattern::query::effective_branch_of_palace;
use crate::rules::relation::PalaceRelation;

/// Detects 昌曲夹命 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for &scope in &request.scopes {
        let Some(base) = detect_base_formation(ctx, scope) else {
            continue;
        };
        let integrity = assess_integrity(ctx, &base);
        emit::push_detection(out, base, integrity);
    }
}

/// 成格: 文昌 and 文曲 occupy the two palaces clamping the Life palace.
fn detect_base_formation(ctx: &PatternContext<'_>, scope: Scope) -> Option<FormationMatch> {
    let anchor = effective_branch_of_palace(ctx, scope, PalaceName::Life)?;

    let [(low_star, low_branch), (high_star, high_branch)] =
        effective_clamp_pair_matches(ctx, scope, anchor, StarName::WenChang, StarName::WenQu)?;

    let mut involved_palaces = vec![anchor, low_branch, high_branch];
    involved_palaces.sort_by_key(|branch| branch.index());
    involved_palaces.dedup();

    Some(FormationMatch {
        id: PatternId::ChangQuJiaMing,
        scope,
        anchor: PatternAnchor::Palace(anchor),
        involved_palaces,
        involved_stars: vec![low_star, high_star],
        involved_mutagens: Vec::new(),
        evidence: vec![
            PatternEvidence::StarInPalace {
                star: low_star,
                branch: low_branch,
            },
            PatternEvidence::StarInPalace {
                star: high_star,
                branch: high_branch,
            },
            PatternEvidence::PalaceRelation {
                from: anchor,
                to: low_branch,
                relation: PalaceRelation::ClampedBy,
            },
            PatternEvidence::PalaceRelation {
                from: anchor,
                to: high_branch,
                relation: PalaceRelation::ClampedBy,
            },
        ],
    })
}

/// 减力/破格: no weakening/breaker policy is modeled.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
