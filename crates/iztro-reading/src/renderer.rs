use crate::{report::ReadingReport, section::ReadingSection};
use iztro_rules::Claim;
use thiserror::Error;

/// Renders structured claims into a deterministic report.
pub trait ReportRenderer {
    /// Renders claims into a report structure.
    fn render(&self, claims: &[Claim]) -> Result<ReadingReport, RenderError>;
}

/// Placeholder renderer for scaffolding and integration tests.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlaceholderRenderer;

impl ReportRenderer for PlaceholderRenderer {
    fn render(&self, claims: &[Claim]) -> Result<ReadingReport, RenderError> {
        let sections = claims
            .iter()
            .map(|claim| {
                let themes = if claim.themes().is_empty() {
                    "no themes".to_owned()
                } else {
                    claim.themes().join(", ")
                };
                ReadingSection::new(
                    claim.domain(),
                    format!("{:?}", claim.domain()),
                    format!("Placeholder reading for themes: {themes}."),
                )
            })
            .collect();

        Ok(ReadingReport::new(sections))
    }
}

/// Errors produced by report rendering.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum RenderError {
    /// Rendering has not been implemented for this renderer.
    #[error("report rendering is not implemented")]
    NotImplemented,
}
