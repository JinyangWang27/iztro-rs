//! Read-only context and request types for pattern detection.

use crate::core::pattern::model::PatternFamily;
use crate::core::{Chart, EffectiveChartState, HoroscopeChart, Scope};

/// A read-only query wrapper over a chart for pattern detection.
///
/// The context borrows chart facts and never mutates them. When a
/// [`HoroscopeChart`] is supplied, its natal chart is used as the underlying
/// [`Chart`]; temporal layers remain overlays and are never folded into natal
/// facts.
///
/// Fields are private so detector and query code cannot bypass the
/// selected-state model or fabricate ambiguous contexts. Read through the
/// accessors ([`chart`](Self::chart), [`horoscope`](Self::horoscope),
/// [`active_scopes`](Self::active_scopes), [`effective`](Self::effective),
/// [`selected_frame_scope`](Self::selected_frame_scope)) and construct through
/// the named constructors so the effective selected state is always built by
/// one of the sanctioned paths.
#[derive(Clone, Debug)]
pub struct PatternContext<'a> {
    /// The natal chart facts being analyzed.
    chart: &'a Chart,
    /// The horoscope chart, when temporal scopes are in play.
    horoscope: Option<&'a HoroscopeChart>,
    /// The temporal scopes currently active for detection.
    active_scopes: Vec<Scope>,
    /// The selected effective chart state, when strict construction succeeds.
    effective: Option<EffectiveChartState<'a>>,
}

impl<'a> PatternContext<'a> {
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
    /// [`PatternContext::horoscope_with_frame`] so the frame scope is explicit
    /// and construction is strict.
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
    /// selected temporal context before detection.
    pub fn horoscope_with_frame(
        chart: &'a HoroscopeChart,
        palace_frame_scope: Scope,
        active_scopes: Vec<Scope>,
    ) -> Self {
        let effective =
            EffectiveChartState::from_horoscope(chart, palace_frame_scope, active_scopes.clone())
                .expect("pattern context requires a valid effective chart state");
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

    /// Returns the temporal scopes currently active for detection.
    pub fn active_scopes(&self) -> &[Scope] {
        &self.active_scopes
    }

    /// Returns the selected effective chart state, when one was constructed.
    ///
    /// This is `None` only for the lenient [`horoscope`](Self::horoscope)
    /// constructor when strict effective-state construction failed; the
    /// [`natal`](Self::natal) and [`horoscope_with_frame`](Self::horoscope_with_frame)
    /// constructors always populate it.
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
