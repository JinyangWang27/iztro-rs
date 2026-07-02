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
//!
//! Canonical 格局/pattern identity lives in `rules::pattern` (`PatternId` →
//! `PatternDetection`). This evaluator does not re-derive QuanShu pattern
//! catalogue entries as separate classical runtime rules; it only references a
//! `PatternId` as corroborating evidence for project-owned pattern rules.

use crate::core::{Chart, EarthlyBranch, Scope, StarName, StarTag};
use crate::rules::classical::claim::{Claim, ClaimId, ClaimScope, ClaimStrength};
use crate::rules::classical::context::ClassicalRuleContext;
use crate::rules::classical::evidence::{Evidence, EvidenceKind};
use crate::rules::classical::outcome::{RuleOutcome, UnsupportedReason};
use crate::rules::classical::predicates::{
    LifeClamp, selected_stars_clamp_life, star_affected_by_void, star_in_branches,
    star_meets_tag_same_palace, stars_clamp_life, sun_and_moon_dim, tian_ma_affected_by_void,
};
use crate::rules::classical::rule::{ClaimSpec, ClassicalRule};
use crate::rules::classical::scope_registry::{
    CHANG_QU_CLAMP_LIFE, HANDLED_RULE_IDS, LU_MA_JIAO_CHI, RI_YUE_FAN_BEI, SHAN_FU_JU_KONG,
    TAN_LANG_HAI_ZI, TIAN_MA_VOID, XING_YU_TAN_LANG, YANG_TUO_CLAMP_LIFE, is_overlay_aware_rule,
};
use crate::rules::classical::source_hit::ClassicalSourceHit;
use crate::rules::classical::void::VoidPolicy;
use crate::rules::pattern::model::PatternId;
use crate::rules::pattern::relation::PalaceRelation;

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
        // 禄马最喜交驰: the Lu/Tian Ma "交驰" relation is school-dependent and not
        // yet modeled as a deterministic chart fact, so the rule does not fire.
        LU_MA_JIAO_CHI => RuleOutcome::Unsupported(UnsupportedReason::LuMaRelationNotModeled),
        id => {
            // A handled id reaching this arm means a match arm above was removed
            // without updating HANDLED_RULE_IDS (or vice versa).
            debug_assert!(
                !HANDLED_RULE_IDS.contains(&id),
                "rule id {id} is in HANDLED_RULE_IDS but has no predicate arm",
            );
            RuleOutcome::NotApplicable
        }
    }
}

/// Evaluates `rule` against a full classical context.
///
/// Existing executable rules keep natal semantics. 昌曲夹命 is the first
/// selected-state vertical slice, so it can match against the selected frame's
/// effective chart state.
pub fn evaluate_in_context(rule: &ClassicalRule, ctx: &ClassicalRuleContext<'_>) -> RuleOutcome {
    if is_overlay_aware_rule(rule.id.as_str()) {
        return match rule.id.as_str() {
            CHANG_QU_CLAMP_LIFE => evaluate_clamp_life_in_context(
                rule,
                ctx,
                StarName::WenChang,
                StarName::WenQu,
                Some(PatternId::ChangQuJiaMing),
            ),
            id => {
                debug_assert!(
                    !is_overlay_aware_rule(id),
                    "overlay-aware rule id {id} has no context evaluator arm",
                );
                RuleOutcome::NotApplicable
            }
        };
    }

    evaluate(rule, ctx.chart())
}

fn build_claim(
    rule: &ClassicalRule,
    spec: &ClaimSpec,
    scope: ClaimScope,
    evidence: Vec<Evidence>,
) -> Claim {
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
    matched_with_scope(rule, ClaimScope::Natal, evidence)
}

fn matched_with_scope(
    rule: &ClassicalRule,
    scope: ClaimScope,
    evidence: Vec<Evidence>,
) -> RuleOutcome {
    let source_hit = ClassicalSourceHit::from_rule(rule, scope, evidence.clone());
    let claim = rule
        .claim
        .as_ref()
        .map(|spec| Box::new(build_claim(rule, spec, scope, evidence)));

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

    matched(rule, clamp_life_evidence(clamp, corroborating_pattern))
}

fn evaluate_clamp_life_in_context(
    rule: &ClassicalRule,
    ctx: &ClassicalRuleContext<'_>,
    star_a: StarName,
    star_b: StarName,
    corroborating_pattern: Option<PatternId>,
) -> RuleOutcome {
    let Some(scope) = ctx.selected_frame_scope().map(claim_scope_for_frame_scope) else {
        return RuleOutcome::NotApplicable;
    };
    let Some(clamp) = selected_stars_clamp_life(ctx.as_rule_context(), star_a, star_b) else {
        return RuleOutcome::NotApplicable;
    };

    matched_with_scope(
        rule,
        scope,
        clamp_life_evidence(clamp, corroborating_pattern),
    )
}

fn clamp_life_evidence(
    clamp: LifeClamp,
    corroborating_pattern: Option<PatternId>,
) -> Vec<Evidence> {
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

    evidence
}

fn claim_scope_for_frame_scope(scope: Scope) -> ClaimScope {
    match scope {
        Scope::Natal => ClaimScope::Natal,
        Scope::Decadal => ClaimScope::Decadal,
        Scope::Age => ClaimScope::Age,
        Scope::Yearly => ClaimScope::Yearly,
        Scope::Monthly => ClaimScope::Monthly,
        Scope::Daily => ClaimScope::Daily,
        Scope::Hourly => ClaimScope::Hourly,
    }
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

    use crate::core::{
        BirthContext, CalendarDate, EarthlyBranch, Gender, HeavenlyStem, MethodProfile, StemBranch,
        build_empty_chart,
    };
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

    /// Every corpus rule authored as `Executable` (or `Tested`) must have a
    /// predicate arm in [`evaluate`]; otherwise it silently never fires. This is
    /// the safety net for the hard-coded `match rule.id` dispatch.
    #[test]
    fn every_executable_corpus_rule_is_handled() {
        for rule in crate::rules::classical::corpus::classical_rules() {
            if matches!(rule.status, RuleStatus::Executable | RuleStatus::Tested) {
                assert!(
                    HANDLED_RULE_IDS.contains(&rule.id.as_str()),
                    "corpus rule {} is `{:?}` but has no predicate arm in `evaluate`; \
                     it would silently return NotApplicable. Add a match arm and list its \
                     id in HANDLED_RULE_IDS.",
                    rule.id,
                    rule.status,
                );
            }
        }
    }

    /// Conversely, every id we claim to handle must resolve to a real corpus
    /// rule, so the handled set can't reference a renamed or deleted rule. (The
    /// status is intentionally not constrained: some handled rules are still
    /// `Normalized` and map to a typed `Unsupported` reason rather than silently
    /// returning `NotApplicable` — e.g. `fortune.lu_ma_jiao_chi.*`.)
    #[test]
    fn handled_rule_ids_reference_real_corpus_rules() {
        for id in HANDLED_RULE_IDS {
            assert!(
                crate::rules::classical::corpus::rule_by_id(id).is_some(),
                "handled rule id {id} is not in the corpus (renamed or deleted?)",
            );
        }
    }

    /// Proves every `HANDLED_RULE_IDS` entry has a *live dispatch arm*, not just a
    /// listing. If a listed id fell through to the `_` arm of [`evaluate`], the
    /// `debug_assert!` there fires; this test exercises `evaluate` once per id
    /// (debug assertions are enabled under `cargo test`), so removing an arm while
    /// leaving the id in the list is caught here.
    #[test]
    fn every_handled_id_reaches_a_live_dispatch_arm() {
        let chart = build_empty_chart(
            BirthContext::new(
                CalendarDate::solar(1990, 5, 17),
                EarthlyBranch::Chen,
                Gender::Female,
            ),
            StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu).expect("valid stem-branch"),
            MethodProfile::placeholder("dispatch_guardrail"),
        )
        .expect("empty chart should build");

        for id in HANDLED_RULE_IDS {
            let rule = crate::rules::classical::corpus::rule_by_id(id)
                .unwrap_or_else(|| panic!("handled rule id {id} is not in the corpus"));
            // Reaching the default arm for a handled id trips the debug_assert!.
            let _ = evaluate(rule, &chart);
        }
    }
}
