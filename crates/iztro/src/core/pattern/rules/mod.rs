//! Initial pattern detection rules.
//!
//! Each rule exposes `detect(ctx, request, out)` which appends any detections it
//! finds to `out`. Rules are intentionally conservative: they emit a detection
//! only when their structural conditions are clearly met by modeled chart facts.

pub mod chang_qu_jia_ming;
pub mod ji_yue_tong_liang;
pub mod quan_shu_v01;
pub mod ri_yue_bing_ming;
pub mod ri_yue_fan_bei;
pub mod yang_tuo_jia_ji;
pub mod zi_fu_chao_yuan;
pub mod zuo_you_jia_ming;
