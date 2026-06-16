//! Immutable chart facts: palace layout, the assembled natal [`Chart`], star
//! placements, and horoscope (運限) overlay models.

// `chart::chart` keeps the assembled `Chart` aggregate beside its palace and
// horoscope facts; the repeated path segment is the intentional module layout.
pub mod age;
#[allow(clippy::module_inception)]
pub mod chart;
pub mod daily;
pub mod decadal;
pub mod horoscope;
pub mod monthly;
pub mod palace;
pub mod snapshot;
pub mod yearly;

mod temporal_layout;

pub use age::{AgePeriod, build_age_period};
pub use chart::{
    Chart, DecorativeStarFamily, DecorativeStarPlacement, DecorativeStarPlacementRef,
    MajorStarPlacementRef, PALACE_COUNT, Palace, StarPlacement, StarPlacementRef,
};
pub use daily::{DailyPeriod, build_daily_period};
pub use decadal::{DecadalDirection, DecadalFrame, DecadalPeriod, build_decadal_frame};
pub use horoscope::{
    HoroscopeChart, MutagenActivation, ScopedStarPlacement, TemporalContext, TemporalLayer,
    TemporalPalaceLayout, TemporalPalaceName,
};
pub use monthly::{MonthlyPeriod, build_monthly_period};
pub use palace::{PALACE_NAMES, PalaceName};
pub use snapshot::{
    ChartLayerKind, ChartLayerSnapshot, ChartStackSnapshot, DecorativeStarSnapshot,
    MutagenActivationSnapshot, PalaceGridPosition, PalaceLayerCellSnapshot, PalaceRoleKind,
    PalaceRoleSnapshot, ScopedStarSnapshot, TypedStarSnapshot, VISUAL_BRANCH_ORDER,
    palace_grid_position,
};
pub use yearly::{YearlyPeriod, build_yearly_period};
