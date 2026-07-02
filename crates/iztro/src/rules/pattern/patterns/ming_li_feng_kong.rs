//! 命里逢空 — Di Kong (地空) and/or Di Jie (地劫) guarding the Life palace.
//!
//! 成格: 地空 and/or 地劫 sit in the selected Life palace. The 空亡-family modeled
//! stars (旬空/空亡/截路/截空) are **not** this pattern — only 地空 and 地劫 trigger it.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{PalaceName, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::query::{selected_branch_of_palace, selected_stars_in_palace};

/// Detects 命里逢空 and appends any detection to `out`.
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

/// 成格: 地空、地劫二星或其中一星守命。
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    let scope = requested_selected_scope(ctx, request)?;
    let branch = selected_branch_of_palace(ctx, PalaceName::Life)?;

    let mut matched: Vec<StarName> = selected_stars_in_palace(ctx, branch)
        .into_iter()
        .map(|placement| placement.placement().name())
        .filter(|star| matches!(star, StarName::DiKong | StarName::DiJie))
        .collect();
    if matched.is_empty() {
        return None;
    }
    matched.sort();
    matched.dedup();

    let evidence = matched
        .iter()
        .map(|star| PatternEvidence::StarInPalace {
            star: *star,
            branch,
        })
        .collect();

    Some(FormationMatch {
        id: PatternId::MingLiFengKong,
        scope,
        anchor: PatternAnchor::Palace(branch),
        involved_palaces: vec![branch],
        involved_stars: matched,
        involved_mutagens: Vec::new(),
        evidence,
    })
}

/// 减力/破格: no weakening/breaker policy is modeled.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
