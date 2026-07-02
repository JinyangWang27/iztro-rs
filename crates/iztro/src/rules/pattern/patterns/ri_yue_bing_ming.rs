//! 日月并明 — Tai Yang (太阳) and Tai Yin (太阴) both shining brightly.
//!
//! 成格: both 太阳 and 太阴 are present and each sits in a clearly bright state
//! (庙/旺/得/利). If either brightness is `Unknown` (or merely `Flat`/dim), nothing
//! is emitted — the rule never guesses an uncalculated brightness.
//! 减力/破格: no weakening/breaker policy is modeled, so integrity is always
//! fulfilled.

use crate::core::{Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::predicates::brightness::is_bright;
use crate::rules::pattern::query::find_star_for_scope;

/// Detects 日月并明 and appends any detection to `out`.
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

/// 成格: 太阳 and 太阴 both present and both bright.
fn detect_base_formation(ctx: &PatternContext<'_>, scope: Scope) -> Option<FormationMatch> {
    let sun = find_star_for_scope(ctx, scope, StarName::TaiYang)?;
    let moon = find_star_for_scope(ctx, scope, StarName::TaiYin)?;

    if !is_bright(sun.placement().brightness()) || !is_bright(moon.placement().brightness()) {
        return None;
    }

    let sun_branch = sun.branch();
    let moon_branch = moon.branch();

    let mut involved_palaces = vec![sun_branch, moon_branch];
    involved_palaces.sort_by_key(|branch| branch.index());
    involved_palaces.dedup();

    Some(FormationMatch {
        id: PatternId::RiYueBingMing,
        scope,
        anchor: PatternAnchor::Chart,
        involved_palaces,
        involved_stars: vec![sun.placement().name(), moon.placement().name()],
        involved_mutagens: Vec::new(),
        evidence: vec![
            PatternEvidence::StarInPalace {
                star: sun.placement().name(),
                branch: sun_branch,
            },
            PatternEvidence::StarInPalace {
                star: moon.placement().name(),
                branch: moon_branch,
            },
        ],
    })
}
