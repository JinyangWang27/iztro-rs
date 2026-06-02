use crate::ganzhi::{EarthlyBranch, HeavenlyStem};
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
    /// Lunar month input must be in the supported non-leap range.
    #[error("invalid lunar month: expected 1..=12, got {value}")]
    InvalidLunarMonth {
        /// Unsupported lunar month value.
        value: u8,
    },
    /// A stem-branch pair must belong to the sexagenary cycle (matching parity).
    #[error("invalid sexagenary stem-branch pair: {stem:?}-{branch:?}")]
    InvalidStemBranchPair {
        /// Heavenly Stem of the rejected pair.
        stem: HeavenlyStem,
        /// Earthly Branch of the rejected pair.
        branch: EarthlyBranch,
    },
    /// Placeholder error used until chart-generation validation exists.
    #[error("chart generation is not implemented")]
    NotImplemented,
}
