//! 日出扶桑 (runtime 日照雷门) — Tai Yang (太阳) and Tian Liang (天梁) at 卯 with
//! auxiliary support.
//!
//! Source-backed (定贵局). 成格: birth time is 卯–未, the natal Life palace is at 卯
//! holding 太阳 and 天梁, and the Life 三方四正 carries auxiliary support (禄存／科权禄
//! ／左右／曲昌／魁钺). Natal-only.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{EarthlyBranch, PalaceName, Scope, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::predicates::sort_dedup_branches;
use crate::rules::pattern::predicates::support::support_in_san_fang_si_zheng_for_scope;
use crate::rules::pattern::query::{branch_of_palace_for_scope, palace_has_all_stars_for_scope};

/// Detects 日出扶桑 and appends any detection to `out`.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(base) = detect_base_formation(ctx, request) else {
        return;
    };
    emit::push_detection(out, base, IntegrityAssessment::fulfilled());
}

/// 成格: 出生时辰卯至未，太阳天梁在卯宫坐命，命宫三方四正见辅佐加会。
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    if !request.scopes.contains(&Scope::Natal) {
        return None;
    }

    let birth_time = ctx.chart().birth_context().birth_time();
    if !matches!(
        birth_time,
        EarthlyBranch::Mao
            | EarthlyBranch::Chen
            | EarthlyBranch::Si
            | EarthlyBranch::Wu
            | EarthlyBranch::Wei
    ) {
        return None;
    }

    let branch = EarthlyBranch::Mao;
    if branch_of_palace_for_scope(ctx, Scope::Natal, PalaceName::Life) != Some(branch) {
        return None;
    }
    if !palace_has_all_stars_for_scope(
        ctx,
        Scope::Natal,
        branch,
        &[StarName::TaiYang, StarName::TianLiang],
    ) {
        return None;
    }

    let support = support_in_san_fang_si_zheng_for_scope(ctx, Scope::Natal, branch);
    if support.is_empty() {
        return None;
    }

    let mut involved_palaces = vec![branch];
    involved_palaces.extend(support.branches.iter().copied());
    sort_dedup_branches(&mut involved_palaces);

    let mut involved_stars = vec![StarName::TaiYang, StarName::TianLiang];
    involved_stars.extend(support.involved_stars());
    involved_stars.sort();
    involved_stars.dedup();

    let mut evidence = vec![PatternEvidence::StarsInSamePalace {
        stars: vec![StarName::TaiYang, StarName::TianLiang],
        branch,
    }];
    evidence.extend(support.evidence());

    Some(FormationMatch {
        id: PatternId::RiChuFuSang,
        scope: Scope::Natal,
        anchor: PatternAnchor::Palace(branch),
        involved_palaces,
        involved_stars,
        involved_mutagens: support.involved_mutagens(),
        evidence,
    })
}
