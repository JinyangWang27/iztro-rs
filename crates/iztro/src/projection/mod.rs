//! GUI/API/CLI-facing static chart projections.
//!
//! Where [`facade_snapshot`](crate::core::model::chart::facade_snapshot) DTOs are
//! compatibility/export payloads, the projections here are *GUI-facing read
//! models*: one selected "chart slice" arranged for a static 12-palace chart,
//! carrying Chinese labels, grouped star lists, temporal overlays, scope-selector
//! state, and reserved highlight annotations.
//!
//! A projection describes *what* a renderer should show. It never chooses CSS
//! classes, colors, canvas coordinates, camera position, animation, or 3D
//! geometry — that is the renderer's responsibility. The same
//! [`StaticChartProjection`] is intended to be reusable later as one frame in a
//! timeline/3D sequence.
//!
//! These projections derive facts from an already-assembled [`Chart`](crate::Chart)
//! or [`HoroscopeChart`](crate::HoroscopeChart). They never mutate natal facts,
//! derive temporal periods, evaluate rules, or detect 成格 patterns. A branch is
//! the stable palace-cell identity; a palace *name* is frame-relative, so a
//! projection carries both an immutable natal identity and an active-frame
//! identity for each branch.

pub mod static_chart;

pub use static_chart::{
    HighlightProjection, StaticChartCenterProjection, StaticChartProjection,
    StaticChartProjectionRequest, StaticChartSelectorProjection, StaticDecadalCellProjection,
    StaticDecorativeStarProjection, StaticFourPillarsProjection, StaticNavigationCellProjection,
    StaticOverlayMutagenProjection, StaticPalaceProjection, StaticPalaceRole,
    StaticPreDecadalCellProjection, StaticSurroundProjection, StaticTemporalOverlayProjection,
    StaticTemporalPanelProjection, StaticTypedStarProjection, StaticYearlyAgeCellProjection,
};
