//! Shared read-only context describing the chart state a rule is evaluated against.
//!
//! Patterns are a subset of rules. Both the pattern engine and the classical
//! rule engine need the same thing from a context: the natal chart facts, an
//! optional [`HoroscopeChart`] overlay, the active temporal scopes, and the
//! selected [`EffectiveChartState`]. [`RuleEvaluationContext`] is that single
//! shared model. Pattern- and classical-specific context types wrap it rather
//! than duplicating its state.
//!
//! [`RuleEvaluationContext`] describes chart/effective-state availability only.
//! It does **not** classify a rule as pattern/classical/diagnostic/etc. Rule
//! identity (whether a rule is a pattern, a source-backed claim, a diagnostic, a
//! flow rule, …) belongs in metadata/output layers, not in this context.
//!
//! Temporal layers remain overlays: when a [`HoroscopeChart`] is supplied its
//! natal chart is used as the underlying [`Chart`], and temporal facts are never
//! folded into natal facts.

use crate::core::{Chart, EffectiveChartState, HoroscopeChart, Scope};

/// The shared, read-only context describing the chart state under evaluation.
///
/// Fields are private so callers cannot bypass the selected-state model or
/// fabricate ambiguous contexts. Construct through the named constructors so the
/// effective selected state is always built by one of the sanctioned paths, and
/// read through the accessors.
#[derive(Clone, Debug)]
pub struct RuleEvaluationContext<'a> {
    /// The natal chart facts being analyzed.
    chart: &'a Chart,
    /// The horoscope chart, when temporal scopes are in play.
    horoscope: Option<&'a HoroscopeChart>,
    /// The temporal scopes currently active for evaluation.
    active_scopes: Vec<Scope>,
    /// The selected effective chart state, when strict construction succeeds.
    effective: Option<EffectiveChartState<'a>>,
}

impl<'a> RuleEvaluationContext<'a> {
    /// Creates a natal-only context.
    ///
    /// This constructor is strict: the natal effective state is always present,
    /// so selected-state helpers never fail closed for a natal context. The
    /// selected palace frame is [`Scope::Natal`] and the only active scope is
    /// [`Scope::Natal`].
    pub fn natal(chart: &'a Chart) -> Self {
        Self {
            chart,
            horoscope: None,
            active_scopes: vec![Scope::Natal],
            effective: Some(
                EffectiveChartState::from_chart(chart, Scope::Natal, vec![Scope::Natal])
                    .expect("natal effective state is valid"),
            ),
        }
    }

    /// Creates a context over a horoscope chart with the given active scopes.
    ///
    /// This is a **compatibility/convenience constructor only**. It derives the
    /// palace frame from the *deepest* active scope (`active_scopes.last()`) and
    /// builds the effective state leniently: if strict effective-state
    /// construction fails, [`effective`](Self::effective) is `None` and every
    /// selected-state helper fails closed, while source/layer-specific helpers
    /// remain available.
    ///
    /// Production selected-view analysis should use
    /// [`RuleEvaluationContext::horoscope_with_frame`] so the frame scope is
    /// explicit and construction is strict.
    pub fn horoscope(chart: &'a HoroscopeChart, active_scopes: Vec<Scope>) -> Self {
        let frame_scope = active_scopes.last().copied().unwrap_or(Scope::Natal);
        let effective =
            EffectiveChartState::from_horoscope(chart, frame_scope, active_scopes.clone()).ok();
        Self {
            chart: chart.natal(),
            horoscope: Some(chart),
            active_scopes,
            effective,
        }
    }

    /// Creates a context over a horoscope chart with an explicit palace frame.
    ///
    /// This is the production constructor for selected-view analysis. The
    /// effective state is built strictly from `palace_frame_scope` and
    /// `active_scopes`; invalid inputs panic because callers must validate
    /// selected temporal context before evaluation.
    pub fn horoscope_with_frame(
        chart: &'a HoroscopeChart,
        palace_frame_scope: Scope,
        active_scopes: Vec<Scope>,
    ) -> Self {
        let effective =
            EffectiveChartState::from_horoscope(chart, palace_frame_scope, active_scopes.clone())
                .expect("rule evaluation context requires a valid effective chart state");
        Self {
            chart: chart.natal(),
            horoscope: Some(chart),
            active_scopes,
            effective: Some(effective),
        }
    }

    /// Returns the natal chart facts being analyzed.
    pub fn chart(&self) -> &'a Chart {
        self.chart
    }

    /// Returns the horoscope chart, when temporal scopes are in play.
    ///
    /// Named `horoscope_chart` rather than `horoscope` because the
    /// [`horoscope`](Self::horoscope) constructor already owns that name.
    pub fn horoscope_chart(&self) -> Option<&'a HoroscopeChart> {
        self.horoscope
    }

    /// Returns the temporal scopes currently active for evaluation.
    pub fn active_scopes(&self) -> &[Scope] {
        &self.active_scopes
    }

    /// Returns the selected effective chart state, when one was constructed.
    ///
    /// This is `None` only for the lenient [`horoscope`](Self::horoscope)
    /// constructor when strict effective-state construction failed; the
    /// [`natal`](Self::natal) and
    /// [`horoscope_with_frame`](Self::horoscope_with_frame) constructors always
    /// populate it.
    pub fn effective(&self) -> Option<&EffectiveChartState<'a>> {
        self.effective.as_ref()
    }

    /// Returns the scope supplying the selected palace-name frame, if an
    /// effective state exists.
    ///
    /// This is the frame that selected-state helpers read against. It is
    /// [`Scope::Natal`] for a natal context and the explicit frame scope for a
    /// [`horoscope_with_frame`](Self::horoscope_with_frame) context.
    pub fn selected_frame_scope(&self) -> Option<Scope> {
        self.effective
            .as_ref()
            .map(EffectiveChartState::palace_frame_scope)
    }
}
