//! 日月并明 — Tai Yang (太阳) and Tai Yin (太阴) both shining brightly.
//!
//! Conservative condition: both 太阳 and 太阴 are present and each sits in a
//! clearly bright/auspicious brightness state (庙/旺/得/利) per the existing
//! [`Brightness`] model. If either star's brightness is `Unknown` (or merely
//! `Flat`/dim), nothing is emitted — the rule never guesses an uncalculated
//! brightness. This reads only natal facts and never mutates them.
//!
//! [`Brightness`]: crate::core::Brightness

use crate::core::StarName;
use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::model::{
    PatternAnchor, PatternDetection, PatternEvidence, PatternFamily, PatternId, PatternPolarity,
    PatternScope, PatternStatus, PatternStrength,
};
use crate::core::pattern::query::is_bright;

const NAME_ZH: &str = "日月并明";

/// Detects 日月并明 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    _request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let chart = ctx.chart;

    let Some(sun) = chart.star(StarName::TaiYang) else {
        return;
    };
    let Some(moon) = chart.star(StarName::TaiYin) else {
        return;
    };

    if !is_bright(sun.placement().brightness()) || !is_bright(moon.placement().brightness()) {
        return;
    }

    let sun_branch = sun.palace().branch();
    let moon_branch = moon.palace().branch();

    let mut involved_palaces = vec![sun_branch, moon_branch];
    involved_palaces.sort_by_key(|branch| branch.index());
    involved_palaces.dedup();

    out.push(PatternDetection {
        id: PatternId::RiYueBingMing,
        name_zh: NAME_ZH,
        family: PatternFamily::MajorStarCombination,
        polarity: PatternPolarity::Auspicious,
        status: PatternStatus::Fulfilled,
        strength: PatternStrength::Medium,
        scope: PatternScope::Natal,
        anchor: PatternAnchor::Chart,
        involved_palaces,
        involved_stars: vec![StarName::TaiYang, StarName::TaiYin],
        involved_mutagens: Vec::new(),
        evidence: vec![
            PatternEvidence::StarInPalace {
                star: StarName::TaiYang,
                branch: sun_branch,
            },
            PatternEvidence::StarInPalace {
                star: StarName::TaiYin,
                branch: moon_branch,
            },
        ],
        missing_conditions: Vec::new(),
        weakening_factors: Vec::new(),
        breaking_factors: Vec::new(),
    });
}
