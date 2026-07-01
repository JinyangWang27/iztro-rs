# ADR 0009：领域模型第一性原则

状态：Accepted

## 背景

本轮规则引擎重构最终形成了共享的 `rules::query` 层，也让领域边界足够清晰，值得把它记录成第一性原则，而不是散落在代码注释里的实现说明。

关键变化包括：

- 通过 `EffectiveChartState` 与 `RuleEvaluationContext` 明确 selected-state 语义；
- 格局识别从 `core::pattern` 迁到 `rules::pattern`，因为格局是规则，不是核心图盘事实；
- 经典出处规则与格局规则成为 `rules` 下的兄弟规则引擎；
- 通用只读查询进入 `rules::query`，使 `rules::pattern` 与 `rules::classical` 不再为了图盘查询互相依赖；
- 昌曲夹命成为第一个 selected-state 经典规则纵切片，证明经典规则可以读取运限选中帧而不改写本命事实。

本文记录后续开发应遵守的领域模型边界。

## 决策

项目应保留以下第一性原则。

### 1. Core 拥有图盘事实，不拥有解释

`core` 拥有确定性的事实与值对象：

- 出生上下文与计算策略；
- 天干、地支、生肖、纳音、五行局、宫位、星曜、四化；
- 本命图盘事实；
- 运限叠加事实；
- 作为选中有效图盘状态的 `EffectiveChartState`。

`core` 不应拥有规则引擎、GUI 状态、渲染布局、本地化长文本或命理解释。

### 2. 只有一个本命图盘；时间只增加叠加层

`Chart` 是不可变的本命事实聚合。时间状态是加法式叠加：

```text
Chart
  + TemporalLayer[]
  -> HoroscopeChart
```

运限层可以贡献带作用域的星曜落点、装饰星事实、宫名布局和四化激活。它们不能改写本命宫位，也不能复制本命事实。

### 3. 地支是稳定坐标；宫名是帧相对语义

`EarthlyBranch` 是稳定的宫位格子坐标。`PalaceName` 由某个宫名帧赋予。

本命盘与每个运限层都可能在同一地支环上提供不同的宫名帧：

```text
地支 = 稳定坐标
宫名 = 某个选中帧下的语义
```

选中流年视图会改变 active palace frame；它不会改变本命图盘。

### 4. Effective state = 选中宫名帧 + active fact stack

`EffectiveChartState` 表示规则通常应该读取的选中状态：

```text
选中宫名帧
+ active scopes
+ 本命事实
+ 可见祖先/当前运限叠加
+ 每个有效事实的来源作用域
```

这是普通 selected-view 规则的默认语义面。明确 source/layer-specific 的规则仍可只读取单一来源层，但这种意图必须体现在 helper 名称和规则语义里。

### 5. Context 描述图盘状态；规则元数据描述规则身份

`RuleEvaluationContext` 描述正在被评估的图盘状态。它不应该描述规则身份。

不要把规则身份放进 context，例如：

```text
is_pattern = true
```

这是错误的轴。格局是规则输出或规则 facet 的一种，不是另一种图盘状态上下文。

当前包装关系是：

```text
RuleEvaluationContext
├── PatternContext
└── ClassicalRuleContext
```

这些 wrapper 是为了各规则引擎自己的 API、请求与查询表面而存在，不应该在图盘状态语义上分叉。

### 6. 格局是规则

格局是关于图盘状态的结构规则。它目前输出 `PatternDetection` 而不是 `Claim`，但它仍然是规则引擎代码。

因此：

```text
rules::pattern = 格局规则引擎
rules::classical = 经典出处 / claim 规则引擎
```

`core::pattern` 不应作为规则引擎命名空间回归，否则 `core` 又会拥有规则逻辑。

### 7. 经典原文是来源，不是生成文本

对经典规则而言，中文原文是权威来源，必须与解释分离。

`source_text_zh_hans` 必须逐字引用被引用的原文条款。解释性内容应放在 normalized note、claim metadata、i18n 文本、commentary metadata 或未来 narrative 层中。

### 8. 被多个规则引擎共享的查询属于 `rules::query`

当通用只读规则查询被多个规则引擎使用时，应放在 `rules::query`。

格局专用 wrapper 可以留在 `rules::pattern::query`；经典规则专用谓词可以留在 `rules::classical::predicates`。但任何一个规则引擎都不应依赖另一个规则引擎的 query module 来做通用星曜、宫位、夹宫、selected-state 或亮度查询。

目标依赖形态：

```text
rules::query
├── rules::pattern
└── rules::classical
```

而不是：

```text
rules::pattern::query
└── rules::classical
```

### 9. Analysis 只协调规则引擎，不解释

`analysis` 协调 selected-view 下逐层可缓存的规则评估：

```text
TemporalAnalysisContext
+ AnalysisLayerKey
+ AnalysisLayerRequest
-> AnalysisLayerResult
```

它可以构造正确 context、为缓存截断 active scopes，并组合 `rules::pattern` 与 `rules::classical`。它不应生成长文本、不应改写图盘，也不应成为第二套规则引擎。

### 10. Projection 与 facade 是 read-model 边界

Projection/facade 代码把事实和分析结果转成适合渲染器消费的 read model。GUI、CLI、TUI、MCP 与未来 3D 界面应消费这些 read model，而不是重新推导图盘或规则逻辑。

渲染器可以决定布局、标签、颜色和交互。它不能拥有安星、effective-state 或规则评估语义。

## 当前模块图

```text
core/
  calculation/         输入规范化与计算策略
  model/               图盘事实和值对象
  placement/           确定性安星构造器
  rule_context.rs      共享 selected-state context，暂时仍在 core

rules/
  query.rs             共享只读规则查询
  pattern/             格局检测
  classical/           来源命中、claim、diagnostic

analysis/              selected-view 逐层协调
projection/            可序列化 GUI/API read model
facade/                围绕 projection 与 core facts 的编排
render/                确定性渲染消费方
reading/               报告结构与 narrative-facing contract
```

`RuleEvaluationContext` 目前仍在 `core`，因为它引入时 pattern 代码还在 `core` 下。将来可以把它迁到 `rules::context`，但这不是当前架构成立的必要条件。

## 反模式

除非未来 ADR 明确改变边界，否则应避免：

- 把 `is_pattern`、`is_classical` 等规则身份标志放进 `RuleEvaluationContext`；
- 让 `core` 依赖 `rules`；
- 把 `core::pattern` 作为转发 shim 恢复；
- 静默把运限宫名当作本命宫名；
- 用“最深 active scope”猜测 selected-view，而不是显式使用 selected frame；
- 在格局与经典引擎中重复实现通用查询 helper；
- 把解释或 claim 文本写进 `source_text_zh_hans`；
- 让 GUI 或渲染代码推导图盘事实或规则结果。

## 后果

这会让模型保持朴素、可测试：

- 图盘生成保持确定性，便于 fixture 测试；
- 运限叠加保持加法式、可检查；
- selected-frame 相关 bug 更容易暴露；
- 格局与经典规则可独立演化，同时共享查询 helper；
- GUI 与报告层可消费结构化事实，而不是变成另一套命理引擎。

代价是边界类型略多：`EffectiveChartState`、`RuleEvaluationContext`、`PatternContext`、`ClassicalRuleContext`、analysis keys。这个代价是有意的：这些名字编码了真实的领域区别，而这些区别过去很容易被混淆。
