//! 武曲守垣 — Wu Qu (武曲) guarding the Life palace at 卯.
//!
//! Source-backed (定贵局). 成格: the Life palace branch is 卯 and 武曲 sits in it.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{EarthlyBranch, PalaceName, Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::query::{effective_branch_of_palace, effective_star_in_palace};

/// Detects 武曲守垣 and appends any detection to `out`.
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

/// 成格: 卯宫命宫，武曲坐守。
fn detect_base_formation(ctx: &PatternContext<'_>, scope: Scope) -> Option<FormationMatch> {
    let branch = EarthlyBranch::Mao;
    if effective_branch_of_palace(ctx, scope, PalaceName::Life) != Some(branch) {
        return None;
    }
    let wu_qu = effective_star_in_palace(ctx, scope, branch, StarName::WuQu)?;

    let star = wu_qu.placement().name();
    Some(FormationMatch {
        id: PatternId::WuQuShouYuan,
        scope,
        anchor: PatternAnchor::Palace(branch),
        involved_palaces: vec![branch],
        involved_stars: vec![star],
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarInPalace { star, branch }],
    })
}
