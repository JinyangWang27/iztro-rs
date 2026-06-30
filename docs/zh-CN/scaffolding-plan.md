# Rust 脚手架计划（历史文档）

> 历史说明：本文件记录了第一次 Rust workspace 脚手架 PR 的原始实现说明。workspace
> 早已超越最初的脚手架阶段。当前已实现的范围请参见
> [`current-status.md`](current-status.md)、[`roadmap.md`](roadmap.md) 与
> [`architecture.md`](architecture.md)。

完整的英文原文（含原始 crate 边界意图、原始非目标、原始 Rust 设置与验收标准）见
[`docs/en/scaffolding-plan.md`](../en/scaffolding-plan.md)。该英文版按
[多语言文档政策](i18n.md) 作为本历史文档的规范来源保留。

要点回顾：

- 当初分布在多个 crate 中的领域边界，现已成为单个 `iztro` crate 内部的模块
  （`core`、`features`、`rules`、`reading`、`render`）；`iztro-cli`、`iztro-i18n`、
  `iztro-gui` 是独立 crate。
- 核心放置确定性命盘事实；特征提取与规则匹配分离；结构化判断与叙事分离；不引入过早的
  GUI/LLM 依赖。
- 原始“非目标”仍是未完成领域的有用护栏：不要把暂缓的功能当作稳定行为来呈现。
