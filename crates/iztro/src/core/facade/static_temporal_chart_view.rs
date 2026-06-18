//! Public facade mapping a temporal navigation choice to a prepared static view.
//!
//! This is the single entry point a renderer uses to make the bottom temporal
//! navigation panel *effective*. The renderer reports which navigation cell the
//! user chose as a [`StaticTemporalNavigationSelection`] index path; core builds
//! the natal chart, resolves the indices to concrete lunar/solar coordinates,
//! derives the partial temporal-overlay stack up to the selected scope, and
//! returns a prepared [`StaticChartViewSnapshot`]. All overlay derivation
//! (decadal frame, flow stars, mutagens, temporal palace layout, lunar→solar
//! resolution) stays inside core — the renderer never constructs a
//! [`HoroscopeChart`] or [`TemporalLayer`] itself.
//!
//! Natal facts are identical across every selection: only the attached temporal
//! overlays and the bottom-panel enable/selected flags differ.
//!
//! [`TemporalLayer`]: crate::core::model::chart::TemporalLayer

use crate::core::calendar::resolve_non_leap_lunar;
use crate::core::error::ChartError;
use crate::core::facade::by_solar::{SolarChartRequest, by_solar};
use crate::core::model::calendar::BirthTime;
use crate::core::model::chart::{Chart, build_decadal_frame};
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::overlay::partial_horoscope::{
    PartialHoroscope, build_partial_horoscope_chart,
};
use crate::core::view::static_chart::{
    StaticChartViewRequest, StaticChartViewSnapshot, StaticTemporalNavigationSelection,
    StaticTemporalPanelView,
};

/// A representative lunar day used when only a 流月 (not a 流日) is selected.
const REPRESENTATIVE_LUNAR_DAY: u8 = 15;

/// Builds a prepared static chart view for one temporal navigation selection.
///
/// - [`Natal`](StaticTemporalNavigationSelection::Natal) /
///   [`PreDecadal`](StaticTemporalNavigationSelection::PreDecadal) return the
///   natal base slice with no temporal overlay.
/// - [`Decadal`](StaticTemporalNavigationSelection::Decadal) and deeper build the
///   partial overlay stack up to the selected scope and select the matching
///   scopes (本命 + 大限 [+ 小限 + 流年 [+ 流月 [+ 流日 [+ 流时]]]]).
///
/// In every case the returned snapshot's bottom panel carries the prepared
/// enable/selected flags and lunar labels for the selection.
pub fn static_temporal_chart_view(
    request: SolarChartRequest,
    selection: StaticTemporalNavigationSelection,
) -> Result<StaticChartViewSnapshot, ChartError> {
    let natal = by_solar(request)?;

    match selection {
        StaticTemporalNavigationSelection::Natal
        | StaticTemporalNavigationSelection::PreDecadal => {
            let mut snapshot = StaticChartViewSnapshot::from_chart(&natal);
            snapshot.temporal_panel = StaticTemporalPanelView::from_selection(&natal, selection);
            Ok(snapshot)
        }
        _ => {
            let (spec, visible_scopes) = resolve_partial(&natal, selection)?;
            let horoscope = build_partial_horoscope_chart(natal, spec)?;
            let mut snapshot = StaticChartViewSnapshot::from_horoscope_chart_with(
                &horoscope,
                &StaticChartViewRequest { visible_scopes },
            );
            snapshot.temporal_panel =
                StaticTemporalPanelView::from_selection(horoscope.natal(), selection);
            Ok(snapshot)
        }
    }
}

/// Resolves a drill-down selection (index path) to a [`PartialHoroscope`] and the
/// scopes that should be visible/selected for it.
fn resolve_partial(
    natal: &Chart,
    selection: StaticTemporalNavigationSelection,
) -> Result<(PartialHoroscope, Vec<Scope>), ChartError> {
    let decadal_index = selection
        .decadal_index()
        .expect("non-natal selection carries a decadal index");

    match selection {
        StaticTemporalNavigationSelection::Decadal { .. } => Ok((
            PartialHoroscope::Decadal {
                period_index: decadal_index,
            },
            vec![Scope::Natal, Scope::Decadal],
        )),
        StaticTemporalNavigationSelection::Yearly { year_index, .. } => {
            let lunar_year = lunar_year_for(natal, decadal_index, year_index)?;
            Ok((
                PartialHoroscope::Yearly {
                    period_index: decadal_index,
                    lunar_year,
                },
                vec![Scope::Natal, Scope::Decadal, Scope::Age, Scope::Yearly],
            ))
        }
        StaticTemporalNavigationSelection::Monthly {
            year_index,
            month_index,
            ..
        } => {
            let lunar_year = lunar_year_for(natal, decadal_index, year_index)?;
            let target = resolve_non_leap_lunar(
                lunar_year,
                month_index + 1,
                REPRESENTATIVE_LUNAR_DAY,
                BirthTime::EarlyZi,
            )?;
            Ok((
                PartialHoroscope::Monthly {
                    period_index: decadal_index,
                    lunar_year,
                    target,
                },
                vec![
                    Scope::Natal,
                    Scope::Decadal,
                    Scope::Age,
                    Scope::Yearly,
                    Scope::Monthly,
                ],
            ))
        }
        StaticTemporalNavigationSelection::Daily {
            year_index,
            month_index,
            day_index,
            ..
        } => {
            let lunar_year = lunar_year_for(natal, decadal_index, year_index)?;
            let target = resolve_non_leap_lunar(
                lunar_year,
                month_index + 1,
                day_index + 1,
                BirthTime::EarlyZi,
            )?;
            Ok((
                PartialHoroscope::Daily {
                    period_index: decadal_index,
                    lunar_year,
                    target,
                },
                vec![
                    Scope::Natal,
                    Scope::Decadal,
                    Scope::Age,
                    Scope::Yearly,
                    Scope::Monthly,
                    Scope::Daily,
                ],
            ))
        }
        StaticTemporalNavigationSelection::Hourly {
            year_index,
            month_index,
            day_index,
            hour_index,
            ..
        } => {
            let lunar_year = lunar_year_for(natal, decadal_index, year_index)?;
            let target_time = BirthTime::from_iztro_time_index(hour_index)?;
            let target =
                resolve_non_leap_lunar(lunar_year, month_index + 1, day_index + 1, target_time)?;
            Ok((
                PartialHoroscope::Hourly {
                    period_index: decadal_index,
                    lunar_year,
                    target,
                },
                vec![
                    Scope::Natal,
                    Scope::Decadal,
                    Scope::Age,
                    Scope::Yearly,
                    Scope::Monthly,
                    Scope::Daily,
                    Scope::Hourly,
                ],
            ))
        }
        StaticTemporalNavigationSelection::Natal
        | StaticTemporalNavigationSelection::PreDecadal => {
            unreachable!("natal/pre-decadal handled before resolve_partial")
        }
    }
}

/// The lunar year of the `year_index`-th 流年 within the selected 大限 period.
fn lunar_year_for(natal: &Chart, decadal_index: usize, year_index: u8) -> Result<i32, ChartError> {
    let frame = build_decadal_frame(natal)?;
    let period = frame
        .periods()
        .get(decadal_index)
        .ok_or(ChartError::InvalidDecadalPeriodIndex {
            index: decadal_index,
            len: frame.periods().len(),
        })?;
    let nominal_age = period.start_age() as i32 + year_index as i32;
    Ok(natal.birth_context().date().year() + nominal_age - 1)
}
