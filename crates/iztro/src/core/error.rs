use crate::core::model::chart::PalaceName;
use crate::core::model::star::StarName;
use crate::core::model::star::mutagen::{Mutagen, Scope};
use lunar_lite::{EarthlyBranch, HeavenlyStem, StemBranch};
use thiserror::Error;

/// Errors produced by core chart construction or validation.
#[derive(Debug, Error, Eq, PartialEq)]
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
    /// Placeholder error used until chart-generation validation exists.
    #[error("chart generation is not implemented")]
    NotImplemented,
}
