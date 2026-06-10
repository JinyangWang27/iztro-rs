//! Internal calendar-conversion adapters.
//!
//! This module isolates the third-party calendar dependency (ICU4X
//! `icu_calendar`) and lunar-date normalization behind internal adapters. ICU4X
//! and calendar-adapter types never leak past this boundary: facades consume
//! only the crate's own typed domain values and expose request/chart facts.

mod lunar_normalize;
mod solar_to_lunar;

pub(crate) use lunar_normalize::resolve_lunar_date;
pub(crate) use solar_to_lunar::solar_to_lunar;
