//! GUI-side cache and planning helpers for the layer-level analysis API.
//!
//! Analysis in core is **layer-level**: an app expands the current temporal
//! selection into the [`AnalysisLayerKey`]s it makes visible
//! ([`analysis_layers_for_selection`]), requests only the layers it is missing
//! ([`detect_analysis_layer`]), and caches each result by its key. Ancestor
//! layers (本命 / 大限 / …) are therefore never recomputed when a deeper temporal
//! overlay changes: moving from one 流月 to another under the same 流年 reuses the
//! cached 本命/大限/小限/流年 results and requests only the new 流月 layer.
//!
//! This module owns the renderer-agnostic cache and the pure planning helper so
//! both can be unit-tested without touching Iced. Analysis results are **never**
//! persisted: the cache is in-memory only and is cleared whenever a new chart is
//! generated.
//!
//! [`analysis_layers_for_selection`]: iztro::analysis::analysis_layers_for_selection
//! [`detect_analysis_layer`]: iztro::analysis::detect_analysis_layer

use std::collections::BTreeMap;

use iztro::analysis::{AnalysisLayerKey, AnalysisLayerResult};
use iztro::core::PatternId;
use iztro::rules::classical::ClassicalRuleId;

/// In-memory, per-layer cache of [`AnalysisLayerResult`]s.
///
/// Keyed by [`AnalysisLayerKey`] so a result is shared across every temporal
/// selection that makes that layer visible. This caches structured analysis
/// values only; it never caches rendered widgets and is not persisted to disk.
#[derive(Clone, Debug, Default)]
pub struct AnalysisCache {
    layers: BTreeMap<AnalysisLayerKey, AnalysisLayerResult>,
}

impl AnalysisCache {
    /// Whether a result for `key` is cached.
    pub fn contains(&self, key: &AnalysisLayerKey) -> bool {
        self.layers.contains_key(key)
    }

    /// The cached result for `key`, if any.
    pub fn get(&self, key: &AnalysisLayerKey) -> Option<&AnalysisLayerResult> {
        self.layers.get(key)
    }

    /// Inserts (or replaces) the cached result for `key`.
    pub fn insert(&mut self, key: AnalysisLayerKey, result: AnalysisLayerResult) {
        self.layers.insert(key, result);
    }

    /// Drops every cached layer. Called when a new chart is generated, since
    /// analysis results belong to one chart's facts.
    pub fn clear(&mut self) {
        self.layers.clear();
    }

    /// Number of cached layers.
    pub fn len(&self) -> usize {
        self.layers.len()
    }

    /// Whether the cache holds no layers.
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }
}

/// The layers in `required` not yet present in `cache`, preserving `required`
/// order (natal-outward).
///
/// This is the planning step of the caching model: the app asks for the layers a
/// selection makes visible, then requests detection only for the ones still
/// missing. Cached ancestor layers are skipped, so changing a deep overlay never
/// re-requests an unchanged ancestor.
pub fn missing_analysis_layers(
    required: &[AnalysisLayerKey],
    cache: &AnalysisCache,
) -> Vec<AnalysisLayerKey> {
    required
        .iter()
        .filter(|key| !cache.contains(key))
        .cloned()
        .collect()
}

/// Identifies one expandable 全书规则 line for click-to-expand state.
///
/// The pair `(layer, rule_id)` is unique within the inspector: the same rule may
/// appear under multiple layers, and each occurrence expands independently.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RuleHitExpansionKey {
    /// The layer the hit is grouped under.
    pub layer: AnalysisLayerKey,
    /// The matched rule.
    pub rule_id: ClassicalRuleId,
}

/// Identifies one expandable 格局 line for click-to-expand state.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PatternHitExpansionKey {
    /// The layer the detection is grouped under.
    pub layer: AnalysisLayerKey,
    /// The detected pattern.
    pub pattern_id: PatternId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use iztro::analysis::AnalysisLayerResult;

    fn empty_result(key: AnalysisLayerKey) -> AnalysisLayerResult {
        AnalysisLayerResult {
            key,
            rule_hits: Vec::new(),
            pattern_hits: Vec::new(),
        }
    }

    #[test]
    fn missing_layers_of_an_empty_cache_is_the_full_requirement() {
        let cache = AnalysisCache::default();
        let required = vec![
            AnalysisLayerKey::Natal,
            AnalysisLayerKey::Decadal { decadal_index: 2 },
        ];
        assert_eq!(missing_analysis_layers(&required, &cache), required);
    }

    #[test]
    fn cached_layers_are_skipped_and_order_is_preserved() {
        let mut cache = AnalysisCache::default();
        cache.insert(
            AnalysisLayerKey::Natal,
            empty_result(AnalysisLayerKey::Natal),
        );
        let required = vec![
            AnalysisLayerKey::Natal,
            AnalysisLayerKey::Decadal { decadal_index: 2 },
        ];
        // Natal already cached, so only the decadal layer is missing.
        assert_eq!(
            missing_analysis_layers(&required, &cache),
            vec![AnalysisLayerKey::Decadal { decadal_index: 2 }]
        );
    }

    #[test]
    fn clearing_drops_every_cached_layer() {
        let mut cache = AnalysisCache::default();
        cache.insert(
            AnalysisLayerKey::Natal,
            empty_result(AnalysisLayerKey::Natal),
        );
        assert_eq!(cache.len(), 1);
        cache.clear();
        assert!(cache.is_empty());
    }
}
