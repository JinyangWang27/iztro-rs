//! Deterministic report structures and rendering contracts for iztro-rs.

pub mod renderer;
pub mod report;
pub mod section;

pub use renderer::{PlaceholderRenderer, RenderError, ReportRenderer};
pub use report::ReadingReport;
pub use section::ReadingSection;
