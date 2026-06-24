//! Internal calendar-conversion adapters.
//!
//! This module isolates the third-party calendar engine (`tyme4rs`) and
//! lunar-date normalization behind internal adapters. Calendar-backend types
//! never leak past this boundary: [`tyme`] is the only module that depends on
//! `tyme4rs`, [`policy`] applies the `iztro-rs`-owned chart calendar policy, and
//! facades consume only the crate's own typed domain values and chart facts.

mod lunar_normalize;
mod lunar_target;
mod policy;
mod solar_to_lunar;
mod tyme;

pub(crate) use lunar_normalize::resolve_lunar_date;
pub(crate) use lunar_target::{
    ResolvedTemporalTarget, lunar_month_has_thirtieth, resolve_non_leap_lunar,
};
pub(crate) use solar_to_lunar::{lunar_facts, solar_to_lunar, solar_to_lunar_with_year_boundary};
pub(crate) use tyme::LunarDateInfo;
