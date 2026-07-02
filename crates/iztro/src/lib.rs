//! Strongly typed Zi Wei Dou Shu (紫微斗数) chart generation aligned with
//! [`iztro`](https://github.com/SylarLong/iztro).
//!
//! The crate keeps clear internal domain boundaries, implemented as modules:
//!
//! - [`core`] — core chart facts, value objects, deterministic placement and
//!   overlay builders, and the public iztro-compatible chart-generation facade;
//! - [`projection`] — GUI/API/CLI-facing static chart read models (projections);
//! - [`facade`] — orchestration that builds [`projection`] read models from core
//!   charts and a temporal navigation selection;
//! - [`features`] — feature extraction contracts derived from chart facts;
//! - [`rules`] — structured rule and claim contracts;
//! - [`analysis`] — lightweight, layer-level coordination over the pattern and
//!   classical rule engines for cacheable per-layer detection;
//! - [`reading`] — deterministic report structures and rendering contracts;
//! - [`render`] — deterministic renderers for chart snapshot read models.
//!
//! The flat re-exports below preserve the stable user-facing API regardless of
//! where a type or function lives internally.

pub mod analysis;
pub mod core;
pub mod facade;
pub mod features;
pub mod projection;
pub mod reading;
pub mod render;
pub mod rules;

pub use core::{
    AdjectiveStarPlacementInput, AdjectiveStarPlacer, AgePeriod, BirthContext, BirthTime,
    Brightness, CalendarDate, CalendarKind, Chart, ChartAlgorithmKind, ChartDiagnosticSnapshot,
    ChartError, ChartLayerKind, ChartLayerSnapshot, ChartPlane, ChartProfile, ChartStackSnapshot,
    CompositeNatalStarPlacementStrategy, DailyPeriod, DecadalDirection, DecadalFrame,
    DecadalHoroscopeInput, DecadalMutagenLayerInput, DecadalPeriod, DecorativeStarFamily,
    DecorativeStarPlacement, DecorativeStarPlacementInput, DecorativeStarPlacementRef,
    DecorativeStarSnapshot, DeterministicAdjectiveStarPlacer, DeterministicDecorativeStarPlacer,
    DeterministicMajorStarPlacer, DeterministicMinorStarPlacer,
    DeterministicNatalStarPlacementStrategy, EARTHLY_BRANCHES, EarthlyBranch, EffectiveChartState,
    EffectiveMutagenRef, EffectiveStarRef, FiveElementBureau, FlowStarBase, FlowStarScope,
    FourPillars, Gender, HEAVENLY_STEMS, HeavenlyStem, HoroscopeAgeSupportedFields, HoroscopeChart,
    HoroscopeFacadeContext, HoroscopeFacadeSnapshot, HoroscopeFlowScopeSupportedFields,
    HoroscopeFlowStarSupportedField, HoroscopeLunarDate, HoroscopeMutagenSupportedFields,
    HoroscopeMutagenTargetSupportedField, HoroscopePalaceNameSupportedField,
    HoroscopePalaceProjectionSnapshot, HoroscopeProjectionMutagenActivationSnapshot,
    HoroscopeScopeSupportedFields, HoroscopeSolarDate, HoroscopeStackInput,
    HoroscopeSupportedFieldsSnapshot, HoroscopeSurroundPalacesSnapshot, HoroscopeTargetContext,
    HoroscopeYearlyDecorativeStarSupportedField, HoroscopeYearlyDecorativeSupportedFields,
    HoroscopeYearlySupportedFields, HourlyPeriod, KnownStarFamily, KnownStarMetadata,
    LifeBodyPalaceIndices, LunarBirthContext, LunarChartRequest, LunarChartRequestBuilder,
    LunarDay, LunarMonth, MajorStarPlacementInput, MajorStarPlacementRef, MajorStarPlacer,
    MethodProfile, MinorStarPlacementInput, MinorStarPlacer, MonthlyPeriod, Mutagen,
    MutagenActivation, MutagenActivationSnapshot, NaYinElement, NatalChartInput,
    NatalChartWithMajorStarsInput, NatalChartWithSupportedStarsInput, NatalDateFacts,
    NatalFacadeDecorativeStarSnapshot, NatalFacadeFourPillarsSnapshot, NatalFacadePalaceRole,
    NatalFacadePalaceSnapshot, NatalFacadeSnapshot, NatalFacadeTypedStarSnapshot,
    NatalStarPlacementStrategy, PALACE_COUNT, PALACE_NAMES, Palace, PalaceDiagnosticSnapshot,
    PalaceGridPosition, PalaceLayerCellSnapshot, PalaceName, PalaceRoleKind, PalaceRoleSnapshot,
    Scope, ScopedDecorativeStarPlacement, ScopedStarPlacement, ScopedStarSnapshot,
    SolarChartRequest, SolarChartRequestBuilder, SolarDay, SolarMonth, StarCategory, StarKind,
    StarMetadata, StarName, StarPlacement, StarPlacementRef, StaticTemporalNavigationSelection,
    StemBranch, TemporalContext, TemporalLayer, TemporalPalaceLayout, TemporalPalaceName,
    TypedStarSnapshot, VISUAL_BRANCH_ORDER, WesternZodiac, YearlyMutagenLayerInput, YearlyPeriod,
    adjective_star_metadata, adjective_star_metadata_table, birth_year_major_star_mutagen,
    birth_year_star_mutagen, body_master, build_age_horoscope_layer, build_age_period,
    build_daily_horoscope_layer, build_daily_period, build_decadal_frame,
    build_decadal_horoscope_chart, build_decadal_horoscope_layer, build_decadal_mutagen_layer,
    build_empty_chart, build_flow_star_layer, build_full_horoscope_chart,
    build_full_horoscope_chart_report, build_hourly_horoscope_layer, build_hourly_period,
    build_minimal_natal_chart, build_monthly_horoscope_layer, build_monthly_period,
    build_natal_chart_with_major_stars, build_natal_chart_with_major_stars_using,
    build_natal_chart_with_supported_stars, build_natal_chart_with_supported_stars_using,
    build_yearly_decorative_star_placements, build_yearly_horoscope_layer,
    build_yearly_mutagen_layer, build_yearly_period, by_lunar, by_solar,
    calculate_life_body_palace_indices, five_element_bureau_from_life_palace, flow_star_name,
    is_valid_chart_algorithm_plane, known_star_metadata, known_star_metadata_table,
    major_star_brightness, major_star_metadata, major_star_metadata_table, minor_star_brightness,
    minor_star_metadata, minor_star_metadata_table, nayin_element, palace_grid_position,
    palace_stem_for_branch, palace_stems_from_year_stem, represented_star_metadata_table,
    soul_master, star_metadata, tian_fu_branch, try_adjective_star_metadata, try_flow_star_parts,
    try_known_star_metadata, try_major_star_metadata, try_minor_star_metadata, try_star_metadata,
    validate_chart_algorithm_plane, western_zodiac, zi_wei_branch,
};
// Shared selected-state context still lives in `core` for now. Pattern rules
// live under `rules::pattern`, but the headline pattern API remains available
// from the crate root.
pub use core::RuleEvaluationContext;
pub use rules::pattern::{
    PalaceRelation, PatternAnchor, PatternCondition, PatternContext, PatternDetection,
    PatternDetectionRequest, PatternDisplayMetadata, PatternEvidence, PatternFamily, PatternId,
    PatternPolarity, PatternScope, PatternSourceGroup, PatternSourceMetadata, PatternSpec,
    PatternStatus, PatternStrength, detect_patterns, pattern_display_metadata,
    pattern_source_metadata, pattern_spec, pattern_specs, try_pattern_spec,
};
// GUI/API/CLI-facing static chart projections and the orchestration facade that
// builds them. These read models moved out of `core` into the `projection` and
// `facade` modules to keep `core` owning domain facts only.
pub use facade::static_temporal_chart_view::{
    static_temporal_chart_view, static_temporal_chart_view_from_chart,
    temporal_selection_for_local_moment, temporal_selection_for_solar_moment,
};
pub use projection::static_chart::{
    HighlightProjection, LunarDateProjection, StaticChartCenterProjection, StaticChartProjection,
    StaticChartProjectionRequest, StaticChartSelectorProjection, StaticDecadalCellProjection,
    StaticDecorativeStarProjection, StaticFourPillarsProjection, StaticNavigationCellProjection,
    StaticOverlayMutagenProjection, StaticPalaceProjection, StaticPalaceRole,
    StaticPreDecadalCellProjection, StaticSurroundProjection, StaticTemporalOverlayProjection,
    StaticTemporalPanelProjection, StaticTypedStarProjection, StaticYearlyAgeCellProjection,
};
// Input calculation policy layer: clock-time birth input and apparent solar
// time. These are a separate axis from `ChartAlgorithmKind` and `ChartPlane`.
pub use core::{
    ApparentSolarTimeConfig, BirthInputCalendarKind, BirthTimeResolutionSnapshot,
    ChartCalculationConfig, ChartCalculationDiagnosticSnapshot, ClockBirthTime,
    EquationOfTimePolicy, HoroscopeCalculationDiagnosticSnapshot, HoroscopeGenerationReport,
    LeapMonthBoundary, LeapMonthBoundaryDiagnosticSnapshot, Longitude, LunarBirthInput, LunarDate,
    NatalChartGenerationReport, NatalChartOptions, NominalAgeBoundary, ResolvedBirthDateTime,
    SolarBirthInput, SolarDate, SolarTimePolicy, SolarTimePolicyDiagnostic, UtcOffset,
    YearBoundary, YearBoundaryDiagnosticSnapshot, by_lunar_with_options,
    by_lunar_with_options_report, by_solar_with_options, by_solar_with_options_report,
    resolve_lunar_birth_input, resolve_solar_birth_input,
};
// Classical rule engine (Chinese-first 《紫微斗数全书》 pilot). The full typed
// schema lives under [`rules::classical`]; these are the headline entry points.
pub use rules::classical::{
    ClaimEvaluation, ClaimEvaluationRequest, ClassicalRuleContext, ClassicalRuleHitRef,
    ClassicalRuleMetadata, classical_rule_metadata, evaluate_classical, evaluate_classical_claims,
    evaluate_classical_in_context,
};
// Lightweight layer-level analysis coordination. These compose the pattern and
// classical engines for cacheable per-layer detection; see [`analysis`].
pub use analysis::{
    AnalysisLayerKey, AnalysisLayerRequest, AnalysisLayerResult, TemporalAnalysisContext,
    analysis_layers_for_selection, analysis_scopes_for_layer_key, detect_analysis_layer,
    detect_static_temporal_analysis_layers_from_chart,
};
