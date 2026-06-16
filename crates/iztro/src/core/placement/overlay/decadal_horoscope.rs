//! Assembly helpers for one selected decadal horoscope layer.
//!
//! This module composes already-deterministic 大限 facts into one model-only
//! [`TemporalLayer`]: the decadal frame chooses the period, the flow-star
//! builder contributes scoped 流曜 placements, the decadal mutagen builder
//! contributes scoped 四化 activations, and the period's palace ring contributes
//! the decadal temporal palace-name layout. It does not mutate natal facts,
//! render prose, or assemble yearly/monthly/daily/hourly layers.

use crate::core::error::ChartError;
use crate::core::model::chart::{
    Chart, DecadalPeriod, HoroscopeChart, TemporalContext, TemporalLayer, TemporalPalaceLayout,
    TemporalPalaceName, build_decadal_frame,
};
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::overlay::decadal::{
    DecadalMutagenLayerInput, build_decadal_mutagen_layer,
};
use crate::core::placement::overlay::flow::build_flow_star_layer;

/// Selects one zero-based decadal period from a natal chart's derived frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct DecadalHoroscopeInput {
    /// Zero-based index into [`DecadalFrame::periods`].
    ///
    /// [`DecadalFrame::periods`]: crate::core::model::chart::DecadalFrame::periods
    pub period_index: usize,
}

/// Builds the composed decadal temporal layer for a selected decadal period.
pub fn build_decadal_horoscope_layer(
    natal: &Chart,
    period: &DecadalPeriod,
) -> Result<TemporalLayer, ChartError> {
    let context = TemporalContext::Decadal {
        stem_branch: period.stem_branch(),
        start_age: period.start_age(),
    };
    let flow_layer = build_flow_star_layer(context)?;
    let mutagen_layer = build_decadal_mutagen_layer(
        natal,
        DecadalMutagenLayerInput::new(period.stem_branch(), period.start_age()),
    )?;
    let palace_layout = build_decadal_palace_layout(natal, period)?;

    TemporalLayer::try_new_with_palace_layout(
        Scope::Decadal,
        context,
        flow_layer.placements().to_vec(),
        mutagen_layer.activations().to_vec(),
        Some(palace_layout),
    )
}

/// Derives the decadal temporal palace-name layout for a selected period.
///
/// A 大限 relabels the twelve palaces so the period's own natal palace becomes
/// 命 (Life), shifting every other branch by the same offset around the ring.
/// The natal palace name at each branch is shifted by minus the period palace's
/// index, keeping the names keyed by their stable [`EarthlyBranch`].
///
/// [`EarthlyBranch`]: lunar_lite::EarthlyBranch
fn build_decadal_palace_layout(
    natal: &Chart,
    period: &DecadalPeriod,
) -> Result<TemporalPalaceLayout, ChartError> {
    let shift = period.palace_name().index() as isize;
    let names = natal
        .palaces()
        .iter()
        .map(|palace| TemporalPalaceName::new(palace.branch(), palace.name().offset(-shift)))
        .collect();

    TemporalPalaceLayout::try_new(Scope::Decadal, names)
}

/// Builds a horoscope chart with exactly one decadal overlay layer.
pub fn build_decadal_horoscope_chart(
    natal: Chart,
    input: DecadalHoroscopeInput,
) -> Result<HoroscopeChart, ChartError> {
    let frame = build_decadal_frame(&natal)?;
    let period =
        frame
            .periods()
            .get(input.period_index)
            .ok_or(ChartError::InvalidDecadalPeriodIndex {
                index: input.period_index,
                len: frame.periods().len(),
            })?;
    let layer = build_decadal_horoscope_layer(&natal, period)?;

    Ok(HoroscopeChart::with_layers(natal, vec![layer]))
}
