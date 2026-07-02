//! 月生沧海 — Tai Yin (太阴) in the Property palace at 子.
//!
//! Source-backed (定贵局). 成格: the Property palace branch is 子 and 太阴 sits in it.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{EarthlyBranch, PalaceName, Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::query::{effective_branch_of_palace, effective_star_in_palace};

/// Detects 月生沧海 and appends any detection to `out`.
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

/// 成格: 子宫田宅宫，太阴坐守。
fn detect_base_formation(ctx: &PatternContext<'_>, scope: Scope) -> Option<FormationMatch> {
    let branch = EarthlyBranch::Zi;
    if effective_branch_of_palace(ctx, scope, PalaceName::Property) != Some(branch) {
        return None;
    }
    let tai_yin = effective_star_in_palace(ctx, scope, branch, StarName::TaiYin)?;

    let star = tai_yin.placement().name();
    Some(FormationMatch {
        id: PatternId::YueShengCangHai,
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
