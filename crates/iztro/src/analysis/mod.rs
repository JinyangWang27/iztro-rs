//! Lightweight, layer-level analysis coordination.
//!
//! This module is a thin coordinator over two existing engines:
//!
//! - [`rules::pattern`](crate::rules::pattern) — pattern (格局) detection;
//! - [`rules::classical`](crate::rules::classical) — classical rule (全书规则)
//!   evaluation.
//!
//! It lives **outside** `core` on purpose: `core` must not depend on `rules`, but
//! a layer-level analysis API needs both. Keeping the coordinator here preserves
//! the layer boundaries while giving an app a single place to request analysis.
//!
//! # What this module is for
//!
//! A future GUI wants two right-sidebar tabs — 全书规则 and 格局 — whose results
//! are grouped by temporal scope (本命 / 大限 / 小限 / 流年 / 流月 / 流日 / 流时).
//! Rather than eagerly computing every overlay and shipping a heavy grouped-text
//! payload, this module exposes a **compact, per-layer** detection API the app can
//! call lazily and cache:
//!
//! - [`AnalysisLayerKey`] identifies one cacheable layer;
//! - [`analysis_layers_for_selection`] expands a navigation selection into the
//!   layers it makes visible;
//! - [`detect_analysis_layer`] analyzes exactly one layer and returns compact
//!   [`AnalysisLayerResult`]s.
//!
//! Source text is **not** duplicated into hits:
//! [`ClassicalRuleHitRef`](crate::rules::classical::ClassicalRuleHitRef) carries a
//! `rule_id`, and the GUI resolves verbatim source text once per rule through
//! [`classical_rule_metadata`](crate::rules::classical::classical_rule_metadata).
//!
//! # Cross-layer interaction policy
//!
//! 斗数 analysis can involve cross-layer interactions (e.g. 流年化忌冲照本命命宫).
//! This module does not add such rules, but it fixes the policy future rules must
//! follow: **a cross-layer hit is assigned to the deepest triggering layer.**
//!
//! | Interaction              | Assigned layer |
//! |--------------------------|----------------|
//! | 本命 + 流年              | Yearly (流年)  |
//! | 大限 + 流年              | Yearly (流年)  |
//! | 流年 + 流月              | Monthly (流月) |
//! | 流月 + 流日              | Daily (流日)   |
//!
//! Detection of a layer may *inspect* ancestor overlays, but the result is always
//! keyed to the deepest layer. This makes caching natural: changing month/day/hour
//! within the same year never invalidates the cached yearly result, and changing
//! day/hour within the same month never invalidates the cached monthly result.
//!
//! # Expected app caching model
//!
//! ```ignore
//! let required = analysis_layers_for_selection(selection);
//! for key in required {
//!     if !analysis_cache.contains_key(&key) {
//!         let result = detect_analysis_layer(&ctx, key.clone(), &request);
//!         analysis_cache.insert(key, result);
//!     }
//! }
//! ```
//!
//! The GUI then keeps separate 全书规则 / 格局 tabs and groups cached results by
//! [`AnalysisLayerKey::scope`], hiding empty groups. No GUI rendering lives here.

pub mod detect;
pub mod layer;
pub mod selected;

pub use detect::{
    AnalysisLayerRequest, AnalysisLayerResult, TemporalAnalysisContext, detect_analysis_layer,
};
pub use layer::{AnalysisLayerKey, analysis_layers_for_selection, analysis_scopes_for_layer_key};
pub use selected::detect_static_temporal_analysis_layers_from_chart;
