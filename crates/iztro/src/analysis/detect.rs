//! Layer-level analysis: request, result, context, and detection entry point.

use serde::Serialize;

use crate::core::pattern::{
    PatternContext, PatternDetection, PatternDetectionRequest, detect_patterns,
};
use crate::core::{Chart, HoroscopeChart, Scope};
use crate::rules::classical::{
    ClaimEvaluationRequest, ClassicalRuleContext, ClassicalRuleHitRef, ClassicalWork,
    DiagnosticMode, evaluate_classical_in_context,
};

use crate::analysis::layer::AnalysisLayerKey;

/// A read-only context for analyzing one or more layers of a chart.
///
/// It provides the natal chart plus an optional horoscope chart, which together
/// are enough to analyze any requested layer. Detection of a deep layer may
/// inspect ancestor overlays through the horoscope, but the result is always
/// assigned to the requested layer (see [`detect_analysis_layer`]).
///
/// # Caller contract
///
/// The context must correspond to the [`AnalysisLayerKey`] passed to
/// [`detect_analysis_layer`]: its `horoscope` should already be projected to the
/// temporal selection the key addresses. The key is used for cache identity and
/// scope assignment; it is **not** currently validated against the horoscope's
/// selected overlays, so supplying a mismatched context/key pair yields hits
/// keyed to the requested layer over whatever overlays the context actually
/// carries. Keeping them in sync is the caller's responsibility.
#[derive(Clone, Debug)]
pub struct TemporalAnalysisContext<'a> {
    /// The natal chart facts.
    pub natal: &'a Chart,
    /// The horoscope chart, when temporal overlays are available.
    pub horoscope: Option<&'a HoroscopeChart>,
}

impl<'a> TemporalAnalysisContext<'a> {
    /// Creates a natal-only analysis context.
    pub fn natal(natal: &'a Chart) -> Self {
        Self {
            natal,
            horoscope: None,
        }
    }

    /// Creates an analysis context over a horoscope chart.
    pub fn horoscope(horoscope: &'a HoroscopeChart) -> Self {
        Self {
            natal: horoscope.natal(),
            horoscope: Some(horoscope),
        }
    }
}

/// Controls what [`detect_analysis_layer`] computes for one layer.
///
/// The request is layer-agnostic: the same request is reused across every layer,
/// and [`detect_analysis_layer`] narrows scope filters to the requested layer
/// itself. `classical` and `patterns` carry the underlying engine filters; their
/// scope fields are overridden per layer and need not be set by callers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnalysisLayerRequest {
    /// Whether to evaluate classical rule hits.
    pub include_rules: bool,
    /// Whether to detect patterns.
    pub include_patterns: bool,
    /// Underlying classical claim evaluation filters.
    pub classical: ClaimEvaluationRequest,
    /// Underlying pattern detection filters.
    pub patterns: PatternDetectionRequest,
}

impl AnalysisLayerRequest {
    /// A user-facing request: rules and patterns on, diagnostics hidden.
    ///
    /// Classical unsupported-rule diagnostics are suppressed (end users should not
    /// see them). Pattern visibility follows the existing
    /// [`PatternDetectionRequest`] defaults: fulfilled, weakened, and broken
    /// patterns are included; partial patterns are not.
    ///
    /// The classical rule stream is restricted to
    /// [`ClassicalWork::ZiWeiDouShuQuanShu`]: the future GUI shows 全书规则 and
    /// 格局 in **separate** tabs, so the analysis rule-hit stream must not include
    /// project pattern-catalog rules ([`ClassicalWork::IztroPatternCatalog`]),
    /// which surface through the pattern (格局) stream instead.
    pub fn user_facing() -> Self {
        Self {
            include_rules: true,
            include_patterns: true,
            classical: ClaimEvaluationRequest {
                diagnostic_mode: DiagnosticMode::None,
                works: vec![ClassicalWork::ZiWeiDouShuQuanShu],
                ..Default::default()
            },
            patterns: PatternDetectionRequest::default(),
        }
    }
}

impl Default for AnalysisLayerRequest {
    fn default() -> Self {
        Self::user_facing()
    }
}

/// The analysis result for one layer.
///
/// `rule_hits` are compact [`ClassicalRuleHitRef`]s (no duplicated source text);
/// `pattern_hits` are full [`PatternDetection`]s. Both belong to [`key`]'s scope.
/// [`PatternDetection`] serializes but does not deserialize (it borrows a
/// `'static` name), so this type derives [`Serialize`] only.
///
/// [`key`]: AnalysisLayerResult::key
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AnalysisLayerResult {
    /// The layer this result belongs to.
    pub key: AnalysisLayerKey,
    /// Compact classical rule hits in this layer's scope.
    pub rule_hits: Vec<ClassicalRuleHitRef>,
    /// Pattern detections in this layer's scope.
    pub pattern_hits: Vec<PatternDetection>,
}

/// Detects rule and pattern hits for one analysis layer.
///
/// # Layer assignment
///
/// Detection may inspect ancestor overlays (through `ctx.horoscope`), but the
/// returned hits always belong to `key`'s scope. A future yearly rule may read
/// natal + decadal + yearly context yet still return a yearly hit. This function
/// does **not** compute ancestor layers; the caller requests missing ancestors
/// separately and caches each layer by its [`AnalysisLayerKey`].
///
/// # Current behaviour
///
/// Current executable classical rules match natal facts only, so `rule_hits` is
/// non-empty only for [`AnalysisLayerKey::Natal`]. Pattern detection is scoped to
/// `key` and returns whatever patterns the detectors support in that scope.
pub fn detect_analysis_layer(
    ctx: &TemporalAnalysisContext<'_>,
    key: AnalysisLayerKey,
    request: &AnalysisLayerRequest,
) -> AnalysisLayerResult {
    let rule_hits = if request.include_rules {
        detect_rule_hits(ctx, &key, request)
    } else {
        Vec::new()
    };

    let pattern_hits = if request.include_patterns {
        detect_pattern_hits(ctx, &key, request)
    } else {
        Vec::new()
    };

    AnalysisLayerResult {
        key,
        rule_hits,
        pattern_hits,
    }
}

/// The inclusive natal-outward scope chain visible when analyzing `key`.
///
/// Used as the context's `active_scopes` so future rules can inspect ancestor
/// overlays. It follows the canonical scope ordering up to the layer's scope.
fn active_scopes_for(key: &AnalysisLayerKey) -> Vec<Scope> {
    const ORDER: [Scope; 7] = [
        Scope::Natal,
        Scope::Decadal,
        Scope::Age,
        Scope::Yearly,
        Scope::Monthly,
        Scope::Daily,
        Scope::Hourly,
    ];
    let target = key.scope();
    let mut scopes = Vec::new();
    for scope in ORDER {
        scopes.push(scope);
        if scope == target {
            break;
        }
    }
    scopes
}

/// Evaluates classical rules narrowed to `key`'s scope and compacts the hits.
///
/// Only the scope filter is overridden to the requested layer; every other filter
/// on `request.classical` — notably `works` (e.g. the QuanShu-only restriction in
/// [`AnalysisLayerRequest::user_facing`]) — is preserved from the caller's
/// request.
fn detect_rule_hits(
    ctx: &TemporalAnalysisContext<'_>,
    key: &AnalysisLayerKey,
    request: &AnalysisLayerRequest,
) -> Vec<ClassicalRuleHitRef> {
    // Clone preserves every caller-supplied filter (works, domains, themes, …);
    // we override only the scope to the requested layer.
    let mut classical = request.classical.clone();
    classical.scopes = vec![key.claim_scope()];

    let rule_ctx = match ctx.horoscope {
        Some(horoscope) => ClassicalRuleContext::horoscope(horoscope, active_scopes_for(key)),
        None => ClassicalRuleContext::natal(ctx.natal),
    };

    evaluate_classical_in_context(&rule_ctx, &classical)
        .source_hits
        .iter()
        .map(ClassicalRuleHitRef::from_source_hit)
        .collect()
}

/// Detects patterns narrowed to `key`'s scope.
fn detect_pattern_hits(
    ctx: &TemporalAnalysisContext<'_>,
    key: &AnalysisLayerKey,
    request: &AnalysisLayerRequest,
) -> Vec<PatternDetection> {
    let mut patterns = request.patterns.clone();
    patterns.scopes = vec![key.scope()];

    let pattern_ctx = match ctx.horoscope {
        Some(horoscope) => PatternContext::horoscope(horoscope, active_scopes_for(key)),
        None => PatternContext::natal(ctx.natal),
    };

    detect_patterns(&pattern_ctx, &patterns)
}
