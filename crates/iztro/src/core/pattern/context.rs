//! Read-only context and request types for pattern detection.

use crate::core::pattern::model::PatternFamily;
use crate::core::{Chart, HoroscopeChart, Scope};

/// A read-only query wrapper over a chart for pattern detection.
///
/// The context borrows chart facts and never mutates them. When a
/// [`HoroscopeChart`] is supplied, its natal chart is used as the underlying
/// [`Chart`]; temporal layers remain overlays and are never folded into natal
/// facts.
#[derive(Clone, Debug)]
pub struct PatternContext<'a> {
    /// The natal chart facts being analyzed.
    pub chart: &'a Chart,
    /// The horoscope chart, when temporal scopes are in play.
    pub horoscope: Option<&'a HoroscopeChart>,
    /// The temporal scopes currently active for detection.
    pub active_scopes: Vec<Scope>,
}

impl<'a> PatternContext<'a> {
    /// Creates a natal-only context.
    pub fn natal(chart: &'a Chart) -> Self {
        Self {
            chart,
            horoscope: None,
            active_scopes: vec![Scope::Natal],
        }
    }

    /// Creates a context over a horoscope chart with the given active scopes.
    pub fn horoscope(chart: &'a HoroscopeChart, active_scopes: Vec<Scope>) -> Self {
        Self {
            chart: chart.natal(),
            horoscope: Some(chart),
            active_scopes,
        }
    }
}

/// A request controlling which detections [`detect_patterns`] returns.
///
/// [`detect_patterns`]: crate::core::pattern::detect_patterns
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatternDetectionRequest {
    /// Scopes to detect within.
    pub scopes: Vec<Scope>,
    /// Whether to include [`PatternStatus::Weakened`] detections.
    ///
    /// [`PatternStatus::Weakened`]: crate::core::pattern::PatternStatus::Weakened
    pub include_weakened: bool,
    /// Whether to include [`PatternStatus::Broken`] detections.
    ///
    /// [`PatternStatus::Broken`]: crate::core::pattern::PatternStatus::Broken
    pub include_broken: bool,
    /// If non-empty, only detections in these families are returned.
    pub families: Vec<PatternFamily>,
}

impl Default for PatternDetectionRequest {
    fn default() -> Self {
        Self {
            scopes: vec![Scope::Natal],
            include_weakened: true,
            include_broken: true,
            families: Vec::new(),
        }
    }
}
