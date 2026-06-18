//! Renderer-neutral GUI view models.
//!
//! Where [`facade_snapshot`](crate::core::model::chart::facade_snapshot) DTOs are
//! compatibility/export payloads, the view models here are *GUI-facing read
//! models*: one selected "chart slice" arranged for a static 12-palace chart,
//! carrying Chinese labels, grouped star lists, temporal overlays, scope-selector
//! state, and reserved highlight annotations.
//!
//! A view model describes *what* a renderer should show. It never chooses CSS
//! classes, colors, canvas coordinates, camera position, animation, or 3D
//! geometry — that is the renderer's responsibility. The same
//! [`StaticChartViewSnapshot`] is intended to be reusable later as one frame in a
//! timeline/3D sequence.
//!
//! These models derive facts from an already-assembled [`Chart`](crate::Chart) or
//! [`HoroscopeChart`](crate::HoroscopeChart). They never mutate natal facts,
//! derive temporal periods, evaluate rules, or detect 成格 patterns.

pub mod static_chart;

pub use static_chart::{
    HighlightView, StaticChartCenterView, StaticChartSelectorView, StaticChartViewRequest,
    StaticChartViewSnapshot, StaticDecadalCellView, StaticDecorativeStarView,
    StaticFourPillarsView, StaticNavigationCellView, StaticOverlayMutagenView, StaticPalaceRole,
    StaticPalaceView, StaticSurroundPalacesView, StaticTemporalNavigationSelection,
    StaticTemporalOverlayView, StaticTemporalPanelView, StaticTypedStarView,
    StaticYearlyAgeCellView,
};
