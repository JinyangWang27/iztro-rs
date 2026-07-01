# iztro-rs

[![Crates.io Version](https://img.shields.io/crates/v/iztro.svg)](https://crates.io/crates/iztro)
[![Crates.io Downloads](https://img.shields.io/crates/d/iztro.svg)](https://crates.io/crates/iztro)
[![Docs.rs](https://img.shields.io/docsrs/iztro.svg)](https://docs.rs/iztro)
[![CI](https://github.com/JinyangWang27/iztro-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/JinyangWang27/iztro-rs/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/JinyangWang27/iztro-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/JinyangWang27/iztro-rs)
[![License](https://img.shields.io/crates/l/iztro.svg)](https://crates.io/crates/iztro)

`iztro-rs` 是一个 Rust 版紫微斗数项目，目标是提供排盘、特征提取，以及未来基于规则的解盘能力。

> 状态：核心排盘事实面、fixture-backed 兼容性切片、运限模型、静态 GUI view model、运行时本地化和本地 GUI 原型均已具备；完整上游 API parity、完整八字/规则/叙事仍未完成。

English version: [README.md](README.md).

## 安装

公开 API 以单个 crate 发布：

```
cargo add iztro
```

```rust
use iztro::{by_solar, Gender, SolarChartRequest};
```

该 crate 在内部保留清晰的领域边界，并以模块形式实现——`core`、`features`、`rules`、`reading`、`render`——同时把稳定的对外核心 API 也从 crate 根部 re-export。

## 快速演示

可运行的纯文本排盘演示：

```bash
cargo run -p iztro --example plain_text
```

## 项目目标

`iztro-rs` 计划提供：

- 强类型的 Rust 紫微斗数核心模型；
- 在适用范围内与 `iztro` 的排盘结果保持兼容；
- 面向宫位、星曜、四化、宫位关系、格局、运限的特征提取层；
- 输出结构化判断而非直接输出文章的规则引擎；
- 确定性的报告生成能力，并为未来可选的 LLM 叙事润色保留接口；
- 未来支持 CLI、TUI、MCP、Python binding、WebAssembly 等使用场景。

## 早期非目标

早期版本不追求：

- 做成算命 SaaS；
- 以 LLM 直接解盘作为核心逻辑；
- 一开始就覆盖所有紫微斗数流派；
- 完整复刻 `iztro` 的所有公开 API；
- 替代人类在古法或现代术数解释中的判断。

## 初始架构

项目按多层边界设计：

1. **Core Chart Layer**：确定性排盘事实与领域模型。
2. **Snapshot / Read Model Layer**：面向渲染器中立的图盘和 GUI/API read model。
3. **Runtime Localization Layer**：在展示边界处理标签与 UI 文案本地化。
4. **Render / Application Layer**：文本、GUI、未来 TUI/MCP/3D 等消费者。
5. **Feature / Rule / Narrative Layers**：面向解释的层，必须消费结构化事实而不是解析渲染文本。

详见 [docs/zh-CN/architecture.md](docs/zh-CN/architecture.md)。领域模型第一性原则见 [ADR 0009](docs/zh-CN/adr/0009-domain-model-first-principles.md)。

## 与 iztro 的关系

本项目受 [`iztro`](https://github.com/SylarLong/iztro) 启发。排盘逻辑会在适用范围内以 `iztro@2.5.8` 作为 fixture-backed 兼容性校验目标，但 Rust 内部 API 可以为了类型安全和长期扩展性而做不同设计。中州地盘/人盘属于 Rust 扩展行为，因为上游 `iztro@2.5.8` 不暴露对应 chart-plane 输出。

详见 [docs/zh-CN/compatibility.md](docs/zh-CN/compatibility.md)。

## 文档

英文文档是工程规格的规范版本；中文文档是一等翻译，并作为紫微斗数术语的规范来源。

- [项目规格](docs/zh-CN/project-spec.md)
- [当前状态](docs/zh-CN/current-status.md)
- [架构](docs/zh-CN/architecture.md)
- [领域模型第一性原则](docs/zh-CN/adr/0009-domain-model-first-principles.md)
- [路线图](docs/zh-CN/roadmap.md)
- [兼容性](docs/zh-CN/compatibility.md)
- [术语表](docs/zh-CN/terminology.md)
- [规则引擎概览](docs/zh-CN/rule-engine.md)
- [经典规则引擎](docs/zh-CN/rules/rule-engine.md)
- [多语言文档](docs/zh-CN/i18n.md)

## 致谢

本项目受 [`iztro`](https://github.com/SylarLong/iztro) 启发。`iztro` 使用 MIT License。`iztro-rs` 会在适用范围内复现与 `iztro` 兼容的排盘行为。

## License

MIT.
