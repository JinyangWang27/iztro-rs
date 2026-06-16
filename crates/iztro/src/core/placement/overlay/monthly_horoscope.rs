//! Assembly helpers for one selected monthly horoscope layer.
//!
//! This module composes deterministic 流月 facts into one model-only
//! [`TemporalLayer`]: the monthly period supplies the target month and palace
//! layout, the flow-star builder contributes scoped 流曜 placements, and the
//! shared Heavenly Stem mutagen helper contributes scoped 四化 activations. It
//! does not mutate natal facts, attach decorative arrays, render prose, or
//! assemble daily/hourly layers.

use crate::core::error::ChartError;
use crate::core::model::chart::{Chart, MonthlyPeriod, TemporalContext, TemporalLayer};
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::overlay::flow::build_flow_star_layer;
use crate::core::placement::overlay::mutagen::stem_mutagen_activations;

/// Builds the composed monthly temporal layer for a selected monthly period.
pub fn build_monthly_horoscope_layer(
    natal: &Chart,
    period: &MonthlyPeriod,
) -> Result<TemporalLayer, ChartError> {
    let context = TemporalContext::Monthly {
        stem_branch: period.stem_branch(),
        lunar_month: period.lunar_month(),
    };
    let flow_layer = build_flow_star_layer(context)?;
    let activations = stem_mutagen_activations(natal, Scope::Monthly, period.stem_branch().stem());

    TemporalLayer::try_new_with_palace_layout(
        Scope::Monthly,
        context,
        flow_layer.placements().to_vec(),
        activations,
        Some(period.palace_layout().clone()),
    )
}
