//! Guardrail tests for the classical rule corpus.
//!
//! These tests enforce invariants that must hold as the QuanShu corpus grows:
//! unique rule ids, metadata coverage, source text purity, and the restriction
//! that user-facing analysis returns only ZiWeiDouShuQuanShu rule hits.
//!
//! All tests operate on typed runtime APIs — `classical_rules()`,
//! `classical_rule_metadata()`, `AnalysisLayerRequest::user_facing()`, and
//! `detect_analysis_layer()` — rather than parsing source text or TOML.

use std::collections::HashSet;

use iztro::rules::classical::{
    ClaimEvaluationRequest, ClassicalRuleHitRef, ClassicalWork, DiagnosticMode, RuleStatus,
    classical_rule_metadata, classical_rules, evaluate_classical,
};
use iztro::{
    AnalysisLayerKey, AnalysisLayerRequest, BirthContext, Brightness, CalendarDate, Chart,
    EarthlyBranch, Gender, HeavenlyStem, MethodProfile, Mutagen, PALACE_NAMES, Palace, Scope,
    StarKind, StarName, StarPlacement, StemBranch, TemporalAnalysisContext, detect_analysis_layer,
};

// ---- synthetic chart builders -----------------------------------------------

type Spec = (EarthlyBranch, StarName, StarKind, Option<Mutagen>);

fn build_chart(life_branch: EarthlyBranch, placements: &[Spec]) -> Chart {
    let palaces: Vec<Palace> = (0..12)
        .map(|index| {
            let name = PALACE_NAMES[index];
            let branch = life_branch.offset(index as isize);
            let stars: Vec<StarPlacement> = placements
                .iter()
                .filter(|(b, ..)| *b == branch)
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
        MethodProfile::placeholder("guardrail_test"),
        palaces,
        None,
        None,
    )
    .expect("synthetic chart should build")
}

/// Chart that fires `migration.tian_ma_void.restless_movement` (TianMa + 旬空
/// sharing Hai).
fn tian_ma_void_chart() -> Chart {
    build_chart(
        EarthlyBranch::Zi,
        &[
            (EarthlyBranch::Hai, StarName::TianMa, StarKind::TianMa, None),
            (
                EarthlyBranch::Hai,
                StarName::XunKong,
                StarKind::Adjective,
                None,
            ),
        ],
    )
}

/// Chart that fires `life.chang_qu_clamp_life.literary_reputation` — a
/// `ClassicalWork::IztroPatternCatalog` rule, not a QuanShu source rule. 文昌
/// and 文曲 clamp the Life palace at Zi.
fn chang_qu_jia_ming_chart() -> Chart {
    build_chart(
        EarthlyBranch::Zi,
        &[
            (EarthlyBranch::Hai, StarName::WenChang, StarKind::Soft, None),
            (EarthlyBranch::Chou, StarName::WenQu, StarKind::Soft, None),
        ],
    )
}

// ---- 1. No duplicate runtime rule ids ---------------------------------------

/// Every executable/runtime classical rule must have a unique id. A duplicate
/// would silently make the second rule unreachable by id-based lookup.
#[test]
fn no_duplicate_runtime_rule_ids() {
    let rules = classical_rules();
    let mut seen: HashSet<&str> = HashSet::with_capacity(rules.len());
    for rule in rules {
        assert!(
            seen.insert(rule.id.as_str()),
            "duplicate classical rule id: {}",
            rule.id
        );
    }
}

// ---- 2. Every runtime rule has metadata -------------------------------------

/// `classical_rule_metadata(rule_id)` must return `Some` for every rule id
/// registered in the runtime corpus. A missing entry means the renderer has no
/// verbatim source text to display for that rule.
#[test]
fn every_runtime_rule_has_metadata() {
    for rule in classical_rules() {
        assert!(
            classical_rule_metadata(rule.id.clone()).is_some(),
            "no metadata entry for runtime rule {}; add it to the metadata table",
            rule.id
        );
    }
}

// ---- 3. Executable QuanShu rules have valid source metadata -----------------

/// For every executable (or tested) QuanShu rule, the runtime metadata must
/// carry non-empty `source_id`, non-empty `source_text_zh_hans`, and at least
/// one entry in `applicable_scopes`. Also asserts no leading/trailing whitespace
/// in the source text, which would indicate a copy-paste error.
#[test]
fn executable_quan_shu_rules_have_valid_source_metadata() {
    for rule in classical_rules() {
        if rule.work != ClassicalWork::ZiWeiDouShuQuanShu {
            continue;
        }
        if !matches!(rule.status, RuleStatus::Executable | RuleStatus::Tested) {
            continue;
        }
        let metadata = classical_rule_metadata(rule.id.clone())
            .unwrap_or_else(|| panic!("no metadata for executable QuanShu rule {}", rule.id));
        assert!(
            !metadata.source_id.is_empty(),
            "executable QuanShu rule {} has empty source_id",
            rule.id
        );
        assert!(
            !metadata.source_text_zh_hans.is_empty(),
            "executable QuanShu rule {} has empty source_text_zh_hans",
            rule.id
        );
        assert!(
            !metadata.applicable_scopes.is_empty(),
            "executable QuanShu rule {} has empty applicable_scopes",
            rule.id
        );
        assert_eq!(
            metadata.source_text_zh_hans.trim(),
            metadata.source_text_zh_hans,
            "executable QuanShu rule {} has leading/trailing whitespace in source_text_zh_hans",
            rule.id
        );
    }
}

// ---- 4. QuanShu runtime metadata has no duplicate source pairs --------------

/// For runtime QuanShu metadata, no two entries may share the same
/// `(source_id, source_clause_id)` pair. A duplicate pair indicates a copy-paste
/// error in the rule corpus that would make the second rule's provenance
/// indistinguishable from the first.
#[test]
fn quan_shu_runtime_metadata_has_no_duplicate_source_pairs() {
    type Key<'a> = (&'a str, Option<&'a str>);
    let mut seen: HashSet<Key<'_>> = HashSet::new();
    for rule in classical_rules() {
        if rule.work != ClassicalWork::ZiWeiDouShuQuanShu {
            continue;
        }
        let metadata = classical_rule_metadata(rule.id.clone())
            .unwrap_or_else(|| panic!("no metadata for QuanShu rule {}", rule.id));
        let key: Key<'_> = (metadata.source_id, metadata.source_clause_id);
        assert!(
            seen.insert(key),
            "duplicate (source_id, source_clause_id) {:?} for QuanShu rule {}",
            key,
            rule.id
        );
    }
}

// ---- 5a. user_facing() classical works = QuanShu only ----------------------

/// `AnalysisLayerRequest::user_facing()` must restrict the classical rule stream
/// to `[ClassicalWork::ZiWeiDouShuQuanShu]`. Widening this would expose
/// pattern-catalog rules in the 全书规则 tab, which is a GUI-separation violation.
#[test]
fn user_facing_analysis_request_is_quan_shu_only() {
    let request = AnalysisLayerRequest::user_facing();
    assert_eq!(
        request.classical.works,
        vec![ClassicalWork::ZiWeiDouShuQuanShu],
        "user_facing() must restrict to ZiWeiDouShuQuanShu; \
         widening this would expose IztroPatternCatalog rules in the QuanShu rule tab"
    );
}

// ---- 5b. user_facing() rule hits exclude IztroPatternCatalog rules ----------

/// With a chart that fires a `ClassicalWork::IztroPatternCatalog` rule (昌曲夹命),
/// `detect_analysis_layer` with the `user_facing()` request must not return
/// that rule's hit. Pattern-catalog rules surface through the pattern (格局)
/// stream, not the classical rule stream.
#[test]
fn user_facing_rule_hits_exclude_pattern_catalog() {
    let chart = chang_qu_jia_ming_chart();
    let ctx = TemporalAnalysisContext::natal(&chart);
    let request = AnalysisLayerRequest::user_facing();
    let result = detect_analysis_layer(&ctx, AnalysisLayerKey::Natal, &request);

    for hit_ref in &result.rule_hits {
        let metadata = classical_rule_metadata(hit_ref.rule_id.clone()).unwrap_or_else(|| {
            panic!(
                "rule hit {} has no metadata; cannot verify work filter",
                hit_ref.rule_id
            )
        });
        assert_ne!(
            metadata.work,
            ClassicalWork::IztroPatternCatalog,
            "user_facing() rule hit {} carries work IztroPatternCatalog; \
             pattern-catalog rules must be excluded from the classical rule stream",
            hit_ref.rule_id
        );
    }
}

// ---- 6. QuanShu source hits carry non-empty evidence ------------------------

/// For executable QuanShu rules whose predicate matches, the emitted source hit
/// must carry at least one structured evidence item. Evidence is what the future
/// GUI uses for palace/star highlighting; an empty list would make the hit
/// unactionable for rendering.
///
/// Allowlist: currently no QuanShu rules are permitted to emit empty evidence.
/// If a rule genuinely cannot produce structured evidence (e.g. a statistical
/// claim), document it here with a comment explaining why.
#[test]
fn quan_shu_source_hits_carry_evidence() {
    // Chart with TianMa and 旬空 sharing Hai: fires 马遇空亡.
    let chart = tian_ma_void_chart();
    let request = ClaimEvaluationRequest {
        works: vec![ClassicalWork::ZiWeiDouShuQuanShu],
        diagnostic_mode: DiagnosticMode::AllUnsupported,
        ..Default::default()
    };

    let evaluation = evaluate_classical(&chart, &request);
    // Only QuanShu rules are requested; at least one should match (马遇空亡).
    assert!(
        !evaluation.source_hits.is_empty(),
        "expected at least one QuanShu source hit from the guardrail chart; \
         update the chart if the pilot corpus changes"
    );

    for hit in &evaluation.source_hits {
        assert_eq!(
            hit.work,
            ClassicalWork::ZiWeiDouShuQuanShu,
            "work filter should have excluded non-QuanShu hit {}",
            hit.rule_id
        );
        assert!(
            !hit.evidence.is_empty(),
            "QuanShu source hit {} carries no structured evidence; \
             every executable QuanShu rule must emit at least one Evidence item",
            hit.rule_id
        );
    }
}

// ---- 7. ClassicalRuleHitRef does not carry source_text_zh_hans --------------

/// `ClassicalRuleHitRef` is the compact, renderer-facing hit reference. It
/// intentionally omits `source_text_zh_hans` so source text stays canonical in
/// the rule corpus (resolved once per rule id via `classical_rule_metadata`)
/// rather than being copied into every hit.
///
/// The absence of `source_text_zh_hans` is enforced at compile time by the
/// struct definition. This test verifies the *positive* invariant: a hit ref
/// built from a full source hit carries exactly the four expected fields
/// (rule_id, scope, claim_key, evidence) and that source text is absent from
/// the compact ref (must be fetched separately via `classical_rule_metadata`).
#[test]
fn classical_rule_hit_ref_does_not_duplicate_source_text() {
    let chart = tian_ma_void_chart();
    let request = ClaimEvaluationRequest {
        works: vec![ClassicalWork::ZiWeiDouShuQuanShu],
        diagnostic_mode: DiagnosticMode::None,
        ..Default::default()
    };

    let evaluation = evaluate_classical(&chart, &request);
    let source_hit = evaluation
        .source_hits
        .first()
        .expect("expected at least one QuanShu source hit from the guardrail chart");

    let hit_ref = ClassicalRuleHitRef::from_source_hit(source_hit);

    // The compact ref carries the four expected navigation fields.
    assert_eq!(hit_ref.rule_id, source_hit.rule_id);
    assert_eq!(hit_ref.scope, source_hit.scope);
    assert!(!hit_ref.evidence.is_empty(), "hit ref must carry evidence");

    // Source text must be fetched separately — not embedded in the compact ref.
    // `ClassicalRuleHitRef` has no `source_text_zh_hans` field (compile-time
    // guarantee). Resolve it via `classical_rule_metadata` as a renderer would.
    let metadata =
        classical_rule_metadata(hit_ref.rule_id.clone()).expect("hit ref rule must have metadata");
    assert!(
        !metadata.source_text_zh_hans.is_empty(),
        "source text is resolved from metadata, not from the compact ref"
    );
}
