//! Read-only context for classical rule evaluation.
//!
//! `ClassicalRuleContext` is a classical-rule wrapper over
//! [`RuleEvaluationContext`](crate::core::RuleEvaluationContext). Patterns are a
//! subset of rules, so both engines evaluate against the same shared chart
//! state; this type exists only for classical-rule-specific APIs.
//!
//! Current executable classical rules remain conservative and mostly natal:
//! they inspect natal chart facts only. Future temporal classical rules should
//! use [`effective`](ClassicalRuleContext::effective) /
//! [`selected_frame_scope`](ClassicalRuleContext::selected_frame_scope) helpers
//! rather than raw natal chart facts.
//!
//! Temporal layers remain overlays: when a [`HoroscopeChart`] is supplied its
//! natal chart is used as the underlying [`Chart`], and temporal facts are never
//! folded into natal facts.

use crate::core::{Chart, EffectiveChartState, HoroscopeChart, RuleEvaluationContext, Scope};

/// A read-only context over a chart for classical rule evaluation.
///
/// The inner shared context is private; read through the accessors and
/// construct through the named constructors so the effective selected state is
/// always built by one of the sanctioned paths.
#[derive(Clone, Debug)]
pub struct ClassicalRuleContext<'a> {
    /// The shared rule-evaluation context this classical context wraps.
    inner: RuleEvaluationContext<'a>,
}

impl<'a> ClassicalRuleContext<'a> {
    /// Creates a natal-only context.
    ///
    /// The selected palace frame is [`Scope::Natal`] and the only active scope
    /// is [`Scope::Natal`]; the natal effective state is always present.
    pub fn natal(chart: &'a Chart) -> Self {
        Self {
            inner: RuleEvaluationContext::natal(chart),
        }
    }

    /// Creates a context over a horoscope chart with the given active scopes.
    ///
    /// This is a **compatibility/convenience constructor only**: it derives the
    /// palace frame from the deepest active scope and builds the effective state
    /// leniently. Production selected-view evaluation should use
    /// [`ClassicalRuleContext::horoscope_with_frame`].
    pub fn horoscope(chart: &'a HoroscopeChart, active_scopes: Vec<Scope>) -> Self {
        Self {
            inner: RuleEvaluationContext::horoscope(chart, active_scopes),
        }
    }

    /// Creates a context over a horoscope chart with an explicit palace frame.
    ///
    /// This is the production constructor for selected-view evaluation. The
    /// effective state is built strictly; invalid inputs panic because callers
    /// must validate selected temporal context before evaluation.
    pub fn horoscope_with_frame(
        chart: &'a HoroscopeChart,
        palace_frame_scope: Scope,
        active_scopes: Vec<Scope>,
    ) -> Self {
        Self {
            inner: RuleEvaluationContext::horoscope_with_frame(
                chart,
                palace_frame_scope,
                active_scopes,
            ),
        }
    }

    /// Returns the shared rule-evaluation context this classical context wraps.
    pub fn as_rule_context(&self) -> &RuleEvaluationContext<'a> {
        &self.inner
    }

    /// Returns the natal chart facts being analyzed.
    pub fn chart(&self) -> &'a Chart {
        self.inner.chart()
    }

    /// Returns the horoscope chart, when temporal scopes are in play.
    pub fn horoscope_chart(&self) -> Option<&'a HoroscopeChart> {
        self.inner.horoscope_chart()
    }

    /// Returns the temporal scopes currently active for evaluation.
    pub fn active_scopes(&self) -> &[Scope] {
        self.inner.active_scopes()
    }

    /// Returns the selected effective chart state, when one was constructed.
    pub fn effective(&self) -> Option<&EffectiveChartState<'a>> {
        self.inner.effective()
    }

    /// Returns the scope supplying the selected palace-name frame, if an
    /// effective state exists.
    pub fn selected_frame_scope(&self) -> Option<Scope> {
        self.inner.selected_frame_scope()
    }
}
