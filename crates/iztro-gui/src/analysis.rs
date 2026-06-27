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
use iztro::core::{EarthlyBranch, Mutagen, PatternDetection, PatternId, StarName};
use iztro::rules::classical::{ClassicalRuleHitRef, ClassicalRuleId, Evidence, EvidenceKind};

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

/// Identifies the currently selected analysis hit driving chart highlighting.
///
/// Reuses the existing expansion keys so click identity, expansion identity, and
/// selection identity stay aligned. Highlight state derived from this lives only
/// in the GUI: core never owns a highlight model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActiveAnalysisSelection {
    /// A classical-rule hit is selected.
    Rule(RuleHitExpansionKey),
    /// A pattern detection is selected.
    Pattern(PatternHitExpansionKey),
}

impl ActiveAnalysisSelection {
    /// The analysis layer the selected hit belongs to.
    pub fn layer(&self) -> &AnalysisLayerKey {
        match self {
            ActiveAnalysisSelection::Rule(key) => &key.layer,
            ActiveAnalysisSelection::Pattern(key) => &key.layer,
        }
    }
}

/// Chart areas that a selected analysis hit asks the renderer to highlight.
///
/// This is a renderer-local read model derived from cached structured evidence.
/// It is never exported from core: highlighting is a UI concern, the analysis
/// result is the only structured fact core owns.
///
/// The fields are deduplicated `Vec`s rather than `BTreeSet`s because several
/// of the contained core enums (e.g. [`EarthlyBranch`]) intentionally do not
/// implement `Ord`. Insertion order is preserved by [`add_branch`] etc., which
/// also drops duplicates so equality of two views stays deterministic.
///
/// [`add_branch`]: Self::add_branch
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ChartHighlightView {
    /// Palace branches to emphasize.
    pub palace_branches: Vec<EarthlyBranch>,
    /// Star names to emphasize.
    pub star_names: Vec<StarName>,
    /// Mutagens (四化) to emphasize.
    pub mutagens: Vec<Mutagen>,
}

impl ChartHighlightView {
    /// Whether the view carries no highlight at all.
    pub fn is_empty(&self) -> bool {
        self.palace_branches.is_empty() && self.star_names.is_empty() && self.mutagens.is_empty()
    }

    /// Whether the view emphasizes `branch`.
    pub fn highlights_palace(&self, branch: EarthlyBranch) -> bool {
        self.palace_branches.contains(&branch)
    }

    fn add_branch(&mut self, branch: EarthlyBranch) {
        if !self.palace_branches.contains(&branch) {
            self.palace_branches.push(branch);
        }
    }

    fn add_star(&mut self, star: StarName) {
        if !self.star_names.contains(&star) {
            self.star_names.push(star);
        }
    }

    fn add_mutagen(&mut self, mutagen: Mutagen) {
        if !self.mutagens.contains(&mutagen) {
            self.mutagens.push(mutagen);
        }
    }
}

/// Projects a [`PatternDetection`] into a chart highlight by reading its
/// already-structured `involved_*` fields. The GUI invents no semantics: an
/// empty field stays empty in the projection.
pub(crate) fn highlight_for_pattern_detection(detection: &PatternDetection) -> ChartHighlightView {
    let mut view = ChartHighlightView::default();
    for branch in &detection.involved_palaces {
        view.add_branch(*branch);
    }
    for star in &detection.involved_stars {
        view.add_star(*star);
    }
    for mutagen in &detection.involved_mutagens {
        view.add_mutagen(*mutagen);
    }
    view
}

/// Projects a [`ClassicalRuleHitRef`] into a chart highlight by reading its
/// structured [`Evidence`] only when the variant unambiguously names a palace
/// branch, star, or mutagen target.
///
/// The projection is intentionally conservative: variants whose targets cannot
/// be mapped safely (e.g. [`EvidenceKind::PalaceRelation`] without an explicit
/// star/mutagen, or unsupported-condition placeholders) contribute nothing
/// rather than fabricating semantics.
pub(crate) fn highlight_for_rule_hit(hit: &ClassicalRuleHitRef) -> ChartHighlightView {
    let mut view = ChartHighlightView::default();
    for evidence in &hit.evidence {
        accumulate_evidence(&mut view, evidence);
    }
    view
}

fn accumulate_evidence(view: &mut ChartHighlightView, evidence: &Evidence) {
    match evidence.kind() {
        EvidenceKind::StarInPalace { star, branch } => {
            view.add_star(*star);
            view.add_branch(*branch);
        }
        EvidenceKind::StarClampsPalace {
            star,
            clamp_branch,
            target_branch,
        } => {
            view.add_star(*star);
            view.add_branch(*clamp_branch);
            view.add_branch(*target_branch);
        }
        EvidenceKind::StarAffectedByVoid { star, branch, .. } => {
            view.add_star(*star);
            view.add_branch(*branch);
        }
        EvidenceKind::MutagenInPalace {
            star,
            mutagen,
            branch,
        } => {
            view.add_star(*star);
            view.add_mutagen(*mutagen);
            view.add_branch(*branch);
        }
        EvidenceKind::BrightnessCondition { star, branch, .. } => {
            view.add_star(*star);
            view.add_branch(*branch);
        }
        // A bare relation between palaces names branches; we surface them so
        // the relevant palaces are emphasized, but we do not fabricate a star
        // or mutagen for the connecting line.
        EvidenceKind::PalaceRelation { from, to, .. } => {
            view.add_branch(*from);
            view.add_branch(*to);
        }
        // The pattern-shape match and unsupported-condition variants don't carry
        // a concrete palace/star/mutagen target; leave them out conservatively.
        EvidenceKind::PatternShapeMatched { .. } | EvidenceKind::UnsupportedCondition { .. } => {}
    }
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

    #[test]
    fn pattern_highlight_projects_involved_branches_stars_and_mutagens() {
        let detection = PatternDetection {
            id: PatternId::ZiFuChaoYuan,
            name_zh: "紫府朝垣",
            family: iztro::core::PatternFamily::MajorStarCombination,
            polarity: iztro::core::PatternPolarity::Auspicious,
            status: iztro::core::PatternStatus::Fulfilled,
            strength: iztro::core::PatternStrength::Strong,
            scope: iztro::core::PatternScope::Natal,
            anchor: iztro::core::PatternAnchor::Chart,
            involved_palaces: vec![EarthlyBranch::Yin, EarthlyBranch::Shen],
            involved_stars: vec![StarName::ZiWei, StarName::TianFu],
            involved_mutagens: vec![Mutagen::Lu],
            evidence: Vec::new(),
            missing_conditions: Vec::new(),
            weakening_factors: Vec::new(),
            breaking_factors: Vec::new(),
        };
        let view = highlight_for_pattern_detection(&detection);
        assert!(view.highlights_palace(EarthlyBranch::Yin));
        assert!(view.highlights_palace(EarthlyBranch::Shen));
        assert!(view.star_names.contains(&StarName::ZiWei));
        assert!(view.star_names.contains(&StarName::TianFu));
        assert!(view.mutagens.contains(&Mutagen::Lu));
    }

    #[test]
    fn pattern_highlight_with_no_involved_fields_is_empty() {
        let detection = PatternDetection {
            id: PatternId::ZiFuChaoYuan,
            name_zh: "紫府朝垣",
            family: iztro::core::PatternFamily::MajorStarCombination,
            polarity: iztro::core::PatternPolarity::Auspicious,
            status: iztro::core::PatternStatus::Partial,
            strength: iztro::core::PatternStrength::Weak,
            scope: iztro::core::PatternScope::Natal,
            anchor: iztro::core::PatternAnchor::Chart,
            involved_palaces: Vec::new(),
            involved_stars: Vec::new(),
            involved_mutagens: Vec::new(),
            evidence: Vec::new(),
            missing_conditions: Vec::new(),
            weakening_factors: Vec::new(),
            breaking_factors: Vec::new(),
        };
        assert!(highlight_for_pattern_detection(&detection).is_empty());
    }

    fn rule_hit(evidence: Vec<Evidence>) -> ClassicalRuleHitRef {
        ClassicalRuleHitRef {
            rule_id: ClassicalRuleId::new("test.rule"),
            scope: iztro::rules::classical::ClaimScope::Natal,
            claim_key: None,
            evidence,
        }
    }

    #[test]
    fn rule_highlight_reads_star_in_palace_evidence() {
        let hit = rule_hit(vec![Evidence::new(EvidenceKind::StarInPalace {
            star: StarName::ZiWei,
            branch: EarthlyBranch::Wu,
        })]);
        let view = highlight_for_rule_hit(&hit);
        assert!(view.star_names.contains(&StarName::ZiWei));
        assert!(view.highlights_palace(EarthlyBranch::Wu));
        assert!(view.mutagens.is_empty());
    }

    #[test]
    fn rule_highlight_reads_mutagen_in_palace_evidence() {
        let hit = rule_hit(vec![Evidence::new(EvidenceKind::MutagenInPalace {
            star: StarName::TianJi,
            mutagen: Mutagen::Quan,
            branch: EarthlyBranch::Chen,
        })]);
        let view = highlight_for_rule_hit(&hit);
        assert!(view.star_names.contains(&StarName::TianJi));
        assert!(view.mutagens.contains(&Mutagen::Quan));
        assert!(view.highlights_palace(EarthlyBranch::Chen));
    }

    #[test]
    fn rule_highlight_is_empty_for_unsupported_or_pattern_only_evidence() {
        let hit = rule_hit(vec![
            Evidence::new(EvidenceKind::PatternShapeMatched {
                pattern: PatternId::ZiFuChaoYuan,
            }),
            Evidence::new(EvidenceKind::UnsupportedCondition {
                reason: iztro::rules::classical::UnsupportedReason::LuMaRelationNotModeled,
            }),
        ]);
        assert!(highlight_for_rule_hit(&hit).is_empty());
    }
}
