//! Immutable chart facts: palace layout, the assembled natal [`Chart`], star
//! placements, and horoscope (運限) overlay models.

// `chart::chart` keeps the assembled `Chart` aggregate beside its palace and
// horoscope facts; the repeated path segment is the intentional module layout.
pub mod age;
#[allow(clippy::module_inception)]
pub mod chart;
pub mod daily;
pub mod decadal;
pub mod diagnostic;
pub mod facade_snapshot;
pub mod horoscope;
pub mod hourly;
pub mod monthly;
pub mod palace;
pub mod runtime;
pub mod snapshot;
pub mod supported_fields;
pub mod yearly;

mod temporal_layout;

pub(crate) use temporal_layout::{
    nominal_age_for_target, nominal_age_for_target_year, select_decadal_period_by_age,
    target_lunar_date,
};

pub use age::{AgePeriod, build_age_period};
pub use chart::{
    Chart, DecorativeStarFamily, DecorativeStarPlacement, DecorativeStarPlacementRef,
    MajorStarPlacementRef, NatalDateFacts, PALACE_COUNT, Palace, StarPlacement, StarPlacementRef,
};
pub use daily::{DailyPeriod, build_daily_period};
pub use decadal::{DecadalDirection, DecadalFrame, DecadalPeriod, build_decadal_frame};
pub use diagnostic::{ChartDiagnosticSnapshot, PalaceDiagnosticSnapshot};
pub use facade_snapshot::{
    HoroscopeFacadeContext, HoroscopeFacadeSnapshot, HoroscopePalaceProjectionSnapshot,
    HoroscopeProjectionMutagenActivationSnapshot, HoroscopeSurroundPalacesSnapshot,
    NatalFacadeDecorativeStarSnapshot, NatalFacadeFourPillarsSnapshot, NatalFacadePalaceRole,
    NatalFacadePalaceSnapshot, NatalFacadeSnapshot, NatalFacadeTypedStarSnapshot,
};
pub use horoscope::{
    HoroscopeChart, HoroscopeLunarDate, HoroscopeSolarDate, HoroscopeTargetContext,
    MutagenActivation, ScopedDecorativeStarPlacement, ScopedStarPlacement, TemporalContext,
    TemporalLayer, TemporalPalaceLayout, TemporalPalaceName,
};
pub use hourly::{HourlyPeriod, build_hourly_period};
pub use monthly::{MonthlyPeriod, build_monthly_period};
pub use palace::{PALACE_NAMES, PalaceName};
pub use runtime::{
    HoroscopePalaceProjection, HoroscopeProjectionMutagenActivation, HoroscopeRuntime,
    HoroscopeSurroundPalaces,
};
pub use snapshot::{
    ChartLayerKind, ChartLayerSnapshot, ChartStackSnapshot, DecorativeStarSnapshot,
    MutagenActivationSnapshot, PalaceGridPosition, PalaceLayerCellSnapshot, PalaceRoleKind,
    PalaceRoleSnapshot, ScopedStarSnapshot, TypedStarSnapshot, VISUAL_BRANCH_ORDER,
    palace_grid_position,
};
pub use supported_fields::{
    HoroscopeAgeSupportedFields, HoroscopeFlowScopeSupportedFields,
    HoroscopeFlowStarSupportedField, HoroscopeMutagenSupportedFields,
    HoroscopeMutagenTargetSupportedField, HoroscopePalaceNameSupportedField,
    HoroscopeScopeSupportedFields, HoroscopeSupportedFieldsSnapshot,
    HoroscopeYearlyDecorativeStarSupportedField, HoroscopeYearlyDecorativeSupportedFields,
    HoroscopeYearlySupportedFields,
};
pub use yearly::{YearlyPeriod, build_yearly_period};
