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
use super::model::PatternDetection;
use super::query::selected_frame_scope;

/// Runs every named detector in a stable order, appending detections to `out`.
///
/// The order below is the canonical detection order; deterministic output
/// ordering is finalized by the top-level `filter_and_sort`.
pub fn detect_all(
    ctx: &PatternContext<'_>,
    request: &PatternDetectionRequest,
    out: &mut Vec<PatternDetection>,
) {
    zi_fu_chao_yuan::detect(ctx, request, out);
    ji_yue_tong_liang::detect(ctx, request, out);
    yang_tuo_jia_ji::detect(ctx, request, out);
    zuo_you_jia_ming::detect(ctx, request, out);
    chang_qu_jia_ming::detect(ctx, request, out);
    ri_yue_bing_ming::detect(ctx, request, out);
    ri_yue_fan_bei::detect(ctx, request, out);

    // Source-backed QuanShu Volume 1 patterns (定富局 / 定贵局 / 定贫贱局).
    jin_can_guang_hui::detect(ctx, request, out);
    yue_luo_hai_gong::detect(ctx, request, out);
    yue_sheng_cang_hai::detect(ctx, request, out);
    ma_tou_dai_jian::detect(ctx, request, out);
    tan_huo_xiang_feng::detect(ctx, request, out);
    wu_qu_shou_yuan::detect(ctx, request, out);
    cai_yu_qiu_chou::detect(ctx, request, out);
    ma_luo_kong_wang::detect(ctx, request, out);

    // Normalized pattern-note detectors.
    ri_chu_fu_sang::detect(ctx, request, out);
    zuo_you_tong_gong::detect(ctx, request, out);
    ming_li_feng_kong::detect(ctx, request, out);
    lu_feng_chong_po::detect(ctx, request, out);
    wen_xing_gong_ming::detect(ctx, request, out);
    tian_ji_si_hai::detect(ctx, request, out);
    ming_zhu_chu_hai::detect(ctx, request, out);
    ming_wu_zheng_yao::detect(ctx, request, out);
    ji_xiang_li_ming::detect(ctx, request, out);
    fu_xiang_chao_yuan::detect(ctx, request, out);
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
}
