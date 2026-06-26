# 《紫微斗数全书》原文资料

本目录保存《紫微斗数全书》三卷原文，作为 `iztro-rs` 经典规则语料整理、出处核对和后续规则规范化的人工参考。

这些 Markdown 文件是**可读原文层**，不是可执行规则层。运行时代码不应直接解析这些文本，也不应把中文原文作为逻辑键。可执行规则仍应通过 `crates/iztro/rule-corpus/quan-shu/` 中的结构化语料和 Rust 判定器进入 classical rule engine。

## 文件

- [卷一 / Volume 1](./volume-01.md)
- [卷二 / Volume 2](./volume-02.md)
- [卷三 / Volume 3](./volume-03.md)

## 对应的结构化 source inventory

本 PR 同时加入一个最小的结构化 source inventory pilot：

```text
crates/iztro/rule-corpus/quan-shu/source/
  README.md
  volume-01.toml
```

`volume-01.toml` 采用「原子 source item」结构：每条受引出处单元（一句断语/规则候选）即一个 source item，登记其对应规则出处。它涵盖太微赋完整规范化映射，但不是卷一全部章节的完整清单；完整 line-by-line inventory、lint 应在后续 PR 中继续扩展。

后续可加入尚未链接的 `raw` / `segmented` source item（`linked_rule_ids = []`）记录已切分但尚未规范化/实现为规则的原文；当前太微赋每个 `rule_linked` 条目均已链接规则。

source inventory 的覆盖情况由维护在仓库中的覆盖报告统计：

```text
docs/zh-CN/rules/quan-shu-coverage.md
```

该报告由测试 `crates/iztro/tests/classical_source_coverage.rs` 生成并校验；扩展 source inventory 后须重新生成该报告，否则测试失败。

结构分两层：

```text
source item = 一条受引的原子出处单元（一句断语/规则候选），由稳定助记 `source_id` 标识
rule        = 链接到某个 source item 的可执行/规范化/歧义/拒绝解释，通过 `linked_rule_ids` 关联
```

source item 的边界是**语义**而非排版：默认以 `。` 切分；一个 `。` 句内若含并列独立断语则继续切分；同一断语的「条件，应验」逗号不切分。一条 Markdown 物理行可包含多个 source item。`source_id` 标识**受引出处单元**而非物理行/段落，为稳定助记符（如 `ma_yu_kong_wang`）；`source_order` 单独保存出处顺序，新增靠前条目只需复核 `source_order`，无需改写稳定 `source_id`。规则一侧通过 `source_id` 指向 source item；source item 一侧通过 `linked_rule_ids` 反向链接（全书规则不再使用 `source_clause_id`）。`source_text_zh_hans` 须逐字引用出处单元（不带句末 `。`），解读归入规则的 `normalized_note_zh_hans`、`ClaimSpec` 或 i18n claim 文案。`待校` / `TODO` 仅用于「确信出自全书但尚未在三卷 Markdown 中定位到的单元」。source inventory 仅由测试校验，不进入运行时评估路径，且只校验全书规则（`work = "zi_wei_dou_shu_quan_shu"`）。

非出自全书的规则（如 `羊陀夹命`、`昌曲夹命` 等由项目直接建模的夹宫/格局结构推导而来的判断）**不是**全书 source inventory 条目，存放于 `crates/iztro/rule-corpus/patterns/`（`work = "iztro_pattern_catalog"`、`pattern.*` source id），不在此登记。

## 用途

这些文件主要用于：

1. 保存三卷原文，便于审校和人工查阅。
2. 为 `source_id` 语料清单提供出处依据。
3. 区分不同类型的全书内容，例如安星诀、格局、断语、加会、破格、限运等。
4. 支持后续把原文逐条整理为结构化 source inventory，再链接到 executable / unsupported / ambiguous rules。

## 与 rule engine 的关系

建议保持如下分层：

```text
原文 Markdown
  -> source inventory TOML
  -> rule metadata TOML
  -> Rust predicates / evaluator
  -> structured Claim[]
```

其中：

- 安星诀、起例诀、排盘诀属于 chart construction / placement 依据，通常应链接到 `core::placement` 实现和测试，不应作为解释性 `Claim` 规则。
- 断语、格局、加会、破格、限运引动等内容，才适合逐步规范化为 classical rule engine 的规则。
- 含义不明或流派差异较大的句子，应先标记为 `ambiguous` 或 `normalized`，不要直接实现为会触发的判断。

## 版权与整理说明

本目录仅用于保存和整理经典原文，供开源项目的规则建模与出处核对使用。不要复制现代出版物中的注释、讲解、排版说明或其他可能受版权保护的编辑性内容。若后续需要记录版本来源、校勘说明或分句差异，应在独立的 source inventory / documentation 中注明。

## 后续工作

后续 PR 应在此基础上补充：

1. 将 `crates/iztro/rule-corpus/quan-shu/source/` 扩展为完整 source inventory。
2. 为每条 source item 补齐稳定 `source_id`、`volume`、`anchor`、`status` 和 `source_text_zh_hans`。
3. 继续校对现有 pilot rules 与原文异文、出处段落、规范化 clause 的关系。
4. 增加 corpus lint，确保 rule 引用的 `source_id` 必须存在。
5. 扩展现有 coverage report（`docs/zh-CN/rules/quan-shu-coverage.md`），细化各原文/规则的状态分类（raw / segmented / normalized / executable / tested / ambiguous / rejected）。
