//! 明珠出海 — empty Life palace at 未 lit by 卯 太阳天梁 and bright 亥 太阴, with
//! auxiliary support.
//!
//! 成格: the selected Life palace is at 未 with no major star, 卯 holds 太阳 and
//! 天梁, 亥 holds a bright 太阴, and the Life 三方四正 carries auxiliary support.
//! 减力/破格: no weakening/breaker policy is modeled; integrity is always fulfilled.

use crate::core::{EarthlyBranch, PalaceName, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::model::{PatternAnchor, PatternDetection, PatternEvidence, PatternId};
use crate::rules::pattern::patterns::emit::{self, FormationMatch, IntegrityAssessment};
use crate::rules::pattern::patterns::requested_selected_scope;
use crate::rules::pattern::predicates::brightness::is_bright;
use crate::rules::pattern::predicates::sort_dedup_branches;
use crate::rules::pattern::predicates::support::selected_support_in_san_fang_si_zheng;
use crate::rules::pattern::query::{
    selected_branch_of_palace, selected_major_star_count_in_palace, selected_palace_has_all_stars,
    selected_stars_in_palace,
};

/// Detects 明珠出海 and appends any detection to `out`.
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

/// 成格: 未宫无正曜坐命，卯宫太阳天梁、亥宫太阴入庙旺合照，三方四正见辅佐加会。
fn detect_base_formation(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<FormationMatch> {
    let scope = requested_selected_scope(ctx, request)?;
    let life = selected_branch_of_palace(ctx, PalaceName::Life)?;
    if life != EarthlyBranch::Wei {
        return None;
    }
    if selected_major_star_count_in_palace(ctx, life) != 0 {
        return None;
    }
    if !selected_palace_has_all_stars(
        ctx,
        EarthlyBranch::Mao,
        &[StarName::TaiYang, StarName::TianLiang],
    ) {
        return None;
    }
    let tai_yin_bright = selected_stars_in_palace(ctx, EarthlyBranch::Hai)
        .into_iter()
        .any(|placement| {
            placement.placement().name() == StarName::TaiYin
                && is_bright(placement.placement().brightness())
        });
    if !tai_yin_bright {
        return None;
    }

    let support = selected_support_in_san_fang_si_zheng(ctx, life);
    if support.is_empty() {
        return None;
    }

    let mut involved_palaces = vec![life, EarthlyBranch::Mao, EarthlyBranch::Hai];
    involved_palaces.extend(support.branches.iter().copied());
    sort_dedup_branches(&mut involved_palaces);

    let mut involved_stars = vec![StarName::TaiYang, StarName::TianLiang, StarName::TaiYin];
    involved_stars.extend(support.involved_stars());
    involved_stars.sort();
    involved_stars.dedup();

    let mut evidence = vec![
        PatternEvidence::NoMajorStarInPalace { branch: life },
        PatternEvidence::StarsInSamePalace {
            stars: vec![StarName::TaiYang, StarName::TianLiang],
            branch: EarthlyBranch::Mao,
        },
        PatternEvidence::StarInPalace {
            star: StarName::TaiYin,
            branch: EarthlyBranch::Hai,
        },
    ];
    evidence.extend(support.evidence());

    Some(FormationMatch {
        id: PatternId::MingZhuChuHai,
        scope,
        anchor: PatternAnchor::Palace(life),
        involved_palaces,
        involved_stars,
        involved_mutagens: support.involved_mutagens(),
        evidence,
    })
}

/// 减力/破格: no weakening/breaker policy is modeled.
fn assess_integrity(_ctx: &PatternContext<'_>, _base: &FormationMatch) -> IntegrityAssessment {
    IntegrityAssessment::fulfilled()
}
