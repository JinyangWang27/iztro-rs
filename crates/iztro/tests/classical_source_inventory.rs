//! Test-only validation for the 《紫微斗数全书》 source inventory.
//!
//! The source inventory under `rule-corpus/quan-shu/source/` is
//! **corpus-management data**, not runtime chart-evaluation data: nothing in
//! `src/` parses it, and `evaluate_classical` never depends on it. These tests
//! exist only to keep the inventory internally consistent and correctly linked
//! to the executable rule corpus (`rule-corpus/quan-shu/rules.toml`).
//!
//! The structs below are private, test-only deserialization shapes. They are
//! intentionally not exported from the crate; adding runtime APIs purely to
//! validate corpus tracking data would blur the layer boundary.

use std::collections::HashSet;

/// Test-only mirror of `rule-corpus/quan-shu/source/volume-01.toml`.
#[derive(Debug, serde::Deserialize)]
struct SourceInventory {
    source_item: Vec<SourceItem>,
}

#[derive(Debug, serde::Deserialize)]
struct SourceItem {
    source_id: String,
    work: String,
    #[allow(dead_code)]
    volume: u8,
    section: String,
    category: String,
    status: String,
    doc_path: String,
    anchor: String,
    source_text_zh_hans: String,
    normalized_clause_zh_hans: Option<String>,
    linked_rule_ids: Vec<String>,
    #[allow(dead_code)]
    notes_zh_hans: Option<String>,
}

/// Test-only minimal mirror of `rule-corpus/quan-shu/rules.toml`. The runtime
/// corpus loader (`iztro::rules::classical::quan_shu_rules`) deserializes the
/// full typed metadata; here we only need the fields that link rules to the
/// source inventory, so we keep an independent minimal struct rather than
/// exposing the runtime types for the IDs and Chinese strings.
#[derive(Debug, serde::Deserialize)]
struct RulesCorpus {
    rule: Vec<RuleEntry>,
}

#[derive(Debug, serde::Deserialize)]
struct RuleEntry {
    id: String,
    source_id: String,
    work: String,
    #[allow(dead_code)]
    source_text_zh_hans: String,
}

const SOURCE_INVENTORY_TOML: &str = include_str!("../rule-corpus/quan-shu/source/volume-01.toml");
const RULES_CORPUS_TOML: &str = include_str!("../rule-corpus/quan-shu/rules.toml");

fn source_inventory() -> SourceInventory {
    toml::from_str(SOURCE_INVENTORY_TOML).expect("QuanShu source inventory TOML must deserialize")
}

fn rules_corpus() -> RulesCorpus {
    toml::from_str(RULES_CORPUS_TOML).expect("QuanShu rule corpus TOML must deserialize")
}

#[test]
fn quan_shu_source_inventory_parses() {
    let inventory = source_inventory();
    assert!(
        !inventory.source_item.is_empty(),
        "source inventory must record at least one source item"
    );
}

#[test]
fn quan_shu_source_inventory_has_unique_source_ids() {
    let inventory = source_inventory();
    let mut seen = HashSet::new();
    for item in &inventory.source_item {
        assert!(
            seen.insert(item.source_id.as_str()),
            "duplicate source_id in inventory: {}",
            item.source_id
        );
    }
}

#[test]
fn quan_shu_source_inventory_required_fields_are_not_empty() {
    let inventory = source_inventory();
    for item in &inventory.source_item {
        // `anchor = "TODO"` and `section = "待校"` are intentionally allowed in
        // the pilot slice, so we only require non-empty, not non-placeholder.
        for (field, value) in [
            ("source_id", &item.source_id),
            ("work", &item.work),
            ("section", &item.section),
            ("category", &item.category),
            ("status", &item.status),
            ("doc_path", &item.doc_path),
            ("anchor", &item.anchor),
            ("source_text_zh_hans", &item.source_text_zh_hans),
        ] {
            assert!(
                !value.trim().is_empty(),
                "source item {} has empty {field}",
                item.source_id
            );
        }
    }
}

#[test]
fn classical_rules_reference_known_source_items() {
    let inventory = source_inventory();
    let rules = rules_corpus();
    let source_ids: HashSet<&str> = inventory
        .source_item
        .iter()
        .map(|item| item.source_id.as_str())
        .collect();

    for rule in &rules.rule {
        assert!(
            source_ids.contains(rule.source_id.as_str()),
            "rule {} references source_id {} not present in the source inventory",
            rule.id,
            rule.source_id
        );
    }
}

#[test]
fn source_inventory_linked_rule_ids_exist() {
    let inventory = source_inventory();
    let rules = rules_corpus();
    let rule_ids: HashSet<&str> = rules.rule.iter().map(|r| r.id.as_str()).collect();

    for item in &inventory.source_item {
        for linked in &item.linked_rule_ids {
            assert!(
                rule_ids.contains(linked.as_str()),
                "source item {} links to unknown rule id {linked}",
                item.source_id
            );
        }
    }
}

#[test]
fn source_inventory_links_match_rule_source_ids() {
    let inventory = source_inventory();
    let rules = rules_corpus();

    for item in &inventory.source_item {
        for linked in &item.linked_rule_ids {
            let rule = rules
                .rule
                .iter()
                .find(|r| r.id == *linked)
                .unwrap_or_else(|| {
                    panic!(
                        "source item {} links to unknown rule id {linked}",
                        item.source_id
                    )
                });
            assert_eq!(
                rule.source_id, item.source_id,
                "rule {} source_id {} disagrees with linking source item {}",
                rule.id, rule.source_id, item.source_id
            );
            assert_eq!(
                rule.work, item.work,
                "rule {} work {} disagrees with linking source item {}",
                rule.id, rule.work, item.source_id
            );
        }
    }
}

/// Regression: the imported Volume 1 太微赋 wording is "马遇空亡，终身奔走".
/// The pilot rule's own `source_text_zh_hans` may keep the variant "马落空亡",
/// but the source inventory must not regress to that variant.
#[test]
fn tian_ma_void_source_uses_imported_wording() {
    const CANONICAL: &str = "马遇空亡，终身奔走";
    const REGRESSION: &str = "马落空亡，终身奔走";
    const RULE_ID: &str = "migration.tian_ma_void.restless_movement";

    let inventory = source_inventory();
    let item = inventory
        .source_item
        .iter()
        .find(|item| item.linked_rule_ids.iter().any(|id| id == RULE_ID))
        .unwrap_or_else(|| panic!("no source item links to {RULE_ID}"));

    let normalized = item.normalized_clause_zh_hans.as_deref().unwrap_or("");
    let uses_canonical =
        item.source_text_zh_hans.contains(CANONICAL) || normalized.contains(CANONICAL);
    assert!(
        uses_canonical,
        "source item {} must use imported wording {CANONICAL:?}",
        item.source_id
    );

    let regresses =
        item.source_text_zh_hans.contains(REGRESSION) || normalized.contains(REGRESSION);
    assert!(
        !regresses,
        "source item {} must not regress to {REGRESSION:?}",
        item.source_id
    );
}
