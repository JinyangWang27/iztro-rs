//! Integration tests for the pattern (格局) detection layer.
//!
//! These tests build small synthetic charts with full control over star
//! placement so detection conditions are exercised deterministically. Structured
//! fields are asserted; incidental `Vec` ordering is compared as a set.

use std::collections::BTreeSet;

use iztro::rules::pattern::query::{
    effective_branch_of_palace, effective_star_in_palace, effective_stars_in_san_fang_si_zheng,
    find_star_branch, palace_has_star, selected_branch_of_palace,
    selected_stars_in_san_fang_si_zheng, source_stars_in_san_fang_si_zheng,
    stars_in_san_fang_si_zheng, stars_in_san_fang_si_zheng_for_scope,
};
use iztro::rules::pattern::registry::{pattern_spec, pattern_specs, try_pattern_spec};
use iztro::{
    BirthContext, Brightness, CalendarDate, Chart, EarthlyBranch, Gender, HeavenlyStem,
    HoroscopeChart, MethodProfile, Mutagen, MutagenActivation, PALACE_NAMES, Palace, PatternAnchor,
    PatternContext, PatternDetectionRequest, PatternFamily, PatternId, PatternPolarity,
    PatternScope, PatternStatus, PatternStrength, Scope, ScopedStarPlacement, StarKind, StarName,
    StarPlacement, StemBranch, TemporalContext, TemporalLayer, TemporalPalaceLayout,
    TemporalPalaceName,
};
use iztro::{PalaceRelation, PatternEvidence};
use iztro::rules::source::ClassicalWork;
use iztro::{PatternSourceGroup, pattern_display_metadata, pattern_source_metadata};

/// One synthetic star placement: (branch, star, kind, optional mutagen).
type Spec = (EarthlyBranch, StarName, StarKind, Option<Mutagen>);

/// Builds a 12-palace natal chart with the Life palace at `life_branch` and the
/// given star placements. Palace branches are assigned distinctly around the
/// cycle so every branch is present.
fn build_chart(life_branch: EarthlyBranch, placements: &[Spec]) -> Chart {
    build_chart_with_body(life_branch, None, placements)
}

fn build_chart_with_body(
    life_branch: EarthlyBranch,
    body_branch: Option<EarthlyBranch>,
    placements: &[Spec],
) -> Chart {
    build_chart_with_time_and_body(life_branch, EarthlyBranch::Chen, body_branch, placements)
}

fn build_chart_with_time(
    life_branch: EarthlyBranch,
    birth_time: EarthlyBranch,
    placements: &[Spec],
) -> Chart {
    build_chart_with_time_and_body(life_branch, birth_time, None, placements)
}

fn build_chart_with_time_and_body(
    life_branch: EarthlyBranch,
    birth_time: EarthlyBranch,
    body_branch: Option<EarthlyBranch>,
    placements: &[Spec],
) -> Chart {
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
        BirthContext::new(CalendarDate::solar(1990, 5, 17), birth_time, Gender::Female),
        StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu).expect("valid stem-branch"),
        MethodProfile::placeholder("pattern_test"),
        palaces,
        body_branch,
        None,
    )
    .expect("synthetic chart should build")
}

fn major(branch: EarthlyBranch, star: StarName) -> Spec {
    (branch, star, StarKind::Major, None)
}

/// A soft/auxiliary (辅佐) star placement, e.g. 左辅/右弼, 文昌/文曲.
fn soft(branch: EarthlyBranch, star: StarName) -> Spec {
    (branch, star, StarKind::Soft, None)
}

fn temporal_context(scope: Scope) -> TemporalContext {
    let stem_branch =
        StemBranch::try_new(HeavenlyStem::Jia, EarthlyBranch::Zi).expect("valid stem-branch");
    match scope {
        Scope::Age => TemporalContext::Age {
            stem_branch,
            nominal_age: 37,
        },
        Scope::Decadal => TemporalContext::Decadal {
            stem_branch,
            start_age: 34,
        },
        Scope::Yearly => TemporalContext::Yearly {
            stem_branch,
            lunar_year: 2026,
        },
        Scope::Monthly => TemporalContext::Monthly {
            stem_branch,
            lunar_month: 5,
        },
        Scope::Daily => TemporalContext::Daily {
            stem_branch,
            lunar_day: 17,
        },
        Scope::Hourly => TemporalContext::Hourly { stem_branch },
        Scope::Natal => panic!("temporal context cannot be natal"),
    }
}

fn temporal_palace_layout(scope: Scope, life_branch: EarthlyBranch) -> TemporalPalaceLayout {
    let names = PALACE_NAMES
        .iter()
        .enumerate()
        .map(|(index, name)| TemporalPalaceName::new(life_branch.offset(index as isize), *name))
        .collect();
    TemporalPalaceLayout::try_new(scope, names).expect("valid temporal palace layout")
}

fn scoped(
    branch: EarthlyBranch,
    star: StarName,
    kind: StarKind,
    scope: Scope,
) -> ScopedStarPlacement {
    ScopedStarPlacement::new(
        branch,
        StarPlacement::new(star, kind, Brightness::Unknown, None, scope),
    )
}

fn horoscope_with_layer(
    natal: Chart,
    scope: Scope,
    temporal_life_branch: EarthlyBranch,
    placements: Vec<ScopedStarPlacement>,
    activations: Vec<MutagenActivation>,
) -> HoroscopeChart {
    let layer = TemporalLayer::try_new_with_palace_layout(
        scope,
        temporal_context(scope),
        placements,
        activations,
        Some(temporal_palace_layout(scope, temporal_life_branch)),
    )
    .expect("valid temporal layer");
    HoroscopeChart::with_layers(natal, vec![layer])
}

fn request_for_scope(scope: Scope) -> PatternDetectionRequest {
    PatternDetectionRequest {
        scopes: vec![scope],
        include_weakened: true,
        include_broken: true,
        families: Vec::new(),
    }
}

/// One synthetic star placement carrying an explicit brightness:
/// (branch, star, kind, brightness).
type BrightSpec = (EarthlyBranch, StarName, StarKind, Brightness);

/// Like [`build_chart`] but lets each placement carry an explicit
/// [`Brightness`], so brightness-gated rules can be exercised deterministically.
fn build_chart_bright(life_branch: EarthlyBranch, placements: &[BrightSpec]) -> Chart {
    build_chart_bright_with_body(life_branch, None, placements)
}

fn build_chart_bright_with_body(
    life_branch: EarthlyBranch,
    body_branch: Option<EarthlyBranch>,
    placements: &[BrightSpec],
) -> Chart {
    build_chart_bright_with_time_and_body(life_branch, EarthlyBranch::Chen, body_branch, placements)
}

fn build_chart_bright_with_time_and_body(
    life_branch: EarthlyBranch,
    birth_time: EarthlyBranch,
    body_branch: Option<EarthlyBranch>,
    placements: &[BrightSpec],
) -> Chart {
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

    Chart::try_new(
        BirthContext::new(CalendarDate::solar(1990, 5, 17), birth_time, Gender::Female),
        StemBranch::try_new(HeavenlyStem::Geng, EarthlyBranch::Wu).expect("valid stem-branch"),
        MethodProfile::placeholder("pattern_test"),
        palaces,
        body_branch,
        None,
    )
    .expect("synthetic chart should build")
}

fn branch_set(branches: &[EarthlyBranch]) -> BTreeSet<usize> {
    branches.iter().map(|branch| branch.index()).collect()
}

fn star_set(stars: &[StarName]) -> BTreeSet<StarName> {
    stars.iter().copied().collect()
}

fn detection(detections: &[iztro::PatternDetection], id: PatternId) -> &iztro::PatternDetection {
    detections
        .iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| panic!("expected pattern detection {id:?}"))
}

fn assert_detection_shape(
    detection: &iztro::PatternDetection,
    id: PatternId,
    family: PatternFamily,
    polarity: PatternPolarity,
    anchor: PatternAnchor,
    stars: &[StarName],
    palaces: &[EarthlyBranch],
) {
    assert_eq!(detection.id, id);
    assert_eq!(detection.family, family);
    assert_eq!(detection.polarity, polarity);
    assert_eq!(detection.status, PatternStatus::Fulfilled);
    assert_eq!(detection.strength, PatternStrength::Medium);
    assert_eq!(detection.anchor, anchor);
    assert_eq!(star_set(&detection.involved_stars), star_set(stars));
    assert_eq!(branch_set(&detection.involved_palaces), branch_set(palaces));
    assert!(
        !detection.evidence.is_empty(),
        "{id:?} must carry structural evidence"
    );
}

// ---- query helpers --------------------------------------------------------

#[test]
fn find_star_branch_locates_palace() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[major(EarthlyBranch::Wu, StarName::ZiWei)],
    );
    assert_eq!(
        find_star_branch(&chart, StarName::ZiWei),
        Some(EarthlyBranch::Wu)
    );
    assert_eq!(find_star_branch(&chart, StarName::TianFu), None);
}

#[test]
fn palace_has_star_checks_branch() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[major(EarthlyBranch::Wu, StarName::ZiWei)],
    );
    assert!(palace_has_star(&chart, EarthlyBranch::Wu, StarName::ZiWei));
    assert!(!palace_has_star(&chart, EarthlyBranch::Zi, StarName::ZiWei));
}

#[test]
fn stars_in_san_fang_si_zheng_filters_to_requested() {
    // SFSZ(Zi) = {Zi, Wu, Chen, Shen}. Mao is outside it.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Wu, StarName::TianJi),
            major(EarthlyBranch::Mao, StarName::TaiYin),
        ],
    );
    let found = stars_in_san_fang_si_zheng(
        &chart,
        EarthlyBranch::Zi,
        &[StarName::TianJi, StarName::TaiYin],
    );
    assert_eq!(found, vec![(StarName::TianJi, EarthlyBranch::Wu)]);
}

// ---- 紫府朝垣 -------------------------------------------------------------

#[test]
fn zi_fu_chao_yuan_positive() {
    // ZiWei@Zi, TianFu@Wu — both in SFSZ(Zi).
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Zi, StarName::ZiWei),
            major(EarthlyBranch::Wu, StarName::TianFu),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let detection = detections
        .iter()
        .find(|d| d.id == PatternId::ZiFuChaoYuan)
        .expect("expected 紫府朝垣");

    assert_eq!(detection.status, PatternStatus::Fulfilled);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::ZiWei, StarName::TianFu])
    );
    assert!(
        branch_set(&detection.involved_palaces)
            .is_superset(&branch_set(&[EarthlyBranch::Zi, EarthlyBranch::Wu]))
    );
    assert!(!detection.evidence.is_empty());
}

#[test]
fn zi_fu_chao_yuan_negative_when_tian_fu_outside() {
    // TianFu@Mao is outside SFSZ(Zi).
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Zi, StarName::ZiWei),
            major(EarthlyBranch::Mao, StarName::TianFu),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::ZiFuChaoYuan));
}

#[test]
fn zi_fu_chao_yuan_weakened_by_sha_star() {
    // Both required stars in SFSZ, plus QingYang (Tough) in an involved palace.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Zi, StarName::ZiWei),
            major(EarthlyBranch::Wu, StarName::TianFu),
            (EarthlyBranch::Zi, StarName::QingYang, StarKind::Tough, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let detection = detections
        .iter()
        .find(|d| d.id == PatternId::ZiFuChaoYuan)
        .expect("expected 紫府朝垣");
    assert_eq!(detection.status, PatternStatus::Weakened);
    assert!(!detection.weakening_factors.is_empty());
}

// ---- 机月同梁 -------------------------------------------------------------

fn ji_yue_required() -> Vec<Spec> {
    vec![
        major(EarthlyBranch::Zi, StarName::TianJi),
        major(EarthlyBranch::Wu, StarName::TaiYin),
        major(EarthlyBranch::Chen, StarName::TianTong),
        major(EarthlyBranch::Shen, StarName::TianLiang),
    ]
}

#[test]
fn ji_yue_tong_liang_positive() {
    // All four stars across SFSZ(Zi) = {Zi, Wu, Chen, Shen}.
    let chart = build_chart(EarthlyBranch::Zi, &ji_yue_required());
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let detection = detections
        .iter()
        .find(|d| d.id == PatternId::JiYueTongLiang)
        .expect("expected 机月同梁");
    assert_eq!(detection.status, PatternStatus::Fulfilled);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[
            StarName::TianJi,
            StarName::TaiYin,
            StarName::TianTong,
            StarName::TianLiang,
        ])
    );
}

#[test]
fn ji_yue_tong_liang_incomplete_formation_emits_nothing() {
    // Only two of four required stars present: the base formation is incomplete,
    // so no detection is emitted. An incomplete formation is not a near-pattern.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Zi, StarName::TianJi),
            major(EarthlyBranch::Wu, StarName::TaiYin),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::JiYueTongLiang));
}

#[test]
fn yearly_ji_yue_tong_liang_uses_effective_life_frame_and_natal_stars() {
    // Natal Life is Zi, but the selected yearly frame relabels Yin as Life.
    // The four natal stars sit in SFSZ(Yin), so a selected-state detector should
    // emit for the yearly layer even though there are no yearly flow stars.
    let natal = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Yin, StarName::TianJi),
            major(EarthlyBranch::Shen, StarName::TaiYin),
            major(EarthlyBranch::Wu, StarName::TianTong),
            major(EarthlyBranch::Xu, StarName::TianLiang),
        ],
    );
    let horoscope = horoscope_with_layer(natal, Scope::Yearly, EarthlyBranch::Yin, vec![], vec![]);

    let detections = iztro::detect_patterns(
        &PatternContext::horoscope_with_frame(
            &horoscope,
            Scope::Yearly,
            vec![Scope::Natal, Scope::Yearly],
        ),
        &request_for_scope(Scope::Yearly),
    );
    let detection = detection(&detections, PatternId::JiYueTongLiang);

    assert_eq!(detection.scope, PatternScope::Yearly);
    assert_eq!(detection.anchor, PatternAnchor::Palace(EarthlyBranch::Yin));
    assert_eq!(
        branch_set(&detection.involved_palaces),
        branch_set(&[
            EarthlyBranch::Yin,
            EarthlyBranch::Shen,
            EarthlyBranch::Wu,
            EarthlyBranch::Xu,
        ])
    );
}

// ---- 羊陀夹忌 -------------------------------------------------------------

#[test]
fn yang_tuo_jia_ji_positive() {
    // 化忌 on TaiYang@Mao; clamp(Mao) = {Yin, Chen}. QingYang@Yin, TuoLuo@Chen.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
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
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let detection = detections
        .iter()
        .find(|d| d.id == PatternId::YangTuoJiaJi)
        .expect("expected 羊陀夹忌");
    assert_eq!(detection.status, PatternStatus::Fulfilled);
    assert_eq!(detection.involved_mutagens, vec![Mutagen::Ji]);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::QingYang, StarName::TuoLuo, StarName::TaiYang])
    );
    assert_eq!(
        branch_set(&detection.involved_palaces),
        branch_set(&[EarthlyBranch::Yin, EarthlyBranch::Chen, EarthlyBranch::Mao])
    );
}

#[test]
fn decadal_yang_tuo_jia_ji_uses_flow_clamps_and_temporal_ji_activation() {
    // A decadal 化忌 activation lands on TaiYang@Zi. 运羊/运陀 clamp Zi from Hai
    // and Chou, so the decadal layer should emit 羊陀夹忌 without requiring natal
    // QingYang/TuoLuo placements.
    let natal = build_chart(
        EarthlyBranch::Zi,
        &[major(EarthlyBranch::Zi, StarName::TaiYang)],
    );
    let horoscope = horoscope_with_layer(
        natal,
        Scope::Decadal,
        EarthlyBranch::Zi,
        vec![
            scoped(
                EarthlyBranch::Hai,
                StarName::YunYang,
                StarKind::Tough,
                Scope::Decadal,
            ),
            scoped(
                EarthlyBranch::Chou,
                StarName::YunTuo,
                StarKind::Tough,
                Scope::Decadal,
            ),
        ],
        vec![MutagenActivation::new(
            Scope::Decadal,
            StarName::TaiYang,
            EarthlyBranch::Zi,
            Mutagen::Ji,
        )],
    );

    let detections = iztro::detect_patterns(
        &PatternContext::horoscope(&horoscope, vec![Scope::Natal, Scope::Decadal]),
        &request_for_scope(Scope::Decadal),
    );
    let detection = detection(&detections, PatternId::YangTuoJiaJi);

    assert_eq!(detection.scope, PatternScope::Decadal);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::YunYang, StarName::YunTuo, StarName::TaiYang])
    );
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::YunYang,
        EarthlyBranch::Hai
    ));
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::YunTuo,
        EarthlyBranch::Chou
    ));
    assert!(detection.evidence.iter().any(|evidence| {
        matches!(
            evidence,
            PatternEvidence::MutagenOnStar {
                star: StarName::TaiYang,
                mutagen: Mutagen::Ji,
                scope: Scope::Decadal,
                branch: EarthlyBranch::Zi,
            }
        )
    }));
}

#[test]
fn yang_tuo_jia_ji_negative_when_not_clamping() {
    // 化忌 on TaiYang@Mao but TuoLuo@Shen does not clamp Mao.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
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
            (EarthlyBranch::Shen, StarName::TuoLuo, StarKind::Tough, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::YangTuoJiaJi));
}

#[test]
fn yang_tuo_jia_ji_checks_all_ji_targets() {
    // Two 化忌 targets. The first encountered (TaiYang@Zi) is not clamped;
    // the later one (JuMen@Wu) is clamped by QingYang@Si and TuoLuo@Wei.
    // A naive `.find()` would stop at the unclamped target and miss this.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            (
                EarthlyBranch::Zi,
                StarName::TaiYang,
                StarKind::Major,
                Some(Mutagen::Ji),
            ),
            (
                EarthlyBranch::Wu,
                StarName::JuMen,
                StarKind::Major,
                Some(Mutagen::Ji),
            ),
            (EarthlyBranch::Si, StarName::QingYang, StarKind::Tough, None),
            (EarthlyBranch::Wei, StarName::TuoLuo, StarKind::Tough, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let detection = detections
        .iter()
        .find(|d| d.id == PatternId::YangTuoJiaJi)
        .expect("expected 羊陀夹忌 for the clamped target");
    assert_eq!(detection.status, PatternStatus::Fulfilled);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::QingYang, StarName::TuoLuo, StarName::JuMen])
    );
    assert_eq!(
        branch_set(&detection.involved_palaces),
        branch_set(&[EarthlyBranch::Si, EarthlyBranch::Wei, EarthlyBranch::Wu])
    );
}

// ---- request filters: scope, family, ordering ----------------------------

/// A chart where all three initial patterns are simultaneously fulfilled.
///
/// Life at Zi, SFSZ(Zi) = {Zi, Wu, Chen, Shen}:
/// - ZiWei@Zi, TianFu@Wu (紫府朝垣);
/// - TianJi@Zi, TaiYin@Wu, TianTong@Chen, TianLiang@Shen (机月同梁);
/// - TaiYang+化忌@Mao clamped by QingYang@Yin and TuoLuo@Chen (羊陀夹忌).
fn all_patterns_chart() -> Chart {
    build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Zi, StarName::ZiWei),
            major(EarthlyBranch::Zi, StarName::TianJi),
            major(EarthlyBranch::Wu, StarName::TianFu),
            major(EarthlyBranch::Wu, StarName::TaiYin),
            major(EarthlyBranch::Chen, StarName::TianTong),
            major(EarthlyBranch::Shen, StarName::TianLiang),
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
        ],
    )
}

#[test]
fn default_request_returns_natal_detections() {
    let chart = all_patterns_chart();
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert!(!detections.is_empty());
    assert!(detections.iter().any(|d| d.id == PatternId::ZiFuChaoYuan));
}

#[test]
fn non_natal_scope_request_returns_no_natal_detections() {
    let chart = all_patterns_chart();
    let request = PatternDetectionRequest {
        scopes: vec![Scope::Yearly],
        ..PatternDetectionRequest::default()
    };
    let detections = iztro::detect_patterns(&PatternContext::natal(&chart), &request);
    assert!(detections.is_empty());
}

#[test]
fn empty_scope_request_returns_nothing() {
    let chart = all_patterns_chart();
    let request = PatternDetectionRequest {
        scopes: Vec::new(),
        ..PatternDetectionRequest::default()
    };
    let detections = iztro::detect_patterns(&PatternContext::natal(&chart), &request);
    assert!(detections.is_empty());
}

#[test]
fn detections_are_deterministically_ordered() {
    let chart = all_patterns_chart();
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let ids: Vec<PatternId> = detections.iter().map(|d| d.id).collect();
    // All natal scope; ordered by family then id: the two MajorStarCombination
    // patterns (ZiFu before JiYue) precede the ShaJi pattern.
    assert_eq!(
        ids,
        vec![
            PatternId::ZiFuChaoYuan,
            PatternId::JiYueTongLiang,
            PatternId::YangTuoJiaJi,
        ]
    );
}

#[test]
fn family_filter_includes_only_requested_families() {
    let chart = all_patterns_chart();

    let sha_ji = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest {
            families: vec![PatternFamily::ShaJi],
            ..PatternDetectionRequest::default()
        },
    );
    assert!(sha_ji.iter().any(|d| d.id == PatternId::YangTuoJiaJi));
    assert!(sha_ji.iter().all(|d| d.id != PatternId::ZiFuChaoYuan));
    assert!(sha_ji.iter().all(|d| d.id != PatternId::JiYueTongLiang));

    let major = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest {
            families: vec![PatternFamily::MajorStarCombination],
            ..PatternDetectionRequest::default()
        },
    );
    assert!(major.iter().all(|d| d.id != PatternId::YangTuoJiaJi));
    assert!(major.iter().any(|d| d.id == PatternId::ZiFuChaoYuan));
}

// ---- 左右夹命 -------------------------------------------------------------

/// `detection.evidence` contains a `StarInPalace` for `star` at `branch`.
fn evidence_has_star_in_palace(
    detection: &iztro::PatternDetection,
    star: StarName,
    branch: EarthlyBranch,
) -> bool {
    detection.evidence.iter().any(|e| {
        matches!(
            e,
            PatternEvidence::StarInPalace { star: s, branch: b } if *s == star && *b == branch
        )
    })
}

/// `detection.evidence` contains a `ClampedBy` relation from `from` to `to`.
fn evidence_has_clamp(
    detection: &iztro::PatternDetection,
    from: EarthlyBranch,
    to: EarthlyBranch,
) -> bool {
    detection.evidence.iter().any(|e| {
        matches!(
            e,
            PatternEvidence::PalaceRelation { from: f, to: t, relation } if
                *f == from && *t == to && *relation == PalaceRelation::ClampedBy
        )
    })
}

#[test]
fn zuo_you_jia_ming_positive() {
    // Life at Zi; clamp(Zi) = {Hai, Chou}. ZuoFu@Hai, YouBi@Chou.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Hai, StarName::ZuoFu),
            soft(EarthlyBranch::Chou, StarName::YouBi),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let detection = detections
        .iter()
        .find(|d| d.id == PatternId::ZuoYouJiaMing)
        .expect("expected 左右夹命");

    assert_eq!(detection.status, PatternStatus::Fulfilled);
    assert_eq!(detection.polarity, PatternPolarity::Auspicious);
    assert_eq!(detection.family, PatternFamily::AuxiliaryStarCombination);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::ZuoFu, StarName::YouBi])
    );
    assert_eq!(
        branch_set(&detection.involved_palaces),
        branch_set(&[EarthlyBranch::Zi, EarthlyBranch::Hai, EarthlyBranch::Chou])
    );
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::ZuoFu,
        EarthlyBranch::Hai
    ));
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::YouBi,
        EarthlyBranch::Chou
    ));
    assert!(evidence_has_clamp(
        detection,
        EarthlyBranch::Zi,
        EarthlyBranch::Hai
    ));
    assert!(evidence_has_clamp(
        detection,
        EarthlyBranch::Zi,
        EarthlyBranch::Chou
    ));
}

#[test]
fn zuo_you_jia_ming_positive_reversed_orientation() {
    // The clamp helper accepts either orientation: YouBi@Hai, ZuoFu@Chou.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Hai, StarName::YouBi),
            soft(EarthlyBranch::Chou, StarName::ZuoFu),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let detection = detections
        .iter()
        .find(|d| d.id == PatternId::ZuoYouJiaMing)
        .expect("expected 左右夹命 in reversed orientation");
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::YouBi,
        EarthlyBranch::Hai
    ));
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::ZuoFu,
        EarthlyBranch::Chou
    ));
}

#[test]
fn zuo_you_jia_ming_negative_when_one_star_outside_clamp() {
    // ZuoFu@Hai clamps, but YouBi@Wu is not a clamp palace of Zi.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Hai, StarName::ZuoFu),
            soft(EarthlyBranch::Wu, StarName::YouBi),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::ZuoYouJiaMing));
}

#[test]
fn zuo_you_jia_ming_negative_when_only_one_clamp_side() {
    // Only ZuoFu@Hai present; the other clamp palace (Chou) is empty.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[soft(EarthlyBranch::Hai, StarName::ZuoFu)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::ZuoYouJiaMing));
}

// ---- 昌曲夹命 -------------------------------------------------------------

#[test]
fn chang_qu_jia_ming_positive() {
    // Life at Zi; clamp(Zi) = {Hai, Chou}. WenChang@Hai, WenQu@Chou.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Hai, StarName::WenChang),
            soft(EarthlyBranch::Chou, StarName::WenQu),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let detection = detections
        .iter()
        .find(|d| d.id == PatternId::ChangQuJiaMing)
        .expect("expected 昌曲夹命");

    assert_eq!(detection.status, PatternStatus::Fulfilled);
    assert_eq!(detection.polarity, PatternPolarity::Auspicious);
    assert_eq!(detection.family, PatternFamily::AuxiliaryStarCombination);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::WenChang, StarName::WenQu])
    );
    assert_eq!(
        branch_set(&detection.involved_palaces),
        branch_set(&[EarthlyBranch::Zi, EarthlyBranch::Hai, EarthlyBranch::Chou])
    );
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::WenChang,
        EarthlyBranch::Hai
    ));
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::WenQu,
        EarthlyBranch::Chou
    ));
    assert!(evidence_has_clamp(
        detection,
        EarthlyBranch::Zi,
        EarthlyBranch::Hai
    ));
    assert!(evidence_has_clamp(
        detection,
        EarthlyBranch::Zi,
        EarthlyBranch::Chou
    ));
}

#[test]
fn chang_qu_jia_ming_negative_when_one_star_outside_clamp() {
    // WenChang@Hai clamps, but WenQu@Wu is outside the clamp palaces of Zi.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Hai, StarName::WenChang),
            soft(EarthlyBranch::Wu, StarName::WenQu),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::ChangQuJiaMing));
}

#[test]
fn chang_qu_jia_ming_negative_when_only_one_clamp_side() {
    // Only WenQu@Chou present; the other clamp palace (Hai) is empty.
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[soft(EarthlyBranch::Chou, StarName::WenQu)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::ChangQuJiaMing));
}

#[test]
fn yearly_chang_qu_jia_ming_matches_scope_specific_flow_stars() {
    // Temporal Life at Zi; clamp(Zi) = {Hai, Chou}. The yearly layer contributes
    // 流昌/流曲, which should satisfy a 文昌/文曲 same-scope request while the
    // detection records the actual runtime flow-star names.
    let natal = build_chart(EarthlyBranch::Zi, &[]);
    let horoscope = horoscope_with_layer(
        natal,
        Scope::Yearly,
        EarthlyBranch::Zi,
        vec![
            scoped(
                EarthlyBranch::Hai,
                StarName::LiuChang,
                StarKind::Soft,
                Scope::Yearly,
            ),
            scoped(
                EarthlyBranch::Chou,
                StarName::LiuQu,
                StarKind::Soft,
                Scope::Yearly,
            ),
        ],
        Vec::new(),
    );

    let detections = iztro::detect_patterns(
        &PatternContext::horoscope(&horoscope, vec![Scope::Natal, Scope::Yearly]),
        &request_for_scope(Scope::Yearly),
    );
    let detection = detection(&detections, PatternId::ChangQuJiaMing);

    assert_eq!(detection.scope, PatternScope::Yearly);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::LiuChang, StarName::LiuQu])
    );
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::LiuChang,
        EarthlyBranch::Hai
    ));
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::LiuQu,
        EarthlyBranch::Chou
    ));
}

#[test]
fn yearly_chang_qu_jia_ming_effective_match_can_mix_natal_and_active_overlay_stars() {
    // The selected yearly Life palace is Zi. Natal WenChang clamps it from Hai,
    // while the active yearly flow WenQu clamps it from Chou. This is selected
    // chart-state matching, not same-source matching.
    let natal = build_chart(
        EarthlyBranch::Zi,
        &[soft(EarthlyBranch::Hai, StarName::WenChang)],
    );
    let horoscope = horoscope_with_layer(
        natal,
        Scope::Yearly,
        EarthlyBranch::Zi,
        vec![scoped(
            EarthlyBranch::Chou,
            StarName::LiuQu,
            StarKind::Soft,
            Scope::Yearly,
        )],
        Vec::new(),
    );

    let detections = iztro::detect_patterns(
        &PatternContext::horoscope_with_frame(
            &horoscope,
            Scope::Yearly,
            vec![Scope::Natal, Scope::Yearly],
        ),
        &request_for_scope(Scope::Yearly),
    );
    let detection = detection(&detections, PatternId::ChangQuJiaMing);

    assert_eq!(detection.scope, PatternScope::Yearly);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::WenChang, StarName::LiuQu])
    );
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::WenChang,
        EarthlyBranch::Hai
    ));
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::LiuQu,
        EarthlyBranch::Chou
    ));
}

#[test]
fn effective_helpers_require_the_selected_palace_frame_scope() {
    // Active scope does not imply selected frame. The yearly frame relabels Hai
    // as Life while natal Life stays Zi; asking an effective helper for Natal
    // must not return the yearly Life branch just because Natal is active.
    let yearly_life = EarthlyBranch::Hai;
    let natal = build_chart(EarthlyBranch::Zi, &[major(yearly_life, StarName::TaiYang)]);
    let horoscope = horoscope_with_layer(natal, Scope::Yearly, yearly_life, vec![], vec![]);
    let ctx = PatternContext::horoscope_with_frame(
        &horoscope,
        Scope::Yearly,
        vec![Scope::Natal, Scope::Yearly],
    );

    assert_ne!(
        horoscope.natal().branch_of_palace(iztro::PalaceName::Life),
        Some(yearly_life)
    );
    assert_eq!(
        effective_branch_of_palace(&ctx, Scope::Yearly, iztro::PalaceName::Life),
        Some(yearly_life)
    );
    assert_eq!(
        effective_branch_of_palace(&ctx, Scope::Natal, iztro::PalaceName::Life),
        None
    );
    assert!(effective_star_in_palace(&ctx, Scope::Natal, yearly_life, StarName::TaiYang).is_none());
    assert!(
        effective_stars_in_san_fang_si_zheng(&ctx, Scope::Natal, yearly_life, &[StarName::TaiYang])
            .is_empty()
    );
}

#[test]
fn monthly_chang_qu_jia_ming_does_not_leak_into_yearly_result() {
    let natal = build_chart(EarthlyBranch::Zi, &[]);
    let monthly = TemporalLayer::try_new_with_palace_layout(
        Scope::Monthly,
        temporal_context(Scope::Monthly),
        vec![
            scoped(
                EarthlyBranch::Hai,
                StarName::YueChang,
                StarKind::Soft,
                Scope::Monthly,
            ),
            scoped(
                EarthlyBranch::Chou,
                StarName::YueQu,
                StarKind::Soft,
                Scope::Monthly,
            ),
        ],
        Vec::new(),
        Some(temporal_palace_layout(Scope::Monthly, EarthlyBranch::Zi)),
    )
    .expect("valid monthly layer");
    let horoscope = HoroscopeChart::with_layers(natal, vec![monthly]);

    let yearly = iztro::detect_patterns(
        &PatternContext::horoscope(
            &horoscope,
            vec![Scope::Natal, Scope::Decadal, Scope::Age, Scope::Yearly],
        ),
        &request_for_scope(Scope::Yearly),
    );
    assert!(yearly.iter().all(|d| d.id != PatternId::ChangQuJiaMing));

    let monthly = iztro::detect_patterns(
        &PatternContext::horoscope_with_frame(
            &horoscope,
            Scope::Monthly,
            vec![Scope::Natal, Scope::Monthly],
        ),
        &request_for_scope(Scope::Monthly),
    );
    let detection = detection(&monthly, PatternId::ChangQuJiaMing);
    assert_eq!(detection.scope, PatternScope::Monthly);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::YueChang, StarName::YueQu])
    );
}

// ---- 日月并明 -------------------------------------------------------------

#[test]
fn ri_yue_bing_ming_positive() {
    // Both 太阳 and 太阴 in clearly bright states (庙/旺).
    let chart = build_chart_bright(
        EarthlyBranch::Zi,
        &[
            (
                EarthlyBranch::Si,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Prosperous,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let detection = detections
        .iter()
        .find(|d| d.id == PatternId::RiYueBingMing)
        .expect("expected 日月并明");
    assert_eq!(detection.status, PatternStatus::Fulfilled);
    assert_eq!(detection.polarity, PatternPolarity::Auspicious);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::TaiYang, StarName::TaiYin])
    );
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::TaiYang,
        EarthlyBranch::Si
    ));
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::TaiYin,
        EarthlyBranch::Hai
    ));
}

#[test]
fn ri_yue_bing_ming_negative_when_brightness_unknown() {
    // 太阴 brightness Unknown: never emit on an uncalculated brightness.
    let chart = build_chart_bright(
        EarthlyBranch::Zi,
        &[
            (
                EarthlyBranch::Si,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Unknown,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::RiYueBingMing));
}

#[test]
fn ri_yue_bing_ming_negative_when_one_star_dim() {
    // 太阳 bright but 太阴 trapped: not both bright.
    let chart = build_chart_bright(
        EarthlyBranch::Zi,
        &[
            (
                EarthlyBranch::Si,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Trapped,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::RiYueBingMing));
}

// ---- 日月反背 -------------------------------------------------------------

#[test]
fn ri_yue_fan_bei_positive() {
    // Both 太阳 and 太阴 in clearly dim states (陷/不).
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
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let detection = detections
        .iter()
        .find(|d| d.id == PatternId::RiYueFanBei)
        .expect("expected 日月反背");
    assert_eq!(detection.status, PatternStatus::Fulfilled);
    assert_eq!(detection.polarity, PatternPolarity::Inauspicious);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::TaiYang, StarName::TaiYin])
    );
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::TaiYang,
        EarthlyBranch::Si
    ));
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::TaiYin,
        EarthlyBranch::Hai
    ));
}

#[test]
fn ri_yue_fan_bei_negative_when_brightness_unknown() {
    // 太阳 brightness Unknown: never emit on an uncalculated brightness.
    let chart = build_chart_bright(
        EarthlyBranch::Zi,
        &[
            (
                EarthlyBranch::Si,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Unknown,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Trapped,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::RiYueFanBei));
}

#[test]
fn ri_yue_fan_bei_negative_when_one_star_bright() {
    // 太阳 dim but 太阴 prosperous: not both dim. `Flat` is also treated as
    // neutral, so a flat star never satisfies the dim condition either.
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
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::RiYueFanBei));

    // Flat brightness is neutral, not dim.
    let flat = build_chart_bright(
        EarthlyBranch::Zi,
        &[
            (
                EarthlyBranch::Si,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Flat,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Trapped,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&flat),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::RiYueFanBei));
}

// ---- AuxiliaryStarCombination family filter -------------------------------

#[test]
fn auxiliary_family_filter_includes_only_clamp_patterns() {
    // A chart fulfilling 左右夹命 (auxiliary) plus 紫府朝垣 (major).
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Hai, StarName::ZuoFu),
            soft(EarthlyBranch::Chou, StarName::YouBi),
            major(EarthlyBranch::Zi, StarName::ZiWei),
            major(EarthlyBranch::Wu, StarName::TianFu),
        ],
    );

    let aux = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest {
            families: vec![PatternFamily::AuxiliaryStarCombination],
            ..PatternDetectionRequest::default()
        },
    );
    assert!(aux.iter().any(|d| d.id == PatternId::ZuoYouJiaMing));
    assert!(aux.iter().all(|d| d.id != PatternId::ZiFuChaoYuan));
    assert!(
        aux.iter()
            .all(|d| d.family == PatternFamily::AuxiliaryStarCombination)
    );
}

// ---- QuanShu Volume 1 source-backed pattern metadata ---------------------

#[test]
fn quan_shu_source_backed_pattern_metadata_is_available_for_executable_subset() {
    let cases = [
        (
            PatternId::JinCanGuangHui,
            "金灿光辉",
            "quan_shu.v01.ding_fu_ju.jin_can_guang_hui",
            "金灿光辉 太阳单守，命在午宫是也",
            PatternSourceGroup::Wealth,
        ),
        (
            PatternId::RiChuFuSang,
            "日出扶桑",
            "quan_shu.v01.ding_gui_ju.ri_chu_fu_sang",
            "日出扶桑 日在卯守命是也，守官禄宫亦然",
            PatternSourceGroup::Noble,
        ),
        (
            PatternId::YueLuoHaiGong,
            "月落亥宫",
            "quan_shu.v01.ding_gui_ju.yue_luo_hai_gong",
            "月落亥宫 月在亥守命是也，又名月朗天门",
            PatternSourceGroup::Noble,
        ),
        (
            PatternId::YueShengCangHai,
            "月生沧海",
            "quan_shu.v01.ding_gui_ju.yue_sheng_cang_hai",
            "月生沧海 月在子宫守田宅是也",
            PatternSourceGroup::Noble,
        ),
        (
            PatternId::MaTouDaiJian,
            "马头带剑",
            "quan_shu.v01.ding_gui_ju.ma_tou_dai_jian",
            "马头带剑 谓马有刃是也不是居午格",
            PatternSourceGroup::Noble,
        ),
        (
            PatternId::TanHuoXiangFeng,
            "贪火相逢",
            "quan_shu.v01.ding_gui_ju.tan_huo_xiang_feng",
            "贪火相逢 谓二星守命同居庙旺是也",
            PatternSourceGroup::Noble,
        ),
        (
            PatternId::WuQuShouYuan,
            "武曲守垣",
            "quan_shu.v01.ding_gui_ju.wu_qu_shou_yuan",
            "武曲守垣 武守命卯宫是也，余不是",
            PatternSourceGroup::Noble,
        ),
        (
            PatternId::CaiYuQiuChou,
            "财与囚仇",
            "quan_shu.v01.ding_pin_jian_ju.cai_yu_qiu_chou",
            "财与囚仇 武贞同守身命是也",
            PatternSourceGroup::PovertyLowStatus,
        ),
        (
            PatternId::MaLuoKongWang,
            "马落空亡",
            "quan_shu.v01.ding_pin_jian_ju.ma_luo_kong_wang",
            "马落空亡 马既落亡虽禄冲会无用主奔波",
            PatternSourceGroup::PovertyLowStatus,
        ),
    ];

    for (id, name, source_id, source_text, group) in cases {
        let metadata = pattern_source_metadata(id)
            .unwrap_or_else(|| panic!("missing source metadata for {id:?}"));
        assert_eq!(metadata.pattern_id, id);
        assert_eq!(metadata.name_zh, name);
        assert_eq!(metadata.source_id, source_id);
        assert_eq!(metadata.source_text_zh_hans, source_text);
        assert_eq!(metadata.group, group);
        assert_eq!(metadata.work, ClassicalWork::ZiWeiDouShuQuanShu);
        // Typed `work` must still serialize to the historical snake_case id.
        assert_eq!(
            serde_json::to_value(metadata.work).unwrap(),
            "zi_wei_dou_shu_quan_shu"
        );
    }
}

#[test]
fn pattern_display_metadata_separates_runtime_display_from_source_provenance() {
    let display = pattern_display_metadata(PatternId::RiChuFuSang);
    assert_eq!(display.pattern_id, PatternId::RiChuFuSang);
    assert_eq!(display.name_zh, "日照雷门");
    assert_eq!(display.aliases_zh, &["日出扶桑格"]);
    assert!(display.condition_note_zh_hans.contains("出生时辰为卯至未"));

    let source = pattern_source_metadata(PatternId::RiChuFuSang).expect("source metadata");
    assert_eq!(source.name_zh, "日出扶桑");
    assert_eq!(
        source.source_text_zh_hans,
        "日出扶桑 日在卯守命是也，守官禄宫亦然"
    );
}

#[test]
fn display_metadata_carries_unverified_source_notes_without_source_metadata() {
    let ming_zhu = pattern_display_metadata(PatternId::MingZhuChuHai);
    assert_eq!(
        ming_zhu.source_note_zh_hans,
        Some("三合明珠生旺地稳步蟾宫（斗数骨髓赋）")
    );
    assert!(pattern_source_metadata(PatternId::MingZhuChuHai).is_none());

    let fu_xiang = pattern_display_metadata(PatternId::FuXiangChaoYuan);
    assert_eq!(
        fu_xiang.source_note_zh_hans,
        Some("府相朝垣 见前批注（紫微斗数全书）")
    );
    assert!(pattern_source_metadata(PatternId::FuXiangChaoYuan).is_none());
}

#[test]
fn pattern_registry_covers_every_pattern_id_once() {
    assert_eq!(pattern_specs().len(), PatternId::ALL.len());

    let mut ids = BTreeSet::new();
    for spec in pattern_specs() {
        assert!(
            ids.insert(spec.id),
            "duplicate pattern spec for {:?}",
            spec.id
        );
        assert!(
            PatternId::ALL.contains(&spec.id),
            "registry contains unknown pattern id {:?}",
            spec.id
        );
        assert!(try_pattern_spec(spec.id).is_some());
    }

    for id in PatternId::ALL {
        assert!(ids.contains(&id), "registry missing pattern id {id:?}");
    }
}

#[test]
fn pattern_metadata_wrappers_delegate_to_registry() {
    for id in PatternId::ALL {
        let spec = pattern_spec(id);
        let display = pattern_display_metadata(id);

        assert_eq!(spec.id, id);
        assert_eq!(spec.name_zh, display.name_zh);
        assert_eq!(spec.aliases_zh, display.aliases_zh);
        assert_eq!(spec.display, *display);
        assert_eq!(spec.source.as_ref(), pattern_source_metadata(id));
    }
}

#[test]
fn pattern_display_metadata_preserves_public_field_shape() {
    let metadata = iztro::PatternDisplayMetadata {
        pattern_id: PatternId::ZiFuChaoYuan,
        name_zh: "紫府朝垣",
        aliases_zh: &[],
        condition_note_zh_hans: "紫微与天府同在命宫三方四正。",
        source_note_zh_hans: None,
        interpretation_note_zh_hans: None,
    };

    assert_eq!(metadata, pattern_spec(PatternId::ZiFuChaoYuan).display);
}

// ---- QuanShu Volume 1 executable pattern subset --------------------------

#[test]
fn jin_can_guang_hui_positive_and_negative() {
    let positive = build_chart(
        EarthlyBranch::Wu,
        &[
            major(EarthlyBranch::Wu, StarName::TaiYang),
            soft(EarthlyBranch::Wu, StarName::WenChang),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&positive),
        &PatternDetectionRequest::default(),
    );
    let matched = detection(&detections, PatternId::JinCanGuangHui);
    assert_detection_shape(
        matched,
        PatternId::JinCanGuangHui,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        PatternAnchor::Palace(EarthlyBranch::Wu),
        &[StarName::TaiYang],
        &[EarthlyBranch::Wu],
    );
    assert!(evidence_has_star_in_palace(
        matched,
        StarName::TaiYang,
        EarthlyBranch::Wu
    ));

    let second_major = build_chart(
        EarthlyBranch::Wu,
        &[
            major(EarthlyBranch::Wu, StarName::TaiYang),
            major(EarthlyBranch::Wu, StarName::TaiYin),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&second_major),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::JinCanGuangHui));
}

#[test]
fn ri_chu_fu_sang_requires_natal_mao_life_tai_yang_tian_liang_and_support() {
    let life = build_chart_with_time(
        EarthlyBranch::Mao,
        EarthlyBranch::Mao,
        &[
            major(EarthlyBranch::Mao, StarName::TaiYang),
            major(EarthlyBranch::Mao, StarName::TianLiang),
            soft(EarthlyBranch::Wei, StarName::WenChang),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&life),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::RiChuFuSang),
        PatternId::RiChuFuSang,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        PatternAnchor::Palace(EarthlyBranch::Mao),
        &[StarName::TaiYang, StarName::TianLiang, StarName::WenChang],
        &[EarthlyBranch::Mao, EarthlyBranch::Wei],
    );

    let wrong_birth_time = build_chart_with_time(
        EarthlyBranch::Mao,
        EarthlyBranch::Yin,
        &[
            major(EarthlyBranch::Mao, StarName::TaiYang),
            major(EarthlyBranch::Mao, StarName::TianLiang),
            soft(EarthlyBranch::Wei, StarName::WenChang),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&wrong_birth_time),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::RiChuFuSang));

    let no_tian_liang = build_chart_with_time(
        EarthlyBranch::Mao,
        EarthlyBranch::Mao,
        &[
            major(EarthlyBranch::Mao, StarName::TaiYang),
            soft(EarthlyBranch::Wei, StarName::WenChang),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&no_tian_liang),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::RiChuFuSang));

    let no_support = build_chart_with_time(
        EarthlyBranch::Mao,
        EarthlyBranch::Mao,
        &[
            major(EarthlyBranch::Mao, StarName::TaiYang),
            major(EarthlyBranch::Mao, StarName::TianLiang),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&no_support),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::RiChuFuSang));
}

#[test]
fn ri_chu_fu_sang_no_longer_matches_career_only_tai_yang_at_mao() {
    let career_life_branch = EarthlyBranch::Mao.offset(-8);
    let career = build_chart_with_time(
        career_life_branch,
        EarthlyBranch::Mao,
        &[
            major(EarthlyBranch::Mao, StarName::TaiYang),
            major(EarthlyBranch::Mao, StarName::TianLiang),
            soft(EarthlyBranch::Wei, StarName::WenChang),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&career),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::RiChuFuSang));
}

#[test]
fn support_requires_explicit_aux_star_not_arbitrary_soft_star() {
    // A 辅佐 base structure (日照雷门) with only an unrelated `StarKind::Soft` star
    // (天空) in the Life 三方四正 is not supported: 天空 is not in the explicit
    // support set (禄存／左右／曲昌／魁钺), so no detection is emitted.
    let unrelated_soft = build_chart_with_time(
        EarthlyBranch::Mao,
        EarthlyBranch::Mao,
        &[
            major(EarthlyBranch::Mao, StarName::TaiYang),
            major(EarthlyBranch::Mao, StarName::TianLiang),
            soft(EarthlyBranch::Wei, StarName::TianKong),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&unrelated_soft),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::RiChuFuSang));
}

#[test]
fn yue_luo_hai_gong_requires_tai_yin_in_hai_life() {
    let positive = build_chart(
        EarthlyBranch::Hai,
        &[major(EarthlyBranch::Hai, StarName::TaiYin)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&positive),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::YueLuoHaiGong),
        PatternId::YueLuoHaiGong,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        PatternAnchor::Palace(EarthlyBranch::Hai),
        &[StarName::TaiYin],
        &[EarthlyBranch::Hai],
    );

    let not_life = build_chart(
        EarthlyBranch::Zi,
        &[major(EarthlyBranch::Hai, StarName::TaiYin)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&not_life),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::YueLuoHaiGong));
}

#[test]
fn yue_sheng_cang_hai_requires_tai_yin_in_zi_property() {
    let life_branch = EarthlyBranch::Zi.offset(-9);
    let positive = build_chart(life_branch, &[major(EarthlyBranch::Zi, StarName::TaiYin)]);
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&positive),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::YueShengCangHai),
        PatternId::YueShengCangHai,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        PatternAnchor::Palace(EarthlyBranch::Zi),
        &[StarName::TaiYin],
        &[EarthlyBranch::Zi],
    );

    let not_property = build_chart(
        EarthlyBranch::Zi,
        &[major(EarthlyBranch::Zi, StarName::TaiYin)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&not_property),
        &PatternDetectionRequest::default(),
    );
    assert!(
        detections
            .iter()
            .all(|d| d.id != PatternId::YueShengCangHai)
    );
}

#[test]
fn ma_tou_dai_jian_matches_tian_ma_with_qing_yang_in_any_branch() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            (
                EarthlyBranch::Chen,
                StarName::TianMa,
                StarKind::TianMa,
                None,
            ),
            (
                EarthlyBranch::Chen,
                StarName::QingYang,
                StarKind::Tough,
                None,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::MaTouDaiJian),
        PatternId::MaTouDaiJian,
        PatternFamily::ShaJi,
        PatternPolarity::Neutral,
        PatternAnchor::Palace(EarthlyBranch::Chen),
        &[StarName::TianMa, StarName::QingYang],
        &[EarthlyBranch::Chen],
    );

    let separated = build_chart(
        EarthlyBranch::Zi,
        &[
            (
                EarthlyBranch::Chen,
                StarName::TianMa,
                StarKind::TianMa,
                None,
            ),
            (EarthlyBranch::Wu, StarName::QingYang, StarKind::Tough, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&separated),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::MaTouDaiJian));
}

#[test]
fn tan_huo_xiang_feng_requires_life_same_palace_and_bright_states() {
    let positive = build_chart_bright(
        EarthlyBranch::Yin,
        &[
            (
                EarthlyBranch::Yin,
                StarName::TanLang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Yin,
                StarName::HuoXing,
                StarKind::Tough,
                Brightness::Prosperous,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&positive),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::TanHuoXiangFeng),
        PatternId::TanHuoXiangFeng,
        PatternFamily::ShaJi,
        PatternPolarity::Auspicious,
        PatternAnchor::Palace(EarthlyBranch::Yin),
        &[StarName::TanLang, StarName::HuoXing],
        &[EarthlyBranch::Yin],
    );

    let unknown = build_chart_bright(
        EarthlyBranch::Yin,
        &[
            (
                EarthlyBranch::Yin,
                StarName::TanLang,
                StarKind::Major,
                Brightness::Unknown,
            ),
            (
                EarthlyBranch::Yin,
                StarName::HuoXing,
                StarKind::Tough,
                Brightness::Prosperous,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&unknown),
        &PatternDetectionRequest::default(),
    );
    assert!(
        detections
            .iter()
            .all(|d| d.id != PatternId::TanHuoXiangFeng)
    );
}

#[test]
fn wu_qu_shou_yuan_requires_wu_qu_life_at_mao() {
    let positive = build_chart(
        EarthlyBranch::Mao,
        &[major(EarthlyBranch::Mao, StarName::WuQu)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&positive),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::WuQuShouYuan),
        PatternId::WuQuShouYuan,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        PatternAnchor::Palace(EarthlyBranch::Mao),
        &[StarName::WuQu],
        &[EarthlyBranch::Mao],
    );

    let wrong_branch = build_chart(
        EarthlyBranch::Wu,
        &[major(EarthlyBranch::Wu, StarName::WuQu)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&wrong_branch),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::WuQuShouYuan));
}

#[test]
fn cai_yu_qiu_chou_matches_wu_qu_lian_zhen_in_life_or_body() {
    let life = build_chart(
        EarthlyBranch::Chou,
        &[
            major(EarthlyBranch::Chou, StarName::WuQu),
            major(EarthlyBranch::Chou, StarName::LianZhen),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&life),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::CaiYuQiuChou),
        PatternId::CaiYuQiuChou,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Inauspicious,
        PatternAnchor::Palace(EarthlyBranch::Chou),
        &[StarName::WuQu, StarName::LianZhen],
        &[EarthlyBranch::Chou],
    );

    let body = build_chart_with_body(
        EarthlyBranch::Zi,
        Some(EarthlyBranch::Si),
        &[
            major(EarthlyBranch::Si, StarName::WuQu),
            major(EarthlyBranch::Si, StarName::LianZhen),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&body),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().any(|d| d.id == PatternId::CaiYuQiuChou));

    let neither = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Wu, StarName::WuQu),
            major(EarthlyBranch::Wu, StarName::LianZhen),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&neither),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::CaiYuQiuChou));
}

#[test]
fn ma_luo_kong_wang_matches_tian_ma_with_modeled_void_star() {
    let chart = build_chart(
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
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::MaLuoKongWang),
        PatternId::MaLuoKongWang,
        PatternFamily::ShaJi,
        PatternPolarity::Inauspicious,
        PatternAnchor::Palace(EarthlyBranch::Hai),
        &[StarName::TianMa, StarName::XunKong],
        &[EarthlyBranch::Hai],
    );

    let tian_kong_is_not_void = build_chart(
        EarthlyBranch::Zi,
        &[
            (EarthlyBranch::Hai, StarName::TianMa, StarKind::TianMa, None),
            (
                EarthlyBranch::Hai,
                StarName::TianKong,
                StarKind::Adjective,
                None,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&tian_kong_is_not_void),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::MaLuoKongWang));
}

#[test]
fn ming_li_feng_kong_requires_di_kong_or_di_jie_in_life() {
    // 地空守命.
    let di_kong = build_chart(
        EarthlyBranch::Zi,
        &[(EarthlyBranch::Zi, StarName::DiKong, StarKind::Tough, None)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&di_kong),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::MingLiFengKong),
        PatternId::MingLiFengKong,
        PatternFamily::ShaJi,
        PatternPolarity::Inauspicious,
        PatternAnchor::Palace(EarthlyBranch::Zi),
        &[StarName::DiKong],
        &[EarthlyBranch::Zi],
    );

    // 地劫守命.
    let di_jie = build_chart(
        EarthlyBranch::Zi,
        &[(EarthlyBranch::Zi, StarName::DiJie, StarKind::Tough, None)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&di_jie),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::MingLiFengKong),
        PatternId::MingLiFengKong,
        PatternFamily::ShaJi,
        PatternPolarity::Inauspicious,
        PatternAnchor::Palace(EarthlyBranch::Zi),
        &[StarName::DiJie],
        &[EarthlyBranch::Zi],
    );

    // 地空、地劫二星同守命: involved_stars carries both, not only the first.
    let both = build_chart(
        EarthlyBranch::Zi,
        &[
            (EarthlyBranch::Zi, StarName::DiKong, StarKind::Tough, None),
            (EarthlyBranch::Zi, StarName::DiJie, StarKind::Tough, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&both),
        &PatternDetectionRequest::default(),
    );
    let matched = detection(&detections, PatternId::MingLiFengKong);
    assert_eq!(
        star_set(&matched.involved_stars),
        star_set(&[StarName::DiKong, StarName::DiJie])
    );
    assert!(evidence_has_star_in_palace(
        matched,
        StarName::DiKong,
        EarthlyBranch::Zi
    ));
    assert!(evidence_has_star_in_palace(
        matched,
        StarName::DiJie,
        EarthlyBranch::Zi
    ));

    // 旬空 (modeled void family)守命 is no longer this pattern.
    let xun_kong = build_chart(
        EarthlyBranch::Zi,
        &[(
            EarthlyBranch::Zi,
            StarName::XunKong,
            StarKind::Adjective,
            None,
        )],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&xun_kong),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::MingLiFengKong));

    // 地空/地劫 outside the Life palace does not trigger.
    let not_life = build_chart(
        EarthlyBranch::Zi,
        &[(EarthlyBranch::Wu, StarName::DiKong, StarKind::Tough, None)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&not_life),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::MingLiFengKong));
}

#[test]
fn lu_feng_chong_po_matches_lu_in_life_broken_by_kong_jie() {
    // 禄存坐命 (Zi) broken by 地空 in the Life 三方四正 (Wu, the opposite palace).
    let lu_cun = build_chart(
        EarthlyBranch::Zi,
        &[
            (EarthlyBranch::Zi, StarName::LuCun, StarKind::LuCun, None),
            (EarthlyBranch::Wu, StarName::DiKong, StarKind::Tough, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&lu_cun),
        &PatternDetectionRequest::default(),
    );
    let matched = detection(&detections, PatternId::LuFengChongPo);
    assert_eq!(matched.status, PatternStatus::Broken);
    assert_eq!(matched.anchor, PatternAnchor::Palace(EarthlyBranch::Zi));
    assert!(matched.involved_mutagens.is_empty());
    assert!(evidence_has_star_in_palace(
        matched,
        StarName::LuCun,
        EarthlyBranch::Zi
    ));
    assert!(matched.breaking_factors.iter().any(|factor| {
        matches!(
            factor,
            iztro::PatternCondition::BrokenByStar {
                star: StarName::DiKong,
                branch: EarthlyBranch::Wu,
            }
        )
    }));

    // 化禄坐命 broken by 地劫 in the Life 三方四正.
    let hua_lu = build_chart(
        EarthlyBranch::Zi,
        &[
            (
                EarthlyBranch::Zi,
                StarName::TaiYang,
                StarKind::Major,
                Some(Mutagen::Lu),
            ),
            (EarthlyBranch::Wu, StarName::DiJie, StarKind::Tough, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&hua_lu),
        &PatternDetectionRequest::default(),
    );
    let matched = detection(&detections, PatternId::LuFengChongPo);
    assert_eq!(matched.status, PatternStatus::Broken);
    assert_eq!(matched.involved_mutagens, vec![Mutagen::Lu]);
    assert!(matched.evidence.iter().any(|evidence| {
        matches!(
            evidence,
            PatternEvidence::MutagenOnStar {
                star: StarName::TaiYang,
                mutagen: Mutagen::Lu,
                scope: Scope::Natal,
                branch: EarthlyBranch::Zi,
            }
        )
    }));

    // 禄存 outside the Life palace: no 禄坐命 base.
    let lu_outside = build_chart(
        EarthlyBranch::Zi,
        &[
            (EarthlyBranch::Wu, StarName::LuCun, StarKind::LuCun, None),
            (EarthlyBranch::Wu, StarName::DiKong, StarKind::Tough, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&lu_outside),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::LuFengChongPo));

    // 禄存坐命 but only 擎羊 in the 三方四正: a 煞星 is not a 地空/地劫 breaker.
    let qing_yang = build_chart(
        EarthlyBranch::Zi,
        &[
            (EarthlyBranch::Zi, StarName::LuCun, StarKind::LuCun, None),
            (EarthlyBranch::Wu, StarName::QingYang, StarKind::Tough, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&qing_yang),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::LuFengChongPo));

    // 禄存坐命 but only 旬空 in the 三方四正: a modeled void star is not a breaker.
    let xun_kong = build_chart(
        EarthlyBranch::Zi,
        &[
            (EarthlyBranch::Zi, StarName::LuCun, StarKind::LuCun, None),
            (
                EarthlyBranch::Wu,
                StarName::XunKong,
                StarKind::Adjective,
                None,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&xun_kong),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::LuFengChongPo));

    // 禄存坐命 with 地空 outside the Life 三方四正 (Mao): no breaker in range.
    let kong_outside = build_chart(
        EarthlyBranch::Zi,
        &[
            (EarthlyBranch::Zi, StarName::LuCun, StarKind::LuCun, None),
            (EarthlyBranch::Mao, StarName::DiKong, StarKind::Tough, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&kong_outside),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::LuFengChongPo));
}

#[test]
fn wen_xing_gong_ming_requires_chang_qu_in_life_san_fang_si_zheng() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Zi, StarName::WenChang),
            soft(EarthlyBranch::Wu, StarName::WenQu),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::WenXingGongMing),
        PatternId::WenXingGongMing,
        PatternFamily::AuxiliaryStarCombination,
        PatternPolarity::Auspicious,
        PatternAnchor::Palace(EarthlyBranch::Zi),
        &[StarName::WenChang, StarName::WenQu],
        &[EarthlyBranch::Zi, EarthlyBranch::Wu],
    );

    let outside = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Zi, StarName::WenChang),
            soft(EarthlyBranch::Mao, StarName::WenQu),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&outside),
        &PatternDetectionRequest::default(),
    );
    assert!(
        detections
            .iter()
            .all(|d| d.id != PatternId::WenXingGongMing)
    );
}

#[test]
fn decadal_wen_xing_gong_ming_uses_effective_selected_state() {
    // Natal Life is Zi, but the selected decadal frame relabels Yin as Life.
    // Natal WenChang/WenQu sit in SFSZ(Yin), so ordinary selected-state
    // detection should emit for Decadal even without decadal flow Chang/Qu.
    let decadal_life = EarthlyBranch::Yin;
    let natal = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(EarthlyBranch::Shen, StarName::WenChang),
            soft(EarthlyBranch::Wu, StarName::WenQu),
        ],
    );
    let horoscope = horoscope_with_layer(natal, Scope::Decadal, decadal_life, vec![], vec![]);

    assert_ne!(
        horoscope.natal().branch_of_palace(iztro::PalaceName::Life),
        Some(decadal_life)
    );

    let detections = iztro::detect_patterns(
        &PatternContext::horoscope_with_frame(
            &horoscope,
            Scope::Decadal,
            vec![Scope::Natal, Scope::Decadal],
        ),
        &request_for_scope(Scope::Decadal),
    );
    let detection = detection(&detections, PatternId::WenXingGongMing);

    assert_eq!(detection.scope, PatternScope::Decadal);
    assert_eq!(detection.anchor, PatternAnchor::Palace(decadal_life));
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::WenChang, StarName::WenQu])
    );
    assert!(detection.evidence.iter().any(|evidence| {
        matches!(
            evidence,
            PatternEvidence::StarsInSanFangSiZheng { stars, anchor, branches }
                if star_set(stars) == star_set(&[StarName::WenChang, StarName::WenQu])
                    && *anchor == decadal_life
                    && branch_set(branches)
                        == branch_set(&[EarthlyBranch::Shen, EarthlyBranch::Wu])
        )
    }));
}

#[test]
fn tian_ji_si_hai_requires_tian_ji_seated_in_si_or_hai_life() {
    // 天机在巳坐命.
    let si = build_chart(
        EarthlyBranch::Si,
        &[major(EarthlyBranch::Si, StarName::TianJi)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&si),
        &PatternDetectionRequest::default(),
    );
    let matched = detection(&detections, PatternId::TianJiSiHai);
    assert_detection_shape(
        matched,
        PatternId::TianJiSiHai,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Inauspicious,
        PatternAnchor::Palace(EarthlyBranch::Si),
        &[StarName::TianJi],
        &[EarthlyBranch::Si],
    );
    assert_eq!(matched.polarity, PatternPolarity::Inauspicious);

    // 天机在亥坐命.
    let hai = build_chart(
        EarthlyBranch::Hai,
        &[major(EarthlyBranch::Hai, StarName::TianJi)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&hai),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::TianJiSiHai),
        PatternId::TianJiSiHai,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Inauspicious,
        PatternAnchor::Palace(EarthlyBranch::Hai),
        &[StarName::TianJi],
        &[EarthlyBranch::Hai],
    );

    // 天机在巳/亥 but not the Life palace (Life Si, 天机 at Hai).
    let not_life = build_chart(
        EarthlyBranch::Si,
        &[major(EarthlyBranch::Hai, StarName::TianJi)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&not_life),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::TianJiSiHai));

    // Life Si but no 天机 there.
    let no_tian_ji = build_chart(
        EarthlyBranch::Si,
        &[major(EarthlyBranch::Si, StarName::TaiYang)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&no_tian_ji),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::TianJiSiHai));
}

#[test]
fn zuo_you_tong_gong_requires_chou_wei_anchor_same_palace_and_support() {
    // 命宫在丑，左辅右弼同宫，禄存在三方四正加会.
    let life_chou = build_chart(
        EarthlyBranch::Chou,
        &[
            soft(EarthlyBranch::Chou, StarName::ZuoFu),
            soft(EarthlyBranch::Chou, StarName::YouBi),
            (EarthlyBranch::Si, StarName::LuCun, StarKind::LuCun, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&life_chou),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::ZuoYouTongGong),
        PatternId::ZuoYouTongGong,
        PatternFamily::AuxiliaryStarCombination,
        PatternPolarity::Auspicious,
        PatternAnchor::Palace(EarthlyBranch::Chou),
        &[StarName::ZuoFu, StarName::YouBi, StarName::LuCun],
        &[EarthlyBranch::Chou, EarthlyBranch::Si],
    );

    // 身宫在未，左辅右弼同宫，文昌在身宫三方四正加会 (Life is not Chou/Wei).
    let body_wei = build_chart_with_body(
        EarthlyBranch::Zi,
        Some(EarthlyBranch::Wei),
        &[
            soft(EarthlyBranch::Wei, StarName::ZuoFu),
            soft(EarthlyBranch::Wei, StarName::YouBi),
            soft(EarthlyBranch::Chou, StarName::WenChang),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&body_wei),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::ZuoYouTongGong),
        PatternId::ZuoYouTongGong,
        PatternFamily::AuxiliaryStarCombination,
        PatternPolarity::Auspicious,
        PatternAnchor::Palace(EarthlyBranch::Wei),
        &[StarName::ZuoFu, StarName::YouBi, StarName::WenChang],
        &[EarthlyBranch::Wei, EarthlyBranch::Chou],
    );

    // Neither Life nor Body is Chou/Wei.
    let wrong_branch = build_chart_with_body(
        EarthlyBranch::Zi,
        Some(EarthlyBranch::Si),
        &[
            soft(EarthlyBranch::Si, StarName::ZuoFu),
            soft(EarthlyBranch::Si, StarName::YouBi),
            (EarthlyBranch::Si, StarName::LuCun, StarKind::LuCun, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&wrong_branch),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::ZuoYouTongGong));

    // 左右同宫 in Chou but no additional support beyond the base pair.
    let no_support = build_chart(
        EarthlyBranch::Chou,
        &[
            soft(EarthlyBranch::Chou, StarName::ZuoFu),
            soft(EarthlyBranch::Chou, StarName::YouBi),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&no_support),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::ZuoYouTongGong));

    // Only 左辅 present at the anchor.
    let only_one = build_chart(
        EarthlyBranch::Chou,
        &[
            soft(EarthlyBranch::Chou, StarName::ZuoFu),
            (EarthlyBranch::Si, StarName::LuCun, StarKind::LuCun, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&only_one),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::ZuoYouTongGong));
}

#[test]
fn yearly_zuo_you_tong_gong_uses_selected_life_not_natal_only() {
    // Natal Life is Zi, but the selected yearly frame relabels Chou as Life.
    // The 左右 pair and extra support are natal facts in the effective yearly
    // state, so the yearly selected-state detector should emit.
    let yearly_life = EarthlyBranch::Chou;
    let natal = build_chart(
        EarthlyBranch::Zi,
        &[
            soft(yearly_life, StarName::ZuoFu),
            soft(yearly_life, StarName::YouBi),
            (EarthlyBranch::Si, StarName::LuCun, StarKind::LuCun, None),
        ],
    );
    let horoscope = horoscope_with_layer(natal, Scope::Yearly, yearly_life, vec![], vec![]);

    assert_ne!(
        horoscope.natal().branch_of_palace(iztro::PalaceName::Life),
        Some(yearly_life)
    );

    let detections = iztro::detect_patterns(
        &PatternContext::horoscope_with_frame(
            &horoscope,
            Scope::Yearly,
            vec![Scope::Natal, Scope::Yearly],
        ),
        &request_for_scope(Scope::Yearly),
    );
    let detection = detection(&detections, PatternId::ZuoYouTongGong);

    assert_eq!(detection.scope, PatternScope::Yearly);
    assert_eq!(detection.anchor, PatternAnchor::Palace(yearly_life));
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::ZuoFu, StarName::YouBi, StarName::LuCun])
    );
    assert!(evidence_has_star_in_palace(
        detection,
        StarName::LuCun,
        EarthlyBranch::Si
    ));
}

/// The exact 明珠出海 structure: 命宫在未无正曜，卯宫太阳天梁，亥宫太阴入庙旺，
/// 命宫三方四正有文昌加会.
fn ming_zhu_chu_hai_chart() -> Chart {
    build_chart_bright(
        EarthlyBranch::Wei,
        &[
            (
                EarthlyBranch::Mao,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Mao,
                StarName::TianLiang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Prosperous,
            ),
            (
                EarthlyBranch::Chou,
                StarName::WenChang,
                StarKind::Soft,
                Brightness::Unknown,
            ),
        ],
    )
}

#[test]
fn ming_zhu_chu_hai_requires_exact_wei_mao_hai_structure_with_support() {
    let chart = ming_zhu_chu_hai_chart();
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let matched = detection(&detections, PatternId::MingZhuChuHai);
    assert_detection_shape(
        matched,
        PatternId::MingZhuChuHai,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        PatternAnchor::Palace(EarthlyBranch::Wei),
        &[
            StarName::TaiYang,
            StarName::TianLiang,
            StarName::TaiYin,
            StarName::WenChang,
        ],
        &[
            EarthlyBranch::Wei,
            EarthlyBranch::Mao,
            EarthlyBranch::Hai,
            EarthlyBranch::Chou,
        ],
    );
    assert!(matched.evidence.iter().any(|evidence| {
        matches!(
            evidence,
            PatternEvidence::NoMajorStarInPalace {
                branch: EarthlyBranch::Wei
            }
        )
    }));
    assert!(evidence_has_star_in_palace(
        matched,
        StarName::TaiYin,
        EarthlyBranch::Hai
    ));

    // 明珠出海 coexists with 命无正曜 on the empty Wei Life palace.
    assert!(detections.iter().any(|d| d.id == PatternId::MingWuZhengYao));

    // Life not Wei.
    let not_wei = build_chart_bright(
        EarthlyBranch::Wu,
        &[
            (
                EarthlyBranch::Mao,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Mao,
                StarName::TianLiang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Prosperous,
            ),
            (
                EarthlyBranch::Chou,
                StarName::WenChang,
                StarKind::Soft,
                Brightness::Unknown,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&not_wei),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::MingZhuChuHai));

    // Life Wei but carrying a major star.
    let wei_has_major = build_chart_bright(
        EarthlyBranch::Wei,
        &[
            (
                EarthlyBranch::Wei,
                StarName::ZiWei,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Mao,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Mao,
                StarName::TianLiang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Prosperous,
            ),
            (
                EarthlyBranch::Chou,
                StarName::WenChang,
                StarKind::Soft,
                Brightness::Unknown,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&wei_has_major),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::MingZhuChuHai));

    // 太阳 at Mao without 天梁.
    let no_tian_liang = build_chart_bright(
        EarthlyBranch::Wei,
        &[
            (
                EarthlyBranch::Mao,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Prosperous,
            ),
            (
                EarthlyBranch::Chou,
                StarName::WenChang,
                StarKind::Soft,
                Brightness::Unknown,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&no_tian_liang),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::MingZhuChuHai));

    // 太阴 not at Hai.
    let tai_yin_elsewhere = build_chart_bright(
        EarthlyBranch::Wei,
        &[
            (
                EarthlyBranch::Mao,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Mao,
                StarName::TianLiang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::You,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Prosperous,
            ),
            (
                EarthlyBranch::Chou,
                StarName::WenChang,
                StarKind::Soft,
                Brightness::Unknown,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&tai_yin_elsewhere),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::MingZhuChuHai));

    // 太阴 at Hai but brightness Unknown: never emit on uncalculated brightness.
    let tai_yin_dim = build_chart_bright(
        EarthlyBranch::Wei,
        &[
            (
                EarthlyBranch::Mao,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Mao,
                StarName::TianLiang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Unknown,
            ),
            (
                EarthlyBranch::Chou,
                StarName::WenChang,
                StarKind::Soft,
                Brightness::Unknown,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&tai_yin_dim),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::MingZhuChuHai));

    // No support in the Life 三方四正.
    let no_support = build_chart_bright(
        EarthlyBranch::Wei,
        &[
            (
                EarthlyBranch::Mao,
                StarName::TaiYang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Mao,
                StarName::TianLiang,
                StarKind::Major,
                Brightness::Temple,
            ),
            (
                EarthlyBranch::Hai,
                StarName::TaiYin,
                StarKind::Major,
                Brightness::Prosperous,
            ),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&no_support),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::MingZhuChuHai));
}

#[test]
fn ming_wu_zheng_yao_emits_no_major_star_evidence() {
    let chart = build_chart_bright(
        EarthlyBranch::Si,
        &[(
            EarthlyBranch::Si,
            StarName::WenChang,
            StarKind::Soft,
            Brightness::Unknown,
        )],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&chart),
        &PatternDetectionRequest::default(),
    );
    let no_major = detection(&detections, PatternId::MingWuZhengYao);
    assert!(no_major.evidence.iter().any(|evidence| {
        matches!(
            evidence,
            PatternEvidence::NoMajorStarInPalace {
                branch: EarthlyBranch::Si
            }
        )
    }));

    let has_major = build_chart(
        EarthlyBranch::Si,
        &[major(EarthlyBranch::Si, StarName::ZiWei)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&has_major),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().all(|d| d.id != PatternId::MingWuZhengYao));
}

#[test]
fn ji_xiang_li_ming_is_fulfilled_without_tough_star_and_broken_with_tough_star() {
    let fulfilled = build_chart(
        EarthlyBranch::Wu,
        &[major(EarthlyBranch::Wu, StarName::ZiWei)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&fulfilled),
        &PatternDetectionRequest::default(),
    );
    let matched = detection(&detections, PatternId::JiXiangLiMing);
    assert_eq!(matched.status, PatternStatus::Fulfilled);

    let broken = build_chart(
        EarthlyBranch::Wu,
        &[
            major(EarthlyBranch::Wu, StarName::ZiWei),
            (EarthlyBranch::Zi, StarName::QingYang, StarKind::Tough, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&broken),
        &PatternDetectionRequest::default(),
    );
    let matched = detection(&detections, PatternId::JiXiangLiMing);
    assert_eq!(matched.status, PatternStatus::Broken);
    assert!(matched.breaking_factors.iter().any(|factor| {
        matches!(
            factor,
            iztro::PatternCondition::BrokenByStar {
                star: StarName::QingYang,
                branch: EarthlyBranch::Zi,
            }
        )
    }));
}

#[test]
fn fu_xiang_chao_yuan_covers_wealth_career_split_and_tian_fu_in_life_forms() {
    // For Life at Zi: Wealth palace is Chen, Career palace is Shen.
    // Form A: 天府居财帛，天相居官禄，禄存在命宫三方四正加会.
    let split = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Chen, StarName::TianFu),
            major(EarthlyBranch::Shen, StarName::TianXiang),
            (EarthlyBranch::Zi, StarName::LuCun, StarKind::LuCun, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&split),
        &PatternDetectionRequest::default(),
    );
    assert_detection_shape(
        detection(&detections, PatternId::FuXiangChaoYuan),
        PatternId::FuXiangChaoYuan,
        PatternFamily::MajorStarCombination,
        PatternPolarity::Auspicious,
        PatternAnchor::Palace(EarthlyBranch::Zi),
        &[StarName::TianFu, StarName::TianXiang, StarName::LuCun],
        &[EarthlyBranch::Chen, EarthlyBranch::Shen, EarthlyBranch::Zi],
    );

    // Form A reversed: 天相居财帛，天府居官禄.
    let split_reversed = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Chen, StarName::TianXiang),
            major(EarthlyBranch::Shen, StarName::TianFu),
            (EarthlyBranch::Zi, StarName::LuCun, StarKind::LuCun, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&split_reversed),
        &PatternDetectionRequest::default(),
    );
    assert!(
        detections
            .iter()
            .any(|d| d.id == PatternId::FuXiangChaoYuan)
    );

    // Form B: 天府坐命，天相在命宫三方四正加会, with 禄存 support.
    let tian_fu_in_life = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Zi, StarName::TianFu),
            major(EarthlyBranch::Wu, StarName::TianXiang),
            (EarthlyBranch::Chen, StarName::LuCun, StarKind::LuCun, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&tian_fu_in_life),
        &PatternDetectionRequest::default(),
    );
    assert!(
        detections
            .iter()
            .any(|d| d.id == PatternId::FuXiangChaoYuan)
    );

    // 天府/天相 only generically in the Life 三方四正 (both at the Travel palace Wu),
    // matching neither the Wealth/Career split nor the 天府坐命 form.
    let generic_only = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Wu, StarName::TianFu),
            major(EarthlyBranch::Wu, StarName::TianXiang),
            (EarthlyBranch::Zi, StarName::LuCun, StarKind::LuCun, None),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&generic_only),
        &PatternDetectionRequest::default(),
    );
    assert!(
        detections
            .iter()
            .all(|d| d.id != PatternId::FuXiangChaoYuan)
    );

    // Valid base form (split) but no support in the Life 三方四正.
    let no_support = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Chen, StarName::TianFu),
            major(EarthlyBranch::Shen, StarName::TianXiang),
        ],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&no_support),
        &PatternDetectionRequest::default(),
    );
    assert!(
        detections
            .iter()
            .all(|d| d.id != PatternId::FuXiangChaoYuan)
    );
}

// ---- PatternContext API boundaries -----------------------------------------

#[test]
fn natal_context_has_natal_selected_frame_and_effective_state() {
    let chart = build_chart(EarthlyBranch::Zi, &[]);
    let ctx = PatternContext::natal(&chart);

    assert_eq!(ctx.selected_frame_scope(), Some(Scope::Natal));
    assert_eq!(ctx.active_scopes(), &[Scope::Natal]);
    assert!(ctx.effective().is_some(), "natal context is strict");
    assert!(ctx.horoscope_chart().is_none());
}

#[test]
fn horoscope_with_frame_context_uses_the_explicit_frame() {
    let natal = build_chart(EarthlyBranch::Zi, &[]);
    let horoscope = horoscope_with_layer(natal, Scope::Yearly, EarthlyBranch::Chou, vec![], vec![]);
    let active = vec![Scope::Natal, Scope::Yearly];
    let ctx = PatternContext::horoscope_with_frame(&horoscope, Scope::Yearly, active.clone());

    assert_eq!(
        ctx.selected_frame_scope(),
        Some(Scope::Yearly),
        "frame comes from the explicit argument, not the deepest active scope"
    );
    assert_eq!(ctx.active_scopes(), active.as_slice());
    assert!(ctx.effective().is_some(), "explicit frame is strict");
    assert_eq!(
        selected_branch_of_palace(&ctx, iztro::PalaceName::Life),
        Some(EarthlyBranch::Chou)
    );
}

#[test]
fn horoscope_context_derives_frame_from_the_deepest_active_scope() {
    let natal = build_chart(EarthlyBranch::Zi, &[]);
    let horoscope = horoscope_with_layer(natal, Scope::Yearly, EarthlyBranch::Chou, vec![], vec![]);
    let ctx = PatternContext::horoscope(&horoscope, vec![Scope::Natal, Scope::Yearly]);

    assert_eq!(
        ctx.selected_frame_scope(),
        Some(Scope::Yearly),
        "compatibility constructor derives the frame from active_scopes.last()"
    );
    assert!(ctx.effective().is_some());
}

#[test]
fn horoscope_context_fails_closed_when_strict_effective_state_is_invalid() {
    // active_scopes without Natal cannot form a strict effective state, so the
    // compatibility constructor leaves `effective` empty and selected-state
    // helpers fail closed rather than guessing.
    let natal = build_chart(
        EarthlyBranch::Zi,
        &[soft(EarthlyBranch::Chou, StarName::WenChang)],
    );
    let horoscope = horoscope_with_layer(natal, Scope::Yearly, EarthlyBranch::Chou, vec![], vec![]);
    let ctx = PatternContext::horoscope(&horoscope, vec![Scope::Yearly]);

    assert_eq!(ctx.active_scopes(), &[Scope::Yearly]);
    assert!(
        ctx.effective().is_none(),
        "strict construction failed, so no effective state"
    );
    assert_eq!(ctx.selected_frame_scope(), None);
    assert_eq!(
        selected_branch_of_palace(&ctx, iztro::PalaceName::Life),
        None
    );
    assert!(
        selected_stars_in_san_fang_si_zheng(&ctx, EarthlyBranch::Chou, &[StarName::WenChang])
            .is_empty(),
        "selected-state helper fails closed when no effective state exists"
    );
}

// ---- selected-state vs source/layer query boundaries -----------------------

#[test]
fn selected_and_source_sfsz_helpers_disagree_on_natal_support_in_a_temporal_frame() {
    // Yearly frame relabels Chou as Life. Natal WenChang sits in that frame's
    // 三方四正 (at Chou itself), and no Yearly WenChang exists. The selected-state
    // helper sees the natal support star through the selected frame; the
    // source/layer helper, restricted to the Yearly layer, does not. This is the
    // exact distinction that PR #145 fixed.
    let natal = build_chart(
        EarthlyBranch::Zi,
        &[soft(EarthlyBranch::Chou, StarName::WenChang)],
    );
    let horoscope = horoscope_with_layer(natal, Scope::Yearly, EarthlyBranch::Chou, vec![], vec![]);
    let ctx = PatternContext::horoscope_with_frame(
        &horoscope,
        Scope::Yearly,
        vec![Scope::Natal, Scope::Yearly],
    );

    assert_eq!(ctx.selected_frame_scope(), Some(Scope::Yearly));
    assert_eq!(
        selected_branch_of_palace(&ctx, iztro::PalaceName::Life),
        Some(EarthlyBranch::Chou),
        "Yearly Life is Chou"
    );

    let selected =
        selected_stars_in_san_fang_si_zheng(&ctx, EarthlyBranch::Chou, &[StarName::WenChang]);
    assert!(
        selected
            .iter()
            .any(|(star, branch)| *star == StarName::WenChang && *branch == EarthlyBranch::Chou),
        "selected-state helper finds the natal support star in the selected frame"
    );

    let source = stars_in_san_fang_si_zheng_for_scope(
        &ctx,
        Scope::Yearly,
        EarthlyBranch::Chou,
        &[StarName::WenChang],
    );
    assert!(
        source.is_empty(),
        "source/layer helper sees only Yearly-layer facts, not natal support"
    );

    // The source_* alias is a pure rename of the *_for_scope helper.
    assert_eq!(
        source,
        source_stars_in_san_fang_si_zheng(
            &ctx,
            Scope::Yearly,
            EarthlyBranch::Chou,
            &[StarName::WenChang],
        )
    );
}
