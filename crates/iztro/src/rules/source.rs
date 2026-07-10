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

use std::collections::HashMap;
use std::sync::OnceLock;

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

/// Section-level source metadata (卷/节) for a cited source unit.
///
/// The markdown anchor equals `section`, and the source document path is
/// derivable as `docs/zh-CN/sources/quan_shu/volume-{volume:02}.md`; both
/// invariants are enforced against the source inventory by
/// `tests/source_sections.rs`, so this type intentionally stores neither.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SourceSection {
    /// The work the section belongs to.
    pub work: ClassicalWork,
    /// Volume (卷) number within the work.
    pub volume: u8,
    /// Section heading (节), e.g. `太微赋`. Canonical Chinese terminology.
    pub section: String,
}

/// The embedded runtime section table. This is the only source-inventory data
/// parsed at runtime; the full inventory under `rule-corpus/quan-shu/source/`
/// (items, governance status, rule links) remains test-only.
const SECTIONS_TOML: &str = include_str!("../../rule-corpus/quan-shu/sections.toml");

#[derive(Deserialize)]
struct SectionTable {
    section: Vec<SectionEntry>,
}

#[derive(Deserialize)]
struct SectionEntry {
    source_id_prefix: String,
    work: ClassicalWork,
    volume: u8,
    section: String,
}

fn section_table() -> &'static HashMap<String, SourceSection> {
    static TABLE: OnceLock<HashMap<String, SourceSection>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let table: SectionTable =
            toml::from_str(SECTIONS_TOML).expect("embedded section table must deserialize");
        table
            .section
            .into_iter()
            .map(|entry| {
                (
                    entry.source_id_prefix,
                    SourceSection {
                        work: entry.work,
                        volume: entry.volume,
                        section: entry.section,
                    },
                )
            })
            .collect()
    })
}

/// Resolves a source-inventory id (e.g. `quan_shu.v01.tai_wei_fu.ma_yu_kong_wang`)
/// to its section metadata.
///
/// The section prefix is the id truncated after its final `.` (inventory item
/// keys never contain dots). Returns `None` for unknown ids and for
/// [`ClassicalWork::IztroPatternCatalog`] provenance, which has no classical
/// section.
///
/// # Panics
///
/// Panics if the embedded section table TOML fails to deserialize, which can
/// only happen if the committed table is malformed (guarded by tests).
pub fn source_section(source_id: &str) -> Option<&'static SourceSection> {
    let cut = source_id.rfind('.')?;
    section_table().get(&source_id[..=cut])
}
