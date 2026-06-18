//! Public facade mapping a temporal navigation choice to a prepared static view.
//!
//! This is the single entry point a renderer uses to make the bottom temporal
//! navigation panel *effective*. The renderer reports which navigation cell the
//! user chose as a [`StaticTemporalNavigationSelection`]; core builds the natal
//! chart, derives any required temporal overlay, and returns a prepared
//! [`StaticChartViewSnapshot`]. All overlay derivation (decadal frame, flow
//! stars, mutagens, temporal palace layout) stays inside core — the renderer
//! never constructs a [`HoroscopeChart`] or [`TemporalLayer`] itself.
//!
//! Natal facts are identical across every selection: only the attached temporal
//! overlays differ. Selections that require a concrete target date core cannot
//! yet infer from cell coordinates alone (流年/流月/流日/流时) resolve to the
//! natal base slice.

use crate::core::error::ChartError;
use crate::core::facade::by_solar::{SolarChartRequest, by_solar};
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::overlay::decadal_horoscope::{
    DecadalHoroscopeInput, build_decadal_horoscope_chart,
};
use crate::core::view::static_chart::{
    StaticChartViewRequest, StaticChartViewSnapshot, StaticTemporalNavigationSelection,
};

/// Builds a prepared static chart view for one temporal navigation selection.
///
/// The natal chart is always built from `request` through [`by_solar`]. The
/// `selection` then chooses which prepared slice to return:
///
/// - [`Natal`](StaticTemporalNavigationSelection::Natal) and
///   [`PreDecadal`](StaticTemporalNavigationSelection::PreDecadal) return the
///   natal base slice with no temporal overlay.
/// - [`Decadal`](StaticTemporalNavigationSelection::Decadal) derives a single
///   decadal horoscope overlay for the selected period and returns the slice
///   with 本命 + 大限 active. An out-of-range period index is an error.
/// - The remaining flowing scopes resolve to the natal base slice for now,
///   because core cannot yet infer their concrete target date from a cell index.
pub fn static_temporal_chart_view(
    request: SolarChartRequest,
    selection: StaticTemporalNavigationSelection,
) -> Result<StaticChartViewSnapshot, ChartError> {
    let natal = by_solar(request)?;

    match selection {
        StaticTemporalNavigationSelection::Decadal { index } => {
            let horoscope =
                build_decadal_horoscope_chart(natal, DecadalHoroscopeInput { period_index: index })?;
            Ok(StaticChartViewSnapshot::from_horoscope_chart_with(
                &horoscope,
                &StaticChartViewRequest {
                    visible_scopes: vec![Scope::Natal, Scope::Decadal],
                },
            ))
        }
        StaticTemporalNavigationSelection::Natal
        | StaticTemporalNavigationSelection::PreDecadal
        | StaticTemporalNavigationSelection::YearlyAge { .. }
        | StaticTemporalNavigationSelection::Month { .. }
        | StaticTemporalNavigationSelection::Day { .. }
        | StaticTemporalNavigationSelection::Hour { .. } => {
            Ok(StaticChartViewSnapshot::from_chart(&natal))
        }
    }
}
