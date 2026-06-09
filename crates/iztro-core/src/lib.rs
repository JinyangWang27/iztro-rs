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

pub mod error;
pub mod facade;
pub mod feature;
pub mod model;
pub mod placement;

pub use error::ChartError;

pub use model::bureau::{FiveElementBureau, five_element_bureau_from_life_palace};
pub use model::calendar::{BirthContext, CalendarDate, CalendarKind, Gender};
pub use model::chart::{
    Chart, HoroscopeChart, MajorStarPlacementRef, MutagenActivation, PALACE_COUNT, PALACE_NAMES,
    Palace, PalaceName, StarPlacement, StarPlacementRef, TemporalContext, TemporalLayer,
};
pub use model::ganzhi::{EARTHLY_BRANCHES, EarthlyBranch, HEAVENLY_STEMS, HeavenlyStem};
pub use model::profile::{ChartAlgorithmKind, MethodProfile};
pub use model::sexagenary::{NaYinElement, StemBranch, is_valid_sexagenary_pair, nayin_element};
pub use model::star::mutagen::{
    Mutagen, Scope, birth_year_major_star_mutagen, birth_year_star_mutagen,
};
pub use model::star::{
    Brightness, KnownStarFamily, KnownStarMetadata, StarCategory, StarKind, StarMetadata, StarName,
    known_star_metadata, known_star_metadata_table, represented_star_metadata_table, star_metadata,
    try_known_star_metadata, try_star_metadata,
};

pub use placement::natal::adjective::{
    AdjectiveStarPlacementInput, AdjectiveStarPlacer, DeterministicAdjectiveStarPlacer,
    adjective_star_metadata, adjective_star_metadata_table, try_adjective_star_metadata,
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
pub use placement::natal::minimal::{build_empty_chart, build_minimal_natal_chart};
pub use placement::natal::minor::{
    DeterministicMinorStarPlacer, MinorStarPlacementInput, MinorStarPlacer, minor_star_brightness,
    minor_star_metadata, minor_star_metadata_table, try_minor_star_metadata,
};
pub use placement::natal::palace_stems::{palace_stem_for_branch, palace_stems_from_year_stem};
pub use placement::natal::supported::{
    build_natal_chart_with_major_stars, build_natal_chart_with_supported_stars,
};
pub use placement::overlay::decadal::{DecadalMutagenLayerInput, build_decadal_mutagen_layer};
pub use placement::overlay::yearly::{YearlyMutagenLayerInput, build_yearly_mutagen_layer};

pub use facade::by_lunar::{LunarChartRequest, LunarChartRequestBuilder, by_lunar};
