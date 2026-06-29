//! Integration tests for the pattern (格局) detection layer.
//!
//! These tests build small synthetic charts with full control over star
//! placement so detection conditions are exercised deterministically. Structured
//! fields are asserted; incidental `Vec` ordering is compared as a set.

use std::collections::BTreeSet;

use iztro::core::pattern::query::{find_star_branch, palace_has_star, stars_in_san_fang_si_zheng};
use iztro::{
    BirthContext, Brightness, CalendarDate, Chart, EarthlyBranch, Gender, HeavenlyStem,
    MethodProfile, Mutagen, PALACE_NAMES, Palace, PatternAnchor, PatternContext,
    PatternDetectionRequest, PatternFamily, PatternId, PatternPolarity, PatternStatus,
    PatternStrength, Scope, StarKind, StarName, StarPlacement, StemBranch,
};
use iztro::{PalaceRelation, PatternEvidence};
use iztro::{PatternSourceGroup, pattern_source_metadata};

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
        BirthContext::new(
            CalendarDate::solar(1990, 5, 17),
            EarthlyBranch::Chen,
            Gender::Female,
        ),
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
    assert!(detection.missing_conditions.is_empty());
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
        assert_eq!(metadata.work, "zi_wei_dou_shu_quan_shu");
    }
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
fn ri_chu_fu_sang_matches_life_or_career_at_mao() {
    let life = build_chart(
        EarthlyBranch::Mao,
        &[major(EarthlyBranch::Mao, StarName::TaiYang)],
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
        &[StarName::TaiYang],
        &[EarthlyBranch::Mao],
    );

    let career_life_branch = EarthlyBranch::Mao.offset(-8);
    let career = build_chart(
        career_life_branch,
        &[major(EarthlyBranch::Mao, StarName::TaiYang)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&career),
        &PatternDetectionRequest::default(),
    );
    assert!(detections.iter().any(|d| d.id == PatternId::RiChuFuSang));

    let neither = build_chart(
        EarthlyBranch::Zi,
        &[major(EarthlyBranch::Mao, StarName::TaiYang)],
    );
    let detections = iztro::detect_patterns(
        &PatternContext::natal(&neither),
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
        PatternPolarity::Mixed,
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
