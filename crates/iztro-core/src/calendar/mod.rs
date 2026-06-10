//! Internal calendar-conversion adapters.
//!
//! This module isolates the third-party calendar dependency (ICU4X
//! `icu_calendar`) behind a small adapter surface. ICU4X types never leak past
//! this boundary: [`solar_to_lunar`] returns a [`LunarConversion`] built from the
//! crate's own strongly typed domain values. The public facade
//! ([`crate::facade::by_solar`]) consumes only that typed output.

mod lunar_normalize;
mod solar_to_lunar;

pub use lunar_normalize::{ResolvedLunarDate, resolve_lunar_date};
pub(crate) use solar_to_lunar::solar_to_lunar;
