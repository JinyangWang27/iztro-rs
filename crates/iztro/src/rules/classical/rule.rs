//! Classical rule metadata, deserialized from the Chinese-first corpus TOML.
//!
//! This is the *metadata* half of the hybrid design: the rule's identity, source,
//! status, and claim shape are data-driven from `rule-corpus/`, while the matching
//! *predicate* is hand-coded in [`super::predicates`]. There is no generic DSL yet.

use serde::{Deserialize, Serialize};

use crate::rules::classical::claim::ClaimDomain;
use crate::rules::classical::theme::{ClaimPolarity, ClaimTheme};
use crate::rules::source::{ClassicalWork, SourceRef};

/// A stable, machine-facing rule identifier (e.g.
/// `migration.tian_ma_void.restless_movement`).
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ClassicalRuleId(String);

impl ClassicalRuleId {
    /// Creates a rule id from any string-like value.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the id as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ClassicalRuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// The encoding maturity of a classical rule.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleStatus {
    /// 原文 — raw, unsegmented source line.
    Raw,
    /// 已断句 — segmented into discrete statements.
    Segmented,
    /// 已规范 — normalized into a structured intent.
    Normalized,
    /// 可执行 — backed by a working predicate over modeled facts.
    Executable,
    /// 已测试 — executable with positive/negative realistic or source-grounded
    /// fixtures, suitable for stable public consumption.
    Tested,
    /// 有歧义 — meaning or condition is ambiguous.
    Ambiguous,
    /// 已弃用 — rejected / not used.
    Rejected,
}

/// The interpretive school (流派) a rule belongs to.
///
/// A placeholder for future multi-school support. Rules default to [`RuleSchool::General`]
/// until schools are explicitly modeled, keeping school selection out of scattered
/// `if`/`match` logic.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RuleSchool {
    /// 通用 — not tied to a specific school.
    #[default]
    General,
}

/// Optional interpretation metadata for a rule that can produce a claim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClaimSpec {
    /// Claim domain this rule produces.
    pub domain: ClaimDomain,
    /// Claim themes this rule produces.
    pub themes: Vec<ClaimTheme>,
    /// Claim polarity this rule produces.
    pub polarity: ClaimPolarity,
    /// Base claim strength before any modifiers.
    pub base_strength: f32,
    /// The i18n key used to render the produced claim's localized text.
    pub claim_key: String,
}

/// Metadata for one classical rule, authored in `rule-corpus/`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassicalRule {
    /// Stable rule identifier.
    pub id: ClassicalRuleId,
    /// Stable identifier of the atomic source unit or project-owned pattern
    /// metadata entry this rule cites.
    pub source_id: String,
    /// Legacy/compatibility provenance discriminator.
    ///
    /// QuanShu rules now cite atomic source units directly via `source_id`, so
    /// new QuanShu rules should not set this. It is retained for backward
    /// compatibility and may still be used by project-owned pattern catalog
    /// entries. This is rule metadata only; it does not load or depend on the
    /// test-only source inventory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_clause_id: Option<String>,
    /// The classical work the rule is drawn from.
    pub work: ClassicalWork,
    /// Canonical classical text (Simplified Chinese).
    pub source_text_zh_hans: String,
    /// Optional normalization note (Simplified Chinese).
    #[serde(default)]
    pub normalized_note_zh_hans: Option<String>,
    /// Encoding maturity.
    pub status: RuleStatus,
    /// Interpretive school.
    #[serde(default)]
    pub school: RuleSchool,
    /// Optional interpretation metadata. Rules without a claim still produce
    /// source hits when their executable predicate matches.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim: Option<ClaimSpec>,
}

impl ClassicalRule {
    /// Builds a [`SourceRef`] citing this rule's classical source unit.
    pub fn source_ref(&self) -> SourceRef {
        SourceRef {
            work: self.work,
            source_id: self.source_id.clone(),
            source_text_zh_hans: self.source_text_zh_hans.clone(),
            normalized_note_zh_hans: self.normalized_note_zh_hans.clone(),
        }
    }
}
