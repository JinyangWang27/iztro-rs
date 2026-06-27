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
//! [`HoroscopeChart`] or
//! [`TemporalLayer`](crate::core::TemporalLayer) itself.
//!
//! Natal facts are identical across every selection: only the attached temporal
//! overlays and the bottom-panel enable/selected flags differ.
//!
//! [`TemporalLayer`]: crate::core::model::chart::TemporalLayer

use crate::core::calendar::{resolve_non_leap_lunar, solar_to_lunar};
use crate::core::error::ChartError;
use crate::core::facade::by_solar::{SolarChartRequest, by_solar};
use crate::core::labels::chinese_date;
use crate::core::model::calendar::{BirthTime, SolarDay, SolarMonth};
use crate::core::model::chart::{
    Chart, HoroscopeChart, HoroscopeTargetContext, build_age_period, build_decadal_frame,
};
use crate::core::model::star::mutagen::Scope;
use crate::core::placement::overlay::partial_horoscope::{
    PartialHoroscope, build_partial_horoscope_chart,
};
use crate::core::view::static_chart::{
    LunarDateView, StaticChartViewRequest, StaticChartViewSnapshot,
    StaticTemporalNavigationSelection, StaticTemporalPanelView,
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
    static_temporal_chart_view_from_chart(natal, selection)
}

/// Builds a prepared static chart view from an already-built natal [`Chart`].
///
/// This is the chart-building-free half of [`static_temporal_chart_view`]: a
/// GUI/app layer can build the natal `Chart` once with [`by_solar`] and then
/// derive both the static chart snapshot (here) and the classical rule panel
/// from that same chart, avoiding a duplicate chart generation.
///
/// Behavior, selection validation, and overlay derivation are identical to
/// [`static_temporal_chart_view`]; only the chart-building step is hoisted out.
/// The natal `Chart` is taken by value because the partial-overlay path
/// (`build_partial_horoscope_chart`) consumes it.
pub fn static_temporal_chart_view_from_chart(
    natal: Chart,
    selection: StaticTemporalNavigationSelection,
) -> Result<StaticChartViewSnapshot, ChartError> {
    validate_selection_indices(selection)?;

    match selection {
        StaticTemporalNavigationSelection::Natal
        | StaticTemporalNavigationSelection::PreDecadal => {
            let mut snapshot = StaticChartViewSnapshot::from_chart(&natal);
            snapshot.temporal_panel = StaticTemporalPanelView::from_selection(&natal, selection);
            decorate_temporal(&mut snapshot, &natal, selection, None)?;
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
            let target = horoscope.target_context().cloned();
            decorate_temporal(&mut snapshot, horoscope.natal(), selection, target.as_ref())?;
            Ok(snapshot)
        }
    }
}

/// An owned chart context for one temporal navigation selection.
///
/// Natal / pre-decadal selections carry no temporal overlay, so they hold the
/// natal [`Chart`] directly; every deeper selection holds the
/// [`HoroscopeChart`](crate::core::HoroscopeChart) whose overlay stack reaches
/// the selected scope. Both expose the natal facts (a [`HoroscopeChart`] borrows
/// its own natal), so an analysis caller can build a read context over either.
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
/// This is the overlay-building half of [`static_temporal_chart_view_from_chart`]
/// without the snapshot/panel decoration: it resolves the selection indices to
/// concrete lunar/solar coordinates and assembles the partial horoscope stack up
/// to the selected scope, returning a [`SelectedTemporalChart`] the caller can
/// analyze. Natal / pre-decadal selections need no overlay and return the natal
/// chart unchanged.
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

/// Fills the selection-dependent center labels (年龄(虚岁), 运限农历, 运限阳历) and
/// marks the active decadal palace.
///
/// All natal facts already live on the snapshot; this only layers on the facts
/// that depend on the temporal navigation selection.
fn decorate_temporal(
    snapshot: &mut StaticChartViewSnapshot,
    natal: &Chart,
    selection: StaticTemporalNavigationSelection,
    target: Option<&HoroscopeTargetContext>,
) -> Result<(), ChartError> {
    if let Some(decadal_index) = selection.decadal_index() {
        if let Ok(frame) = build_decadal_frame(natal) {
            if let Some(period) = frame.periods().get(decadal_index) {
                let active_branch = period.palace_branch();
                for palace in &mut snapshot.palaces {
                    if palace.branch == active_branch {
                        palace.limit.is_active_decadal = true;
                    }
                }

                let nominal_age =
                    u16::from(period.start_age()) + selection.year_index().map_or(0, u16::from);
                snapshot.center.nominal_age_label = Some(format!("{nominal_age} 岁"));
                snapshot.center.nominal_age = Some(nominal_age);

                // 小限 (Minor Limit) is an annual age marker, so it only exists
                // once a concrete year is selected (流年/流月/流日/流时). A
                // Decadal-only / PreDecadal selection carries no selected year
                // and therefore no active 小限.
                if selection.year_index().is_some() {
                    decorate_active_small_limit(snapshot, natal, nominal_age)?;
                }
            }
        }
    }

    if let Some(target) = target {
        let solar = target.solar_date();
        let lunar = target.lunar_date();
        snapshot.center.temporal_solar_label = Some(chinese_date::solar_date_label_unpadded(
            solar.year(),
            solar.month(),
            solar.day(),
        ));
        snapshot.center.temporal_lunar_label = Some(chinese_date::lunar_date_label(
            lunar.year(),
            lunar.month(),
            lunar.day(),
            lunar.is_leap_month(),
        ));
        snapshot.center.temporal_lunar_date = Some(LunarDateView {
            year: lunar.year(),
            month: lunar.month(),
            day: lunar.day(),
            is_leap_month: lunar.is_leap_month(),
        });
    } else if let (Some(decadal_index), Some(year_index)) =
        (selection.decadal_index(), selection.year_index())
    {
        // A 流年 selection resolves only to a lunar year, not a concrete day.
        if let Ok(year) = lunar_year_for(natal, decadal_index, year_index) {
            snapshot.center.temporal_lunar_label =
                Some(format!("{}年", chinese_date::chinese_year_digits(year)));
            snapshot.center.temporal_solar_label = Some(format!("{year}"));
            snapshot.center.temporal_lunar_year = Some(year);
        }
    }

    Ok(())
}

/// Marks the active 小限 (Minor Limit) palace for `nominal_age`.
///
/// 小限 is the nominal-age (虚岁) marker of [`Scope::Age`]; it is derived from
/// the selected nominal age via the existing age-domain logic
/// ([`build_age_period`]) and is deliberately distinct from 流年
/// ([`Scope::Yearly`]), which is selected-year / stem-branch / 太岁 based.
///
/// Exactly one palace (the one whose branch matches the resolved 小限 branch) is
/// marked active; all others are left inactive.
///
/// A nominal age outside the modeled 小限 range (`1..=120`, reachable for the
/// final 大限) carries no 小限 and is left clear; this is a valid navigation
/// state, not an error. Any genuine inconsistency surfaced by
/// [`build_age_period`] (an unbuildable palace stem-branch) is propagated.
fn decorate_active_small_limit(
    snapshot: &mut StaticChartViewSnapshot,
    natal: &Chart,
    nominal_age: u16,
) -> Result<(), ChartError> {
    let Some(age) = u8::try_from(nominal_age)
        .ok()
        .filter(|age| (1..=120).contains(age))
    else {
        return Ok(());
    };
    let branch = build_age_period(natal, age)?.palace_branch();
    snapshot.center.small_limit_age = Some(nominal_age);
    snapshot.center.small_limit_branch = Some(branch);
    for palace in &mut snapshot.palaces {
        let active = palace.branch == branch;
        palace.limit.is_active_small_limit = active;
        palace.limit.active_small_limit_age = active.then_some(nominal_age);
    }
    debug_assert_eq!(
        snapshot
            .palaces
            .iter()
            .filter(|palace| palace.limit.is_active_small_limit)
            .count(),
        1,
        "the resolved 小限 branch must match exactly one palace",
    );
    Ok(())
}

/// Resolves the temporal navigation selection that points at a given local
/// solar moment ("today"), for the `今` control.
///
/// The renderer supplies only the explicit current solar date/time facts; all
/// calendar conversion, nominal-age, and decadal-period mapping happens here.
/// The clock `hour` (`0..=23`) is mapped to the conventional double-hour
/// `timeIndex`; `minute` is currently unused but accepted so the renderer can
/// pass a complete moment.
pub fn temporal_selection_for_local_moment(
    natal: &Chart,
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
) -> Result<StaticTemporalNavigationSelection, ChartError> {
    let _ = minute;
    let conversion = solar_to_lunar(
        year,
        SolarMonth::new(month)?,
        SolarDay::new(day)?,
        time_index_for_hour(hour),
    )?;

    let birth_lunar_year = natal.birth_context().date().year();
    let nominal_age = conversion.lunar_year() - birth_lunar_year + 1;
    let frame = build_decadal_frame(natal)?;
    let periods = frame.periods();
    let first_start = periods.first().map_or(1, |period| period.start_age());

    if nominal_age < i32::from(first_start) {
        return Ok(StaticTemporalNavigationSelection::PreDecadal);
    }

    let (decadal_index, period) = periods
        .iter()
        .enumerate()
        .find(|(_, period)| {
            i32::from(period.start_age()) <= nominal_age
                && nominal_age <= i32::from(period.end_age())
        })
        .unwrap_or((periods.len() - 1, periods.last().expect("12 periods")));

    let year_index = (nominal_age - i32::from(period.start_age())).clamp(0, 9) as u8;
    let month_index = conversion.lunar_month().value().saturating_sub(1).min(11);
    let day_index = conversion.lunar_day().value().saturating_sub(1).min(29);
    let hour_index = time_index_for_hour(hour);

    Ok(StaticTemporalNavigationSelection::Hourly {
        decadal_index,
        year_index,
        month_index,
        day_index,
        hour_index,
    })
}

/// Resolves the "today" temporal selection straight from a [`SolarChartRequest`]
/// and a local solar moment, so a renderer never has to build or hold a
/// [`Chart`] itself.
pub fn temporal_selection_for_solar_moment(
    request: SolarChartRequest,
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
) -> Result<StaticTemporalNavigationSelection, ChartError> {
    let natal = by_solar(request)?;
    temporal_selection_for_local_moment(&natal, year, month, day, hour, minute)
}

/// Maps a clock hour (`0..=23`) to the conventional double-hour `timeIndex`.
///
/// Hour `0` is early Zi (`0`) and hour `23` is late Zi (`12`); every other hour
/// folds into its branch's two-hour window.
///
/// This is the crate's single canonical clock-hour to `timeIndex` mapping. It is
/// shared by the calculation-policy resolver so that clock-time and
/// apparent-solar-time inputs derive 时辰 through the same rule.
pub(crate) const fn time_index_for_hour(hour: u8) -> u8 {
    match hour {
        0 => 0,
        23 => 12,
        h => h.div_ceil(2),
    }
}

fn validate_selection_indices(
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
