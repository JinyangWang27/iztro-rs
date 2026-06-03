//! Core chart facts and strongly typed Zi Wei Dou Shu domain models.

pub mod builder;
pub mod bureau;
pub mod calendar;
pub mod chart;
pub mod error;
pub mod ganzhi;
pub mod life_body;
pub mod major_stars;
pub mod mutagen;
pub mod natal;
pub mod palace;
pub mod palace_stems;
pub mod profile;
pub mod sexagenary;
pub mod star;

pub use builder::build_empty_chart;
pub use bureau::{FiveElementBureau, five_element_bureau_from_life_palace};
pub use calendar::{BirthContext, CalendarDate, CalendarKind, Gender};
pub use chart::{Chart, PALACE_COUNT, Palace, StarPlacement};
pub use error::ChartError;
pub use ganzhi::{EARTHLY_BRANCHES, EarthlyBranch, HEAVENLY_STEMS, HeavenlyStem};
pub use life_body::{
    LifeBodyPalaceIndices, LunarBirthContext, LunarDay, LunarMonth,
    calculate_life_body_palace_indices,
};
pub use major_stars::{
    DeterministicMajorStarPlacer, MajorStarPlacementInput, MajorStarPlacer, tian_fu_branch,
    zi_wei_branch,
};
pub use mutagen::{Mutagen, Scope};
pub use natal::{NatalChartInput, build_minimal_natal_chart};
pub use palace::{PALACE_NAMES, PalaceName};
pub use palace_stems::{palace_stem_for_branch, palace_stems_from_year_stem};
pub use profile::{ChartAlgorithmKind, MethodProfile};
pub use sexagenary::{NaYinElement, StemBranch, is_valid_sexagenary_pair, nayin_element};
pub use star::{Brightness, StarCategory, StarName};
