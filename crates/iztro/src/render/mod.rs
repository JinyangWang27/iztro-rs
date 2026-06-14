//! Deterministic renderers for chart snapshot read models.
//!
//! This module consumes renderer-neutral `ChartStackSnapshot` read models. It
//! must not generate chart facts, evaluate rules, localize terminology, or
//! perform narrative interpretation.

pub mod text;

pub use text::{PlainTextChartRenderer, PlainTextRenderOptions, render_chart_stack_text};
