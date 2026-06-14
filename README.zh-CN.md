# iztro-rs

[![CI](https://github.com/JinyangWang27/iztro-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/JinyangWang27/iztro-rs/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/JinyangWang27/iztro-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/JinyangWang27/iztro-rs)

`iztro-rs` 是一个 Rust 版紫微斗数项目，目标是提供排盘、特征提取，以及未来基于规则的解盘能力。

> 状态：早期设计与脚手架阶段。当前项目还不是完整排盘或解盘引擎。

English version: [README.md](README.md).

## 安装

公开 API 以单个 crate 发布：

```toml
[dependencies]
iztro = "0.1"
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
- 未来支持 CLI、Python binding、WebAssembly 等使用场景。

## 早期非目标

早期版本不追求：

- 做成算命 SaaS；
- 以 LLM 直接解盘作为核心逻辑；
- 一开始就覆盖所有紫微斗数流派；
- 完整复刻 `iztro` 的所有公开 API；
- 替代人类在古法或现代术数解释中的判断。

## 初始架构

项目按四层设计：

1. **Core Chart Layer**：确定性排盘事实与领域模型。
2. **Feature Extraction Layer**：从星盘提取结构化语义特征。
3. **Rule Engine Layer**：规则把特征转成带证据的结构化判断。
4. **Narrative Layer**：把结构化判断渲染成人类可读报告。

详见 [docs/zh-CN/architecture.md](docs/zh-CN/architecture.md)。

## 与 iztro 的关系

本项目受 [`iztro`](https://github.com/SylarLong/iztro) 启发。早期排盘逻辑会在适用范围内以 `iztro` 作为兼容性校验目标，但 Rust 内部 API 可以为了类型安全和长期扩展性而做不同设计。

详见 [docs/zh-CN/compatibility.md](docs/zh-CN/compatibility.md)。

## 文档

英文文档是工程规格的规范版本；中文文档是一等翻译，并作为紫微斗数术语的规范来源。

- [项目规格](docs/zh-CN/project-spec.md)
- [架构](docs/zh-CN/architecture.md)
- [路线图](docs/zh-CN/roadmap.md)
- [兼容性](docs/zh-CN/compatibility.md)
- [术语表](docs/zh-CN/terminology.md)
- [规则引擎](docs/zh-CN/rule-engine.md)
- [多语言文档](docs/zh-CN/i18n.md)

## 致谢

本项目受 [`iztro`](https://github.com/SylarLong/iztro) 启发。`iztro` 使用 MIT License。`iztro-rs` 早期会在适用范围内复现与 `iztro` 兼容的排盘行为。

## License

MIT.
