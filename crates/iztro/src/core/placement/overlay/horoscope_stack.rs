//! Full horoscope stack assembly for the supported temporal fact surface.
//!
//! This module composes the already-implemented temporal period/layer builders
//! into one [`HoroscopeChart`] holding a deterministic six-layer overlay stack:
//! decadal (大限), nominal-age (小限), yearly (流年), monthly (流月), daily (流日),
//! and hourly (流时). It only composes existing supported facts: it does not
//! derive new star placements, attach `yearlyDecStar`, project runtime palaces,
//! expose query helpers, render prose, or reproduce the upstream
//! `FunctionalAstrolabe#horoscope` payload shape.

use crate::core::calculation::NominalAgeBoundary;
use crate::core::error::ChartError;
use crate::core::model::calendar::{BirthTime, SolarDay, SolarMonth};
use crate::core::model::chart::{
    Chart, HoroscopeChart, HoroscopeLunarDate, HoroscopeSolarDate, HoroscopeTargetContext,
    TemporalLayer, build_age_period, build_daily_period, build_decadal_frame, build_hourly_period,
    build_monthly_period, build_yearly_period, nominal_age_for_target, select_decadal_period_by_age,
    target_lunar_date,
};
use crate::core::placement::overlay::age::build_age_horoscope_layer;
use crate::core::placement::overlay::daily_horoscope::build_daily_horoscope_layer;
use crate::core::placement::overlay::decadal_horoscope::build_decadal_horoscope_layer;
use crate::core::placement::overlay::hourly_horoscope::build_hourly_horoscope_layer;
use crate::core::placement::overlay::monthly_horoscope::build_monthly_horoscope_layer;
use crate::core::placement::overlay::yearly_horoscope::build_yearly_horoscope_layer;

/// The target solar date/time a full horoscope stack is assembled for.
///
/// Mirrors the `targetSolarDate` and `targetTimeIndex` upstream
/// `FunctionalAstrolabe#horoscope` consumes. It carries only the target instant:
/// no language, rendering, query-helper, or facade JSON fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct HoroscopeStackInput {
    target_solar_year: i32,
    target_solar_month: SolarMonth,
    target_solar_day: SolarDay,
    target_time: BirthTime,
    nominal_age_boundary: NominalAgeBoundary,
}

impl HoroscopeStackInput {
    /// Creates a full horoscope stack input from a target solar date and time.
    ///
    /// The 虚岁分界 policy defaults to [`NominalAgeBoundary::NaturalYear`],
    /// preserving existing nominal-age behaviour. Use
    /// [`with_nominal_age_boundary`](Self::with_nominal_age_boundary) to override
    /// it.
    pub const fn new(
        target_solar_year: i32,
        target_solar_month: SolarMonth,
        target_solar_day: SolarDay,
        target_time: BirthTime,
    ) -> Self {
        Self {
            target_solar_year,
            target_solar_month,
            target_solar_day,
            target_time,
            nominal_age_boundary: NominalAgeBoundary::NaturalYear,
        }
    }

    /// Returns a copy with the 虚岁分界 policy replaced.
    pub const fn with_nominal_age_boundary(
        mut self,
        nominal_age_boundary: NominalAgeBoundary,
    ) -> Self {
        self.nominal_age_boundary = nominal_age_boundary;
        self
    }

    /// Returns the 虚岁分界 policy used for nominal-age resolution.
    pub const fn nominal_age_boundary(&self) -> NominalAgeBoundary {
        self.nominal_age_boundary
    }

    /// Returns the target solar year.
    pub const fn target_solar_year(&self) -> i32 {
        self.target_solar_year
    }

    /// Returns the target solar month.
    pub const fn target_solar_month(&self) -> SolarMonth {
        self.target_solar_month
    }

    /// Returns the target solar day.
    pub const fn target_solar_day(&self) -> SolarDay {
        self.target_solar_day
    }

    /// Returns the target double-hour birth time.
    pub const fn target_time(&self) -> BirthTime {
        self.target_time
    }
}

/// Assembles the full six-layer horoscope stack for a target date/time.
///
/// Derives the target lunar date and nominal age from the target solar date,
/// selects the covering decadal period by nominal age (never a hard-coded index),
/// and composes one layer per scope in the fixed order decadal → age → yearly →
/// monthly → daily → hourly. The natal chart is moved in unchanged; every
/// temporal fact is an additive overlay.
pub fn build_full_horoscope_chart(
    natal: Chart,
    input: HoroscopeStackInput,
) -> Result<HoroscopeChart, ChartError> {
    let target_lunar = target_lunar_date(
        input.target_solar_year,
        input.target_solar_month,
        input.target_solar_day,
    )?;
    let nominal_age = nominal_age_for_target(
        &natal,
        target_lunar.year,
        target_lunar.month,
        target_lunar.day,
        input.nominal_age_boundary,
    )?;

    let decadal_frame = build_decadal_frame(&natal)?;
    let decadal_period = select_decadal_period_by_age(&decadal_frame, nominal_age)?;
    let decadal_layer = build_decadal_horoscope_layer(&natal, decadal_period)?;

    let age_period = build_age_period(&natal, nominal_age)?;
    let age_layer = build_age_horoscope_layer(&natal, &age_period)?;

    let yearly_period = build_yearly_period(target_lunar.year)?;
    let yearly_layer = build_yearly_horoscope_layer(&natal, &yearly_period)?;

    let monthly_period = build_monthly_period(
        &natal,
        input.target_solar_year,
        input.target_solar_month,
        input.target_solar_day,
        input.target_time,
    )?;
    let monthly_layer = build_monthly_horoscope_layer(&natal, &monthly_period)?;

    let daily_period = build_daily_period(
        &natal,
        input.target_solar_year,
        input.target_solar_month,
        input.target_solar_day,
        input.target_time,
    )?;
    let daily_layer = build_daily_horoscope_layer(&natal, &daily_period)?;

    let hourly_period = build_hourly_period(
        &natal,
        input.target_solar_year,
        input.target_solar_month,
        input.target_solar_day,
        input.target_time,
    )?;
    let hourly_layer = build_hourly_horoscope_layer(&natal, &hourly_period)?;

    let layers: Vec<TemporalLayer> = vec![
        decadal_layer,
        age_layer,
        yearly_layer,
        monthly_layer,
        daily_layer,
        hourly_layer,
    ];
    let target_context = HoroscopeTargetContext::new(
        HoroscopeSolarDate::new(
            input.target_solar_year,
            input.target_solar_month.value(),
            input.target_solar_day.value(),
        ),
        HoroscopeLunarDate::new(
            target_lunar.year,
            target_lunar.month,
            target_lunar.day,
            target_lunar.is_leap_month,
        ),
        input.target_time.iztro_time_index(),
    );

    Ok(HoroscopeChart::with_layers_and_target_context(
        natal,
        layers,
        target_context,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::facade::by_solar::{SolarChartRequest, by_solar};
    use crate::core::model::calendar::Gender;
    use crate::core::model::chart::TemporalContext;
    use crate::core::model::profile::{ChartAlgorithmKind, MethodProfile};
    use crate::core::model::star::mutagen::Scope;

    fn natal_born_1985_02_15() -> Chart {
        // Solar 1985-02-15 converts to lunar 1984-12-26, a late birth lunar
        // month, so a mid-year target lands before the lunar birthday.
        by_solar(
            SolarChartRequest::builder()
                .solar_year(1985)
                .solar_month(SolarMonth::new(2).expect("valid month"))
                .solar_day(SolarDay::new(15).expect("valid day"))
                .birth_time(lunar_lite::EarthlyBranch::Chen)
                .gender(Gender::Female)
                .method_profile(MethodProfile::new(
                    "quanshu_test",
                    ChartAlgorithmKind::QuanShu,
                    "quanshu test",
                ))
                .build()
                .expect("request should build"),
        )
        .expect("natal chart should build")
    }

    fn stack_input(boundary: NominalAgeBoundary) -> HoroscopeStackInput {
        HoroscopeStackInput::new(
            2000,
            SolarMonth::new(6).expect("valid month"),
            SolarDay::new(1).expect("valid day"),
            BirthTime::from_iztro_time_index(2).expect("valid time index"),
        )
        .with_nominal_age_boundary(boundary)
    }

    fn age_layer_nominal_age(chart: &HoroscopeChart) -> u8 {
        chart
            .layers_in_scope(Scope::Age)
            .find_map(|layer| match layer.context() {
                TemporalContext::Age { nominal_age, .. } => Some(*nominal_age),
                _ => None,
            })
            .expect("horoscope should expose a nominal-age layer")
    }

    #[test]
    fn nominal_age_boundary_threads_into_full_horoscope() {
        // Target 2000-06-01 is before the late-year lunar birthday, so the
        // birthday boundary withholds the +1 increment the natural-year boundary
        // always applies.
        let natural = build_full_horoscope_chart(
            natal_born_1985_02_15(),
            stack_input(NominalAgeBoundary::NaturalYear),
        )
        .expect("natural-year horoscope should build");
        let birthday = build_full_horoscope_chart(
            natal_born_1985_02_15(),
            stack_input(NominalAgeBoundary::Birthday),
        )
        .expect("birthday horoscope should build");

        assert_eq!(
            age_layer_nominal_age(&natural),
            age_layer_nominal_age(&birthday) + 1,
            "the birthday boundary should be one nominal year behind before the birthday",
        );
    }

    #[test]
    fn default_nominal_age_boundary_is_natural_year() {
        let default_chart = build_full_horoscope_chart(
            natal_born_1985_02_15(),
            HoroscopeStackInput::new(
                2000,
                SolarMonth::new(6).expect("valid month"),
                SolarDay::new(1).expect("valid day"),
                BirthTime::from_iztro_time_index(2).expect("valid time index"),
            ),
        )
        .expect("default horoscope should build");
        let natural = build_full_horoscope_chart(
            natal_born_1985_02_15(),
            stack_input(NominalAgeBoundary::NaturalYear),
        )
        .expect("natural horoscope should build");

        assert_eq!(
            age_layer_nominal_age(&default_chart),
            age_layer_nominal_age(&natural),
        );
    }
}
