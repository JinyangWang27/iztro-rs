//! 命无正曜 — no major star (正曜) in the Life palace.
//!
//! 成格: the selected Life palace contains no major star.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::PalaceName;
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::query::{
    selected_branch_of_palace, selected_major_star_count_in_palace,
};

/// Detects 命无正曜 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(base) = detect_base_formation(ctx, request) else {
        return;
    };
    emit::push_detection(out, base, IntegrityAssessment::fulfilled());
}

/// 成格: 命宫无主星坐命。
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    let scope = requested_selected_scope(ctx, request)?;
    let branch = selected_branch_of_palace(ctx, PalaceName::Life)?;
    if selected_major_star_count_in_palace(ctx, branch) != 0 {
        return None;
    }

    Some(FormationMatch {
        id: PatternId::MingWuZhengYao,
        scope,
        anchor: PatternAnchor::Palace(branch),
        involved_palaces: vec![branch],
        involved_stars: Vec::new(),
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::NoMajorStarInPalace { branch }],
    })
}
