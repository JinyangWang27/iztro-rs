//! Test-backed coverage report for the 《紫微斗数全书》 source inventory.
//!
//! Like `classical_source_inventory.rs`, this is **corpus-tracking data**, not
//! runtime chart-evaluation data: nothing in `src/` parses the inventory, and
//! `evaluate_classical` never depends on it. This test deserializes the source
//! inventory and the rule corpus, computes coverage metrics, generates a
//! deterministic Markdown report, and asserts that it matches the committed
//! `docs/zh-CN/rules/quan-shu-coverage.md`. This gives a "generated but
//! committed" report without adding an `xtask`: editing the inventory without
//! regenerating the report fails the test.

mod support;

use std::collections::BTreeSet;
use std::fmt::Write as _;
use support::classical_source::{RulesCorpus, SourceInventory, rules_corpus, source_inventory};

const COVERAGE_REPORT: &str = include_str!("../../../docs/zh-CN/rules/quan-shu-coverage.md");

/// Coverage metrics over the whole source inventory. Counts of source items,
/// clauses, and the rules clauses link to (classified by the rule's `status`).
struct CoverageMetrics {
    source_items: usize,
    located_source_items: usize,
    pending_source_items: usize,
    clauses: usize,
    linked_clauses: usize,
    unlinked_clauses: usize,
    linked_rules: usize,
    executable_linked_rules: usize,
    normalized_linked_rules: usize,
    ambiguous_linked_rules: usize,
    rejected_linked_rules: usize,
}

impl CoverageMetrics {
    fn compute(inventory: &SourceInventory, rules: &RulesCorpus) -> Self {
        let source_items = inventory.source_item.len();
        let located_source_items = inventory
            .source_item
            .iter()
            .filter(|item| !item.is_pending())
            .count();
        let pending_source_items = source_items - located_source_items;

        let clauses: usize = inventory.source_item.iter().map(|i| i.clause.len()).sum();
        let linked_clauses = inventory
            .source_item
            .iter()
            .flat_map(|i| &i.clause)
            .filter(|c| !c.linked_rule_ids.is_empty())
            .count();
        let unlinked_clauses = clauses - linked_clauses;

        // Distinct rule ids referenced by any clause, then classified by the
        // rule's own `status` in the corpus.
        let linked_rule_ids: BTreeSet<&str> = inventory
            .source_item
            .iter()
            .flat_map(|i| &i.clause)
            .flat_map(|c| c.linked_rule_ids.iter())
            .map(String::as_str)
            .collect();

        let status_of = |id: &str| -> &str {
            rules
                .rule
                .iter()
                .find(|r| r.id == id)
                .map(|r| r.status.as_str())
                .unwrap_or("")
        };
        let count_status = |want: &str| {
            linked_rule_ids
                .iter()
                .filter(|id| status_of(id) == want)
                .count()
        };

        Self {
            source_items,
            located_source_items,
            pending_source_items,
            clauses,
            linked_clauses,
            unlinked_clauses,
            linked_rules: linked_rule_ids.len(),
            executable_linked_rules: count_status("executable"),
            normalized_linked_rules: count_status("normalized"),
            ambiguous_linked_rules: count_status("ambiguous"),
            rejected_linked_rules: count_status("rejected"),
        }
    }
}

/// Per-section metrics for the `Volume 1 — 太微赋` slice.
struct SectionMetrics {
    source_items: usize,
    clauses: usize,
    linked_clauses: usize,
    unlinked_clauses: usize,
    pending_source_items: usize,
}

impl SectionMetrics {
    fn compute(inventory: &SourceInventory, volume: u8, section: &str) -> Self {
        let items: Vec<_> = inventory
            .source_item
            .iter()
            .filter(|i| i.volume == volume && i.section == section)
            .collect();
        let clauses: usize = items.iter().map(|i| i.clause.len()).sum();
        let linked_clauses = items
            .iter()
            .flat_map(|i| &i.clause)
            .filter(|c| !c.linked_rule_ids.is_empty())
            .count();
        Self {
            source_items: items.len(),
            clauses,
            linked_clauses,
            unlinked_clauses: clauses - linked_clauses,
            pending_source_items: items.iter().filter(|i| i.is_pending()).count(),
        }
    }
}

fn generate_report() -> String {
    let inventory = source_inventory();
    let rules = rules_corpus();
    let m = CoverageMetrics::compute(&inventory, &rules);
    let tai_wei_fu = SectionMetrics::compute(&inventory, 1, "太微赋");

    let mut out = String::new();
    out.push_str("# 《紫微斗数全书》语料覆盖报告\n\n");
    out.push_str("本报告统计 `crates/iztro/rule-corpus/quan-shu/source/` 中已结构化的 source inventory。\n\n");
    out.push_str(
        "本报告由 `crates/iztro/tests/classical_source_coverage.rs` 生成并校验：修改 source \
         inventory 或 rule corpus 后须重新生成本文件，否则测试 \
         `quan_shu_coverage_report_is_current` 失败。\n\n",
    );

    out.push_str("## Summary\n\n");
    out.push_str("| Metric | Count |\n| --- | ---: |\n");
    let _ = writeln!(out, "| Source items | {} |", m.source_items);
    let _ = writeln!(out, "| Located source items | {} |", m.located_source_items);
    let _ = writeln!(out, "| Pending source items | {} |", m.pending_source_items);
    let _ = writeln!(out, "| Clauses | {} |", m.clauses);
    let _ = writeln!(out, "| Linked clauses | {} |", m.linked_clauses);
    let _ = writeln!(out, "| Unlinked clauses | {} |", m.unlinked_clauses);
    let _ = writeln!(out, "| Linked rules | {} |", m.linked_rules);
    let _ = writeln!(
        out,
        "| Executable linked rules | {} |",
        m.executable_linked_rules
    );
    let _ = writeln!(
        out,
        "| Normalized linked rules | {} |",
        m.normalized_linked_rules
    );
    let _ = writeln!(
        out,
        "| Ambiguous linked rules | {} |",
        m.ambiguous_linked_rules
    );
    let _ = writeln!(
        out,
        "| Rejected linked rules | {} |",
        m.rejected_linked_rules
    );

    out.push_str("\n## Volume 1 — 太微赋\n\n");
    out.push_str("| Metric | Count |\n| --- | ---: |\n");
    let _ = writeln!(out, "| Source items | {} |", tai_wei_fu.source_items);
    let _ = writeln!(out, "| Clauses | {} |", tai_wei_fu.clauses);
    let _ = writeln!(out, "| Linked clauses | {} |", tai_wei_fu.linked_clauses);
    let _ = writeln!(
        out,
        "| Unlinked clauses | {} |",
        tai_wei_fu.unlinked_clauses
    );
    let _ = writeln!(
        out,
        "| Pending source items | {} |",
        tai_wei_fu.pending_source_items
    );

    out.push_str("\n## Unlinked clauses\n\n");
    out.push_str("| Source ID | Clause ID | Text |\n| --- | --- | --- |\n");
    for item in &inventory.source_item {
        for clause in &item.clause {
            if clause.linked_rule_ids.is_empty() {
                let _ = writeln!(
                    out,
                    "| {} | {} | {} |",
                    item.source_id, clause.clause_id, clause.text_zh_hans
                );
            }
        }
    }

    out
}

#[test]
fn quan_shu_coverage_report_is_current() {
    let actual = generate_report();
    assert_eq!(
        COVERAGE_REPORT.trim(),
        actual.trim(),
        "docs/zh-CN/rules/quan-shu-coverage.md is stale; regenerate from \
         classical_source_coverage.rs::generate_report"
    );
}
