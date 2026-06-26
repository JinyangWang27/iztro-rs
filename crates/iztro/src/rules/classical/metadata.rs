//! Static rule metadata lookup for renderer/GUI resolution.
//!
//! A [`ClassicalRuleHitRef`] keeps results compact by carrying only a
//! [`ClassicalRuleId`], a [`ClaimScope`], an optional claim key, and structured
//! evidence. It deliberately does **not** duplicate the verbatim
//! `source_text_zh_hans` into every hit. Instead, downstream consumers (the
//! future GUI's 全书规则 tab) resolve the canonical source text — and other
//! display metadata — once per rule id through [`classical_rule_metadata`].
//!
//! [`source_text_zh_hans`](ClassicalRuleMetadata::source_text_zh_hans) is the
//! **verbatim** classical source clause, identical to
//! [`ClassicalRule::source_text_zh_hans`](crate::rules::classical::rule::ClassicalRule::source_text_zh_hans).
//! It must never carry an interpretation, paraphrase, or claim text.
//!
//! [`ClassicalRuleHitRef`]: crate::rules::classical::hit_ref::ClassicalRuleHitRef

use std::sync::OnceLock;

use crate::rules::classical::claim::ClaimScope;
use crate::rules::classical::corpus::classical_rules;
use crate::rules::classical::rule::ClassicalRuleId;
use crate::rules::classical::source::ClassicalWork;

/// Default applicable scope set for current executable rules.
///
/// Current 《紫微斗数全书》 / 太微赋 executable rules carry no explicit temporal
/// semantics, so they default to natal-only. Temporal rules are not auto-applied
/// to every scope; a rule must declare wider scopes explicitly before it appears
/// outside [`ClaimScope::Natal`].
const NATAL_ONLY_SCOPES: &[ClaimScope] = &[ClaimScope::Natal];

/// Static, display-facing metadata for one classical rule.
///
/// This is the canonical place a renderer resolves a [`ClassicalRuleId`] back to
/// its verbatim source clause and display attributes. All string fields borrow
/// the process-lifetime corpus, so the lookup hands out `&'static` references
/// without cloning per call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassicalRuleMetadata {
    /// Stable rule identifier.
    pub rule_id: ClassicalRuleId,
    /// The classical work the rule is drawn from.
    pub work: ClassicalWork,
    /// Stable identifier for the source unit or pattern metadata entry.
    pub source_id: &'static str,
    /// Optional legacy/pattern provenance discriminator.
    pub source_clause_id: Option<&'static str>,
    /// Verbatim canonical source clause, Simplified Chinese.
    ///
    /// This is the source of truth a renderer resolves `rule_id` to. It is never
    /// an interpretation or claim text.
    pub source_text_zh_hans: &'static str,
    /// Optional normalization note, Simplified Chinese.
    pub normalized_note_zh_hans: Option<&'static str>,
    /// The i18n key for the produced claim, when the rule has claim metadata.
    pub claim_key: Option<&'static str>,
    /// The scopes this rule may be asserted within.
    ///
    /// Current executable rules are natal-only (`NATAL_ONLY_SCOPES`); this does
    /// not promote QuanShu rules to every temporal scope automatically.
    pub applicable_scopes: &'static [ClaimScope],
}

/// Builds the process-lifetime metadata table from the embedded corpus once.
fn metadata_table() -> &'static [ClassicalRuleMetadata] {
    static TABLE: OnceLock<Vec<ClassicalRuleMetadata>> = OnceLock::new();
    TABLE
        .get_or_init(|| {
            classical_rules()
                .iter()
                .map(|rule| ClassicalRuleMetadata {
                    rule_id: rule.id.clone(),
                    work: rule.work,
                    source_id: rule.source_id.as_str(),
                    source_clause_id: rule.source_clause_id.as_deref(),
                    source_text_zh_hans: rule.source_text_zh_hans.as_str(),
                    normalized_note_zh_hans: rule.normalized_note_zh_hans.as_deref(),
                    claim_key: rule.claim.as_ref().map(|claim| claim.claim_key.as_str()),
                    applicable_scopes: NATAL_ONLY_SCOPES,
                })
                .collect()
        })
        .as_slice()
}

/// Resolves a [`ClassicalRuleId`] to its static display metadata, if known.
///
/// Returns `None` for unknown ids. The returned reference is process-lifetime, so
/// a GUI can cache it freely and resolve verbatim `source_text_zh_hans` without
/// re-cloning per rule hit.
///
/// This lookup is intentionally **work-agnostic**: it resolves metadata for any
/// rule id, including [`ClassicalWork::IztroPatternCatalog`] entries. Filtering by
/// work is a concern of the rule *stream*, not this metadata table. The future
/// 全书规则 tab should therefore consume QuanShu-filtered rule hits — e.g. those
/// produced by [`AnalysisLayerRequest::user_facing`], which restricts `works` to
/// [`ClassicalWork::ZiWeiDouShuQuanShu`] — rather than relying on this lookup to
/// exclude pattern-catalog rules.
///
/// [`AnalysisLayerRequest::user_facing`]: crate::analysis::AnalysisLayerRequest::user_facing
pub fn classical_rule_metadata(rule_id: ClassicalRuleId) -> Option<&'static ClassicalRuleMetadata> {
    metadata_table()
        .iter()
        .find(|metadata| metadata.rule_id == rule_id)
}
