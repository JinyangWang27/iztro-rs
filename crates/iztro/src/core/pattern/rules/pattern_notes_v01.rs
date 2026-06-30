//! Normalized pattern detections from maintained pattern notes.
//!
//! These predicates are runtime-normalized structures. They may be informed by
//! source notes, but they are not direct source-provenance rows unless
//! `PatternSourceMetadata` separately says so.

use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::display_metadata::pattern_display_metadata;
use crate::core::pattern::model::{
    PatternAnchor, PatternCondition, PatternDetection, PatternEvidence, PatternFamily, PatternId,
    PatternPolarity, PatternStatus, PatternStrength,
};
use crate::core::pattern::query::{
    branch_of_palace_for_scope, is_bright, major_star_count_in_palace_for_scope,
    modeled_void_star_in_palace_for_scope, mutagen_activations_for_scope,
    palace_has_all_stars_for_scope, pattern_scope_for, stars_in_palace_for_scope,
    stars_in_san_fang_si_zheng_for_scope,
};
use crate::core::pattern::relation::{is_in_san_fang_si_zheng, is_opposite, san_fang_si_zheng};
use crate::core::{EarthlyBranch, Mutagen, PalaceName, Scope, StarKind, StarName};

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

    for &scope in &request.scopes {
        detect_ming_li_feng_kong(ctx, scope, out);
        detect_lu_feng_chong_po(ctx, scope, out);
        detect_wen_xing_gong_ming(ctx, scope, out);
        detect_tian_ji_si_hai(ctx, scope, out);
        detect_ming_zhu_chu_hai(ctx, scope, out);
        detect_ming_wu_zheng_yao(ctx, scope, out);
        detect_ji_xiang_li_ming(ctx, scope, out);
        detect_fu_xiang_chao_yuan(ctx, scope, out);
    }
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

fn support_in_san_fang_si_zheng_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    anchor: EarthlyBranch,
) -> PatternSupportMatch {
    let mut support = PatternSupportMatch::default();

    for branch in san_fang_si_zheng(anchor) {
        for placement in stars_in_palace_for_scope(ctx, scope, branch) {
            let star = placement.placement().name();
            if SUPPORT_STARS.contains(&star) || placement.placement().kind() == StarKind::Soft {
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

fn detect_ri_chu_fu_sang(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    if !request.scopes.contains(&Scope::Natal) {
        return;
    }

    let birth_time = ctx.chart.birth_context().birth_time();
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

fn detect_ming_li_feng_kong(
    ctx: &PatternContext<'_>,
    scope: Scope,
    out: &mut Vec<PatternDetection>,
) {
    let Some(branch) = branch_of_palace_for_scope(ctx, scope, PalaceName::Life) else {
        return;
    };
    let Some(void_star) = modeled_void_star_in_palace_for_scope(ctx, scope, branch) else {
        return;
    };

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
            involved_stars: vec![void_star],
            involved_mutagens: Vec::new(),
            evidence: vec![PatternEvidence::StarInPalace {
                star: void_star,
                branch,
            }],
            breaking_factors: Vec::new(),
        },
    );
}

fn detect_lu_feng_chong_po(
    ctx: &PatternContext<'_>,
    scope: Scope,
    out: &mut Vec<PatternDetection>,
) {
    let Some(life_branch) = branch_of_palace_for_scope(ctx, scope, PalaceName::Life) else {
        return;
    };

    let support = lu_support_in_san_fang_si_zheng_for_scope(ctx, scope, life_branch);
    for lu_support in support {
        let Some((breaker, breaker_branch)) = breaker_for_lu_support(ctx, scope, lu_support.branch)
        else {
            continue;
        };

        let mut involved_palaces = vec![lu_support.branch, breaker_branch];
        sort_dedup_branches(&mut involved_palaces);

        let mut involved_stars = vec![lu_support.star, breaker];
        involved_stars.sort();
        involved_stars.dedup();

        let mut involved_mutagens = Vec::new();
        let mut evidence = vec![lu_support.evidence];
        if let Some(mutagen) = lu_support.mutagen {
            involved_mutagens.push(mutagen);
        }
        evidence.push(PatternEvidence::StarInPalace {
            star: breaker,
            branch: breaker_branch,
        });
        if is_opposite(lu_support.branch, breaker_branch) {
            evidence.push(PatternEvidence::PalaceRelation {
                from: lu_support.branch,
                to: breaker_branch,
                relation: crate::core::pattern::relation::PalaceRelation::Opposite,
            });
        }

        push_detection(
            out,
            DetectionDraft {
                id: PatternId::LuFengChongPo,
                family: PatternFamily::ShaJi,
                polarity: PatternPolarity::Inauspicious,
                status: PatternStatus::Broken,
                scope,
                anchor: PatternAnchor::Palace(lu_support.branch),
                involved_palaces,
                involved_stars,
                involved_mutagens,
                evidence,
                breaking_factors: vec![PatternCondition::BrokenByStar {
                    star: breaker,
                    branch: breaker_branch,
                }],
            },
        );
    }
}

struct LuSupport {
    star: StarName,
    mutagen: Option<Mutagen>,
    branch: EarthlyBranch,
    evidence: PatternEvidence,
}

fn lu_support_in_san_fang_si_zheng_for_scope(
    ctx: &PatternContext<'_>,
    scope: Scope,
    anchor: EarthlyBranch,
) -> Vec<LuSupport> {
    let mut support = Vec::new();
    for branch in san_fang_si_zheng(anchor) {
        for placement in stars_in_palace_for_scope(ctx, scope, branch) {
            let star = placement.placement().name();
            if star == StarName::LuCun {
                support.push(LuSupport {
                    star,
                    mutagen: None,
                    branch,
                    evidence: PatternEvidence::StarInPalace { star, branch },
                });
            }
            if placement.placement().mutagen() == Some(Mutagen::Lu) {
                support.push(LuSupport {
                    star,
                    mutagen: Some(Mutagen::Lu),
                    branch,
                    evidence: PatternEvidence::MutagenOnStar {
                        star,
                        mutagen: Mutagen::Lu,
                        scope: placement.placement().scope(),
                        branch,
                    },
                });
            }
        }
    }

    if scope != Scope::Natal {
        support.extend(
            mutagen_activations_for_scope(ctx, scope)
                .into_iter()
                .filter(|activation| {
                    activation.mutagen() == Mutagen::Lu
                        && is_in_san_fang_si_zheng(anchor, activation.target_branch())
                })
                .map(|activation| LuSupport {
                    star: activation.target_star(),
                    mutagen: Some(Mutagen::Lu),
                    branch: activation.target_branch(),
                    evidence: PatternEvidence::MutagenOnStar {
                        star: activation.target_star(),
                        mutagen: Mutagen::Lu,
                        scope: activation.source_scope(),
                        branch: activation.target_branch(),
                    },
                }),
        );
    }

    support
}

fn breaker_for_lu_support(
    ctx: &PatternContext<'_>,
    scope: Scope,
    branch: EarthlyBranch,
) -> Option<(StarName, EarthlyBranch)> {
    for breaker_branch in [branch, branch.offset(6)] {
        for placement in stars_in_palace_for_scope(ctx, scope, breaker_branch) {
            let star = placement.placement().name();
            if placement.placement().kind() == StarKind::Tough {
                return Some((star, breaker_branch));
            }
        }
        if let Some(void_star) = modeled_void_star_in_palace_for_scope(ctx, scope, breaker_branch) {
            return Some((void_star, breaker_branch));
        }
    }
    None
}

fn detect_wen_xing_gong_ming(
    ctx: &PatternContext<'_>,
    scope: Scope,
    out: &mut Vec<PatternDetection>,
) {
    let Some(anchor) = branch_of_palace_for_scope(ctx, scope, PalaceName::Life) else {
        return;
    };
    let found = stars_in_san_fang_si_zheng_for_scope(
        ctx,
        scope,
        anchor,
        &[StarName::WenChang, StarName::WenQu],
    );
    if !contains_all(&found, &[StarName::WenChang, StarName::WenQu]) {
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

fn detect_tian_ji_si_hai(ctx: &PatternContext<'_>, scope: Scope, out: &mut Vec<PatternDetection>) {
    let Some(anchor) = branch_of_palace_for_scope(ctx, scope, PalaceName::Life) else {
        return;
    };
    let found = stars_in_san_fang_si_zheng_for_scope(ctx, scope, anchor, &[StarName::TianJi]);
    let Some((star, branch)) = found
        .into_iter()
        .find(|(_, branch)| matches!(branch, EarthlyBranch::Si | EarthlyBranch::Hai))
    else {
        return;
    };

    push_detection(
        out,
        DetectionDraft {
            id: PatternId::TianJiSiHai,
            family: PatternFamily::MajorStarCombination,
            polarity: PatternPolarity::Auspicious,
            status: PatternStatus::Fulfilled,
            scope,
            anchor: PatternAnchor::Palace(branch),
            involved_palaces: vec![branch],
            involved_stars: vec![star],
            involved_mutagens: Vec::new(),
            evidence: vec![PatternEvidence::StarInPalace { star, branch }],
            breaking_factors: Vec::new(),
        },
    );
}

fn detect_zuo_you_tong_gong(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    if !request.scopes.contains(&Scope::Natal) {
        return;
    }
    let Some(branch) = ctx.chart.body_palace_branch() else {
        return;
    };
    if !palace_has_all_stars_for_scope(
        ctx,
        Scope::Natal,
        branch,
        &[StarName::ZuoFu, StarName::YouBi],
    ) {
        return;
    }

    push_detection(
        out,
        DetectionDraft {
            id: PatternId::ZuoYouTongGong,
            family: PatternFamily::AuxiliaryStarCombination,
            polarity: PatternPolarity::Auspicious,
            status: PatternStatus::Fulfilled,
            scope: Scope::Natal,
            anchor: PatternAnchor::Palace(branch),
            involved_palaces: vec![branch],
            involved_stars: vec![StarName::ZuoFu, StarName::YouBi],
            involved_mutagens: Vec::new(),
            evidence: vec![PatternEvidence::StarsInSamePalace {
                stars: vec![StarName::ZuoFu, StarName::YouBi],
                branch,
            }],
            breaking_factors: Vec::new(),
        },
    );
}

fn detect_ming_zhu_chu_hai(
    ctx: &PatternContext<'_>,
    scope: Scope,
    out: &mut Vec<PatternDetection>,
) {
    let Some(anchor) = branch_of_palace_for_scope(ctx, scope, PalaceName::Life) else {
        return;
    };
    let found = stars_in_san_fang_si_zheng_for_scope(
        ctx,
        scope,
        anchor,
        &[StarName::TaiYang, StarName::TaiYin],
    );
    if !contains_all(&found, &[StarName::TaiYang, StarName::TaiYin]) {
        return;
    }
    if !found.iter().all(|(star, branch)| {
        stars_in_palace_for_scope(ctx, scope, *branch)
            .into_iter()
            .any(|placement| {
                placement.placement().name() == *star
                    && is_bright(placement.placement().brightness())
            })
    }) {
        return;
    }

    push_san_fang_detection(
        out,
        PatternId::MingZhuChuHai,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        scope,
        anchor,
        found,
    );
}

fn detect_ming_wu_zheng_yao(
    ctx: &PatternContext<'_>,
    scope: Scope,
    out: &mut Vec<PatternDetection>,
) {
    let Some(branch) = branch_of_palace_for_scope(ctx, scope, PalaceName::Life) else {
        return;
    };
    if major_star_count_in_palace_for_scope(ctx, scope, branch) != 0 {
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
    scope: Scope,
    out: &mut Vec<PatternDetection>,
) {
    let branch = EarthlyBranch::Wu;
    if branch_of_palace_for_scope(ctx, scope, PalaceName::Life) != Some(branch)
        || !palace_has_all_stars_for_scope(ctx, scope, branch, &[StarName::ZiWei])
    {
        return;
    }

    let breaker = san_fang_si_zheng(branch).into_iter().find_map(|candidate| {
        stars_in_palace_for_scope(ctx, scope, candidate)
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

fn detect_fu_xiang_chao_yuan(
    ctx: &PatternContext<'_>,
    scope: Scope,
    out: &mut Vec<PatternDetection>,
) {
    let Some(anchor) = branch_of_palace_for_scope(ctx, scope, PalaceName::Life) else {
        return;
    };
    let found = stars_in_san_fang_si_zheng_for_scope(
        ctx,
        scope,
        anchor,
        &[StarName::TianFu, StarName::TianXiang],
    );
    if !contains_all(&found, &[StarName::TianFu, StarName::TianXiang]) {
        return;
    }

    let wealth = branch_of_palace_for_scope(ctx, scope, PalaceName::Wealth);
    let career = branch_of_palace_for_scope(ctx, scope, PalaceName::Career);
    let split = wealth.zip(career).is_some_and(|(wealth, career)| {
        has_found_at(&found, StarName::TianFu, wealth)
            && has_found_at(&found, StarName::TianXiang, career)
            || has_found_at(&found, StarName::TianFu, career)
                && has_found_at(&found, StarName::TianXiang, wealth)
    });
    let tian_fu_in_life = has_found_at(&found, StarName::TianFu, anchor);
    if !(split || tian_fu_in_life || found.len() >= 2) {
        return;
    }

    push_san_fang_detection(
        out,
        PatternId::FuXiangChaoYuan,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        scope,
        anchor,
        found,
    );
}

fn contains_all(found: &[(StarName, EarthlyBranch)], stars: &[StarName]) -> bool {
    stars
        .iter()
        .all(|star| found.iter().any(|(found_star, _)| found_star == star))
}

fn has_found_at(
    found: &[(StarName, EarthlyBranch)],
    star: StarName,
    branch: EarthlyBranch,
) -> bool {
    found
        .iter()
        .any(|(found_star, found_branch)| *found_star == star && *found_branch == branch)
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
