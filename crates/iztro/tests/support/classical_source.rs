//! Test-only deserialization shapes for the 《紫微斗数全书》 source inventory and
//! its linked rule corpus.
//!
//! These mirror `rule-corpus/quan-shu/source/volume-01.toml` and
//! `rule-corpus/quan-shu/rules.toml`. They are intentionally private to the
//! test binaries and not exported from the crate: adding runtime APIs purely to
//! validate corpus-tracking data would blur the layer boundary. Each binary
//! uses a different subset of the fields, so the structs carry
//! `#[allow(dead_code)]`.

#![allow(dead_code)]

/// Test-only raw mirror of the **canonical** grouped
/// `rule-corpus/quan-shu/source/volume-01.toml`. A `source_group` carries shared
/// metadata / section defaults; each `source_group.item` is one atomic cited
/// source unit. The flat [`SourceItem`] view used by the tests is produced by
/// [`RawSourceInventory::expand`], which joins each item with its group defaults.
#[derive(Debug, serde::Deserialize)]
pub struct RawSourceInventory {
    pub source_group: Vec<SourceGroup>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SourceGroup {
    pub source_id_prefix: String,
    pub work: String,
    pub volume: u8,
    pub section: String,
    pub category: String,
    pub status: String,
    pub doc_path: String,
    pub anchor: String,
    pub item: Vec<SourceGroupItem>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SourceGroupItem {
    pub key: String,
    pub source_order: usize,
    pub source_text_zh_hans: String,
    #[serde(default)]
    pub linked_rule_ids: Vec<String>,
    pub notes_zh_hans: Option<String>,
}

impl RawSourceInventory {
    /// Expand the grouped TOML into flat source items: `source_id` is
    /// `source_id_prefix + item.key`, and the group metadata is copied onto each
    /// item. Tests operate on this flat view and stay agnostic to the grouping.
    pub fn expand(self) -> SourceInventory {
        let mut source_item = Vec::new();
        for group in self.source_group {
            for item in group.item {
                source_item.push(SourceItem {
                    source_id: format!("{}{}", group.source_id_prefix, item.key),
                    source_order: item.source_order,
                    work: group.work.clone(),
                    volume: group.volume,
                    section: group.section.clone(),
                    category: group.category.clone(),
                    status: group.status.clone(),
                    doc_path: group.doc_path.clone(),
                    anchor: group.anchor.clone(),
                    source_text_zh_hans: item.source_text_zh_hans,
                    linked_rule_ids: item.linked_rule_ids,
                    notes_zh_hans: item.notes_zh_hans,
                });
            }
        }
        SourceInventory { source_item }
    }
}

/// Expanded, flat view of the source inventory. Each [`SourceItem`] is one
/// atomic cited source unit with its group defaults already applied.
#[derive(Debug)]
pub struct SourceInventory {
    pub source_item: Vec<SourceItem>,
}

#[derive(Debug)]
pub struct SourceItem {
    pub source_id: String,
    pub source_order: usize,
    pub work: String,
    pub volume: u8,
    pub section: String,
    pub category: String,
    pub status: String,
    pub doc_path: String,
    pub anchor: String,
    pub source_text_zh_hans: String,
    pub linked_rule_ids: Vec<String>,
    pub notes_zh_hans: Option<String>,
}

impl SourceItem {
    /// Pending items (not yet located in the Markdown volumes) are flagged with
    /// the placeholder `section`/`anchor`. They count as pending in the coverage
    /// report.
    pub fn is_pending(&self) -> bool {
        self.anchor == "TODO" || self.section == "待校"
    }
}

/// Test-only minimal mirror of `rule-corpus/quan-shu/rules.toml`. The runtime
/// corpus loader deserializes the full typed metadata; here we only need the
/// fields that link rules to the source inventory plus the coverage `status`.
#[derive(Debug, serde::Deserialize)]
pub struct RulesCorpus {
    pub rule: Vec<RuleEntry>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RuleEntry {
    pub id: String,
    pub source_id: String,
    #[serde(default)]
    pub source_clause_id: Option<String>,
    pub work: String,
    pub source_text_zh_hans: String,
    #[serde(default)]
    pub normalized_note_zh_hans: Option<String>,
    pub status: String,
}

const SOURCE_INVENTORY_TOML: &str =
    include_str!("../../rule-corpus/quan-shu/source/volume-01.toml");
const RULES_CORPUS_TOML: &str = include_str!("../../rule-corpus/quan-shu/rules.toml");
const PATTERN_RULES_CORPUS_TOML: &str = include_str!("../../rule-corpus/patterns/rules.toml");

/// The serde name of the only work the QuanShu source inventory validates
/// against. Rules with any other `work` (e.g. the pattern catalog) are not
/// QuanShu source rules and must not be required in the inventory.
pub const QUAN_SHU_WORK: &str = "zi_wei_dou_shu_quan_shu";

/// Deserializes the canonical grouped TOML and expands it into the flat
/// source-item view the tests operate on.
pub fn source_inventory() -> SourceInventory {
    let raw: RawSourceInventory = toml::from_str(SOURCE_INVENTORY_TOML)
        .expect("QuanShu source inventory TOML must deserialize");
    raw.expand()
}

/// The 《紫微斗数全书》 rule corpus (`rule-corpus/quan-shu/rules.toml`).
pub fn rules_corpus() -> RulesCorpus {
    toml::from_str(RULES_CORPUS_TOML).expect("QuanShu rule corpus TOML must deserialize")
}

/// The project pattern/格局 rule corpus (`rule-corpus/patterns/rules.toml`).
/// These are not QuanShu source rules and are not tracked by the QuanShu
/// source inventory.
pub fn pattern_rules_corpus() -> RulesCorpus {
    toml::from_str(PATTERN_RULES_CORPUS_TOML).expect("pattern rule corpus TOML must deserialize")
}

/// Strips the punctuation we treat as insignificant when comparing Chinese
/// passage/clause/rule text. Intentionally light: we do not normalize the
/// characters themselves, only drop separators so containment is not defeated by
/// a trailing comma or full stop.
pub fn strip_punct(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, '，' | '。' | '、' | '；' | '：' | ' '))
        .collect()
}
