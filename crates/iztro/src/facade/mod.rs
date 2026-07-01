//! Orchestration facade for GUI/API/CLI-facing chart projections.
//!
//! These entry points build or receive core charts, resolve the selected temporal
//! layers, and assemble [`projection`](crate::projection) read models for a
//! renderer. Chart-generation entry points (`by_solar`/`by_lunar`/options) remain
//! in [`core::facade`](crate::core::facade); this module owns the projection-facing
//! orchestration only.

pub mod static_temporal_chart_view;

pub use static_temporal_chart_view::{
    static_temporal_chart_view, static_temporal_chart_view_from_chart,
    temporal_selection_for_local_moment, temporal_selection_for_solar_moment,
};
