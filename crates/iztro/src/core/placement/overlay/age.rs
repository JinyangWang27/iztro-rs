//! Assembly helper for one selected nominal-age horoscope layer.
//!
//! This module composes a derived 小限 [`AgePeriod`] into one model-only
//! [`TemporalLayer`]. Age layers carry a temporal palace-name layout and
//! Heavenly Stem mutagen activations only. They intentionally do not place
//! scoped flow stars or assemble yearly/monthly/daily/hourly horoscope facts.

use crate::core::error::ChartError;
use crate::core::model::chart::{AgePeriod, Chart, TemporalContext, TemporalLayer};
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::overlay::mutagen::stem_mutagen_activations;

/// Builds the composed 小限 temporal layer for a selected nominal-age period.
pub fn build_age_horoscope_layer(
    natal: &Chart,
    period: &AgePeriod,
) -> Result<TemporalLayer, ChartError> {
    let context = TemporalContext::Age {
        stem_branch: period.stem_branch(),
        nominal_age: period.nominal_age(),
    };
    let activations = stem_mutagen_activations(natal, Scope::Age, period.stem_branch().stem());

    TemporalLayer::try_new_with_palace_layout(
        Scope::Age,
        context,
        Vec::new(),
        activations,
        Some(period.palace_layout().clone()),
    )
}
