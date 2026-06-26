//! Integration tests for the renderer-neutral classical rule panel view.
//!
//! These use the public API only and mirror the synthetic chart-building style of
//! `tests/classical_rules.rs` so each panel facet (claims, source hits,
//! diagnostics, corpus metadata) is exercised deterministically.

use iztro::rules::classical::{
    ClaimEvaluationRequest, ClassicalRulePanelRequest, EvidenceKind, RuleStatus,
    classical_rule_panel_view,
};
use iztro::{
    BirthContext, Brightness, CalendarDate, Chart, EarthlyBranch, Gender, HeavenlyStem, Mutagen,
    PALACE_NAMES, Palace, Scope, StarKind, StarName, StarPlacement, StemBranch,
};

// ---- synthetic chart builders (mirrors tests/classical_rules.rs) -----------

type Spec = (EarthlyBranch, StarName, StarKind, Option<Mutagen>);

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
    Chart::try_new(
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
        StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu).expect("valid stem-branch"),
        iztro::MethodProfile::placeholder("classical_panel_test"),
        palaces,
        None,
        None,
    )
    .expect("synthetic chart should build")
}

fn tough(branch: EarthlyBranch, star: StarName) -> Spec {
    (branch, star, StarKind::Tough, None)
}

fn tian_ma(branch: EarthlyBranch) -> Spec {
    (branch, StarName::TianMa, StarKind::TianMa, None)
}

const TIAN_MA_VOID: &str = "migration.tian_ma_void.restless_movement";
const LU_MA: &str = "fortune.lu_ma_jiao_chi.favorable_convergence";
const TAN_LANG_HAI_ZI: &str = "relationship.tan_ju_hai_zi.water_romance";
const XING_YU_TAN_LANG: &str = "relationship.xing_yu_tan_lang.romance_with_penalty";

fn relationship_chart_hai() -> Chart {
    build_chart(
        EarthlyBranch::Zi,
        &[tough(EarthlyBranch::Hai, StarName::TanLang)],
    )
}

fn xing_yu_tan_lang_chart() -> Chart {
    build_chart(
        EarthlyBranch::Zi,
        &[
            tough(EarthlyBranch::Wu, StarName::TanLang),
            tough(EarthlyBranch::Wu, StarName::QingYang),
        ],
    )
}

// ---- corpus view tests -----------------------------------------------------

#[test]
fn default_panel_includes_corpus_rules() {
    let chart = relationship_chart_hai();
    let panel = classical_rule_panel_view(&chart, &ClassicalRulePanelRequest::default());

    assert!(!panel.corpus_rules.is_empty());
    assert_eq!(panel.summary.corpus_rule_count, panel.corpus_rules.len());

    for id in [TAN_LANG_HAI_ZI, XING_YU_TAN_LANG, TIAN_MA_VOID] {
        assert!(
            panel.corpus_rules.iter().any(|r| r.rule_id.as_str() == id),
            "corpus should contain {id}"
        );
    }

    // Every corpus entry carries canonical source text and a status.
    for rule in &panel.corpus_rules {
        assert!(
            !rule.source_text_zh_hans.is_empty(),
            "corpus rule {} must carry source text",
            rule.rule_id
        );
        // status is a closed enum; assert it deserialized into a known variant.
        assert!(matches!(
            rule.status,
            RuleStatus::Raw
                | RuleStatus::Segmented
                | RuleStatus::Normalized
                | RuleStatus::Executable
                | RuleStatus::Tested
                | RuleStatus::Ambiguous
                | RuleStatus::Rejected
        ));
    }
}

#[test]
fn corpus_status_filter_keeps_only_requested_statuses() {
    let chart = relationship_chart_hai();
    let request =
        ClassicalRulePanelRequest::default().with_corpus_statuses([RuleStatus::Executable]);
    let panel = classical_rule_panel_view(&chart, &request);

    assert!(!panel.corpus_rules.is_empty());
    assert!(
        panel
            .corpus_rules
            .iter()
            .all(|r| r.status == RuleStatus::Executable)
    );
}

#[test]
fn without_corpus_omits_corpus_rules() {
    let chart = relationship_chart_hai();
    let request = ClassicalRulePanelRequest::default().without_corpus();
    let panel = classical_rule_panel_view(&chart, &request);

    assert!(panel.corpus_rules.is_empty());
    assert_eq!(panel.summary.corpus_rule_count, 0);
}

// ---- evaluation view tests -------------------------------------------------

#[test]
fn tan_ju_hai_zi_produces_matching_claim_and_source_hit() {
    let chart = relationship_chart_hai();
    let panel = classical_rule_panel_view(&chart, &ClassicalRulePanelRequest::default());

    let source_hit = panel
        .source_hits
        .iter()
        .find(|hit| hit.rule_id.as_str() == TAN_LANG_HAI_ZI)
        .expect("expected 贪居亥子 source hit");
    assert_eq!(source_hit.source_text_zh_hans, "贪居亥子，名为犯水桃花");

    let claim = panel
        .claims
        .iter()
        .find(|c| c.rule_id.as_str() == TAN_LANG_HAI_ZI)
        .expect("expected 贪居亥子 claim");
    assert_eq!(
        claim.source_refs[0].source_text_zh_hans,
        "贪居亥子，名为犯水桃花"
    );

    assert_eq!(panel.summary.claim_count, panel.claims.len());
    assert_eq!(panel.summary.source_hit_count, panel.source_hits.len());
}

#[test]
fn xing_yu_tan_lang_evidence_carries_both_stars() {
    let chart = xing_yu_tan_lang_chart();
    let panel = classical_rule_panel_view(&chart, &ClassicalRulePanelRequest::default());

    let source_hit = panel
        .source_hits
        .iter()
        .find(|hit| hit.rule_id.as_str() == XING_YU_TAN_LANG)
        .expect("expected 刑遇贪狼 source hit");
    assert_eq!(source_hit.source_text_zh_hans, "刑遇贪狼，号曰风流彩杖");

    let claim = panel
        .claims
        .iter()
        .find(|c| c.rule_id.as_str() == XING_YU_TAN_LANG)
        .expect("expected 刑遇贪狼 claim");
    assert!(claim.evidence.iter().any(|e| matches!(
        e.kind(),
        EvidenceKind::StarInPalace {
            star: StarName::TanLang,
            ..
        }
    )));
    assert!(claim.evidence.iter().any(|e| matches!(
        e.kind(),
        EvidenceKind::StarInPalace {
            star: StarName::QingYang,
            ..
        }
    )));
}

#[test]
fn user_facing_request_hides_diagnostics() {
    let chart = build_chart(EarthlyBranch::Zi, &[tian_ma(EarthlyBranch::Wu)]);
    let panel = classical_rule_panel_view(&chart, &ClassicalRulePanelRequest::user_facing());
    assert!(panel.diagnostics.is_empty());
    assert_eq!(panel.summary.diagnostic_count, 0);
}

#[test]
fn developer_request_surfaces_unsupported_lu_ma_diagnostic() {
    let chart = build_chart(EarthlyBranch::Zi, &[tian_ma(EarthlyBranch::Wu)]);
    let panel = classical_rule_panel_view(&chart, &ClassicalRulePanelRequest::developer());
    assert!(
        panel.diagnostics.iter().any(|d| d.rule_id.as_str() == LU_MA),
        "developer panel should surface the unsupported 禄马交驰 diagnostic"
    );
    assert_eq!(panel.summary.diagnostic_count, panel.diagnostics.len());
}

// ---- panel preserves underlying evaluation ---------------------------------

#[test]
fn panel_preserves_evaluation_claims_and_source_hits() {
    let chart = relationship_chart_hai();
    let request = ClassicalRulePanelRequest::user_facing();
    let panel = classical_rule_panel_view(&chart, &request);

    // The panel must not re-derive or reorder the evaluation output.
    let evaluation = iztro::rules::classical::evaluate_classical(&chart, &request.evaluation);
    assert_eq!(panel.claims, evaluation.claims);
    assert_eq!(panel.source_hits, evaluation.source_hits);
    assert_eq!(panel.diagnostics, evaluation.diagnostics);
}

// ---- serialization ---------------------------------------------------------

#[test]
fn panel_round_trips_through_json() {
    let chart = relationship_chart_hai();
    let panel = classical_rule_panel_view(&chart, &ClassicalRulePanelRequest::default());

    let json = serde_json::to_string(&panel).unwrap();
    let back: iztro::rules::classical::ClassicalRulePanelView =
        serde_json::from_str(&json).unwrap();
    assert_eq!(back, panel);
}

// ---- corpus claim-filter behaviour -----------------------------------------

#[test]
fn corpus_domain_filter_drops_claimless_rules() {
    use iztro::rules::classical::ClaimDomain;

    let chart = relationship_chart_hai();
    let request = ClassicalRulePanelRequest {
        evaluation: ClaimEvaluationRequest {
            domains: vec![ClaimDomain::Relationship],
            ..Default::default()
        },
        include_corpus: true,
        corpus_statuses: Vec::new(),
    };
    let panel = classical_rule_panel_view(&chart, &request);

    assert!(!panel.corpus_rules.is_empty());
    // A non-empty domain filter keeps only rules whose claim is in that domain;
    // claimless corpus rules are excluded.
    for rule in &panel.corpus_rules {
        assert_eq!(rule.domain, Some(ClaimDomain::Relationship));
        assert!(rule.has_claim);
    }
}
