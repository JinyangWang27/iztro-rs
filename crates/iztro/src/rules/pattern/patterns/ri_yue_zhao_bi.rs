//! 日月照璧 — Tai Yang (太阳) and Tai Yin (太阴) together in the Property palace.
//!
//! Source-backed (定富局). 成格: 太阳 and 太阴 both sit in the Property palace.
//! 减力/破格: the source's enhancer 喜居墓库 (辰戌丑未) is not modeled; integrity is
//! always fulfilled.

use crate::core::{PalaceName, Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::query::{effective_branch_of_palace, effective_star_in_palace};

/// Detects 日月照璧 and appends any detection to `out`.
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

/// 成格: 太阳、太阴同临田宅宫。
fn detect_base_formation(ctx: &PatternContext<'_>, scope: Scope) -> Option<FormationMatch> {
    let branch = effective_branch_of_palace(ctx, scope, PalaceName::Property)?;
    let tai_yang = effective_star_in_palace(ctx, scope, branch, StarName::TaiYang)?;
    let tai_yin = effective_star_in_palace(ctx, scope, branch, StarName::TaiYin)?;

    let stars = vec![tai_yang.placement().name(), tai_yin.placement().name()];
    Some(FormationMatch {
        id: PatternId::RiYueZhaoBi,
        scope,
        anchor: PatternAnchor::Palace(branch),
        involved_palaces: vec![branch],
        involved_stars: stars.clone(),
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarsInSamePalace { stars, branch }],
    })
}

/// 减力/破格: the 喜居墓库 enhancer is not modeled as a weakening/breaker policy.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
