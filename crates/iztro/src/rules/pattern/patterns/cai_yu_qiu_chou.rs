//! 财与囚仇 — Wu Qu (武曲) and Lian Zhen (廉贞) together in the Life or Body palace.
//!
//! Source-backed (定贫贱局). 成格: 武曲 and 廉贞 share the Life palace, or (natal
//! only) the Body palace.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{EarthlyBranch, PalaceName, Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::query::{effective_branch_of_palace, effective_palace_has_all_stars};

/// Detects 财与囚仇 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for &scope in &request.scopes {
        for base in detect_base_formations(ctx, scope) {
            emit::push_detection(out, base, IntegrityAssessment::fulfilled());
        }
    }
}

/// 成格: 武曲、廉贞同守身命宫（每个符合条件的宫位各出一格）。
fn detect_base_formations(ctx: &PatternContext<'_>, scope: Scope) -> Vec<FormationMatch> {
    let mut branches = Vec::new();
    if let Some(branch) = effective_branch_of_palace(ctx, scope, PalaceName::Life) {
        branches.push(branch);
    }
    if scope == Scope::Natal {
        if let Some(branch) = ctx.chart().body_palace_branch() {
            branches.push(branch);
        }
    }
    branches.sort_by_key(|branch| branch.index());
    branches.dedup();

    branches
        .into_iter()
        .filter(|&branch| {
            effective_palace_has_all_stars(
                ctx,
                scope,
                branch,
                &[StarName::WuQu, StarName::LianZhen],
            )
        })
        .map(|branch| same_palace_formation(scope, branch))
        .collect()
}

fn same_palace_formation(scope: Scope, branch: EarthlyBranch) -> FormationMatch {
    let stars = vec![StarName::WuQu, StarName::LianZhen];
    FormationMatch {
        id: PatternId::CaiYuQiuChou,
        scope,
        anchor: PatternAnchor::Palace(branch),
        involved_palaces: vec![branch],
        involved_stars: stars.clone(),
        involved_mutagens: Vec::new(),
        evidence: vec![PatternEvidence::StarsInSamePalace { stars, branch }],
    }
}
