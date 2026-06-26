//! Public iztro-compatible facade entry points.
//!
//! [`by_lunar()`] mirrors iztro's `astro.byLunar(...)` and [`by_solar()`] mirrors
//! `astro.bySolar(...)` conceptually, through the typed [`LunarChartRequest`] and
//! [`SolarChartRequest`] request objects.

pub mod by_lunar;
pub mod by_solar;
pub mod options;
pub mod static_temporal_chart_view;

pub use by_lunar::{LunarChartRequest, LunarChartRequestBuilder, by_lunar};
pub use by_solar::{SolarChartRequest, SolarChartRequestBuilder, by_solar};
pub use options::{
    LunarBirthInput, LunarDate, NatalChartOptions, SolarBirthInput, by_lunar_with_options,
    by_solar_with_options,
};
pub use static_temporal_chart_view::{
    static_temporal_chart_view, static_temporal_chart_view_from_chart,
    temporal_selection_for_local_moment, temporal_selection_for_solar_moment,
};
