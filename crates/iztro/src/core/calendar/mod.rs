//! Internal calendar-conversion adapters.
//!
//! This module isolates the third-party calendar dependency (`lunar-lite`) and
//! lunar-date normalization behind internal adapters. Calendar-backend types
//! never leak past this boundary: facades consume only the crate's own typed
//! domain values and expose request/chart facts.

mod lunar_normalize;
mod lunar_target;
mod solar_to_lunar;

pub(crate) use lunar_normalize::resolve_lunar_date;
pub(crate) use lunar_target::{
    ResolvedTemporalTarget, lunar_month_has_thirtieth, resolve_non_leap_lunar,
};
pub(crate) use solar_to_lunar::{
    resolve_effective_birth_year, solar_to_lunar, solar_to_lunar_with_year_boundary,
};
