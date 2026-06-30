//! Integration tests for the layer-level analysis API.
//!
//! These exercise only the public surface: selection-to-layer expansion, layer
//! scope mapping, per-layer detection, and rule metadata resolution. Synthetic
//! charts are built with full control over star placement so the natal classical
//! rule and natal patterns fire deterministically.

use iztro::rules::classical::{ClaimScope, ClassicalRuleId, classical_rule_metadata};
use iztro::{
    AnalysisLayerKey, AnalysisLayerRequest, BirthContext, Brightness, CalendarDate, Chart,
    ClaimEvaluationRequest, ClassicalRuleContext, EarthlyBranch, Gender, HeavenlyStem,
    MethodProfile, Mutagen, PALACE_NAMES, Palace, PatternScope, Scope, SolarChartRequest,
    SolarDay, SolarMonth, StarKind, StarName, StarPlacement, StaticTemporalNavigationSelection,
    StemBranch, TemporalAnalysisContext, analysis_layers_for_selection,
    analysis_scopes_for_layer_key, by_solar, detect_analysis_layer,
    detect_static_temporal_analysis_layers_from_chart, evaluate_classical,
    evaluate_classical_in_context,
};

// ---- synthetic chart builders --------------------------------------------

/// One synthetic star placement: (branch, star, kind, optional mutagen).
type Spec = (EarthlyBranch, StarName, StarKind, Option<Mutagen>);

/// Builds a 12-palace natal chart with the Life palace at `life_branch`.
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
        MethodProfile::placeholder("analysis_layer_test"),
        palaces,
        None,
        None,
    )
    .expect("synthetic chart should build")
}

fn major(branch: EarthlyBranch, star: StarName) -> Spec {
    (branch, star, StarKind::Major, None)
}

fn soft(branch: EarthlyBranch, star: StarName) -> Spec {
    (branch, star, StarKind::Soft, None)
}

/// The id of the 马遇空亡 QuanShu rule, the natal classical hit under test.
const TIAN_MA_VOID: &str = "migration.tian_ma_void.restless_movement";

/// The id of the 昌曲夹命 rule, a project pattern-catalog
/// (`ClassicalWork::IztroPatternCatalog`) classical rule.
const CHANG_QU: &str = "life.chang_qu_clamp_life.literary_reputation";

/// A chart where 文昌/文曲 clamp the Life palace at Zi, firing the project
/// pattern-catalog 昌曲夹命 classical rule (not a QuanShu source rule).
fn chang_qu_jia_ming_chart() -> Chart {
    build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Hai, StarName::WenChang),
            soft(EarthlyBranch::Chou, StarName::WenQu),
        ],
    )
}

/// A chart that fires both the natal 马遇空亡 classical rule (TianMa sharing a
/// palace with 旬空) and three natal patterns (紫府朝垣 / 机月同梁 / 羊陀夹忌).
///
/// The 马遇空亡 stars sit in Hai, which is not part of any pattern's structure, so
/// the classical hit and the patterns do not interfere.
fn rich_natal_chart() -> Chart {
    build_chart(
        EarthlyBranch::Zi,
        &[
            // 紫府朝垣 + 机月同梁.
            major(EarthlyBranch::Zi, StarName::ZiWei),
            major(EarthlyBranch::Zi, StarName::TianJi),
            major(EarthlyBranch::Wu, StarName::TianFu),
            major(EarthlyBranch::Wu, StarName::TaiYin),
            major(EarthlyBranch::Chen, StarName::TianTong),
            major(EarthlyBranch::Shen, StarName::TianLiang),
            // 羊陀夹忌.
            (
                EarthlyBranch::Mao,
                StarName::TaiYang,
                StarKind::Major,
                Some(Mutagen::Ji),
            ),
            (
                EarthlyBranch::Yin,
                StarName::QingYang,
                StarKind::Tough,
                None,
            ),
            (EarthlyBranch::Chen, StarName::TuoLuo, StarKind::Tough, None),
            // 马遇空亡 (TianMa + 旬空 sharing Hai).
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

fn sample_solar_chart() -> Chart {
    let request = SolarChartRequest::builder()
        .solar_year(1990)
        .solar_month(SolarMonth::new(5).expect("valid month"))
        .solar_day(SolarDay::new(17).expect("valid day"))
        .birth_time(EarthlyBranch::Chen)
        .gender(Gender::Female)
        .method_profile(MethodProfile::placeholder("analysis_layer_temporal_test"))
        .build()
        .expect("valid request");
    by_solar(request).expect("sample chart should build")
}

// ---- analysis_layers_for_selection ---------------------------------------

#[test]
fn natal_selection_requires_only_natal_layer() {
    assert_eq!(
        analysis_layers_for_selection(StaticTemporalNavigationSelection::Natal),
        vec![AnalysisLayerKey::Natal]
    );
}

#[test]
fn pre_decadal_selection_requires_only_natal_layer() {
    assert_eq!(
        analysis_layers_for_selection(StaticTemporalNavigationSelection::PreDecadal),
        vec![AnalysisLayerKey::Natal]
    );
}

#[test]
fn decadal_selection_requires_natal_and_decadal() {
    assert_eq!(
        analysis_layers_for_selection(StaticTemporalNavigationSelection::Decadal {
            decadal_index: 3
        }),
        vec![
            AnalysisLayerKey::Natal,
            AnalysisLayerKey::Decadal { decadal_index: 3 },
        ]
    );
}

#[test]
fn yearly_selection_requires_natal_decadal_age_yearly() {
    // Age (小限) and Yearly (流年) are distinct scopes that both become relevant
    // once a year is selected.
    assert_eq!(
        analysis_layers_for_selection(StaticTemporalNavigationSelection::Yearly {
            decadal_index: 3,
            year_index: 4,
        }),
        vec![
            AnalysisLayerKey::Natal,
            AnalysisLayerKey::Decadal { decadal_index: 3 },
            AnalysisLayerKey::Age {
                decadal_index: 3,
                year_index: 4,
            },
            AnalysisLayerKey::Yearly {
                decadal_index: 3,
                year_index: 4,
            },
        ]
    );
}

#[test]
fn monthly_selection_requires_through_monthly() {
    assert_eq!(
        analysis_layers_for_selection(StaticTemporalNavigationSelection::Monthly {
            decadal_index: 3,
            year_index: 4,
            month_index: 5,
        }),
        vec![
            AnalysisLayerKey::Natal,
            AnalysisLayerKey::Decadal { decadal_index: 3 },
            AnalysisLayerKey::Age {
                decadal_index: 3,
                year_index: 4,
            },
            AnalysisLayerKey::Yearly {
                decadal_index: 3,
                year_index: 4,
            },
            AnalysisLayerKey::Monthly {
                decadal_index: 3,
                year_index: 4,
                month_index: 5,
            },
        ]
    );
}

#[test]
fn daily_selection_requires_through_daily() {
    assert_eq!(
        analysis_layers_for_selection(StaticTemporalNavigationSelection::Daily {
            decadal_index: 3,
            year_index: 4,
            month_index: 5,
            day_index: 6,
        }),
        vec![
            AnalysisLayerKey::Natal,
            AnalysisLayerKey::Decadal { decadal_index: 3 },
            AnalysisLayerKey::Age {
                decadal_index: 3,
                year_index: 4,
            },
            AnalysisLayerKey::Yearly {
                decadal_index: 3,
                year_index: 4,
            },
            AnalysisLayerKey::Monthly {
                decadal_index: 3,
                year_index: 4,
                month_index: 5,
            },
            AnalysisLayerKey::Daily {
                decadal_index: 3,
                year_index: 4,
                month_index: 5,
                day_index: 6,
            },
        ]
    );
}

#[test]
fn hourly_selection_requires_through_hourly() {
    assert_eq!(
        analysis_layers_for_selection(StaticTemporalNavigationSelection::Hourly {
            decadal_index: 3,
            year_index: 4,
            month_index: 5,
            day_index: 6,
            hour_index: 7,
        }),
        vec![
            AnalysisLayerKey::Natal,
            AnalysisLayerKey::Decadal { decadal_index: 3 },
            AnalysisLayerKey::Age {
                decadal_index: 3,
                year_index: 4,
            },
            AnalysisLayerKey::Yearly {
                decadal_index: 3,
                year_index: 4,
            },
            AnalysisLayerKey::Monthly {
                decadal_index: 3,
                year_index: 4,
                month_index: 5,
            },
            AnalysisLayerKey::Daily {
                decadal_index: 3,
                year_index: 4,
                month_index: 5,
                day_index: 6,
            },
            AnalysisLayerKey::Hourly {
                decadal_index: 3,
                year_index: 4,
                month_index: 5,
                day_index: 6,
                hour_index: 7,
            },
        ]
    );
}

// ---- layer key scope mapping ---------------------------------------------

#[test]
fn layer_key_maps_to_correct_scopes() {
    let cases = [
        (
            AnalysisLayerKey::Natal,
            Scope::Natal,
            ClaimScope::Natal,
            PatternScope::Natal,
        ),
        (
            AnalysisLayerKey::Decadal { decadal_index: 0 },
            Scope::Decadal,
            ClaimScope::Decadal,
            PatternScope::Decadal,
        ),
        (
            AnalysisLayerKey::Age {
                decadal_index: 0,
                year_index: 0,
            },
            Scope::Age,
            ClaimScope::Age,
            PatternScope::Age,
        ),
        (
            AnalysisLayerKey::Yearly {
                decadal_index: 0,
                year_index: 0,
            },
            Scope::Yearly,
            ClaimScope::Yearly,
            PatternScope::Yearly,
        ),
        (
            AnalysisLayerKey::Monthly {
                decadal_index: 0,
                year_index: 0,
                month_index: 0,
            },
            Scope::Monthly,
            ClaimScope::Monthly,
            PatternScope::Monthly,
        ),
        (
            AnalysisLayerKey::Daily {
                decadal_index: 0,
                year_index: 0,
                month_index: 0,
                day_index: 0,
            },
            Scope::Daily,
            ClaimScope::Daily,
            PatternScope::Daily,
        ),
        (
            AnalysisLayerKey::Hourly {
                decadal_index: 0,
                year_index: 0,
                month_index: 0,
                day_index: 0,
                hour_index: 0,
            },
            Scope::Hourly,
            ClaimScope::Hourly,
            PatternScope::Hourly,
        ),
    ];

    for (key, scope, claim_scope, pattern_scope) in cases {
        assert_eq!(key.scope(), scope);
        assert_eq!(key.claim_scope(), claim_scope);
        assert_eq!(key.pattern_scope(), pattern_scope);
    }
}

// ---- analysis_scopes_for_layer_key ---------------------------------------

#[test]
fn analysis_scopes_truncate_to_each_layer() {
    use Scope::*;
    let cases = [
        (AnalysisLayerKey::Natal, vec![Natal]),
        (
            AnalysisLayerKey::Decadal { decadal_index: 0 },
            vec![Natal, Decadal],
        ),
        (
            AnalysisLayerKey::Age {
                decadal_index: 0,
                year_index: 0,
            },
            vec![Natal, Decadal, Age],
        ),
        (
            AnalysisLayerKey::Yearly {
                decadal_index: 0,
                year_index: 0,
            },
            vec![Natal, Decadal, Age, Yearly],
        ),
        (
            AnalysisLayerKey::Monthly {
                decadal_index: 0,
                year_index: 0,
                month_index: 0,
            },
            vec![Natal, Decadal, Age, Yearly, Monthly],
        ),
        (
            AnalysisLayerKey::Daily {
                decadal_index: 0,
                year_index: 0,
                month_index: 0,
                day_index: 0,
            },
            vec![Natal, Decadal, Age, Yearly, Monthly, Daily],
        ),
        (
            AnalysisLayerKey::Hourly {
                decadal_index: 0,
                year_index: 0,
                month_index: 0,
                day_index: 0,
                hour_index: 0,
            },
            vec![Natal, Decadal, Age, Yearly, Monthly, Daily, Hourly],
        ),
    ];

    for (key, expected) in cases {
        assert_eq!(
            analysis_scopes_for_layer_key(&key),
            expected,
            "active scopes for {key:?} must be its ancestors and itself, never a descendant"
        );
    }
}

// ---- detect_analysis_layer: rules ----------------------------------------

#[test]
fn quan_shu_rule_hits_are_returned_under_natal_only() {
    let chart = rich_natal_chart();
    let ctx = TemporalAnalysisContext::natal(&chart);
    let request = AnalysisLayerRequest::user_facing();

    let natal = detect_analysis_layer(&ctx, AnalysisLayerKey::Natal, &request);
    assert!(
        natal
            .rule_hits
            .iter()
            .any(|hit| hit.rule_id.as_str() == TIAN_MA_VOID),
        "expected the 马遇空亡 natal rule hit"
    );
    assert!(
        natal
            .rule_hits
            .iter()
            .all(|hit| hit.scope == ClaimScope::Natal),
        "all natal-layer rule hits must carry the natal scope"
    );

    // Current executable rules are natal-only: deeper layers carry no rule hits.
    let decadal = detect_analysis_layer(
        &ctx,
        AnalysisLayerKey::Decadal { decadal_index: 0 },
        &request,
    );
    assert!(decadal.rule_hits.is_empty());

    let yearly = detect_analysis_layer(
        &ctx,
        AnalysisLayerKey::Yearly {
            decadal_index: 0,
            year_index: 0,
        },
        &request,
    );
    assert!(yearly.rule_hits.is_empty());
}

#[test]
fn rule_hit_ref_resolves_claim_key_from_metadata_without_duplicating_source_text() {
    let chart = rich_natal_chart();
    let ctx = TemporalAnalysisContext::natal(&chart);
    let result = detect_analysis_layer(
        &ctx,
        AnalysisLayerKey::Natal,
        &AnalysisLayerRequest::user_facing(),
    );

    let hit = result
        .rule_hits
        .iter()
        .find(|hit| hit.rule_id.as_str() == TIAN_MA_VOID)
        .expect("expected the 马遇空亡 hit");

    let metadata = classical_rule_metadata(hit.rule_id.clone()).expect("metadata for matched rule");
    // The compact hit carries the claim key resolved from metadata, never the
    // verbatim source text.
    assert_eq!(hit.claim_key.as_deref(), metadata.claim_key);
    assert!(!hit.evidence.is_empty());
}

#[test]
fn user_facing_rule_hits_exclude_pattern_catalog_rules() {
    let chart = chang_qu_jia_ming_chart();

    // Sanity: the unfiltered classical engine does fire the pattern-catalog rule
    // (昌曲夹命) on this chart.
    let unfiltered = evaluate_classical(&chart, &ClaimEvaluationRequest::default());
    assert!(
        unfiltered
            .source_hits
            .iter()
            .any(|hit| hit.rule_id.as_str() == CHANG_QU),
        "the chart should fire the 昌曲夹命 pattern-catalog rule"
    );

    // The user-facing analysis request is QuanShu-only, so 全书规则 rule hits must
    // not include the IztroPatternCatalog rule — it belongs to the 格局 stream.
    let ctx = TemporalAnalysisContext::natal(&chart);
    let result = detect_analysis_layer(
        &ctx,
        AnalysisLayerKey::Natal,
        &AnalysisLayerRequest::user_facing(),
    );
    assert!(
        result
            .rule_hits
            .iter()
            .all(|hit| hit.rule_id.as_str() != CHANG_QU),
        "user-facing rule hits must exclude pattern-catalog rules"
    );
}

// ---- detect_analysis_layer: patterns -------------------------------------

#[test]
fn pattern_hits_are_returned_under_requested_scope() {
    let chart = rich_natal_chart();
    let ctx = TemporalAnalysisContext::natal(&chart);
    let request = AnalysisLayerRequest::user_facing();

    let natal = detect_analysis_layer(&ctx, AnalysisLayerKey::Natal, &request);
    assert!(
        !natal.pattern_hits.is_empty(),
        "expected natal pattern detections"
    );
    assert!(
        natal
            .pattern_hits
            .iter()
            .all(|detection| detection.scope == PatternScope::Natal),
        "natal-layer patterns must carry the natal pattern scope"
    );

    // The same natal facts must not leak into a deeper scope's layer.
    let yearly = detect_analysis_layer(
        &ctx,
        AnalysisLayerKey::Yearly {
            decadal_index: 0,
            year_index: 0,
        },
        &request,
    );
    assert!(yearly.pattern_hits.is_empty());
}

#[test]
fn selected_view_facade_returns_monthly_pattern_hits_under_requested_key() {
    let chart = sample_solar_chart();
    let request = AnalysisLayerRequest::user_facing();

    for year_index in 0..10 {
        for month_index in 0..12 {
            let selection = StaticTemporalNavigationSelection::Monthly {
                decadal_index: 0,
                year_index,
                month_index,
            };
            let requested = AnalysisLayerKey::Monthly {
                decadal_index: 0,
                year_index: year_index as usize,
                month_index: month_index as usize,
            };

            let results = detect_static_temporal_analysis_layers_from_chart(
                chart.clone(),
                selection,
                &[requested.clone()],
                &request,
            )
            .expect("selected monthly analysis should build");
            assert_eq!(results.len(), 1);
            let result = &results[0];
            assert_eq!(result.key, requested);

            if !result.pattern_hits.is_empty() {
                assert!(
                    result
                        .pattern_hits
                        .iter()
                        .all(|hit| hit.scope == PatternScope::Monthly),
                    "monthly selected-view pattern hits must carry monthly scope"
                );
                return;
            }
        }
    }

    panic!("expected at least one monthly selected-view pattern hit");
}

#[test]
fn request_toggles_suppress_facets() {
    let chart = rich_natal_chart();
    let ctx = TemporalAnalysisContext::natal(&chart);

    let rules_only = AnalysisLayerRequest {
        include_patterns: false,
        ..AnalysisLayerRequest::user_facing()
    };
    let result = detect_analysis_layer(&ctx, AnalysisLayerKey::Natal, &rules_only);
    assert!(!result.rule_hits.is_empty());
    assert!(result.pattern_hits.is_empty());

    let patterns_only = AnalysisLayerRequest {
        include_rules: false,
        ..AnalysisLayerRequest::user_facing()
    };
    let result = detect_analysis_layer(&ctx, AnalysisLayerKey::Natal, &patterns_only);
    assert!(result.rule_hits.is_empty());
    assert!(!result.pattern_hits.is_empty());
}

// ---- metadata lookup ------------------------------------------------------

#[test]
fn metadata_lookup_returns_verbatim_source_text() {
    let metadata = classical_rule_metadata(ClassicalRuleId::new(TIAN_MA_VOID))
        .expect("metadata for the 马遇空亡 rule");
    assert_eq!(metadata.source_text_zh_hans, "马遇空亡，终身奔走");
    assert!(metadata.applicable_scopes.contains(&ClaimScope::Natal));
}

#[test]
fn metadata_lookup_returns_none_for_unknown_rule() {
    assert!(classical_rule_metadata(ClassicalRuleId::new("does.not.exist")).is_none());
}

// ---- compatibility --------------------------------------------------------

#[test]
fn evaluate_classical_is_unchanged_by_context_wrapper() {
    let chart = rich_natal_chart();
    let request = ClaimEvaluationRequest::default();

    let direct = evaluate_classical(&chart, &request);
    let via_context = evaluate_classical_in_context(&ClassicalRuleContext::natal(&chart), &request);

    assert_eq!(direct, via_context);
    assert!(
        direct
            .source_hits
            .iter()
            .any(|hit| hit.rule_id.as_str() == TIAN_MA_VOID),
        "the natal classical evaluation must still fire 马遇空亡"
    );
}
