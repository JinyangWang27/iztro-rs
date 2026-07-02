//! 马头带剑 — Tian Ma (天马) sharing a palace with Qing Yang (擎羊).
//!
//! Source-backed (定贵局). 成格: 天马 and 擎羊 occupy the same palace.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::query::find_star_for_scope;

/// Detects 马头带剑 and appends any detection to `out`.
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

/// 成格: 天马与擎羊同宫。
fn detect_base_formation(ctx: &PatternContext<'_>, scope: Scope) -> Option<FormationMatch> {
    let tian_ma = find_star_for_scope(ctx, scope, StarName::TianMa)?;
    let branch = tian_ma.branch();
    let qing_yang = find_star_for_scope(ctx, scope, StarName::QingYang)?;
    if qing_yang.branch() != branch {
        return None;
    }

    let stars = vec![tian_ma.placement().name(), qing_yang.placement().name()];
    Some(FormationMatch {
        id: PatternId::MaTouDaiJian,
        scope,
        anchor: PatternAnchor::Palace(branch),
        involved_palaces: vec![branch],
        involved_stars: stars.clone(),
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarsInSamePalace { stars, branch }],
    })
}
