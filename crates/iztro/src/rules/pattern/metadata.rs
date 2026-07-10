//! Source provenance for canonical project-level pattern detections.
//!
//! This metadata links a canonical [`PatternId`] to a cited source item, such as
//! a QuanShu Volume 1 pattern-catalogue entry. It is **not** classical runtime
//! rule metadata and is **not** consumed by `evaluate_classical`: a 格局/pattern
//! has exactly one canonical runtime identity (`PatternId`, detected by
//! `rules::pattern`), and this table only records where that pattern is cited.
//!
//! The QuanShu source inventory TOML remains governance/test data. Runtime code
//! only carries provenance for patterns that have executable detections, so a
//! GUI or docs layer can display a pattern's ancient source citation. Section
//! metadata (卷/节) is not duplicated here: resolve `source_id` through
//! [`crate::rules::source::source_section`].

use serde::{Deserialize, Serialize};

use crate::rules::pattern::model::PatternId;
use crate::rules::pattern::registry::try_pattern_spec;
use crate::rules::source::ClassicalWork;

/// Source catalogue group for a source-backed pattern.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternSourceGroup {
    /// 定富局.
    Wealth,
    /// 定贵局.
    Noble,
    /// 定贫贱局.
    PovertyLowStatus,
    /// 定杂局.
    Miscellaneous,
    /// 斗数骨髓赋.
    DouShuGuSuiFu,
    /// 卷三：论诸星同垣各司所宜分别富贵贫贱夭寿.
    ZhuXingTongYuan,
}

/// Static source provenance for one canonical pattern detection.
///
/// Links a canonical [`PatternId`] to its cited source item. This is provenance
/// only; it does not imply a separate classical runtime rule exists for the
/// pattern.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PatternSourceMetadata {
    /// Canonical pattern id this provenance is attached to.
    pub pattern_id: PatternId,
    /// Canonical Chinese pattern name.
    pub name_zh: &'static str,
    /// Classical work this provenance is drawn from. Shared source vocabulary
    /// with the classical engine ([`ClassicalWork`]); serializes to the same
    /// snake_case identifier used in the source-inventory TOML.
    pub work: ClassicalWork,
    /// Full source inventory id.
    pub source_id: &'static str,
    /// Verbatim Simplified Chinese source text, without final `。`.
    pub source_text_zh_hans: &'static str,
    /// Source catalogue group.
    pub group: PatternSourceGroup,
}

/// Returns static source metadata for executable source-backed patterns.
pub fn pattern_source_metadata(pattern_id: PatternId) -> Option<&'static PatternSourceMetadata> {
    try_pattern_spec(pattern_id).and_then(|spec| spec.source.as_ref())
}
