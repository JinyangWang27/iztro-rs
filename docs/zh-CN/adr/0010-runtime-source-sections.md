# ADR 0010：运行时卷节表

状态：已接受

## 背景

显示层（首先是 GUI 检查器）需要标注经典规则命中与有出处格局检测的来源：
典籍、卷、节，并指向 `docs/zh-CN/sources/quan_shu/` 下的规范 Markdown 卷册。

这些元数据此前仅存在于 QuanShu source inventory
（`rule-corpus/quan-shu/source/volume-*.toml`）中，而 inventory 是仅由测试解析的
语料治理数据（ADR 0008 时期的边界：「`src/` 不解析它」）。运行时规则只携带
`source_id` 与逐字出处文本；`PatternSourceMetadata` 则手工重复了一份 `section`。

若在运行时内嵌完整 inventory，将为主要是治理记账（条目顺序、链接状态、分类）
的数据付出约 100KB 的二进制体积；若把卷/节反规范化进 64 条 `[[rule]]`，
则是把组级事实复制到每条规则上，且仍覆盖不到格局检测。

## 决定

新增小型**运行时卷节表** `rule-corpus/quan-shu/sections.toml`：
inventory 中每个 `source_group` 对应一条，仅含 `source_id_prefix`、`work`、
`volume`、`section`。经 `include_str!` 内嵌，暴露为
`rules::source::source_section(source_id) -> Option<&'static SourceSection>`。

- 查找方式：把 id 截断到最后一个 `.` 之后，做 `HashMap` 精确命中。之所以成立，
  是因为 inventory 的 item key 从不含点号——同步测试通过解析每个 inventory 条目
  隐式强制该不变式。
- `SourceSection` 不存 `anchor`（恒等于 `section`），也不存 `doc_path`
  （恒可推导为 `docs/zh-CN/sources/quan_shu/volume-{volume:02}.md`）。
  同步测试（`tests/source_sections.rs`）双向校验这两条可推导性不变式，
  未定位（`待校`/`TODO`）单元豁免。
- `SourceSection` 与 `SourceRef`/`ClassicalWork` 同住 `rules::source`：
  引文即来源的元数据，不是独立概念。
- 移除 `PatternSourceMetadata.section`；卷节表成为规则命中与格局检测的
  卷节元数据在运行时的唯一属主。
- 完整 inventory 及全部 item 级/治理字段（`status`、`category`、
  `linked_rule_ids`、`source_order`）仍为 test-only。运行时/治理边界只移动
  这一张小表，不再外扩。

## 后果

- GUI 等显示层能以约 1–2KB 的二进制代价，为任何可解析的 `source_id`
  标注《紫微斗数全书》卷/节，无需解析治理数据。
- 卷节元数据只存一份；rules.toml、pattern 元数据与 inventory 无法漂移
  （漂移即测试失败）。
- 新增 inventory `source_group` 时必须补充对应的 `sections.toml` 条目；
  否则 `tests/source_sections.rs` 显式失败。
- 若未来某典籍打破 anchor/doc_path 可推导性不变式，则卷节表须补回相应字段；
  绊线测试已记录这一点。
