//! Strongly typed Zi Wei Dou Shu (紫微斗数) chart generation aligned with
//! [`iztro`](https://github.com/SylarLong/iztro).
//!
//! The crate keeps clear internal domain boundaries, implemented as modules:
//!
//! - [`core`] — core chart facts, value objects, deterministic placement and
//!   overlay builders, and the public iztro-compatible facade entry points;
//! - [`features`] — feature extraction contracts derived from chart facts;
//! - [`rules`] — structured rule and claim contracts;
//! - [`reading`] — deterministic report structures and rendering contracts;
//! - [`render`] — deterministic renderers for chart snapshot read models.
//!
//! The flat re-exports below preserve the stable user-facing API regardless of
//! where a type or function lives internally.

pub mod core;
pub mod features;
pub mod reading;
pub mod render;
pub mod rules;

pub use core::{
    AdjectiveStarPlacementInput, AdjectiveStarPlacer, AgePeriod, BirthContext, BirthTime,
    Brightness, CalendarDate, CalendarKind, Chart, ChartAlgorithmKind, ChartError, ChartLayerKind,
    ChartLayerSnapshot, ChartStackSnapshot, DailyPeriod, DecadalDirection, DecadalFrame,
    DecadalHoroscopeInput, DecadalMutagenLayerInput, DecadalPeriod, DecorativeStarFamily,
    DecorativeStarPlacement, DecorativeStarPlacementInput, DecorativeStarPlacementRef,
    DecorativeStarSnapshot, DeterministicAdjectiveStarPlacer, DeterministicDecorativeStarPlacer,
    DeterministicMajorStarPlacer, DeterministicMinorStarPlacer, EARTHLY_BRANCHES, EarthlyBranch,
    FiveElementBureau, FlowStarBase, FlowStarScope, Gender, HEAVENLY_STEMS, HeavenlyStem,
    HoroscopeAgeSupportedFields, HoroscopeChart, HoroscopeFacadeContext, HoroscopeFacadeSnapshot,
    HoroscopeFlowScopeSupportedFields, HoroscopeFlowStarSupportedField, HoroscopeLunarDate,
    HoroscopeMutagenSupportedFields, HoroscopeMutagenTargetSupportedField,
    HoroscopePalaceNameSupportedField, HoroscopePalaceProjectionSnapshot,
    HoroscopeProjectionMutagenActivationSnapshot, HoroscopeScopeSupportedFields,
    HoroscopeSolarDate, HoroscopeStackInput, HoroscopeSupportedFieldsSnapshot,
    HoroscopeSurroundPalacesSnapshot, HoroscopeTargetContext,
    HoroscopeYearlyDecorativeStarSupportedField, HoroscopeYearlyDecorativeSupportedFields,
    HoroscopeYearlySupportedFields, HourlyPeriod, KnownStarFamily, KnownStarMetadata,
    LifeBodyPalaceIndices, LunarBirthContext, LunarChartRequest, LunarChartRequestBuilder,
    LunarDay, LunarMonth, MajorStarPlacementInput, MajorStarPlacementRef, MajorStarPlacer,
    MethodProfile, MinorStarPlacementInput, MinorStarPlacer, MonthlyPeriod, Mutagen,
    MutagenActivation, MutagenActivationSnapshot, NaYinElement, NatalChartInput,
    NatalChartWithMajorStarsInput, NatalChartWithSupportedStarsInput, PALACE_COUNT, PALACE_NAMES,
    Palace, PalaceGridPosition, PalaceLayerCellSnapshot, PalaceName, PalaceRoleKind,
    PalaceRoleSnapshot, Scope, ScopedDecorativeStarPlacement, ScopedStarPlacement,
    ScopedStarSnapshot, SolarChartRequest, SolarChartRequestBuilder, SolarDay, SolarMonth,
    StarCategory, StarKind, StarMetadata, StarName, StarPlacement, StarPlacementRef, StemBranch,
    TemporalContext, TemporalLayer, TemporalPalaceLayout, TemporalPalaceName, TypedStarSnapshot,
    VISUAL_BRANCH_ORDER, YearlyMutagenLayerInput, YearlyPeriod, adjective_star_metadata,
    adjective_star_metadata_table, birth_year_major_star_mutagen, birth_year_star_mutagen,
    build_age_horoscope_layer, build_age_period, build_daily_horoscope_layer, build_daily_period,
    build_decadal_frame, build_decadal_horoscope_chart, build_decadal_horoscope_layer,
    build_decadal_mutagen_layer, build_empty_chart, build_flow_star_layer,
    build_full_horoscope_chart, build_hourly_horoscope_layer, build_hourly_period,
    build_minimal_natal_chart, build_monthly_horoscope_layer, build_monthly_period,
    build_natal_chart_with_major_stars, build_natal_chart_with_supported_stars,
    build_yearly_decorative_star_placements, build_yearly_horoscope_layer,
    build_yearly_mutagen_layer, build_yearly_period, by_lunar, by_solar,
    calculate_life_body_palace_indices, five_element_bureau_from_life_palace, flow_star_name,
    known_star_metadata, known_star_metadata_table, major_star_brightness, major_star_metadata,
    major_star_metadata_table, minor_star_brightness, minor_star_metadata,
    minor_star_metadata_table, nayin_element, palace_grid_position, palace_stem_for_branch,
    palace_stems_from_year_stem, represented_star_metadata_table, star_metadata, tian_fu_branch,
    try_adjective_star_metadata, try_flow_star_parts, try_known_star_metadata,
    try_major_star_metadata, try_minor_star_metadata, try_star_metadata, zi_wei_branch,
};
