//! Test-only validation for the 《紫微斗数全书》 source inventory.
//!
//! The source inventory under `rule-corpus/quan-shu/source/` is
//! **corpus-management data**, not runtime chart-evaluation data: nothing in
//! `src/` parses it, and `evaluate_classical` never depends on it. These tests
//! exist only to keep the inventory internally consistent and correctly linked
//! to the executable rule corpus (`rule-corpus/quan-shu/rules.toml`).
//!
//! Structure: a `source_item` is a source passage/location identified by
//! `source_id`; each item holds one or more `clause`s (individual candidate rule
//! phrases identified by `clause_id`); a rule links to a clause by carrying both
//! `source_id` and `source_clause_id`, and the clause mirrors that link via
//! `linked_rule_ids`.
//!
//! The deserialization shapes live in the shared, test-only `support` module
//! (`tests/support/classical_source.rs`). They are intentionally not exported
//! from the crate; adding runtime APIs purely to validate corpus tracking data
//! would blur the layer boundary.

mod support;

use std::collections::HashSet;
use support::classical_source::{rules_corpus, source_inventory, strip_punct};

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
        assert!(
            !item.clause.is_empty(),
            "source item {} must record at least one clause",
            item.source_id
        );
    }
}

/// 5.1 Clause IDs are unique within a source item.
#[test]
fn clause_ids_are_unique_within_a_source_item() {
    let inventory = source_inventory();
    for item in &inventory.source_item {
        let mut seen = HashSet::new();
        for clause in &item.clause {
            assert!(
                seen.insert(clause.clause_id.as_str()),
                "duplicate clause_id {} within source item {}",
                clause.clause_id,
                item.source_id
            );
        }
    }
}

/// 5.2 Clause id and text are non-empty.
#[test]
fn clause_required_fields_are_not_empty() {
    let inventory = source_inventory();
    for item in &inventory.source_item {
        for clause in &item.clause {
            assert!(
                !clause.clause_id.trim().is_empty(),
                "source item {} has a clause with empty clause_id",
                item.source_id
            );
            assert!(
                !clause.text_zh_hans.trim().is_empty(),
                "source item {} clause {} has empty text_zh_hans",
                item.source_id,
                clause.clause_id
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

/// 5.3 Every rule `source_clause_id` exists inside its referenced source item.
#[test]
fn rule_source_clause_ids_exist_in_their_source_item() {
    let inventory = source_inventory();
    let rules = rules_corpus();

    for rule in &rules.rule {
        let Some(clause_id) = rule.source_clause_id.as_deref() else {
            continue;
        };
        let item = inventory
            .source_item
            .iter()
            .find(|item| item.source_id == rule.source_id)
            .unwrap_or_else(|| {
                panic!(
                    "rule {} references source_id {} not present in the source inventory",
                    rule.id, rule.source_id
                )
            });
        assert!(
            item.clause.iter().any(|c| c.clause_id == clause_id),
            "rule {} references clause {clause_id} not present in source item {}",
            rule.id,
            item.source_id
        );
    }
}

#[test]
fn source_inventory_linked_rule_ids_exist() {
    let inventory = source_inventory();
    let rules = rules_corpus();
    let rule_ids: HashSet<&str> = rules.rule.iter().map(|r| r.id.as_str()).collect();

    for item in &inventory.source_item {
        for clause in &item.clause {
            for linked in &clause.linked_rule_ids {
                assert!(
                    rule_ids.contains(linked.as_str()),
                    "source item {} clause {} links to unknown rule id {linked}",
                    item.source_id,
                    clause.clause_id
                );
            }
        }
    }
}

/// 5.4 Linked clauses and rules agree on source_id, clause_id, and work.
#[test]
fn source_inventory_clause_links_match_rules() {
    let inventory = source_inventory();
    let rules = rules_corpus();

    for item in &inventory.source_item {
        for clause in &item.clause {
            for linked in &clause.linked_rule_ids {
                let rule = rules
                    .rule
                    .iter()
                    .find(|r| r.id == *linked)
                    .unwrap_or_else(|| {
                        panic!(
                            "source item {} clause {} links to unknown rule id {linked}",
                            item.source_id, clause.clause_id
                        )
                    });
                assert_eq!(
                    rule.source_id, item.source_id,
                    "rule {} source_id {} disagrees with linking source item {}",
                    rule.id, rule.source_id, item.source_id
                );
                assert_eq!(
                    rule.source_clause_id.as_deref(),
                    Some(clause.clause_id.as_str()),
                    "rule {} source_clause_id {:?} disagrees with linking clause {}",
                    rule.id,
                    rule.source_clause_id,
                    clause.clause_id
                );
                assert_eq!(
                    rule.work, item.work,
                    "rule {} work {} disagrees with linking source item {}",
                    rule.id, rule.work, item.source_id
                );
            }
        }
    }
}

/// 5.5 Clause text matches or contains the linked rule's source text.
///
/// For `executable` rules we normally require containment in either direction.
/// Two pilot rules carry already-normalized claim phrasing that does not match
/// its clause verbatim (`wealth.lu_ma_remote_wealth` is `normalized`;
/// `life.ri_yue_fan_bei.hardship_pressure` is `executable`). Those are accepted
/// only when the clause or source item documents the divergence via
/// `notes_zh_hans`. See docs/zh-CN/sources/quan_shu/README.md.
#[test]
fn clause_text_matches_or_contains_rule_source_text() {
    let inventory = source_inventory();
    let rules = rules_corpus();

    for item in &inventory.source_item {
        for clause in &item.clause {
            let clause_text = strip_punct(&clause.text_zh_hans);
            for linked in &clause.linked_rule_ids {
                let rule = rules
                    .rule
                    .iter()
                    .find(|r| r.id == *linked)
                    .expect("linked rule must exist");
                let rule_text = strip_punct(&rule.source_text_zh_hans);
                let contained =
                    clause_text.contains(&rule_text) || rule_text.contains(&clause_text);
                let documented = clause.notes_zh_hans.is_some() || item.notes_zh_hans.is_some();
                assert!(
                    contained || documented,
                    "rule {} (status {}) source text {:?} neither matches nor is contained by \
                     clause {} text {:?}, and no notes_zh_hans explains the divergence",
                    rule.id,
                    rule.status,
                    rule.source_text_zh_hans,
                    clause.clause_id,
                    clause.text_zh_hans
                );
            }
        }
    }
}

/// 5.6 For located source items, the passage text contains every clause text.
///
/// Pending items (`section = "待校"` / `anchor = "TODO"`) are not yet located, so
/// this is skipped for them.
#[test]
fn located_source_text_contains_each_clause() {
    let inventory = source_inventory();
    for item in &inventory.source_item {
        if item.is_pending() {
            continue;
        }
        let passage = strip_punct(&item.source_text_zh_hans);
        for clause in &item.clause {
            let clause_text = strip_punct(&clause.text_zh_hans);
            assert!(
                passage.contains(&clause_text),
                "located source item {} text {:?} does not contain clause {} text {:?}",
                item.source_id,
                item.source_text_zh_hans,
                clause.clause_id,
                clause.text_zh_hans
            );
        }
    }
}

/// The 天马空亡 clause must use the imported Volume 1 太微赋 wording
/// "马遇空亡，终身奔走".
#[test]
fn tian_ma_void_source_uses_imported_wording() {
    const CANONICAL: &str = "马遇空亡，终身奔走";
    const RULE_ID: &str = "migration.tian_ma_void.restless_movement";

    let inventory = source_inventory();
    let clause = inventory
        .source_item
        .iter()
        .flat_map(|item| &item.clause)
        .find(|clause| clause.linked_rule_ids.iter().any(|id| id == RULE_ID))
        .unwrap_or_else(|| panic!("no clause links to {RULE_ID}"));

    assert_eq!(
        clause.text_zh_hans, CANONICAL,
        "clause linking {RULE_ID} must use imported wording {CANONICAL:?}"
    );
}
