//! Integration tests for the pattern (格局) detection layer.
//!
//! These tests build small synthetic charts with full control over star
//! placement so detection conditions are exercised deterministically. Structured
//! fields are asserted; incidental `Vec` ordering is compared as a set.

use std::collections::BTreeSet;

use iztro::core::pattern::query::{find_star_branch, palace_has_star, stars_in_san_fang_si_zheng};
use iztro::{
    BirthContext, Brightness, CalendarDate, Chart, EarthlyBranch, Gender, HeavenlyStem,
    MethodProfile, Mutagen, PALACE_NAMES, Palace, PatternContext, PatternDetectionRequest,
    PatternFamily, PatternId, PatternStatus, Scope, StarKind, StarName, StarPlacement, StemBranch,
};

/// One synthetic star placement: (branch, star, kind, optional mutagen).
type Spec = (EarthlyBranch, StarName, StarKind, Option<Mutagen>);

/// Builds a 12-palace natal chart with the Life palace at `life_branch` and the
/// given star placements. Palace branches are assigned distinctly around the
/// cycle so every branch is present.
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
        MethodProfile::placeholder("pattern_test"),
        palaces,
        None,
        None,
    )
    .expect("synthetic chart should build")
}

fn major(branch: EarthlyBranch, star: StarName) -> Spec {
    (branch, star, StarKind::Major, None)
}

fn branch_set(branches: &[EarthlyBranch]) -> BTreeSet<usize> {
    branches.iter().map(|branch| branch.index()).collect()
}

fn star_set(stars: &[StarName]) -> BTreeSet<StarName> {
    stars.iter().copied().collect()
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
fn ji_yue_tong_liang_negative_without_partial() {
    // Only two of four present; default request excludes partials.
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
fn ji_yue_tong_liang_partial_when_requested() {
    let chart = build_chart(
        EarthlyBranch::Zi,
        &[
            major(EarthlyBranch::Zi, StarName::TianJi),
            major(EarthlyBranch::Wu, StarName::TaiYin),
        ],
    );
    let request = PatternDetectionRequest {
        include_partial: true,
        ..PatternDetectionRequest::default()
    };
    let detections = iztro::detect_patterns(&PatternContext::natal(&chart), &request);
    let detection = detections
        .iter()
        .find(|d| d.id == PatternId::JiYueTongLiang)
        .expect("expected partial 机月同梁");
    assert_eq!(detection.status, PatternStatus::Partial);
    assert_eq!(
        star_set(&detection.involved_stars),
        star_set(&[StarName::TianJi, StarName::TaiYin])
    );
    // The two absent stars are recorded as missing conditions.
    assert_eq!(detection.missing_conditions.len(), 2);
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

// ---- 机月同梁 partial guard ---------------------------------------------

#[test]
fn ji_yue_tong_liang_partial_requires_two_stars() {
    let request = PatternDetectionRequest {
        include_partial: true,
        ..PatternDetectionRequest::default()
    };

    // Zero required stars present: no detection even with partials enabled.
    let none = build_chart(
        EarthlyBranch::Zi,
        &[major(EarthlyBranch::Zi, StarName::ZiWei)],
    );
    let detections = iztro::detect_patterns(&PatternContext::natal(&none), &request);
    assert!(detections.iter().all(|d| d.id != PatternId::JiYueTongLiang));

    // Only one required star present: still no partial detection.
    let one = build_chart(
        EarthlyBranch::Zi,
        &[major(EarthlyBranch::Zi, StarName::TianJi)],
    );
    let detections = iztro::detect_patterns(&PatternContext::natal(&one), &request);
    assert!(detections.iter().all(|d| d.id != PatternId::JiYueTongLiang));
}
