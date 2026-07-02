//! 天机巳亥 — Tian Ji (天机) guarding the Life palace at 巳 or 亥.
//!
//! 成格: the Life palace branch is 巳 or 亥, and 天机 occupies the Life palace itself
//! (not merely elsewhere in its 三方四正).
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{EarthlyBranch, PalaceName, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::query::{selected_branch_of_palace, selected_palace_has_all_stars};

/// Detects 天机巳亥 and appends any detection to `out`.
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

/// 成格: 命宫地支为巳或亥，天机坐守命宫。
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    let scope = requested_selected_scope(ctx, request)?;
    let branch = selected_branch_of_palace(ctx, PalaceName::Life)?;
    if !matches!(branch, EarthlyBranch::Si | EarthlyBranch::Hai) {
        return None;
    }
    if !selected_palace_has_all_stars(ctx, branch, &[StarName::TianJi]) {
        return None;
    }

    Some(FormationMatch {
        id: PatternId::TianJiSiHai,
        scope,
        anchor: PatternAnchor::Palace(branch),
        involved_palaces: vec![branch],
        involved_stars: vec![StarName::TianJi],
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarInPalace {
            star: StarName::TianJi,
            branch,
        }],
    })
}

/// 减力/破格: no weakening/breaker policy is modeled.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
