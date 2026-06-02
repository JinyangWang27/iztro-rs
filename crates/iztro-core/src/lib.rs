//! Core chart facts and strongly typed Zi Wei Dou Shu domain models.

pub mod builder;
pub mod calendar;
pub mod chart;
pub mod error;
pub mod ganzhi;
pub mod mutagen;
pub mod palace;
pub mod profile;
pub mod star;

pub use builder::build_empty_chart;
pub use calendar::{BirthContext, CalendarDate, CalendarKind, Gender};
pub use chart::{Chart, PALACE_COUNT, Palace, StarPlacement};
pub use error::ChartError;
pub use ganzhi::{EARTHLY_BRANCHES, EarthlyBranch, HEAVENLY_STEMS, HeavenlyStem};
pub use mutagen::{Mutagen, Scope};
pub use palace::{PALACE_NAMES, PalaceName};
pub use profile::{ChartAlgorithmKind, MethodProfile};
pub use star::{Brightness, StarCategory, StarName};
