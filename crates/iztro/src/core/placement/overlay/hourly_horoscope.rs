//! Assembly helpers for one selected hourly horoscope layer.
//!
//! This module composes deterministic 流时 facts into one model-only
//! [`TemporalLayer`]: the hourly period supplies the target double-hour and
//! palace layout, the flow-star builder contributes scoped 流曜 placements, and
//! the shared Heavenly Stem mutagen helper contributes scoped 四化 activations.
//! It does not mutate natal facts, attach decorative arrays, render prose, or
//! assemble the full horoscope stack.

use crate::core::error::ChartError;
use crate::core::model::chart::{Chart, HourlyPeriod, TemporalContext, TemporalLayer};
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::overlay::flow::build_flow_star_layer;
use crate::core::placement::overlay::mutagen::stem_mutagen_activations;

/// Builds the composed hourly temporal layer for a selected hourly period.
pub fn build_hourly_horoscope_layer(
    natal: &Chart,
    period: &HourlyPeriod,
) -> Result<TemporalLayer, ChartError> {
    let context = TemporalContext::Hourly {
        stem_branch: period.stem_branch(),
    };
    let flow_layer = build_flow_star_layer(context)?;
    let activations = stem_mutagen_activations(natal, Scope::Hourly, period.stem_branch().stem());

    TemporalLayer::try_new_with_palace_layout(
        Scope::Hourly,
        context,
        flow_layer.placements().to_vec(),
        activations,
        Some(period.palace_layout().clone()),
    )
}
