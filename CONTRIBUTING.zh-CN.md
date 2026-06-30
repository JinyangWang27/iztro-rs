# 贡献指南

感谢你为 `iztro-rs` 做贡献。

本项目处于 1.0 之前阶段：排盘已实现并有 fixture 兜底，而特征提取、规则引擎与叙事解盘仍在演进。贡献应遵守 `docs/` 下记录的架构设计。

## 构建前置要求

桌面 GUI 内置了一个 CJK 字体，该字体由 **Git LFS** 跟踪，并在编译期通过
`include_bytes!` 嵌入。构建 `iztro-gui` 前，请安装
[Git LFS](https://git-lfs.com/) 并确保资源已被检出（smudge）：

```bash
git lfs install
git lfs pull
```

若未使用 LFS，字体资源会停留在指针文件状态，GUI 将嵌入无效字节。
`crates/iztro-gui/src/fonts.rs` 中的守卫测试会在此情况下立即失败，
CI 也以 `lfs: true` 进行检出。

## 开发原则

请阅读：

- [项目规格](docs/zh-CN/project-spec.md)
- [架构](docs/zh-CN/architecture.md)
- [工程原则](docs/zh-CN/engineering-principles.md)
- [规则引擎](docs/zh-CN/rule-engine.md)
- [兼容性政策](docs/zh-CN/compatibility.md)

## TDD 预期

对于确定性逻辑，在可行时使用测试驱动开发：

1. 添加或更新一个失败测试；
2. 实现最小的确定性改动；
3. 在测试保持通过的前提下重构。

这尤其适用于：

- 排盘逻辑；
- 历法与边界行为；
- 索引运算；
- 安星；
- 特征提取；
- 规则匹配；
- 判断聚合；
- 确定性报告渲染。

## Rust 风格设计

Rust 不使用类继承，因此本项目通过 Rust 惯用法理解 SOLID：

- 小模块；
- 小 trait；
- 显式契约；
- 组合优于继承；
- enum 表示封闭集合；
- trait 表示可扩展策略；
- 通过 trait 和 method profile 实现依赖倒置。

避免创建混合排盘、特征提取、规则求值和报告渲染职责的大型 trait 或模块。

## 文档政策

主要文档在适用时应保持双语。

- 英文是工程规格的规范来源。
- 中文是紫微斗数术语的规范来源。

如果 PR 只更新一种语言的主要文档，请说明原因。

## Pull request checklist

- [ ] 改动遵守已记录的层边界。
- [ ] 确定性行为有测试，或说明暂不添加测试的原因。
- [ ] 面向用户的术语与术语表一致。
- [ ] 适用时同步更新英文和中文文档。
- [ ] 如有新的架构决策，已补充 ADR。
