use crate::core::model::chart::PalaceName;
use crate::core::model::profile::{ChartAlgorithmKind, ChartPlane, is_valid_chart_algorithm_plane};
use crate::core::model::star::StarName;
use crate::core::model::star::mutagen::{Mutagen, Scope};
use lunar_lite::{EarthlyBranch, HeavenlyStem, StemBranch};
use thiserror::Error;

/// Errors produced by core chart construction or validation.
///
/// This type intentionally derives only [`PartialEq`] (not [`Eq`]): some
/// calculation-policy variants carry `f64` measurements (longitude correction,
/// equation-of-time minutes) that are comparable but not totally ordered.
#[derive(Debug, Error, PartialEq)]
pub enum ChartError {
    /// A chart must contain exactly the expected number of palaces.
    #[error("invalid palace count: expected {expected}, got {actual}")]
    InvalidPalaceCount {
        /// Expected number of palaces.
        expected: usize,
        /// Actual number of palaces.
        actual: usize,
    },
    /// Lunar month input must be in the supported non-leap range.
    #[error("invalid lunar month: expected 1..=12, got {value}")]
    InvalidLunarMonth {
        /// Unsupported lunar month value.
        value: u8,
    },
    /// Lunar day input must be in the supported range.
    #[error("invalid lunar day: expected 1..=30, got {value}")]
    InvalidLunarDay {
        /// Unsupported lunar day value.
        value: u8,
    },
    /// iztro `timeIndex` input must be in the supported `0..=12` range.
    #[error("invalid birth time index: expected 0..=12, got {value}")]
    InvalidBirthTimeIndex {
        /// Unsupported iztro time index value.
        value: u8,
    },
    /// Solar (Gregorian) month input must be in the `1..=12` range.
    #[error("invalid solar month: expected 1..=12, got {value}")]
    InvalidSolarMonth {
        /// Unsupported solar month value.
        value: u8,
    },
    /// Solar (Gregorian) day input must be in the `1..=31` range.
    ///
    /// This is a coarse range check; whether the day exists for the given month
    /// and year (for example 31 April or 29 February) is validated during
    /// calendar conversion and reported as [`ChartError::InvalidSolarDate`].
    #[error("invalid solar day: expected 1..=31, got {value}")]
    InvalidSolarDay {
        /// Unsupported solar day value.
        value: u8,
    },
    /// A solar (Gregorian) date is not a real calendar date.
    #[error("invalid solar date: {year}-{month}-{day}")]
    InvalidSolarDate {
        /// Gregorian year of the rejected date.
        year: i32,
        /// Gregorian month of the rejected date.
        month: u8,
        /// Gregorian day of the rejected date.
        day: u8,
    },
    /// A solar date falls outside the supported calendar-conversion range.
    #[error("unsupported calendar date: {year}-{month}-{day}")]
    UnsupportedCalendarDate {
        /// Gregorian year of the unsupported date.
        year: i32,
        /// Gregorian month of the unsupported date.
        month: u8,
        /// Gregorian day of the unsupported date.
        day: u8,
    },
    /// Gregorian-to-Chinese-lunisolar conversion did not yield usable lunar facts.
    #[error("calendar conversion failed for {year}-{month}-{day}")]
    CalendarConversionFailed {
        /// Gregorian year that failed to convert.
        year: i32,
        /// Gregorian month that failed to convert.
        month: u8,
        /// Gregorian day that failed to convert.
        day: u8,
    },
    /// A leap-month request cannot be represented by the supported chart slice.
    ///
    /// The supported slice models a leap month by shifting the effective lunar
    /// month forward by one for the second half of the month. A leap twelfth
    /// month would roll the effective month into the next lunar year, which is
    /// out of scope, so it is rejected rather than guessed.
    #[error(
        "unsupported leap-month combination: lunar month {lunar_month}, day {lunar_day} (effective month would exceed 12)"
    )]
    UnsupportedLeapMonthCombination {
        /// Lunar month of the rejected leap-month request.
        lunar_month: u8,
        /// Lunar day of the rejected leap-month request.
        lunar_day: u8,
    },
    /// A stem-branch pair must belong to the sexagenary cycle (matching parity).
    #[error("invalid sexagenary stem-branch pair: {stem:?}-{branch:?}")]
    InvalidStemBranchPair {
        /// Heavenly Stem of the rejected pair.
        stem: HeavenlyStem,
        /// Earthly Branch of the rejected pair.
        branch: EarthlyBranch,
    },
    /// A provided four-pillar fact must agree with the chart's retained birth year.
    #[error(
        "four-pillar year {four_pillars_year:?} does not match chart birth year {birth_year:?}"
    )]
    FourPillarsBirthYearMismatch {
        /// Retained chart birth-year fact.
        birth_year: StemBranch,
        /// Year pillar carried by the four-pillar fact.
        four_pillars_year: StemBranch,
    },
    /// A required builder input was not provided before `build`.
    #[error("missing required input: {field}")]
    MissingRequiredInput {
        /// Name of the missing required field.
        field: &'static str,
    },
    /// A star placement rule depends on a previously placed star that is absent.
    #[error("required star is missing: {star:?}")]
    RequiredStarMissing {
        /// Star required by the placement rule.
        star: StarName,
    },
    /// A placement rule depends on the Life or Body Palace, which is absent.
    #[error("required Life/Body Palace context is missing")]
    RequiredLifeBodyPalaceMissing,
    /// A placement rule depends on the five-element bureau, which is absent.
    #[error("required five-element bureau is missing")]
    RequiredFiveElementBureauMissing,
    /// A chart-plane re-anchor needs a palace identified by name that is absent.
    #[error("required palace {palace_name:?} is missing")]
    RequiredPalaceNameMissing {
        /// Palace name that could not be found in the chart.
        palace_name: PalaceName,
    },
    /// A required palace lookup found no palace occupying an earthly branch.
    #[error("required palace at branch {branch:?} is missing")]
    RequiredPalaceBranchMissing {
        /// Earthly branch that no palace occupies in the chart.
        branch: EarthlyBranch,
    },
    /// A decorative placement must name a known decorative (untyped) star whose
    /// family matches and whose known metadata carries no `StarKind`.
    #[error("invalid decorative star placement: {star:?}")]
    InvalidDecorativeStarPlacement {
        /// Star rejected as a decorative placement.
        star: StarName,
    },
    /// A temporal layer cannot use the natal scope; natal facts live in the chart.
    #[error("temporal layer cannot use the natal scope")]
    NatalScopeInTemporalLayer,
    /// An effective chart state must always include natal facts.
    #[error("effective chart state must include the natal scope")]
    EffectiveChartStateMissingNatalScope,
    /// An effective chart state cannot include the same active scope twice.
    #[error("effective chart state repeats active scope {scope:?}")]
    DuplicateEffectiveChartStateScope {
        /// Scope that appears more than once.
        scope: Scope,
    },
    /// Nominal-age periods support only a bounded human-age range.
    #[error("invalid nominal age: expected 1..=120, got {value}")]
    InvalidNominalAge {
        /// Unsupported nominal age value.
        value: u8,
    },
    /// Flow-star placement is unavailable for the requested temporal scope.
    #[error("flow-star placement is unavailable for scope {scope:?}")]
    FlowStarsUnavailableForScope {
        /// Scope that does not have flow-star placement support.
        scope: Scope,
    },
    /// A temporal layer's scope must match its temporal context.
    #[error("temporal layer scope {layer:?} does not match context scope {context:?}")]
    TemporalScopeMismatch {
        /// Scope declared on the layer.
        layer: Scope,
        /// Scope implied by the temporal context.
        context: Scope,
    },
    /// A scoped placement in a temporal layer must carry the layer's scope.
    #[error("temporal placement scope {placement:?} does not match layer scope {layer:?}")]
    TemporalPlacementScopeMismatch {
        /// Scope declared on the layer.
        layer: Scope,
        /// Scope carried by the rejected placement.
        placement: Scope,
    },
    /// A mutagen activation in a temporal layer must carry the layer's scope.
    #[error("temporal activation scope {activation:?} does not match layer scope {layer:?}")]
    TemporalActivationScopeMismatch {
        /// Scope declared on the layer.
        layer: Scope,
        /// Source scope carried by the rejected activation.
        activation: Scope,
    },
    /// A temporal palace-name layout must carry the layer's scope.
    #[error("temporal palace layout scope {layout:?} does not match layer scope {layer:?}")]
    TemporalPalaceLayoutScopeMismatch {
        /// Scope declared on the layer.
        layer: Scope,
        /// Scope carried by the rejected palace layout.
        layout: Scope,
    },
    /// A scoped decorative placement in a temporal layer must carry the layer's scope.
    #[error(
        "temporal decorative placement scope {decorative:?} does not match layer scope {layer:?}"
    )]
    TemporalDecorativeScopeMismatch {
        /// Scope declared on the layer.
        layer: Scope,
        /// Scope carried by the rejected decorative placement.
        decorative: Scope,
    },
    /// A temporal palace-name layout must contain exactly twelve names.
    #[error("invalid temporal palace layout count: expected {expected}, got {actual}")]
    InvalidTemporalPalaceLayoutCount {
        /// Expected number of temporal palace names.
        expected: usize,
        /// Actual number of temporal palace names.
        actual: usize,
    },
    /// Each branch in a temporal palace-name layout must appear exactly once.
    #[error("temporal palace layout repeats branch {branch:?}")]
    DuplicateTemporalPalaceLayoutBranch {
        /// Branch that appears more than once in the layout.
        branch: EarthlyBranch,
    },
    /// Each palace name in a temporal palace-name layout must appear exactly once.
    #[error("temporal palace layout repeats palace name {palace_name:?}")]
    DuplicateTemporalPalaceLayoutName {
        /// Palace name that appears more than once in the layout.
        palace_name: PalaceName,
    },
    /// A requested decadal period index is outside the derived decadal frame.
    #[error("invalid decadal period index: index {index} is out of range for {len} periods")]
    InvalidDecadalPeriodIndex {
        /// Requested zero-based decadal period index.
        index: usize,
        /// Number of periods available in the derived decadal frame.
        len: usize,
    },
    /// A temporal navigation selection carried an out-of-range child index.
    #[error("invalid temporal selection index {field}: expected 0..={max}, got {value}")]
    InvalidTemporalSelectionIndex {
        /// Field name, e.g. "year_index", "month_index", "day_index", "hour_index".
        field: &'static str,
        /// Provided value.
        value: u8,
        /// Inclusive maximum valid value.
        max: u8,
    },
    /// A requested analysis layer is not visible under the selected temporal view.
    ///
    /// Returned by the selected-view batch analysis facade
    /// (`detect_static_temporal_analysis_layers_from_chart`) when a requested
    /// analysis layer key does not exactly match any layer the current temporal
    /// selection makes visible — a wrong scope, a sibling index, or a mismatched
    /// ancestor index.
    #[error("analysis layer scope {scope:?} is not visible for the selected temporal view")]
    AnalysisLayerNotVisibleForSelection {
        /// Scope of the requested-but-not-visible analysis layer.
        scope: Scope,
    },
    /// A static chart projection request selects a non-natal active palace frame
    /// whose scope is not in the request's visible scopes.
    ///
    /// The active frame must be part of the visible temporal view: the projection
    /// refuses to render palace titles from a frame the request does not also
    /// mark visible, rather than silently adding the scope.
    #[error("active frame scope {scope:?} is not visible in the static chart projection request")]
    ActiveFrameScopeNotVisible {
        /// Scope selected as the active frame but absent from `visible_scopes`.
        scope: Scope,
    },
    /// A nominal age has no covering period in the derived decadal frame.
    #[error("nominal age {nominal_age} is outside the derived decadal frame")]
    NominalAgeOutsideDecadalFrame {
        /// Nominal age that no decadal period covers.
        nominal_age: u8,
    },
    /// A supported-fields export requires exactly one layer for each horoscope scope.
    #[error("missing horoscope layer for scope {scope:?}")]
    MissingHoroscopeLayer {
        /// Required scope that is absent.
        scope: Scope,
    },
    /// A supported-fields export cannot choose between repeated horoscope layers.
    #[error("duplicate horoscope layer for scope {scope:?}")]
    DuplicateHoroscopeLayer {
        /// Scope that appears more than once.
        scope: Scope,
    },
    /// A supported-fields export requires temporal palace names for every scope.
    #[error("missing horoscope palace layout for scope {scope:?}")]
    MissingHoroscopePalaceLayout {
        /// Scope with no palace-name layout.
        scope: Scope,
    },
    /// A runtime projection needs the natal palace occupying a branch.
    #[error("missing natal palace at branch {branch:?}")]
    MissingNatalPalaceForBranch {
        /// Branch with no natal palace.
        branch: EarthlyBranch,
    },
    /// A runtime projection needs a palace name in the selected scope.
    #[error("missing horoscope palace {palace_name:?} for scope {scope:?}")]
    MissingHoroscopePalaceName {
        /// Scope being projected.
        scope: Scope,
        /// Palace name that could not be found.
        palace_name: PalaceName,
    },
    /// A supported-fields export requires the four modeled mutagen activations.
    #[error("missing horoscope mutagen activation {mutagen:?} for scope {scope:?}")]
    MissingHoroscopeMutagenActivation {
        /// Scope with the missing activation.
        scope: Scope,
        /// Transform that has no activation in the layer.
        mutagen: Mutagen,
    },
    /// A temporal placement is not a normalized flow star for its horoscope scope.
    #[error("invalid horoscope flow star {star:?} for scope {scope:?}")]
    InvalidHoroscopeFlowStar {
        /// Scope being exported.
        scope: Scope,
        /// Star that cannot be normalized for that scope.
        star: StarName,
    },
    /// A requested non-leap lunar date could not be resolved to a solar date
    /// within the bounded scan window (for example lunar day 30 of a 29-day
    /// month, or a date outside the supported calendar-conversion range).
    #[error("unresolvable lunar date: {lunar_year}-{lunar_month}-{lunar_day}")]
    UnresolvableLunarDate {
        /// Lunar year of the unresolved date.
        lunar_year: i32,
        /// Lunar month (1..=12, non-leap) of the unresolved date.
        lunar_month: u8,
        /// Lunar day (1..=30) of the unresolved date.
        lunar_day: u8,
    },
    /// The requested chart plane is not supported for the given algorithm family.
    #[error("unsupported chart plane {plane:?} for algorithm {algorithm:?}")]
    UnsupportedChartPlane {
        /// Algorithm family that does not support the requested plane.
        algorithm: ChartAlgorithmKind,
        /// Chart plane that was rejected.
        plane: ChartPlane,
    },
    /// The requested chart plane is a domain-valid combination for the
    /// algorithm family, but chart generation for it is not implemented yet.
    ///
    /// This is distinct from [`ChartError::UnsupportedChartPlane`], which marks
    /// a combination that is semantically invalid (for example `QuanShu +
    /// Earth`). The Zhongzhou Earth and Human planes are now implemented and no
    /// longer return this error; the variant is retained for chart planes added
    /// in the future before their generation lands.
    #[error("chart generation for plane {plane:?} with algorithm {algorithm:?} is not implemented")]
    ChartPlaneNotImplemented {
        /// Algorithm family of the unimplemented plane.
        algorithm: ChartAlgorithmKind,
        /// Chart plane that is valid but not implemented yet.
        plane: ChartPlane,
    },
    /// A longitude must lie within `-180.0..=180.0` degrees.
    #[error("invalid longitude: expected -180.0..=180.0 degrees, got {value}")]
    InvalidLongitude {
        /// Rejected longitude in degrees.
        value: f64,
    },
    /// A UTC offset must lie within the sane real-world range
    /// `-12:00..=+14:00` (in minutes, `-720..=840`).
    #[error("invalid utc offset: expected -720..=840 minutes, got {minutes}")]
    InvalidUtcOffset {
        /// Rejected UTC offset in minutes.
        minutes: i32,
    },
    /// A wall-clock birth time must be a real 24-hour time.
    #[error("invalid clock time: expected hour 0..=23 and minute 0..=59, got {hour}:{minute}")]
    InvalidClockTime {
        /// Rejected hour value.
        hour: u8,
        /// Rejected minute value.
        minute: u8,
    },
    /// The requested equation-of-time policy is not implemented yet.
    ///
    /// Only [`EquationOfTimePolicy::Disabled`] (exact longitude correction with
    /// zero equation-of-time minutes) is currently supported.
    ///
    /// [`EquationOfTimePolicy::Disabled`]: crate::core::calculation::EquationOfTimePolicy::Disabled
    #[error("the requested equation-of-time policy is not supported yet")]
    UnsupportedEquationOfTimePolicy,
    /// Apparent solar time is an input calculation policy defined over a civil
    /// solar (Gregorian) date. It cannot be applied to a lunar-date input,
    /// because a longitude correction that crosses midnight would require lunar
    /// calendar day arithmetic that this slice does not perform.
    #[error("apparent solar time requires a solar (Gregorian) date input")]
    ApparentSolarTimeRequiresSolarDate,
    /// Placeholder error used until chart-generation validation exists.
    #[error("chart generation is not implemented")]
    NotImplemented,
}

/// Validates that `plane` is a domain-valid chart plane for `algorithm`.
///
/// Returns `Ok(())` if the combination is semantically recognised.
/// Returns `Err(ChartError::UnsupportedChartPlane)` if the combination is
/// definitively unsupported.
///
/// Domain validity does not imply implementation readiness. For example,
/// `Zhongzhou + Earth` returns `Ok(())` even though Zhongzhou 地盘 chart
/// generation is not yet implemented.
pub fn validate_chart_algorithm_plane(
    algorithm: ChartAlgorithmKind,
    plane: ChartPlane,
) -> Result<(), ChartError> {
    if is_valid_chart_algorithm_plane(algorithm, plane) {
        Ok(())
    } else {
        Err(ChartError::UnsupportedChartPlane { algorithm, plane })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quanshu_heaven_validates() {
        assert_eq!(
            validate_chart_algorithm_plane(ChartAlgorithmKind::QuanShu, ChartPlane::Heaven),
            Ok(()),
        );
    }

    #[test]
    fn quanshu_earth_errors() {
        assert_eq!(
            validate_chart_algorithm_plane(ChartAlgorithmKind::QuanShu, ChartPlane::Earth),
            Err(ChartError::UnsupportedChartPlane {
                algorithm: ChartAlgorithmKind::QuanShu,
                plane: ChartPlane::Earth,
            }),
        );
    }

    #[test]
    fn zhongzhou_earth_validates() {
        assert_eq!(
            validate_chart_algorithm_plane(ChartAlgorithmKind::Zhongzhou, ChartPlane::Earth),
            Ok(()),
        );
    }
}
