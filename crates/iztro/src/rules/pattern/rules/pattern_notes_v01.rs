//! Normalized pattern detections from maintained pattern notes.
//!
//! These predicates are runtime-normalized structures. They may be informed by
//! source notes, but they are not direct source-provenance rows unless
//! `PatternSourceMetadata` separately says so.

use crate::core::{EarthlyBranch, Mutagen, PalaceName, Scope, StarKind, StarName};
use crate::rules::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::rules::pattern::display_metadata::pattern_display_metadata;
use crate::rules::pattern::model::{
    PatternAnchor, PatternCondition, PatternDetection, PatternEvidence, PatternFamily, PatternId,
    PatternPolarity, PatternStatus, PatternStrength,
};
use crate::rules::pattern::query::{
    branch_of_palace_for_scope, is_bright, mutagen_activations_for_scope,
    palace_has_all_stars_for_scope, pattern_scope_for, selected_branch_of_palace,
    selected_frame_scope, selected_major_star_count_in_palace, selected_palace_has_all_stars,
    selected_star_in_palace, selected_stars_in_palace, selected_stars_in_san_fang_si_zheng,
    stars_in_palace_for_scope,
};
use crate::rules::pattern::relation::{is_in_san_fang_si_zheng, san_fang_si_zheng};

const SUPPORT_STARS: [StarName; 7] = [
    StarName::ZuoFu,
    StarName::YouBi,
    StarName::WenChang,
    StarName::WenQu,
    StarName::TianKui,
    StarName::TianYue,
    StarName::LuCun,
];

/// Detects maintained normalized pattern-note rules.
pub fn detect(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    detect_ri_chu_fu_sang(ctx, request, out);
    detect_zuo_you_tong_gong(ctx, request, out);
    detect_ming_li_feng_kong(ctx, request, out);
    detect_lu_feng_chong_po(ctx, request, out);
    detect_wen_xing_gong_ming(ctx, request, out);
    detect_tian_ji_si_hai(ctx, request, out);
    detect_ming_zhu_chu_hai(ctx, request, out);
    detect_ming_wu_zheng_yao(ctx, request, out);
    detect_ji_xiang_li_ming(ctx, request, out);
    detect_fu_xiang_chao_yuan(ctx, request, out);
}

#[derive(Clone, Debug, Default)]
struct PatternSupportMatch {
    stars: Vec<(StarName, EarthlyBranch)>,
    mutagens: Vec<(StarName, Mutagen, Scope, EarthlyBranch)>,
    branches: Vec<EarthlyBranch>,
}

impl PatternSupportMatch {
    fn is_empty(&self) -> bool {
        self.stars.is_empty() && self.mutagens.is_empty()
    }

    fn involved_stars(&self) -> Vec<StarName> {
        let mut stars: Vec<StarName> = self
            .stars
            .iter()
            .map(|(star, _)| *star)
            .chain(self.mutagens.iter().map(|(star, _, _, _)| *star))
            .collect();
        stars.sort();
        stars.dedup();
        stars
    }

    fn involved_mutagens(&self) -> Vec<Mutagen> {
        let mut mutagens: Vec<Mutagen> = self
            .mutagens
            .iter()
            .map(|(_, mutagen, _, _)| *mutagen)
            .collect();
        mutagens.sort();
        mutagens.dedup();
        mutagens
    }

    fn evidence(&self) -> Vec<PatternEvidence> {
        let mut evidence: Vec<PatternEvidence> = self
            .stars
            .iter()
            .map(|(star, branch)| PatternEvidence::StarInPalace {
                star: *star,
                branch: *branch,
            })
            .collect();
        evidence.extend(self.mutagens.iter().map(|(star, mutagen, scope, branch)| {
            PatternEvidence::MutagenOnStar {
                star: *star,
                mutagen: *mutagen,
                scope: *scope,
                branch: *branch,
            }
        }));
        evidence
    }
}

/// Collects the explicit support set (禄存／左右／曲昌／魁钺 plus 化禄/权/科) within
/// the 三方四正 of `anchor`.
///
/// Support is restricted to the named [`SUPPORT_STARS`] and the
/// 禄/权/科 mutagens. Arbitrary [`StarKind::Soft`] stars are **not** accepted: the
/// detector requires the specific auxiliary set the maintained conditions name.
fn support_in_san_fang_si_zheng_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    anchor: EarthlyBranch,
) -> PatternSupportMatch {
    let mut support = PatternSupportMatch::default();

    for branch in san_fang_si_zheng(anchor) {
        for placement in stars_in_palace_for_scope(ctx, scope, branch) {
            let star = placement.placement().name();
            if SUPPORT_STARS.contains(&star) {
                support.stars.push((star, branch));
                support.branches.push(branch);
            }
            if matches!(
                placement.placement().mutagen(),
                Some(Mutagen::Lu | Mutagen::Quan | Mutagen::Ke)
            ) {
                let mutagen = placement.placement().mutagen().expect("checked mutagen");
                support
                    .mutagens
                    .push((star, mutagen, placement.placement().scope(), branch));
                support.branches.push(branch);
            }
        }
    }

    if scope != Scope::Natal {
        for activation in mutagen_activations_for_scope(ctx, scope) {
            if matches!(
                activation.mutagen(),
                Mutagen::Lu | Mutagen::Quan | Mutagen::Ke
            ) && is_in_san_fang_si_zheng(anchor, activation.target_branch())
            {
                support.mutagens.push((
                    activation.target_star(),
                    activation.mutagen(),
                    activation.source_scope(),
                    activation.target_branch(),
                ));
                support.branches.push(activation.target_branch());
            }
        }
    }

    sort_dedup_branches(&mut support.branches);
    support
}

fn requested_selected_scope(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<Scope> {
    let scope = selected_frame_scope(ctx)?;
    request.scopes.contains(&scope).then_some(scope)
}

fn selected_support_in_san_fang_si_zheng(
    ctx: &PatternContext<'_>,
    anchor: EarthlyBranch,
) -> PatternSupportMatch {
    let mut support = PatternSupportMatch::default();
    let branches = san_fang_si_zheng(anchor);

    for branch in branches {
        for requested_star in SUPPORT_STARS {
            if let Some(placement) = selected_star_in_palace(ctx, branch, requested_star) {
                support.stars.push((placement.placement().name(), branch));
                support.branches.push(branch);
            }
        }

        for placement in selected_stars_in_palace(ctx, branch) {
            let star = placement.placement().name();
            if placement.source_scope() == Scope::Natal
                && matches!(
                    placement.placement().mutagen(),
                    Some(Mutagen::Lu | Mutagen::Quan | Mutagen::Ke)
                )
            {
                let mutagen = placement.placement().mutagen().expect("checked mutagen");
                support
                    .mutagens
                    .push((star, mutagen, placement.source_scope(), branch));
                support.branches.push(branch);
            }
        }
    }

    if let Some(state) = ctx.effective() {
        for activation in state.mutagen_activations() {
            if matches!(
                activation.activation().mutagen(),
                Mutagen::Lu | Mutagen::Quan | Mutagen::Ke
            ) && is_in_san_fang_si_zheng(anchor, activation.activation().target_branch())
            {
                support.mutagens.push((
                    activation.activation().target_star(),
                    activation.activation().mutagen(),
                    activation.source_scope(),
                    activation.activation().target_branch(),
                ));
                support
                    .branches
                    .push(activation.activation().target_branch());
            }
        }
    }

    support
        .stars
        .sort_by_key(|(star, branch)| (*star, branch.index()));
    support.stars.dedup();
    support
        .mutagens
        .sort_by_key(|(star, mutagen, scope, branch)| {
            (*star, *mutagen, scope_sort_key(*scope), branch.index())
        });
    support.mutagens.dedup();
    sort_dedup_branches(&mut support.branches);
    support
}

fn detect_ri_chu_fu_sang(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    if !request.scopes.contains(&Scope::Natal) {
        return;
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
        return;
    }

    let branch = EarthlyBranch::Mao;
    if branch_of_palace_for_scope(ctx, Scope::Natal, PalaceName::Life) != Some(branch) {
        return;
    }
    if !palace_has_all_stars_for_scope(
        ctx,
        Scope::Natal,
        branch,
        &[StarName::TaiYang, StarName::TianLiang],
    ) {
        return;
    }

    let support = support_in_san_fang_si_zheng_for_scope(ctx, Scope::Natal, branch);
    if support.is_empty() {
        return;
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

    push_detection(
        out,
        DetectionDraft {
            id: PatternId::RiChuFuSang,
            family: PatternFamily::MajorStarCombination,
            polarity: PatternPolarity::Auspicious,
            status: PatternStatus::Fulfilled,
            scope: Scope::Natal,
            anchor: PatternAnchor::Palace(branch),
            involved_palaces,
            involved_stars,
            involved_mutagens: support.involved_mutagens(),
            evidence,
            breaking_factors: Vec::new(),
        },
    );
}

/// 命里逢空格 (凶): 地劫、地空二星或其中一星守命。
///
/// Anchored on the Life palace. The 空亡-family modeled stars (旬空/空亡/截路/截空)
/// are **not** this pattern: only 地空 (DiKong) and 地劫 (DiJie) sitting in the Life
/// palace trigger it.
fn detect_ming_li_feng_kong(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(scope) = requested_selected_scope(ctx, request) else {
        return;
    };
    let Some(branch) = selected_branch_of_palace(ctx, PalaceName::Life) else {
        return;
    };

    let mut matched: Vec<StarName> = selected_stars_in_palace(ctx, branch)
        .into_iter()
        .map(|placement| placement.placement().name())
        .filter(|star| matches!(star, StarName::DiKong | StarName::DiJie))
        .collect();
    if matched.is_empty() {
        return;
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

    push_detection(
        out,
        DetectionDraft {
            id: PatternId::MingLiFengKong,
            family: PatternFamily::ShaJi,
            polarity: PatternPolarity::Inauspicious,
            status: PatternStatus::Fulfilled,
            scope,
            anchor: PatternAnchor::Palace(branch),
            involved_palaces: vec![branch],
            involved_stars: matched,
            involved_mutagens: Vec::new(),
            evidence,
            breaking_factors: Vec::new(),
        },
    );
}

/// 禄逢冲破格 (凶): 禄存或化禄坐命，在三方四正中，有被地劫、地空冲破。
///
/// The 禄 base must sit in the Life palace itself (禄存 or a star carrying 化禄,
/// including temporal 化禄 activations). The breaker is restricted to 地空/地劫
/// within the Life 三方四正; arbitrary 煞星 or 空亡-family stars are not accepted.
fn detect_lu_feng_chong_po(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(scope) = requested_selected_scope(ctx, request) else {
        return;
    };
    let Some(life_branch) = selected_branch_of_palace(ctx, PalaceName::Life) else {
        return;
    };

    let Some(lu_base) = selected_lu_base_in_life(ctx, life_branch) else {
        return;
    };

    let breakers = selected_kong_jie_in_san_fang_si_zheng(ctx, life_branch);
    if breakers.is_empty() {
        return;
    }

    let mut involved_palaces = vec![life_branch];
    involved_palaces.extend(breakers.iter().map(|(_, branch)| *branch));
    sort_dedup_branches(&mut involved_palaces);

    let mut involved_stars = vec![lu_base.star];
    involved_stars.extend(breakers.iter().map(|(star, _)| *star));
    involved_stars.sort();
    involved_stars.dedup();

    let mut involved_mutagens = Vec::new();
    if let Some(mutagen) = lu_base.mutagen {
        involved_mutagens.push(mutagen);
    }

    let mut breaker_stars: Vec<StarName> = breakers.iter().map(|(star, _)| *star).collect();
    breaker_stars.sort();
    breaker_stars.dedup();
    let mut breaker_branches: Vec<EarthlyBranch> =
        breakers.iter().map(|(_, branch)| *branch).collect();
    sort_dedup_branches(&mut breaker_branches);

    let mut evidence = vec![lu_base.evidence];
    evidence.push(PatternEvidence::StarsInSanFangSiZheng {
        stars: breaker_stars,
        anchor: life_branch,
        branches: breaker_branches,
    });

    let breaking_factors = breakers
        .iter()
        .map(|(star, branch)| PatternCondition::BrokenByStar {
            star: *star,
            branch: *branch,
        })
        .collect();

    push_detection(
        out,
        DetectionDraft {
            id: PatternId::LuFengChongPo,
            family: PatternFamily::ShaJi,
            polarity: PatternPolarity::Inauspicious,
            status: PatternStatus::Broken,
            scope,
            anchor: PatternAnchor::Palace(life_branch),
            involved_palaces,
            involved_stars,
            involved_mutagens,
            evidence,
            breaking_factors,
        },
    );
}

struct LuBase {
    star: StarName,
    mutagen: Option<Mutagen>,
    evidence: PatternEvidence,
}

/// Returns a 禄 base (禄存 or 化禄) sitting in the Life palace itself.
fn selected_lu_base_in_life(
    ctx: &PatternContext<'_>,
    life_branch: EarthlyBranch,
) -> Option<LuBase> {
    for placement in selected_stars_in_palace(ctx, life_branch) {
        let star = placement.placement().name();
        if star == StarName::LuCun {
            return Some(LuBase {
                star,
                mutagen: None,
                evidence: PatternEvidence::StarInPalace {
                    star,
                    branch: life_branch,
                },
            });
        }
        if placement.placement().mutagen() == Some(Mutagen::Lu) {
            return Some(LuBase {
                star,
                mutagen: Some(Mutagen::Lu),
                evidence: PatternEvidence::MutagenOnStar {
                    star,
                    mutagen: Mutagen::Lu,
                    scope: placement.source_scope(),
                    branch: life_branch,
                },
            });
        }
    }

    if let Some(state) = ctx.effective() {
        for activation in state.mutagen_activations() {
            let activation_fact = activation.activation();
            if activation_fact.mutagen() == Mutagen::Lu
                && activation_fact.target_branch() == life_branch
            {
                return Some(LuBase {
                    star: activation_fact.target_star(),
                    mutagen: Some(Mutagen::Lu),
                    evidence: PatternEvidence::MutagenOnStar {
                        star: activation_fact.target_star(),
                        mutagen: Mutagen::Lu,
                        scope: activation.source_scope(),
                        branch: life_branch,
                    },
                });
            }
        }
    }

    None
}

/// Returns each 地空/地劫 found within the 三方四正 of `anchor`, with its branch.
fn selected_kong_jie_in_san_fang_si_zheng(
    ctx: &PatternContext<'_>,
    anchor: EarthlyBranch,
) -> Vec<(StarName, EarthlyBranch)> {
    let mut found = Vec::new();
    for branch in san_fang_si_zheng(anchor) {
        for placement in selected_stars_in_palace(ctx, branch) {
            let star = placement.placement().name();
            if matches!(star, StarName::DiKong | StarName::DiJie) {
                found.push((star, branch));
            }
        }
    }
    found
}

fn detect_wen_xing_gong_ming(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(scope) = requested_selected_scope(ctx, request) else {
        return;
    };
    let Some(anchor) = selected_branch_of_palace(ctx, PalaceName::Life) else {
        return;
    };
    let found =
        selected_stars_in_san_fang_si_zheng(ctx, anchor, &[StarName::WenChang, StarName::WenQu]);
    if selected_stars_in_san_fang_si_zheng(ctx, anchor, &[StarName::WenChang]).is_empty()
        || selected_stars_in_san_fang_si_zheng(ctx, anchor, &[StarName::WenQu]).is_empty()
    {
        return;
    }

    push_san_fang_detection(
        out,
        PatternId::WenXingGongMing,
        PatternFamily::AuxiliaryStarCombination,
        PatternPolarity::Auspicious,
        scope,
        anchor,
        found,
    );
}

/// 天机巳亥格 (凶): 天机在巳或亥坐守命宫。
///
/// The Life palace branch must be Si or Hai, and 天机 must occupy the Life palace
/// itself — not merely appear elsewhere in the Life 三方四正.
fn detect_tian_ji_si_hai(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(scope) = requested_selected_scope(ctx, request) else {
        return;
    };
    let Some(branch) = selected_branch_of_palace(ctx, PalaceName::Life) else {
        return;
    };
    if !matches!(branch, EarthlyBranch::Si | EarthlyBranch::Hai) {
        return;
    }
    if !selected_palace_has_all_stars(ctx, branch, &[StarName::TianJi]) {
        return;
    }

    push_detection(
        out,
        DetectionDraft {
            id: PatternId::TianJiSiHai,
            family: PatternFamily::MajorStarCombination,
            polarity: PatternPolarity::Inauspicious,
            status: PatternStatus::Fulfilled,
            scope,
            anchor: PatternAnchor::Palace(branch),
            involved_palaces: vec![branch],
            involved_stars: vec![StarName::TianJi],
            involved_mutagens: Vec::new(),
            evidence: vec![PatternEvidence::StarInPalace {
                star: StarName::TianJi,
                branch,
            }],
            breaking_factors: Vec::new(),
        },
    );
}

/// 左右同宫格 (吉): 命身宫入丑未，左辅右弼同宫，更于吉星同宫或加会者，为本格。
///
/// The selected Life palace anchors non-natal frames. Natal also accepts the
/// natal Body palace, preserving the traditional 命身 condition. 左辅 and 右弼
/// must share the anchor palace, with additional support (`更于吉星`) in the
/// anchor 三方四正 beyond the base 左右 pair itself.
fn detect_zuo_you_tong_gong(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(scope) = requested_selected_scope(ctx, request) else {
        return;
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

    for anchor in anchors {
        if !selected_palace_has_all_stars(ctx, anchor, &[StarName::ZuoFu, StarName::YouBi]) {
            continue;
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
            continue;
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

        push_detection(
            out,
            DetectionDraft {
                id: PatternId::ZuoYouTongGong,
                family: PatternFamily::AuxiliaryStarCombination,
                polarity: PatternPolarity::Auspicious,
                status: PatternStatus::Fulfilled,
                scope,
                anchor: PatternAnchor::Palace(anchor),
                involved_palaces,
                involved_stars,
                involved_mutagens: additional.involved_mutagens(),
                evidence,
                breaking_factors: Vec::new(),
            },
        );
    }
}

/// 明珠出海格 (吉): 安命在未无正曜，卯宫太阳天梁、亥宫太阴入庙旺合照命宫，三方四正见
/// 禄存，科权禄、左右、曲昌、魁钺加会为本格。
fn detect_ming_zhu_chu_hai(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(scope) = requested_selected_scope(ctx, request) else {
        return;
    };
    let Some(life) = selected_branch_of_palace(ctx, PalaceName::Life) else {
        return;
    };
    if life != EarthlyBranch::Wei {
        return;
    }
    if selected_major_star_count_in_palace(ctx, life) != 0 {
        return;
    }
    if !selected_palace_has_all_stars(
        ctx,
        EarthlyBranch::Mao,
        &[StarName::TaiYang, StarName::TianLiang],
    ) {
        return;
    }
    let tai_yin_bright = selected_stars_in_palace(ctx, EarthlyBranch::Hai)
        .into_iter()
        .any(|placement| {
            placement.placement().name() == StarName::TaiYin
                && is_bright(placement.placement().brightness())
        });
    if !tai_yin_bright {
        return;
    }

    let support = selected_support_in_san_fang_si_zheng(ctx, life);
    if support.is_empty() {
        return;
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

    push_detection(
        out,
        DetectionDraft {
            id: PatternId::MingZhuChuHai,
            family: PatternFamily::MajorStarCombination,
            polarity: PatternPolarity::Auspicious,
            status: PatternStatus::Fulfilled,
            scope,
            anchor: PatternAnchor::Palace(life),
            involved_palaces,
            involved_stars,
            involved_mutagens: support.involved_mutagens(),
            evidence,
            breaking_factors: Vec::new(),
        },
    );
}

fn detect_ming_wu_zheng_yao(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(scope) = requested_selected_scope(ctx, request) else {
        return;
    };
    let Some(branch) = selected_branch_of_palace(ctx, PalaceName::Life) else {
        return;
    };
    if selected_major_star_count_in_palace(ctx, branch) != 0 {
        return;
    }

    push_detection(
        out,
        DetectionDraft {
            id: PatternId::MingWuZhengYao,
            family: PatternFamily::MajorStarCombination,
            polarity: PatternPolarity::Neutral,
            status: PatternStatus::Fulfilled,
            scope,
            anchor: PatternAnchor::Palace(branch),
            involved_palaces: vec![branch],
            involved_stars: Vec::new(),
            involved_mutagens: Vec::new(),
            evidence: vec![PatternEvidence::NoMajorStarInPalace { branch }],
            breaking_factors: Vec::new(),
        },
    );
}

fn detect_ji_xiang_li_ming(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(scope) = requested_selected_scope(ctx, request) else {
        return;
    };
    let branch = EarthlyBranch::Wu;
    if selected_branch_of_palace(ctx, PalaceName::Life) != Some(branch)
        || !selected_palace_has_all_stars(ctx, branch, &[StarName::ZiWei])
    {
        return;
    }

    let breaker = san_fang_si_zheng(branch).into_iter().find_map(|candidate| {
        selected_stars_in_palace(ctx, candidate)
            .into_iter()
            .find(|placement| placement.placement().kind() == StarKind::Tough)
            .map(|placement| (placement.placement().name(), candidate))
    });

    let (status, breaking_factors) = if let Some((star, breaker_branch)) = breaker {
        (
            PatternStatus::Broken,
            vec![PatternCondition::BrokenByStar {
                star,
                branch: breaker_branch,
            }],
        )
    } else {
        (PatternStatus::Fulfilled, Vec::new())
    };

    let mut involved_palaces = vec![branch];
    let mut involved_stars = vec![StarName::ZiWei];
    let mut evidence = vec![PatternEvidence::StarInPalace {
        star: StarName::ZiWei,
        branch,
    }];
    if let Some((star, breaker_branch)) = breaker {
        involved_palaces.push(breaker_branch);
        involved_stars.push(star);
        evidence.push(PatternEvidence::StarInPalace {
            star,
            branch: breaker_branch,
        });
    }
    sort_dedup_branches(&mut involved_palaces);
    involved_stars.sort();
    involved_stars.dedup();

    push_detection(
        out,
        DetectionDraft {
            id: PatternId::JiXiangLiMing,
            family: PatternFamily::MajorStarCombination,
            polarity: PatternPolarity::Auspicious,
            status,
            scope,
            anchor: PatternAnchor::Palace(branch),
            involved_palaces,
            involved_stars,
            involved_mutagens: Vec::new(),
            evidence,
            breaking_factors,
        },
    );
}

/// 府相朝垣格 (吉): 天府、天相二星一居财帛宫，一居官禄宫，来合照命宫，或者天府坐命，
/// 加会天相。命宫三方四正有禄存，科权禄、左右、曲昌、魁钺加会。
fn detect_fu_xiang_chao_yuan(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    let Some(scope) = requested_selected_scope(ctx, request) else {
        return;
    };
    let Some(life) = selected_branch_of_palace(ctx, PalaceName::Life) else {
        return;
    };

    let Some(base) = selected_fu_xiang_base_form(ctx, life) else {
        return;
    };

    let support = selected_support_in_san_fang_si_zheng(ctx, life);
    if support.is_empty() {
        return;
    }

    let mut involved_palaces = base.palaces.clone();
    involved_palaces.extend(support.branches.iter().copied());
    sort_dedup_branches(&mut involved_palaces);

    let mut involved_stars = vec![StarName::TianFu, StarName::TianXiang];
    involved_stars.extend(support.involved_stars());
    involved_stars.sort();
    involved_stars.dedup();

    let mut evidence = base.evidence;
    evidence.extend(support.evidence());

    push_detection(
        out,
        DetectionDraft {
            id: PatternId::FuXiangChaoYuan,
            family: PatternFamily::MajorStarCombination,
            polarity: PatternPolarity::Auspicious,
            status: PatternStatus::Fulfilled,
            scope,
            anchor: PatternAnchor::Palace(life),
            involved_palaces,
            involved_stars,
            involved_mutagens: support.involved_mutagens(),
            evidence,
            breaking_factors: Vec::new(),
        },
    );
}

struct FuXiangBase {
    palaces: Vec<EarthlyBranch>,
    evidence: Vec<PatternEvidence>,
}

/// Returns the matched 府相 base formation, restricted to the two maintained forms:
///
/// A. 天府 and 天相 occupy the Wealth and Career palaces, one in each.
/// B. 天府 sits in the Life palace, and 天相 appears in the Life 三方四正.
fn selected_fu_xiang_base_form(
    ctx: &PatternContext<'_>,
    life: EarthlyBranch,
) -> Option<FuXiangBase> {
    let wealth = selected_branch_of_palace(ctx, PalaceName::Wealth);
    let career = selected_branch_of_palace(ctx, PalaceName::Career);
    if let (Some(wealth), Some(career)) = (wealth, career) {
        let fu_at = |branch| selected_palace_has_all_stars(ctx, branch, &[StarName::TianFu]);
        let xiang_at = |branch| selected_palace_has_all_stars(ctx, branch, &[StarName::TianXiang]);
        if fu_at(wealth) && xiang_at(career) {
            return Some(FuXiangBase {
                palaces: vec![wealth, career],
                evidence: vec![
                    PatternEvidence::StarInPalace {
                        star: StarName::TianFu,
                        branch: wealth,
                    },
                    PatternEvidence::StarInPalace {
                        star: StarName::TianXiang,
                        branch: career,
                    },
                ],
            });
        }
        if fu_at(career) && xiang_at(wealth) {
            return Some(FuXiangBase {
                palaces: vec![wealth, career],
                evidence: vec![
                    PatternEvidence::StarInPalace {
                        star: StarName::TianFu,
                        branch: career,
                    },
                    PatternEvidence::StarInPalace {
                        star: StarName::TianXiang,
                        branch: wealth,
                    },
                ],
            });
        }
    }

    if selected_palace_has_all_stars(ctx, life, &[StarName::TianFu]) {
        let xiang = selected_stars_in_san_fang_si_zheng(ctx, life, &[StarName::TianXiang]);
        if let Some((_, xiang_branch)) = xiang.first().copied() {
            return Some(FuXiangBase {
                palaces: vec![life, xiang_branch],
                evidence: vec![
                    PatternEvidence::StarInPalace {
                        star: StarName::TianFu,
                        branch: life,
                    },
                    PatternEvidence::StarInPalace {
                        star: StarName::TianXiang,
                        branch: xiang_branch,
                    },
                ],
            });
        }
    }

    None
}

fn push_san_fang_detection(
    out: &mut Vec<PatternDetection>,
    id: PatternId,
    family: PatternFamily,
    polarity: PatternPolarity,
    scope: Scope,
    anchor: EarthlyBranch,
    found: Vec<(StarName, EarthlyBranch)>,
) {
    let mut branches: Vec<EarthlyBranch> = found.iter().map(|(_, branch)| *branch).collect();
    sort_dedup_branches(&mut branches);
    let mut stars: Vec<StarName> = found.iter().map(|(star, _)| *star).collect();
    stars.sort();
    stars.dedup();
    let evidence = vec![PatternEvidence::StarsInSanFangSiZheng {
        stars: stars.clone(),
        anchor,
        branches: branches.clone(),
    }];

    push_detection(
        out,
        DetectionDraft {
            id,
            family,
            polarity,
            status: PatternStatus::Fulfilled,
            scope,
            anchor: PatternAnchor::Palace(anchor),
            involved_palaces: branches,
            involved_stars: stars,
            involved_mutagens: Vec::new(),
            evidence,
            breaking_factors: Vec::new(),
        },
    );
}

struct DetectionDraft {
    id: PatternId,
    family: PatternFamily,
    polarity: PatternPolarity,
    status: PatternStatus,
    scope: Scope,
    anchor: PatternAnchor,
    involved_palaces: Vec<EarthlyBranch>,
    involved_stars: Vec<StarName>,
    involved_mutagens: Vec<Mutagen>,
    evidence: Vec<PatternEvidence>,
    breaking_factors: Vec<PatternCondition>,
}

fn push_detection(out: &mut Vec<PatternDetection>, draft: DetectionDraft) {
    let metadata = pattern_display_metadata(draft.id);
    out.push(PatternDetection {
        id: draft.id,
        name_zh: metadata.name_zh,
        family: draft.family,
        polarity: draft.polarity,
        status: draft.status,
        strength: PatternStrength::Medium,
        scope: pattern_scope_for(draft.scope),
        anchor: draft.anchor,
        involved_palaces: draft.involved_palaces,
        involved_stars: draft.involved_stars,
        involved_mutagens: draft.involved_mutagens,
        evidence: draft.evidence,
        weakening_factors: Vec::new(),
        breaking_factors: draft.breaking_factors,
    });
}

fn sort_dedup_branches(branches: &mut Vec<EarthlyBranch>) {
    branches.sort_by_key(|branch| branch.index());
    branches.dedup();
}

const fn scope_sort_key(scope: Scope) -> u8 {
    match scope {
        Scope::Natal => 0,
        Scope::Decadal => 1,
        Scope::Age => 2,
        Scope::Yearly => 3,
        Scope::Monthly => 4,
        Scope::Daily => 5,
        Scope::Hourly => 6,
    }
}
