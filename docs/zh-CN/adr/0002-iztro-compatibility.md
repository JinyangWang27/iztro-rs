# ADR 0002：iztro 兼容性

## 状态

已接受。

## 背景

`iztro-rs` 受 `iztro` 启发。项目应借助 `iztro` 作为兼容性参考，同时使用 Rust 原生模型和架构。

## 决策

`iztro-rs` 会在适用范围内以 `iztro` 校验选定的排盘行为。兼容性是测试目标，不要求完全复制内部架构或公开 API。

## 影响

- Golden fixtures 应标明所使用的具体 `iztro` 版本或 commit。
- Rust 内部 API 可以为了强类型和扩展性而不同。
- 与 `iztro` 的差异应记录，而不是隐藏。
- 若直接改写或移植逻辑，应保留适当 MIT attribution。
