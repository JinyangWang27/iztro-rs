//! 紫府夹命 — Zi Wei (紫微) and Tian Fu (天府) clamping the Life palace.
//!
//! Source-backed (卷三·论诸星同垣). 成格: the two palaces clamping (夹) the selected
//! Life palace are occupied — one by 紫微 and the other by 天府, in either
//! orientation. The source phrase (紫府夹命为贵格) only requires the clamp; no extra
//! support is required in this first implementation.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{PalaceName, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::predicates::clamp::selected_clamp_pair_matches;
use crate::rules::pattern::predicates::sort_dedup_branches;
use crate::rules::pattern::query::selected_branch_of_palace;
use crate::rules::relation::PalaceRelation;

/// Detects 紫府夹命 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(base) = detect_base_formation(ctx, request) else {
        return;
    };
    let integrity = assess_integrity(ctx, &base);
    emit::push_detection(out, base, integrity);
}

/// 成格: 紫微 and 天府 occupy the two palaces clamping the Life palace.
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    let scope = requested_selected_scope(ctx, request)?;
    let anchor = selected_branch_of_palace(ctx, PalaceName::Life)?;

    let [(low_star, low_branch), (high_star, high_branch)] =
        selected_clamp_pair_matches(ctx, anchor, StarName::ZiWei, StarName::TianFu)?;

    let mut involved_palaces = vec![anchor, low_branch, high_branch];
    sort_dedup_branches(&mut involved_palaces);

    Some(FormationMatch {
        id: PatternId::ZiFuJiaMing,
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
