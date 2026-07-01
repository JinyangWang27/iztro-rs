# 路线图

本路线图刻意保守。项目应先保持命盘事实稳定且由 fixtures 支撑，再扩展本地化渲染与应用 surface，最后再扩展解读深度。

## Phase 0：文档与架构

- [x] 项目规格。
- [x] 中英文 README。
- [x] 架构文档。
- [x] 兼容性政策。
- [x] 规则引擎设计。
- [x] 术语表。
- [x] 关键决策 ADR。
- [x] 当前状态文档。
- [x] 可运行纯文本 chart demo。
- [x] 记录 static-chart-first GUI 方向。
- [x] 记录 TUI、MCP、3D 作为 typed facts / projections 的下游消费者。
- [x] 记录 core chart generation 架构、天地人三盘、diagnostics 和 invariants。

## Phase 1：Rust workspace 脚手架

- [x] 创建 Rust workspace。
- [x] 添加 workspace crates：
  - [x] `iztro` core/library crate；
  - [x] `iztro-cli`；
  - [x] `iztro-i18n`；
  - [x] `iztro-gui` local desktop prototype。
- [x] 将 `iztro` 库组织为内部领域模块（而非独立 crate）：`core`、`features`、
  `rules`、`reading`、`render`。
- [x] 添加格式化、clippy、测试 CI。
- [x] 添加序列化和 fixture-based test 基础设施。

`core` 的源码树按领域模块组织：`model`（值对象、星曜事实、不可变命盘事实和 renderer-neutral snapshots）、`placement`（确定性安星与 overlay activation builders）、`facade`（公开 iztro-compatible 排盘入口）。面向 GUI 的 read model 放在 `core` 之外的顶层 `projection` 模块（静态盘 projections），由顶层 `facade` 模块负责编排；依赖方向为 `core <- {analysis, projection} <- facade`。渲染、本地化和应用前端都位于 placement logic 之外。

## Phase 2：核心星盘模型

- [x] 定义天干、地支、宫位、星曜、四化、作用范围、性别、历法配置。
- [x] 定义星盘、宫位和星曜落点模型。
- [x] 定义大限与 horoscope overlay models。
- [x] 确保已实现模型强类型且可序列化。
- [x] 将上游 `iztro@2.5.8` runtime 星曜名称清单与已表示 chart facts 分开维护。
- [x] 直接使用 `lunar-lite` 提供的底层天干、地支、干支循环与四柱值对象。
- [x] 通过由 `iztro-rs` re-export 的 `lunar-lite` `FourPillars` 值对象保留事实性本命四柱。
- [x] 将紫微斗数特有的纳音与五行局逻辑隔离在 `core`。
- [x] 在 `Chart` 上保留出生年 `StemBranch` 事实。
- [x] 添加 `ChartProfile` metadata，让生成的 chart 携带 method profile 与 chart-plane facts。
- [x] 添加 typed palace lookup helpers 和 required lookup variants。
- [x] 添加 renderer-neutral `ChartStackSnapshot` read model。
- [x] 添加 compact `ChartDiagnosticSnapshot`，用于结构诊断与 invariant debugging。

大限、小限和 horoscope models 都是 typed facts / overlays。`build_decadal_frame` 从本命 chart facts 推导十二个十年大限 period；`build_age_period` 从虚岁推导 fixture-backed 小限 period；`HoroscopeChart` 包裹不可变本命 `Chart`，并持有 temporal layers 和可选 target context。

## Phase 3：排盘兼容性

- [x] 实现最小 `by_lunar` 入口。
- [x] 实现最小 `by_solar` 入口。
- [x] 将当前排盘切片拆成小的确定性模块进行移植或重写。
- [x] 加入与选定 `iztro` 输出对齐的 implemented-slice golden tests。
- [x] 记录 implemented slice 的已知差异。
- [x] 添加默认算法本命杂曜。
- [x] 添加中州派特有本命杂曜。
- [x] 添加中州天盘/地盘/人盘支持，作为 Rust extension behaviour。
- [x] 将 natal chart-plane anchor resolution 抽出为专门的 placement resolver。
- [x] 为已支持 natal algorithm/plane 组合添加 invariant coverage。
- [x] 安放装饰性 runtime 星曜家族为无类型 `DecorativeStarPlacement`。
- [x] 安放 scoped flow stars 为带地支标签的 `ScopedStarPlacement`。
- [x] 通过内部 `lunar-lite` 历法适配器（`core/calendar`）添加阳历转农历与闰月行为。
- [x] 添加 solar time、year boundary、leap-month boundary 和 nominal-age boundary calculation policies。
- [x] 添加上游 `timeIndex` `0..=12` 早晚子时变体。
- [x] 通过内部 `lunar-lite` 历法适配器推导出生年干支并保留在 `Chart` 上。
- [x] 在 `by_solar` charts 上保留完整事实性本命四柱；`by_lunar` 仍保持显式输入且不支持完整四柱推导。
- [x] 添加 typed decadal-frame derivation。
- [x] 在选定大限 layer 上添加 decadal temporal palace-name layout。
- [x] 添加 fixture-backed 小限 / age period context、palace-name layout 和 mutagen overlay。
- [x] 添加 fixture-backed 流月 / monthly period context、palace-name layout、mutagen overlay 和 flow-star layer assembly。
- [x] 添加 fixture-backed 流日 / daily period context、palace-name layout、mutagen overlay 和 flow-star layer assembly。
- [x] 添加 fixture-backed 流时 / hourly period context、palace-name layout、mutagen overlay 和 flow-star layer assembly。
- [x] 添加 full horoscope stack assembly：把大限 / 小限 / 流年 / 流月 / 流日 / 流时 layers 组装为一个 `HoroscopeChart`，并按推导虚岁选择大限 period。
- [x] 添加流年 `yearlyDecStar`（岁前/将前十二神）作为 yearly-scope temporal decorative facts。
- [x] 添加 `HoroscopeSupportedFieldsSnapshot`，用于已实现 full horoscope supported-field fact surface。
- [x] 添加 typed upstream runtime query helpers 与 runtime palace projections。
- [x] 添加 upstream-like horoscope facade payload snapshot。
- [x] 添加 minimal natal astrolabe facade snapshot。
- [x] 在 facade snapshots 中暴露事实性本命四柱。
- [ ] 添加 temporal decorative arrays beyond yearly `yearlyDecStar`。
- [ ] 添加 full facade serialization parity。
- [ ] 添加 factual `by_solar` natal four pillars 之外的 full BaZi interpretation/output。

当前支持切片：`by_lunar` 接受显式农历输入与显式出生年干支，生成确定性本命 chart facts，并在上游可比较 surface 上用 `iztro@2.5.8` fixtures 校验。`by_solar` 添加 `lunar-lite`-backed 阳历转农历（置于内部 `core/calendar` 适配器之后），通过可配置年分界（时刻级 `YearBoundary::LiChun`）推导出生年干支与事实性四柱（`FourPillars`），并委托 `by_lunar` 安星。`ChartCalculationConfig` 是独立于 `ChartAlgorithmKind` 与 `ChartPlane` 的 calculation-policy 维度，覆盖 solar time、year boundary、leap-month boundary 和 runtime nominal-age boundary。中州地盘/人盘是 Rust-only extension，因为上游 `iztro@2.5.8` 不暴露这些 chart planes。

计算诊断通过 generation reports 与 preview/resolution snapshots 暴露。它们检查解析后的输入事实与 runtime 虚岁事实，不让 `Chart` 存储 calculation config，也不改变 chart 序列化。

## Phase 4：Snapshot、rendering 与 static GUI

- [x] 添加 `ChartStackSnapshot` 作为 renderer-neutral stacked read model。
- [x] 在 snapshot cells 中保留传统十二宫格位置。
- [x] 将本命与 temporal fact surfaces 保持为独立 layer/cell sections。
- [x] 添加 `render` crate。
- [x] 添加确定性 plain text chart-stack renderer。
- [x] 添加从真实 `by_solar` 输入生成的可运行 plain text demo。
- [x] 添加 GUI-ready static chart projection：`projection::static_chart::StaticChartProjection`。
- [x] 添加 renderer-neutral highlight annotation DTOs，预留给 feature/rule layers。
- [x] 添加本地 Iced static chart GUI prototype。
- [x] 添加 GUI saved-chart startup flow。
- [x] 添加 GUI temporal controls，并以 `static_temporal_chart_view` 支撑。
- [x] 添加 renderer-side 三方四正 hover/click highlighting。
- [x] 添加 renderer-side mutagen badges。
- [ ] 完成第一轮 GUI polish：temporal control layout、跨 period navigation edge cases、宫位标签对齐、saved-chart edit/delete naming flow。
- [ ] 如仍有价值，添加更丰富的非 GUI 2D palace-grid renderer。
- [ ] 添加 timeline frame builder，将 static chart projections 作为可复用时间帧。
- [ ] 添加可选 3D stacked temporal view。

Render layer 消费 snapshots 与 projections；它不能生成 chart facts、推导 temporal periods、评估规则或产生解读。Static GUI 是近期主要 frontend，因为它能直观验证 chart projection。Timeline 与 3D views 应是同一 frame model 的后续消费者，而不是新 chart engine。

## Phase 5：运行时 i18n

- [x] 添加 `crates/iztro-i18n`。
- [x] 使用 Fluent resources，并在编译期打包。
- [x] 首批支持 `en-US` 与 `zh-Hans`。
- [x] 将 `en-US` 作为默认 runtime / GUI locale。
- [x] 将简体中文术语作为一等 locale 保留。
- [x] 为星曜、宫位、四化、时间标签、亮度标签和共享 UI 字符串添加 typed helpers。
- [x] 迁移现有 `iztro-gui` 用户可见字符串，使当前 UI 可用英文或简体中文使用。
- [x] 保持 core facts language-neutral，并把 localization 放在 presentation/export boundaries。
- [ ] 在 English / Simplified Chinese surface 稳定后，再添加更多 locale。
- [ ] 对未来 GUI/TUI/MCP surfaces 做 hardcoded user-facing string audit。

`iztro-i18n` 独立于 chart generation。Facade snapshots 可以保留附加 conventional zh-CN labels 用于兼容和可读性，但 GUI runtime localization 通过 `iztro-i18n`。

## Phase 6：TUI 与 MCP 工具

- [ ] 为 selected render/view outputs 添加 CLI integration。
- [ ] 在 `ChartStackSnapshot` / `StaticChartProjection` 之上添加 TUI frontend。
- [ ] 为 coding agents 定义稳定 machine-readable query outputs。
- [ ] 等 typed facade/query surface 足够稳定后再添加 MCP server/tooling。
- [ ] 暴露 chart facts、view snapshots、pattern hits、claims 和 evidence 为结构化 outputs。
- [ ] 避免在已有 typed fact surface 时只暴露 prose。

TUI 与 MCP 是 tooling/application consumers。它们不能复制 placement logic、解析 rendered text，或成为另一套 interpretation engine。

## Phase 7：Feature extraction 与 patterns

- [x] 提取 palace features。
- [x] 提取 star features。
- [x] 提取 natal mutagen flows。
- [x] 提取 palace relations、triads 与 oppositions。
- [x] 添加第一个 read-only `rules::pattern` 切片，用结构化 facts 表示 classical pattern detection。
- [ ] 添加 strength-score placeholders。
- [ ] 添加 temporal activation interfaces。
- [ ] 添加适合后续成格和 highlight annotations 的 pattern-hit interfaces。

第一轮 feature slice：`BasicFeatureExtractor` 将 deterministic chart facts 转换为 palace features、star features、natal mutagen flows 与 cyclic palace relations。Pattern slice 将 chart facts 识别为结构化结果；它不输出 prose，也不修改 chart state。

## Phase 8：Rule engine skeleton

- [ ] 定义 rule schema。
- [ ] 从 TOML 加载 rules。
- [ ] 基于 extracted features 匹配 rules。
- [ ] 输出带 evidence 与 source metadata 的 structured claims。
- [ ] 输出用于成格、limit-triggered、flow-triggered configurations 的 structured pattern/highlight annotations。
- [ ] 为 rule matching 添加 deterministic unit tests。

Pattern 和 成格 highlighting 应从 features/rules 流向 structured annotations。Renderer 可以高亮相关宫位、星曜、四化或 temporal scopes，但不应包含 astrology-specific rule logic。

## Phase 9：基础确定性解读

- [ ] 添加小型 seed rule set。
- [ ] 为性格、事业、财富、关系生成 domain-level claims。
- [ ] 从 structured claims 渲染 deterministic reports。
- [ ] 保持 narrative 简洁且 evidence-based。

## Phase 10：多方法扩展

- [ ] 添加更丰富的 method profile 配置。
- [ ] 支持多个 chart-generation 或 feature-extraction strategies。
- [ ] 为不同流派或解释风格添加可选 rule sets。
- [ ] 保持 profile combinations 显式且可测试。

## Phase 11：Bindings 与应用

- [ ] Python bindings。
- [ ] WebAssembly bindings。
- [ ] GUI/WASM application。
- [ ] 可选 LLM-assisted narrative polishing。

应用前端仍是 typed facts、snapshots、projections、features、claims、evidence、annotations 和 reports 的消费者。它们不应解析 narrative text 来恢复领域事实，也不应把 chart-generation/rule logic 嵌入 UI code。

## 发布政策

`0.1.0` 之前 API 可自由调整。`0.1.0` 之后，breaking changes 应记录在 `CHANGELOG.md`，必要时也记录 ADR。
