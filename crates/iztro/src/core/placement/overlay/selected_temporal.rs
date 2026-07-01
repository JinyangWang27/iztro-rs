//! Selection-driven temporal chart resolution.
//!
//! Given a natal [`Chart`] and a [`StaticTemporalNavigationSelection`] index path,
//! these helpers resolve the selection to concrete lunar/solar coordinates and
//! assemble the partial horoscope overlay stack up to the selected scope. The
//! output is core domain data ([`Chart`] / [`HoroscopeChart`]) with no
//! presentation concern, so both the projection facade and the analysis layer
//! depend on it — it lives in `core`, below them.

use crate::core::calendar::resolve_non_leap_lunar;
use crate::core::error::ChartError;
use crate::core::model::calendar::BirthTime;
use crate::core::model::chart::{
    Chart, HoroscopeChart, StaticTemporalNavigationSelection, build_decadal_frame,
};
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::overlay::partial_horoscope::{
    PartialHoroscope, build_partial_horoscope_chart,
};

/// A representative lunar day used when only a 流月 (not a 流日) is selected.
pub(crate) const REPRESENTATIVE_LUNAR_DAY: u8 = 15;

/// An owned chart context for one temporal navigation selection.
///
/// Natal / pre-decadal selections carry no temporal overlay, so they hold the
/// natal [`Chart`] directly; every deeper selection holds the [`HoroscopeChart`]
/// whose overlay stack reaches the selected scope. Both expose the natal facts (a
/// [`HoroscopeChart`] borrows its own natal), so an analysis caller can build a
/// read context over either.
///
/// This keeps overlay construction inside core: callers receive a built context,
/// never the overlay-building primitives.
pub(crate) enum SelectedTemporalChart {
    /// Natal / pre-decadal: natal facts with no temporal overlay.
    Natal(Chart),
    /// Decadal or deeper: the partial overlay stack up to the selected scope.
    Horoscope(Box<HoroscopeChart>),
}

/// Builds the owned chart context for one temporal navigation selection.
///
/// This is the overlay-building half of the static temporal projection without
/// the snapshot/panel decoration: it resolves the selection indices to concrete
/// lunar/solar coordinates and assembles the partial horoscope stack up to the
/// selected scope, returning a [`SelectedTemporalChart`] the caller can analyze.
/// Natal / pre-decadal selections need no overlay and return the natal chart
/// unchanged.
///
/// The natal `Chart` is taken by value because the partial-overlay path
/// ([`build_partial_horoscope_chart`]) consumes it.
pub(crate) fn build_selected_temporal_chart(
    natal: Chart,
    selection: StaticTemporalNavigationSelection,
) -> Result<SelectedTemporalChart, ChartError> {
    validate_selection_indices(selection)?;

    match selection {
        StaticTemporalNavigationSelection::Natal
        | StaticTemporalNavigationSelection::PreDecadal => Ok(SelectedTemporalChart::Natal(natal)),
        _ => {
            let (spec, _visible_scopes) = resolve_partial(&natal, selection)?;
            let horoscope = build_partial_horoscope_chart(natal, spec)?;
            Ok(SelectedTemporalChart::Horoscope(Box::new(horoscope)))
        }
    }
}

/// Validates the per-scope index ranges a selection index path may carry.
pub(crate) fn validate_selection_indices(
    selection: StaticTemporalNavigationSelection,
) -> Result<(), ChartError> {
    match selection {
        StaticTemporalNavigationSelection::Yearly { year_index, .. } if year_index > 9 => {
            Err(ChartError::InvalidTemporalSelectionIndex {
                field: "year_index",
                value: year_index,
                max: 9,
            })
        }
        StaticTemporalNavigationSelection::Monthly { year_index, .. }
        | StaticTemporalNavigationSelection::Daily { year_index, .. }
        | StaticTemporalNavigationSelection::Hourly { year_index, .. }
            if year_index > 9 =>
        {
            Err(ChartError::InvalidTemporalSelectionIndex {
                field: "year_index",
                value: year_index,
                max: 9,
            })
        }
        StaticTemporalNavigationSelection::Monthly { month_index, .. }
        | StaticTemporalNavigationSelection::Daily { month_index, .. }
        | StaticTemporalNavigationSelection::Hourly { month_index, .. }
            if month_index > 11 =>
        {
            Err(ChartError::InvalidTemporalSelectionIndex {
                field: "month_index",
                value: month_index,
                max: 11,
            })
        }
        StaticTemporalNavigationSelection::Daily { day_index, .. }
        | StaticTemporalNavigationSelection::Hourly { day_index, .. }
            if day_index > 29 =>
        {
            Err(ChartError::InvalidTemporalSelectionIndex {
                field: "day_index",
                value: day_index,
                max: 29,
            })
        }
        StaticTemporalNavigationSelection::Hourly { hour_index, .. } if hour_index > 12 => {
            Err(ChartError::InvalidTemporalSelectionIndex {
                field: "hour_index",
                value: hour_index,
                max: 12,
            })
        }
        _ => Ok(()),
    }
}

/// Resolves a drill-down selection (index path) to a [`PartialHoroscope`] and the
/// scopes that should be visible/selected for it.
pub(crate) fn resolve_partial(
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
pub(crate) fn lunar_year_for(
    natal: &Chart,
    decadal_index: usize,
    year_index: u8,
) -> Result<i32, ChartError> {
    let frame = build_decadal_frame(natal)?;
    let period =
        frame
            .periods()
            .get(decadal_index)
            .ok_or(ChartError::InvalidDecadalPeriodIndex {
                index: decadal_index,
                len: frame.periods().len(),
            })?;
    let nominal_age = period.start_age() as i32 + year_index as i32;
    Ok(natal.birth_context().date().year() + nominal_age - 1)
}
