//! 月落亥宫 — Tai Yin (太阴) guarding the Life palace at 亥.
//!
//! Source-backed (定贵局). 成格: the Life palace branch is 亥 and 太阴 sits in it.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{EarthlyBranch, PalaceName, Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::query::{effective_branch_of_palace, effective_star_in_palace};

/// Detects 月落亥宫 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for &scope in &request.scopes {
        let Some(base) = detect_base_formation(ctx, scope) else {
            continue;
        };
        emit::push_detection(out, base, IntegrityAssessment::fulfilled());
    }
}

/// 成格: 亥宫命宫，太阴坐守。
fn detect_base_formation(ctx: &PatternContext<'_>, scope: Scope) -> Option<FormationMatch> {
    let branch = EarthlyBranch::Hai;
    if effective_branch_of_palace(ctx, scope, PalaceName::Life) != Some(branch) {
        return None;
    }
    let tai_yin = effective_star_in_palace(ctx, scope, branch, StarName::TaiYin)?;

    let star = tai_yin.placement().name();
    Some(FormationMatch {
        id: PatternId::YueLuoHaiGong,
        scope,
        anchor: PatternAnchor::Palace(branch),
        involved_palaces: vec![branch],
        involved_stars: vec![star],
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarInPalace { star, branch }],
    })
}
