//! Classical source references (出处) for rules and claims.
//!
//! Chinese source text is canonical for classical terminology. These types
//! preserve the original 全书 text verbatim so downstream layers can cite it; the
//! machine logic keys off the typed [`super::rule::ClassicalRuleId`] and enums,
//! never the Chinese strings.

use serde::{Deserialize, Serialize};

/// A classical work (典籍) a rule is drawn from.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClassicalWork {
    /// 《紫微斗数全书》.
    ZiWeiDouShuQuanShu,
}

/// An auditable reference to a classical source line.
///
/// `source_text_zh_hans` preserves the canonical classical text;
/// `normalized_note_zh_hans` is an optional editor's note clarifying how the line
/// was interpreted into a rule. Both are Chinese-first and never used as logic
/// keys.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SourceRef {
    /// The work this line is drawn from.
    pub work: ClassicalWork,
    /// Stable identifier for the source line (e.g. `quan_shu.ma_luo_kong_wang`).
    pub source_id: String,
    /// Canonical classical text, Simplified Chinese.
    pub source_text_zh_hans: String,
    /// Optional normalization note, Simplified Chinese.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalized_note_zh_hans: Option<String>,
}
