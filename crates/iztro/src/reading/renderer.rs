use crate::reading::{report::ReadingReport, section::ReadingSection};
use crate::rules::classical::Claim;
use thiserror::Error;

/// Renders structured classical claims into a deterministic report.
pub trait ReportRenderer {
    /// Renders claims into a report structure.
    fn render(&self, claims: &[Claim]) -> Result<ReadingReport, RenderError>;
}

/// Placeholder renderer for scaffolding and integration tests.
///
/// It renders only stable, machine-readable claim fields (`claim_key`,
/// `domain`, `themes`, `polarity`). It deliberately produces no localized prose:
/// human-facing text is the responsibility of the i18n/narrative layer, which is
/// out of scope here.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlaceholderRenderer;

impl ReportRenderer for PlaceholderRenderer {
    fn render(&self, claims: &[Claim]) -> Result<ReadingReport, RenderError> {
        let sections = claims
            .iter()
            .map(|claim| {
                let themes = if claim.themes.is_empty() {
                    "no themes".to_owned()
                } else {
                    claim
                        .themes
                        .iter()
                        .map(|theme| format!("{theme:?}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                ReadingSection::new(
                    claim.domain,
                    claim.claim_key().to_owned(),
                    format!(
                        "Placeholder reading ({:?}) for themes: {themes}.",
                        claim.polarity
                    ),
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
