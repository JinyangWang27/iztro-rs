use thiserror::Error;

/// Errors produced by core chart construction or validation.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum ChartError {
    /// A chart must contain exactly the expected number of palaces.
    #[error("invalid palace count: expected {expected}, got {actual}")]
    InvalidPalaceCount {
        /// Expected number of palaces.
        expected: usize,
        /// Actual number of palaces.
        actual: usize,
    },
    /// Placeholder error used until chart-generation validation exists.
    #[error("chart generation is not implemented")]
    NotImplemented,
}
