//! Internal emit helpers shared by named pattern detectors.
//!
//! A named detector produces a [`FormationMatch`] (成格: the base formation) and
//! an [`IntegrityAssessment`] (破格/减力: whether the formation is fulfilled,
//! weakened, or broken). [`push_detection`] combines them with registry metadata
//! into a [`PatternDetection`].
//!
//! These helper types are **internal** (`pub(crate)`); they are not part of the
//! public pattern API. Runtime display name, family, and polarity always come
//! from the registry via [`pattern_spec`], never from source metadata.

use crate::core::{EarthlyBranch, Mutagen, Scope, StarName};
use crate::rules::pattern::model::{
    PatternAnchor, PatternCondition, PatternDetection, PatternEvidence, PatternId, PatternStatus,
    PatternStrength,
};
use crate::rules::pattern::query::pattern_scope_for;
use crate::rules::pattern::registry::pattern_spec;

/// The base formation (成格) a named detector matched, before integrity is
/// assessed. Identity, family, and polarity are resolved from the registry at
/// emit time, so this carries only the chart-specific facts.
pub(crate) struct FormationMatch {
    pub(crate) id: PatternId,
    pub(crate) scope: Scope,
    pub(crate) anchor: PatternAnchor,
    pub(crate) involved_palaces: Vec<EarthlyBranch>,
    pub(crate) involved_stars: Vec<StarName>,
    pub(crate) involved_mutagens: Vec<Mutagen>,
    pub(crate) evidence: Vec<PatternEvidence>,
}

/// The integrity (破格/减力) verdict for a base formation that already exists.
pub(crate) struct IntegrityAssessment {
    pub(crate) status: PatternStatus,
    pub(crate) weakening_factors: Vec<PatternCondition>,
    pub(crate) breaking_factors: Vec<PatternCondition>,
}

impl IntegrityAssessment {
    /// 成格: the base formation exists and no modeled weakening/breaker applies.
    pub(crate) fn fulfilled() -> Self {
        Self {
            status: PatternStatus::Fulfilled,
            weakening_factors: Vec::new(),
            breaking_factors: Vec::new(),
        }
    }

    /// 成而减力: the base formation exists but the given weakening factors apply.
    pub(crate) fn weakened(weakening_factors: Vec<PatternCondition>) -> Self {
        Self {
            status: PatternStatus::Weakened,
            weakening_factors,
            breaking_factors: Vec::new(),
        }
    }

    /// 破格: the base formation exists but the given breaker conditions apply.
    pub(crate) fn broken(breaking_factors: Vec<PatternCondition>) -> Self {
        Self {
            status: PatternStatus::Broken,
            weakening_factors: Vec::new(),
            breaking_factors,
        }
    }
}

/// Builds a [`PatternDetection`] from a base formation, its integrity verdict,
/// and the registry [`PatternSpec`], then appends it to `out`.
///
/// Display name, family, and polarity come from `pattern_spec(base.id)`; strength
/// is [`PatternStrength::Medium`], preserving existing behaviour.
///
/// [`PatternSpec`]: crate::rules::pattern::registry::PatternSpec
pub(crate) fn push_detection(
    out: &mut Vec<PatternDetection>,
    base: FormationMatch,
    integrity: IntegrityAssessment,
) {
    let spec = pattern_spec(base.id);
    out.push(PatternDetection {
        id: base.id,
        name_zh: spec.name_zh,
        family: spec.family,
        polarity: spec.polarity,
        status: integrity.status,
        strength: PatternStrength::Medium,
        scope: pattern_scope_for(base.scope),
        anchor: base.anchor,
        involved_palaces: base.involved_palaces,
        involved_stars: base.involved_stars,
        involved_mutagens: base.involved_mutagens,
        evidence: base.evidence,
        weakening_factors: integrity.weakening_factors,
        breaking_factors: integrity.breaking_factors,
    });
}
