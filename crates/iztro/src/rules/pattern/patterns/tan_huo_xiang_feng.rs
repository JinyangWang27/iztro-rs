//! 贪火相逢 — Tan Lang (贪狼) and Huo Xing (火星) both bright in the Life palace.
//!
//! Source-backed (定贵局). 成格: 贪狼 and 火星 share the Life palace, and both are
//! in a bright state (庙旺).
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{PalaceName, Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::predicates::brightness::is_bright;
use crate::rules::pattern::query::{
    effective_branch_of_palace, effective_palace_has_all_stars, effective_star_in_palace,
};

/// Detects 贪火相逢 and appends any detection to `out`.
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

/// 成格: 贪狼、火星同守命宫，且二者皆入庙旺。
fn detect_base_formation(ctx: &PatternContext<'_>, scope: Scope) -> Option<FormationMatch> {
    let branch = effective_branch_of_palace(ctx, scope, PalaceName::Life)?;
    if !effective_palace_has_all_stars(ctx, scope, branch, &[StarName::TanLang, StarName::HuoXing])
    {
        return None;
    }

    let tan_lang = effective_star_in_palace(ctx, scope, branch, StarName::TanLang)?;
    let huo_xing = effective_star_in_palace(ctx, scope, branch, StarName::HuoXing)?;
    if !is_bright(tan_lang.placement().brightness())
        || !is_bright(huo_xing.placement().brightness())
    {
        return None;
    }

    let stars = vec![tan_lang.placement().name(), huo_xing.placement().name()];
    Some(FormationMatch {
        id: PatternId::TanHuoXiangFeng,
        scope,
        anchor: PatternAnchor::Palace(branch),
        involved_palaces: vec![branch],
        involved_stars: stars.clone(),
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarsInSamePalace { stars, branch }],
    })
}
