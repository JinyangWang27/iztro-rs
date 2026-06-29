//! Classical rule evaluators: predicate facts → structured [`Claim`].
//!
//! Each evaluator pairs a rule's data-driven metadata (from the embedded rule
//! corpora) with its hand-coded predicate (from [`super::predicates`]). It
//! returns a typed [`RuleOutcome`]: a claim is emitted only when the condition
//! matches on modeled facts; otherwise the outcome is `NotApplicable` or, for
//! rules whose condition is not yet modeled, `Unsupported`.
//!
//! This module handles rules from multiple provenance corpora (currently QuanShu
//! source rules and project pattern/格局 rules). It is intentionally named for the
//! evaluation role rather than for any one source corpus.

use crate::core::pattern::context::{PatternContext, PatternDetectionRequest};
use crate::core::pattern::detector::detect_patterns;
use crate::core::pattern::model::{
    PatternEvidence as CorePatternEvidence, PatternId, PatternScope,
};
use crate::core::pattern::relation::PalaceRelation;
use crate::core::{Chart, EarthlyBranch, Scope, StarName, StarTag};
use crate::rules::classical::claim::{Claim, ClaimId, ClaimScope, ClaimStrength};
use crate::rules::classical::evidence::{Evidence, EvidenceKind};
use crate::rules::classical::outcome::{RuleOutcome, UnsupportedReason};
use crate::rules::classical::predicates::{
    star_affected_by_void, star_in_branches, star_meets_tag_same_palace, stars_clamp_life,
    sun_and_moon_dim, tian_ma_affected_by_void,
};
use crate::rules::classical::rule::{ClaimSpec, ClassicalRule};
use crate::rules::classical::source_hit::ClassicalSourceHit;
use crate::rules::classical::void::VoidPolicy;

// Stable rule ids handled by this module.
const TIAN_MA_VOID: &str = "migration.tian_ma_void.restless_movement";
const YANG_TUO_CLAMP_LIFE: &str = "life.yang_tuo_clamp_life.constraint_damage";
const CHANG_QU_CLAMP_LIFE: &str = "life.chang_qu_clamp_life.literary_reputation";
const LU_MA_JIAO_CHI: &str = "fortune.lu_ma_jiao_chi.favorable_convergence";
const RI_YUE_FAN_BEI: &str = "life.ri_yue_fan_bei.hardship_pressure";
const TAN_LANG_HAI_ZI: &str = "relationship.tan_ju_hai_zi.water_romance";
const XING_YU_TAN_LANG: &str = "relationship.xing_yu_tan_lang.romance_with_penalty";
const SHAN_FU_JU_KONG: &str = "fortune.shan_fu_ju_kong.monastic_life";
const JIN_CAN_GUANG_HUI: &str = "wealth.jin_can_guang_hui.sun_bright_life_wu";
const RI_CHU_FU_SANG: &str = "status.ri_chu_fu_sang.sun_rising_mao";
const YUE_LUO_HAI_GONG: &str = "status.yue_luo_hai_gong.moon_hai_life";
const YUE_SHENG_CANG_HAI: &str = "wealth.yue_sheng_cang_hai.moon_zi_property";
const MA_TOU_DAI_JIAN: &str = "status.ma_tou_dai_jian.horse_blade";
const TAN_HUO_XIANG_FENG: &str = "status.tan_huo_xiang_feng.tan_lang_fire_star";
const WU_QU_SHOU_YUAN: &str = "status.wu_qu_shou_yuan.wu_qu_life_mao";
const CAI_YU_QIU_CHOU: &str = "hardship.cai_yu_qiu_chou.wu_lian_life_body";
const MA_LUO_KONG_WANG: &str = "migration.ma_luo_kong_wang.horse_void";

/// Evaluates `rule` against `chart`, returning a typed outcome.
///
/// All pilot claims are asserted in the natal scope.
pub fn evaluate(rule: &ClassicalRule, chart: &Chart) -> RuleOutcome {
    match rule.id.as_str() {
        TIAN_MA_VOID => evaluate_tian_ma_void(rule, chart),
        YANG_TUO_CLAMP_LIFE => {
            evaluate_clamp_life(rule, chart, StarName::QingYang, StarName::TuoLuo, None)
        }
        CHANG_QU_CLAMP_LIFE => evaluate_clamp_life(
            rule,
            chart,
            StarName::WenChang,
            StarName::WenQu,
            Some(PatternId::ChangQuJiaMing),
        ),
        RI_YUE_FAN_BEI => evaluate_ri_yue_fan_bei(rule, chart),
        TAN_LANG_HAI_ZI => evaluate_tan_lang_hai_zi(rule, chart),
        XING_YU_TAN_LANG => evaluate_xing_yu_tan_lang(rule, chart),
        SHAN_FU_JU_KONG => evaluate_shan_fu_ju_kong(rule, chart),
        JIN_CAN_GUANG_HUI => evaluate_pattern_detection(rule, chart, PatternId::JinCanGuangHui),
        RI_CHU_FU_SANG => evaluate_pattern_detection(rule, chart, PatternId::RiChuFuSang),
        YUE_LUO_HAI_GONG => evaluate_pattern_detection(rule, chart, PatternId::YueLuoHaiGong),
        YUE_SHENG_CANG_HAI => evaluate_pattern_detection(rule, chart, PatternId::YueShengCangHai),
        MA_TOU_DAI_JIAN => evaluate_pattern_detection(rule, chart, PatternId::MaTouDaiJian),
        TAN_HUO_XIANG_FENG => evaluate_pattern_detection(rule, chart, PatternId::TanHuoXiangFeng),
        WU_QU_SHOU_YUAN => evaluate_pattern_detection(rule, chart, PatternId::WuQuShouYuan),
        CAI_YU_QIU_CHOU => evaluate_pattern_detection(rule, chart, PatternId::CaiYuQiuChou),
        MA_LUO_KONG_WANG => evaluate_pattern_detection(rule, chart, PatternId::MaLuoKongWang),
        // 禄马最喜交驰: the Lu/Tian Ma "交驰" relation is school-dependent and not
        // yet modeled as a deterministic chart fact, so the rule does not fire.
        LU_MA_JIAO_CHI => RuleOutcome::Unsupported(UnsupportedReason::LuMaRelationNotModeled),
        _ => RuleOutcome::NotApplicable,
    }
}

fn evaluate_pattern_detection(
    rule: &ClassicalRule,
    chart: &Chart,
    pattern: PatternId,
) -> RuleOutcome {
    let request = PatternDetectionRequest {
        scopes: vec![Scope::Natal],
        ..PatternDetectionRequest::default()
    };
    let detections = detect_patterns(&PatternContext::natal(chart), &request);
    let Some(detection) = detections
        .iter()
        .find(|detection| detection.id == pattern && detection.scope == PatternScope::Natal)
    else {
        return RuleOutcome::NotApplicable;
    };

    let mut evidence = vec![Evidence::new(EvidenceKind::PatternShapeMatched { pattern })];
    for item in &detection.evidence {
        evidence.extend(pattern_evidence_to_classical(item));
    }
    matched(rule, evidence)
}

fn pattern_evidence_to_classical(evidence: &CorePatternEvidence) -> Vec<Evidence> {
    match evidence {
        CorePatternEvidence::StarInPalace { star, branch } => {
            vec![Evidence::new(EvidenceKind::StarInPalace {
                star: *star,
                branch: *branch,
            })]
        }
        CorePatternEvidence::StarInPalaceRelation {
            star,
            anchor,
            branch,
            relation,
        } => vec![
            Evidence::new(EvidenceKind::StarInPalace {
                star: *star,
                branch: *branch,
            }),
            Evidence::new(EvidenceKind::PalaceRelation {
                from: *anchor,
                to: *branch,
                relation: *relation,
            }),
        ],
        CorePatternEvidence::StarsInSamePalace { stars, branch } => stars
            .iter()
            .map(|star| {
                Evidence::new(EvidenceKind::StarInPalace {
                    star: *star,
                    branch: *branch,
                })
            })
            .collect(),
        CorePatternEvidence::MutagenOnStar {
            star,
            mutagen,
            branch,
            ..
        } => vec![Evidence::new(EvidenceKind::MutagenInPalace {
            star: *star,
            mutagen: *mutagen,
            branch: *branch,
        })],
        CorePatternEvidence::PalaceRelation { from, to, relation } => {
            vec![Evidence::new(EvidenceKind::PalaceRelation {
                from: *from,
                to: *to,
                relation: *relation,
            })]
        }
        CorePatternEvidence::StarsInSanFangSiZheng { .. } => Vec::new(),
    }
}

/// Builds a natal claim from rule metadata and the given evidence.
fn build_claim(rule: &ClassicalRule, spec: &ClaimSpec, evidence: Vec<Evidence>) -> Claim {
    let scope = ClaimScope::Natal;
    Claim {
        id: ClaimId::new(&rule.id, scope),
        rule_id: rule.id.clone(),
        domain: spec.domain,
        themes: spec.themes.clone(),
        polarity: spec.polarity,
        strength: ClaimStrength::new(spec.base_strength),
        scope,
        evidence,
        counter_evidence: Vec::new(),
        source_refs: vec![rule.source_ref()],
        claim_key: spec.claim_key.clone(),
    }
}

fn matched(rule: &ClassicalRule, evidence: Vec<Evidence>) -> RuleOutcome {
    let scope = ClaimScope::Natal;
    let source_hit = ClassicalSourceHit::from_rule(rule, scope, evidence.clone());
    let claim = rule
        .claim
        .as_ref()
        .map(|spec| Box::new(build_claim(rule, spec, evidence)));

    RuleOutcome::Matched {
        source_hit: Box::new(source_hit),
        claim,
    }
}

fn evaluate_tian_ma_void(rule: &ClassicalRule, chart: &Chart) -> RuleOutcome {
    match tian_ma_affected_by_void(chart, VoidPolicy::DEFAULT) {
        Some(fact) => {
            let evidence = vec![Evidence::new(EvidenceKind::StarAffectedByVoid {
                star: StarName::TianMa,
                void_kind: fact.void_kind,
                branch: fact.tian_ma_branch,
            })];
            matched(rule, evidence)
        }
        None => RuleOutcome::NotApplicable,
    }
}

fn evaluate_clamp_life(
    rule: &ClassicalRule,
    chart: &Chart,
    star_a: StarName,
    star_b: StarName,
    corroborating_pattern: Option<PatternId>,
) -> RuleOutcome {
    let Some(clamp) = stars_clamp_life(chart, star_a, star_b) else {
        return RuleOutcome::NotApplicable;
    };

    let mut evidence = vec![
        Evidence::new(EvidenceKind::StarClampsPalace {
            star: clamp.low.0,
            clamp_branch: clamp.low.1,
            target_branch: clamp.life_branch,
        }),
        Evidence::new(EvidenceKind::StarClampsPalace {
            star: clamp.high.0,
            clamp_branch: clamp.high.1,
            target_branch: clamp.life_branch,
        }),
        Evidence::new(EvidenceKind::PalaceRelation {
            from: clamp.life_branch,
            to: clamp.low.1,
            relation: PalaceRelation::ClampedBy,
        }),
        Evidence::new(EvidenceKind::PalaceRelation {
            from: clamp.life_branch,
            to: clamp.high.1,
            relation: PalaceRelation::ClampedBy,
        }),
    ];
    if let Some(pattern) = corroborating_pattern {
        evidence.push(Evidence::new(EvidenceKind::PatternShapeMatched { pattern }));
    }

    matched(rule, evidence)
}

fn evaluate_ri_yue_fan_bei(rule: &ClassicalRule, chart: &Chart) -> RuleOutcome {
    match sun_and_moon_dim(chart) {
        Some(fact) => {
            let evidence = vec![
                Evidence::new(EvidenceKind::BrightnessCondition {
                    star: StarName::TaiYang,
                    brightness: fact.sun.1,
                    branch: fact.sun.0,
                }),
                Evidence::new(EvidenceKind::BrightnessCondition {
                    star: StarName::TaiYin,
                    brightness: fact.moon.1,
                    branch: fact.moon.0,
                }),
            ];
            matched(rule, evidence)
        }
        None => RuleOutcome::NotApplicable,
    }
}

/// 贪居亥子，名为犯水桃花. Conservatively: 贪狼 sits in the 亥 or 子 branch.
fn evaluate_tan_lang_hai_zi(rule: &ClassicalRule, chart: &Chart) -> RuleOutcome {
    let Some(fact) = star_in_branches(
        chart,
        StarName::TanLang,
        &[EarthlyBranch::Hai, EarthlyBranch::Zi],
    ) else {
        return RuleOutcome::NotApplicable;
    };

    matched(
        rule,
        vec![Evidence::new(EvidenceKind::StarInPalace {
            star: fact.star,
            branch: fact.branch,
        })],
    )
}

/// 刑遇贪狼，号曰风流彩杖. Conservatively: 贪狼 shares a palace with a 刑曜
/// ([`StarTag::Punishment`] = 擎羊、天刑).
fn evaluate_xing_yu_tan_lang(rule: &ClassicalRule, chart: &Chart) -> RuleOutcome {
    let encounters = star_meets_tag_same_palace(chart, StarName::TanLang, StarTag::Punishment);

    if encounters.is_empty() {
        return RuleOutcome::NotApplicable;
    }

    let branch = encounters[0].branch;
    let mut evidence = Vec::with_capacity(encounters.len() + 1);

    evidence.push(Evidence::new(EvidenceKind::StarInPalace {
        star: StarName::TanLang,
        branch,
    }));

    for encounter in encounters {
        evidence.push(Evidence::new(EvidenceKind::StarInPalace {
            star: encounter.matched_star,
            branch: encounter.branch,
        }));
    }

    matched(rule, evidence)
}

/// 善福居空位，天竺生涯. Conservatively: 善=天机 and 福=天同, and both are
/// affected by modeled 空亡-family stars.
fn evaluate_shan_fu_ju_kong(rule: &ClassicalRule, chart: &Chart) -> RuleOutcome {
    let Some(tian_ji) = star_affected_by_void(chart, StarName::TianJi, VoidPolicy::DEFAULT) else {
        return RuleOutcome::NotApplicable;
    };
    let Some(tian_tong) = star_affected_by_void(chart, StarName::TianTong, VoidPolicy::DEFAULT)
    else {
        return RuleOutcome::NotApplicable;
    };

    matched(
        rule,
        vec![
            Evidence::new(EvidenceKind::StarAffectedByVoid {
                star: tian_ji.star,
                void_kind: tian_ji.void_kind,
                branch: tian_ji.branch,
            }),
            Evidence::new(EvidenceKind::StarAffectedByVoid {
                star: tian_tong.star,
                void_kind: tian_tong.void_kind,
                branch: tian_tong.branch,
            }),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::EarthlyBranch;
    use crate::rules::classical::rule::{ClassicalRuleId, RuleStatus};
    use crate::rules::classical::source::ClassicalWork;

    #[test]
    fn matched_rule_without_claim_spec_emits_source_hit_only() {
        let rule = ClassicalRule {
            id: ClassicalRuleId::new("experimental.source_hit_only"),
            source_id: "pattern.source_hit_only".to_string(),
            source_clause_id: Some("source_hit_clause".to_string()),
            work: ClassicalWork::IztroPatternCatalog,
            source_text_zh_hans: "只记录出处命中".to_string(),
            normalized_note_zh_hans: Some("测试无判断元数据的出处命中。".to_string()),
            status: RuleStatus::Executable,
            school: Default::default(),
            claim: None,
        };
        let evidence = vec![Evidence::new(EvidenceKind::StarInPalace {
            star: StarName::TianMa,
            branch: EarthlyBranch::Wu,
        })];

        let outcome = matched(&rule, evidence.clone());

        let RuleOutcome::Matched { source_hit, claim } = outcome else {
            panic!("expected matched source hit");
        };
        assert!(claim.is_none());
        assert_eq!(source_hit.rule_id, rule.id);
        assert_eq!(source_hit.work, ClassicalWork::IztroPatternCatalog);
        assert_eq!(source_hit.source_id, "pattern.source_hit_only");
        assert_eq!(
            source_hit.source_clause_id.as_deref(),
            Some("source_hit_clause")
        );
        assert_eq!(source_hit.source_text_zh_hans, "只记录出处命中");
        assert_eq!(source_hit.status, RuleStatus::Executable);
        assert_eq!(source_hit.scope, ClaimScope::Natal);
        assert_eq!(source_hit.evidence, evidence);
    }
}
