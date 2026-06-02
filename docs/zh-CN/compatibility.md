# 兼容性政策

`iztro-rs` 受 `iztro` 启发，早期应在适用范围内以 `iztro` 校验排盘行为。

## 兼容性的含义

兼容性意味着：

- 选定的排盘输出应与 `iztro` golden fixtures 保持一致；
- 差异应明确记录；
- Rust 公开模型应尽量保留相同的星盘事实；
- 测试应显式说明兼容目标，而不是隐含假设。

## 兼容性不意味着

兼容性不要求：

- 内部架构完全一致；
- 公开 API 名称完全一致；
- 数据表示仍然是字符串优先；
- 解盘叙事输出完全一致；
- 第一版支持 `iztro` 的全部功能。

## 兼容目标

当前兼容目标为：

- `iztro` npm package version `2.5.8`。

后续兼容 fixture 如需更新目标版本，必须同时记录版本变化和预期输出差异。

## 当前 fixtures

第一个 fixture 为：

- `fixtures/iztro/minimal_natal_1990_05_17_chen_female.json`

该 fixture 只比较 `iztro-rs` 当前已实现的字段：

- 出生时辰；
- 性别；
- 命宫地支；
- 身宫地支；
- 十二宫地支；
- 十二宫名称。

它有意不比较星曜、亮度、四化、大限、流年或解读文本。

## Golden tests

Golden tests 应包括：

- 阳历排盘；
- 农历排盘；
- 闰月行为；
- 早子时与晚子时行为；
- 年分界行为；
- default algorithm 行为；
- 若支持中州派，则包含中州派行为。

## 致谢

`iztro` 使用 MIT License。若直接改写或移植部分逻辑，应在源码注释或文档中保留适当 attribution。
