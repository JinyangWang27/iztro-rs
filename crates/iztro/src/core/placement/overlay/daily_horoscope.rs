//! Assembly helpers for one selected daily horoscope layer.
//!
//! This module composes deterministic 流日 facts into one model-only
//! [`TemporalLayer`]: the daily period supplies the target day and palace
//! layout, the flow-star builder contributes scoped 流曜 placements, and the
//! shared Heavenly Stem mutagen helper contributes scoped 四化 activations. It
//! does not mutate natal facts, attach decorative arrays, render prose, or
//! assemble hourly layers.

use crate::core::error::ChartError;
use crate::core::model::chart::{Chart, DailyPeriod, TemporalContext, TemporalLayer};
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::overlay::flow::build_flow_star_layer;
use crate::core::placement::overlay::mutagen::stem_mutagen_activations;

/// Builds the composed daily temporal layer for a selected daily period.
pub fn build_daily_horoscope_layer(
    natal: &Chart,
    period: &DailyPeriod,
) -> Result<TemporalLayer, ChartError> {
    let context = TemporalContext::Daily {
        stem_branch: period.stem_branch(),
        lunar_day: period.lunar_day(),
    };
    let flow_layer = build_flow_star_layer(context)?;
    let activations = stem_mutagen_activations(natal, Scope::Daily, period.stem_branch().stem());

    TemporalLayer::try_new_with_palace_layout(
        Scope::Daily,
        context,
        flow_layer.placements().to_vec(),
        activations,
        Some(period.palace_layout().clone()),
    )
}
