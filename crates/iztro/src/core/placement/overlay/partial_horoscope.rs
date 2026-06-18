//! Partial horoscope-stack assembly for the static-chart drill-down.
//!
//! The bottom temporal panel reveals one scope at a time (大限 → 流年 → 流月 →
//! 流日 → 流时). This module composes the existing per-layer builders into a
//! [`HoroscopeChart`] holding only the layers up to the selected scope, in the
//! same fixed order as [`build_full_horoscope_chart`]: decadal, age, yearly,
//! monthly, daily, hourly. It derives no new star placements; it only truncates
//! the existing stack to the selected depth.
//!
//! [`build_full_horoscope_chart`]:
//! crate::core::placement::overlay::horoscope_stack::build_full_horoscope_chart

use crate::core::calendar::ResolvedTemporalTarget;
use crate::core::error::ChartError;
use crate::core::model::chart::{
    Chart, HoroscopeChart, HoroscopeLunarDate, HoroscopeSolarDate, HoroscopeTargetContext,
    TemporalLayer, build_age_period, build_daily_period, build_decadal_frame, build_hourly_period,
    build_monthly_period, build_yearly_period, nominal_age_for_target_year,
};
use crate::core::placement::overlay::age::build_age_horoscope_layer;
use crate::core::placement::overlay::daily_horoscope::build_daily_horoscope_layer;
use crate::core::placement::overlay::decadal_horoscope::build_decadal_horoscope_layer;
use crate::core::placement::overlay::hourly_horoscope::build_hourly_horoscope_layer;
use crate::core::placement::overlay::monthly_horoscope::build_monthly_horoscope_layer;
use crate::core::placement::overlay::yearly_horoscope::build_yearly_horoscope_layer;

/// The selected drill-down depth, with the concrete coordinates each layer needs.
///
/// Every deeper variant carries its ancestors' coordinates so the stack can be
/// composed in one pass. `lunar_year` drives both the age (小限) and yearly (流年)
/// layers; `target` carries the resolved solar date/time for the sub-yearly
/// layers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PartialHoroscope {
    /// Natal + 大限 only.
    Decadal { period_index: usize },
    /// Natal + 大限 + 小限 + 流年.
    Yearly {
        period_index: usize,
        lunar_year: i32,
    },
    /// … + 流月.
    Monthly {
        period_index: usize,
        lunar_year: i32,
        target: ResolvedTemporalTarget,
    },
    /// … + 流日.
    Daily {
        period_index: usize,
        lunar_year: i32,
        target: ResolvedTemporalTarget,
    },
    /// … + 流时.
    Hourly {
        period_index: usize,
        lunar_year: i32,
        target: ResolvedTemporalTarget,
    },
}

impl PartialHoroscope {
    const fn period_index(&self) -> usize {
        match self {
            Self::Decadal { period_index }
            | Self::Yearly { period_index, .. }
            | Self::Monthly { period_index, .. }
            | Self::Daily { period_index, .. }
            | Self::Hourly { period_index, .. } => *period_index,
        }
    }

    /// The lunar year for the age/yearly layers, when the depth reaches 流年.
    const fn lunar_year(&self) -> Option<i32> {
        match self {
            Self::Decadal { .. } => None,
            Self::Yearly { lunar_year, .. }
            | Self::Monthly { lunar_year, .. }
            | Self::Daily { lunar_year, .. }
            | Self::Hourly { lunar_year, .. } => Some(*lunar_year),
        }
    }

    /// The resolved solar target for the sub-yearly layers, when present.
    const fn target(&self) -> Option<ResolvedTemporalTarget> {
        match self {
            Self::Decadal { .. } | Self::Yearly { .. } => None,
            Self::Monthly { target, .. }
            | Self::Daily { target, .. }
            | Self::Hourly { target, .. } => Some(*target),
        }
    }
}

/// Assembles the partial horoscope stack for one drill-down selection.
///
/// Layers are appended in fixed scope order and truncated at the selected depth.
/// The natal chart is moved in unchanged; every temporal fact is an additive
/// overlay built from the existing per-layer builders.
pub(crate) fn build_partial_horoscope_chart(
    natal: Chart,
    spec: PartialHoroscope,
) -> Result<HoroscopeChart, ChartError> {
    let frame = build_decadal_frame(&natal)?;
    let period_index = spec.period_index();
    let period =
        frame
            .periods()
            .get(period_index)
            .ok_or(ChartError::InvalidDecadalPeriodIndex {
                index: period_index,
                len: frame.periods().len(),
            })?;

    let mut layers: Vec<TemporalLayer> = vec![build_decadal_horoscope_layer(&natal, period)?];

    if let Some(lunar_year) = spec.lunar_year() {
        let nominal_age = nominal_age_for_target_year(&natal, lunar_year)?;
        let age_period = build_age_period(&natal, nominal_age)?;
        layers.push(build_age_horoscope_layer(&natal, &age_period)?);

        let yearly_period = build_yearly_period(lunar_year)?;
        layers.push(build_yearly_horoscope_layer(&natal, &yearly_period)?);
    }

    if let Some(target) = spec.target() {
        let monthly_period = build_monthly_period(
            &natal,
            target.solar_year,
            target.solar_month,
            target.solar_day,
            target.target_time,
        )?;
        layers.push(build_monthly_horoscope_layer(&natal, &monthly_period)?);

        if matches!(
            spec,
            PartialHoroscope::Daily { .. } | PartialHoroscope::Hourly { .. }
        ) {
            let daily_period = build_daily_period(
                &natal,
                target.solar_year,
                target.solar_month,
                target.solar_day,
                target.target_time,
            )?;
            layers.push(build_daily_horoscope_layer(&natal, &daily_period)?);
        }

        if matches!(spec, PartialHoroscope::Hourly { .. }) {
            let hourly_period = build_hourly_period(
                &natal,
                target.solar_year,
                target.solar_month,
                target.solar_day,
                target.target_time,
            )?;
            layers.push(build_hourly_horoscope_layer(&natal, &hourly_period)?);
        }
    }

    let chart = HoroscopeChart::with_layers(natal, layers);
    Ok(match spec.target() {
        Some(target) => chart.with_target_context(HoroscopeTargetContext::new(
            HoroscopeSolarDate::new(
                target.solar_year,
                target.solar_month.value(),
                target.solar_day.value(),
            ),
            HoroscopeLunarDate::new(
                target.lunar_year,
                target.lunar_month,
                target.lunar_day,
                target.is_leap_month,
            ),
            target.target_time.iztro_time_index(),
        )),
        None => chart,
    })
}
