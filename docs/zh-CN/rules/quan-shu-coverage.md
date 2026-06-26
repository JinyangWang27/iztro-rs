# 《紫微斗数全书》语料覆盖报告

本报告统计 `crates/iztro/rule-corpus/quan-shu/source/` 中已结构化的 source inventory，仅覆盖《紫微斗数全书》出处条目，不包含项目 pattern/格局规则目录（`crates/iztro/rule-corpus/patterns/`）。

source inventory 以原子 source item 记录每条受引出处单元；`source_id` 为稳定助记符，`source_order` 单独保存出处顺序。规则经 `linked_rule_ids` 链接。

本报告由 `crates/iztro/tests/classical_source_coverage.rs` 生成并校验：修改 source inventory 或 rule corpus 后须重新生成本文件，否则测试 `quan_shu_coverage_report_is_current` 失败。

## Summary

| Metric | Count |
| --- | ---: |
| Source items | 64 |
| Located source items | 64 |
| Pending source items | 0 |
| Linked source items | 64 |
| Unlinked source items | 0 |
| Linked rules | 64 |
| Executable linked rules | 2 |
| Normalized linked rules | 44 |
| Ambiguous linked rules | 17 |
| Rejected linked rules | 1 |

## Volume 1 — 太微赋

| Metric | Count |
| --- | ---: |
| Source items | 64 |
| Linked source items | 64 |
| Unlinked source items | 0 |
| Pending source items | 0 |

## Unlinked source items

| Source ID | Order | Text |
| --- | ---: | --- |
