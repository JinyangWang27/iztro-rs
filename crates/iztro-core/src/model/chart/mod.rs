//! Immutable chart facts: palace layout, the assembled natal [`Chart`], star
//! placements, and horoscope (運限) overlay models.

// `chart::chart` keeps the assembled `Chart` aggregate beside its palace and
// horoscope facts; the repeated path segment is the intentional module layout.
#[allow(clippy::module_inception)]
pub mod chart;
pub mod horoscope;
pub mod palace;

pub use chart::{
    Chart, DecorativeStarFamily, DecorativeStarPlacement, DecorativeStarPlacementRef,
    MajorStarPlacementRef, PALACE_COUNT, Palace, StarPlacement, StarPlacementRef,
};
pub use horoscope::{
    HoroscopeChart, MutagenActivation, ScopedStarPlacement, TemporalContext, TemporalLayer,
};
pub use palace::{PALACE_NAMES, PalaceName};
