//! 金灿光辉 — Tai Yang (太阳) alone guarding the Life palace at 午.
//!
//! Source-backed (定富局). 成格: the Life palace branch is 午, 太阳 sits in it, and
//! 太阳 is the only major star there.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{EarthlyBranch, PalaceName, Scope, StarKind, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::query::{
    effective_branch_of_palace, effective_star_in_palace, selected_stars_in_palace,
};

/// Detects 金灿光辉 and appends any detection to `out`.
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

/// 成格: 午宫命宫，太阳单守（唯一主星）。
fn detect_base_formation(ctx: &PatternContext<'_>, scope: Scope) -> Option<FormationMatch> {
    let branch = EarthlyBranch::Wu;
    if effective_branch_of_palace(ctx, scope, PalaceName::Life) != Some(branch) {
        return None;
    }
    let tai_yang = effective_star_in_palace(ctx, scope, branch, StarName::TaiYang)?;
    if selected_stars_in_palace(ctx, branch)
        .iter()
        .filter(|star| star.placement().kind() == StarKind::Major)
        .count()
        != 1
    {
        return None;
    }

    let star = tai_yang.placement().name();
    Some(FormationMatch {
        id: PatternId::JinCanGuangHui,
        scope,
        anchor: PatternAnchor::Palace(branch),
        involved_palaces: vec![branch],
        involved_stars: vec![star],
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarInPalace { star, branch }],
    })
}

/// 减力/破格: no weakening/breaker policy is modeled.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
