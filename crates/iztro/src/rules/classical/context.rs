//! Read-only context for classical rule evaluation.
//!
//! Mirroring [`PatternContext`](crate::core::pattern::PatternContext), this
//! context borrows chart facts and the active temporal scopes a rule may inspect.
//! It exists so the classical engine has the same context/layer shape as the
//! pattern engine, even though current executable rules still evaluate against
//! natal facts only.
//!
//! Temporal layers remain overlays: when a [`HoroscopeChart`] is supplied its
//! natal chart is used as the underlying [`Chart`], and temporal facts are never
//! folded into natal facts.

use crate::core::{Chart, HoroscopeChart, Scope};

/// A read-only context over a chart for classical rule evaluation.
#[derive(Clone, Debug)]
pub struct ClassicalRuleContext<'a> {
    /// The natal chart facts being analyzed.
    pub chart: &'a Chart,
    /// The horoscope chart, when temporal scopes are in play.
    pub horoscope: Option<&'a HoroscopeChart>,
    /// The temporal scopes currently active for evaluation.
    ///
    /// Current executable rules ignore this and match natal facts only; it is
    /// carried so future temporal rules can inspect ancestor overlays without an
    /// API change.
    pub active_scopes: Vec<Scope>,
}

impl<'a> ClassicalRuleContext<'a> {
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
