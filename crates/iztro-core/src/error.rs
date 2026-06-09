use crate::model::ganzhi::{EarthlyBranch, HeavenlyStem};
use crate::model::star::StarName;
use crate::model::star::mutagen::Scope;
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
    /// Lunar day input must be in the supported range.
    #[error("invalid lunar day: expected 1..=30, got {value}")]
    InvalidLunarDay {
        /// Unsupported lunar day value.
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
    /// A required builder input was not provided before `build`.
    #[error("missing required input: {field}")]
    MissingRequiredInput {
        /// Name of the missing required field.
        field: &'static str,
    },
    /// A star placement rule depends on a previously placed star that is absent.
    #[error("required star is missing: {star:?}")]
    RequiredStarMissing {
        /// Star required by the placement rule.
        star: StarName,
    },
    /// A placement rule depends on the Life or Body Palace, which is absent.
    #[error("required Life/Body Palace context is missing")]
    RequiredLifeBodyPalaceMissing,
    /// A placement rule depends on the five-element bureau, which is absent.
    #[error("required five-element bureau is missing")]
    RequiredFiveElementBureauMissing,
    /// A decorative placement must name a known decorative (untyped) star whose
    /// family matches and whose known metadata carries no `StarKind`.
    #[error("invalid decorative star placement: {star:?}")]
    InvalidDecorativeStarPlacement {
        /// Star rejected as a decorative placement.
        star: StarName,
    },
    /// A temporal layer cannot use the natal scope; natal facts live in the chart.
    #[error("temporal layer cannot use the natal scope")]
    NatalScopeInTemporalLayer,
    /// A temporal layer's scope must match its temporal context.
    #[error("temporal layer scope {layer:?} does not match context scope {context:?}")]
    TemporalScopeMismatch {
        /// Scope declared on the layer.
        layer: Scope,
        /// Scope implied by the temporal context.
        context: Scope,
    },
    /// A scoped placement in a temporal layer must carry the layer's scope.
    #[error("temporal placement scope {placement:?} does not match layer scope {layer:?}")]
    TemporalPlacementScopeMismatch {
        /// Scope declared on the layer.
        layer: Scope,
        /// Scope carried by the rejected placement.
        placement: Scope,
    },
    /// A mutagen activation in a temporal layer must carry the layer's scope.
    #[error("temporal activation scope {activation:?} does not match layer scope {layer:?}")]
    TemporalActivationScopeMismatch {
        /// Scope declared on the layer.
        layer: Scope,
        /// Source scope carried by the rejected activation.
        activation: Scope,
    },
    /// Placeholder error used until chart-generation validation exists.
    #[error("chart generation is not implemented")]
    NotImplemented,
}
