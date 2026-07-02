//! Named 格局 (pattern) detectors.
//!
//! Each submodule is one named pattern and exposes `detect(ctx, request, out)`.
//! A detector makes its two conceptual layers explicit:
//!
//! 1. **成格 detection** (`detect_base_formation`): does the base formation exist?
//! 2. **破格 / 减力 assessment** (`assess_integrity`): once the base exists, is it
//!    fulfilled, weakened, or broken?
//!
//! [`detect_all`] owns the ordered list of detector calls. Ordering is kept stable
//! so downstream sorting and existing tests stay deterministic. Detectors emit
//! through [`emit::push_detection`], which resolves display name, family, and
//! polarity from the registry.

pub(crate) mod emit;

pub mod cai_yu_qiu_chou;
pub mod chang_qu_jia_ming;
pub mod fu_xiang_chao_yuan;
pub mod ji_xiang_li_ming;
pub mod ji_yue_tong_liang;
pub mod jin_can_guang_hui;
pub mod lu_feng_chong_po;
pub mod ma_luo_kong_wang;
pub mod ma_tou_dai_jian;
pub mod ming_li_feng_kong;
pub mod ming_wu_zheng_yao;
pub mod ming_zhu_chu_hai;
pub mod ri_chu_fu_sang;
pub mod ri_yue_bing_ming;
pub mod ri_yue_fan_bei;
pub mod tan_huo_xiang_feng;
pub mod tian_ji_si_hai;
pub mod wen_xing_gong_ming;
pub mod wu_qu_shou_yuan;
pub mod yang_tuo_jia_ji;
pub mod yue_luo_hai_gong;
pub mod yue_sheng_cang_hai;
pub mod zi_fu_chao_yuan;
pub mod zuo_you_jia_ming;
pub mod zuo_you_tong_gong;

use crate::core::Scope;

use super::context::{PatternContext, PatternDetectionRequest};
use super::model::{PatternDetection, PatternId};
use super::query::selected_frame_scope;

type DetectorFn = fn(&PatternContext<'_>, &PatternDetectionRequest, &mut Vec<PatternDetection>);

pub(crate) struct PatternDetectorRegistration {
    pub(crate) id: PatternId,
    pub(crate) detect: DetectorFn,
}

pub(crate) const DETECTORS: &[PatternDetectorRegistration] = &[
    PatternDetectorRegistration {
        id: PatternId::ZiFuChaoYuan,
        detect: zi_fu_chao_yuan::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::JiYueTongLiang,
        detect: ji_yue_tong_liang::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::YangTuoJiaJi,
        detect: yang_tuo_jia_ji::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::ZuoYouJiaMing,
        detect: zuo_you_jia_ming::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::ChangQuJiaMing,
        detect: chang_qu_jia_ming::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::RiYueBingMing,
        detect: ri_yue_bing_ming::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::RiYueFanBei,
        detect: ri_yue_fan_bei::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::JinCanGuangHui,
        detect: jin_can_guang_hui::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::YueLuoHaiGong,
        detect: yue_luo_hai_gong::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::YueShengCangHai,
        detect: yue_sheng_cang_hai::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::MaTouDaiJian,
        detect: ma_tou_dai_jian::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::TanHuoXiangFeng,
        detect: tan_huo_xiang_feng::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::WuQuShouYuan,
        detect: wu_qu_shou_yuan::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::CaiYuQiuChou,
        detect: cai_yu_qiu_chou::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::MaLuoKongWang,
        detect: ma_luo_kong_wang::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::RiChuFuSang,
        detect: ri_chu_fu_sang::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::ZuoYouTongGong,
        detect: zuo_you_tong_gong::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::MingLiFengKong,
        detect: ming_li_feng_kong::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::LuFengChongPo,
        detect: lu_feng_chong_po::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::WenXingGongMing,
        detect: wen_xing_gong_ming::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::TianJiSiHai,
        detect: tian_ji_si_hai::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::MingZhuChuHai,
        detect: ming_zhu_chu_hai::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::MingWuZhengYao,
        detect: ming_wu_zheng_yao::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::JiXiangLiMing,
        detect: ji_xiang_li_ming::detect,
    },
    PatternDetectorRegistration {
        id: PatternId::FuXiangChaoYuan,
        detect: fu_xiang_chao_yuan::detect,
    },
];

pub(crate) const DETECTORLESS_PATTERN_IDS: &[PatternId] = &[PatternId::LingChangTuoWu];

/// Runs every named detector in a stable order, appending detections to `out`.
///
/// The order below is the canonical detection order; deterministic output
/// ordering is finalized by the top-level `filter_and_sort`.
pub fn detect_all(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    for registration in DETECTORS {
        debug_assert!(
            !DETECTORLESS_PATTERN_IDS.contains(&registration.id),
            "detectorless pattern {:?} must not have a detector registration",
            registration.id,
        );
        (registration.detect)(ctx, request, out);
    }
}

/// Returns the selected frame scope when the request asks for it.
///
/// Selected-view detectors (the pattern-note family) evaluate a single selected
/// frame; this resolves that scope and gates it on the request's scope filter.
pub(crate) fn requested_selected_scope(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
) -> Option<Scope> {
    let scope = selected_frame_scope(ctx)?;
    request.scopes.contains(&scope).then_some(scope)
}

#[cfg(test)]
mod tests {
    use super::{DETECTORLESS_PATTERN_IDS, DETECTORS};
    use crate::rules::pattern::metadata::pattern_source_metadata;
    use crate::rules::pattern::model::PatternId;

    /// The source-backed `PatternId`s emitted by the QuanShu Volume 1 detectors.
    /// Each must resolve to source metadata, or emission would misrepresent an
    /// unbacked pattern as source-backed. (Inventory-reference correctness is
    /// checked separately in `tests/classical_source_inventory.rs`.)
    const SOURCE_BACKED_PATTERN_IDS: [PatternId; 8] = [
        PatternId::JinCanGuangHui,
        PatternId::YueLuoHaiGong,
        PatternId::YueShengCangHai,
        PatternId::MaTouDaiJian,
        PatternId::TanHuoXiangFeng,
        PatternId::WuQuShouYuan,
        PatternId::CaiYuQiuChou,
        PatternId::MaLuoKongWang,
    ];

    #[test]
    fn every_source_backed_pattern_has_source_metadata() {
        for id in SOURCE_BACKED_PATTERN_IDS {
            assert!(
                pattern_source_metadata(id).is_some(),
                "source-backed pattern {id:?} has no source metadata",
            );
        }
    }

    #[test]
    fn every_pattern_id_has_exactly_one_detector_or_is_allowlisted() {
        for id in PatternId::ALL {
            let detector_count = DETECTORS
                .iter()
                .filter(|registration| registration.id == id)
                .count();
            let allowlisted = DETECTORLESS_PATTERN_IDS.contains(&id);
            assert!(
                detector_count == 1 || allowlisted,
                "{id:?} must have exactly one detector or be explicitly detectorless",
            );
            assert!(
                detector_count <= 1,
                "{id:?} has {detector_count} detector registrations",
            );
            assert!(
                !(allowlisted && detector_count > 0),
                "{id:?} is allowlisted as detectorless but also has a detector",
            );
        }
    }

    #[test]
    fn detector_registration_order_is_stable() {
        let registered_ids: Vec<PatternId> = DETECTORS
            .iter()
            .map(|registration| registration.id)
            .collect();
        assert_eq!(
            registered_ids,
            vec![
                PatternId::ZiFuChaoYuan,
                PatternId::JiYueTongLiang,
                PatternId::YangTuoJiaJi,
                PatternId::ZuoYouJiaMing,
                PatternId::ChangQuJiaMing,
                PatternId::RiYueBingMing,
                PatternId::RiYueFanBei,
                PatternId::JinCanGuangHui,
                PatternId::YueLuoHaiGong,
                PatternId::YueShengCangHai,
                PatternId::MaTouDaiJian,
                PatternId::TanHuoXiangFeng,
                PatternId::WuQuShouYuan,
                PatternId::CaiYuQiuChou,
                PatternId::MaLuoKongWang,
                PatternId::RiChuFuSang,
                PatternId::ZuoYouTongGong,
                PatternId::MingLiFengKong,
                PatternId::LuFengChongPo,
                PatternId::WenXingGongMing,
                PatternId::TianJiSiHai,
                PatternId::MingZhuChuHai,
                PatternId::MingWuZhengYao,
                PatternId::JiXiangLiMing,
                PatternId::FuXiangChaoYuan,
            ],
        );
    }
}
