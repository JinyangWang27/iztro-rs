//! Public iztro-compatible facade entry points.
//!
//! [`by_lunar()`] mirrors iztro's `astro.byLunar(...)` and [`by_solar()`] mirrors
//! `astro.bySolar(...)` conceptually, through the typed [`LunarChartRequest`] and
//! [`SolarChartRequest`] request objects.

pub mod by_lunar;
pub mod by_solar;

pub use by_lunar::{LunarChartRequest, LunarChartRequestBuilder, by_lunar};
pub use by_solar::{SolarChartRequest, SolarChartRequestBuilder, by_solar};
