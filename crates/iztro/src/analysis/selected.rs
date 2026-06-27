//! Selected-view batch analysis facade.
//!
//! [`detect_static_temporal_analysis_layers_from_chart`] is the result-oriented
//! entry point an app calls to fill its per-layer analysis cache. The app already
//! holds a natal [`Chart`], the current
//! [`StaticTemporalNavigationSelection`], and the list of
//! [`AnalysisLayerKey`]s it is still missing (see
//! [`analysis_layers_for_selection`]);
//! it passes those in and receives one [`AnalysisLayerResult`] per requested key.
//!
//! Core owns the temporal context: it builds the selected horoscope overlay stack
//! **once** and detects each requested layer against it. The app never constructs
//! a [`HoroscopeChart`](crate::core::HoroscopeChart) or a borrowed
//! [`TemporalAnalysisContext`] itself — it stays a cache/render layer.
//!
//! # Per-key scope truncation
//!
//! The selected view fixes the deepest visible scope, but each requested layer is
//! detected with **its own** active-scope chain
//! ([`analysis_scopes_for_layer_key`](crate::analysis::analysis_scopes_for_layer_key)),
//! not the deepest chain. Detecting a 流年 layer inside a selected 流月 view sees
//! only `Natal..=Yearly`, never 流月, so a cached 流年 result stays stable when the
//! selected 流月 / 流日 / 流时 changes.

use crate::core::facade::static_temporal_chart_view::{
    SelectedTemporalChart, build_selected_temporal_chart,
};
use crate::core::{Chart, ChartError, StaticTemporalNavigationSelection};

use crate::analysis::detect::{
    AnalysisLayerRequest, AnalysisLayerResult, TemporalAnalysisContext, detect_analysis_layer,
};
use crate::analysis::layer::{AnalysisLayerKey, analysis_layers_for_selection};

/// Detects the requested analysis layers for one temporal navigation selection.
///
/// This is the selected-view batch facade: it builds the temporal context for
/// `selection` once and detects exactly the requested `keys`, returning one
/// [`AnalysisLayerResult`] per key in input order.
///
/// # Semantics
///
/// - An empty `keys` slice returns `Ok(Vec::new())` without building any context.
/// - Every requested key must be **exactly** visible under `selection`: it must
///   match (scope and all temporal indexes) one of the layers
///   [`analysis_layers_for_selection`] expands `selection` into. A key for a
///   sibling index, a descendant scope, or a mismatched ancestor index is
///   rejected with [`ChartError::AnalysisLayerNotVisibleForSelection`].
/// - The selected horoscope overlay stack is built once for the whole call.
/// - Each requested layer is detected with its own truncated active-scope chain,
///   so a deeper selection never leaks descendant overlays into an ancestor
///   layer's result.
/// - Only the requested keys are returned; ancestor layers are **not** requested
///   automatically. The app drives ancestor caching through
///   [`missing_analysis_layers`](crate::analysis::analysis_layers_for_selection)
///   planning.
///
/// The natal `Chart` is taken by value because the overlay-building path consumes
/// it; an app caching a natal chart passes `natal.clone()`.
pub fn detect_static_temporal_analysis_layers_from_chart(
    natal: Chart,
    selection: StaticTemporalNavigationSelection,
    keys: &[AnalysisLayerKey],
    request: &AnalysisLayerRequest,
) -> Result<Vec<AnalysisLayerResult>, ChartError> {
    if keys.is_empty() {
        return Ok(Vec::new());
    }

    // Validate every requested key against the layers this selection makes
    // visible, comparing exact indexes (not just scope kind).
    let visible = analysis_layers_for_selection(selection);
    for key in keys {
        if !visible.contains(key) {
            return Err(ChartError::AnalysisLayerNotVisibleForSelection { scope: key.scope() });
        }
    }

    // Build the selected temporal context once; reuse it for every requested key.
    let selected = build_selected_temporal_chart(natal, selection)?;
    let ctx = match &selected {
        SelectedTemporalChart::Natal(natal) => TemporalAnalysisContext::natal(natal),
        SelectedTemporalChart::Horoscope(horoscope) => {
            TemporalAnalysisContext::horoscope(horoscope)
        }
    };

    Ok(keys
        .iter()
        .map(|key| detect_analysis_layer(&ctx, key.clone(), request))
        .collect())
}
