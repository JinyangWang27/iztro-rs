//! Assembly helpers for one selected yearly horoscope layer.
//!
//! This module composes deterministic 流年 facts into one model-only
//! [`TemporalLayer`]: the yearly period supplies the target year and palace
//! layout, the flow-star builder contributes scoped 流曜 placements, the yearly
//! mutagen builder contributes scoped 四化 activations, and the yearly decorative
//! builder contributes scoped `yearlyDecStar` (岁前/将前) facts. It does not mutate
//! natal facts, render prose, or assemble monthly/daily/hourly layers.

use crate::core::error::ChartError;
use crate::core::model::chart::{Chart, TemporalContext, TemporalLayer, YearlyPeriod};
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::overlay::flow::build_flow_star_layer;
use crate::core::placement::overlay::yearly::{
    YearlyMutagenLayerInput, build_yearly_mutagen_layer,
};
use crate::core::placement::overlay::yearly_decorative::build_yearly_decorative_star_placements;

/// Builds the composed yearly temporal layer for a selected yearly period.
pub fn build_yearly_horoscope_layer(
    natal: &Chart,
    period: &YearlyPeriod,
) -> Result<TemporalLayer, ChartError> {
    let context = TemporalContext::Yearly {
        stem_branch: period.stem_branch(),
        lunar_year: period.lunar_year(),
    };
    let flow_layer = build_flow_star_layer(context)?;
    let mutagen_layer = build_yearly_mutagen_layer(
        natal,
        YearlyMutagenLayerInput::new(period.stem_branch(), period.lunar_year()),
    )?;
    let decorative_placements = build_yearly_decorative_star_placements(
        period,
        natal.method_profile().algorithm_kind(),
    )?;

    TemporalLayer::try_new_with_palace_layout_and_decorative_stars(
        Scope::Yearly,
        context,
        flow_layer.placements().to_vec(),
        mutagen_layer.activations().to_vec(),
        Some(period.palace_layout().clone()),
        decorative_placements,
    )
}
