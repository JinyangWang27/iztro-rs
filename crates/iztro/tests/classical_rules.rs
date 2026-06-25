//! Integration tests for the classical rule engine.
//!
//! These build small synthetic charts with full control over star placement so
//! each pilot rule's condition is exercised deterministically, mirroring the
//! approach in `tests/patterns.rs`.

use iztro::rules::classical::{
    Claim, ClaimDomain, ClaimEvaluationRequest, ClaimPolarity, ClaimScope, ClaimTheme,
    ClassicalRule, ClassicalRuleId, ClassicalWork, Evidence, EvidenceKind, RuleStatus,
    UnsupportedReason, VoidKind, evaluate_classical, evaluate_classical_claims, quan_shu_rules,
    rule_by_id,
};
use iztro::{
    BirthContext, Brightness, CalendarDate, Chart, EarthlyBranch, Gender, HeavenlyStem, Mutagen,
    PALACE_NAMES, Palace, Scope, StarKind, StarName, StarPlacement, StemBranch,
};

// ---- synthetic chart builders --------------------------------------------

/// One synthetic star placement: (branch, star, kind, optional mutagen).
type Spec = (EarthlyBranch, StarName, StarKind, Option<Mutagen>);

/// Builds a 12-palace natal chart with the Life palace at `life_branch`, every
/// placement carrying `Brightness::Unknown`.
fn build_chart(life_branch: EarthlyBranch, placements: &[Spec]) -> Chart {
    let palaces: Vec<Palace> = (0..12)
        .map(|index| {
            let name = PALACE_NAMES[index];
            let branch = life_branch.offset(index as isize);
            let stars: Vec<StarPlacement> = placements
                .iter()
                .filter(|(spec_branch, ..)| *spec_branch == branch)
                .map(|(_, star, kind, mutagen)| {
                    StarPlacement::new(*star, *kind, Brightness::Unknown, *mutagen, Scope::Natal)
                })
                .collect();
            Palace::new(name, branch, HeavenlyStem::Jia, stars)
        })
        .collect();
    assemble(palaces)
}

/// One brightness-carrying placement: (branch, star, kind, brightness).
type BrightSpec = (EarthlyBranch, StarName, StarKind, Brightness);

/// Builds a chart where each placement carries an explicit brightness.
fn build_chart_bright(life_branch: EarthlyBranch, placements: &[BrightSpec]) -> Chart {
    let palaces: Vec<Palace> = (0..12)
        .map(|index| {
            let name = PALACE_NAMES[index];
            let branch = life_branch.offset(index as isize);
            let stars: Vec<StarPlacement> = placements
                .iter()
                .filter(|(spec_branch, ..)| *spec_branch == branch)
                .map(|(_, star, kind, brightness)| {
                    StarPlacement::new(*star, *kind, *brightness, None, Scope::Natal)
                })
                .collect();
            Palace::new(name, branch, HeavenlyStem::Jia, stars)
        })
        .collect();
    assemble(palaces)
}

fn assemble(palaces: Vec<Palace>) -> Chart {
    Chart::try_new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu).expect("valid stem-branch"),
        iztro::MethodProfile::placeholder("classical_test"),
        palaces,
        None,
        None,
    )
    .expect("synthetic chart should build")
}

fn tough(branch: EarthlyBranch, star: StarName) -> Spec {
    (branch, star, StarKind::Tough, None)
}

fn soft(branch: EarthlyBranch, star: StarName) -> Spec {
    (branch, star, StarKind::Soft, None)
}

fn adj(branch: EarthlyBranch, star: StarName) -> Spec {
    (branch, star, StarKind::Adjective, None)
}

fn tian_ma(branch: EarthlyBranch) -> Spec {
    (branch, StarName::TianMa, StarKind::TianMa, None)
}

fn claim_ids(claims: &[Claim]) -> Vec<String> {
    claims.iter().map(|c| c.rule_id.to_string()).collect()
}

fn has_rule(claims: &[Claim], id: &str) -> bool {
    claims.iter().any(|c| c.rule_id.as_str() == id)
}

const TIAN_MA_VOID: &str = "migration.tian_ma_void.restless_movement";
const YANG_TUO: &str = "life.yang_tuo_clamp_life.constraint_damage";
const CHANG_QU: &str = "life.chang_qu_clamp_life.literary_reputation";
const LU_MA: &str = "wealth.lu_ma_remote_wealth";
const RI_YUE: &str = "life.ri_yue_fan_bei.hardship_pressure";

// ---- corpus deserialization ----------------------------------------------

#[test]
fn corpus_deserializes_all_pilot_rules() {
    let rules = quan_shu_rules();
    assert_eq!(rules.len(), 5);
    for id in [TIAN_MA_VOID, YANG_TUO, CHANG_QU, LU_MA, RI_YUE] {
        assert!(rule_by_id(id).is_some(), "missing rule {id}");
    }
}

#[test]
fn corpus_fields_match_metadata() {
    let migration = rule_by_id(TIAN_MA_VOID).expect("rule present");
    assert_eq!(migration.work, ClassicalWork::ZiWeiDouShuQuanShu);
    assert_eq!(migration.source_text_zh_hans, "马落空亡，终身奔走");
    assert_eq!(migration.status, RuleStatus::Executable);
    assert_eq!(migration.domain, ClaimDomain::Migration);
    assert_eq!(migration.polarity, ClaimPolarity::MixedNegative);
    assert_eq!(
        migration.themes,
        vec![ClaimTheme::RestlessMovement, ClaimTheme::Instability]
    );
    assert!((migration.base_strength - 0.60).abs() < 1e-6);

    // 禄马交驰 is metadata-only / not executable.
    let lu_ma = rule_by_id(LU_MA).expect("rule present");
    assert_eq!(lu_ma.status, RuleStatus::Normalized);
}

// ---- enum serde names ------------------------------------------------------

#[test]
fn enum_serde_names_are_snake_case() {
    use serde_json::json;
    assert_eq!(
        serde_json::to_value(ClaimDomain::Migration).unwrap(),
        json!("migration")
    );
    assert_eq!(
        serde_json::to_value(ClaimPolarity::MixedNegative).unwrap(),
        json!("mixed_negative")
    );
    assert_eq!(
        serde_json::to_value(ClaimTheme::RestlessMovement).unwrap(),
        json!("restless_movement")
    );
    assert_eq!(
        serde_json::to_value(RuleStatus::Executable).unwrap(),
        json!("executable")
    );
    assert_eq!(
        serde_json::to_value(ClassicalWork::ZiWeiDouShuQuanShu).unwrap(),
        json!("zi_wei_dou_shu_quan_shu")
    );

    // Round-trip a full rule through JSON.
    let rule = rule_by_id(TIAN_MA_VOID).unwrap();
    let value = serde_json::to_value(rule).unwrap();
    let back: ClassicalRule = serde_json::from_value(value).unwrap();
    assert_eq!(&back, rule);
}

// ---- 马落空亡 (executable; conservative void policy) -----------------------

#[test]
fn tian_ma_void_positive_on_modeled_void_star() {
    // TianMa shares a palace with 旬空 (a modeled 空亡-family star).
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            tian_ma(EarthlyBranch::Wu),
            adj(EarthlyBranch::Wu, StarName::XunKong),
        ],
    );
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    let claim = claims
        .iter()
        .find(|c| c.rule_id.as_str() == TIAN_MA_VOID)
        .expect("expected 马落空亡 claim");
    assert_eq!(claim.domain, ClaimDomain::Migration);
    assert_eq!(claim.scope, ClaimScope::Natal);
    assert!(claim.evidence.iter().any(|e| matches!(
        e.kind(),
        EvidenceKind::StarAffectedByVoid {
            star: StarName::TianMa,
            void_kind: VoidKind::XunKong,
            branch: EarthlyBranch::Wu,
        }
    )));
    assert_eq!(
        claim.source_refs[0].source_text_zh_hans,
        "马落空亡，终身奔走"
    );
}

#[test]
fn tian_ma_void_negative_when_void_in_other_palace() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            tian_ma(EarthlyBranch::Wu),
            adj(EarthlyBranch::Zi, StarName::XunKong),
        ],
    );
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    assert!(!has_rule(&claims, TIAN_MA_VOID));
}

#[test]
fn tian_ma_void_does_not_fire_on_tian_kong() {
    // 天空 (TianKong) is NOT a 空亡-family star: it must never trigger the rule.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            tian_ma(EarthlyBranch::Wu),
            adj(EarthlyBranch::Wu, StarName::TianKong),
        ],
    );
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    assert!(!has_rule(&claims, TIAN_MA_VOID));
}

// ---- 羊陀夹命 --------------------------------------------------------------

#[test]
fn yang_tuo_clamp_life_positive() {
    // Life@Zi; clamp(Zi) = {Hai, Chou}. QingYang@Hai, TuoLuo@Chou.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            tough(EarthlyBranch::Hai, StarName::QingYang),
            tough(EarthlyBranch::Chou, StarName::TuoLuo),
        ],
    );
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    let claim = claims
        .iter()
        .find(|c| c.rule_id.as_str() == YANG_TUO)
        .expect("expected 羊陀夹命 claim");
    assert_eq!(claim.domain, ClaimDomain::Life);
    assert_eq!(claim.polarity, ClaimPolarity::Negative);
    let clamp_count = claim
        .evidence
        .iter()
        .filter(|e| matches!(e.kind(), EvidenceKind::StarClampsPalace { .. }))
        .count();
    assert_eq!(clamp_count, 2);
}

#[test]
fn yang_tuo_clamp_life_negative_when_not_clamping() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            tough(EarthlyBranch::Hai, StarName::QingYang),
            tough(EarthlyBranch::Wu, StarName::TuoLuo),
        ],
    );
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    assert!(!has_rule(&claims, YANG_TUO));
}

// ---- 昌曲夹命 --------------------------------------------------------------

#[test]
fn chang_qu_clamp_life_positive_emits_claim_with_pattern_evidence() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Hai, StarName::WenChang),
            soft(EarthlyBranch::Chou, StarName::WenQu),
        ],
    );
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    let claim = claims
        .iter()
        .find(|c| c.rule_id.as_str() == CHANG_QU)
        .expect("expected 昌曲夹命 claim");
    assert_eq!(claim.domain, ClaimDomain::Life);
    assert_eq!(claim.polarity, ClaimPolarity::Positive);
    assert!(claim.themes.contains(&ClaimTheme::LiteraryTalent));
    assert!(claim.evidence.iter().any(|e| matches!(
        e.kind(),
        EvidenceKind::PatternDetected {
            pattern: iztro::PatternId::ChangQuJiaMing
        }
    )));
}

#[test]
fn chang_qu_clamp_life_negative_when_one_star_outside() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Hai, StarName::WenChang),
            soft(EarthlyBranch::Wu, StarName::WenQu),
        ],
    );
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    assert!(!has_rule(&claims, CHANG_QU));
}

// ---- 日月反背 --------------------------------------------------------------

#[test]
fn ri_yue_fan_bei_positive_when_both_dim() {
    let chart = build_chart_bright(
        EarthlyBranch::Zi,
        &[
            (
                EarthlyBranch::Si,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Trapped,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Weak,
            ),
        ],
    );
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    let claim = claims
        .iter()
        .find(|c| c.rule_id.as_str() == RI_YUE)
        .expect("expected 日月反背 claim");
    let brightness_count = claim
        .evidence
        .iter()
        .filter(|e| matches!(e.kind(), EvidenceKind::BrightnessCondition { .. }))
        .count();
    assert_eq!(brightness_count, 2);
}

#[test]
fn ri_yue_fan_bei_negative_when_one_bright() {
    let chart = build_chart_bright(
        EarthlyBranch::Zi,
        &[
            (
                EarthlyBranch::Si,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Trapped,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Prosperous,
            ),
        ],
    );
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    assert!(!has_rule(&claims, RI_YUE));
}

// ---- 禄马交驰 (metadata-only / unsupported, typed + visible) ---------------

#[test]
fn lu_ma_is_unsupported_and_never_emits() {
    let chart = build_chart(EarthlyBranch::Zi, &[tian_ma(EarthlyBranch::Wu)]);
    let evaluation = evaluate_classical(&chart, &ClaimEvaluationRequest::default());

    assert!(
        !has_rule(&evaluation.claims, LU_MA),
        "禄马交驰 must not emit a claim"
    );
    let diagnostic = evaluation
        .diagnostics
        .iter()
        .find(|d| d.rule_id.as_str() == LU_MA)
        .expect("expected a typed diagnostic for 禄马交驰");
    assert_eq!(diagnostic.reason, UnsupportedReason::LuMaRelationNotModeled);
}

// ---- deterministic sorting -------------------------------------------------

/// A chart fulfilling 羊陀夹命 + 昌曲夹命 (both Life) and 马落空亡 (Migration).
fn multi_claim_chart() -> Chart {
    build_chart(
        EarthlyBranch::Zi,
        &[
            tough(EarthlyBranch::Hai, StarName::QingYang),
            tough(EarthlyBranch::Chou, StarName::TuoLuo),
            soft(EarthlyBranch::Hai, StarName::WenChang),
            soft(EarthlyBranch::Chou, StarName::WenQu),
            tian_ma(EarthlyBranch::Wu),
            adj(EarthlyBranch::Wu, StarName::KongWang),
        ],
    )
}

#[test]
fn claims_are_sorted_by_scope_domain_rule_key() {
    let chart = multi_claim_chart();
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    // Life domain sorts before Migration; within Life, rule ids sort
    // lexicographically (chang_qu before yang_tuo).
    assert_eq!(claim_ids(&claims), vec![CHANG_QU, YANG_TUO, TIAN_MA_VOID]);
}

// ---- request filtering -----------------------------------------------------

#[test]
fn filter_by_domain() {
    let chart = multi_claim_chart();
    let request = ClaimEvaluationRequest {
        domains: vec![ClaimDomain::Migration],
        ..Default::default()
    };
    let claims = evaluate_classical_claims(&chart, &request);
    assert_eq!(claim_ids(&claims), vec![TIAN_MA_VOID]);
}

#[test]
fn filter_by_theme() {
    let chart = multi_claim_chart();
    let request = ClaimEvaluationRequest {
        themes: vec![ClaimTheme::LiteraryTalent],
        ..Default::default()
    };
    let claims = evaluate_classical_claims(&chart, &request);
    assert_eq!(claim_ids(&claims), vec![CHANG_QU]);
}

#[test]
fn filter_by_polarity() {
    let chart = multi_claim_chart();
    let request = ClaimEvaluationRequest {
        polarities: vec![ClaimPolarity::Positive],
        ..Default::default()
    };
    let claims = evaluate_classical_claims(&chart, &request);
    assert_eq!(claim_ids(&claims), vec![CHANG_QU]);
}

#[test]
fn filter_by_rule_id() {
    let chart = multi_claim_chart();
    let request = ClaimEvaluationRequest {
        rule_ids: vec![ClassicalRuleId::new(YANG_TUO)],
        ..Default::default()
    };
    let claims = evaluate_classical_claims(&chart, &request);
    assert_eq!(claim_ids(&claims), vec![YANG_TUO]);
}

#[test]
fn filter_by_work_includes_all_quan_shu_claims() {
    let chart = multi_claim_chart();
    let request = ClaimEvaluationRequest {
        works: vec![ClassicalWork::ZiWeiDouShuQuanShu],
        ..Default::default()
    };
    let claims = evaluate_classical_claims(&chart, &request);
    assert_eq!(claims.len(), 3);
}

#[test]
fn filter_by_scope() {
    let chart = multi_claim_chart();
    // No claims are asserted in the Yearly scope yet.
    let yearly = ClaimEvaluationRequest {
        scopes: vec![ClaimScope::Yearly],
        ..Default::default()
    };
    assert!(evaluate_classical_claims(&chart, &yearly).is_empty());

    let natal = ClaimEvaluationRequest {
        scopes: vec![ClaimScope::Natal],
        ..Default::default()
    };
    assert_eq!(evaluate_classical_claims(&chart, &natal).len(), 3);
}

// ---- JSON export -----------------------------------------------------------

#[test]
fn claims_serialize_to_deterministic_json_with_required_fields() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            tian_ma(EarthlyBranch::Wu),
            adj(EarthlyBranch::Wu, StarName::XunKong),
        ],
    );
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    let claim = claims
        .iter()
        .find(|c| c.rule_id.as_str() == TIAN_MA_VOID)
        .expect("claim present");

    // Determinism: serializing twice yields identical output.
    let first = serde_json::to_string(claim).unwrap();
    let second = serde_json::to_string(claim).unwrap();
    assert_eq!(first, second);

    let value: serde_json::Value = serde_json::from_str(&first).unwrap();
    let obj = value.as_object().unwrap();
    for field in [
        "id",
        "rule_id",
        "domain",
        "themes",
        "polarity",
        "strength",
        "scope",
        "evidence",
        "counter_evidence",
        "source_refs",
        "claim_key",
    ] {
        assert!(obj.contains_key(field), "missing JSON field {field}");
    }
    assert_eq!(obj["rule_id"], serde_json::json!(TIAN_MA_VOID));
    assert_eq!(
        obj["claim_key"],
        serde_json::json!("claim.migration.tian-ma-void.restless-movement")
    );
    assert_eq!(obj["domain"], serde_json::json!("migration"));
    assert_eq!(obj["polarity"], serde_json::json!("mixed_negative"));
    // Source Chinese text is preserved verbatim.
    assert_eq!(
        obj["source_refs"][0]["source_text_zh_hans"],
        serde_json::json!("马落空亡，终身奔走")
    );

    // Full round-trip.
    let back: Claim = serde_json::from_str(&first).unwrap();
    assert_eq!(&back, claim);
}

// ---- evidence serialization ------------------------------------------------

#[test]
fn evidence_kinds_round_trip_through_json() {
    let items = vec![
        Evidence::new(EvidenceKind::StarAffectedByVoid {
            star: StarName::TianMa,
            void_kind: VoidKind::KongWang,
            branch: EarthlyBranch::Wu,
        }),
        Evidence::new(EvidenceKind::UnsupportedCondition {
            reason: UnsupportedReason::LuMaRelationNotModeled,
        }),
    ];
    let json = serde_json::to_string(&items).unwrap();
    let back: Vec<Evidence> = serde_json::from_str(&json).unwrap();
    assert_eq!(back, items);
}
