//! Core chart facts and strongly typed Zi Wei Dou Shu domain models.
//!
//! The crate is organized into domain modules:
//!
//! - [`model`] — value objects, star facts, and immutable chart facts;
//! - [`placement`] — deterministic 安星 placement and overlay activation
//!   builders;
//! - [`facade`] — public iztro-compatible entry points;
//! - [`feature`] — boundary for future derived-fact extraction helpers;
//! - [`error`] — the cross-cutting crate error type.
//!
//! The flat re-exports below preserve the stable public API regardless of where
//! a type or function lives internally.

pub mod calculation;
pub mod error;
pub mod facade;
pub mod feature;
pub mod labels;
pub mod model;
pub mod pattern;
pub mod placement;
pub mod view;

// Internal calendar-conversion and normalization adapters. Calendar-backend
// types are isolated here and never exposed in the public API; public callers
// use the facade request types and chart facts instead.
mod calendar;

pub use error::{ChartError, validate_chart_algorithm_plane};

pub use calculation::{
    ApparentSolarTimeConfig, BirthInputCalendarKind, BirthTimeResolutionSnapshot,
    ChartCalculationConfig, ChartCalculationDiagnosticSnapshot, ClockBirthTime,
    EquationOfTimePolicy, HoroscopeCalculationDiagnosticSnapshot, LeapMonthBoundary,
    LeapMonthBoundaryDiagnosticSnapshot, Longitude, NominalAgeBoundary, ResolvedBirthDateTime,
    SolarTimePolicy, SolarTimePolicyDiagnostic, UtcOffset, YearBoundary,
    YearBoundaryDiagnosticSnapshot,
};
pub use lunar_lite::{
    EARTHLY_BRANCHES, EarthlyBranch, FourPillars, HEAVENLY_STEMS, HeavenlyStem, StemBranch,
    StemBranchError,
};
pub use model::bureau::{FiveElementBureau, five_element_bureau_from_life_palace};
pub use model::calendar::{
    BirthContext, BirthTime, CalendarDate, CalendarKind, Gender, SolarDate, SolarDay, SolarMonth,
};
pub use model::chart::{
    AgePeriod, Chart, ChartDiagnosticSnapshot, ChartLayerKind, ChartLayerSnapshot,
    ChartStackSnapshot, DailyPeriod, DecadalDirection, DecadalFrame, DecadalPeriod,
    DecorativeStarFamily, DecorativeStarPlacement, DecorativeStarPlacementRef,
    DecorativeStarSnapshot, HoroscopeAgeSupportedFields, HoroscopeChart, HoroscopeFacadeContext,
    HoroscopeFacadeSnapshot, HoroscopeFlowScopeSupportedFields, HoroscopeFlowStarSupportedField,
    HoroscopeLunarDate, HoroscopeMutagenSupportedFields, HoroscopeMutagenTargetSupportedField,
    HoroscopePalaceNameSupportedField, HoroscopePalaceProjection,
    HoroscopePalaceProjectionSnapshot, HoroscopeProjectionMutagenActivation,
    HoroscopeProjectionMutagenActivationSnapshot, HoroscopeRuntime, HoroscopeScopeSupportedFields,
    HoroscopeSolarDate, HoroscopeSupportedFieldsSnapshot, HoroscopeSurroundPalaces,
    HoroscopeSurroundPalacesSnapshot, HoroscopeTargetContext,
    HoroscopeYearlyDecorativeStarSupportedField, HoroscopeYearlyDecorativeSupportedFields,
    HoroscopeYearlySupportedFields, HourlyPeriod, MajorStarPlacementRef, MonthlyPeriod,
    MutagenActivation, MutagenActivationSnapshot, NatalDateFacts,
    NatalFacadeDecorativeStarSnapshot, NatalFacadeFourPillarsSnapshot, NatalFacadePalaceRole,
    NatalFacadePalaceSnapshot, NatalFacadeSnapshot, NatalFacadeTypedStarSnapshot, PALACE_COUNT,
    PALACE_NAMES, Palace, PalaceDiagnosticSnapshot, PalaceGridPosition, PalaceLayerCellSnapshot,
    PalaceName, PalaceRoleKind, PalaceRoleSnapshot, ScopedDecorativeStarPlacement,
    ScopedStarPlacement, ScopedStarSnapshot, StarPlacement, StarPlacementRef, TemporalContext,
    TemporalLayer, TemporalPalaceLayout, TemporalPalaceName, TypedStarSnapshot,
    VISUAL_BRANCH_ORDER, YearlyPeriod, build_age_period, build_daily_period, build_decadal_frame,
    build_hourly_period, build_monthly_period, build_yearly_period, palace_grid_position,
};
pub use model::master::{body_master, soul_master};
pub use model::nayin::{NaYinElement, nayin_element};
pub use model::profile::{
    ChartAlgorithmKind, ChartPlane, ChartProfile, MethodProfile, is_valid_chart_algorithm_plane,
};
pub use model::star::mutagen::{
    Mutagen, Scope, birth_year_major_star_mutagen, birth_year_star_mutagen,
};
pub use model::star::{
    Brightness, FlowStarBase, FlowStarScope, KnownStarFamily, KnownStarMetadata, StarCategory,
    StarKind, StarMetadata, StarName, StarTag, StarTagStrength, flow_star_name, has_star_tag,
    known_star_metadata, known_star_metadata_table, represented_star_metadata_table, star_metadata,
    star_tag_strength, try_flow_star_parts, try_known_star_metadata, try_star_metadata,
};
pub use model::zodiac::{WesternZodiac, western_zodiac};
pub use pattern::{
    PalaceRelation, PatternAnchor, PatternCondition, PatternContext, PatternDetection,
    PatternDetectionRequest, PatternEvidence, PatternFamily, PatternId, PatternPolarity,
    PatternScope, PatternSourceGroup, PatternSourceMetadata, PatternStatus, PatternStrength,
    detect_patterns, pattern_source_metadata,
};

pub use placement::natal::adjective::{
    AdjectiveStarPlacementInput, AdjectiveStarPlacer, DeterministicAdjectiveStarPlacer,
    adjective_star_metadata, adjective_star_metadata_table, try_adjective_star_metadata,
};
pub use placement::natal::decorative::{
    DecorativeStarPlacementInput, DecorativeStarPlacer, DeterministicDecorativeStarPlacer,
};
pub use placement::natal::input::{
    NatalChartInput, NatalChartWithMajorStarsInput, NatalChartWithSupportedStarsInput,
};
pub use placement::natal::life_body::{
    LifeBodyPalaceIndices, LunarBirthContext, LunarDay, LunarMonth,
    calculate_life_body_palace_indices,
};
pub use placement::natal::major::{
    DeterministicMajorStarPlacer, MajorStarPlacementInput, MajorStarPlacer, major_star_brightness,
    major_star_metadata, major_star_metadata_table, tian_fu_branch, try_major_star_metadata,
    zi_wei_branch,
};
pub use placement::natal::minimal::{
    NatalChartAnchor, build_empty_chart, build_minimal_natal_chart,
    build_minimal_natal_chart_with_anchor,
};
pub use placement::natal::minor::{
    DeterministicMinorStarPlacer, MinorStarPlacementInput, MinorStarPlacer, minor_star_brightness,
    minor_star_metadata, minor_star_metadata_table, try_minor_star_metadata,
};
pub use placement::natal::palace_stems::{palace_stem_for_branch, palace_stems_from_year_stem};
pub use placement::natal::strategy::{
    CompositeNatalStarPlacementStrategy, DeterministicNatalStarPlacementStrategy,
    NatalStarPlacementStrategy,
};
pub use placement::natal::supported::{
    build_natal_chart_with_major_stars, build_natal_chart_with_major_stars_using,
    build_natal_chart_with_supported_stars, build_natal_chart_with_supported_stars_using,
    build_natal_chart_with_supported_stars_using_anchor_and_strategy,
};
pub use placement::overlay::age::build_age_horoscope_layer;
pub use placement::overlay::daily_horoscope::build_daily_horoscope_layer;
pub use placement::overlay::decadal::{DecadalMutagenLayerInput, build_decadal_mutagen_layer};
pub use placement::overlay::decadal_horoscope::{
    DecadalHoroscopeInput, build_decadal_horoscope_chart, build_decadal_horoscope_layer,
};
pub use placement::overlay::flow::build_flow_star_layer;
pub use placement::overlay::horoscope_stack::{
    HoroscopeGenerationReport, HoroscopeStackInput, build_full_horoscope_chart,
    build_full_horoscope_chart_report,
};
pub use placement::overlay::hourly_horoscope::build_hourly_horoscope_layer;
pub use placement::overlay::monthly_horoscope::build_monthly_horoscope_layer;
pub use placement::overlay::yearly::{YearlyMutagenLayerInput, build_yearly_mutagen_layer};
pub use placement::overlay::yearly_decorative::build_yearly_decorative_star_placements;
pub use placement::overlay::yearly_horoscope::build_yearly_horoscope_layer;

pub use facade::by_lunar::{LunarChartRequest, LunarChartRequestBuilder, by_lunar};
pub use facade::by_solar::{SolarChartRequest, SolarChartRequestBuilder, by_solar};
pub use facade::options::{
    LunarBirthInput, LunarDate, NatalChartGenerationReport, NatalChartOptions, SolarBirthInput,
    by_lunar_with_options, by_lunar_with_options_report, by_solar_with_options,
    by_solar_with_options_report, resolve_lunar_birth_input, resolve_solar_birth_input,
};
pub use facade::static_temporal_chart_view::{
    static_temporal_chart_view, static_temporal_chart_view_from_chart,
    temporal_selection_for_local_moment, temporal_selection_for_solar_moment,
};

pub use view::static_chart::{
    HighlightView, LunarDateView, StaticChartCenterView, StaticChartSelectorView,
    StaticChartViewRequest, StaticChartViewSnapshot, StaticDecadalCellView,
    StaticDecorativeStarView, StaticFourPillarsView, StaticNavigationCellView,
    StaticOverlayMutagenView, StaticPalaceRole, StaticPalaceView, StaticPreDecadalCellView,
    StaticSurroundPalacesView, StaticTemporalNavigationSelection, StaticTemporalOverlayView,
    StaticTemporalPanelView, StaticTypedStarView, StaticYearlyAgeCellView,
};
