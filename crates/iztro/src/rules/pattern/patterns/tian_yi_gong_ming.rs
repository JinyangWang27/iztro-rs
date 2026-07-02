//! 天乙拱命 (坐贵向贵) — Tian Kui (天魁) and Tian Yue (天钺) arranged across the Life
//! and opposite (迁移) palaces.
//!
//! Source-backed (定贵局). 成格: 天魁 sits in the selected Life palace and 天钺 in its
//! opposite palace, or the reverse. The source (魁钺在命迭相坐拱) restricts the
//! formation to the Life/opposite axis, not the whole 三方四正.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{PalaceName, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::predicates::sort_dedup_branches;
use crate::rules::pattern::query::{selected_branch_of_palace, selected_palace_has_star};
use crate::rules::relation::{PalaceRelation, opposite};

/// Detects 天乙拱命 and appends any detection to `out`.
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

/// 成格: 天魁、天钺一在命宫、一在迁移宫相对拱照。
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    let scope = requested_selected_scope(ctx, request)?;
    let life = selected_branch_of_palace(ctx, PalaceName::Life)?;
    let opp = opposite(life);

    let (kui_branch, yue_branch) = if selected_palace_has_star(ctx, life, StarName::TianKui)
        && selected_palace_has_star(ctx, opp, StarName::TianYue)
    {
        (life, opp)
    } else if selected_palace_has_star(ctx, life, StarName::TianYue)
        && selected_palace_has_star(ctx, opp, StarName::TianKui)
    {
        (opp, life)
    } else {
        return None;
    };

    let mut involved_palaces = vec![life, opp];
    sort_dedup_branches(&mut involved_palaces);

    Some(FormationMatch {
        id: PatternId::TianYiGongMing,
        scope,
        anchor: PatternAnchor::Palace(life),
        involved_palaces,
        involved_stars: vec![StarName::TianKui, StarName::TianYue],
        involved_mutagens: Vec::new(),
        evidence: vec![
            PatternEvidence::StarInPalace {
                star: StarName::TianKui,
                branch: kui_branch,
            },
            PatternEvidence::StarInPalace {
                star: StarName::TianYue,
                branch: yue_branch,
            },
            PatternEvidence::PalaceRelation {
                from: life,
                to: opp,
                relation: PalaceRelation::Opposite,
            },
        ],
    })
}

/// 减力/破格: no weakening/breaker policy is modeled.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
