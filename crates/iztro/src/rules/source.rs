//! Shared rule source/provenance vocabulary (出处) for both rule engines.
//!
//! This is not classical-engine logic. `ClassicalWork` and `SourceRef` are
//! shared source/provenance types cited by classical rules, claims, and
//! project-owned pattern provenance alike, so they live in a neutral
//! `rules::source` home rather than inside either engine. `rules::classical::source`
//! re-exports these for compatibility.
//!
//! Chinese source text is canonical for classical terminology. These types
//! preserve the original 全书 text verbatim so downstream layers can cite it; the
//! machine logic keys off typed ids and enums, never the Chinese strings.

use serde::{Deserialize, Serialize};

/// A classical work (典籍) a rule is drawn from.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClassicalWork {
    /// 《紫微斗数全书》.
    ZiWeiDouShuQuanShu,
    /// Project-owned pattern/格局 catalog for rules derived from modeled chart
    /// structures (e.g. 夹宫 shapes) rather than from a cited QuanShu passage.
    IztroPatternCatalog,
}

/// An auditable reference to a classical source unit.
///
/// `source_text_zh_hans` preserves the canonical classical text;
/// `normalized_note_zh_hans` is an optional editor's note clarifying how the unit
/// was interpreted into a rule. Both are Chinese-first and never used as logic
/// keys.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SourceRef {
    /// The work this unit is drawn from.
    pub work: ClassicalWork,
    /// Stable identifier for the source unit (e.g. `quan_shu.ma_yu_kong_wang`).
    pub source_id: String,
    /// Canonical classical text, Simplified Chinese.
    pub source_text_zh_hans: String,
    /// Optional normalization note, Simplified Chinese.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalized_note_zh_hans: Option<String>,
}
