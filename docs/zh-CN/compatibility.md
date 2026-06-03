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

fixtures 为：

- `fixtures/iztro/minimal_natal_1990_05_17_chen_female.json`
- `fixtures/iztro/major_stars_1990_05_17_chen_female.json`

minimal-natal fixture 只比较 `iztro-rs` 当前已实现的字段：

- 出生时辰；
- 性别；
- 命宫地支；
- 身宫地支；
- 十二宫地支；
- 十二宫名称；
- 十二宫天干；
- 五行局。

十二宫天干由出生年干按起五行寅例生成，并与 iztro 每宫的 `heavenlyStem`
对照。五行局与 iztro 的 `fiveElementsClass` 对照（`火六局` 映射为 `fire6`）。

出生年干目前通过 fixture input 中的 `birth_year_stem` 显式提供，因为公历到干支年
推导尚未实现。

它有意不比较星曜、亮度、四化、大限、流年或解读文本。

### 十四主星

`major_stars_1990_05_17_chen_female.json` 比较十四主星已表示的事实，与 iztro
每宫的 `majorStars` 对照：

- 每宫的主星名称；
- 每颗主星所在的宫位地支；
- 每颗主星的亮度；
- 已表示主星的生年四化。

安星复现 iztro 2.5.8（`getStartIndex` 与 `getMajorStar`）：紫微由五行局与农历
日推出，天府为紫微关于寅–申轴的镜像，紫微系与天府系按固定偏移排布。每颗星的
category 为 `major`，scope 为 `natal`（iztro 的 `origin`）。
亮度复现 iztro 2.5.8 `STARS_INFO` 中十四主星的表，保留 `de`（`得`）为
`advantage`，`li`（`利`）为 `favourable`。生年四化复现 iztro 2.5.8 的天干四化
表，但仅在目标星属于当前已表示的十四主星时记录。

农历日通过 `input.lunar_day` 显式提供，因为完整历法转换尚未实现。兼容性测试走公开
的 `build_natal_chart_with_major_stars` builder path：先生成 minimal natal chart，
再使用其派生出的五行局、显式农历日和显式出生年干安十四主星，并附加已支持的事实
状态。该 fixture 仍**不**比较特征提取、规则引擎输出、叙事输出、历法转换、辅星、
杂曜、非主星、非主星四化、大限、流年或其他时间范围。

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
