//! Convenient public facade entry points over strongly typed chart builders.

use crate::core::calendar::resolve_lunar_date;
use crate::core::error::{ChartError, validate_chart_algorithm_plane};
use crate::core::model::calendar::{BirthContext, BirthTime, CalendarDate, Gender};
use crate::core::model::chart::Chart;
use crate::core::model::profile::{ChartAlgorithmKind, ChartPlane, MethodProfile};
use crate::core::placement::natal::input::NatalChartWithSupportedStarsInput;
use crate::core::placement::natal::life_body::{LunarDay, LunarMonth};
use crate::core::placement::natal::strategy::DeterministicNatalStarPlacementStrategy;
use crate::core::placement::natal::supported::build_natal_chart_with_supported_stars_using;
use lunar_lite::{EarthlyBranch, HeavenlyStem, StemBranch};

/// Typed lunar-date request for the iztro-compatible natal chart facade.
///
/// This mirrors iztro's `byLunar` conceptually while keeping explicit Rust
/// domain types. The birth year stem and branch remain explicit because
/// by-lunar full four-pillar derivation is not supported yet.
///
/// `chart_plane` defaults to [`ChartPlane::Heaven`], which reproduces existing
/// chart-generation behaviour. `Earth` and `Human` are accepted as request
/// values but are not implemented yet: a Zhongzhou Earth/Human request returns
/// [`ChartError::ChartPlaneNotImplemented`] until a later PR adds it, and an
/// invalid combination (for example QuanShu Earth) returns
/// [`ChartError::UnsupportedChartPlane`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LunarChartRequest {
    lunar_year: i32,
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    birth_time: BirthTime,
    gender: Gender,
    birth_year_stem: HeavenlyStem,
    birth_year_branch: EarthlyBranch,
    is_leap_month: bool,
    fix_leap: bool,
    method_profile: MethodProfile,
    chart_plane: ChartPlane,
}

impl LunarChartRequest {
    /// Starts building a typed lunar chart request.
    ///
    /// Set each required field on the returned builder, then call
    /// [`LunarChartRequestBuilder::build`].
    pub fn builder() -> LunarChartRequestBuilder {
        LunarChartRequestBuilder::default()
    }

    /// Returns the provided lunar year.
    pub const fn lunar_year(&self) -> i32 {
        self.lunar_year
    }

    /// Returns the validated lunar month.
    pub const fn lunar_month(&self) -> LunarMonth {
        self.lunar_month
    }

    /// Returns the validated lunar day.
    pub const fn lunar_day(&self) -> LunarDay {
        self.lunar_day
    }

    /// Returns the birth time branch.
    pub const fn birth_time(&self) -> EarthlyBranch {
        self.birth_time.branch()
    }

    /// Returns the full birth-time variant.
    pub const fn birth_time_variant(&self) -> BirthTime {
        self.birth_time
    }

    /// Returns the gender marker.
    pub const fn gender(&self) -> Gender {
        self.gender
    }

    /// Returns the explicit birth year Heavenly Stem.
    pub const fn birth_year_stem(&self) -> HeavenlyStem {
        self.birth_year_stem
    }

    /// Returns the explicit birth year Earthly Branch.
    pub const fn birth_year_branch(&self) -> EarthlyBranch {
        self.birth_year_branch
    }

    /// Returns whether the lunar month is a leap month (闰月).
    pub const fn is_leap_month(&self) -> bool {
        self.is_leap_month
    }

    /// Returns whether leap-month adjustment is applied (调整闰月).
    pub const fn fix_leap(&self) -> bool {
        self.fix_leap
    }

    /// Returns the method profile metadata.
    pub const fn method_profile(&self) -> &MethodProfile {
        &self.method_profile
    }

    /// Returns the requested chart plane (天盘 / 地盘 / 人盘).
    ///
    /// Defaults to [`ChartPlane::Heaven`] when not set on the builder.
    pub const fn chart_plane(&self) -> ChartPlane {
        self.chart_plane
    }
}

/// Builder for [`LunarChartRequest`].
///
/// Each field is optional until set; [`build`](LunarChartRequestBuilder::build)
/// fails with [`ChartError::MissingRequiredInput`] if a required field is
/// missing, keeping construction explicit and deterministic.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LunarChartRequestBuilder {
    lunar_year: Option<i32>,
    lunar_month: Option<LunarMonth>,
    lunar_day: Option<LunarDay>,
    birth_time: Option<BirthTime>,
    gender: Option<Gender>,
    birth_year_stem: Option<HeavenlyStem>,
    birth_year_branch: Option<EarthlyBranch>,
    is_leap_month: Option<bool>,
    fix_leap: Option<bool>,
    method_profile: Option<MethodProfile>,
    chart_plane: Option<ChartPlane>,
}

impl LunarChartRequestBuilder {
    /// Sets the lunar year.
    pub fn lunar_year(mut self, value: i32) -> Self {
        self.lunar_year = Some(value);
        self
    }

    /// Sets the validated lunar month.
    pub fn lunar_month(mut self, value: LunarMonth) -> Self {
        self.lunar_month = Some(value);
        self
    }

    /// Sets the validated lunar day.
    pub fn lunar_day(mut self, value: LunarDay) -> Self {
        self.lunar_day = Some(value);
        self
    }

    /// Sets the birth time branch.
    pub fn birth_time(mut self, value: EarthlyBranch) -> Self {
        self.birth_time = Some(BirthTime::from_branch(value));
        self
    }

    /// Sets the full birth-time variant.
    pub fn birth_time_variant(mut self, value: BirthTime) -> Self {
        self.birth_time = Some(value);
        self
    }

    /// Sets the birth time from an upstream `iztro` `timeIndex`.
    pub fn iztro_time_index(mut self, value: u8) -> Result<Self, ChartError> {
        self.birth_time = Some(BirthTime::from_iztro_time_index(value)?);
        Ok(self)
    }

    /// Sets the gender marker.
    pub fn gender(mut self, value: Gender) -> Self {
        self.gender = Some(value);
        self
    }

    /// Sets the explicit birth year Heavenly Stem.
    pub fn birth_year_stem(mut self, value: HeavenlyStem) -> Self {
        self.birth_year_stem = Some(value);
        self
    }

    /// Sets the explicit birth year Earthly Branch.
    pub fn birth_year_branch(mut self, value: EarthlyBranch) -> Self {
        self.birth_year_branch = Some(value);
        self
    }

    /// Sets whether the lunar month is a leap month (闰月).
    ///
    /// Defaults to `false` when unset, preserving the original non-leap
    /// behavior.
    pub fn is_leap_month(mut self, value: bool) -> Self {
        self.is_leap_month = Some(value);
        self
    }

    /// Sets whether leap-month adjustment is applied (调整闰月).
    ///
    /// Defaults to `true` when unset, matching upstream `iztro@2.5.8`.
    pub fn fix_leap(mut self, value: bool) -> Self {
        self.fix_leap = Some(value);
        self
    }

    /// Sets the method profile metadata.
    pub fn method_profile(mut self, value: MethodProfile) -> Self {
        self.method_profile = Some(value);
        self
    }

    /// Sets the requested chart plane (天盘 / 地盘 / 人盘).
    ///
    /// Defaults to [`ChartPlane::Heaven`] when unset. `Earth` and `Human` are
    /// accepted but only valid for the Zhongzhou (中州) family, and Zhongzhou
    /// Earth/Human chart generation is not implemented yet.
    pub const fn chart_plane(mut self, chart_plane: ChartPlane) -> Self {
        self.chart_plane = Some(chart_plane);
        self
    }

    /// Builds the immutable request, requiring every field to be set.
    pub fn build(self) -> Result<LunarChartRequest, ChartError> {
        Ok(LunarChartRequest {
            lunar_year: self.lunar_year.ok_or(ChartError::MissingRequiredInput {
                field: "lunar_year",
            })?,
            lunar_month: self.lunar_month.ok_or(ChartError::MissingRequiredInput {
                field: "lunar_month",
            })?,
            lunar_day: self
                .lunar_day
                .ok_or(ChartError::MissingRequiredInput { field: "lunar_day" })?,
            birth_time: self.birth_time.ok_or(ChartError::MissingRequiredInput {
                field: "birth_time",
            })?,
            gender: self
                .gender
                .ok_or(ChartError::MissingRequiredInput { field: "gender" })?,
            birth_year_stem: self
                .birth_year_stem
                .ok_or(ChartError::MissingRequiredInput {
                    field: "birth_year_stem",
                })?,
            birth_year_branch: self
                .birth_year_branch
                .ok_or(ChartError::MissingRequiredInput {
                    field: "birth_year_branch",
                })?,
            is_leap_month: self.is_leap_month.unwrap_or(false),
            fix_leap: self.fix_leap.unwrap_or(true),
            method_profile: self
                .method_profile
                .ok_or(ChartError::MissingRequiredInput {
                    field: "method_profile",
                })?,
            chart_plane: self.chart_plane.unwrap_or_default(),
        })
    }
}

/// Builds a natal chart with currently supported natal stars from explicit lunar inputs.
///
/// This facade records the lunar date as chart input facts and delegates to the
/// existing strongly typed builder. It does not perform solar-to-lunar
/// conversion or year-to-ganzhi derivation; calendar conversion is provided by
/// [`by_solar`](crate::core::facade::by_solar::by_solar), which delegates here.
///
/// Leap-month behavior is explicit and upstream-compatible. The requested
/// `is_leap_month` is first resolved against the real calendar through an
/// internal normalizer (the flag is honored only when the requested month is
/// actually the year's leap month, mirroring upstream `lunar2solar`). The
/// recorded calendar date keeps the resolved lunar year/month/day, while
/// month-based star placement uses the effective month derived from
/// `effective_lunar_month` applied to the resolved leap state.
pub fn by_lunar(request: LunarChartRequest) -> Result<Chart, ChartError> {
    let strategy = select_natal_placement_strategy(
        request.method_profile().algorithm_kind(),
        request.chart_plane(),
    )?;

    let resolved = resolve_lunar_date(
        request.lunar_year(),
        request.lunar_month(),
        request.lunar_day(),
        request.is_leap_month(),
    )?;

    let birth_time = request.birth_time_variant();
    let effective_lunar_month = effective_lunar_month(
        resolved.lunar_month(),
        resolved.lunar_day(),
        resolved.is_leap_month(),
        request.fix_leap(),
        birth_time,
    )?;
    let major_lunar_day = major_lunar_day(resolved.lunar_day(), resolved.month_days(), birth_time)?;
    let daily_star_offset = daily_star_offset(resolved.lunar_day(), birth_time);
    let birth_year = StemBranch::try_new(request.birth_year_stem(), request.birth_year_branch())
        .map_err(|err| match err {
            lunar_lite::StemBranchError::InvalidStemBranchPair { stem, branch } => {
                ChartError::InvalidStemBranchPair { stem, branch }
            }
        })?;

    let birth_context = BirthContext::new_with_birth_time_variant(
        CalendarDate::lunar(
            resolved.lunar_year(),
            resolved.lunar_month().value(),
            resolved.lunar_day().value(),
        ),
        birth_time,
        request.gender(),
    );

    build_natal_chart_with_supported_stars_using(
        NatalChartWithSupportedStarsInput::new_with_daily_star_offset(
            birth_context,
            request.method_profile().clone(),
            effective_lunar_month,
            major_lunar_day,
            daily_star_offset,
            birth_year.stem(),
            birth_year.branch(),
        ),
        &strategy,
    )
}

/// Selects the natal star-placement strategy for an algorithm + chart plane.
///
/// This is the single centralized dispatch point that maps a requested
/// `(ChartAlgorithmKind, ChartPlane)` to a placement strategy. Keeping the
/// decision here means star placers never branch on [`ChartPlane`].
///
/// Behaviour:
/// - Invalid combinations (for example `QuanShu + Earth`) fail early with
///   [`ChartError::UnsupportedChartPlane`] via [`validate_chart_algorithm_plane`].
/// - `Zhongzhou + Earth` and `Zhongzhou + Human` are domain-valid but not
///   implemented yet, so they fail with [`ChartError::ChartPlaneNotImplemented`]
///   rather than silently falling back to the Heaven plane.
/// - Every implemented combination (any algorithm with `Heaven`) returns the
///   existing [`DeterministicNatalStarPlacementStrategy`].
///
/// PR 102 will extend this function with real Earth/Human strategies; until
/// then it is intentionally boring.
fn select_natal_placement_strategy(
    algorithm: ChartAlgorithmKind,
    plane: ChartPlane,
) -> Result<DeterministicNatalStarPlacementStrategy, ChartError> {
    validate_chart_algorithm_plane(algorithm, plane)?;

    match (algorithm, plane) {
        (ChartAlgorithmKind::Zhongzhou, ChartPlane::Earth | ChartPlane::Human) => {
            Err(ChartError::ChartPlaneNotImplemented { algorithm, plane })
        }
        _ => Ok(DeterministicNatalStarPlacementStrategy::default()),
    }
}

/// Computes the effective lunar month used for month-based star placement.
///
/// Mirrors upstream `iztro@2.5.8` `fixLunarMonthIndex`: when the birth month is a
/// leap month, leap-month adjustment is enabled, and the lunar day is in the
/// second half of the month (after the 15th), the effective month advances by
/// one. Otherwise a leap month is treated as the same numeric month, and a
/// non-leap month is always used as-is.
///
/// `is_leap_month` here is the **resolved** leap state from the internal
/// calendar normalizer, not the raw request flag, so an invalid leap request (a
/// month that is not actually leap that year) never advances the month.
///
/// A leap twelfth month would push the effective month past 12 into the next
/// lunar year, which is out of the supported slice, so it returns
/// [`ChartError::UnsupportedLeapMonthCombination`] rather than guessing.
fn effective_lunar_month(
    lunar_month: LunarMonth,
    lunar_day: LunarDay,
    is_leap_month: bool,
    fix_leap: bool,
    birth_time: BirthTime,
) -> Result<LunarMonth, ChartError> {
    let needs_advance =
        is_leap_month && fix_leap && lunar_day.value() > 15 && !birth_time.is_late_zi();
    if !needs_advance {
        return Ok(lunar_month);
    }

    LunarMonth::new(lunar_month.value() + 1).map_err(|_| {
        ChartError::UnsupportedLeapMonthCombination {
            lunar_month: lunar_month.value(),
            lunar_day: lunar_day.value(),
        }
    })
}

fn major_lunar_day(
    lunar_day: LunarDay,
    month_days: u8,
    birth_time: BirthTime,
) -> Result<LunarDay, ChartError> {
    if !birth_time.is_late_zi() {
        return Ok(lunar_day);
    }

    let next_day = if lunar_day.value() >= month_days {
        1
    } else {
        lunar_day.value() + 1
    };

    LunarDay::new(next_day)
}

fn daily_star_offset(lunar_day: LunarDay, birth_time: BirthTime) -> u8 {
    if birth_time.is_late_zi() {
        lunar_day.value()
    } else {
        lunar_day.value() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_builder(profile: MethodProfile) -> LunarChartRequestBuilder {
        LunarChartRequest::builder()
            .lunar_year(1990)
            .lunar_month(LunarMonth::new(5).expect("valid lunar month"))
            .lunar_day(LunarDay::new(17).expect("valid lunar day"))
            .birth_time(EarthlyBranch::Chen)
            .gender(Gender::Female)
            .birth_year_stem(HeavenlyStem::Geng)
            .birth_year_branch(EarthlyBranch::Wu)
            .method_profile(profile)
    }

    fn quanshu_profile() -> MethodProfile {
        MethodProfile::new("quanshu_test", ChartAlgorithmKind::QuanShu, "quanshu test")
    }

    fn zhongzhou_profile() -> MethodProfile {
        MethodProfile::new(
            "zhongzhou_test",
            ChartAlgorithmKind::Zhongzhou,
            "zhongzhou test",
        )
    }

    #[test]
    fn chart_plane_defaults_to_heaven() {
        let request = base_builder(quanshu_profile())
            .build()
            .expect("request should build");

        assert_eq!(request.chart_plane(), ChartPlane::Heaven);
    }

    #[test]
    fn explicit_chart_plane_is_preserved() {
        let request = base_builder(zhongzhou_profile())
            .chart_plane(ChartPlane::Earth)
            .build()
            .expect("request should build");

        assert_eq!(request.chart_plane(), ChartPlane::Earth);
    }

    #[test]
    fn default_request_matches_explicit_heaven_request() {
        let default_chart = by_lunar(
            base_builder(quanshu_profile())
                .build()
                .expect("default request should build"),
        )
        .expect("default by_lunar should build");

        let heaven_chart = by_lunar(
            base_builder(quanshu_profile())
                .chart_plane(ChartPlane::Heaven)
                .build()
                .expect("heaven request should build"),
        )
        .expect("heaven by_lunar should build");

        assert_eq!(default_chart, heaven_chart);
    }

    #[test]
    fn quanshu_earth_is_unsupported() {
        let request = base_builder(quanshu_profile())
            .chart_plane(ChartPlane::Earth)
            .build()
            .expect("request should build");

        assert_eq!(
            by_lunar(request),
            Err(ChartError::UnsupportedChartPlane {
                algorithm: ChartAlgorithmKind::QuanShu,
                plane: ChartPlane::Earth,
            }),
        );
    }

    #[test]
    fn quanshu_human_is_unsupported() {
        let request = base_builder(quanshu_profile())
            .chart_plane(ChartPlane::Human)
            .build()
            .expect("request should build");

        assert_eq!(
            by_lunar(request),
            Err(ChartError::UnsupportedChartPlane {
                algorithm: ChartAlgorithmKind::QuanShu,
                plane: ChartPlane::Human,
            }),
        );
    }

    #[test]
    fn zhongzhou_heaven_still_succeeds() {
        let request = base_builder(zhongzhou_profile())
            .chart_plane(ChartPlane::Heaven)
            .build()
            .expect("request should build");

        assert!(by_lunar(request).is_ok());
    }

    #[test]
    fn zhongzhou_earth_is_not_implemented() {
        let request = base_builder(zhongzhou_profile())
            .chart_plane(ChartPlane::Earth)
            .build()
            .expect("request should build");

        assert_eq!(
            by_lunar(request),
            Err(ChartError::ChartPlaneNotImplemented {
                algorithm: ChartAlgorithmKind::Zhongzhou,
                plane: ChartPlane::Earth,
            }),
        );
    }

    #[test]
    fn zhongzhou_human_is_not_implemented() {
        let request = base_builder(zhongzhou_profile())
            .chart_plane(ChartPlane::Human)
            .build()
            .expect("request should build");

        assert_eq!(
            by_lunar(request),
            Err(ChartError::ChartPlaneNotImplemented {
                algorithm: ChartAlgorithmKind::Zhongzhou,
                plane: ChartPlane::Human,
            }),
        );
    }
}
