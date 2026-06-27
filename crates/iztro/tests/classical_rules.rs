//! Integration tests for the classical rule engine.
//!
//! These build small synthetic charts with full control over star placement so
//! each pilot rule's condition is exercised deterministically, mirroring the
//! approach in `tests/patterns.rs`.

use iztro::rules::classical::{
    Claim, ClaimDomain, ClaimEvaluationRequest, ClaimId, ClaimPolarity, ClaimScope, ClaimSpec,
    ClaimTheme, ClassicalRule, ClassicalRuleId, ClassicalSourceHit, ClassicalWork, DiagnosticMode,
    Evidence, EvidenceKind, RuleStatus, UnsupportedReason, VoidKind, VoidPolicy, classical_rules,
    evaluate_classical, evaluate_classical_claims, pattern_rules, quan_shu_rules, rule_by_id,
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

fn major(branch: EarthlyBranch, star: StarName) -> Spec {
    (branch, star, StarKind::Major, None)
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

fn source_hit_ids(source_hits: &[ClassicalSourceHit]) -> Vec<String> {
    source_hits
        .iter()
        .map(|hit| hit.rule_id.to_string())
        .collect()
}

const TIAN_MA_VOID: &str = "migration.tian_ma_void.restless_movement";
const YANG_TUO: &str = "life.yang_tuo_clamp_life.constraint_damage";
const CHANG_QU: &str = "life.chang_qu_clamp_life.literary_reputation";
const LU_MA: &str = "fortune.lu_ma_jiao_chi.favorable_convergence";
const RI_YUE: &str = "life.ri_yue_fan_bei.hardship_pressure";
const TAN_LANG_HAI_ZI: &str = "relationship.tan_ju_hai_zi.water_romance";
const XING_YU_TAN_LANG: &str = "relationship.xing_yu_tan_lang.romance_with_penalty";
const SHAN_FU_JU_KONG: &str = "fortune.shan_fu_ju_kong.monastic_life";

// ---- corpus deserialization ----------------------------------------------

#[test]
fn corpus_deserializes_all_pilot_rules() {
    // The four claim-bearing pilot rules still load through the combined corpus.
    // 禄马最喜交驰 (LU_MA) is a source-backed normalized rule that is
    // unsupported and carries no claim. The QuanShu corpus now also carries the
    // 太微赋 normalization map (many normalized/ambiguous source-hit-only rules
    // without claim metadata), so we assert structural invariants rather than a
    // fixed total.
    for id in [TIAN_MA_VOID, YANG_TUO, CHANG_QU, RI_YUE] {
        let rule = rule_by_id(id).unwrap_or_else(|| panic!("missing rule {id}"));
        assert!(rule.claim.is_some(), "rule {id} should have claim metadata");
    }
    // The two Tan Lang QuanShu rules are now executable and claim-bearing.
    for id in [TAN_LANG_HAI_ZI, XING_YU_TAN_LANG] {
        let rule = rule_by_id(id).unwrap_or_else(|| panic!("missing rule {id}"));
        assert!(rule.claim.is_some(), "rule {id} should have claim metadata");
        assert_eq!(rule.status, RuleStatus::Executable);
    }
    let shan_fu =
        rule_by_id(SHAN_FU_JU_KONG).unwrap_or_else(|| panic!("missing rule {SHAN_FU_JU_KONG}"));
    assert!(
        shan_fu.claim.is_none(),
        "rule {SHAN_FU_JU_KONG} should be source-hit-only"
    );
    assert_eq!(shan_fu.status, RuleStatus::Executable);
    let lu_ma = rule_by_id(LU_MA).unwrap_or_else(|| panic!("missing rule {LU_MA}"));
    assert!(
        lu_ma.claim.is_none(),
        "禄马最喜交驰 must not carry claim metadata"
    );

    // The combined corpus is exactly the QuanShu rules followed by the pattern
    // rules.
    assert_eq!(
        classical_rules().len(),
        quan_shu_rules().len() + pattern_rules().len()
    );

    // The three pilot QuanShu rules live in the QuanShu corpus; 羊陀夹命 /
    // 昌曲夹命 are pattern-derived and must not.
    let quan_shu_ids: Vec<&str> = quan_shu_rules().iter().map(|r| r.id.as_str()).collect();
    assert!(quan_shu_ids.contains(&TIAN_MA_VOID));
    assert!(quan_shu_ids.contains(&LU_MA));
    assert!(quan_shu_ids.contains(&RI_YUE));
    assert!(!quan_shu_ids.contains(&YANG_TUO));
    assert!(!quan_shu_ids.contains(&CHANG_QU));

    let pattern_ids: Vec<&str> = pattern_rules().iter().map(|r| r.id.as_str()).collect();
    assert_eq!(pattern_ids, vec![YANG_TUO, CHANG_QU]);
    for rule in pattern_rules() {
        assert_eq!(rule.work, ClassicalWork::IztroPatternCatalog);
        assert!(rule.source_id.starts_with("pattern."));
    }
}

/// The 太微赋 normalization map adds many non-executable, claimless rules to the
/// QuanShu corpus. They must not change runtime behaviour: each loads cleanly,
/// is `normalized`/`ambiguous`/`rejected`, carries no `[rule.claim]`, and so
/// emits neither a claim nor a source hit (the evaluator returns
/// `NotApplicable`).
#[test]
fn tai_wei_fu_normalized_rules_are_inert_at_runtime() {
    let normalized_only: Vec<&iztro::rules::classical::ClassicalRule> = quan_shu_rules()
        .iter()
        .filter(|r| {
            !matches!(
                r.id.as_str(),
                TIAN_MA_VOID
                    | LU_MA
                    | RI_YUE
                    | TAN_LANG_HAI_ZI
                    | XING_YU_TAN_LANG
                    | SHAN_FU_JU_KONG
            )
        })
        .collect();
    assert!(
        !normalized_only.is_empty(),
        "expected the 太微赋 normalization map to add rules"
    );
    for rule in &normalized_only {
        assert!(
            matches!(
                rule.status,
                RuleStatus::Normalized | RuleStatus::Ambiguous | RuleStatus::Rejected
            ),
            "normalization-map rule {} should not be executable yet",
            rule.id
        );
        assert!(
            rule.claim.is_none(),
            "normalization-map rule {} should not invent claim metadata",
            rule.id
        );
    }

    // None of them fire on a chart that only triggers the wired pilots.
    let chart = multi_claim_chart();
    let evaluation = evaluate_classical(&chart, &ClaimEvaluationRequest::default());
    let inert_ids: std::collections::HashSet<&str> =
        normalized_only.iter().map(|r| r.id.as_str()).collect();
    assert!(
        evaluation
            .claims
            .iter()
            .all(|c| !inert_ids.contains(c.rule_id.as_str()))
    );
    assert!(
        evaluation
            .source_hits
            .iter()
            .all(|h| !inert_ids.contains(h.rule_id.as_str()))
    );
}

#[test]
fn corpus_fields_match_metadata() {
    let migration = rule_by_id(TIAN_MA_VOID).expect("rule present");
    assert_eq!(migration.work, ClassicalWork::ZiWeiDouShuQuanShu);
    assert_eq!(migration.source_text_zh_hans, "马遇空亡，终身奔走");
    assert_eq!(migration.status, RuleStatus::Executable);
    let claim = migration.claim.as_ref().expect("claim metadata");
    assert_eq!(claim.domain, ClaimDomain::Migration);
    assert_eq!(claim.polarity, ClaimPolarity::MixedNegative);
    assert_eq!(
        claim.themes,
        vec![ClaimTheme::RestlessMovement, ClaimTheme::Instability]
    );
    assert!((claim.base_strength - 0.60).abs() < 1e-6);

    // 禄马最喜交驰 is source-backed, normalized, and not executable; it carries
    // no claim and uses the actual QuanShu source unit wording.
    let lu_ma = rule_by_id(LU_MA).expect("rule present");
    assert_eq!(lu_ma.status, RuleStatus::Normalized);
    assert!(lu_ma.claim.is_none());
    assert_eq!(lu_ma.source_text_zh_hans, "禄马最喜交驰");
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
    assert_eq!(
        value["claim"]["claim_key"],
        serde_json::json!("claim.migration.tian-ma-void.restless-movement")
    );
    assert!(value.get("domain").is_none());
    let back: ClassicalRule = serde_json::from_value(value).unwrap();
    assert_eq!(&back, rule);
}

#[test]
fn rule_without_claim_metadata_round_trips() {
    let rule = ClassicalRule {
        id: ClassicalRuleId::new("experimental.source_only"),
        source_id: "pattern.source_only".to_string(),
        source_clause_id: None,
        work: ClassicalWork::IztroPatternCatalog,
        source_text_zh_hans: "仅记录出处命中".to_string(),
        normalized_note_zh_hans: None,
        status: RuleStatus::Executable,
        school: Default::default(),
        claim: None,
    };

    let value = serde_json::to_value(&rule).unwrap();
    assert!(value.get("claim").is_none());

    let back: ClassicalRule = serde_json::from_value(value).unwrap();
    assert_eq!(back, rule);
}

#[test]
fn claim_spec_round_trips_inside_rule_json() {
    let rule = rule_by_id(CHANG_QU).expect("rule present");
    let value = serde_json::to_value(rule).unwrap();
    let spec: ClaimSpec = serde_json::from_value(value["claim"].clone()).unwrap();
    assert_eq!(
        spec.claim_key,
        "claim.life.chang-qu-clamp-life.literary-reputation"
    );

    let back: ClassicalRule = serde_json::from_value(value).unwrap();
    assert_eq!(&back, rule);
}

#[test]
fn claim_id_supports_discriminator_for_multi_hit_rules() {
    let rule_id = ClassicalRuleId::new(YANG_TUO);
    assert_eq!(
        ClaimId::new(&rule_id, ClaimScope::Natal).as_str(),
        format!("{YANG_TUO}@natal")
    );
    assert_eq!(
        ClaimId::with_discriminator(&rule_id, ClaimScope::Natal, "anchor=zi").as_str(),
        format!("{YANG_TUO}@natal#anchor=zi")
    );
}

// ---- 马遇空亡 (executable; conservative void policy) -----------------------

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
    let evaluation = evaluate_classical(&chart, &ClaimEvaluationRequest::default());
    let claim = evaluation
        .claims
        .iter()
        .find(|c| c.rule_id.as_str() == TIAN_MA_VOID)
        .expect("expected 马遇空亡 claim");
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
        "马遇空亡，终身奔走"
    );

    let source_hit = evaluation
        .source_hits
        .iter()
        .find(|hit| hit.rule_id.as_str() == TIAN_MA_VOID)
        .expect("expected 马遇空亡 source hit");
    assert_eq!(source_hit.work, ClassicalWork::ZiWeiDouShuQuanShu);
    assert_eq!(
        source_hit.source_id,
        "quan_shu.v01.tai_wei_fu.ma_yu_kong_wang"
    );
    // QuanShu rules now point directly at the atomic source item; they no longer
    // carry source_clause_id.
    assert_eq!(source_hit.source_clause_id.as_deref(), None);
    assert_eq!(source_hit.source_text_zh_hans, "马遇空亡，终身奔走");
    assert_eq!(source_hit.status, RuleStatus::Executable);
    assert_eq!(source_hit.scope, ClaimScope::Natal);
    assert_eq!(source_hit.evidence, claim.evidence);
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

// ---- void-related QuanShu source-hit-only executables ----------------------

fn assert_source_hit_only_rule(
    chart: &Chart,
    rule_id: &str,
    source_text_zh_hans: &str,
    evidence: impl Fn(&[Evidence]) -> bool,
) {
    let evaluation = evaluate_classical(chart, &ClaimEvaluationRequest::default());
    assert!(
        !evaluation
            .claims
            .iter()
            .any(|claim| claim.rule_id.as_str() == rule_id),
        "{rule_id} should not emit a claim"
    );
    let source_hit = evaluation
        .source_hits
        .iter()
        .find(|hit| hit.rule_id.as_str() == rule_id)
        .unwrap_or_else(|| panic!("expected source hit for {rule_id}"));
    assert_eq!(source_hit.work, ClassicalWork::ZiWeiDouShuQuanShu);
    assert_eq!(source_hit.source_text_zh_hans, source_text_zh_hans);
    assert_eq!(source_hit.status, RuleStatus::Executable);
    assert!(
        evidence(&source_hit.evidence),
        "missing expected evidence for {rule_id}"
    );
}

#[test]
fn shan_fu_ju_kong_positive_when_tian_ji_and_tian_tong_meet_void() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Yin, StarName::TianJi),
            adj(EarthlyBranch::Yin, StarName::XunKong),
            major(EarthlyBranch::Mao, StarName::TianTong),
            adj(EarthlyBranch::Mao, StarName::KongWang),
        ],
    );

    assert_source_hit_only_rule(
        &chart,
        SHAN_FU_JU_KONG,
        "善福居空位，天竺生涯",
        |evidence| {
            evidence.iter().any(|e| {
                matches!(
                    e.kind(),
                    EvidenceKind::StarAffectedByVoid {
                        star: StarName::TianJi,
                        void_kind: VoidKind::XunKong,
                        branch: EarthlyBranch::Yin,
                    }
                )
            }) && evidence.iter().any(|e| {
                matches!(
                    e.kind(),
                    EvidenceKind::StarAffectedByVoid {
                        star: StarName::TianTong,
                        void_kind: VoidKind::KongWang,
                        branch: EarthlyBranch::Mao,
                    }
                )
            })
        },
    );
}

#[test]
fn shan_fu_ju_kong_negative_when_only_one_star_meets_void() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Yin, StarName::TianJi),
            adj(EarthlyBranch::Yin, StarName::XunKong),
            major(EarthlyBranch::Mao, StarName::TianTong),
        ],
    );
    let evaluation = evaluate_classical(&chart, &ClaimEvaluationRequest::default());
    assert!(
        evaluation
            .source_hits
            .iter()
            .all(|hit| hit.rule_id.as_str() != SHAN_FU_JU_KONG)
    );
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
fn chang_qu_clamp_life_positive_emits_claim_with_pattern_shape_evidence() {
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
        EvidenceKind::PatternShapeMatched {
            pattern: iztro::PatternId::ChangQuJiaMing
        }
    )));
}

#[test]
fn pattern_catalog_rule_emits_pattern_source_hit() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Hai, StarName::WenChang),
            soft(EarthlyBranch::Chou, StarName::WenQu),
        ],
    );

    let evaluation = evaluate_classical(&chart, &ClaimEvaluationRequest::default());
    let source_hit = evaluation
        .source_hits
        .iter()
        .find(|hit| hit.rule_id.as_str() == CHANG_QU)
        .expect("expected 昌曲夹命 source hit");

    assert_eq!(source_hit.work, ClassicalWork::IztroPatternCatalog);
    assert_eq!(source_hit.source_id, "pattern.chang_qu_jia_ming");
    assert_eq!(
        source_hit.source_clause_id.as_deref(),
        Some("chang_qu_jia_ming")
    );
    assert_eq!(source_hit.source_text_zh_hans, "昌曲夹命，主贵显");
    assert_eq!(source_hit.scope, ClaimScope::Natal);
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

// ---- 贪居亥子 (executable; 贪狼居亥或子) ----------------------------------

fn assert_tan_ju_hai_zi(chart: &Chart, branch: EarthlyBranch) {
    let evaluation = evaluate_classical(chart, &ClaimEvaluationRequest::default());
    let claim = evaluation
        .claims
        .iter()
        .find(|c| c.rule_id.as_str() == TAN_LANG_HAI_ZI)
        .expect("expected 贪居亥子 claim");
    assert_eq!(claim.domain, ClaimDomain::Relationship);
    assert_eq!(claim.scope, ClaimScope::Natal);
    assert!(claim.evidence.iter().any(|e| matches!(
        e.kind(),
        EvidenceKind::StarInPalace {
            star: StarName::TanLang,
            branch: b,
        } if *b == branch
    )));

    let source_hit = evaluation
        .source_hits
        .iter()
        .find(|hit| hit.rule_id.as_str() == TAN_LANG_HAI_ZI)
        .expect("expected 贪居亥子 source hit");
    assert_eq!(source_hit.source_text_zh_hans, "贪居亥子，名为犯水桃花");
    assert_eq!(source_hit.status, RuleStatus::Executable);
}

#[test]
fn tan_ju_hai_zi_positive_in_hai() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[tough(EarthlyBranch::Hai, StarName::TanLang)],
    );
    assert_tan_ju_hai_zi(&chart, EarthlyBranch::Hai);
}

#[test]
fn tan_ju_hai_zi_positive_in_zi() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[tough(EarthlyBranch::Zi, StarName::TanLang)],
    );
    assert_tan_ju_hai_zi(&chart, EarthlyBranch::Zi);
}

#[test]
fn tan_ju_hai_zi_negative_in_other_branch() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[tough(EarthlyBranch::Yin, StarName::TanLang)],
    );
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    assert!(!has_rule(&claims, TAN_LANG_HAI_ZI));
}

// ---- 刑遇贪狼 (executable; 贪狼与刑曜同宫) --------------------------------

fn assert_xing_yu_tan_lang(chart: &Chart, penalty_star: StarName) {
    let evaluation = evaluate_classical(chart, &ClaimEvaluationRequest::default());
    let claim = evaluation
        .claims
        .iter()
        .find(|c| c.rule_id.as_str() == XING_YU_TAN_LANG)
        .expect("expected 刑遇贪狼 claim");
    assert_eq!(claim.domain, ClaimDomain::Relationship);
    assert_eq!(claim.scope, ClaimScope::Natal);
    assert!(claim.evidence.iter().any(|e| matches!(
        e.kind(),
        EvidenceKind::StarInPalace {
            star: StarName::TanLang,
            ..
        }
    )));
    assert!(claim.evidence.iter().any(|e| matches!(
        e.kind(),
        EvidenceKind::StarInPalace { star, .. } if *star == penalty_star
    )));

    let source_hit = evaluation
        .source_hits
        .iter()
        .find(|hit| hit.rule_id.as_str() == XING_YU_TAN_LANG)
        .expect("expected 刑遇贪狼 source hit");
    assert_eq!(source_hit.source_text_zh_hans, "刑遇贪狼，号曰风流彩杖");
    assert_eq!(source_hit.status, RuleStatus::Executable);
}

#[test]
fn xing_yu_tan_lang_positive_with_qing_yang() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            tough(EarthlyBranch::Wu, StarName::TanLang),
            tough(EarthlyBranch::Wu, StarName::QingYang),
        ],
    );
    assert_xing_yu_tan_lang(&chart, StarName::QingYang);
}

#[test]
fn xing_yu_tan_lang_positive_with_tian_xing() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            tough(EarthlyBranch::Wu, StarName::TanLang),
            adj(EarthlyBranch::Wu, StarName::TianXing),
        ],
    );
    assert_xing_yu_tan_lang(&chart, StarName::TianXing);
}

#[test]
fn xing_yu_tan_lang_negative_when_penalty_in_other_palace() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            tough(EarthlyBranch::Wu, StarName::TanLang),
            adj(EarthlyBranch::Zi, StarName::TianXing),
        ],
    );
    let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
    assert!(!has_rule(&claims, XING_YU_TAN_LANG));
}

#[test]
fn xing_yu_tan_lang_negative_with_void_symbol_star() {
    // 空劫/空曜 stars (地空、天空) carry KongJie/VoidSymbol tags, not Punishment,
    // so 刑遇贪狼 must not fire on them.
    for void_star in [StarName::DiKong, StarName::TianKong] {
        let chart = build_chart(
            EarthlyBranch::Zi,
            &[
                tough(EarthlyBranch::Wu, StarName::TanLang),
                adj(EarthlyBranch::Wu, void_star),
            ],
        );
        let claims = evaluate_classical_claims(&chart, &ClaimEvaluationRequest::default());
        assert!(
            !has_rule(&claims, XING_YU_TAN_LANG),
            "{void_star:?} should not trigger 刑遇贪狼"
        );
    }
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
    assert!(
        evaluation
            .source_hits
            .iter()
            .all(|hit| hit.rule_id.as_str() != LU_MA),
        "unsupported rules should not emit source hits"
    );
    let diagnostic = evaluation
        .diagnostics
        .iter()
        .find(|d| d.rule_id.as_str() == LU_MA)
        .expect("expected a typed diagnostic for 禄马交驰");
    assert_eq!(diagnostic.reason, UnsupportedReason::LuMaRelationNotModeled);
}

#[test]
fn diagnostic_mode_none_suppresses_unsupported_diagnostics() {
    let chart = build_chart(EarthlyBranch::Zi, &[tian_ma(EarthlyBranch::Wu)]);
    let request = ClaimEvaluationRequest {
        diagnostic_mode: DiagnosticMode::None,
        ..Default::default()
    };
    let evaluation = evaluate_classical(&chart, &request);
    assert!(evaluation.diagnostics.is_empty());
}

#[test]
fn diagnostic_mode_matching_request_filters_unsupported_by_rule_id() {
    let chart = build_chart(EarthlyBranch::Zi, &[tian_ma(EarthlyBranch::Wu)]);

    let yang_tuo_request = ClaimEvaluationRequest {
        diagnostic_mode: DiagnosticMode::MatchingRequest,
        rule_ids: vec![ClassicalRuleId::new(YANG_TUO)],
        ..Default::default()
    };
    assert!(
        evaluate_classical(&chart, &yang_tuo_request)
            .diagnostics
            .is_empty()
    );

    let lu_ma_request = ClaimEvaluationRequest {
        diagnostic_mode: DiagnosticMode::MatchingRequest,
        rule_ids: vec![ClassicalRuleId::new(LU_MA)],
        ..Default::default()
    };
    let evaluation = evaluate_classical(&chart, &lu_ma_request);
    assert!(
        evaluation
            .diagnostics
            .iter()
            .any(|d| d.rule_id.as_str() == LU_MA)
    );
}

// ---- deterministic sorting -------------------------------------------------

/// A chart fulfilling 羊陀夹命 + 昌曲夹命 (both Life) and 马遇空亡 (Migration).
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

#[test]
fn source_hits_are_sorted_by_scope_work_source_clause_rule() {
    let chart = multi_claim_chart();
    let evaluation = evaluate_classical(&chart, &ClaimEvaluationRequest::default());

    assert_eq!(
        source_hit_ids(&evaluation.source_hits),
        vec![TIAN_MA_VOID, CHANG_QU, YANG_TUO]
    );
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
fn filter_by_work_separates_quan_shu_and_pattern_claims() {
    let chart = multi_claim_chart();

    // multi_claim_chart emits CHANG_QU, YANG_TUO (pattern) and TIAN_MA_VOID
    // (QuanShu). Filtering by work splits them by provenance.
    let quan_shu = ClaimEvaluationRequest {
        works: vec![ClassicalWork::ZiWeiDouShuQuanShu],
        ..Default::default()
    };
    assert_eq!(
        claim_ids(&evaluate_classical_claims(&chart, &quan_shu)),
        vec![TIAN_MA_VOID]
    );

    let pattern = ClaimEvaluationRequest {
        works: vec![ClassicalWork::IztroPatternCatalog],
        ..Default::default()
    };
    assert_eq!(
        claim_ids(&evaluate_classical_claims(&chart, &pattern)),
        vec![CHANG_QU, YANG_TUO]
    );
}

#[test]
fn work_filter_separates_quan_shu_and_pattern_source_hits() {
    let chart = multi_claim_chart();

    let quan_shu = ClaimEvaluationRequest {
        works: vec![ClassicalWork::ZiWeiDouShuQuanShu],
        ..Default::default()
    };
    assert_eq!(
        source_hit_ids(&evaluate_classical(&chart, &quan_shu).source_hits),
        vec![TIAN_MA_VOID]
    );

    let pattern = ClaimEvaluationRequest {
        works: vec![ClassicalWork::IztroPatternCatalog],
        ..Default::default()
    };
    assert_eq!(
        source_hit_ids(&evaluate_classical(&chart, &pattern).source_hits),
        vec![CHANG_QU, YANG_TUO]
    );
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

#[test]
fn evaluate_classical_claims_remains_claims_only() {
    let chart = multi_claim_chart();
    let request = ClaimEvaluationRequest::default();
    let evaluation = evaluate_classical(&chart, &request);

    assert_eq!(
        evaluate_classical_claims(&chart, &request),
        evaluation.claims
    );
    assert_eq!(evaluation.source_hits.len(), evaluation.claims.len());
}

// ---- void policy -----------------------------------------------------------

#[test]
fn default_void_policy_includes_all_modeled_void_kinds() {
    let kinds = VoidPolicy::DEFAULT.kinds();
    assert_eq!(
        kinds,
        &[
            VoidKind::XunKong,
            VoidKind::KongWang,
            VoidKind::JieLu,
            VoidKind::JieKong
        ]
    );
    for kind in kinds {
        assert!(VoidPolicy::DEFAULT.includes(*kind));
    }
}

#[test]
fn xun_kong_only_void_policy_includes_only_xun_kong() {
    assert_eq!(VoidPolicy::XUN_KONG_ONLY.kinds(), &[VoidKind::XunKong]);
    assert!(VoidPolicy::XUN_KONG_ONLY.includes(VoidKind::XunKong));
    assert!(!VoidPolicy::XUN_KONG_ONLY.includes(VoidKind::KongWang));
    assert!(!VoidPolicy::XUN_KONG_ONLY.includes(VoidKind::JieLu));
    assert!(!VoidPolicy::XUN_KONG_ONLY.includes(VoidKind::JieKong));
}

#[test]
fn non_void_empty_stars_do_not_map_to_void_kind() {
    assert_eq!(VoidKind::from_star(StarName::TianKong), None);
    assert_eq!(VoidKind::from_star(StarName::DiKong), None);
    assert_eq!(VoidKind::from_star(StarName::DiJie), None);
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
        serde_json::json!("马遇空亡，终身奔走")
    );

    // Full round-trip.
    let back: Claim = serde_json::from_str(&first).unwrap();
    assert_eq!(&back, claim);
}

#[test]
fn claim_evaluation_json_includes_deterministic_source_hits() {
    let chart = multi_claim_chart();
    let evaluation = evaluate_classical(&chart, &ClaimEvaluationRequest::default());

    let first = serde_json::to_string(&evaluation).unwrap();
    let second = serde_json::to_string(&evaluation).unwrap();
    assert_eq!(first, second);

    let value: serde_json::Value = serde_json::from_str(&first).unwrap();
    let source_hits = value["source_hits"].as_array().expect("source_hits array");
    assert_eq!(source_hits.len(), 3);
    assert_eq!(source_hits[0]["rule_id"], serde_json::json!(TIAN_MA_VOID));
    assert_eq!(
        source_hits[0]["source_id"],
        serde_json::json!("quan_shu.v01.tai_wei_fu.ma_yu_kong_wang")
    );
    // QuanShu rules no longer carry source_clause_id; it serializes as null.
    assert_eq!(source_hits[0]["source_clause_id"], serde_json::Value::Null);
    assert!(source_hits[0].get("claim_key").is_none());
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
