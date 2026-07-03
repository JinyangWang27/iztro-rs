//! 贞杀同宫 — Lian Zhen (廉贞) and Qi Sha (七杀) sharing the Life palace at 丑 or 未.
//!
//! Source-backed (卷三·论诸星同垣). 成格: the selected Life palace branch is 丑 or 未
//! and both 廉贞 and 七杀 occupy it. This detector recognises the
//! 廉贞七杀同守丑未命宫 structure only. The cited source distinguishes favourable
//! 庙旺 cases from 陷地化忌 cases; this implementation recognises the base structure
//! and does not infer later modern claims.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{PalaceName, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::query::{selected_branch_of_palace, selected_palace_has_all_stars};

/// Detects 贞杀同宫 and appends any detection to `out`.
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

/// 成格: 丑/未命宫，廉贞、七杀同守。
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    use crate::core::EarthlyBranch::{Chou, Wei};

    let scope = requested_selected_scope(ctx, request)?;
    let life = selected_branch_of_palace(ctx, PalaceName::Life)?;
    if life != Chou && life != Wei {
        return None;
    }

    let stars = [StarName::LianZhen, StarName::QiSha];
    if !selected_palace_has_all_stars(ctx, life, &stars) {
        return None;
    }

    Some(FormationMatch {
        id: PatternId::LianZhenQiShaTongGong,
        scope,
        anchor: PatternAnchor::Palace(life),
        involved_palaces: vec![life],
        involved_stars: stars.to_vec(),
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarsInSamePalace {
            stars: stars.to_vec(),
            branch: life,
        }],
    })
}

/// 减力/破格: no weakening/breaker policy is modeled.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
