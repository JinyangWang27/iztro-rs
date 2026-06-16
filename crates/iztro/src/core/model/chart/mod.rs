//! Immutable chart facts: palace layout, the assembled natal [`Chart`], star
//! placements, and horoscope (運限) overlay models.

// `chart::chart` keeps the assembled `Chart` aggregate beside its palace and
// horoscope facts; the repeated path segment is the intentional module layout.
#[allow(clippy::module_inception)]
pub mod chart;
pub mod decadal;
pub mod horoscope;
pub mod palace;
pub mod snapshot;

pub use chart::{
    Chart, DecorativeStarFamily, DecorativeStarPlacement, DecorativeStarPlacementRef,
    MajorStarPlacementRef, PALACE_COUNT, Palace, StarPlacement, StarPlacementRef,
};
pub use decadal::{DecadalDirection, DecadalFrame, DecadalPeriod, build_decadal_frame};
pub use horoscope::{
    HoroscopeChart, MutagenActivation, ScopedStarPlacement, TemporalContext, TemporalLayer,
    TemporalPalaceLayout, TemporalPalaceName,
};
pub use palace::{PALACE_NAMES, PalaceName};
pub use snapshot::{
    ChartLayerKind, ChartLayerSnapshot, ChartStackSnapshot, DecorativeStarSnapshot,
    MutagenActivationSnapshot, PalaceGridPosition, PalaceLayerCellSnapshot, PalaceRoleKind,
    PalaceRoleSnapshot, ScopedStarSnapshot, TypedStarSnapshot, VISUAL_BRANCH_ORDER,
    palace_grid_position,
};
