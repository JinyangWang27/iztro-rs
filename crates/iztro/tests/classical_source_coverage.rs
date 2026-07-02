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
const PATTERN_RULE_SECTIONS: [&str; 4] = ["定富局", "定贵局", "定贫贱局", "定杂局"];
const SEGMENTED_APHORISM_SECTIONS: [&str; 1] = ["斗数骨髓赋"];

/// Coverage metrics over the whole source inventory. Counts of source items and
/// the rules they link to (classified by the rule's `status`).
struct CoverageMetrics {
    source_items: usize,
    located_source_items: usize,
    pending_source_items: usize,
    linked_source_items: usize,
    unlinked_source_items: usize,
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

        let linked_source_items = inventory
            .source_item
            .iter()
            .filter(|item| !item.linked_rule_ids.is_empty())
            .count();
        let unlinked_source_items = source_items - linked_source_items;

        // Distinct rule ids referenced by any source item, then classified by
        // the rule's own `status` in the corpus.
        let linked_rule_ids: BTreeSet<&str> = inventory
            .source_item
            .iter()
            .flat_map(|i| i.linked_rule_ids.iter())
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
            linked_source_items,
            unlinked_source_items,
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
    linked_source_items: usize,
    unlinked_source_items: usize,
    pending_source_items: usize,
}

impl SectionMetrics {
    fn compute(inventory: &SourceInventory, volume: u8, section: &str) -> Self {
        let items: Vec<_> = inventory
            .source_item
            .iter()
            .filter(|i| i.volume == volume && i.section == section)
            .collect();
        let linked_source_items = items
            .iter()
            .filter(|i| !i.linked_rule_ids.is_empty())
            .count();
        Self {
            source_items: items.len(),
            linked_source_items,
            unlinked_source_items: items.len() - linked_source_items,
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
    out.push_str(
        "本报告统计 `crates/iztro/rule-corpus/quan-shu/source/` 中已结构化的 source \
         inventory，仅覆盖《紫微斗数全书》出处条目，不包含项目 pattern/格局规则目录\
         （`crates/iztro/rule-corpus/patterns/`）。\n\n",
    );
    out.push_str(
        "source inventory 以原子 source item 记录每条受引出处单元；`source_id` 为稳定助记符，\
         `source_order` 单独保存出处顺序。规则经 `linked_rule_ids` 链接。\n\n",
    );
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
    let _ = writeln!(out, "| Linked source items | {} |", m.linked_source_items);
    let _ = writeln!(
        out,
        "| Unlinked source items | {} |",
        m.unlinked_source_items
    );
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
    let _ = writeln!(
        out,
        "| Linked source items | {} |",
        tai_wei_fu.linked_source_items
    );
    let _ = writeln!(
        out,
        "| Unlinked source items | {} |",
        tai_wei_fu.unlinked_source_items
    );
    let _ = writeln!(
        out,
        "| Pending source items | {} |",
        tai_wei_fu.pending_source_items
    );

    for section in PATTERN_RULE_SECTIONS {
        let metrics = SectionMetrics::compute(&inventory, 1, section);
        out.push_str("\n## Volume 1 — ");
        out.push_str(section);
        out.push_str("\n\n");
        out.push_str("Category: `pattern_rule`\n\n");
        out.push_str("| Metric | Count |\n| --- | ---: |\n");
        let _ = writeln!(out, "| Source items | {} |", metrics.source_items);
        let _ = writeln!(
            out,
            "| Linked classical rule source items | {} |",
            metrics.linked_source_items
        );
        let _ = writeln!(
            out,
            "| Segmented pattern-only source items | {} |",
            metrics.unlinked_source_items
        );
        let _ = writeln!(
            out,
            "| Pending source items | {} |",
            metrics.pending_source_items
        );
    }

    for section in SEGMENTED_APHORISM_SECTIONS {
        let metrics = SectionMetrics::compute(&inventory, 1, section);
        out.push_str("\n## Volume 1 — ");
        out.push_str(section);
        out.push_str("\n\n");
        out.push_str("Category: `aphorism_rule`\n\n");
        out.push_str("| Metric | Count |\n| --- | ---: |\n");
        let _ = writeln!(out, "| Source items | {} |", metrics.source_items);
        let _ = writeln!(
            out,
            "| Linked classical rule source items | {} |",
            metrics.linked_source_items
        );
        let _ = writeln!(
            out,
            "| Segmented aphorism source items | {} |",
            metrics.unlinked_source_items
        );
        let _ = writeln!(
            out,
            "| Pending source items | {} |",
            metrics.pending_source_items
        );
    }

    out.push_str("\n## Unlinked source items\n\n");
    out.push_str("| Source ID | Order | Text |\n| --- | ---: | --- |\n");
    for item in &inventory.source_item {
        if item.linked_rule_ids.is_empty() {
            let _ = writeln!(
                out,
                "| {} | {} | {} |",
                item.source_id, item.source_order, item.source_text_zh_hans
            );
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

/// The coverage report is deterministic: regenerating it from the same committed
/// inventory and corpus yields byte-identical output. Status counts come from a
/// `BTreeSet` of rule ids, so ordering and totals do not depend on corpus order.
#[test]
fn quan_shu_coverage_report_is_deterministic() {
    assert_eq!(generate_report(), generate_report());
}

/// After completing the 太微赋 normalization map every 太微赋 aphorism source
/// item is linked. Source-backed pattern catalogues and newly segmented
/// aphorism sections may be unlinked while they remain source-inventory
/// governance data, not runtime claim rules.
#[test]
fn quan_shu_coverage_unlinked_items_are_intentionally_segmented_sources() {
    let inventory = source_inventory();
    let rules = rules_corpus();
    let m = CoverageMetrics::compute(&inventory, &rules);
    let unlinked: Vec<_> = inventory
        .source_item
        .iter()
        .filter(|item| item.linked_rule_ids.is_empty())
        .collect();

    assert_eq!(m.unlinked_source_items, unlinked.len());
    for item in unlinked {
        let allowed_segmented_source = item.category == "pattern_rule"
            || (item.category == "aphorism_rule"
                && SEGMENTED_APHORISM_SECTIONS.contains(&item.section.as_str()));
        assert!(
            allowed_segmented_source,
            "unlinked item {} must be an intentionally segmented source, not missed classical rule work",
            item.source_id
        );
        assert_eq!(
            item.status, "segmented",
            "unlinked item {} must be segmented until a non-claim pattern metadata path links it",
            item.source_id
        );
    }
    assert_eq!(
        m.executable_linked_rules
            + m.normalized_linked_rules
            + m.ambiguous_linked_rules
            + m.rejected_linked_rules,
        m.linked_rules,
        "every linked rule must fall into a known status bucket"
    );
}
