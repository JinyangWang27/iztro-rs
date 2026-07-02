//! 左右同宫 — Zuo Fu (左辅) and You Bi (右弼) together in the 命/身 palace at 丑/未
//! with additional auspicious support.
//!
//! 成格: the selected Life palace anchors non-natal frames; natal also accepts the
//! natal Body palace (the traditional 命身 condition). 左辅 and 右弼 share the anchor
//! palace at 丑 or 未, with additional support (`更于吉星`) in the anchor 三方四正
//! beyond the base 左右 pair itself.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{EarthlyBranch, PalaceName, Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::predicates::sort_dedup_branches;
use crate::rules::pattern::predicates::support::{
    PatternSupportMatch, selected_support_in_san_fang_si_zheng,
};
use crate::rules::pattern::query::{selected_branch_of_palace, selected_palace_has_all_stars};

/// Detects 左右同宫 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for base in detect_base_formations(ctx, request) {
        emit::push_detection(out, base, IntegrityAssessment::fulfilled());
    }
}

/// 成格: 命身宫入丑未，左辅右弼同宫，更于吉星加会（每个符合条件的宫位各出一格）。
fn detect_base_formations(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Vec<FormationMatch> {
    let Some(scope) = requested_selected_scope(ctx, request) else {
        return Vec::new();
    };

    let life = selected_branch_of_palace(ctx, PalaceName::Life);
    let body = (scope == Scope::Natal)
        .then(|| ctx.chart().body_palace_branch())
        .flatten();

    let mut anchors: Vec<EarthlyBranch> = Vec::new();
    for candidate in [life, body].into_iter().flatten() {
        if matches!(candidate, EarthlyBranch::Chou | EarthlyBranch::Wei)
            && !anchors.contains(&candidate)
        {
            anchors.push(candidate);
        }
    }

    let mut out = Vec::new();
    for anchor in anchors {
        if let Some(base) = formation_for_anchor(ctx, scope, anchor) {
            out.push(base);
        }
    }
    out
}

fn formation_for_anchor(
    ctx: &PatternContext<'_>,
    scope: Scope,
    anchor: EarthlyBranch,
) -> Option<FormationMatch> {
    if !selected_palace_has_all_stars(ctx, anchor, &[StarName::ZuoFu, StarName::YouBi]) {
        return None;
    }

    let support = selected_support_in_san_fang_si_zheng(ctx, anchor);
    // `更于吉星`: support beyond the base 左右 pair sitting in the anchor palace.
    let additional_stars: Vec<(StarName, EarthlyBranch)> = support
        .stars
        .iter()
        .copied()
        .filter(|(star, branch)| {
            !(*branch == anchor && matches!(star, StarName::ZuoFu | StarName::YouBi))
        })
        .collect();
    if additional_stars.is_empty() && support.mutagens.is_empty() {
        return None;
    }
    let additional = PatternSupportMatch {
        stars: additional_stars,
        mutagens: support.mutagens.clone(),
        branches: Vec::new(),
    };

    let mut involved_palaces = vec![anchor];
    involved_palaces.extend(additional.stars.iter().map(|(_, branch)| *branch));
    involved_palaces.extend(additional.mutagens.iter().map(|(_, _, _, branch)| *branch));
    sort_dedup_branches(&mut involved_palaces);

    let mut involved_stars = vec![StarName::ZuoFu, StarName::YouBi];
    involved_stars.extend(additional.involved_stars());
    involved_stars.sort();
    involved_stars.dedup();

    let mut evidence = vec![PatternEvidence::StarsInSamePalace {
        stars: vec![StarName::ZuoFu, StarName::YouBi],
        branch: anchor,
    }];
    evidence.extend(additional.evidence());

    Some(FormationMatch {
        id: PatternId::ZuoYouTongGong,
        scope,
        anchor: PatternAnchor::Palace(anchor),
        involved_palaces,
        involved_stars,
        involved_mutagens: additional.involved_mutagens(),
        evidence,
    })
}
