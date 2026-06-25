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

`volume-01.toml` 只登记当前 classical pilot rules 对应的五条 source item，用来建立 source inventory 的格式和链接方式。它不是完整的卷一分句清单；完整 line-by-line inventory、lint 和 coverage report 应在后续 PR 中继续扩展。

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
5. 增加 coverage report，追踪每条原文处于 raw / segmented / normalized / executable / tested / ambiguous / rejected 中的哪一类。
