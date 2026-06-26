//! Compact, renderer-facing classical rule hit references.
//!
//! [`ClassicalSourceHit`] is the full evaluation-facing provenance record: it
//! embeds the verbatim `source_text_zh_hans` for every match. For a layer-level
//! analysis API that may produce many hits across many cached layers, duplicating
//! that source text per hit is wasteful and pushes canonical source text toward
//! the GUI.
//!
//! [`ClassicalRuleHitRef`] is the compact alternative: it carries only the stable
//! `rule_id`, the matched [`ClaimScope`], an optional `claim_key`, and structured
//! [`Evidence`]. A renderer resolves verbatim source text and other display
//! metadata once per rule id via
//! [`classical_rule_metadata`](crate::rules::classical::metadata::classical_rule_metadata).
//!
//! [`ClassicalSourceHit`]: crate::rules::classical::source_hit::ClassicalSourceHit

use serde::{Deserialize, Serialize};

use crate::rules::classical::claim::ClaimScope;
use crate::rules::classical::evidence::Evidence;
use crate::rules::classical::metadata::classical_rule_metadata;
use crate::rules::classical::rule::ClassicalRuleId;
use crate::rules::classical::source_hit::ClassicalSourceHit;

/// A compact reference to one matched classical rule.
///
/// This is enough for a GUI to:
///
/// 1. group hits by [`scope`](Self::scope);
/// 2. resolve verbatim source text by [`rule_id`](Self::rule_id) through
///    [`classical_rule_metadata`];
/// 3. resolve localized claim text by [`claim_key`](Self::claim_key);
/// 4. later support highlighting through [`evidence`](Self::evidence).
///
/// It intentionally omits `source_text_zh_hans` so source text stays canonical in
/// the rule corpus rather than being copied into every hit.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ClassicalRuleHitRef {
    /// The rule whose predicate matched.
    pub rule_id: ClassicalRuleId,
    /// The scope the rule matched in.
    pub scope: ClaimScope,
    /// The i18n key for the produced claim, when the rule has claim metadata.
    pub claim_key: Option<String>,
    /// Machine-readable evidence for the match.
    pub evidence: Vec<Evidence>,
}

impl ClassicalRuleHitRef {
    /// Builds a compact hit ref from a full [`ClassicalSourceHit`].
    ///
    /// The `claim_key` is resolved from static rule metadata rather than read off
    /// the source hit (which carries no claim key), keeping the compact ref's key
    /// consistent with [`classical_rule_metadata`].
    pub fn from_source_hit(hit: &ClassicalSourceHit) -> Self {
        let claim_key = classical_rule_metadata(hit.rule_id.clone())
            .and_then(|metadata| metadata.claim_key.map(str::to_string));
        Self {
            rule_id: hit.rule_id.clone(),
            scope: hit.scope,
            claim_key,
            evidence: hit.evidence.clone(),
        }
    }
}
