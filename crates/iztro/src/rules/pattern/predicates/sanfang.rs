//! Shared 三方四正 (three-sides-four-directions) discovery helpers.
//!
//! Thin re-exports over the canonical 三方四正 set and membership helpers plus the
//! scoped/selected star-search wrappers, so named detectors share one source for
//! 三方四正 mechanics.

pub(crate) use crate::rules::pattern::query::{
    effective_stars_in_san_fang_si_zheng, selected_stars_in_san_fang_si_zheng,
    stars_in_san_fang_si_zheng_for_scope,
};
pub(crate) use crate::rules::pattern::relation::{is_in_san_fang_si_zheng, san_fang_si_zheng};
