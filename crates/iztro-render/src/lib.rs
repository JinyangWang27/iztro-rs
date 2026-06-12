//! Deterministic renderers for chart snapshot read models.
//!
//! This crate consumes renderer-neutral `iztro-core` snapshots. It must not
//! generate chart facts, evaluate rules, localize terminology, or perform
//! narrative interpretation.

pub mod text;

pub use text::{PlainTextChartRenderer, PlainTextRenderOptions, render_chart_stack_text};
