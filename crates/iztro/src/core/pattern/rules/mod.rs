//! Initial pattern detection rules.
//!
//! Each rule exposes `detect(ctx, request, out)` which appends any detections it
//! finds to `out`. Rules are intentionally conservative: they emit a detection
//! only when their structural conditions are clearly met by modeled chart facts.

pub mod ji_yue_tong_liang;
pub mod yang_tuo_jia_ji;
pub mod zi_fu_chao_yuan;
