# 路线图

本路线图刻意保守。项目应先建立稳定架构和兼容性测试，再扩展解读深度。

## Phase 0：文档与架构

- [x] 项目规格。
- [x] 中英文 README。
- [x] 架构文档。
- [x] 兼容性政策。
- [x] 规则引擎设计。
- [x] 术语表。
- [x] 关键决策 ADR。

## Phase 1：Rust workspace 脚手架

- [x] 创建 Rust workspace。
- [x] 添加核心 crates：
  - [x] `iztro-core`；
  - [x] `iztro-features`；
  - [x] `iztro-rules`；
  - [x] `iztro-reading`；
  - [x] `iztro-cli`。
- [x] 添加格式化、clippy、测试 CI。
- [x] 添加序列化和 fixture-based test 基础设施。

`iztro-core` 的源码树按领域模块组织：`model`（值对象、星曜事实与不可变星盘事实）、`placement`（确定性安星与叠加层激活构建器）、`facade`（对外的 iztro 兼容入口）、`feature`（为未来派生事实提取保留的边界）。crate 错误类型保留在 crate 根部。这只是内部重组；公开 API 与排盘行为均未改变。

## Phase 2：核心星盘模型

- [x] 定义天干、地支、宫位、星曜、四化、作用范围、性别、历法配置。
- [x] 定义星盘、宫位和星曜落点模型。
- [x] 定义大限与运限模型。
- [x] 确保已实现模型强类型且可序列化。
- [x] 将上游 `iztro@2.5.8` runtime 星曜名称清单与已表示星盘事实分开维护。

大限与运限模型以叠加层形式定义：`HoroscopeChart` 包裹不可变的本命 `Chart`，并持有
零个或多个 `TemporalLayer`，每个层带有非本命的 `Scope`、强类型的 `TemporalContext`、
按范围划分的 `StarPlacement` 和 `MutagenActivation`。这些只是调用方显式提供的模型
事实；时间范围安星与历法推导仍推迟到 Phase 3。这些模型之上的前两个时间算法现已
提供：`build_yearly_mutagen_layer` 从显式的流年干支生成流年 `TemporalLayer` 四化
激活，`build_decadal_mutagen_layer` 则从显式的大限干支及起运年龄生成大限版。两者都
复用共享的天干四化表，作用于本命盘中已存在的星曜。它们仅为叠加层——不安放流曜、
不修改本命、不推导历法/年龄区间/大限命宫/大限宫位——四化仍作为 `MutagenActivation`
事实，而非独立星曜。

星曜 metadata 有意拆成两层。已表示 metadata table 仍只包含 66 颗已安放且有 fixture
覆盖的本命星。Known metadata table 记录 170 个上游 `iztro@2.5.8` runtime 星曜名称
条目，包括中州派特有杂曜、装饰性数组以及 horoscope 流曜名称；但这些新增条目仅是
metadata。`xunzhong` / `旬中` 因只属于 locale 而被排除；中州派安星、神煞安星、流曜
安星、horoscope 安星、亮度扩展，以及把四化建模为星曜，仍然延期。

## Phase 3：排盘兼容性

- [x] 实现最小 `by_lunar` 入口。
- [ ] 实现最小 `by_solar` 入口。
- [x] 将当前排盘切片拆成小的确定性模块进行移植或重写。
- [x] 加入与选定 `iztro` 输出对齐的 implemented-slice golden tests。
- [x] 记录 implemented slice 的已知差异。
- [x] 添加默认算法本命杂曜。iztro 2.5.8 默认算法的全部 38 颗杂曜均已安放；逐星落点基准见兼容性文档「默认算法本命杂曜全集」。四颗中州派特有杂曜（龙德/截空/劫煞/大耗）与中州派算法选择仍然推迟。
- [ ] 添加阳历转农历、闰月行为、早晚子时变体、时间范围星曜和 bindings。

当前核心切片：`by_lunar` 接受显式农历输入以及显式出生年干、年支，生成确定性的本命星盘事实，并用选定的 `iztro` 2.5.8 fixtures 校验 minimal chart 字段、十四主星、十四颗已支持辅星，以及完整默认算法的 38 颗本命杂曜/辅助星——14 主星 + 14 辅星 + 38 杂曜/辅助星 = 66 颗已表示本命星。中州派特有杂曜与中州派算法选择、默认 `getAdjectiveStar` 切片以外的神煞、流曜、阳历转农历、闰月行为、早晚子时变体、时间/horoscope 星曜范围和 bindings 仍然推迟。四化仍作为安星上的 `Mutagen` 事实，而非独立星曜。

更广的 known 星曜名称清单支持 API discovery 和未来小范围工作，但不改变 `by_lunar`、
安星行为、fixtures、亮度或四化建模。

## Phase 4：特征提取

- [x] 提取宫位特征。
- [x] 提取星曜特征。
- [x] 提取本命四化流向。
- [x] 提取宫位关系、三方四正、对宫。
- [ ] 添加强弱评分占位接口。
- [ ] 添加时间激活接口。

首个切片已实现：`iztro-features` 的 `BasicFeatureExtractor` 将确定性星盘事实转换为结构化的宫位特征、星曜特征、本命四化流向和宫位循环关系。星曜特征保留所有落点星曜事实；宫位与领域的映射是可选元数据，目前仅限五个直接的宫位—领域映射（命宫、官禄宫、财帛宫、夫妻宫、疾厄宫），其他宫位的星曜不带领域。此阶段仅做特征提取——不做规则匹配、不产出判断、不做解读、不生成叙事。强弱评分与时间激活接口仍然推迟。

## Phase 5：规则引擎骨架

- [ ] 定义规则 schema。
- [ ] 从 TOML 加载规则。
- [ ] 用规则匹配提取后的特征。
- [ ] 输出带证据和来源元数据的结构化判断。
- [ ] 为规则匹配添加确定性单元测试。

## Phase 6：基础确定性解读

- [ ] 添加少量 seed rules。
- [ ] 为性格、事业、财富、关系生成领域判断。
- [ ] 从结构化判断渲染确定性报告。
- [ ] 叙事保持简洁、证据优先。

## Phase 7：多方法扩展

- [ ] 添加 method profiles。
- [ ] 支持多个排盘或特征提取策略。
- [ ] 添加不同派别或解读风格的可选规则集。
- [ ] 保持 profile 组合显式且可测试。

## Phase 8：绑定与应用

- [ ] CLI。
- [ ] Python bindings。
- [ ] WebAssembly bindings。
- [ ] TUI 前端，推迟到核心模型和报告结构稳定后再设计。
- [ ] GUI 前端，推迟到核心模型和报告结构稳定后再设计。
- [ ] 可选 LLM 叙事润色。

应用前端会被刻意推迟。核心 crates 应保持 UI 无关、确定性、可序列化，使未来 CLI、TUI、GUI、WASM 和 Python 前端可以直接消费星盘、特征、判断、证据和报告结构，而不需要解析自然语言文本。

## 发布策略

`0.1.0` 之前 API 可以自由变更。`0.1.0` 之后，破坏性变更应记录在 `CHANGELOG.md`，必要时补充 ADR。
