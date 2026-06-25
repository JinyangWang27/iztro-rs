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

use crate::core::pattern::model::PatternId;
use crate::core::pattern::relation::PalaceRelation;
use crate::core::{Chart, StarName};
use crate::rules::classical::claim::{Claim, ClaimId, ClaimScope, ClaimStrength};
use crate::rules::classical::evidence::{Evidence, EvidenceKind};
use crate::rules::classical::outcome::{RuleOutcome, UnsupportedReason};
use crate::rules::classical::predicates::{
    stars_clamp_life, sun_and_moon_dim, tian_ma_affected_by_void,
};
use crate::rules::classical::rule::{ClaimSpec, ClassicalRule};
use crate::rules::classical::source_hit::ClassicalSourceHit;
use crate::rules::classical::void::VoidPolicy;

// Stable rule ids handled by this module.
const TIAN_MA_VOID: &str = "migration.tian_ma_void.restless_movement";
const YANG_TUO_CLAMP_LIFE: &str = "life.yang_tuo_clamp_life.constraint_damage";
const CHANG_QU_CLAMP_LIFE: &str = "life.chang_qu_clamp_life.literary_reputation";
const LU_MA_REMOTE_WEALTH: &str = "wealth.lu_ma_remote_wealth";
const RI_YUE_FAN_BEI: &str = "life.ri_yue_fan_bei.hardship_pressure";

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
        // 禄马交驰: the Lu/Tian Ma "交驰" relation is school-dependent and not yet
        // modeled as a deterministic chart fact, so the rule does not fire.
        LU_MA_REMOTE_WEALTH => RuleOutcome::Unsupported(UnsupportedReason::LuMaRelationNotModeled),
        _ => RuleOutcome::NotApplicable,
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
