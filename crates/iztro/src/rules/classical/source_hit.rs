//! Matched classical source/provenance entries.
//!
//! A source hit records that an executable classical rule's predicate matched
//! chart facts. It is evaluation-facing provenance, not interpretive output:
//! claims are produced separately only when the rule has claim metadata.

use serde::{Deserialize, Serialize};

use crate::rules::classical::claim::ClaimScope;
use crate::rules::classical::evidence::Evidence;
use crate::rules::classical::rule::{ClassicalRule, ClassicalRuleId, RuleStatus};
use crate::rules::source::ClassicalWork;

/// Evidence-backed source/provenance for one matched executable rule.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassicalSourceHit {
    /// The rule whose predicate matched.
    pub rule_id: ClassicalRuleId,
    /// The work or project-owned catalog the source entry belongs to.
    pub work: ClassicalWork,
    /// Stable identifier for the atomic source unit or project-owned pattern
    /// metadata entry.
    pub source_id: String,
    /// Optional legacy/pattern provenance discriminator. Absent for QuanShu
    /// source hits, which cite their source unit directly via `source_id`; it
    /// may still be present for project-owned pattern catalog entries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_clause_id: Option<String>,
    /// Canonical source text, Simplified Chinese.
    pub source_text_zh_hans: String,
    /// Optional normalization note, Simplified Chinese.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalized_note_zh_hans: Option<String>,
    /// Rule encoding maturity at match time.
    pub status: RuleStatus,
    /// The scope the rule matched in.
    pub scope: ClaimScope,
    /// Machine-readable evidence for the match.
    pub evidence: Vec<Evidence>,
}

impl ClassicalSourceHit {
    /// Builds a source hit from rule provenance and matched evidence.
    pub fn from_rule(rule: &ClassicalRule, scope: ClaimScope, evidence: Vec<Evidence>) -> Self {
        Self {
            rule_id: rule.id.clone(),
            work: rule.work,
            source_id: rule.source_id.clone(),
            source_clause_id: rule.source_clause_id.clone(),
            source_text_zh_hans: rule.source_text_zh_hans.clone(),
            normalized_note_zh_hans: rule.normalized_note_zh_hans.clone(),
            status: rule.status,
            scope,
            evidence,
        }
    }
}
